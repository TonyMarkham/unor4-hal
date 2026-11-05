# unor4-hal — hardware validation record

The witness log: every slice of the HAL, proven on a real UNO R4 WiFi, with
the build, flash and check steps that reproduce it, and the two-board
validation rig (R3 controller + R4 responder + the `validate` host CLI) that
re-proves all of them in one run. Moved here from the README on 2026-11-04;
the content below is unchanged from that move. How to *use* the HAL is in
[README.md](README.md).

## Testing on hardware

One subsection per slice, updated in the same commit as the slice — the
durable record of "the board shows it". Each subsection is complete:
build, convert, flash, evaluate. The COM port is discovered once in the
preamble below and set as `$PORT` — nothing in the
per-slice commands is machine-specific.

> [!NOTE]
> **2026-10-20:** the per-slice programs and R3 sketches were retired in
> favour of the validation pair (All-in-one validation, at the end of this
> section). One `validate all` run re-proves Slices 1–11 (Slice 10's
> power cycles excepted: they need hands, `validate s10 --watch`).
>
> The Slice 1–8 subsections below are the historical record. To repeat
> one exactly as written, run its commands in a worktree of commit
> `6649d4c`, where every program, script and sketch it names still exists:
>
> ```powershell
> git worktree add ..\unor4-hal-6649d4c 6649d4c
> ```
>
> The Slice 9 per-slice programs were never committed, so they cannot be
> recovered.

Two rules for every upload, from the course's verified procedure:

- **Never upload cold** — double-tap the reset button first (LED fades =
  bootloader mode), then re-check the port with `arduino-cli board list`.
- If the upload dies mid-write with repeated `SAM-BA operation failed`,
  the DFU escape hatch in the course repo's `Basics.md` recovers it.

> ## Preamble
> **Before either test** (run after the double-tap reset: the
> bootloader can enumerate a different COM number than the sketch).
>
> Setting `$PORT` (the R4, used by every upload) and `$uartPort` (the
> slice 3 bridge R3) here feeds every command below. Boards are picked
> by board name, not by port position — a second connected board cannot
> hijack `$PORT`.
> If a later flash fails on the port, re-run the preamble and flash again.
> The flashing flow is verified on Windows PowerShell
> On macOS/Linux the same steps apply, with
> the port discovered as `PORT=$(arduino-cli board list --format json | jq -r '.ports[0].address')`.
>
> > ### List the boards and find the R4's port:
> > ```powershell
> > arduino-cli board list
> > ```
> > > Example Result:
> > > ```text
> > > Port    Protocol  Type                 Board Name           FQBN                           Core
> > > COM3    serial    Arduino UNO R4 WiFi  Arduino UNO R4 WiFi  arduino:renesas_uno:unor4wifi  arduino:renesas_uno
> > > COM4    serial    Arduino UNO          Arduino UNO          arduino:avr:uno                arduino:avr
> > > ```
> >
>
> > ### Set the port and verify it:
> > ```powershell
> > $PORT = (arduino-cli board list --format json | ConvertFrom-Json).detected_ports | Where-Object { $_.matching_boards.fqbn -eq "arduino:renesas_uno:unor4wifi" } | Select-Object -First 1 | ForEach-Object { $_.port.address }
> > ```
> > ### Proof:
> > ```pwsh
> > $PORT
> > ```
> > > ### Sample Result:
> > > ```pwsh
> > > COM3
> > > ```
>
> > ### List the boards and find the bridge R3's port:
> > ```powershell
> > arduino-cli board list
> > ```
> > > Example Result:
> > > ```text
> > > Port    Protocol  Type                 Board Name           FQBN                           Core
> > > COM3    serial    Arduino UNO R4 WiFi  Arduino UNO R4 WiFi  arduino:renesas_uno:unor4wifi  arduino:renesas_uno
> > > COM4    serial    Arduino UNO          Arduino UNO          arduino:avr:uno                arduino:avr
> > > ```
> >
>
> > ### Set the bridge port and verify it:
> > ```powershell
> > $uartPort = (arduino-cli board list --format json | ConvertFrom-Json).detected_ports | Where-Object { $_.matching_boards.fqbn -eq "arduino:avr:uno" } | Select-Object -First 1 | ForEach-Object { $_.port.address }
> > ```
> > ### Proof:
> > ```pwsh
> > $uartPort
> > ```
> > > ### Sample Result:
> > > ```pwsh
> > > COM4
> > > ```

> ### Slice 1 — blink (`slice01_blink`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S1.
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> Compiles the firmware in release mode (size-optimized, LTO) and
> prints the final `text` size — the budget recorded in the binary's
> doc comment:
>
> ```pwsh
> cargo build --release --bin slice01_blink
> ```
>
> Converts the linked ELF into a raw binary image — the file format
> `arduino-cli upload` consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice01_blink -- -O binary slice01_blink.bin
> ```
>
> **Flash** — double-tap reset first (LED fade = bootloader mode), then:
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice01_blink.bin
> ```
>
> (Minima board: `--fqbn arduino:renesas_uno:minima`.)
>
> **Evaluate:**
>
> - **Pass:** the onboard LED (D13) blinks at exactly 1 Hz — 1 s on, 1 s
    >   off, steady, indefinitely.
> - **Fail → no blink at all:** re-check that the upload reported
    >   success; try the never-upload-cold reset again.
> - **Fail → blinks at the wrong speed:** the clock or SysTick init is
    >   wrong — stop and investigate before any other slice.
>
> **Verified:** see the binary's doc comment for the recorded `text`
> size (1,584 bytes at slice 1).

> ### Slice 2 — button (`slice02_button`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S2.
>
> > [!IMPORTANT]
> > **Wiring:** a pushbutton between **D2 and GND** — nothing else. The
> > internal pull-up idles the line high, so pressed = low.
>
> **Build and convert** (from `crates/test-firmware/`, WiFi map):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> Compiles the button-mirror firmware for the WiFi pin map:
>
> ```powershell
> cargo build --release --bin slice02_button
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
>
> **Flash** (never-cold rule applies):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> **Evaluate:**
>
> - **Pass:** LED off at rest; press the button → LED on; release → LED
    >   off. Repeatable, immediate, no visible flicker.
> - Contact bounce is invisible — a *level* mirror has no edges to
    >   double-count; that is expected, not a bug.
> - **Fail → LED always on:** button wired to 5 V instead of GND, or the
    >   wrong header pin.
> - **Fail → LED never reacts:** confirm the WiFi image is flashed and
    >   the button is on D2 (P104 on the WiFi map).
>
> **Minima-map cross-check (optional but instructive):** build, convert
> and flash the Minima map — the same code now listens on P105, which on
> a WiFi board is header **D3**.
>
> Rebuilds the same firmware with the `minima` pin map selected:
>
> ```pwsh
> cargo build --release --no-default-features --features minima --bin slice02_button
> ```
>
> Overwrites `slice02_button.bin` with the Minima-map image (same
> output name, different pin table):
>
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
>
> Flashes the Minima-map image over the still-set `$PORT`:
>
> ```pwsh
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> Move the button from D2 to D3: it works again. That is the pin-map
> switch, observed on hardware. **Then restore the WiFi image.**
>
> Rebuilds and overwrites `slice02_button.bin` with the default
> (WiFi-map) image:
>
> ```powershell
> cargo build --release --bin slice02_button
> ```
>
> Converts the restored WiFi-map ELF:
>
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
>
> Re-flashes the WiFi map so the board is back to its standard state
> (button on D2 again):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> **Verified:** pending hardware run.

