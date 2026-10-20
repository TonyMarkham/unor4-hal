param(
    [switch]$SkipBuild,
    [switch]$SkipUpload,
    [int]$BridgeBytes = 4096,
    [int]$PaceMs = 2,
    [int]$DurationSeconds = 25
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($BridgeBytes -lt 1 -or $PaceMs -lt 1 -or $DurationSeconds -lt 20) {
    throw 'Use BridgeBytes >= 1, PaceMs >= 1, DurationSeconds >= 20.'
}
$root = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$artifacts = Join-Path $root 'target/slice05-hardware'
New-Item -ItemType Directory -Force $artifacts | Out-Null

function Invoke-Checked([scriptblock]$Command) {
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "Command failed with exit code $LASTEXITCODE" }
}

# Discover by board identity, never by guessed COM numbering.
$listing = & arduino-cli board list --format json
if ($LASTEXITCODE -ne 0) { throw 'Board discovery failed.' }
$boards = ($listing | ConvertFrom-Json).detected_ports
$r4Boards = @($boards | Where-Object { $_.matching_boards.fqbn -contains 'arduino:renesas_uno:unor4wifi' })
$unoBoards = @($boards | Where-Object { $_.matching_boards.fqbn -contains 'arduino:avr:uno' })
if ($r4Boards.Count -ne 1 -or $unoBoards.Count -ne 1) {
    throw 'Connect exactly one UNO R4 WiFi and one UNO R3.'
}
$r4Port = $r4Boards[0].port.address
$unoPort = $unoBoards[0].port.address
Write-Host "R4=$r4Port UNO=$unoPort"

