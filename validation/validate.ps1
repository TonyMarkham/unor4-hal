param(
    [string]$R3Port,
    [switch]$SkipUsb
)

# Host side of the validation pair (r3-validator + r4_validator). It
# neither builds nor uploads: flash both boards first (README, Validation).
#
#   1. R3 "a": S1-S8 automatic checks, including the Slice 2 button (the
#      R3 presses R4 D3; R4 D2 is free/open). S4's token must appear on
#      the R4 bridge monitor (SCI9 -> ESP32-S3 -> USB, 115200).
#   2. S9 (unless -SkipUsb): R3 "u" takes native USB; the native port
#      (2341:006D) must appear while the bridge (2341:1002) is gone, echo
#      exactly and tick about once per second. R3 "s" must report
#      Configured.
#   3. Knock: opening the native port at 1200 baud must bring the bridge
#      back and leave the R4 silent (bootloader hold).
#   4. Reset bits: R3 "!" must restart the validator (identify answers);
#      R3 "@" must hold the bootloader (no answer, bridge present); a
#      final "!" must restart the validator again.
#
# Exit code 0 only if every executed check passed. Results go to
# target/validation/<timestamp>.txt.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$artifacts = Join-Path $PSScriptRoot '../target/validation'
New-Item -ItemType Directory -Force $artifacts | Out-Null
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$results = [Collections.Generic.List[object]]::new()

function Add-Result([string]$Check, [string]$Verdict, [string]$Evidence) {
    $results.Add([pscustomobject]@{ Check = $Check; Verdict = $Verdict; Evidence = $Evidence })
}

function Find-Port([string]$UsbPid) {
    $ports = @([IO.Ports.SerialPort]::GetPortNames())
    $found = @(Get-PnpDevice -PresentOnly | Where-Object {
        $_.Class -eq 'Ports' -and $_.InstanceId -like "USB\VID_2341&PID_$UsbPid*"
    } | ForEach-Object {
        if ($_.FriendlyName -match '\((COM\d+)\)' -and $ports -contains $Matches[1]) { $Matches[1] }
    })
    if ($found.Count -eq 1) { return $found[0] }
    return $null
}

function Wait-Port([string]$UsbPid, [bool]$Present, [double]$Seconds) {
    $clock = [Diagnostics.Stopwatch]::StartNew()
    while ($clock.Elapsed.TotalSeconds -lt $Seconds) {
        $port = Find-Port $UsbPid
        if ([bool]$port -eq $Present) { return $port }
        Start-Sleep -Milliseconds 100
    }
    return $null
}

# Sends one R3 command and returns its text, frames removed, once
# $Until matches or $Seconds pass.
function Send-R3([string]$Command, [string]$Until, [double]$Seconds) {
    $null = $r3.ReadExisting()
    $r3.Write($Command)
    $text = ''
    $clock = [Diagnostics.Stopwatch]::StartNew()
    while ($clock.Elapsed.TotalSeconds -lt $Seconds) {
        $text += $r3.ReadExisting()
        if ($monitor) { $script:monitorText += $monitor.ReadExisting() }
        if ($Until -and ($text -replace '\x02[^\x03]*\x03', '') -match $Until) { break }
        Start-Sleep -Milliseconds 50
    }
    return ($text -replace '\x02[^\x03]*\x03', '')
}

function Test-Identify {
    # Wait for the whole line: "ID ID VALIDATOR 1 ..." or "ID no reply ...".
    return (Send-R3 'i' '(?m)^ID .*\r\n' 3) -match 'ID ID VALIDATOR 1'
}