> ### Slice 3 - UART SCI2 (`slice03_sci2`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S3.
>
> > [!IMPORTANT]
> > **Test setup** (no USB-UART adapter needed): an R3
> > (`arduino:avr:uno`) runs the relay sketch
> > `arduino/sketches/slice03-adapter/slice03-adapter.ino` and acts as
> > the USB-UART bridge. Wiring, 3 wires:
> >
> > - R3 **D0** <-> R4 **D1** (P302, the R4's TX)
> > - R3 **D1** <-> R4 **D0** (P301, the R4's RX)
> > - R3 **GND** <-> R4 **GND**
> >
> > The relay forwards both directions: anything the R4 sends appears
> > in the monitor on the R3's USB port, and anything typed in the
> > monitor goes to the R4. Monitor runs at **115200 8N1**.
>
> **Install the bridge sketch** (one-time, from the repo root; the
> bridge R3's `$uartPort` comes from the Preamble).
>
> Compiles the relay sketch for the R3 (AVR target):
>
> ```powershell
> arduino-cli compile --fqbn arduino:avr:uno arduino/sketches/slice03-adapter
> ```
>
> Installs it on the bridge R3:
>
> ```powershell
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno arduino/sketches/slice03-adapter
> ```
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command below from `crates/test-firmware/`
>
> Compiles the SCI2 polling firmware (release, size-optimized):
>
> ```powershell
> cargo build --release --bin slice03_sci2
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice03_sci2 -- -O binary slice03_sci2.bin
> ```
>
> **Flash** the R4 (upload port `$PORT` from the Preamble):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice03_sci2.bin
> ```
>
> **Evaluate:** open the **bridge R3's** port at 115200 (not the R4's
> own USB port - this slice talks on the header pins):
>
> ```powershell
> arduino-cli monitor -p $uartPort --config baudrate=115200
> ```
>
> Then plug in (or reset) the R4 and watch the monitor.
>
> - **Pass:** one garbage byte at plug-in (expected — see below), then
    >   a boot banner and `tick N` once per second:
>
>   ```text
>   �unor4-hal slice 03 - SCI2 polling, 115200 8N1
>   tick 0
>   tick 1
>   tick 2
>   ```
>
>   Let it run for **10 minutes**: no garbage, no hang, no missing
    >   seconds.
> - **Expected, not a fail - one garbage byte before the banner when
    >   the R4 is plugged in:** while the R4's power rail ramps up, its
    >   unpowered TX pin clamps the shared line low and the receiving
    >   UART latches one junk byte. This happens before any code runs on
    >   the R4 and would appear with any firmware (or a bare USB-UART
    >   adapter). Exactly one byte at plug-in = pass; garbage *during*
    >   the tick stream = fail.
> - **Fail - nothing at all:** check the three wires (D0<->D1 crossed
    >   as listed above, GND common) and that the relay sketch is on the
    >   bridge R3.
> - **Fail - garbage characters in the tick stream:** baud mismatch.
    >   Run `cargo test` - the calculator's 3% tolerance is proven on the
    >   host, so suspect the monitor's configured rate first.
>
> **Verified:** host-verified (9 baud-calculator tests, both board-map
> builds, `text` = 4,084 bytes); boot banner + ticks observed on the
> bridge (2026-10-18); 10-minute soak run pending.

> ### Slice 4 — UART SCI9, the monitor path (`slice04_sci9`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S4.
>
> > [!IMPORTANT]
> > **Test setup** (no adapter, no second board): SCI9 talks on the
> > board-internal **P109 (TX) / P110 (RX)** pair, which runs through
> > the level translator to the ESP32-S3. The bridge firmware forwards
> > that line to the board's own USB port — the same port `$PORT` from
> > the Preamble. Monitor at **115200 8N1**.
> >
> > **Never open this port at 1200 or 2400 baud.** Those are the
> > bridge's magic rates: 1200 resets the RA4M1, 2400 forces it into
> > DFU boot mode (observed in the bridge firmware's USB line-coding
> > handler).
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> Compiles the SCI9 polling firmware (release, size-optimized):
>
> ```powershell
> cargo build --release --bin slice04_sci9
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice04_sci9 -- -O binary slice04_sci9.bin
> ```
>
> **Flash** (never-cold rule applies; `$PORT` from the Preamble):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice04_sci9.bin
> ```
>
> **Evaluate:** open the **R4's own port** (the bridge CDC port — the
> same `$PORT` the upload used) at 115200:
>
> ```powershell
> arduino-cli monitor -p $PORT --config baudrate=115200
> ```
>
> Then reset the board and watch the monitor.
>
> - **Pass:** a boot banner and `tick N` once per second, with no
    >   adapter and no second board connected:
>
>   ```text
>   unor4-hal slice 04 - SCI9 polling, 115200 8N1
>   tick 0
>   tick 1
>   tick 2
>   ```
>
> - **Expected, not a fail:** bridge/ESP32 diagnostic noise around
    >   reset — the bridge shares this port with its own logs. Garbage
    >   *during* the tick stream = baud mismatch (the bridge reconfigures
    >   its UART to the monitor's baud, so suspect the monitor config
    >   first).
> - **Fail — nothing at all:** confirm the WiFi image is flashed and
    >   the monitor is on the R4's port, not a second board.
>
> **This line is also the upload path.** The bootloader talks to the
> bridge over the same SCI9 pins, so the run is only complete once
> flashing is proven intact: after the tick stream, re-flash this same
> binary (double-tap reset, upload again). If the upload succeeds, the
> shared wire survived — record it under **Verified** below.
>
> **Verified:** pending hardware run.

> ### Slice 5 — interrupts: IRQ-mode UART echo + heartbeat (`slice05_echo`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S5.
>
> **Automated two-board regression (`slice05_rig`) — preferred:** connect
> R3 D0 ↔ R4 D1, R3 D1 ↔ R4 D0, D3 ↔ D3, and GND ↔ GND.
> Both boards remain connected over USB. The R4's D1 is TX and D0 is RX;
> the same is true for the R3. The controller uses D3 as an active-low
> test gate. No D2 button is needed for this automated binary.
>
> From `crates/test-firmware/`:
>
> ```powershell
> powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice05.ps1
> ```
>
> The execution-policy override applies only to this PowerShell process.
> The script discovers both ports by FQBN using `arduino-cli board list`,
> builds the reference/controller sketches and Rust firmware, and uploads
> all three in order. The reference sketch first leaves the R4's D0/D1
> as inputs so its TX driver cannot interfere with the R3 bootloader.
> Never send host commands to the R3 port while the R4 drives its RX pin.
>
> - `slice05-reference`: Arduino-core SCI9 register dump and echo
>   reference at 9600; also the safe staging image for R3 uploads.
> - `slice05-controller`: autonomous R3 checker at 115200.
>   It generates eight **continuous 4096-byte UART bursts**, receives the
>   uppercase echoes simultaneously, and compares every returned byte.
> - `slice05_rig`: both SCI2 and SCI9 in IRQ mode at 115200, both using
>   the public default rings (256 RX/128 TX). The main loop toggles
>   D13 every half-second using elapsed SysTick cycles, not loop counts.
> - The host independently sends 4096 paced bytes through the SCI9 USB
>   bridge and checks the complete uppercase response. Both paths must
>   have zero receive errors/overflows, no duplicate/inherited ICU routes,
>   and consecutive heartbeat reports about one second apart.
>
> The ESP32-S3 bridge dropped host USB bursts in this setup; that is not
> a HAL ring-size test. The unpaced burst test therefore uses the R3 wire,
> while SCI9 RX is checked separately with individually paced USB bytes.
> `arduino-cli monitor` local echo is not evidence of a board response.
>
> Logs are saved under `target/slice05-hardware/{r4,uno}.log`. To repeat
> against the already-flashed pair, add `-SkipBuild -SkipUpload`.
>
> **Verified on 2026-10-19:** R4 `COM3`, R3 `COM4`, Arduino
> renesas_uno core 1.6.0. All eight rounds returned `sent=4096 received=4096
> mismatches=0` (32768 bytes); concurrent SCI9 echo matched all 4096 bytes.
> Both UART error/overflow counters stayed zero, `stale=0 routes=6
> duplicates=0`, and 25 heartbeat reports continued with observed arrival
> intervals of 0.993–1.016 seconds using the default rings. Repeated uploads succeeded without
> disconnecting the wires. LED electrical/visual timing was not measured;
> the automated heartbeat timing assertion measures serial report arrival.
>
> **Bugs fixed:** the bootloader left SCI9 event links in ICU slots 4–7.
> Linking those events again in slots 0–3 violated RA4M1 hardware manual
> R01UH0887EJ0100 §13.4.2 and caused one-shot IRQ delivery. HAL startup now
> clears inherited routes/NVIC state, and linking rejects duplicate events.
> A second race primed TDR while TXI already owned it, overwriting bytes;
> priming now requires idle TX interrupts. Per-byte TIE toggles, ISR polling,
> the bit-band experiment, and the unused TEI interrupt were removed.
>
> **Application startup:** after attaching IRQ peripherals, main calls
> `unsafe { unor4_hal::intc::enable_interrupts() };` outside any critical
> section. The bootloader leaves PRIMASK set. Enabling an individual IELSR
> slot no longer clears that global mask behind the caller's back.
>
> **Standalone manual example (`slice05_echo`):**
>
> > [!IMPORTANT]
> > **Test setup** (no adapter, no second board): the echo runs on the
> > same board-internal SCI9 monitor line as slice 4 — **P109 (TX) /
> > P110 (RX)**, forwarded by the ESP32-S3 bridge to the board's own
> > USB port (`$PORT` from the Preamble). Monitor at **115200 8N1**.
> > The same never-open-at-1200/2400-baud rule as slice 4 applies.
> > Wiring: the slice 2 pushbutton between **D2 and GND**. The firmware
> > waits until D2 reads low; it does not detect monitor attachment or
> > require a new button-press edge. This is not a host/board handshake.
> >
> > This binary needs the `uart-irq` feature (it is not a default):
> > the build command below enables it.
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> > >
> > > Run every command in this section from `crates/test-firmware/`
>
> Compiles the IRQ-mode echo firmware (release, size-optimized):
>
> ```powershell
> cargo build --release --bin slice05_echo --features uart-irq
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice05_echo --features uart-irq -- -O binary slice05_echo.bin
> ```
>
> **Flash** (never-cold rule applies; `$PORT` from the Preamble):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice05_echo.bin
> ```
>
> **Evaluate:** upload, then open the R4's own port at 115200 (the same
> `$PORT` the upload used):
>
> ```powershell
> arduino-cli monitor -p $PORT --config baudrate=115200
> ```
>
> If the firmware is still waiting at its D2 gate, **press the D2
> pushbutton** after connecting the monitor. If the gate has already
> passed, ticks are already running and the one-time banner may have
> been sent before the monitor connected. The firmware has no monitor-
> attachment handshake or command to restart its tick count.
>
> - **Manual monitor check — ongoing heartbeat and counters:**
>
>   ```text
>   tick 10 ov=0 er=0
>   tick 11 ov=0 er=0
>   tick 12 ov=0 er=0
>   ```
>
>   The first visible tick need not be 1. Missing the banner is not, by
>   itself, a failure. Check that ticks continue and observe D13 for the
>   heartbeat; the main loop schedules its toggles every half-second.
> - **Typed text is not an echo test:** `arduino-cli monitor` locally
>   displays input, so seeing lowercase text such as `tony` does not
>   establish that the RA4M1 received or returned it. Typed text and board
>   output can share a display line; do not expect the clean separate
>   `hello` / `HELLO` transcript previously shown here. The firmware
>   uppercases bytes it actually receives, but this monitor command does
>   not establish instant per-keystroke delivery through the host/bridge.
>   Use the automated two-board test for verified byte-for-byte SCI9 echo.
> - **Counter interpretation:** `ov=0 er=0` means no RX-ring overflow or
>   receive-error event has been counted. It does not prove that typed
>   bytes reached the board or were echoed successfully.
> - **Burst verification:** use the automated two-board test above.
>   Pasting 4 KB into this bridge's monitor does not reliably deliver
>   those bytes to the RA4M1, regardless of the HAL's ring capacity.
> - **Expected around reset:** bridge/ESP32 diagnostic noise, as in
>   slice 4. Continued malformed board output or increasing counters
>   needs investigation; do not classify it as harmless reset noise.
> - **Fail — no echo but ticks fine:** distinguish host/bridge delivery
>   from the HAL receive path using the automated test and its counters.
> - **Fail — echo works but LED stutters:** investigate foreground
>   blocking or excessive ISR time; the heartbeat uses elapsed cycles.
>
> **Regression note:** this slice replaces cortex-m-rt's generic
> 240-entry interrupt table with the HAL's real 32-entry IELSR table.
> Every earlier binary got *smaller* (the table shrank by 832 bytes and
> grew by 32 one-purpose trampolines): blink 1,584 → 1,452 `text`,
> SCI2/SCI9 4,084/4,088 → 3,960 (historical measurements). Current WiFi
> release sizes from `cargo size --release --bin <name> --features uart-irq`:
> `slice05_echo`: text=9064, data=0, bss=668;
> `slice05_rig`: text=11508, data=0, bss=1072 bytes.
>
> **Verification scope:** automated rig verified above. The owner's
> standalone monitor capture showed ticks 9–15 with `ov=0 er=0` and mixed
> input/banner/heartbeat text on the first displayed line. That capture
> confirms ongoing heartbeat reports, not uppercase RX echo or D2-gate
> timing. The standalone button interaction remains unverified.

> ### Slice 6 — the WiFi AT channel (`slice06_at`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S6.
>
> Proves the mission-critical claim: the RA4M1 drives **SCI1**
> (P501 = TX, P502 = RX, board-internal to the ESP32-S3) and the
> ESP32-S3's AT server answers. The firmware sends `AT\r\n` once every
> 5 seconds for 30 seconds per run and relays every received byte to
> the SCI9 monitor path, with non-printables shown as `\xHH`.
>
> **Hardware facts**:
> P501/P502 mux to TXD1/RXD1 at `PSEL[4:0] = 00101b` (manual
> R01UH0887EJ0100 §19, Table 19.13); `MSTPCRB.MSTPB30` releases SCI1
> (manual §10.2.3); the stock bridge firmware implements its AT server
> via SudoMaker/chAT at 115200 8N1 (`uno-r4-wifi-usb-bridge`,
> `UNOR4USBBridge.ino` — behavioral reference).
>
> **Test setup:** no adapter, no wiring beyond the bench. The R4's
> monitor is `$PORT` at **115200 8N1** (`arduino-cli monitor` asserts
> DTR, which the bridge's CDC requires for output — a raw `SerialPort`
> reader must set `DtrEnable` or it will capture nothing). The run is
> gated by the bench line R3 D3 → R4 D3: `slice06-gate` (on the R3) holds
> the line high 30 s, low 50 s, cycling; the R4
> firmware runs one test per falling edge and re-arms on the rising
> edge, so one flash serves many runs. Upload the R4 first (its D0/D1
> stay inputs; the R3's upload shares its own D0/D1 with the cross-wire).
>
> **Build and convert** (from `crates/test-firmware/`):
>
> ```powershell
> cargo build --release --bin slice06_at --features uart-irq
> cargo objcopy --release --bin slice06_at --features uart-irq -- -O binary slice06_at.bin
> ```
>
> **Flash** (never-cold rule applies; `$PORT` = the R4):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice06_at.bin
> arduino-cli compile --fqbn arduino:avr:uno ..\..\arduino\sketches\slice06-gate
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno ..\..\arduino\sketches\slice06-gate
> ```
>
> **Evaluate:** open the R4's port at 115200 during a high phase (dots
> mark the waiting firmware); the next falling edge prints:
>
> ```text
> gate open
> giving the ESP32-S3 AT server 2 s to settle...
> round 0: sending AT\r\n -> OK\x0d\x0a
> round 0 done: 4 bytes
> ...
> round 5: sending AT\r\n -> OK\x0d\x0a
> round 5 done: 4 bytes
> done: total=24 ov=0 er=0
> waiting for the gate to rise to re-arm
> ```
>
> The ESP32-S3 answers exactly `OK\r\n` — no command echo, no banner.
>
> **Verified on 2026-10-19:** R4 `COM3`, R3 `COM4`, Arduino renesas_uno
> core 1.6.0. One complete run (6/6 rounds) and one partial capture
> (rounds 3–5, 3/3) — **9/9 witnessed rounds** answered exactly
> `OK\x0d\x0a`, `ov=0 er=0` throughout; the partial run's own report
> (`total=24`) shows its unwitnessed rounds also received 4 bytes each.
> Uploads succeeded before and after the runs over the same
> USB path. WiFi map only — P501/P502 exist on the Minima on its SWD
> connector, untested there. Size (WiFi, `uart-irq`): `slice06_at`
> text=14888, data=0, bss=668.
>
> **Fail — silence after `gate open`:** the AT server runs from the
> ESP32-S3's boot; power-cycle the whole board once (fresh ESP32 boot)
> before investigating further, then check `er=` (nonzero suggests a
> baud/format mismatch) and the PSEL value.
>
> **Fail — dots but no gate open:** the R3 side is not driving D3 low;
> verify the gate sketch is on the R3 and the D3↔D3 wire is seated.
>
> ### Slice 7 — PWM, the LED breathes (`slice07_breathe`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S7.
>
> Proves the GPT PWM driver: saw-wave PWM on **D6** at exactly 1 kHz
> (PCLKD/1, `GTPR = 47999`, 48 000 duty steps) with a sine duty profile,
> while the D13 heartbeat keeps ticking. The measurement is instrumented:
> D6 is wired to the R3's **D2 (INT0)** and the R3 sketch
> `slice07-measure` measures duty and period edge-by-edge — no LED, no
> eyeballing. Per gate round (R3 D3 → R4 D3, the slice 6 pattern) the
> firmware runs a 1 s settle, a duty staircase (0/25/50/75/100%, 2 s per
> step — the two extremes exercise `GTUDDTYC.OADTY` and the
> never-matching-compare mechanisms), then sine breathing (2.048 s
> cycle) until the next gate edge.
>
> **Hardware facts**:
> WiFi D6 = P111 → `GTIOC3A` on GPT163 at `PSEL[4:0] = 00011b` (manual
> §19 Tables 19.6/19.7/19.10/19.12, cross-checked entry-for-entry
> against the Arduino core's `pinmux.inc`); `MSTPCRD.MSTPD6` releases
> GPT162–167 (manual §10, p.175); the count clock is PCLKD = 48 MHz
> (manual §22.1/§8 Table 8.2); saw period = `GTPR + 1` (§22.2.20); duty
> 0%/100% via `GTUDDTYC.OADTY` / never-matching compare, because a
> compare match at the cycle end loses to the cycle-end setting in
> saw-wave mode (Table 22.5 Note 2, §22.3.6). The public trait is
> `embedded-hal` 1.0 `SetDutyCycle` (the 0.2
> multi-channel `Pwm` trait does not exist in 1.0).
>
> *Superseded 2026-11-04 by the header pin model (CHANGELOG 0.2.0): one
> `Pwm::new(timer, pin, …)` for any of the 16 headers with a GPT output,
> the channel and output read from the pin's `Gtioc` facts;
> `src/tests/pwm.rs` now locks the six classic headers' facts. The text
> below is the slice as it was proven.*
>
> **The full PWM surface:** all six header PWM pins are
> addressable, named by header pin with the engine/pin/output fixed at
> compile time — `Pwm::d3`, `d5`, `d6`, `d9`, `d10`, `d11` (D9 shares
> one constructor across both maps; the rest are per-map, like the pin
> tables). The mapping table is locked by the golden constructor test
> (`src/tests/pwm.rs`), which asserts every constructor's exact
> signature against manual §19 Tables 19.6/19.7/19.10/19.12. Hardware
> proof: WiFi D6 (GPT163) — the register recipe is common code, but the
> other pin/engine pairs have no bench wires and are compile-checked
> only. (D8 also carries `GTIOC7A` per Table 19.10 — outside the six
> spec pins; one more constructor if ever wanted.)
>
> **Build and convert** (from `crates/test-firmware/`):
>
> ```powershell
> cargo build --release --bin slice07_breathe
> cargo objcopy --release --bin slice07_breathe -- -O binary slice07_breathe.bin
> ```
>
> **Flash** (R4 first, then the R3 rig; never-cold rule applies;
> `$PORT` = the R4, `$uartPort` = the R3):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice07_breathe.bin
> arduino-cli compile --fqbn arduino:avr:uno ..\..\arduino\sketches\slice07-measure
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno ..\..\arduino\sketches\slice07-measure
> ```
>
> **Evaluate:** open both ports at 115200. The R4 prints the staircase
> steps and continuous `tick N` heartbeats; the R3 prints, per 10 s
> window, twenty 500 ms duty samples (`H`/`L` = flat high/low — the
> DC-line evidence for the 0%/100% steps) plus the aggregate. One
> aligned round (R3 `W3` against the R4's second staircase):
>
> ```text
> R4: step 0 duty=0 of 48000 ... step 4 duty=48000 of 48000
> R3: 0 L L L 252 252 248 248 501 500 500 501 752 752 748 752 749 H H H
> R3: W3 duty=500/1000 (min 244 max 756) period=1000us edges=6013
> ```
>
> Reading: period exactly 1000 µs; 6013 edges = exactly the ~6 s of the
> three edged steps; every commanded duty lands within ±1 tick of
> 48 000 (±4 per-mille); the extremes are DC lines (no edges), which is
> the OQ-P2 answer; breathing windows sweep 59→1000 per-mille
> (their period mean reads 1007–1019 µs because the R3's ISR merges
> edges when the pulse gaps at the sine extremes drop below the ISR
> duration — a rig artifact, not a GPT defect).
>
> **Verified on 2026-10-19:** R4 `COM3`, R3 `COM4`, simultaneous
> capture of one aligned round plus breathing windows; duty staircase
> exact at 1 kHz, flat-line extremes confirmed, heartbeat continuous
> (ticks 456–516 with no gap) through gate/staircase/breathing. Uploads
> succeeded before and after. WiFi map verified; Minima image builds
> (`gpt0b`/P106 path) but is hardware-untested — no Minima on the bench.
> Size (WiFi): `slice07_breathe` text=12700, data=0, bss=4.
>
> **Fail — R3 windows show `period=1333us` and stale-looking samples:**
> the R3 was reset mid-round (its first window after a serial open
> inherits the R4's previous phase). The next aligned round is the
> evidence — check `W3` onward.
>
> **Fail — no `H`/`L` and duty stuck near one value:** the D6→D2 wire
> is off, or the R4 is not past its banner (check the R4 log for
> `gate open`).

> ### Slice 8 — ADC, the chip senses (`slice08_adc`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S8.
>
> Proves the ADC140 driver: single-shot, software-triggered, blocking
> reads on **A0** (raw + millivolts at 4 Hz) through the slice-4 SCI9
> monitor path, while the D13 heartbeat keeps ticking. The measurement
> is instrumented: R3 **D5** drives a PWM duty staircase straight into
> R4 **A0** (R3 sketch `slice08-source`), and per gate round
> (R3 D3 → R4 D3, the slice 6/7 pattern) the firmware averages five
> 1.5 s windows — 631 650 samples each — one window per duty step. The
> mean of a PWM wave over whole periods is duty × VREF, so the window
> means trace the staircase with no RC filter.
>
> **Hardware facts**:
> A0–A5 are the same chip pins on both boards — A0 = P014 → **AN009**
> (also DA0), A1 = P000 → AN000, A2 = P001 → AN001, A3 = P002 → AN002,
> A4 = P101 → AN021, A5 = P100 → AN022 (manual §19 Tables 19.5/19.6
> `ASEL` rows, cross-checked entry-for-entry against the Arduino
> core's `pinmux.inc`; the golden test in `src/tests/adc.rs` locks the
> table). `MSTPCRD.MSTPD16` releases the ADC140 (manual §10, p.175)
> with the §35.8.5 ≥ 1 µs settle; the software trigger needs no
> trigger setup (`ADCSR.ADST` works regardless of `TRGE`/`EXTRG`/
> `TRSA`, §35.2.12); `ADST` **self-clears** at scan end in single scan
> mode (§35.3.2.1 step 4) — the driver polls that self-clear, bounded;
> the reference is the reset selection AVCC0/AVSS0 (§35.2.31) = the
> board's 5 V analog rail; sampling states stay at the reset 0Dh,
> valid at this crate's 1:2 PCLKB:PCLKC ratio (≥ 6 states, §35.2.14).
> Conversion ≈ 1206 ns (12-bit high-speed, `calc_scan_time`, host
> tested). **There is no ADC trait in `embedded-hal` 1.0** — the
> driver exposes its own `read()` API (amended
> 2026-10-19), the same stance the UART module takes outside
> `embedded-io`.
>
> *Superseded 2026-11-04 by the header pin model (CHANGELOG 0.2.0):
> `Adc::read` takes any pin with an `AdcPin` channel from the pin table;
> `Channel` is gone. The text below is the slice as it was proven.*
>
> **The full analog surface:** all six header pins are
> addressable through the sealed `Channel` trait (one `const AN` per
> pin type; only analog-state pins type-check). Hardware proof: A0
> end-to-end (accuracy instrumented), and A1–A5 converted through the
> same driver during the bring-up diagnostic (~13 M conversions each,
> zero errors — their *values* float because nothing drives them;
> only A0's accuracy is measured).
>
> **Build and convert** (from `crates/test-firmware/`):
>
> ```powershell
> cargo build --release --bin slice08_adc
> cargo objcopy --release --bin slice08_adc -- -O binary slice08_adc.bin
> ```
>
> **Flash** (R4 first, then the R3 rig; never-cold rule applies;
> `$PORT` = the R4, `$uartPort` = the R3):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice08_adc.bin
> arduino-cli compile --fqbn arduino:avr:uno ..\..\arduino\sketches\slice08-source
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno ..\..\arduino\sketches\slice08-source
> ```
>
> **Evaluate:** open both ports at 115200. Between rounds the R4
> prints `a0 raw=… mv=…` at 4 Hz — with the sketch driving duty 0
> between rounds this sits at ~0 mV; jumper A0 → 5 V / 3V3 / GND for
> the rails check. Per round the R3 prints its commanded duties and
> the R4 prints the five window lines. One round (2026-10-20):
>
> ```text
> R3: duty 0 / duty 64 / duty 127 / duty 191 / duty 255
> R4: W0 n=631650 mean=7     min=0    max=53   err=0
>     W1 n=631650 mean=1045   min=0    max=4095 err=0
>     W2 n=631650 mean=2051   min=0    max=4095 err=0
>     W3 n=631650 mean=3072   min=0    max=4095 err=0
>     W4 n=631650 mean=4095   min=4095 max=4095 err=0
> ```
>
> Reading: the staircase 0 → 1045 → 2051 → 3072 → 4095 against the
> expected duty/255 × 4095 = 0 → 1028 → 2040 → 3068 → 4095; worst
> error +17 LSB ≈ 21 mV, inside the datasheet's ±32 LSB (±39 mV) max
> for A0's precision class (and W1–W3 are inside the ±20 LSB
> high-precision line); window means repeat within ±2 LSB across
> rounds; W0/W4 are DC lines (rail-to-rail ends solid), W1–W3's
> min/max 0/4095 is the instantaneous PWM ripple the averaging is
> for. Heartbeat continuous (`tick N` every second through gate,
> staircase, and windows).
>
> **Verified on 2026-10-20:** R4 `COM3`, R3 `COM4`, three aligned
> rounds captured simultaneously; staircase within tolerance at both
> rails (0 and 4095 exactly — the datasheet-tolerance done-when),
> ~10 M conversions with `err=0`, uploads succeeded before and after.
> WiFi map verified; Minima image builds (A0–A5 are the same chip pins
> on both maps) but is hardware-untested — no Minima on the bench.
> Size (WiFi): `slice08_adc` text=15108, data=0, bss=4.
>
> **Fail — every analog reading wanders ~800–3400 raw and no `W`
> window is ever at a rail, while the R3 log shows the staircase:**
> the D5→A0 path is open. A solid duty-255 (5 V DC) can only read full
> scale on a connected pin, so this signature means the wire is not on
> the driven pin. Triage history: the R3 end was parked in **D4** —
> not a PWM pin on the ATmega328p (PWM: D3/D5/D6/D9/D10/D11) — moving
> it to D5 fixed everything in one round. `slice08_probe` (20-pin
> digital scan) finds where a signal actually lands if this recurs.
>
> **Fail — `a0 err=timeout`:** the ADC140 never completed a scan; dump
> `ADCSR` before theorizing (the poll bound is thousands of times the
> computed conversion time — reaching it is a fault, not a short
> wait).

### Slice 9 — native USB CDC (verified on WiFi)

> [!NOTE]
> **Historical record.** None of the Slice 9 programs and scripts named
> below were ever committed:
>
> - `slice09_cdc.rs` and `test-slice09-cdc.ps1`;
> - `slice09_rig.rs` and `test-slice09.ps1`;
> - `noop.rs` and the other `slice09_*` probes.
>
> All of them were deleted on 2026-10-20, so the commands in this
> subsection cannot be repeated. The results stand as recorded. The current
> Slice 9 check is `arduino/validation/` (S9, knock, `!`/`@`); its R4 image arms
> the same knock and serves the same echo and ticks.

**Slice binary:** `crates/test-firmware/src/bin/slice09_cdc.rs`. It prints a
banner on the SCI9 monitor, leaves the connector with the bridge for a 3 s
upload window, takes it, then enumerates as a native CDC port
(VID 2341 / PID 006D, product `UNO R4 slice09`). It echoes what it receives
and prints `tick N` about once per second. The opt-in bootloader knock is
armed.

Build from `crates/test-firmware/`, then upload through the bridge:

```powershell
cargo objcopy --release --features usb --bin slice09_cdc -- -O binary slice09_cdc.bin
arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice09_cdc.bin
```

Evaluate from `crates/test-firmware/`. The script finds the native port by
USB identity, not by a remembered COM number:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09-cdc.ps1 -Knock
```

It checks the following:

- **Ticks:** every line is `tick N`, the numbers are consecutive, and no gap
  exceeds 1.5 s.
- **Echo:** printable payloads of 1–4096 bytes come back exactly, with tick
  lines stripped first.
- **Knock (`-Knock`):** a 1200-baud open of the native port makes the
  native port disappear and the bridge (PID 1002) return.

**Uploading while the slice binary holds the connector** takes two steps.
Either open the native port at 1200 baud (the knock) or double-tap RESET.
Then run a standard `arduino-cli upload` on the bridge port that returns.

A single command aimed at the native port fails with
`No device found on COMx`, including with
`--upload-property upload.wait_for_upload_port=true`. The bootloader holds,
but the bridge forwards `bossac` only after its own 1200-baud touch.
Uploads to an R4 already held in bootloader also failed whenever that touch
was disabled. This was observed with both a knock hold and a physical
double-tap hold.

**Diagnostic rig, retired 2026-10-20.** None of its three parts was ever
committed, and all were deleted, so the rig cannot be rebuilt. The results
below were recorded with it.

The rig's parts were:

- R3: `slice09-rig.ino` (removed).
- R4: `slice09_rig.rs`, features `usb,uart-irq` (removed).
- Windows host: `test-slice09.ps1` (removed).

The R3 is the stable host endpoint while the R4 changes USB identity. The
pair negotiates its own diagnostic contract (`H` + raw version 9), then allows
one outstanding request. Replies carry a sequence, length, hex payload and
checksum. Hex encoding keeps register values from becoming commands.

Both R3 serial links run at **9600 8N1**. The R4 releases SCI2 TX (P302) to
input between replies.

**Shared-RX limitation:** the PC's bytes and the R4's TX share the R3's only
RX input. While any R4 program keeps SCI2 TX driven, PC-written R3 commands,
`!`/`@` included, do not reach the R3. This was reproduced with the R3
sketch `utility-req-res-reset` (since removed, never committed) +
`slice08_req_res_reset`: rows kept flowing and
`!!!`/`@@@` had no effect. With the R4 held in bootloader, `!!!` worked.
So the R3 reset path only works when the R4 releases or idles that pin.

R3 commands, sent without line endings:

| Command | Effect |
| --- | --- |
| `h` | renegotiate |
| `?` | snapshot |
| `u` | take native USB |
| `r` | detach and release |
| `t` | trace record |
| `v` | toggle native ticks |
| `w` | enable the USB interrupt bell |
| `b` | arm the bootloader knock |
| `x` | read installed bootloader bytes (read-only) |
| `y` | diagnostic: write the boot magic and read it back |
| `z` | diagnostic: bare R4 software reset |
| `!` | single RESET pulse |
| `@` | double RESET pulse |

The 94-byte snapshot also reports the reset cause captured at boot
(RSTSR0/2/1, manual §5.2) and the tick flag. The host script uses the tick
flag to switch ticks on or off rather than blindly toggling them.

Build/stage sequence, as it was run (none of it can be repeated now):

1. Upload `noop.bin` to the R4 first, so its UART TX cannot disturb the AVR
   bootloader.
2. Compile and upload the R3 sketch `slice09-rig` with `--verify`.
3. Upload `slice09_rig.bin` to the R4.

```powershell
cargo objcopy --release --features usb,uart-irq --bin slice09_rig -- -O binary slice09_rig.bin
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09.ps1 -Mode Echo -R3Port $uartPort
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09.ps1 -Mode Soak -R3Port $uartPort
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09.ps1 -Mode Echo -R3Port $uartPort -EnableWake
```

`Soak` is a functional tick run with a 30 s default. **Owner amendment,
2026-10-20:** the hour-long soak requirement is cancelled and no
replacement duration is required. Logs and results go to
`target/slice09-hardware/`.

**Witnessed on WiFi, 2026-10-20:**

`slice09_cdc`:

- Native COM port present while the bridge port was absent.
- 9 consecutive ticks (maximum gap 1.03 s) and 48 exact echoes
  (21,744 bytes each direction).
- Knock: native gone and bridge back after 1.19 s. A standard upload on the
  bridge then succeeded, and the native port returned with ticks and echo
  passing again.

`slice09_rig`:

- Polled echo: 210 exact cases, 73,450 bytes each direction.
- Interrupt bell: 210 cases, 2,850 bell interrupts.
- 30 s tick run: 29 consecutive ticks, maximum gap 1.07 s.
- SCI2 `ov=0 er=0` throughout.
- Two-step upload: knock, R4 silent (held) 3 s later, then a standard
  bridge upload succeeded.

R3 D4 → R4 RESET:

- `!` restarts the R4.
- `@` returns the bridge with the bootloader held.

Build matrix:

- Release firmware builds for WiFi and Minima, with and without
  `usb,uart-irq`, have no warnings.
- 58 host tests pass on each map.

**Bootloader knock fix.** The knock first failed: the R4 came back running
the application, with RSTSR1.SWRF = 1, so no RES-pin reset followed. The
cause:

- The installed bootloader is byte-identical to the `dfu_wifi.hex` that
  ArduinoCore-renesas 1.6.0 ships.
- At offset 0x21EA it compares one 32-bit load of `VBTBKR0..3` with
  `0x07738135`.
- On this silicon that load returns `VBTBKR0` as the most significant byte.
  Bytes written `35 81 73 07` read back as `0x35817307`.

The HAL now writes the magic big-endian, still with 8-bit accesses
(§11.2.6). After that change, a bare software reset holds the bootloader.

The same disassembly answers part of OQ-USB-1. The `VBTBKR[1] = 40` marker
that `UsbPort::take` writes asks the bootloader to drive P408 high, delay,
drive it low and clear the byte at the next boot.

Earlier on 2026-10-20, the R3 trace exposed two USB defects that were then
fixed:

- A control-OUT status acknowledgement must use CCPL even while CTSQ still
  reads data stage 3 (§27.2.25 / Figure 27.16).
- Class OUT pipes need arming after configuration.

**Physical replug, witnessed 2026-10-20 21:19:**

1. The owner unplugged and replugged the R4 with `slice09_cdc` installed.
2. The bridge appeared during the upload window, then native COM7.
3. Ticks (5 consecutive) and echo (16 exact cases) passed.
4. The knock returned the bridge after 1.2 s.
5. A standard `arduino-cli upload` on the bridge succeeded.
6. The native port came back and passed again.

**Still open:**

- UNO R4 Minima is hardware-unverified, including its knock.
- ~~`cargo doc` with `-D warnings` still fails on two pre-existing links
  outside USB (`adc/adc.rs` `ADST_POLL_LIMIT`, `pwm/mod.rs`
  `Pwm::set_duty_cycle`).~~ Fixed 2026-11-04 (CHANGELOG 0.2.0, Fixed).

### Slice 11 — step generator on a 2 ms hardware tick (verified on WiFi)

A DM542 stepper driver for the cart-pole rig on the UNO R4 WiFi: STEP on
D4 (GPT320), DIR on D5, an exact pulse count, and a 2 ms hardware tick that
applies the queued rate and direction and latches the count. The plan —
requirements, design, API, build steps, bench and open items — is
[`Slice11Plan.md`](Slice11Plan.md). Steps 1–5c below are each proven on the
board; one intermittent miscount is open (end of this subsection).

> **Superseded API (2026-11-04).** The records below name the API as it was
> built then: `StepGenerator::d4`, `StepConfig`, a GPT321 counter. That API
> is gone. A generator is now set up with validating builders (`PulPinBuilder`,
> `DirPinBuilder`, optional `EnaPinBuilder`, `CounterBuilder`, `TickBuilder`,
> `StepSetup`) and handed its hardware with `StepGenerator::builder`. How to
> use it — what each part supports, the per-tick loop, every error, a
> complete example — is in the crate's `step` module docs. The validator now
> builds its generator that way, with the counter on GPT162 (16-bit). The
> hardware behaviour recorded here is unchanged, and every check still
> passes (25/25, below).

**Step 1 — ELC driver (`elc` module), verified on WiFi 2026-11-03.**
`Elc::new` releases module stop (`MSTPCRC.MSTPC14`), clears `ELSR0`–`7`
and sets `ELCON`; `link`/`unlink` route an `ElcEvent` to a `GptInput`;
`fire` raises a software event. Proof (`S11 elc`, R4 request `L<n>`):
GPT321 counts up on `ELC_GPTA`, linked to `ELC_SWEVT0`; it counted
1/1, 100/100 and 10000/10000 software events, and none of 10 more fired
after `unlink` (the control: the count came through the route). The check
is R4-internal — the R3 only reads the reply. Host tests: 3 new (`elc`
table integrity against Table 18.2/18.3).

**Step 2 — step generator on D4 (`step` module), verified on WiFi
2026-11-03.** `StepGenerator::d4` claims GPT320 and P106 (`GTIOC0B`), pin
HIGH while stopped; `run(rate)` starts the train (`GTIOB = 10110b`: LOW
for `GTCCRB` counts at each period start), `stop()` stops at a pulse
boundary — at once while the pin is HIGH, else on the pulse's end through
the hardware stop (`GPT0_CCMPB` → ELC input B → `GTPSR.PSELCB`). Proof
(`S11 step`, R4 requests `T<rate>` and `Z<zone>`; the R3 timestamps every
falling edge on D8 with Timer1 input capture and times each LOW by polling
from a hardware-latched fall):

| Run | Measured | Want |
| --- | --- | --- |
| 1 pulse/s | period 1 000 779 µs, low 5.0 µs | 1 000 000 µs (the two boards' clocks differ by 0.08 %), 5.0 µs |
| 1 000/s | period 1 000.5–1 001.5 µs, low 4.5–6.0 µs (303 of 304 pulses timed), high ≥ 995 µs | 1 000 µs, 5.0 µs ± one 0.5 µs tick |
| 44 600/s | mean 22.44 µs, every period 22.0–22.5 µs, 4 643 edges | 22.42 µs (1 076 counts) |
| 55 733/s | mean 17.94 µs, every period 17.5–18.0 µs, 5 808 edges | 17.94 µs (861 counts) |
| 50 stops | all HIGH afterwards, no pulse after, no cut pulse: 10 each stopped mid-pulse (the hardware stop), just after a pulse and just before the next, at 1 000/s with every LOW timed; 20 at random phases at 55 733/s | — |

Limits of the measurement: the R3 times a LOW only when its interrupt starts
inside the pulse (it did for 303 of 304 at 1 000/s), and at 44 600/s and
above it measures periods only. The first run of this check timed the
interrupt's own delay instead of the pulse (the pin was read after the ISR's
arithmetic) and was discarded. Host tests: 6 new (`step` period maths).
The temporary `S11 wire` sanity check was retired: this check uses the same
wire with the real STEP output.

**Step 3 — exact pulse count, verified on WiFi 2026-11-03.**
`StepGenerator::d4` now also takes GPT321, which counts up on ELC input B —
the same `GPT0_CCMPB` the hardware stop uses — so it counts each pulse as it
ends, in hardware, with no CPU interrupt; `pulses()` reads it (32-bit,
wrapping). Proof (`S11 count`, R4 request `N`): 20 start/stop runs — 9 at
1 000/s stopped mid-pulse, just after and just before a pulse (Z1–Z3), 3 at
44 600/s and 3 at 55 733/s for 200 ms, 5 at 55 733/s for 1–30 ms with zone
stops — each compared the R4's count difference against the edges the R3
captured over the same run. 20/20 equal; totals 72 444 = 72 444 (an
earlier `slices S11` run: 72 863 = 72 863). The R3's own count was checked
for a missed edge in every run (no interval ≥ 1.5 periods) and lost none.
Step 1's software-event check moved from GPT321 to GPT166 (16-bit), since
GPT321 now belongs to the step generator.

**Step 4 — 2 ms tick and latch, verified on WiFi 2026-11-03.** GPT164
overflows every 96 000 PCLKD counts (PCLKD/4, `GTPR = 23 999`); its
`GPT4_OVF` goes to ELC input C, where GPT321 captures the pulse count and
GPT320 its own counter (the phase) at the same instant, and GPT165 counts
the ticks (one more channel than the plan listed, so a skipped poll shows).
`poll_tick()` returns the latest latch as a `TickReport` (tick number,
pulses, phase, skipped); `predicted_pulses()` gives the count expected at
the next tick, `None` when it cannot be exact (no tick yet, or a
`run`/`stop` since the previous report). Proof (`S11 tick`, R4 requests
`Y<rate>` for 60 ticks and `V<rate>` for 2 500): every tick's latch must
equal the previous tick's prediction; per-tick changes must stay within
rate × 2 ms ± 1; the run's total count must equal the R3's edges.

- **Found on the board:** a pulse end reaches the tick latch 2 PCLKD counts
  after GPT320 reaches `GTCCRB`, as seen through GPT320's captured phase. The
  first model (0 counts) mispredicted ticks whose phase was `low + 1` or
  `low + 2` (always one pulse too many). The R4 then scored models of 0–4
  counts side by side, with 44 486/s added (1 079 counts: the phase visits
  every value within 2.2 s, so every boundary is met each run). In 27 runs,
  six of them full sweeps, only 2 counts was exact every time (0 failed in
  8 runs, 1 in 6, 3 in 10, 4 in 11). `PULSE_END_LATCH_DELAY = 2` records it
  as measured; the manual gives no figure, and how the 2 counts split
  between the compare event, the count's synchronisation and the phase
  capture is unverified.
- **With the 2-count model:** 3 × 9 runs exact (0, 1 000, 44 600, 55 733,
  12 345/s for 60 ticks; 5 s each at 44 600, 55 733 and twice at 44 486/s:
  2 498 predictions per stress run), no tick skipped, the R4 total equal to
  the R3's edges in every run (up to 278 621 pulses).
- **Limit:** the tick's period is the R4's own claim in this step — no tick
  pin reaches the R3. Step 5a checks it from outside.
- **Also fixed here (validator):** the `Z` zone-stop windows were 8 counts
  wide and the R4's read loop could step over them at 55 733/s; the stop
  then never happened (`Z ERR`), and the R3 check took `Z ERR` for a stop.
  The windows are now 24+ counts and the R3 requires the zone number.

**Step 5a — rate applied at the tick by the DTC, verified on WiFi
2026-11-03.** `queue(rate)` arms a one-shot DTC transfer on the tick's ICU
slot (`GPT4_OVF`, `IELSRn.DTCE`; the CPU never takes the interrupt): at the
tick instant the DTC writes GPT320 `GTPR` (the period in progress) and
`GTPBR` (the new steady period, reloaded at every overflow, `GTBER.PR`),
then reads `GTCNT` back in a chained transfer. The period in progress
follows the agreed rule: the next pulse one new period after the last one,
or `min_high` after the tick if that has passed. The DTC clears `DTCE`
itself after the transfer, so an unarmed tick writes nothing. `Peripherals`
gains `dtc`; the HAL tracks the period sequence at each tick as a
`Schedule` (in progress, next, steady), and `predicted_pulses()` uses it.

- **Measured first — the DTC's write latency:** 14–17 PCLKD counts
  (0.29–0.35 µs) after the tick, 3 × 500 samples (R4 request `X`). The
  written `GTPR` is always ≥ 120 counts past the counter, so the write is
  always in time.
- **The one race, handled by observation:** when the old period ends inside
  those ~17 counts, its pulse fires before the write, and the transition
  period then follows that pulse. The chained read-back tells which order
  happened (`TickReport::old_pulse_first`), and the schedule follows it, so
  predictions stay exact either way.
- **Proof (`S11 queue`, R4 request `Q<mode>`):**

  | Run | Result |
  | --- | --- |
  | Q1 ×2: 960/s and 44 600/s alternating, 40 ticks | 40/40 applied; every count and phase prediction exact; the R3 timed the 20 burst starts (each `min_high` after a tick) 4 002.0–4 003.5 µs apart — 2 ticks of 2 ms on the R3's clock, which runs 0.08 % apart from the R4's (step 2's 1 pulse/s run) |
  | Q2 ×2: ±2 421/s ramp, 1 000 → 44 578 → 1 000/s, 37 ticks | 37/37 applied, every prediction exact |
  | Q3: a different rate every tick, 30 000–55 733/s, 2 500 ticks | 2 500/2 500 applied, every prediction exact; the old pulse fired first at 50 ticks (53 in an earlier run) |

  In every run R4 count = R3 edges (up to 213 916 pulses) and no interval
  was shorter than 17.5 µs.
- **Deviation from the agreed rule (accepted by cart-pole 2026-11-04):** at a tick
  where the old period ends inside the DTC's write latency (2 % of ticks in
  Q3), the old pulse fires about 0.3 µs after the tick and the transition
  period starts from it, so that one period runs longer than the rule asks
  (by up to the old period's remainder plus 2.5 µs). The hardware gives no
  way to write before a pulse that is already firing; the outcome is known
  and predicted exactly.
- **Found on the way (HAL):** the first version marked the schedule lost
  whenever ticks went unpolled, including the idle gap before a run, so it
  never became known and the `Q` loop hung the R4. Only unpolled ticks
  while running unchanged lose it now, and `Q` gives up after 10 ticks.

**Step 5c — rate 0, held ticks, idle stop, start from stopped, trip stop;
verified on WiFi 2026-11-03.** `queue(0)` is applied at the tick like any
rate, as a ~89 s period: no further pulse, while the counter keeps the time
since the last one, so a later rate still follows the agreed rule. After
1 s idle, `poll_tick` stops the counter (long before it could wrap).
`queue(rate)` on a stopped generator arms a hardware start at the tick
(`GTSSR`, ELC input C), first pulse `min_high` after it. A tick without a
queued rate holds the running one; `period_counts()` gives what runs.
`stop()` (the trip) also cancels a queued start or change. Proof
(`S11 scenario`, R4 request `O`, 618 ticks): ramp 5 000 → 20 000/s; 0;
0 again; a tick with no queue; restart at 12 000/s from idle; two ticks with
no queue (held); 8 000/s; 0; 600 ticks idle (stopped itself after 1 s); a
start from stopped at 6 000/s; 9 000/s; a trip `stop()` part-way into a
tick. All 13 changes applied, every count and phase prediction exact, no
refused queue; R4 count = R3 edges (285 and 288 in two runs); every pulse
LOW 4.5–6.0 µs, the trip's last one included; the line HIGH and silent
after the trip.

- **Found on the way:** `stop()` and `run()` kept the previous run's
  schedule, so a queue right after them was refused (the `Q` runs only
  passed because of an extra wait); both now clear it. A hardware start
  takes effect one count clock after the start event (manual §22.9.4): the
  tick after a start had its phase 1 lower than predicted, so the schedule
  at a start tick uses the captured phase − 1.
- **Validator:** a zone stop whose last pulse the R3 could not time (its
  interrupt started late while receiving the R4's reply) is now repeated
  (up to 3 tries) instead of failing; it happened once in 50 stops.

**Step 5b — direction and position, verified on WiFi 2026-11-04.** Rates
are signed (`run(i32)`, `queue(i32)`), and header D5 (P107) is DIR, at
`StepConfig::forward` for positive rates. A reversal at a running tick
extends the tick's DTC chain to four records:

1. the period write (`GTPR`, `GTPBR`);
2. a delay record (block mode, source and destination fixed,
   `ceil((low + 16) / 2) + 8` words: 136 for the cart-pole's 240-count low);
3. the DIR write, one word to `PORT1.PCNTR3` (P107: bit 7 sets, bit 23
   clears; manual §19.2.3);
4. the `GTCNT` read-back.

The delay makes DIR land after any pulse that is LOW at the tick, or that
fires inside the write latency, has ended. The next pulse is placed
`dir_setup` after the latest write the HAL allows for
(`dir_write_window()`), whenever the agreed rule would put it sooner. A
reversal while idle (rate 0) or from stopped needs no DTC: the CPU sets
DIR at queue time, at least 40 µs before the tick. `position()` and
`TickReport::position` count completed pulses forward minus backward. A
pulse that started before the flip counts in the old direction.
`TickReport::dir_changed_at` gives the DIR write's offset from the tick.

- **Measured first — a delay record's length (R4 request `X<words>`):**
  N words hold the chain 2N + 22…32 PCLKD counts (N = 32, 64, 96, 128),
  on top of the 14–17-count first-record latency (step 5a). The latest
  allowed DIR write adds 120 counts of slack to the measured maxima.
- **Proof (`S11 reverse`, R4 request `W<mode>`; the R3 reads DIR on D7 at
  every STEP fall).** For each run, the R4 checks every reversal from its
  own read-back:
  - DIR was written inside the window;
  - after any pulse LOW at the tick, or fired inside the write latency,
    had ended;
  - at least `dir_setup` before the next pulse.

  The R3 checks the R4's position change against its own DIR-signed count
  (pulses with DIR HIGH minus pulses with DIR LOW).

  | Run | Result (8 runs, 2026-11-04) |
  | --- | --- |
  | W1: a reversal at every tick, ±15 000–30 000/s, 500 ticks | 500/500 reversals, 0 rule violations; 69 with a pulse LOW at the tick and 1 with the old pulse first, every run; DIR written 326–339 counts after the tick (342 at most in earlier runs); position R4 = R3 (−14 or −15), 22 375–22 376 pulses equal; shortest interval 33.0 µs |
  | W2: ±4 842/s ramp through 0, 80 ticks (reversals through idle) | position 1–2 = R3; counts equal |
  | W3: 600 ticks stopped, then signed rates from a hardware start | 2 running reversals; position −28 = R3; counts equal |

- **The R3's DIR reads:** the R3 reads DIR as soon as its interrupt starts.
  A read taken after the pulse has ended (the interrupt started late) may
  already see the new direction. The R3 counts those as uncertain (95–138
  per W1 run), and its check allows the position to differ by up to twice
  their number. In all 8 runs the positions were equal exactly. At
  30 000–55 733/s the R3 overloaded while reading DIR (11 % late reads, a
  0.0 µs "minimum interval"), so W1 runs at 15 000–30 000/s. This is an
  instrument limit, not an R4 one.
- **Deviation from the agreed rule (accepted by cart-pole 2026-11-04):** a reversal
  flips DIR about 7 µs after the tick, always, not at the tick or at the
  end of a pulse in flight. The next pulse then comes at least `dir_setup`
  after the latest allowed write. With the cart-pole timing that is
  ~13 µs after the tick, whenever the one-period rule would allow sooner.
- **Limit:** `low_ns` is at most 400 counts (8.3 µs), so the delay record
  stays within its block size; `d4` refuses more (`ConfigInvalid`).
- **Trip-stop figure (R13, accepted by cart-pole 2026-11-04):** `stop()` takes at most
  the low time plus 24 counts (5.5 µs with the cart-pole timing), not the
  agreed ≤ 5 µs. A call within 0.5 µs of a pulse's start lets that pulse
  through and stops at its end; the pulse is never cut. (Superseded the
  same day by the stop fix below: the figure still holds in a release
  build. The claim "never cut" did not; see below.)

**An intermittent one-edge miscount (2026-11-04), now fixed.** In the first
full run with 5b, the R3 counted exactly one edge more than the R4 in
two checks: `S11 count`, 1 of 20 runs (55 733/s stopped by `T0`: R4
11 807, R3 11 808), and `S11 tick`, one stress run (44 600/s: R4 222 929,
R3 222 930). It has not happened again in 3 `slices S11` runs and
3 `validate all` runs since. The R3 now logs `interval min..max` with
those lines, and the next occurrence will separate the two causes:
- a short interval means the R3 saw an extra edge;
- normal intervals mean the R4 did not count a pulse.

**Reproduced with interval logging, same day:** 20 `slices S11` runs; run
19 failed the same way (`S11 count`, 55 733/s stopped by `T0`: R4 11 824,
R3 11 825, every interval 17.5–18.0 µs). The R3 saw no short interval, so
it didn't see noise. The extra edge was a real pulse start one period after
the previous one, and the R4 didn't count it. All three failures (two
count, one tick) were runs at 44 600–55 733/s ended by `stop()`.
**Unverified hypothesis:** a pulse cut by the software stop. The HAL counts
a pulse when it ends (`GPT0_CCMPB`), so a cut pulse would never be counted.
`stop()` clears `CST` only when the counter is at least 24 counts
(`STOP_MARGIN`) from the next pulse. If the read-then-write takes longer
than that, the next pulse starts before the stop lands, and the stop cuts
it. Not yet observed directly: the count check doesn't time widths at
these rates.

**Follow-up 1 — `stop()` cut pulses; fixed and verified on WiFi
2026-11-04.**

- **The experiment (R4 request `H<cycles>`, in `S11 count`).** It runs
  start/stop cycles at 55 733/s, each stopped at a pseudo-random phase.
  After each stop the frozen counter says where the stop landed: below the
  low time means the stop cut a pulse. The R3 counts every edge over the
  burst. A cut pulse is an edge the R3 sees and the R4 never counts.

  | `stop()` version | Build | Cut pulses | Stop write after the decision |
  | --- | --- | --- | --- |
  | margin of 24 counts, read-modify-write of `GTCR` (the original) | release | 65 in 10 000 (0.65 %) | up to ~30 counts |
  | the same margin, counter read last and a plain `GTCR` write | release | 0 in 10 000 | up to 17 counts |
  | the same | unoptimised (`dev` profile) | 2 673 in 10 000 (27 %) | up to ~260 counts (5.4 µs) |
  | **the longest period first, then stop after any pulse in flight** | release | **0 in 394 000** | no race |
  | the same | unoptimised | **0 in 10 000** | no race |

  The hypothesis held: the race was the only cause. With the original
  `stop()`, R3 − R4 matched the cut count (minus a few cuts shorter than
  ~0.1 µs that the R3 couldn't see).
- **Any fix that races the counter depends on how the firmware is
  compiled**, so `stop()` no longer races it. It waits for a DTC transfer
  already under way (`DTCSTS.ACT`, manual §17.2.10), then writes the
  longest period to `GTPBR` and `GTPR` (the rate-0 write). After that no
  pulse can start for ~89 s. It waits until a pulse in flight has ended,
  then clears `CST`. The hardware stop (`GTPSR`) is no longer used.
- **The stop still never waits for a later pulse.** It returns after at
  most the rest of a pulse in flight plus its own code time. In release
  that is ≤ the low time + ~0.5 µs, so R13's 5.5 µs still holds. In an
  unoptimised build it is about 10 µs more.
- **Instrument limit found on the way.** Restarting at once after a stop
  can put the next cycle's first pulse ~13 µs after the previous cycle's
  last. That is closer than the R3's capture interrupt resolves when its
  own timer interrupt is due. The R3 then lost one edge (R3 = R4 − 1,
  4 in 71 bursts, shortest interval 13.0 µs, some intervals logged as
  0.0 µs). The R4's frozen-counter test doesn't depend on the R3 and showed
  no cut in those bursts. The burst now pauses 25 µs after each stop. Over
  96 more bursts (192 000 stops): R3 = R4 every time, shortest interval
  17.5 µs.
- **Validator changes:** `S11 count` gains 5 bursts of 2 000 stops (no cut
  pulse, R3 = R4); the CLI waits 120 s for the R3's full run (the added
  checks pushed it past 60 s). `S11 reverse` W1 now runs 2 000 ticks with
  hashed rates. The 500-tick walking pattern met the "old pulse first" race
  once per run before this change, and never after the stop change
  shifted its starting phase. Now it meets it 10–14 times per run.

**Follow-up 2 — the tick wakes the CPU from `WFI` (cart-pole request);
verified on WiFi 2026-11-04.** `set_tick_wake(true)` enables the tick's
ICU slot's NVIC line (`Ielsr::enable`; `Ielsr::disable` is new for the off
switch). Every tick then raises a CPU interrupt (manual Table 13.5,
§17.7): an unarmed tick at once, an armed one when the DTC has finished
(it clears `DTCE` and passes the request to the CPU). No handler is
needed: the HAL's slot trampoline acknowledges `IR`.
`since_tick_counts()` reads GPT164's counter × 4, the time since the tick.

- **Proof (`S11 wake`, R4 request `J`):** the `Q3` pattern (a different
  rate over 30 000–55 733/s every tick, 2 500 ticks) with the R4 asleep in
  `WFI` between ticks. It found 2 500/2 500 ticks, none skipped, every
  change applied, every count and phase prediction exact. R4 count = R3
  edges (214 053). 2 502 wakes in all: no other interrupt woke it. Each
  tick was seen at most 92 counts (1.9 µs) after it, and each `queue`
  finished at most 1 444 counts (30 µs) after it. That leaves 92 600
  counts before the 40 µs queue guard.
- **Unverified:** waking with CPU interrupts masked (`PRIMASK` set). The
  docs ask for `intc::enable_interrupts`, as the validator has.

**Follow-up 3: ticks passed over (missed-tick recovery). Verified on WiFi
2026-11-04.** The cart-pole session found it by reading the code: a tick
that `poll_tick` passed over left the generator without a schedule for good.
Its causes are any stall over 2 ms, such as a data-flash erase (6 ms).
- **Idle stall:** the 1 s idle stop needed a schedule, so it never fired,
  and the idle period (`u32::MAX` counts) fired a stray pulse every 89.48 s
  until `stop()`.
- **Moving stall:** the motor held its last rate, and every `queue()` was
  refused until `stop()`.
- **Missed reversal** (found while tracing the first two): a reversal
  queued for a missed tick still flipped DIR in hardware, but not in the
  generator, so `position` counted every later pulse with the wrong sign.

All three now work from the hardware state, not the history:
- **The idle stop** fires when `GTPR` and `GTPBR` both hold the idle length
  and the phase captured at the tick (the time since the last pulse) is
  over 1 s.
- **The schedule is rebuilt from the registers.** With no change armed,
  `GTPR` changes only at an overflow (§22.3.2.1). So if the STEP counter
  moved at least as far as the tick timer since the tick, there was no
  overflow: the period at the tick is `GTPR + 1`, and every later one
  `GTPBR + 1`. Otherwise the next poll retries.
- **A missed reversal** is split exactly as at a polled tick, from the
  previous (exact) schedule and the DTC's read-back. It then flips the
  direction and corrects `position`.

The arithmetic is in `step/recovery.rs`, host-tested.

- **Proof (`S11 skip`, R4 request `k`):**
  - Modes 1 and 2 queue a rate every tick and stall 5 ms after every 20th
    queue, so the change lands on a tick nobody polls. Mode 1 stalls over
    rate changes, mode 2 over reversals.
  - Of 20 stalls per run, the ten that end just after a tick were rebuilt
    in the same poll; the ten that end anywhere took at most one more poll.
  - Every count and phase prediction afterwards was exact. Count and
    position equal the R3's edges and DIR reads, R4 −252 = R3 −252 in
    mode 2.
  - Mode 3 stalls while idle. The generator stopped 500 ticks (1 s) after
    going idle, with no pulse after it.
  - With the old code every mode fails: `queue` never recovers, and the
    idle generator never stops.

**Also in this step:** the repo-wide rule sweep of 2026-11-04 (one type
per file, one `use` tree per root, doc links). It changed structure and
docs only, re-proved by 23/23 on the board.

#### Cart-pole dogfooding fix: the DIR set-up before the first pulse (2026-11-04)

Cart-pole's dogfooding report found that `run` started the first pulse the
PUL minimum HIGH time after setting DIR, not the DIR set-up time. Its rig
has a 5 µs set-up and a 2.5 µs minimum HIGH, so a direction change through
`run` gave the DM542 half its set-up. Reading the code found a sibling:
`queue` from stopped or idle writes DIR at least 40 µs before the tick and
started the first pulse the minimum HIGH time after it, so a set-up over
about 40 µs was not met.

The fix (`step/start_lead.rs`, `step/first_cycle.rs`, host-tested):
- **The wait:** after a DIR change, the first pulse waits the DIR set-up
  time, less what has already passed since DIR was written (nothing for
  `run`, the 40 µs queue guard for `queue`), and never less than the
  minimum HIGH time.
- **The first period:** the counter is preloaded so the first overflow
  comes that long after the start. When the wait plus the LOW time does
  not fit in one period, `GTPR` holds a longer first period and `GTPBR`
  the steady one, which the overflow reloads (§22.3.2.1). A start from
  stopped through `queue` records that first period in its schedule, so
  predictions stay exact.
- **The limit:** `DirPinBuilder` refuses a set-up over 1 ms. `run`'s first
  pulse then always comes before the second tick after it, where the
  generator takes the train as steady.

- **Proof (`S11 dirlead`, R4 request `d1`–`d4`, R3 command `L`):** ten
  starts per mode, their signs alternating, so DIR changes at nine or ten.
  The R3 times each DIR change on D7 to the next STEP falling edge on D8
  (Timer1 at 62.5 ns, the edge by input capture). DIR is polled, and from
  the compiled loop a time reads up to 25 cycles (1.56 µs) short, never
  long. The thresholds allow for that.
  - `d1`: `run(±1 000)` from stopped. Lead 4.63–5.06 µs (pass: 3.5–7 µs).
  - `d2`: `run(±120 000)`, where the 5 µs set-up plus the 5 µs LOW does not
    fit in the 400-count period, so the first period is longer. Lead
    4.63–5.06 µs, and every prediction on the following ticks exact.
  - `d3`: `queue(±1 000)` from stopped. Lead at least 1 178 µs (pass:
    41 µs). DIR is written early in the tick here, so this shows the
    guard is kept, not how close to it a late `queue` comes.
  - `d4`: direction changes through idle (`queue(0)`, then the other
    sign). Lead at least 1 985 µs.
  - The R4 checks every phase and count prediction on five ticks per
    start: 0 misses in 175 checked ticks.
  - **With the old `run` timing** (an image built with the old lead, for
    this comparison only), `d1` measured 2.31–2.50 µs and `d2` 2.00–2.44 µs,
    and the check failed.
- **Not run on the board:** a set-up longer than the 40 µs guard, through
  `queue` from stopped or idle. The bench's 5 µs set-up never needs the
  longer first period there; it is host-tested (`tests/start.rs`).
- **`is_enabled(&self)`:** the ENA latch moved into a `Cell`, so
  `is_enabled` no longer needs `&mut`. The validator has no ENA pin, so
  this is compile-checked, not run on the board.

#### Interrupt wake-ups instead of polling (2026-11-04)

Cart-pole's firmware sleeps in `WFI` and wakes only on interrupts. Two
things it waited for could not wake the CPU: a limit switch's edge (the
latch had no CPU interrupt) and the end of a move of exact length (it
polled `pulses()` and could run one pulse over).

**Switch wake-up** (`SwitchPinBuilder::with_wake`, needs `with_edge`):
- The edge's ICU slot also interrupts the CPU, so the edge ends a `WFI`.
- The HAL's dispatcher clears `IELSRn.IR` before any handler, which used
  to erase the latch `activated()` reads. It now records each request it
  acknowledges (`dispatch::ACKNOWLEDGED`), and `activated()` reports a
  flagged or an acknowledged request (`Ielsr::take_request`).
- **Proof (`S2 wake`, R4 request `w`, R3 command `W`):** the R4 lends its
  D3 pin to a `Switch` (normally open, internal pull-up, filter, edge,
  wake-up) through the public `Claim`/`Release` traits, sleeps in `WFI`,
  and takes the pin back with `free` after the press. The R3 presses D3
  100 ms after the request. Reply `w 1 1 1`: `activated()` reported the
  press after its interrupt ran, the CPU woke once (nothing else woke
  it), and D3 still read pressed.

**Stop at an exact pulse count** (`StepGenerator::stop_at`):
- The counter's compare match B goes to the STEP channel over ELC input D,
  whose `GTPSR.PSELCD` (bit 19, SVD) stops its counter, and to the CPU
  through a second ICU slot (`set_count_wake`).
- **Measured: the compare match fires one count late.** With
  `GTCCRB = count` every move ran exactly one pulse over: +5 over five
  moves in modes 1 and 2, 2 001 for 2 000 in mode 4, at 1 000/s as at
  55 733/s, so it is not a late stop. The match of the event-counting
  counter fires on the count event that leaves the compared value, so
  `GTCCRB` holds `count − 1` (`count_compare_value`, host-tested).
- A move from rest of 1 pulse, which arms `GTCCRB` equal to the current
  count, stopped after exactly 1: writing the value does not match at once.
- **Proof (`S11 countstop`, R4 request `n1`–`n4`, R3 command `M`):** after
  every move the count is exactly the target and unchanged 2 ms later;
  the R3 captures every STEP edge over the request.
  - `n1`: 1, 2, 3, 50 and 500 pulses, armed, then `queue(1 000)` from
    stopped. R4 556 = R3 556.
  - `n2`: 1, 10, 1 000, 30 000 and 65 535 (the 16-bit counter's reach),
    armed, then `run(55 733)`. R4 96 546 = R3 96 546.
  - `n3`: armed 2, 3, 100 and 5 000 ahead while running at 55 733/s, and
    the refusals (1 ahead while running, the current count, 66 000 ahead
    on the 16-bit counter): 12 of 12 refused as documented.
  - `n4`: 2 000 pulses at 30 000/s with the tick wake off: the CPU woke
    once, from the count stop, with the count exact.
- Not run on the board: a 32-bit counter's count stop (the validator
  counts on GPT162), and `cancel_stop_at` racing the match.
- **Later the same day, on the cart-pole rig** (its report, not witnessed
  on this bench): its calibration firmware sleeps in `WFI` through every
  move and wait. It ran the count stop with `set_count_wake` on GPT321 (a
  32-bit counter; centring ends on the count stop), normally-closed limit
  switches on D2 and D3 with `with_edge().with_wake()`, and ENA, with
  `set_tick_wake` through the ENA lead and its 1 ms trip filter. It
  reports the rig behaving as its polling version did. Still not run on
  any board: `cancel_stop_at` racing the match, and a switch with an
  external pull-up.

#### Bench checks for the never-run behaviours (2026-11-04)

Six behaviours had never run on a board. All six now have bench checks,
and two of the checks found HAL defects, fixed the same day.

**`S11 cancelrace`** (R4 request `c`, R3 command `C`, part 3):
- At 55 733/s the R4 arms a stop 4 pulses ahead, waits for the pulse
  before the target to end, waits a swept time (0 to about 1.4 periods),
  and calls `cancel_stop_at`, 2 000 times.
- Each cancel must leave one of two states. Either the stop won:
  `stopped_at_count()` true, the count exactly the target, no pulse in the
  next 100 µs. Or the cancel won: `stopped_at_count()` false, pulses still
  coming.
- **Defect found.** The first run: 680 stops, 1 320 cancels, 0
  violations. The next: 670, 1 315 and **15 violations of type 1**, a
  stop reported at a wrong count.
  - `cancel_stop_at` cleared `GTPSR`, then read `TCFB`. A match between
    the two sets `TCFB` with the stop source off.
  - The cancel then took the stop as reached, waited for a hardware stop
    that never came, and stopped the train itself, past the count.
  - Fixed: a stop counts as reached only if the STEP counter stopped
    (`GTCR.CST` clear).
- **After the fix,** five runs: 670 stops, 1 330 cancels, 0 violations
  each. The 15 now resolve as cancel wins. R4 14 465 = R3 14 465.
- **Then the waits were made random,** drawn afresh each request (the
  sweep had repeated the same moments every run). Nine runs: 657–703
  stops, 0 violations in 18 000 cancels.

**`S11 sleep`** (R4 request `s1`–`s3`, R3 command `S`, part 3):
- `run`, 5 ticks checked, then a wait on the SysTick clock with no
  generator call (tick wake off), then one poll and `queue` of a new rate.
  Then 20 ticks checked. The R3 counts STEP on D8 and reads DIR on D7.
  - `s1`: 100 ms at 50 000/s, then 30 000/s.
  - `s2`: 500 ms at 40 000/s, then −40 000/s.
  - `s3`: 3 s at 10 000/s, then 20 000/s.
- Every mode keeps the 16-bit counter under 32 768 pulses between reads,
  its documented limit.
- **First runs: timing.** The sleep was timed with
  `cortex_m::asm::delay`, which lasted 3× its count (149 ticks skipped
  for 100 ms, 749 for 500 ms).
  - At 40 000/s, 1.5 s is 62 022 pulses, past the 16-bit read limit, and
    the R4's count came out wrong, as the README says it will.
  - Mode 3 (about 9 s) outlasted the R3's timeout, and its late replies
    failed `S11 elc`, which ran next.
  - The sleep now runs on the SysTick clock.
- **Defect found.** With the timing fixed, `queue` was still refused at
  all 10 polls after 6 of 24 sleeps (all three modes). The count and
  position were exact every time.
  - Diagnostic fields (the first tick's phase, the refusals) showed every
    failure had the tick 118–260 counts before a period's end: phases
    838 and 786 of 960, 1 082 of 1 200, 4 540 of 4 800. Every pass had
    more room.
  - The rebuild (`resync_schedule`) refused any wrap of the STEP counter
    since the tick. The poll reaches its reads more than 5 µs after the
    tick.
  - These rates fit the tick a whole number of times (100, 80 and 20
    periods), so the tick lands at the same phase every time, and every
    poll refused.
  - Fixed: with `GTPR = GTPBR` a wrapped train is proven from where the
    counter must stand. `resync` also retakes reads that straddle a wrap.
- **After the fix,** 18/18 sleeps queued at the first poll, among them
  phases 824 of 960 and 932 of 1 200. Skipped 48, 248 and 1 498, every
  later prediction exact, count and position = the R3's.

**`S2 pullup`** (R4 request `e`, R3 command `E`, part 1):
- The R4 builds the D3 switch with `Pull::External(50_000)`. The R3's
  `INPUT_PULLUP` (ATmega328P `RPU` 20–50 kΩ) is the only pull-up on the
  line. Then the R3 presses (LOW 200 ms) and releases.
- Reply `e 0 0 1 1 1 1`: `P105PFS.PCR` read 0 (the R4's pull-up off), D3
  idle (not active), the press woke the CPU once and was reported, D3
  read active at the wake, and the release brought D3 back to idle, which
  only the R3's pull-up drives.

**`S11 boot`** (R4 request `b`, R3 command `B`, part 3, run last because
it resets the R4):
- `free` and a new `build` take the whole `Peripherals`. So, as the
  README's order rule says, they run at boot, before any driver takes a
  field. No public API changed for this.
- At boot the validator builds a second setup: STEP on D3 (GPT321 output
  A; the main setup is GPT320 output B), DIR on D5 with a 200 µs set-up.
  - It changes direction four times through `queue`: from stopped,
    through idle, from stopped, through idle.
  - Each change is queued 60 µs before a tick, so DIR leads the tick by
    about 60 µs and the HAL must hold the first pulse back another
    140 µs.
  - Then it calls `free` while the train runs, reads back the pins,
    GPT321 and GPT162's raw count, and builds the main generator from the
    returned peripherals.
- The R3 resets the R4 (`!`), timestamps every STEP fall on D3 (INT1) and
  every DIR change on D7 (PCINT23), then reads `b`.
  - Pass: 4 DIR changes, each leading the next pulse by 190 to 500 µs as
    read.
  - The falls equal the R4's count after `free` (no cut pulse, none
    after), the pins are GPIO inputs, GPT321 is stopped, and D3 idles
    HIGH.
- **Result:** leads 212–216 µs, falls R3 36 = R4 36, `b 1 4 0 0 0 24 1 1 1
  36`. Queued early in the tick instead, as in the first version of this
  check, the leads were 2 144 µs or more and tested nothing.

**`cortex_m::asm::delay` waits at least its count in CPU cycles, and how
much longer depends on the call site.**
- In the first `S11 sleep` version it took 3 cycles per count (measured
  against the step generator's tick: 149 ticks skipped for 100 ms).
- The delay probe (R4 request `m`, R3 command `D`, part 3) times it on the
  SysTick clock at its own call site: 250, 251, 250 and 252 ms for 4 ×
  2.4 million, 10 × 960 000, 40 × 240 000 and 8 000 × 1 200 counts, which
  is 1.25 cycles per count. A first version timed rows longer than the
  SysTick wrap (349 ms) and read low (151 and 204 ms); every row now
  stays under one wrap.
- So the validator's waits written as counts are minimums. Their comments
  now say "at least" (5 ms, 2 ms, 20 ms), and the `S11 skip` stalls are
  stalls of at least 5 ms. No check needs a wait to be shorter than its
  stated time.

#### Slice 12: the wake timer (2026-11-04)

Cart-pole's firmware sleeps in `WFI`, and the step generator's 2 ms tick
was its only timed wake-up. Its 1 ms switch re-check landed 1 to 3 ms
after the press. The wake timer (`wake::WakeTimer`) is a one-shot timed
wait on its own GPT channel.

**How it works** (manual §22.2.6, §22.2.12, §22.2.16):
- the channel counts up from 0 and overflows; the overflow reaches its own
  `GTPSR.PSELCx` through a claimed ELC input, so the counter stops in
  hardware, and raises the timer's ICU slot, which ends a `WFI`;
- `expired()` reads `GTST.TCFPO`, which the HAL's acknowledgement leaves
  set, so it works in a masked sleep; `start` and `cancel` clear it, the
  ICU request, the acknowledged bit and the NVIC pending bit;
- each `start` picks the finest prescaler (`TPCS`, written while stopped)
  that reaches the wait, rounding up.

**ELC inputs claimable:** `ElcA`–`ElcH` are `Peripherals` parts. The step
generator claims B, C and D; `Elc::link` takes a claimed input (the
validator's `S11 elc` check now links its claimed `elc_a`).

**Proof (`S12 wake`, R4 requests `t1`–`t3`, R3 command `T`, part 3).** The
validator's timer is GPT167 on `ELC_GPTE`, timed against GPT321
free-running at PCLKD (20.8 ns).
- **`t1`, lengths:** 5 µs, 100 µs, 1 ms, 1.365 333 ms, 1.365 334 ms
  (the prescaler boundary), 21.8 ms, 87.4 ms, 349.5 ms and 1.398 s, each
  slept through masked. None short (at most 9–13 counts early, the read
  after `start`), none more than one step late, one `WFI` wake each, the
  counter stopped 2 counts after its overflow.
- **`t2`, behaviour:** fired once (no second end, counter still); a start
  after an unread end reads false until its own end; a cancel just after
  the end reads false; a cancel before the end stops it and nothing fires;
  a zero wait ends at once, once; 1 ns past the longest is refused and the
  longest is 1.398 101 333 s. All six as documented.
- **`t3`, on a wire:** the R4 drives D3 LOW at `start` and HIGH at the
  wake, and the R3 times the widths on INT1. 1 ms read 1 008–1 012 µs and
  100 ms read 100 080–100 112 µs. The R3's clock reads the R4's time about
  0.1 % long (the S1 check reads 20 ms phases at 20 012–20 020 µs), so
  the pass band is 8 µs short to 0.3 % + 30 µs long.
- **Defect found by `t3`, fixed:** the first `start` divided in 128 bits
  per prescaler tried, a software routine on this CPU. The wire read
  1 040 µs and 100 284 µs, about 37 µs per division before the count
  began, which `t1` could not see because it reads the reference after
  `start` returns. A first fix still computed the longest wait (a 128-bit
  division) on each call: 1 060–1 064 µs. Now the clock constants are
  worked out at build, and `start` multiplies and shifts only.
- **Also found while fixing it:** `longest_nanos` overflowed a `u64` on a
  32-bit channel; host-tested now.

**The ENA lead fix (`S11 boot`):** the boot-time setup gained ENA on D2
(free on the bench) with a 5 µs lead. `enable()` to `is_enabled()`, timed
on GPT166 at PCLKD, read 331–342 counts against the lead's 240. The timing
brackets both calls, so it shows the lead is at least 5 µs and how soon
`is_enabled()` follows. The one-count margin itself (the lead could end up
to 83 ns short before the fix) is host-tested (`tests/ena.rs`).

**`S11 countstop` mode 3 race, found while running the sessions:** 2 of 9
runs failed with one move refused. The check reads `pulses()`, then arms
2 ahead while running at 55 733/s (an 18 µs period). A pulse can end
between the two, and `stop_at` then refuses with `CountNotAhead`, as
documented. With the failing move and refusal reported in the reply, and a
retry only when the count moved in between: 2 of 20 runs raced once each,
and all 20 passed.

#### Interrupt wake-ups: R3 flash note

The R3 sketch is now at 32 220 of 32 256 bytes of flash. To fit these
checks, report strings were shortened (the `validate` CLI matches each
line's prefix) and the help text points to the sketch's header. The next
check needs more room made first. (Same day: the sketch was split into
parts that each fit, two and then three; see the All-in-one record.)

### All-in-one validation — Slices 1–11 (`arduino/validation/`)

The validation pair re-proves Slices 1–11 on the existing bench without
swapping firmware between slices. It has three parts:

- `arduino/validation/r3-validator/r3-validator.ino`: the R3 controller.
  The checks no longer fit the R3's 32 KB in one image, so the one source
  builds as three parts, chosen by `R3_PART` at compile time, each
  uploaded for its own `validate` session:
  - **part 1** runs Slices 1–10, and has the boot-counter watch and the
    USB take-over the host checks use;
  - **part 2** runs Slice 11's first checks: step, count, tick, queue,
    scenario, reverse, wake, skip, elc;
  - **part 3** runs Slice 11's later checks (dirlead, countstop,
    cancelrace, sleep, boot), Slice 12 (the wake timer) and the delay
    probe.

  Parts 2 and 3 share the Slice 11 code, and the linker drops what a part
  never calls. Every part answers `i`, `s`, `!`, `@` and `?` (which names
  the part). A build without `R3_PART` stops with an `#error`.
- `crates/test-firmware/src/bin/r4_validator/main.rs`: one R4 image built on
  this HAL, with features `usb,uart-irq`.
- the `validate` CLI (`crates/validation`): the host side. It also checks
  what only a PC can see or time: Slice 4 on the bridge monitor, Slice 9
  on the native USB port, and Slice 10's boot counter across resets and
  power cycles.

The R3 and R4 exchange framed requests (`STX` body `ETX`) at 9600 8N1 on
SCI2. R4 replies contain only `A–Z`, `0–9` and spaces, while PC commands to
the R3 are lowercase, `?`, `!` or `@`. That keeps the shared R3 RX
unambiguous, and the R4 releases SCI2 TX between replies, so `!`/`@` always
reach the R3. Both source files document the full contract.

Each check. Part 1 runs the Slice 1–10 rows and the knock. Part 3 runs
`11 (dirlead)`, `(countstop)`, `(cancelrace)`, `(sleep)`, `(boot)` and
`12 (wake)`, and part 2 the other Slice 11 rows. Every part runs `!`/`@`. A session
records the other parts' checks as NOT RUN, so a full validation is all
three sessions:

| Slice | How it is checked | Pass condition |
| --- | --- | --- |
| 1 | The R4 drives D3 with the HAL `Delay` (20 ms phases); the R3 times the edges on INT1 | 22 edges, every phase 20 000 µs ± 2 % |
| 2 | The R3 drives D3 low, then high; the R4 reads it | reads 0, then 1 |
| 2 (button) | The R3 presses R4 D3 like a switch to GND (LOW for 200 ms, then released to input); the R4 times it | press ~100 ms after the request, held ~200 ms |
| 3 | 10 identify round trips over SCI2 | 10/10 |
| 4 | The R4 prints a token on SCI9; the host reads the bridge port | token seen |
| 5 | 4096-byte burst through the interrupt-driven SCI2 echo | byte-exact, `OV 0 ER 0` |
| 5 (traits) | The same burst, echoed only through the HAL's `embedded-io` 0.7 `Read`/`Write` on `&UartIrq` (blocking `read`, `write_all`, `flush`) | byte-exact, `OV 0 ER 0` |
| 6 | `AT` to the ESP32-S3 over SCI1 | `OK` (received bytes shown in hex) |
| 7 | D6 PWM at 0/25/50/75/100 %, measured by the R3 on INT0 | period 1000 ± 10 µs, duty ± 10/1000, 0/100 % flat |
| 8 | R3 D5 duty 0/64/127/191/255 into A0; the R4 averages 200 ms windows | mean within ± 82 LSB (2 % of 12-bit full scale) |
| 9 | Native CDC after R3 `u`, while the bridge is gone | ticks consecutive, max gap ≤ 1.5 s, exact echoes, state Configured |
| 10 | Data flash block 6 (`F`): erase, blank check, program 1024 bytes, read back, a second program of a used byte, erase, blank check | byte-exact, the rewrite refused (`NotBlank`), blank after each erase |
| 10 (retention) | The R4 appends one byte per boot to a log in block 7; the R3 watch (`k`) polls `K` and pulses `!` three times | each reset advances the counter by exactly 1, log reads back intact |
| 10 (power cycles) | The same watch while the owner power-cycles the R4 (`validate s10 --watch`, manual, not in `all`) | ≥ 20 boot events, each +1 and intact |
| 11 (reverse) | Slice 11 step 5b (`W`): signed rates — a reversal at every tick at ±15 000–30 000/s (2 000 ticks, hashed rates), a ramp through 0 (80 ticks), signed rates from a hardware start; the R3 reads DIR on D7 at every STEP fall | every reversal within the DIR rule (R4 read-back), both hazards met in W1, every prediction exact, R4 total = R3 edges, R4 position = R3 DIR-signed count (within 2 × uncertain reads) |
| 11 (step) | Slice 11 step 2 (`T`, `Z`): the step generator on R4 D4 at 1, 1 000, 44 600 and 55 733 pulses/s, timestamped on R3 D8 by Timer1 input capture; LOW timed at 1 and 1 000/s; 50 stops (30 inside chosen phases, 20 random) | periods within 0.5 % + 1–2 ticks; LOW 4.5–7.0 µs; HIGH and silent after every stop |
| 11 (scenario) | Slice 11 step 5c (`O`): 618 ticks — ramp, rate 0, held ticks, restart from idle, 1 s idle stop, start from stopped, trip stop | 13/13 changes applied, every prediction exact, R4 total = R3 edges, every LOW 4.5–7 µs, HIGH and silent after the trip |
| 11 (queue) | Slice 11 step 5a (`Q`): a rate queued after every tick and written by the DTC at the tick — 960/44 600/s alternating (×2), a ±2 421/s ramp (×2), 2 500 ticks of random rates | every change applied at its tick, every count and phase prediction exact, burst starts 4 ms apart on the R3, the late-write race met in the random run, R4 total = R3 edges |
| 11 (tick) | Slice 11 step 4 (`Y`, `V`): 60-tick runs at 0, 1 000, 44 600, 55 733, 12 345/s and 2 500-tick runs at 44 600, 55 733 and 44 486/s (×2); each tick's latched count against the previous prediction | every prediction exact; per tick within rate × 2 ms ± 1; no skipped tick; R4 total = R3 edges |
| 11 (count) | Slice 11 step 3 (`N`): the step generator's hardware pulse count against the R3's captured edges, 20 start/stop runs at 1 000, 44 600 and 55 733/s with zone and plain stops; then (follow-up 1, `H`) 5 bursts of 2 000 stops at random phases at 55 733/s | equal in every run; the R3 missed no edge; no stop froze the counter inside a pulse (no cut pulse), R3 = R4 in every burst |
| 11 (wake) | Slice 11 follow-up 2 (`J`): 2 500 ticks of random 30 000–55 733/s rates with the R4 asleep in `WFI`, woken by the tick | every tick seen and applied, none skipped, every prediction exact, each `queue` done before the 40 µs guard, R4 total = R3 edges |
| 2 (pullup) | Never-run checks (`e`, R3 `E`): the D3 switch with `Pull::External`, the R3's pin pull-up the only pull-up | `PCR` off, idle not active, the press wakes the CPU and is reported, the release returns D3 to idle |
| 11 (cancelrace) | Never-run checks (`c`, R3 `C`): `cancel_stop_at` at 2 000 random moments around the count match at 55 733/s | every cancel leaves a reported stop exactly at the count or a running train, both outcomes seen, R4 count = R3 edges |
| 11 (sleep) | Never-run checks (`s1`–`s3`, R3 `S`): 100 ms, 500 ms and 3 s with no generator call at a steady rate, then a poll and a `queue` (`s2` reverses) | skipped ticks reported, `queue` within 3 polls, every later prediction exact, R4 count and position = R3 edges and DIR |
| 11 (boot) | Never-run checks (`b`, R3 `B`, which resets the R4): at boot, STEP on D3 (GPT321 A) with a 200 µs DIR set-up and ENA on D2 with a 5 µs lead, four direction changes each queued 60 µs before a tick, `free` while running, the main generator rebuilt | DIR leads 190–500 µs, R3 falls = R4 count after `free`, pins inputs, GPT321 stopped, `enable()` to `is_enabled()` at least the lead and within 200 counts of it |
| 12 (wake) | Wake timer (`t1`–`t3`, R3 `T`): GPT167 on `ELC_GPTE` against GPT321 at PCLKD — nine waits 5 µs to 1.398 s, the documented behaviour, and a 1 ms and a 100 ms wait marked on D3 | every wait at least its length and within one step + 10 µs, one `WFI` wake each; all six behaviours; on the wire 8 µs short to 0.3 % + 30 µs long |
| 11 (skip) | Slice 11 follow-up 3 (`k1`–`k3`): stalls of at least 5 ms of the poll loop over rate changes, over reversals, and while idle | every stall recovered (same poll when it ends just after a tick, ≤ 1 more otherwise), every later prediction exact, R4 count and position = R3 edges and DIR, idle stop 1 s after idle |
| 2 (wake) | Interrupt wake-ups (`w`, R3 `W`): D3 as a switch with `with_edge` and `with_wake`, the R4 asleep in `WFI` until the R3 presses it | `activated()` reports the press after its interrupt ran, one wake-up, D3 read pressed |
| 11 (countstop) | Interrupt wake-ups (`n1`–`n4`, R3 `M`): moves of 1 to 65 535 pulses stopped by `stop_at`, armed before and during a move, at 1 000, 30 000 and 55 733/s; one move woken only by the count stop | every move exactly its count and quiet after it, the refusals as documented, R4 count = R3 edges |
| 11 (dirlead) | Cart-pole dogfooding fix (`d1`–`d4`, R3 `L`): each DIR change on D5 timed to the next STEP pulse, for starts through `run` (±1 000/s, ±120 000/s) and `queue` (from stopped, through idle) | `run` leads 3.5–7 µs as read (the 5 µs set-up; the R3 reads up to 1.56 µs short), `queue` leads ≥ 41 µs, every DIR change seen, every prediction exact |
| 11 (elc) | Slice 11 step 1 (`L`): ELC software event 0 routed to GPT166's count-up input, 1/100/10000 events fired, then 10 more after `unlink`. R4-internal; the R3 reads the reply | count = events each time; the unrouted 10 not counted |
| knock | 1200-baud open of the native port | bridge back, R4 silent (bootloader held) |
| `!` / `@` | R3 RESET pulses | `!` restarts the validator; `@` holds the bootloader |

Build from `crates/test-firmware/`. Like every binary there, the image uses
the crate's `memory.x` (FLASH ORIGIN 0x4000):

```powershell
cargo objcopy --release --features usb,uart-irq --bin r4_validator -- -O binary r4_validator.bin
```

Build the three R3 parts and flash the R4, from the repo root. Upload the
R3 while the R4's SCI2 TX is quiet: the R4 in bootloader, or already
running `r4_validator`.

```powershell
arduino-cli compile --fqbn arduino:avr:uno --build-property compiler.cpp.extra_flags=-DR3_PART=1 --build-path target/validation-r3-part1 arduino/validation/r3-validator
arduino-cli compile --fqbn arduino:avr:uno --build-property compiler.cpp.extra_flags=-DR3_PART=2 --build-path target/validation-r3-part2 arduino/validation/r3-validator
arduino-cli compile --fqbn arduino:avr:uno --build-property compiler.cpp.extra_flags=-DR3_PART=3 --build-path target/validation-r3-part3 arduino/validation/r3-validator
arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file crates/test-firmware/r4_validator.bin
```

Session 1: upload part 1 and run, from the repo root. The CLI finds the
R3 (2341:0043), the bridge (2341:1002) and the native port (2341:006D) by
USB identity, and reads the part from the R3:

```powershell
arduino-cli upload -p $uartPort --fqbn arduino:avr:uno --input-file target/validation-r3-part1/r3-validator.ino.hex --verify
cargo run --release -p validation -- all
```

Sessions 2 and 3: upload each part and run again. The R4 keeps running
`r4_validator` (every session ends with `!`, which restarts it):

```powershell
arduino-cli upload -p $uartPort --fqbn arduino:avr:uno --input-file target/validation-r3-part2/r3-validator.ino.hex --verify
cargo run --release -p validation -- all
```

```powershell
arduino-cli upload -p $uartPort --fqbn arduino:avr:uno --input-file target/validation-r3-part3/r3-validator.ino.hex --verify
cargo run --release -p validation -- all
```

R4 D2 is free/open: the push button is no longer part of the bench
(AGENTS.md). The R3 acts as the Slice 2 button on the existing D3 line.
R4 D4 → R3 D8 (STEP) was added on 2026-11-03 for the Slice 11 step-generator
rig; the `S11 step`, `count`, `tick`, `queue`, `scenario`, `reverse`,
`wake`, `skip` and `dirlead` checks use it. R4 D5 → R3 D7 (DIR) was added
on 2026-11-04 for `S11 reverse`; `S11 skip` and `S11 dirlead` use it too.

`validate all --skip-usb` leaves out Slice 9 and the knock. Per-slice runs:
`cargo run --release -p validation -- slices S2` (also `s4`, `usb`,
`reset --single|--double`, `identify`, `s10 [--resets N]`). `slices` runs
the loaded part's checks and shows the other parts' as NOT RUN; `s4`,
`usb` and `s10` need part 1 and refuse the others. Without the CLI, open the
R3 at 9600 and send `a` for the loaded part's checks, or one letter per
check (the sketch header lists them per part). Results go to
`target/validation/`.

Slice 10's power cycles (the done-when's ~20) need hands. The
R4 runs from its barrel jack with **USB unplugged**; the R3 stays on USB
(it must stay powered while D4 is wired to the R4's RESET, AGENTS.md).
Then, from the repo root:

```powershell
cargo run --release -p validation -- s10 --watch 900
```

Within the 900 s, unplug and replug the jack about 20 times, a few
seconds apart. Each replug must print one `S10 BOOT <count> 1 1 1` line
(counter, intact, +1, after a gap). `--min-boots` changes the 20.
Unverified: while the R4 is unpowered, the R3's idle-HIGH TX (D1) and
D3 pull-up still reach R4 pins. If that back-powers the R4 enough to keep
it running, no `GAP`/`BOOT` appears and the check fails visibly.

**Witnessed on WiFi, 2026-10-20:** 14/14 checks passed in one host run, on
the HAL ported to `embedded-io` 0.7.1:

- **S1:** phases 20 008–20 012 µs.
- **S2 button:** press seen 100 ms after the request, held 199 ms.
- **S5:** 4096/4096 bytes, `OV 0 ER 0`.
- **S5 traits:** 4096/4096 bytes through `embedded_io::Read`/`Write`,
  `OV 0 ER 0`.
  - The blocking `read` path runs at least once per run: the R4 calls
    `read` before the R3 starts sending.
  - How often it waited after that is not instrumented.
- **Host tests (same day):** 60 pass on each map with `usb,uart-irq`.
  - Two of them check the 0.7 write contract on the CDC port: trait
    `write` returns a non-zero count and trait `write_all` completes.
  - The CDC `Ok(0)` guard itself cannot be reached through
    `usbd-serial` 0.2.2, which reports a full buffer as `WouldBlock`.
- **S6:** `OK\r\n`.
- **S7:** duty 249/499/749 per mille at 998–1000 µs.
- **S8:** means 10/1047/2055/3070/4095.
- **S9:** 5 consecutive ticks and 6152 echoed bytes each direction, with the
  bridge absent.
- **Knock, `!` and `@`:** passed.

Minima builds, but is hardware-unverified.

**Witnessed on WiFi, 2026-10-21 (Slice 10, sandbox worktree):** 16/16
checks passed in one `validate all` run, Slices 1–9 unchanged:

- **Read-only probe first** (a temporary image, research §7.1, nothing
  programmed): at boot `DFLCTL = 00h` (`DFLEN` resets to 0 and the
  bootloader leaves it clear), `FENTRYR = 0000h`, `FENTRYR_MF4 = 0000h`,
  `FISR = 0`, `FPMCR = 08h`. After `DataFlash::take`: `DFLCTL = 01h`,
  `FISR = 17h` (`PCKA` = 23, writable only in P/E mode, so the entry
  sequence worked), `FPSR.PERR = 0`. All eight blocks blank-checked
  blank, `FENTRYR` back to `0000h` after every session, and two 8 KB
  dumps matched byte for byte. Every byte read `FFh`.
- **S10:** 1024 bytes programmed in 44 ms (≈ 43 µs/byte; datasheet
  Table 2.73 typical 42.1–52.4 µs), the programmed block erased in 6 ms
  (typical 6.42–8.98 ms), 0 mismatches, rewrite refused.
- **S10 retention:** 3/3 resets each +1 with the log intact (counter
  7 → 10). A separate `validate s10 --resets 5` run: 5/5 (2 → 7), each
  boot after a `GAP`.
- **Host tests:** 70 pass on each map with `usb,uart-irq` (10 new
  `dflash`), 12 in `validation` (3 new).
- **Re-run the same day after moving the SRAM helpers from a `.ram_code`
  `memory.x` section to `.data.*` input sections** (stock `memory.x`):
  16/16 again. S10: 44 ms program, 6 ms erase, 0 mismatches. Retention
  3/3 (13 → 16). The log written by the earlier image read back intact
  through the new one, across the firmware upload.
- **Not yet witnessed:** the ~20 power cycles (`validate s10 --watch`).

**Witnessed on WiFi, 2026-11-03 (rebuilt bench):** the bench had been
taken apart and was rewired from AGENTS.md's list, plus the new R4 D4 →
R3 D8 wire. R4 on COM3, R3 on COM4.

- **Before the new check:** 16/16 checks passed in one `validate all` run
  with the 2026-10-21 images. S1 phases 20 016–20 024 µs, S10 retention
  3/3 (63 → 66).
- **With the `S11 wire` check added:** 17/17 in one `validate all` run.
  S11 wire: read 0 and 1, 100/100 edges, periods 1005–1006 µs. S1 phases
  20 020–20 028 µs, S10 program 44 ms / erase 6 ms / 0 mismatches,
  retention 3/3 (69 → 72), S9 6152 bytes each direction.
- **Host tests (same day):** 13 pass in `validation` (1 new).
- **With Slice 11 step 1 (`S11 elc`) added, same day:** 18/18 in one
  `validate all` run. S11 elc: 1/1, 100/100, 10000/10000, unrouted events
  not counted. Host tests: 73 on the largest feature set (3 new `elc`).
- **With Slice 11 step 2 (`S11 step`, replacing `S11 wire`), same day:**
  18/18 in one `validate all` run; the step evidence is in the Slice 11
  record above. Host tests: 79 on the largest feature set (6 new `step`).
- **With Slice 11 step 3 (`S11 count`) added, same day:** 19/19 in one
  `validate all` run. S11 count: 20/20 runs equal, 72 444 pulses.
- **With Slice 11 step 4 (`S11 tick`) added, same day:** 20/20 in one
  `validate all` run. Host tests: 83 on the largest feature set (4 new:
  the per-tick pulse maths against brute force, the partition over 1 000
  ticks, and the measured boundary).
- **With Slice 11 step 5a (`S11 queue`) added, same day:** 21/21 in one
  `validate all` run. Host tests: 85 on the largest feature set (2 new:
  `Schedule` against the closed form and against a pulse-by-pulse walk).
- **With Slice 11 step 5c (`S11 scenario`) added, same day:** 22/22 in one
  `validate all` run.

**Witnessed on WiFi, 2026-11-04 (new wire R4 D5 → R3 D7 for DIR):**

- **With Slice 11 step 5b (`S11 reverse`) added:** 3 × `slices S11` at 7/7
  before the first full run. That full run was 21/23: the one-edge
  miscount in `S11 count` and `S11 tick` (Slice 11 record). After the R3
  gained interval logging: 3 × `slices S11` at 7/7 and 2 × `validate all`
  at 23/23. S11 reverse evidence is in the Slice 11 record.
- **After the repo-rule sweep (structure and docs only), same day:**
  23/23 in one `validate all` run with the rebuilt R4 image.
- **Host tests:** 85 on the largest feature set (no new ones in 5b; its
  checks are on the board), 13 in `validation`.
- **The miscount hunt, same day:** 20 × `slices S11`; run 19 reproduced it
  (Slice 11, follow-up 1).
- **With the `stop()` fix, the stop bursts in `S11 count`, and `S11 wake`
  added, same day:** 24/24 in one `validate all` run. S11 count: 20/20
  runs equal and 0 cut pulses in 10 000 stops. S11 wake: 2 500 ticks,
  wake ≤ 92 counts, queue done ≤ 1 440 counts. S11 reverse W1: old pulse
  first 14 times. An unoptimised (`dev` profile) R4 image with the fixed
  `stop()`, built before the wake check existed, passed `slices S11` 7/7
  with 0 cut pulses. Host tests unchanged: 85 and 13 (these checks are on
  the board).
- **After the header pin model (pins named by header, drivers checked by
  trait bounds; CHANGELOG 0.2.0), same day:** 24/24 in one `validate all`
  run with the rebuilt R4 image; the R3 sketch and the checks are
  unchanged. S11 count: totals R4 72 332 = R3 72 332, 0 cut pulses in
  10 000 stops. S11 wake: R4 214 053 = R3 214 053 pulses. Host tests: 86
  on the largest feature set (the PWM golden test now checks the pin
  table's `Gtioc` facts; 1 new: AN019/AN020 on WiFi D10/D13), 13 in
  `validation`.
- **After the step builders, ENA, switch inputs and the missed-tick
  recovery (CHANGELOG 0.2.0), same day:** 25/25 in one `validate all` run
  (`S11 skip` new). The validator builds its generator with
  `StepGenerator::builder` (counter on 16-bit GPT162). S11 count: totals
  R4 73 073 = R3 73 073, 0 cut pulses in 10 000 stops. S11 wake: R4
  214 054 = R3 214 054. S11 skip: 3/3, position R4 −252 = R3 −252 across
  20 missed reversals. ENA and switch inputs are host-tested only (not
  run on the board). Host tests: 138 on the largest feature set (34
  builder checks with full messages, 12 recovery), 13 in `validation`.
- **After the builder-shaped setup and `with_peripherals` (CHANGELOG
  0.2.0), same day:** 25/25 in one `validate all` run. The validator's
  generator is now `StepGeneratorBuilder::new().with_pul(…).with_dir(…)
  .with_counter(…).with_tick(…).build()` and
  `StepGenerator::new(SETUP).with_peripherals(p).build()`. `Peripherals`
  has a used/unused type per pin and timer. S11 count: totals R4 73 050 =
  R3 73 050, 0 cut pulses in 10 000 stops. S11 wake: R4 214 058 = R3
  214 058. S11 skip 3/3. Host tests: 139 on the largest feature set;
  harness outside the repo 52/52, including a pin claimed twice failing
  with `` `D5` is already used by another driver ``.
- **After the cart-pole dogfooding fix (the DIR set-up before the first
  pulse, CHANGELOG 0.2.0), same day:** 26/26 in one `validate all` run
  (`S11 dirlead` new). S11 count: totals R4 73 084 = R3 73 084, 0 cut
  pulses in 10 000 stops. S11 wake: R4 214 053 = R3 214 053. S11 dirlead
  4/4: `run` leads 4.63–5.06 µs as read, `queue` leads ≥ 1 178 µs, 0
  prediction misses. Host tests: 149 on the largest feature set (9 new in
  `tests/start.rs`, the DIR set-up limit in `tests/builders.rs`), 13 in
  `validation`.
- **After the interrupt wake-ups (switch wake-up and count stop, CHANGELOG
  0.2.0), same day:** 28/28 in one `validate all` run (`S2 wake` and
  `S11 countstop` new). S2 wake: `w 1 1 1`. S11 countstop 4/4: R4 556 = R3
  556, 96 546 = 96 546, 6 240 = 6 240, 2 000 = 2 000 (mode 4 woken once).
  S11 count: totals R4 73 105 = R3 73 105. S11 wake: R4 214 045 = R3
  214 045. Host tests: 165 on the largest feature set; harness outside the
  repo 55/55.
- **After splitting the R3 validator into two parts, same day:** part 1
  uses 11 864 bytes of flash (36 %), part 2 26 108 (80 %), from
  32 220 (99.9 %) in one image. The checks are unchanged; only their
  grouping moved. R4 image unchanged.
  - **Session 1 (part 1):** 16/17 run, 11 NOT RUN (Slice 11). S1 phases
    20 012–20 020 µs, S2 wake `w 1 1 1`, S10 44 ms / 6 ms / 0 mismatches,
    retention 3/3 (222 → 225), S4, knock, `!`, `@` passed. **S9 FAILED:**
    the R4 took native USB (`STATUS usb=2 taken=1`, the bridge gone) and
    COM7 appeared, but opening it failed with "The system cannot find the
    file specified." (the error the CLI now reports instead of dropping
    it). Repeated with `validate usb` alone. Opened by hand 6 s after `u`,
    COM7 opened and the R4's `tick` lines arrived. S9 passed in the 28/28
    run earlier the same day.
  - **S9 fix:** the CLI retries opening the native port for up to 10 s
    (`NATIVE_OPEN_WAIT`) and reports how long it took. **Session 1
    re-run: 17/17.** S9: opened after 227 ms, 5 consecutive ticks (max gap
    1.001 s), 9 exact echoes, 6 152 bytes each direction. S1 phases
    20 012–20 016 µs, retention 3/3 (232 → 235). With session 2's 13/13
    (11 Slice 11 checks), all 28 checks pass over the two sessions.
  - **Session 2 (part 2):** 13/13 run, 15 NOT RUN (Slices 1–10 and the
    host checks). All 11 Slice 11 checks passed: S11 count totals R4
    73 089 = R3 73 089, S11 wake R4 214 051 = R3 214 051, S11 countstop
    4/4, S11 dirlead 4/4, S11 skip 3/3; `!` and `@` passed.
  - Host tests: 20 in `validation` (7 new: the part line, which part runs
    each check, `NOT RUN` and the summary).
- **With the never-run checks (`S2 pullup`, `S11 cancelrace`, `S11
  sleep`), same day:** new R4 image. R3 part 1 12 076 bytes (37 %), part
  2 28 082 (87 %).
  - **Session 2:** 15/15 run. S11 cancelrace 680/1 320/0, R4 14 345 = R3
    14 345. S11 sleep 3/3 (above). S11 count totals R4 72 656 = R3
    72 656. S11 wake R4 214 048 = R3 214 048.
  - **Session 1:** 18/18 run. S2 pullup `e 0 0 1 1 1 1`. S9 opened after
    226 ms. Retention 3/3 (243 → 246).
  - All 31 checks pass over the two sessions. Host tests: 20 in
    `validation`; R4 firmware clippy clean on both maps, and `cargo fmt`
    run over it (it was not fmt-clean before).
- **With `S11 boot` and the two HAL fixes (`cancel_stop_at`, the
  rebuild after missed ticks), same day:** new R4 image; R3 part 2 29 570
  bytes (91 %).
  - **Session 2:** 16/16 run. S11 boot leads 212–216 µs, falls 36 = 36.
    S11 cancelrace 670/1 330/0. S11 sleep 3/3, every `queue` at the
    first poll. S11 count totals R4 72 584 = R3 72 584. S11 wake R4
    214 048 = R3 214 048.
  - **Session 1:** 18/18 run. S2 pullup `e 0 0 1 1 1 1`, S9 opened after
    226 ms, retention 3/3 (258 → 261).
  - All 32 checks pass over the two sessions. Host tests: the HAL matrix
    passes (`just test`), with 4 new resync tests and 2 changed; 20 in
    `validation`.
- **With the R3 validator in three parts, the cancel race randomised and
  the delay probe, same day:** final R4 image. R3 part 1 12 076 bytes
  (37 %), part 2 24 126 (74 %), part 3 13 584 (42 %).
  - **Session 3:** 7/7 run. S11 cancelrace 666/1 334/0, R4 14 459 = R3
    14 459. S11 sleep 3/3, S11 boot leads 212–216 µs, falls 36 = 36.
    Delay probe 250 251 250 252.
  - **Session 2:** 11/11 run. S11 count totals R4 73 077 = R3 73 077. S11
    wake R4 214 049 = R3 214 049.
  - **Session 1:** 18/18 run. S2 pullup `e 0 0 1 1 1 1`, S9 opened after
    228 ms, retention 3/3 (281 → 284).
  - All 32 checks pass over the three sessions. Host tests: 20 in
    `validation` (the part tests now cover three parts).
- **With the wake timer (Slice 12), claimable ELC inputs and the ENA lead
  fix, same day:** new R4 image; R3 part 3 14 702 bytes (45 %).
  - **Session 3:** 8/8 run. S12 wake: wire 1 012 µs / 100 080 µs, `t1 9 0
    0 12 0 1 2`, `t2 1 1 1 1 1 1`. S11 boot leads 212 µs, ENA 331 counts
    (lead 240). S11 countstop 4/4. S11 cancelrace 711/1 289/0.
  - **Session 2:** 11/11 run. S11 count totals R4 73 104 = R3 73 104. S11
    wake R4 214 049 = R3 214 049. S11 elc on the claimed input A: 1/1,
    100/100, 10 000/10 000.
  - **Session 1:** 18/18 run. S9 opened after 227 ms, retention 3/3
    (302 → 305).
  - All 33 checks pass over the three sessions. Host tests: the HAL matrix
    (`just test`), with 11 wake-timer and 4 ENA tests; 21 in `validation`;
    the harness outside the repo 62/62 (7 new: the wake builder's missing
    slots, an ELC input or channel already held by the step generator, an
    input claimed twice, `Elc::link` without a claimed input, and a
    passing stepper + switch + wake timer + ELC firmware).

Two earlier runs showed an S6 reply of 7 bytes with no `OK` (the first run,
before the hex dump existed), and an SCI2 `er=235` total. That total came
after an R3 upload, whose 115200-baud traffic reaches the R4's 9600 RX; the
burst check reports per-check deltas instead.

