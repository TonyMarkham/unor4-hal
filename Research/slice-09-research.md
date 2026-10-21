# Slice 9 research — USB CDC (`usb.rs`)

> **2026-10-20:** the Slice 9 binaries named here (`slice09_cdc.rs`, `slice09_probe.rs` and the other `slice09_*` programs) were never committed and were deleted when the per-slice programs were retired. The current check is the validation pair (README, All-in-one validation; S9).

> Planning notes assembled 2026-10-20, before any Slice 9 code. Verified
> facts are cited to the hardware manual or to the vendored `ra4m1` PAC;
> cross-check data (Arduino core, TinyUSB, board datasheet) is labeled as
> such. Open items and owner decisions are named explicitly. No code has
> been written.

Manual citations refer to **R01UH0887EJ0100 Rev.1.00** (the PDF/MD in
`Research/`), chapter 27 "USB 2.0 Full-Speed Module (USBFS)" unless
another chapter is named.

## What the slice is

`usb.rs` (feature `usb`, opt-in per HalSpec §3): a `usb-device` bus adapter
(`UsbBus` impl) for the RA4M1 USBFS, CDC-ACM via `usbd-serial`, and the
explicit `take_usb_port()` mux handling. Binary: `slice09_cdc.rs` —
enumerates as a CDC device, echoes, prints ticks. Observable: a **new**
COM port, no adapter, bridge port gone while held. Done-when: descriptor
visible in Device Manager, hour-long tick run with no port drop, and
*upload still works* after replugging. This is the largest single slice —
HalSpec OQ-3 gates it.

## 1. OQ-3 answered: no existing `usb-device` adapter — this is the big build

Researched 2026-10-20:

- The `usb-device` crate's official adapter list
  (rust-embedded-community/usb-device README) is: atsam4, atsamd,
  imxrt-usbd, musb, rp2040-hal, stm32-usbd. **No Renesas, no RA.**
- All ~130 reverse dependencies of `usb-device` on crates.io were reviewed
  by name: no Renesas/RA/UNO R4 crate.
- GitHub repo search `usb-device+renesas` (0 results) and `ra4m1+usb`
  (2 results, neither Rust). GitHub *code* search was not run (auth
  required) — small residual uncertainty, but the crates.io check is
  strong evidence.
- Community UNO R4 Rust write-ups (domwil.co.uk series) stop at GPIO/UART/
  CAN — no USB work.