# Native CDC: ticks for $TickSeconds, then printable echo of several sizes.
# Tick lines arrive only between echo chunks and are stripped first.
function Test-Native([string]$Port) {
    $usb = [IO.Ports.SerialPort]::new($Port, 115200, 'None', 8, 'One')
    $usb.DtrEnable = $true
    $usb.WriteTimeout = 5000
    $usb.ReadBufferSize = 65536
    try {
        $usb.Open()
        Start-Sleep -Milliseconds 200
        $usb.DiscardInBuffer()
        $tail = ''; $last = -1L; $count = 0; $gap = 0.0; $at = 0.0; $skip = $true
        $clock = [Diagnostics.Stopwatch]::StartNew()
        while ($clock.Elapsed.TotalSeconds -lt 6) {
            $tail += $usb.ReadExisting()
            while (($newline = $tail.IndexOf("`n")) -ge 0) {
                $line = $tail.Substring(0, $newline).TrimEnd("`r")
                $tail = $tail.Substring($newline + 1)
                if ($skip) { $skip = $false; continue } # may be cut by the discard
                if ($line -notmatch '^tick (\d+)$') { return "malformed line '$line'" }
                $number = [long]$Matches[1]
                if ($last -ge 0 -and $number -ne $last + 1) { return "tick $last -> $number" }
                $now = $clock.Elapsed.TotalSeconds
                if ($count) { $gap = [Math]::Max($gap, $now - $at) }
                $at = $now; $last = $number; ++$count
            }
            Start-Sleep -Milliseconds 10
        }
        if ($count -lt 4 -or $gap -gt 1.5) { return "only $count ticks, maximum gap $([Math]::Round($gap,3)) s" }
        $alphabet = [Text.Encoding]::ASCII.GetBytes('ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789')
        $random = [Random]::new(9009)
        $total = 0
        foreach ($length in 1, 7, 64, 65, 128, 255, 512, 1024, 4096) {
            [byte[]]$sent = [byte[]]::new($length)
            for ($i = 0; $i -lt $length; ++$i) { $sent[$i] = $alphabet[$random.Next($alphabet.Length)] }
            $usb.Write($sent, 0, $length)
            $received = ''
            $deadline = [Diagnostics.Stopwatch]::StartNew()
            while ($deadline.Elapsed.TotalSeconds -lt 5) {
                $received += $usb.ReadExisting()
                if ([regex]::Replace($received, 'tick \d+\r\n', '').Length -ge $length) { break }
                Start-Sleep -Milliseconds 2
            }
            $echoed = [regex]::Replace($received, 'tick \d+\r\n', '')
            if ($echoed.Length -lt $length -or $echoed.Substring(0, $length) -ne [Text.Encoding]::ASCII.GetString($sent)) {
                return "echo of $length bytes failed"
            }
            if ($echoed.Substring($length) -notmatch '^(t(i(c(k( \d*\r?)?)?)?)?)?$') { return "extra bytes after $length-byte echo" }
            $total += $length
        }
        return "PASS $count consecutive ticks (max gap $([Math]::Round($gap,3)) s), 9 exact echoes, $total bytes each direction"
    } finally {
        $usb.Dispose()
    }
}