Push-Location $PSScriptRoot
try {
    $reference = Join-Path $root 'arduino/sketches/slice05-reference'
    $controller = Join-Path $root 'arduino/sketches/slice05-controller'
    $referenceBuild = Join-Path $artifacts 'reference-build'
    $controllerBuild = Join-Path $artifacts 'controller-build'
    if (-not $SkipBuild) {
        Invoke-Checked { arduino-cli compile --fqbn arduino:renesas_uno:unor4wifi --output-dir $referenceBuild $reference }
        Invoke-Checked { arduino-cli compile --fqbn arduino:avr:uno --output-dir $controllerBuild $controller }
        Invoke-Checked { cargo objcopy --release --bin slice05_rig --features uart-irq -- -O binary slice05_rig.bin }
    }
    if (-not $SkipUpload) {
        # The R4 reference leaves D0/D1 as inputs. This avoids the R4's
        # TX driver corrupting the UNO bootloader while all wires stay on.
        Invoke-Checked { arduino-cli upload -p $r4Port --fqbn arduino:renesas_uno:unor4wifi --input-dir $referenceBuild }
        Invoke-Checked { arduino-cli upload -p $unoPort --fqbn arduino:avr:uno --input-dir $controllerBuild }
        Invoke-Checked { arduino-cli upload -p $r4Port --fqbn arduino:renesas_uno:unor4wifi --input-file slice05_rig.bin }
    }

    $r4 = [System.IO.Ports.SerialPort]::new($r4Port,115200,'None',8,'One')
    $uno = [System.IO.Ports.SerialPort]::new($unoPort,115200,'None',8,'One')
    $r4.ReadBufferSize = 131072
    $uno.ReadBufferSize = 131072
    $r4.WriteTimeout = 1000
    $r4.DtrEnable = $true
    $r4.RtsEnable = $true
    $uno.DtrEnable = $true
    $uno.RtsEnable = $true
    $r4Log = [System.Text.StringBuilder]::new()
    $unoLog = [System.Text.StringBuilder]::new()
    $tickTimes = [System.Collections.Generic.List[double]]::new()
    $tickNumbers = [System.Collections.Generic.List[int]]::new()
    $lineTail = ''
    $payload = -join (0..($BridgeBytes - 1) | ForEach-Object { [char][int](97 + (($_ * 11 + [math]::Floor($_ / 26)) % 26)) })
    $sent = 0
    try {
        $r4.Open()
        # Opening COM4 resets the UNO and starts its autonomous test.
        # NEVER write host commands to COM4: its RX is driven by the R4.
        $uno.Open()
        # Let the UNO reboot, then discard USB-adapter data buffered
        # before this reset. Its first test starts at four seconds.
        Start-Sleep -Seconds 3
        $warmup = $r4.ReadExisting()
        $baselineRows = [regex]::Matches($warmup, 'tick=\d+ n2=(\d+).*? n9=(\d+)')
        if ($baselineRows.Count -eq 0) { throw 'No R4 heartbeat during controller startup.' }
        $baseline = $baselineRows[$baselineRows.Count - 1].Groups
        $expected2 = [int]$baseline[1].Value + 32768
        $expected9 = [int]$baseline[2].Value + $BridgeBytes
        $r4.DiscardInBuffer()
        $uno.DiscardInBuffer()
        $clock = [System.Diagnostics.Stopwatch]::StartNew()
        $nextSend = 1000.0
        while ($clock.Elapsed.TotalSeconds -lt $DurationSeconds) {
            $chunk = $r4.ReadExisting()
            [void]$r4Log.Append($chunk)
            $lineTail += $chunk
            while (($newline = $lineTail.IndexOf("`n")) -ge 0) {
                $line = $lineTail.Substring(0, $newline)
                $lineTail = $lineTail.Substring($newline + 1)
                if ($line -match 'tick=(\d+)') {
                    $tickTimes.Add($clock.Elapsed.TotalSeconds)
                    $tickNumbers.Add([int]$Matches[1])
                }
            }
            [void]$unoLog.Append($uno.ReadExisting())
            if ($sent -lt $BridgeBytes -and $clock.Elapsed.TotalMilliseconds -ge $nextSend) {
                # The ESP32 bridge drops USB bursts in this setup.
                # Pace individual bytes; the independent UNO path is
                # the continuous line-rate, unpaced burst test.
                $r4.Write($payload.Substring($sent, 1))
                ++$sent
                $nextSend = $clock.Elapsed.TotalMilliseconds + $PaceMs
            }
            if ($sent -lt $BridgeBytes) {
                # Windows PowerShell's millisecond sleep may round up
                # to a whole scheduler tick. Use the monotonic deadline
                # above instead of silently turning 2 ms into ~16 ms.
                [System.Threading.Thread]::SpinWait(100)
            } else {
                Start-Sleep -Milliseconds 10
            }
        }
        [void]$r4Log.Append($r4.ReadExisting())
        [void]$unoLog.Append($uno.ReadExisting())
    } finally {
        $r4.Dispose()
        $uno.Dispose()
        $r4Log.ToString() | Set-Content (Join-Path $artifacts 'r4.log')
        $unoLog.ToString() | Set-Content (Join-Path $artifacts 'uno.log')
    }

    $r4Text = $r4Log.ToString()
    $unoText = $unoLog.ToString()
    $results = [regex]::Matches($unoText, 'RESULT round=(\d+) sent=(\d+) received=(\d+) mismatches=(\d+)')
    foreach ($result in $results) { Write-Host $result.Value }
    if ($results.Count -ne 8) { throw "Expected 8 UNO rounds; received $($results.Count). See $artifacts" }
    for ($i = 0; $i -lt 8; ++$i) {
        $g = $results[$i].Groups
        if ([int]$g[1].Value -ne $i + 1 -or $g[2].Value -ne '4096' -or
            $g[3].Value -ne '4096' -or $g[4].Value -ne '0') {
            throw "UNO round failed: $($results[$i].Value)"
        }
    }
    # Echo payload is uppercase; all rig telemetry is lowercase/digits.
    $echo = $r4Text -creplace '[^A-Z]', ''
    if ($sent -ne $BridgeBytes -or $echo -cne $payload.ToUpperInvariant()) {
        throw "SCI9 echo mismatch: sent=$sent received=$($echo.Length) expected=$BridgeBytes. See $artifacts"
    }
    $ticks = [regex]::Matches($r4Text, 'tick=\d+ n2=\d+ tx2=\d+ ov2=\d+ er2=\d+ n9=\d+ tx9=\d+ ov9=\d+ er9=\d+ stale=\d+ routes=\d+ duplicates=\d+')
    if ($ticks.Count -lt 10) { throw "Heartbeat stalled: only $($ticks.Count) ticks." }
    foreach ($tick in $ticks) {
        if ($tick.Value -notmatch 'ov2=0 er2=0 .*ov9=0 er9=0 stale=0 routes=6 duplicates=0$') {
            throw "UART/route error: $($tick.Value)"
        }
    }
    $intervals = @()
    for ($i = 1; $i -lt $tickTimes.Count; ++$i) {
        $interval = $tickTimes[$i] - $tickTimes[$i - 1]
        $intervals += $interval
        if ($tickNumbers[$i] -ne $tickNumbers[$i - 1] + 1 -or $interval -lt 0.75 -or $interval -gt 1.25) {
            throw "Heartbeat discontinuity: $interval seconds between ticks $($tickNumbers[$i-1]) and $($tickNumbers[$i])."
        }
    }
    $last = $ticks[$ticks.Count - 1].Value
    if ($last -notmatch "n2=$expected2 .*n9=$expected9 ") { throw "Incomplete RX counters: $last" }
    Write-Host $last
    $timing = $intervals | Measure-Object -Minimum -Maximum
    Write-Host ('Heartbeat intervals: {0:F3}..{1:F3} seconds' -f $timing.Minimum,$timing.Maximum)
    Write-Host "PASS: SCI2 8 x 4096 continuous bytes; SCI9 $BridgeBytes paced bytes; exact uppercase echo, zero errors/overflows; $($ticks.Count) heartbeat ticks."
    Write-Host "Logs: $artifacts"
} finally {
    Pop-Location
}