**The one Rust USB driver that exists for this silicon** is
[`ra-hal`](https://crates.io/crates/ra-hal) 0.3.1 (2026-06), which
implements **`embassy-usb-driver`** (not `usb-device`) for USBFS. It is
**LGPL-3.0-or-later** → per the repo's license audit rule it can never be
a dependency, and per AGENTS.md it is a behavioral reference only, never
transcribed. Its predecessor `ra4-hal` 0.2.3 declares the dependency but
ships no `usb` module. `arduino-uno-r4-hal` 0.2.0 has no USB at all.
`musb` (a `usb-device` adapter) targets the Mentor musb IP — the RA USBFS
is Renesas's own IP, not applicable.

**Decision (per HalSpec §5.5's preference): build the `usb-device::UsbBus`
adapter ourselves in `usb.rs`.** The tinyusb-via-C-ABI fallback stays
documented but is not taken. The `ra-hal` embassy port and TinyUSB's
`rusb2` RA port (below) are the two behavioral cross-checks.

## 2. What the proven C stack tells us the silicon can do

The Arduino core (`ArduinoCore-renesas`) runs **TinyUSB 0.15.0** on the
RA4M1 via upstream's Renesas port
`tinyusb/src/portable/renesas/rusb2/` (`dcd_rusb2.c` + `rusb2_ra.h`;
core submodule pinned at `ecfa7a0`, port symlinked from
`cores/arduino/tinyusb/rusb2`). Facts from that port (behavioral
references, not transcribed):

- **10 pipes** (hardware endpoint channels): pipe 0 = control, 64 B
  single-buffered; pipes 1–2 = bulk/isochronous, ≤2 KB double-buffered;
  pipes 3–5 = bulk, ≤2 KB double-buffered; pipes 6–9 = interrupt, 64 B
  single-buffered. (Manual §27.3.4/§27.3.10–27.3.12 are the source of
  truth for pipe classes; the port's allocation matches.)
- **No DMA.** CPU copies packets through the FIFO registers (CFIFO for
  EP0/DCP, D0FIFO/D1FIFO for other pipes) via 8/16-bit accesses. OUT bulk
  uses the hardware transaction counters (`PIPEnTRE`/`PIPEnTRN`).
- **Full-speed only on RA4M1** (`RUSB2_SUPPORT_HIGHSPEED` gated to RA6M5/
  RA6M3/series-8).
- **Clock setup is NOT in the dcd** (`rusb2_phy_init()` is empty for RA) —
  the 48 MHz UCLK must be configured before `dcd_init` (see §4).
- **SOF is disabled** (`USE_SOF 0`, "this interrupt too often (1ms)").
  SOF is only toggled to wake from suspend. A v1 implementation does not
  need SOF.
- The core points **4 IELSR slots** (EVENT_USBFS_INT, RESUME, FIFO_0,
  FIFO_1) at one handler that calls `tud_int_handler(0)` + `tud_task()` in
  ISR context. We need at minimum **one** slot (see §6).

## 3. The `ra4m1` PAC covers USBFS completely

Verified against the vendored source
(`~/.cargo/registry/src/…/ra4m1-0.2.0/src/usbfs/`): **all 33 registers of
manual §27.2 are present** — `syscfg`, `syssts0`, `dvstctr0`,
`cfifo/cfifoctr/cfifol/cfifosel`, `d0fifo*/d1fifo*`, `intenb0/1`,
`intsts0/1`, `brdyenb/brdysts`, `nrdyenb/nrdysts`, `bempenb/bempsts`,
`sofcfg`, `frmnum`, `usbreq/usbval/usbindx/usbleng`, `dcpcfg/dcpctr/
dcpmaxp`, `pipesel/pipecfg/pipectr/pipemaxp/pipeperi/pipetre/pipetrn`,
`devadd`, `usbmc`, `usbbcctrl0`. No gap to work around. (`ra4m1` 0.2.0 is
already a workspace dep; license MIT OR Apache-2.0, per OQ-5.)

## 4. Bring-up sequence (manual facts, in order)

All register addresses below are from the PAC/manual; the order is the
manual's constraints plus the core's proven order (cross-check:
`cores/arduino/usb/USB.cpp __USBStart()` + FSP clock config):

1. **HOCO at 48 MHz** — already the workspace's clock since Slice 1
   (`Clocks`). UCLK must be 48 MHz (manual §8.3: "A 48-MHz clock must be
   supplied to the USBFS module").
2. **Select UCLK = HOCO**: `SYSTEM.USBCKCR` (0x4001_E0D0), bit 0
   `USBCLKSEL`: 0 = PLL (reset), 1 = HOCO (§8.2.24 p.151). Constraints
   from the manual: rewrite **while `SYSCFG.SCKE` = 0** (i.e. before
   releasing the module stop / before SCKE=1); `USBCLKSEL = 1` is legal
   **only in device-controller mode**; the HOCO user-trimming function is
   unavailable while `USBCLKSEL = 1`. SYSTEM registers are PRCR-protected
   — same unlock pattern `clock.rs` already uses (verify the PRC key bits
   for the clock group at implementation time).
3. **Release module stop**: `MSTPCRB.MSTPB11` (chapter 10, Low Power
   Modes, module-stop registers, p.~172–173; bit text names USBFS as the
   target module). Core cross-check: `USB.cpp` clears MSTPCRB bit 11 (and
   bit 12, the high-speed module — harmless on RA4M1).
4. **USB regulator on**: `USBFS.USBMC` (0x4009_00CC) bit 7 `VDCEN = 1`
   (§27.2.34 p.642). Bit 0 `VDDUSBE` is only for battery-charging
   detection — board does not use BC, leave 0 (§27.3.1.4 Figure 27.4, the
   "BC not used" wiring has VDCEN=1, VDDUSBE=0). USBMC's reset value is
   0x0002: bit 1 is reserved-read-as-1, **writes must set it**. Core
   cross-check: both variants set `VDCEN = 1` in
   `usb_post_initialization()`.
5. **Device mode + clock enable** in `USBFS.SYSCFG` (0x4009_0000, §27.2.1
   p.598): `DCFM = 0` (device; reset value 0 — must only be changed while
   `DPRPU`/`DMRPU`/`DRPD` are all 0, true after reset), then `SCKE = 1`
   (48 MHz clock enable) with a **read-back confirm** (register note 2),
   then `USBE = 1`. While `SCKE = 0` only SYSCFG may be touched.
6. **VBUS**: with `VDCEN = 1`, `INTSTS0.VBSTS` reflects the bus. The R4 is
   bus-powered; TinyUSB's dcd gates `DPRPU` on `VBSTS` (connect only when
   VBUS present). **OQ-USB-4:** whether the UNO R4 WiFi routes VBUS to the
   RA4M1's USB_VBUS pin (the analog muxes carry D+/D−; VBUS sensing
   through the switch path is unverified) — first hardware probe decides
   (§8 step 2). Fallback if VBSTS never sets: connect unconditionally.
7. **Connect**: `SYSCFG.DPRPU = 1` pulls up D+ and the host sees an
   attach (§27.3.1.3 p.644). Dropping it signals detach — this is also
   what a reboot-into-bootloader path must do before resetting (core's
   `goBootloader()` clears `DPRPU` first).
8. **Configure DCP + pipes and interrupt enables** (INTENB0, BRDYENB,
   BEMPENB) — see §5.

## 5. Adapter design

### 5.1 Pipe plan for CDC-ACM

| usb-device endpoint | USB type | RA pipe | Buffering |
|---|---|---|---|
| EP0 (control) | control | DCP | 64 B, single, CFIFO port |
| CDC notification IN (interrupt, 8 B) | interrupt IN | pipe 6 | 64 B, single |
| CDC bulk IN 64 B | bulk IN | pipe 1 | ≤2 KB, double |
| CDC bulk OUT 64 B | bulk OUT | pipe 2 | ≤2 KB, double |

Pipes are configured via `PIPESEL`→`PIPECFG`/`PIPEMAXP` (§27.2.26–28);
direction and endpoint number are fixed per pipe in `PIPECFG`. The
adapter keeps an EP-number↔pipe table and serves `alloc_ep` from the free
list (bulk from 1–5, interrupt from 6–9), rejecting over-allocation as
`usb-device` expects.

### 5.2 Data flow

No DMA: `UsbBus::read`/`write` copy through the FIFO port registers
(D0FIFO/D1FIFO with `FIFOSEL.MBW` 16-bit for bulk, CFIFO for DCP), using
BRDY (buffer ready) / BEMP (buffer empty) status in `BRDYSTS`/`BEMPSTS`
per pipe. Bulk OUT can use `PIPEnTRN` transaction counting. Control
transfers: `usb-device` parses the requests; the bus adapter must move
EP0 packets at the right stage boundaries — driven by `INTSTS0.CTSQ`
(control transfer stage, §27.3.3.5) in polled mode or the CTRT interrupt
in IRQ mode. This stage machine is the fiddliest part of the adapter;
`dcd_rusb2.c` is the behavioral cross-check.

### 5.3 `UsbBus` surface (usb-device 0.3.2)

Methods to implement: `enable`, `reset`, `set_address`, `poll`,
`alloc_ep`, `enable_ep`, `disable_ep`, `read`, `write`, `flush`, `stall`,
`set_stalled`, `is_stalled`. Bus reset detection: `INTSTS0.DVSQ` state
transitions (DVST source, §27.3.3.4). Suspend/resume: report
`PollResult::Suspend/Resume` from `DVSQ` — minimal, SOF not needed.

### 5.4 Polled first, IRQ later (recommendation)

`usb-device`'s canonical model is `UsbDevice::poll()` from the main loop.
**Recommend v1 is fully polled** (status registers read each poll; no
IELSR slot, no `intc` changes), and the IRQ variant —
`Event::UsbfsInt = 0x33` linked through the existing `intc` machinery —
lands as a follow-up if the hour-long run or latency demands it. The
USBFS raises **one combined interrupt** (`USBFS_USBI`) carrying
VBINT/RESM/SOFR/DVST/CTRT/BEMP/NRDY/BRDY (§27.3.2 Table 27.13 +
Figure 27.11); resume has its own signal (`USBFS_USBR`, event 0x34). The
`Event` enum already carries all four USBFS events (0x31 D0FIFO,
0x32 D1FIFO — DMA/DTC requests, not needed for v1 — 0x33 INT, 0x34
RESUME), cross-checked against the ICU chapter event table. The Arduino
core links 4 IELSR slots to one handler; we would need only slot(s) for
0x33 (+0x34 if resume handling is wanted).

Trait matrix (HalSpec §6): CDC gets `embedded-io` Read/Write +
`core::fmt::Write` on a wrapper type with blocking semantics (poll-until-
accepted), mirroring the `uart-irq` module's pattern.

## 6. The mux — `take_usb_port()`

- **P408 is the switch control** (verified: `variants/UNOWIFIR4/
  variant.cpp` pin 21 comment "USB switch, drive high for RA4"; Arduino
  ABX00087 datasheet §6.2: U2/U6 analog switches route the connector's
  D+/D− to ESP32-S3 or RA4M1, driven by P408 high). By default (reset)
  nothing drives P408 → the ESP32-S3 bridge owns the connector (it
  enumerates as 2341:1002 CMSIS-DAP and bridges Serial1). **Minima has no
  mux** — connector wired directly to the RA4M1 (verified by absence of
  mux code in the MINIMA variant + datasheet) → on `minima`,
  `take_usb_port()` is a no-op returning the same guard type.
- API per HalSpec §5.5: explicit `take_usb_port() -> UsbPortGuard`,
  documented as disconnecting the bridge and killing the upload path
  while held; docs reference SJ1 (solder jumper). The core additionally
  writes `R_SYSTEM->VBTBKR[1] = 40` (a VBAT backup register) right before
  driving P408 high — **semantics unverified** (OQ-USB-1: likely a
  handoff marker for the bridge/bootloader; recommend replicating the
  write since the core always does it, marked unverified until probed).
- **Upload-recovery design decision (OQ-USB-2)** — the owner must pick:
  firmware that flips the mux unconditionally at boot makes uploads
  nearly impossible (the 1200-baud touch and the bridge both live on the
  ESP32 side of the switch). Options: (a) fixed window after reset
  before `take_usb_port()` takes effect, (b) gate the flip on Button 0
  (D2 — already on the bench), (c) unconditional + rely on the double-
  tap reset. The Done-when "upload still works after replugging" needs a
  deterministic window → (a) or (b) recommended. The binary decides; the
  library only requires the call to be explicit.

## 7. Dependencies (license audit)

| Crate | Version | License | Notes |
|---|---|---|---|
| `usb-device` | 0.3.2 (2024-03) | **MIT** | no required deps (only optional `log`); no MSRV declared; edition 2018 |
| `usbd-serial` | 0.2.2 (2024-04) | **MIT** | depends on `usb-device` 0.3 |

Both MIT → covered by the repo's single MIT file (README dependency
table gets two rows). `usb-device`/`usbd-serial` are library (optional,
behind feature `usb`) deps of `unor4-hal`; the test-firmware binary calls
`UsbDevice::new`/`UsbDeviceBuilder` itself, so `test-firmware` also needs
`usb-device` as a direct (workspace-pinned) dep with a matching version,
plus a `usb` feature forwarded to the lib. CI's build matrix grows the
`usb on/off` axis (HalSpec §7.1).

## 8. Bring-up plan (differential, each step observable)

1. **Probe binary** (`slice09_probe` style, like slice 8's): steps 1–5 of
   §4, then dump `USBCKCR`, `SYSCFG`, `USBMC`, `SYSSTS0`, `INTSTS0`
   (incl. `VBSTS`) to the SCI9 monitor **before** touching `DPRPU`.
   Answers OQ-USB-4 (VBUS sensing) and OQ-USB-5 (regulator) from state,
   not theory.
2. **Attach only**: `DPRPU = 1` with no descriptors configured → Device
   Manager shows a device that fails to enumerate (or an unknown device).
   Proves PHY, pull-up, and the mux flip.
3. **EP0-only enumeration**: minimal `UsbBus` + control pipe → device
   enumerates with the CDC descriptors (`lsusb`/Device Manager shows
   them) but class endpoints may not stream yet.
4. **Full CDC**: `usbd-serial` on top, echo + `tick N` — the slice's
   observable.
5. **Stamina + recovery**: hour-long tick run with no port drop; upload
   still works after replug (tests OQ-USB-2's window).

No new bench wiring is needed — the USB connector is the observable, and
slice 10 (data flash) needs no wires either, so this slice adds nothing
to the bench plan.

## 9. Owner decisions (resolved 2026-10-20)

Governing principle agreed with the owner: **`usb.rs` ships mechanisms
only — every policy (mux window, badge, driver model, upload knock) is a
decision the consumer's `main()` makes.** The crate never picks a
behavior implicitly.

1. **Mux window:** caller-owned. `take_usb_port()` never fires on its
   own; the application waits as long as it wants (its own `DelayNs`,
   or a button check) before calling. The slice binary demos a 3 s
   window; no crate-level policy exists.
2. **VID/PID:** default Arduino's VID/PID, overridable per call via the
   builder. Owner accepts the badge-borrowing implication; reversible
   before first publish. **Implementation-time check:** the exact PID
   the Arduino core uses for R4 CDC must be verified from the core's
   variant headers, never from memory.
3. **Polled vs IRQ:** both exposed. The consumer drives `poll()` from
   its own loop whenever it wants (the `usb-device` contract); an
   optional `intc`-linked wake on `Event::UsbfsInt` (0x33) is available
   for consumers who want the bell. The demo binary starts polled.
4. **Bootloader knock (1200 baud + DTR low → detach + reboot):** an
   opt-in method on the CDC wrapper (`enable_bootloader_knock()`-
   shaped). The crate never arms it implicitly. Lands in slice 9
   because it directly strengthens the Done-when upload test; if the
   slice balloons, the knock can split into an immediate follow-up
   increment without any API change.

## 10. Hardware findings — probe run 1 (2026-10-20, WiFi board)

`slice09_probe.rs` brought the USBFS up in stages and streamed register
state at 1 Hz. Everything below is measured, not inferred.

### 10.1 OQ answers

- **OQ-USB-6 (PRCR):** `PRCR = 0xA503` (PRC0+PRC1) unlocks `USBCKCR`;
  read-back shows `USBCLKSEL = 1` sticks. **But the window must also
  cover the module-stop write** — see 10.2.
- **OQ-USB-4 (VBUS):** **VBUS IS sensed on the WiFi board**
  (`INTSTS0.VBSTS = 1` with the mux still on the ESP32 side). VBUS is
  not switched by U2/U6 — only D+/D− are. VBUS-gated connect logic is
  usable as-is.
- **OQ-USB-5 (regulator):** `USBMC = 0x0082` (VDCEN + reserved b1)
  sticks.
- **OQ-USB-1 (VBTBKR[1] = 40):** written before the flip (no PRCR
  protection needed for VBTBKR — §11.2.6 shows none; the PRC3 notes in
  the manual belong to the LVD registers). Semantics still unknown, but
  the flip works with it.

  > **Correction (2026-10-20, later the same day):** the protection
  > claim is wrong. Manual §12.1's protection table lists `VBTBKRn`
  > (n = 0 to 511) among the **PRC1**-protected battery-backup
  > registers. §11.2.6 just doesn't repeat it.
  >
  > Measured with `slice09_rig`:
  > - With PRCR locked, byte writes of the boot magic read back
  >   `00 00 00 00` (VBTCR1 = 1, VBTSR = 0x11).
  > - With PRC1 unlocked (`PRCR = 0xA502`), the same writes read back
  >   exactly.
  >
  > `UsbPort::take` now unlocks PRC1 for its marker write. The
  > marker's semantics are answered in the §10.7 correction.

### 10.2 Bring-up recipe corrections (the library MUST follow these)

1. **`MSTPCRB` is PRC1-protected** — a write with PRCR locked is
   silently ignored (the first run proved it: release dropped, all
   USBFS registers read 0, even `USBMC`'s 0x0002 reset value gone).
   Same `PRCR` window as `USBCKCR`, exactly as `uart.rs`'s
   `release_module_stop_sciX` already does. The HAL knew; my probe
   didn't.
2. **`MSTPCRB` requires 32-bit access** — narrow (u16) accesses to the
   MSTP block are dropped by the bus even with PRC1 unlocked. The PAC's
   `mstp.mstpcrb` (32-bit) is correct; the bus table ("0x00 32", read/
   0xFFFF) agrees. A u16 RMW read also returns the reset pattern.
3. **PFS bit layout: b0 = PODR (output latch), b1 = PIDR (pin state)**
   (§19, p.357). Raw reads of "the pin" must use bit 1 or the port
   block's `PIDR` (what `gpio::Pin::read` does). The probe's first
   button check read the latch and always saw "pressed"; the mux was
   first driven with PDR+PIDR (no-op) instead of PDR+PODR.

### 10.3 Verified sequence (works end-to-end)

PRCR unlock (0xA503) → `USBCKCR.USBCLKSEL = 1` + `MSTPCRB.MSTPB11 = 0`
(32-bit, same window) → relock → `SYSCFG = 0x0400` (SCKE, read-back
confirm) → `USBMC = 0x0082` → `SYSCFG |= 0x0001` (USBE) → module alive
(state: `syscfg=0411` with DPRPU, `usbmc=0082`, `vbus=1`).

### 10.4 The mux cycle (proven, fully software-reversible)

`VBTBKR[1] = 40` → P408 high (PFS: PODR+PDR, PWPR unlock) → **the
host drops the bridge** (COM3 gone; `VID_2341&PID_1002` goes Unknown in
PnP within ~100 ms of boot) → RA4M1 attach (DPRPU=1) → **the host
resets the bus and sends a SETUP packet the USBFS actually receives**
(`INTSTS0`: VALID=1, CTSQ=1 control-read stage, DVST/VBINT latched) —
enumeration fails only because nothing answers EP0 yet → detach
(DPRPU=0) + P408 low → **the bridge re-enumerates by itself** (COM3
back, PnP OK, no physical intervention). Upload after a full cycle
succeeded (`arduino-cli upload`, 4 pages, Done).

Pre-flip, the attached RA4M1 sees only its own pull-up: `dvsq=4`
(suspended), `lnst=1` (J-state) — exactly the alone-on-the-bus
signature. Post-flip-with-host: reset + setup reception. Both sides of
the mux behave as the manual/datasheet predict.

### 10.5 What this means for `usb.rs`

- The §4 bring-up sequence is hardware-verified; the library can
  implement it with the PAC (32-bit `mstp` access, PRC1 window) as-is.
- The mux (`take_usb_port`) mechanics are proven including automatic
  bridge recovery on release — the windowed approach from owner
  decision 1 is viable, and the "upload still works" Done-when has a
  proven mechanism (window + release, or the CDC knock later).
- The remaining work is exactly the adapter: EP0/DCP stage machine
  (the host's setup packet is already arriving) + pipe config + the
  `usb-device` trait surface.

### 10.6 `usb-device` 0.3.2 integration facts (source-read, vendored)

- **`UsbBus::enable()` is never called by usb-device 0.3.2** — the only
  `enable` call in the crate is `UsbBusAllocator::enable()` (bus.rs:177),
  an explicit helper; `UsbDevice::poll`/`build` do not invoke it, and
  `UsbDevice::build` starts at `device_state: Default`. Consequence: the
  adapter must connect (DPRPU) itself — done lazily in `poll()` (VBSTS-
  gated), which is the only connect path in this HAL.

  > **Correction (2026-10-20, later the same day):** wrong. usb-device
  > 0.3.2 **does** call `UsbBus::enable()` during the build:
  > `UsbDevice::build` calls `alloc.freeze()` (`device.rs:104`), and
  > `UsbBusAllocator::freeze` calls `self.bus.borrow_mut().enable()`
  > (`bus.rs:172–177`). The line this note cited is that call, not a
  > separate helper.
  >
  > The host test
  > `given_allocator_when_device_built_then_usb_bus_enable_is_called`
  > (`src/tests/usb.rs`) pins the behaviour. `Usbfs::enable` connects
  > DPRPU when VBUS is present. The lazy VBSTS-gated connect in `poll()`
  > remains only for VBUS that arrives after the build.
- **`UsbDevice::build` pins the control endpoint addresses**
  (`alloc(Some(0x00.into()), Control, …)` / `0x80`) and `.expect()`s the
  result — an adapter that rejects pinned addresses panics the
  application right after the mux flip, where the monitor path is dead
  (this was slice 9's first silent failure; found by bisection).
- Builder strings go through `.strings(&[StringDescriptors::new(LangID)
  .product(…)])` (the builder has no `.product`); `LangID` is re-exported
  at the crate root (`usb_device::LangID`).
- `max_packet_size_0` accepts only 8/16/32/64 (BuilderError otherwise).

### 10.7 The bridge and reset (open)

After the CDC firmware held the mux, a **reset did not bring the bridge
back** within 45 s of polling; a double-tap reset did (and re-flashing
worked). Working hypothesis (OQ-USB-1): the `VBTBKR[1] = 40` marker —
which `UsbPort::take` replicated from the core and which survives reset
in the VBATT domain — tells the bridge to stay off until cleared (power
cycle) or until software releases the mux. If confirmed, the marker
write should be dropped or cleared; the mux flip itself does not need it
(untested without).

> **Correction (2026-10-20, later the same day):** the hypothesis is
> disproved. The marker is read by the RA4M1's Arduino bootloader, not
> the bridge, and it asks for the switch to be **restored**.
>
> The installed bootloader was read back from flash and is
> byte-identical to `bootloaders/UNO_R4/dfu_wifi.hex` from
> ArduinoCore-renesas 1.6.0 (the magic literal is at 0x22DC). It was
> disassembled as a behavioural reference only, never transcribed. At
> 0x2160–0x21BC, if `VBTBKR[1] == 40`, it:
>
> 1. makes P408 an output and drives it high (PORT4 PCNTR3 POSR);
> 2. calls a delay with argument 1000 (unit unverified);
> 3. drives P408 low (PORR) and returns it to input;
> 4. clears the byte.
>
> At 0x21E8–0x21F0 it compares one 32-bit load of `VBTBKR0..3` with
> 0x07738135. If they are equal, it clears the magic and stays in the
> bootloader; otherwise it runs the double-tap window and boots the
> application. On this silicon that 32-bit load returns `VBTBKR0` as
> its **most significant** byte: bytes `35 81 73 07` read back as
> 0x35817307. The HAL's knock therefore writes the magic big-endian,
> still with 8-bit accesses.
>
> Observed on the bench (README, Slice 9):
> - A double-tap RESET returns the bridge with the bootloader held.
> - The 1200-baud knock returns the bridge with the bootloader held,
>   and a standard bridge upload then succeeds.
> - An upload after a physical replug succeeded.
>
> The 45 s failure recorded above was not re-tested in its original
> form.

## 11. Open questions

- **OQ-USB-1** `VBTBKR[1] = 40` marker semantics (core writes it before
  flipping the mux). Unverified; replicate-and-probe.
  **Update 2026-10-20:** partly answered. The installed bootloader
  treats 40 as a request to restore the USB switch at the next boot
  (§10.7 correction). What it does for the bridge beyond the P408
  sequence, and the delay unit, remain unverified.
- **OQ-USB-2** Mux-capture window mechanism for keeping uploads alive
  (owner decision, §6).
- **OQ-USB-3** VID/PID values for our CDC device (owner decision; do not
  blindly copy Arduino's).
- **OQ-USB-4** Is VBUS sensed on the R4 WiFi (`INTSTS0.VBSTS` meaningful
  through the switch path)? Probe first (§8 step 1).
- **OQ-USB-5** Does the board use the internal USB LDO (VDCEN=1)? The
  core sets it on both variants; confirm against the R4 schematic or the
  §8 step-1 dump.
- **OQ-USB-6** `PRC1`-key unlock for `USBCKCR` — confirm the exact PRCR
  bits from the manual's PRC chapter at implementation (pattern already
  exists in `clock.rs`).
- Carried: OQ-1's USB half is answered by this research (UCLK = HOCO 48
  MHz via USBCKCR, §4) — record in HalSpec when the slice closes.