$r3 = $null
$monitor = $null
$monitorText = ''
try {
    if (-not $R3Port) { $R3Port = Find-Port '0043' }
    if (-not $R3Port) { throw 'R3 (2341:0043) not found; pass -R3Port.' }
    $bridge = Find-Port '1002'
    if (-not $bridge) { throw 'R4 bridge port (2341:1002) not found. If the R4 holds native USB, open that port at 1200 baud first (the knock).' }
    Write-Output "R3=$R3Port R4 bridge=$bridge"

    $monitor = [IO.Ports.SerialPort]::new($bridge, 115200, 'None', 8, 'One')
    $monitor.DtrEnable = $true # the bridge forwards SCI9 only to an open, DTR-asserted port
    $monitor.Open()
    $r3 = [IO.Ports.SerialPort]::new($R3Port, 9600, 'None', 8, 'One')
    $r3.Open()
    Start-Sleep -Seconds 2 # the R3 may restart when its port opens
    $null = $r3.ReadExisting()
    $null = $monitor.ReadExisting()

    # ---- 1. S1-S8
    $text = Send-R3 'a' 'DONE \d+ PASS \d+ FAIL\r\n' 60
    Start-Sleep -Milliseconds 300
    $monitorText += $monitor.ReadExisting()
    $text -split "`r?`n" | Add-Content (Join-Path $artifacts "$stamp-r3.log")
    $checks = [ordered]@{
        'S1' = 'S1 (PASS|FAIL) ?(.*)'
        'S2 input' = 'S2 (PASS|FAIL) ?(R4 D3 read.*|no reply.*)'
        'S2 button' = 'S2 (PASS|FAIL) ?(button on D3.*|no reply.*)'
        'S3' = 'S3 (PASS|FAIL) ?(.*)'
        'S5' = 'S5 (PASS|FAIL) ?(echoed.*|no reply.*)'
        'S5 traits' = 'S5 (PASS|FAIL) ?(traits: .*|no reply.*)'
        'S6' = 'S6 (PASS|FAIL|NA) ?(.*)'
        'S7' = 'S7 (PASS|FAIL) ?(.*)'
        'S8' = 'S8 (PASS|FAIL) ?(.*)'
    }
    $taken = @{}
    foreach ($check in $checks.Keys) {
        # S2 and S5 each print two lines that differ only in their
        # evidence; take each line once.
        $line = [regex]::Matches($text, "(?m)$($checks[$check])\r?$") | Where-Object { -not $taken.ContainsKey($_.Index) } | Select-Object -First 1
        if ($line) { $taken[$line.Index] = $true; Add-Result $check $line.Groups[1].Value $line.Groups[2].Value.Trim() }
        else { Add-Result $check 'FAIL' 'no result line from the R3' }
    }
    $token = [regex]::Match($text, 'S4 SENT TOKEN (\d+)')
    if (-not $token.Success) { Add-Result 'S4' 'FAIL' 'no token from the R3' }
    elseif ($monitorText -match "VALIDATE S4 TOKEN $($token.Groups[1].Value)\r?\n") {
        Add-Result 'S4' 'PASS' "token $($token.Groups[1].Value) seen on the bridge monitor $bridge"
    } else { Add-Result 'S4' 'FAIL' "token $($token.Groups[1].Value) not seen on $bridge" }

    $monitorText | Add-Content (Join-Path $artifacts "$stamp-monitor.log")
    $monitor.Dispose()
    $monitor = $null

    if (-not $SkipUsb) {
        # ---- 2. S9
        $text = Send-R3 'u' 'S9 .*\r\n' 3
        $native = Wait-Port '006D' $true 15
        $bridgeHeld = Find-Port '1002'
        if (-not $native) { Add-Result 'S9' 'FAIL' "native port did not appear (R3: $($text.Trim()))" }
        else {
            $outcome = Test-Native $native
            $state = Send-R3 's' 'STATUS .*\r?\n' 3
            $configured = $state -match 'STATUS usb=2 taken=1'
            $verdict = if ($outcome -like 'PASS*' -and $configured -and -not $bridgeHeld) { 'PASS' } else { 'FAIL' }
            Add-Result 'S9' $verdict "native $native, bridge while held: $(if ($bridgeHeld) { $bridgeHeld } else { 'absent' }); $($outcome -replace '^PASS ',''); $(($state -split "`r?`n" | Where-Object { $_ -like 'STATUS*' }) -join '')"

            # ---- 3. knock
            $touch = [IO.Ports.SerialPort]::new($native, 1200, 'None', 8, 'One')
            try { $touch.Open(); Start-Sleep -Milliseconds 300; $touch.Close() } catch { } finally { $touch.Dispose() }
            $back = Wait-Port '1002' $true 10
            $gone = -not (Find-Port '006D')
            Start-Sleep -Seconds 2
            $silent = -not (Test-Identify)
            Add-Result 'Knock' $(if ($back -and $gone -and $silent) { 'PASS' } else { 'FAIL' }) "bridge=$back native gone=$gone R4 silent (bootloader hold)=$silent"
        }
    }

    # ---- 4. reset bits
    $null = Send-R3 '!' 'RESET 1' 2
    Start-Sleep -Seconds 3
    $restarted = Test-Identify
    Add-Result 'R3 !' $(if ($restarted) { 'PASS' } else { 'FAIL' }) "validator answers identify after a single RESET pulse: $restarted"
    $null = Send-R3 '@' 'RESET 2' 2
    Start-Sleep -Seconds 3
    $held = -not (Test-Identify)
    $bridgeUp = [bool](Find-Port '1002')
    $null = Send-R3 '!' 'RESET 1' 2
    Start-Sleep -Seconds 3
    $again = Test-Identify
    Add-Result 'R3 @' $(if ($held -and $bridgeUp -and $again) { 'PASS' } else { 'FAIL' }) "bootloader hold (silent)=$held bridge present=$bridgeUp; restarted by ! =$again"
} finally {
    if ($null -ne $monitor) { $monitor.Dispose() }
    if ($null -ne $r3) { $r3.Dispose() }
}

$table = $results | Format-Table -AutoSize -Wrap | Out-String -Width 200
$failed = @($results | Where-Object { $_.Verdict -eq 'FAIL' }).Count
$summary = "$($results.Count - $failed)/$($results.Count) checks passed ($failed failed)"
"$table$summary" | Set-Content (Join-Path $artifacts "$stamp.txt")
$table
$summary
if ($failed) { exit 1 }
