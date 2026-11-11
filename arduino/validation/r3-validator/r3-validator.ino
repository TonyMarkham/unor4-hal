// r3-validator — R3 side of the all-in-one unor4-hal validation pair.
// Peer: crates/test-firmware/src/bin/r4_validator/ (one R4 image for Slices 1-10).
// Host: the validate CLI (crates/validation) drives this sketch, and also
// checks what only a PC can see (4: the bridge monitor, 9: native USB) or
// time (10: the boot counter across resets and power cycles).
//
// Wiring (the bench in AGENTS.md, unchanged):
//   R3 D0/D1 <-> R4 D1/D0  request link, 9600 8N1 (shared with this USB port)
//   R3 D2    <-  R4 D6     Slice 7 PWM, measured on INT0
//   R3 D3    <-> R4 D3     Slice 2 input + button (R3 drives) / Slice 1 timing (R4 drives)
//   R3 D4    ->  R4 RESET  LOW pulses only, released to input, never HIGH
//   R3 D5    ->  R4 A0     Slice 8 stimulus (Timer0 PWM, ~976 Hz)
//   R3 D8    <-  R4 D4     Slice 11 STEP (GPT320), timestamped by Timer1 ICP1
//   R3 D7    <-  R4 D5     Slice 11 DIR, read at each STEP pulse
//   R3 GND   <-> R4 GND
// R4 D2 is free/open: the Slice 2 button is this sketch pressing D3.
// Keep the R3 powered whenever D4 is connected to the R4's RESET.
// D3 is INPUT_PULLUP on both boards except while one side drives it for a
// test it asked for, so the boards never drive it against each other.
//
// Parts: all the checks no longer fit the ATmega328P's 32 KB, so this one
// source builds as three images, chosen by R3_PART at compile time; each
// is uploaded for its own validate session (Validation.md):
//   part 1: Slices 1-10 (S1-S8, S2 wake and pullup, S10 flash and the
//           boot-counter watch, S9's USB take-over);
//   part 2: Slice 11's first checks (step, count, tick, queue, scenario,
//           reverse, wake, skip, ELC);
//   part 3: Slice 11's later checks (dirlead, countstop, cancelrace,
//           sleep, boot) and the delay probe.
// Parts 2 and 3 share the Slice 11 code; the linker drops the functions a
// part never calls (-ffunction-sections, --gc-sections), so each pays only
// for its own checks. Every part answers ? i s ! @ and a.
//   arduino-cli compile --fqbn arduino:avr:uno
//     --build-property compiler.cpp.extra_flags=-DR3_PART=1 ...
// Without R3_PART the build stops with an #error.
//
// PC commands (9600 8N1, no line ending; lowercase so no R4 reply can
// ever look like one). Every part: ? i s ! @ and a (that part's checks).
//   ?  help: "r3-validator part <n> of 3: see header"
//   i  identify the R4
//   a  run every automatic check of this part. Part 1: S3, S1, S2 input,
//      S2 button, S2 wake, S2 pullup, S4, S5 burst, S5 traits, S6-S8,
//      S10 flash. Part 2: S11 step, count, tick, queue, scenario,
//      reverse, wake, skip, elc. Part 3: S11 dirlead, countstop,
//      cancelrace, sleep, S12 wake, S11 boot (last: it resets the R4).
// Part 1 only:
//   l  S3  link: 10 identify round trips
//   c  S1  clock/GPIO/Delay: R4 square wave on D3, timed here on INT1
//   g  S2  input: drive D3 low/high, R4 reads it
//   b  S2  button: this sketch "presses" D3 (open-drain LOW for 200 ms,
//          then releases it); the R4 times press and release
//   m  S4  monitor token: R4 prints it on SCI9 (host checks the bridge)
//   e  S5  4096-byte burst echo through the R4's interrupt-driven SCI2
//   t  S5  the same burst, echoed only through the HAL's embedded-io
//          Read/Write traits (blocking read, write_all, flush)
//   w  S6  WiFi AT channel: R4 asks the ESP32-S3 for OK
//   p  S7  PWM on D6: 0/25/50/75/100 %, period and duty measured on INT0
//   n  S8  ADC A0: D5 duty 0/64/127/191/255, R4 averages A0
//   u  S9  take native USB on the R4 (bridge drops; knock armed)
//   f  S10 data flash self-test on block 6: erase, blank check, program
//          1024 bytes, read back, refused rewrite, erase, blank check
//   k  S10 boot-counter watch: polls K until x; reports BASE, GAP and
//          BOOT lines. Inside the watch only x (end) and ! (reset the
//          R4) are obeyed. Power-cycling the R4 (jack, USB off) or ! each
//          must show one BOOT with the counter increased and intact.
//   W  S2  wake: the R4 asleep in WFI with D3 as a switch until this
//          sketch presses it
//   E  S2  pull-up: the same switch with Pull::External, this sketch's
//          INPUT_PULLUP the only pull-up on D3
// Part 2 only (part 3's after it):
//   d  S11 step 2: the R4's step generator on D4, measured here on D8 by
//          Timer1 input capture: periods at 1, 1000, 44600 and 55733/s,
//          each pulse's LOW at 1 and 1000/s, and 40 stops at random phases
//          (pin HIGH after, no cut pulse, no pulse after)
//   q  S11 step 3: the R4's hardware pulse count (N) against the edges
//          captured here, over 20 start/stop runs at 1000, 44600 and
//          55733/s with Z1-Z3 and T0 stops
//   v  S11 step 1: ELC software events counted by the R4's GPT166
//          (1, 100, 10000 events; R4-internal, reply only)
// Part 3 only:
//   D      the delay probe: the R4 times cortex_m::asm::delay on its
//          SysTick clock (m), reply printed as DELAY <reply>
//   L  S11 dirlead: each DIR change on D7 timed to the next STEP pulse on
//          D8, for starts through run (at least the 5 us DIR set-up) and
//          through queue (at least the 40 us queue guard)
//   M  S11 countstop: moves of an exact pulse count, stopped in hardware;
//          the R4's count against the STEP edges captured here
//   j h o r z K  S11 tick, queue, scenario, reverse, wake, skip
//   C  S11 cancelrace: cancel_stop_at at swept moments around the count
//          match; the R4's count against the STEP edges captured here
//   S  S11 sleep: 100 ms / 500 ms / 3 s without a call at a steady rate,
//          then a queue; count and DIR-signed position against D8/D7
//   B  S11 boot: resets the R4 and watches its boot-time setup (STEP on
//          D3 = GPT321 A, DIR on D5 with a 200 us set-up, ENA on D2 with
//          a 5 us lead, freed while running): STEP falls on INT1, DIR
//          changes on PCINT23
//   T  S12 wake: the R4's wake timer (GPT167): its own lengths and
//          behaviour checks (t1, t2), then two waits marked on D3 (t3),
//          timed here on INT1
// Both parts:
//   s      R4 status: USB state, taken flag, uptime, SCI2 ov/er
//   !      single RESET pulse          @  double RESET pulse (bootloader)
// The reset pulses need no R4 firmware. They reach this sketch only while
// the R4 is not driving SCI2 TX; r4_validator releases it between
// replies and the bootloader leaves it idle.
//
// Link contract: requests and replies are STX (0x02) body ETX (0x03)
// frames. Reply bodies contain only A-Z, 0-9 and spaces. This sketch's
// report lines also reach the R4, which ignores everything outside a
// frame. One request is outstanding at a time.
//
// Every check prints one line: "S<n> PASS|FAIL <evidence>". S4 prints
// "S4 SENT TOKEN <t>" because only the host can see the bridge monitor.
// "a" ends with "DONE <pass> PASS <fail> FAIL".

#ifndef R3_PART
#error "build a part: --build-property compiler.cpp.extra_flags=-DR3_PART=1 (or 2, 3); see Validation.md"
#elif R3_PART < 1 || R3_PART > 3
#error "R3_PART must be 1, 2 or 3"
#endif

const uint8_t PWM_IN = 2;     // INT0, from R4 D6
const uint8_t SHARED = 3;      // INT1, R4 D3
const uint8_t RESET_PIN = 4;   // to R4 RESET
const uint8_t ANALOG_OUT = 5;  // to R4 A0
const uint8_t STEP_IN = 8;     // ICP1 (PB0), from R4 D4 (STEP)
const uint8_t DIR_IN = 7;      // PD7, from R4 D5 (DIR)
const uint8_t STX = 0x02;
const uint8_t ETX = 0x03;
const uint16_t BURST = 4096;
const char PATTERN[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

char reply[64];
uint8_t replyLen = 0;
uint8_t passes = 0;
uint8_t failures = 0;

// ---- RESET pulses: 10 ms LOW, then released to input; never HIGH ---------

void pulse() {
  digitalWrite(RESET_PIN, LOW);
  pinMode(RESET_PIN, OUTPUT);
  delay(10);
  pinMode(RESET_PIN, INPUT);
}

// ---- the request link --------------------------------------------------

// Collects one reply frame. `inFrame` = the STX was already consumed.
bool awaitReply(uint32_t timeoutMs, bool inFrame) {
  const uint32_t start = millis();
  replyLen = 0;
  while (millis() - start < timeoutMs) {
    if (!Serial.available()) continue;
    const int c = Serial.read();
    if (c == STX) {
      inFrame = true;
      replyLen = 0;
    } else if (c == ETX && inFrame) {
      reply[replyLen] = 0;
      return true;
    } else if (inFrame) {
      if (replyLen < sizeof(reply) - 1) reply[replyLen++] = c;
      else inFrame = false;
    }
  }
  reply[0] = 0;
  return false;
}

bool request(const char *body, uint32_t timeoutMs) {
  while (Serial.available()) Serial.read();
  Serial.write(STX);
  Serial.print(body);
  Serial.write(ETX);
  return awaitReply(timeoutMs, false);
}

// The reply starts with `prefix` (e.g. "G ").
bool replyIs(const char *prefix) {
  return strncmp(reply, prefix, strlen(prefix)) == 0;
}

// Space-separated field `index` of the reply as a number (field 0 is the
// letter). Returns -1 when absent.
long field(uint8_t index) {
  uint8_t at = 0;
  for (uint8_t f = 0; f < index; f++) {
    while (reply[at] && reply[at] != ' ') at++;
    if (!reply[at]) return -1;
    at++;
  }
  if (reply[at] < '0' || reply[at] > '9') return -1;
  return atol(reply + at);
}

void verdict(uint8_t slice, bool pass) {
  Serial.print(F("S"));
  Serial.print(slice);
  Serial.print(pass ? F(" PASS ") : F(" FAIL "));
  if (pass) passes++;
  else failures++;
}

void noReply(uint8_t slice) {
  verdict(slice, false);
  Serial.println(F("no reply from R4"));
}

#if R3_PART == 1

// ---- Slice 3: the link -------------------------------------------------

void checkLink() {
  uint8_t good = 0;
  for (uint8_t i = 0; i < 10; i++) {
    if (request("I", 500) && replyIs("ID VALIDATOR 1")) good++;
  }
  verdict(3, good == 10);
  Serial.print(good);
  Serial.print(F("/10 identify round trips; last: "));
  Serial.println(reply);
}

// ---- Slice 1: clock + GPIO + Delay, timed on INT1 ----------------------

volatile uint32_t edges[24];
volatile uint8_t edgeCount = 0;

void edgeIsr() {
  if (edgeCount < 24) edges[edgeCount++] = micros();
}

void checkClock() {
  pinMode(SHARED, INPUT_PULLUP);
  attachInterrupt(digitalPinToInterrupt(SHARED), edgeIsr, CHANGE);
  // An edge latched in EIFR while INT1 was off fires once on attach.
  delay(2);
  noInterrupts();
  edgeCount = 0;
  interrupts();
  const bool answered = request("D", 3000);
  delay(5);
  detachInterrupt(digitalPinToInterrupt(SHARED));
  if (!answered) { noReply(1); return; }
  // R4: low (1 edge), ten high/low cycles (20 edges), release (1 edge);
  // every phase 20 ms of the HAL's Delay at 48 MHz.
  uint32_t shortest = 0xFFFFFFFF, longest = 0;
  const uint8_t count = edgeCount;
  for (uint8_t i = 1; i < count; i++) {
    const uint32_t phase = edges[i] - edges[i - 1];
    if (phase < shortest) shortest = phase;
    if (phase > longest) longest = phase;
  }
  const bool pass = count == 22 && shortest >= 19600 && longest <= 20400;
  verdict(1, pass);
  Serial.print(F("edges="));
  Serial.print(count);
  Serial.print(F("/22 phase="));
  Serial.print(count > 1 ? shortest : 0);
  Serial.print(F(".."));
  Serial.print(longest);
  Serial.println(F("us (want 20000 +-2%)"));
}

// ---- Slice 2: input ------------------------------------------------------

void checkInput() {
  digitalWrite(SHARED, LOW);  // pull-up off before driving
  pinMode(SHARED, OUTPUT);
  delay(5);
  const bool low = request("G", 500) && replyIs("G 0");
  digitalWrite(SHARED, HIGH);
  delay(5);
  const bool high = request("G", 500) && replyIs("G 1");
  pinMode(SHARED, INPUT_PULLUP);
  verdict(2, low && high);
  Serial.print(F("R4 D3 read "));
  Serial.print(low ? F("0") : F("?"));
  Serial.print(F(" when driven low, "));
  Serial.print(high ? F("1") : F("?"));
  Serial.println(F(" when driven high"));
}

// A button press, the way a switch to GND makes one: drive LOW, then
// release to input (never drive HIGH); the R4's pull-up restores HIGH.
// Sends request `letter`, then "presses" D3 the way a switch to GND would:
// open-drain LOW from 100 ms after the request for 200 ms, then released.
// Returns whether the R4 replied.
bool pressD3(const char *letter) {
  pinMode(SHARED, INPUT_PULLUP);
  while (Serial.available()) Serial.read();
  Serial.write(STX);
  Serial.print(letter);
  Serial.write(ETX);
  Serial.flush();  // the request, and any queued report text, has left
  delay(100);      // the R4 is now watching D3
  digitalWrite(SHARED, LOW);
  pinMode(SHARED, OUTPUT);
  delay(200);
  pinMode(SHARED, INPUT_PULLUP);
  return awaitReply(3000, false);
}

void checkButton() {
  if (!pressD3("B")) { noReply(2); return; }
  // R4: press seen ~100 ms after the request; held ~200 ms (its 20 ms
  // debounce wait is inside the hold).
  const long press = field(2);
  const long hold = field(3);
  const bool pass = replyIs("B OK") && press >= 50 && press <= 300 && hold >= 150 && hold <= 300;
  verdict(2, pass);
  Serial.print(F("button on D3: press after "));
  Serial.print(press);
  Serial.print(F(" ms, held "));
  Serial.print(hold);
  Serial.print(F(" ms (want ~100 / ~200); R4: "));
  Serial.println(reply);
}

// The switch wake-up: the R4 sleeps in WFI with D3 as a switch (with_edge,
// with_wake) until this sketch presses it. The R4 must wake on the press,
// still see it in activated() and read D3 pressed.
void checkSwitchWake() {
  const bool answered = pressD3("w") && replyIs("w ");
  verdict(2, answered && field(1) == 1 && field(3) == 1);
  Serial.print(F("switch wake: D3 woke the R4 from WFI; R4: "));
  Serial.println(answered ? reply : "none");
}

// The switch with an external pull-up: the R4 builds the D3 switch with
// Pull::External, so its own pull-up is off and this sketch's INPUT_PULLUP
// (20-50 kOhm) is the only one. The R4 must read its pull-up off, D3 idle
// (not active), wake on the press, and see D3 go back to idle on the
// release, which only this pull-up drives.
void checkExternalPullUp() {
  const bool answered = pressD3("e") && replyIs("e ");
  verdict(2, answered && field(1) == 0 && field(2) == 0 && field(3) == 1 &&
                 field(5) == 1 && field(6) == 1);
  Serial.print(F("switch pullup: R3 pull-up only; R4: "));
  Serial.println(answered ? reply : "none");
}

// ---- Slice 4: the SCI9 monitor (seen by the host) ----------------------

void checkMonitor() {
  if (!request("M", 500) || !replyIs("M ")) { noReply(4); return; }
  Serial.print(F("S4 SENT TOKEN "));
  Serial.println(field(1));
}

// ---- Slice 5: interrupt-driven burst echo ------------------------------

// One burst exchange: `letter` E = the R4's ring echo, R = its echo through
// the embedded-io Read/Write traits. Same framing and pass rule for both.
void echoCheck(char letter, const __FlashStringHelper *label) {
  char body[8];
  char ready[8];
  snprintf(body, sizeof(body), "%c%u", letter, BURST);
  snprintf(ready, sizeof(ready), "%c READY", letter);
  if (!request(body, 1000) || !replyIs(ready)) { noReply(5); return; }
  uint16_t sent = 0, got = 0, wrong = 0;
  bool frame = false;
  uint32_t last = millis();
  while ((sent < BURST || got < BURST) && millis() - last < 2000 && !frame) {
    if (sent < BURST) Serial.write(PATTERN[sent++ % 36]);
    while (Serial.available()) {
      const int c = Serial.read();
      if (c == STX) { frame = true; break; }  // R4 finished early
      if (c != PATTERN[got % 36]) wrong++;
      got++;
      last = millis();
    }
  }
  const bool answered = awaitReply(3000, frame);
  const bool pass = answered && got == BURST && wrong == 0 &&
                    field(1) == BURST && field(3) == 0 && field(5) == 0;
  Serial.println();
  verdict(5, pass);
  Serial.print(label);
  Serial.print(F("echoed "));
  Serial.print(got);
  Serial.print(F("/"));
  Serial.print(BURST);
  Serial.print(F(" wrong="));
  Serial.print(wrong);
  Serial.print(F(" R4: "));
  Serial.println(answered ? reply : "no final reply");
}

void checkBurst() { echoCheck('E', F("")); }
void checkTraits() { echoCheck('R', F("traits: ")); }

// ---- Slice 6: the WiFi AT channel --------------------------------------

void checkAt() {
  if (!request("A", 3000)) { noReply(6); return; }
  if (replyIs("AT NA")) {
    Serial.println(F("S6 NA no AT channel on this board"));
    return;
  }
  verdict(6, field(1) == 1);
  Serial.print(F("ESP32-S3 answered: "));
  Serial.println(reply);
}

// ---- Slice 7: PWM on D6, measured on INT0 ------------------------------

volatile uint32_t riseAt = 0, fallAt = 0, sumHigh = 0, sumPeriod = 0, periods = 0;
volatile uint8_t armed = 0, fell = 0;

void pwmIsr() {
  const uint32_t now = micros();
  if (PIND & _BV(PIND2)) {
    if (armed) {
      const uint32_t period = now - riseAt;
      sumPeriod += period;
      sumHigh += fell ? fallAt - riseAt : period;
      periods++;
    }
    riseAt = now;
    armed = 1;
    fell = 0;
  } else {
    fallAt = now;
    fell = 1;
  }
}

void checkPwm() {
  const uint16_t steps[5] = {0, 250, 500, 750, 1000};
  bool all = true;
  pinMode(PWM_IN, INPUT);
  for (uint8_t s = 0; s < 5; s++) {
    char body[8];
    snprintf(body, sizeof(body), "P%u", steps[s]);
    if (!request(body, 500) || !replyIs("P ")) { noReply(7); return; }
    delay(20);
    noInterrupts();
    armed = fell = 0;
    sumHigh = sumPeriod = periods = 0;
    interrupts();
    attachInterrupt(digitalPinToInterrupt(PWM_IN), pwmIsr, CHANGE);
    delay(250);
    detachInterrupt(digitalPinToInterrupt(PWM_IN));
    bool pass;
    Serial.print(F("  P "));
    Serial.print(steps[s]);
    Serial.print(F("/1000: "));
    if (steps[s] == 0 || steps[s] == 1000) {
      // 0 % and 100 % are DC lines (Slice 7: OADTY / no compare match).
      const bool level = digitalRead(PWM_IN);
      pass = periods == 0 && level == (steps[s] == 1000);
      Serial.print(F("edges="));
      Serial.print(periods);
      Serial.println(level ? F(" flat HIGH") : F(" flat LOW"));
    } else {
      const uint32_t meanPeriod = periods ? sumPeriod / periods : 0;
      const uint32_t duty = sumPeriod ? (uint32_t)(((uint64_t)sumHigh * 1000ULL) / sumPeriod) : 0;
      pass = periods >= 200 && meanPeriod >= 990 && meanPeriod <= 1010 &&
             duty + 10 >= steps[s] && duty <= steps[s] + 10;
      Serial.print(F("period="));
      Serial.print(meanPeriod);
      Serial.print(F("us duty="));
      Serial.print(duty);
      Serial.print(F("/1000 periods="));
      Serial.println(periods);
    }
    all = all && pass;
  }
  request("P0", 500);
  verdict(7, all);
  Serial.println(F("D6 1 kHz: period 1000+-10us, duty +-10/1000, 0/100% flat"));
}

// ---- Slice 8: ADC on A0 --------------------------------------------------

void checkAdc() {
  const uint8_t duties[5] = {0, 64, 127, 191, 255};
  bool all = true;
  for (uint8_t s = 0; s < 5; s++) {
    analogWrite(ANALOG_OUT, duties[s]);
    delay(50);
    if (!request("C200", 1500) || !replyIs("C ")) {
      digitalWrite(ANALOG_OUT, LOW);
      noReply(8);
      return;
    }
    // Mean of a PWM wave over whole periods = duty x VREF (Slice 8).
    const long expected = (long)duties[s] * 4095L / 255L;
    const long mean = field(1);
    const bool pass = mean >= 0 && labs(mean - expected) <= 82 && field(5) == 0;
    all = all && pass;
    Serial.print(F("  D5 duty "));
    Serial.print(duties[s]);
    Serial.print(F(": A0 mean="));
    Serial.print(mean);
    Serial.print(F(" want "));
    Serial.print(expected);
    Serial.print(F("+-82 n="));
    Serial.println(field(2));
  }
  digitalWrite(ANALOG_OUT, LOW);
  verdict(8, all);
  Serial.println(F("A0 12-bit window means within 2% of full scale"));
}

// ---- Slice 10: data flash -----------------------------------------------

void checkFlash() {
  // Two erases (<= 286 ms each, datasheet) plus 1024 programs: allow 5 s.
  if (!request("F", 5000)) { noReply(10); return; }
  // F OK <blank erase ms> <program ms> <erase ms> <mismatches>
  const bool pass = replyIs("F OK") && field(5) == 0;
  verdict(10, pass);
  Serial.print(F("flash self-test: program 1024 B "));
  Serial.print(field(3));
  Serial.print(F(" ms, erase "));
  Serial.print(field(4));
  Serial.print(F(" ms, mismatches "));
  Serial.print(field(5));
  Serial.print(F("; R4: "));
  Serial.println(reply);
}

#endif  // R3_PART == 1

#if R3_PART >= 2

// ---- Slice 11 step 2: the step generator, R4 D4 -> R3 D8 ------------------

// Timer1 at clk/8 (0.5 us ticks), normal mode, input capture on falling
// edges (ATmega328P datasheet section 16.6): the hardware latches TCNT1 into
// ICR1 at each STEP pulse start, so the ISR has a whole pulse period to run
// and a missed edge shows as a doubled interval. The overflow ISR extends
// the time to 32 bits for the 1 pulse/s check. With capWidths set, the ISR
// also polls D8 until it rises, timing each pulse's LOW (the fall is the
// hardware-latched ICR1; the rise is polled, about one tick late). When the
// ISR starts after the pulse has already ended (another interrupt delayed
// it), the width is unknown: that sample is counted as late and skipped,
// so a late ISR can never pass for a long pulse.
volatile uint16_t ovfHigh = 0;
volatile uint8_t capWidths = 0;
volatile uint32_t capN = 0;
volatile uint32_t capPrev = 0, capSum = 0, capMin = 0, capMax = 0;
volatile uint32_t lastRise = 0, gapMin = 0;
volatile uint16_t lowMin = 0, lowMax = 0, lowLate = 0, lastLow = 0;
volatile uint8_t riseKnown = 0, lastLate = 0;
// Burst starts (Slice 11 step 5a): an edge after a long interval (> 200 us)
// that is followed by a short one (< 50 us) — the first pulse of a fast
// burst, which the R4 places min_high after a tick.
volatile uint8_t capBursts = 0, burstN = 0, burstCandidate = 0;
// DIR at each STEP pulse (Slice 11 step 5b), counted when capDir is set.
volatile uint8_t capDir = 0;
volatile uint32_t dirHighCount = 0, dirLowCount = 0, dirUncertain = 0;
volatile uint32_t burstFrom = 0;
volatile uint32_t burstAt[24];

ISR(TIMER1_OVF_vect) { ovfHigh++; }

ISR(TIMER1_CAPT_vect) {
  const uint16_t icr = ICR1;
  // The pulse width first, before any arithmetic: the 5 us LOW is shorter
  // than the 32-bit bookkeeping below.
  const bool lowAtEntry = !(PINB & _BV(PINB0));
  // DIR (R4 D5 on D7 = PD7), right after: the R4 changes it only while STEP
  // is HIGH and at least 5 us before a pulse, and a reversal's write lands
  // ~6.8 us after the tick, after any pulse in flight has ended. Read while
  // the pulse is still LOW, DIR is that pulse's direction; read after it
  // ended, it may already be the new one (counted as uncertain).
  const bool dirHigh = PIND & _BV(PIND7);
  if (capDir) {
    if (dirHigh) dirHighCount++;
    else dirLowCount++;
    if (!lowAtEntry) dirUncertain++;
  }
  uint16_t rise = icr;
  if (capWidths && lowAtEntry) {
    rise = TCNT1;
    while (!(PINB & _BV(PINB0)) && (uint16_t)(rise - icr) < 400) rise = TCNT1;
  }
  uint16_t high = ovfHigh;
  // An overflow still pending behind this capture came before it when ICR1
  // is small.
  if ((TIFR1 & _BV(TOV1)) && icr < 0x8000) high++;
  const uint32_t at = ((uint32_t)high << 16) | icr;
  if (capN) {
    const uint32_t period = at - capPrev;
    capSum += period;
    if (period < capMin) capMin = period;
    if (period > capMax) capMax = period;
    if (capBursts) {
      if (burstCandidate && period < 100 && burstN < 24) burstAt[burstN++] = burstFrom;
      burstCandidate = period > 400;
      burstFrom = at;
    }
  }
  if (capWidths) {
    if (riseKnown) {
      const uint32_t gap = at - lastRise;
      if (gap < gapMin) gapMin = gap;
    }
    if (!lowAtEntry) {
      // Already HIGH: this ISR started after the pulse ended.
      lowLate++;
      lastLate = 1;
      riseKnown = 0;
    } else {
      const uint16_t low = rise - icr;
      if (low < lowMin) lowMin = low;
      if (low > lowMax) lowMax = low;
      lastLow = low;
      lastLate = 0;
      lastRise = at + low;
      riseKnown = 1;
    }
  }
  capPrev = at;
  capN++;
}

void captureStart(bool widths) {
  noInterrupts();
  TCCR1A = 0;
  TCCR1B = _BV(CS11);  // clk/8; ICES1 = 0: falling edge; no noise canceler
  TCNT1 = 0;
  ovfHigh = 0;
  TIFR1 = _BV(ICF1) | _BV(TOV1);
  capN = 0;
  capSum = 0;
  capMin = 0xFFFFFFFF;
  capMax = 0;
  gapMin = 0xFFFFFFFF;
  lowMin = 0xFFFF;
  lowMax = 0;
  lowLate = 0;
  lastLow = 0;
  lastLate = 0;
  riseKnown = 0;
  burstN = 0;
  burstCandidate = 0;
  dirHighCount = 0;
  dirLowCount = 0;
  dirUncertain = 0;
  capWidths = widths;
  TIMSK1 = _BV(ICIE1) | _BV(TOIE1);
  interrupts();
}

void captureStop() {
  noInterrupts();
  TIMSK1 = 0;
  interrupts();
}

uint32_t capturedCount() {
  noInterrupts();
  const uint32_t n = capN;
  interrupts();
  return n;
}

// Within tol of want, both in 1/48 us counts: the R4 period is `counts`
// PCLKD counts; one R3 tick (0.5 us) is 24 counts. tol covers the two
// boards' clock mismatch (0.5 %) plus `ticks` R3 ticks of quantisation.
bool near(uint32_t gotCounts, long counts, uint8_t ticks) {
  const long tol = counts / 200L + 24L * ticks;
  return labs((long)gotCounts - counts) <= tol;
}

// The repeated stop runs print only their failures.
bool stepQuiet = false;
// Set by stepRun when a zone stop failed only because the R3's interrupt
// started after the last pulse had ended, so that pulse could not be timed
// (the R3 is busy receiving the R4's reply then). Such a run is repeated.
bool stepInconclusive = false;

// One run: start `rate`, capture for `windowMs`, then stop with `stopBody`
// ("T0", or "Z<zone>" to stop inside a chosen phase). Prints one line
// (unless stepQuiet and it passed) and returns whether period, pulse shape
// (when `widths`) and the stop held. Pulse widths: the non-late samples
// must be 4.5-7.0 us (5.0 us plus the polled rise), at most a tenth of
// them late; a Z stop's last pulse must be measured and full width.
bool stepRun(uint32_t rate, uint16_t windowMs, bool widths, uint16_t minEdges,
             const char *stopBody) {
  char body[12];
  snprintf(body, sizeof(body), "T%lu", (unsigned long)rate);
  if (!request(body, 500) || !replyIs("T ") || field(1) <= 0) {
    Serial.print(F("  T "));
    Serial.print(rate);
    Serial.print(F("/s: no start, R4: "));
    Serial.println(reply);
    return false;
  }
  const long counts = field(1);
  captureStart(widths);
  delay(windowMs);
  const bool zoneStop = stopBody[0] == 'Z';
  // A zone stop must name its zone: "Z ERR" (zone not reached) is no stop.
  const bool stopped = request(stopBody, 1000) && replyIs(zoneStop ? "Z " : "T 0") &&
                       (!zoneStop || field(1) > 0);
  const uint32_t atStop = capturedCount();
  delay(5);
  captureStop();
  const bool restHigh = digitalRead(STEP_IN) == HIGH;
  const uint32_t n = capN;
  const bool quietAfter = n == atStop;
  // Mean period in PCLKD counts (one R3 tick = 24 counts), not truncated.
  const uint32_t meanCounts = n > 1 ? (uint32_t)(((uint64_t)capSum * 24ULL) / (n - 1)) : 0;
  bool pass = stopped && restHigh && quietAfter && n >= minEdges && near(meanCounts, counts, 1) &&
              near(capMin * 24UL, counts, 2) && near(capMax * 24UL, counts, 2);
  // 5.0 us low = 10 ticks: accepted 9..14 (4.5-7.0 us); gap >= 5 ticks.
  stepInconclusive = false;
  if (widths) {
    pass = pass && lowMin >= 9 && lowMax <= 14 && lowLate <= n / 10 + 2 && gapMin >= 5;
    if (zoneStop) {
      stepInconclusive = pass && lastLate;
      pass = pass && !lastLate && lastLow >= 9 && lastLow <= 14;
    }
  }
  if (pass && stepQuiet) return true;
  Serial.print(F("  T "));
  Serial.print(rate);
  Serial.print(F("/s "));
  Serial.print(stopBody);
  Serial.print(F(": n="));
  Serial.print(n);
  Serial.print(F(" period="));
  Serial.print(capMin / 2.0, 1);
  Serial.print(F(".."));
  Serial.print(capMax / 2.0, 1);
  Serial.print(F("us mean="));
  Serial.print(meanCounts / 48.0, 2);
  Serial.print(F(" want "));
  Serial.print(counts / 48.0, 2);
  if (widths) {
    Serial.print(F(" low="));
    Serial.print(lowMin / 2.0, 1);
    Serial.print(F(".."));
    Serial.print(lowMax / 2.0, 1);
    Serial.print(F("us late="));
    Serial.print(lowLate);
    Serial.print(F(" last="));
    Serial.print(lastLate ? -1.0 : lastLow / 2.0, 1);
    Serial.print(F(" gap>="));
    Serial.print(gapMin / 2.0, 1);
  }
  Serial.print(restHigh ? F(" stop HIGH") : F(" stop LOW"));
  Serial.print(quietAfter ? F(" quiet; R4: ") : F(" PULSES AFTER STOP; R4: "));
  Serial.println(reply);
  return pass;
}

void checkStep() {
  pinMode(STEP_IN, INPUT_PULLUP);
  // Timer1 borrowed from the core (its D9/D10 PWM is unused on this bench).
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  bool all = true;
  all &= stepRun(1000, 300, true, 250, "T0");     // shape + period at 1 kHz
  all &= stepRun(44600, 100, false, 4000, "T0");  // the rig's 24 V top rate
  all &= stepRun(55733, 100, false, 5000, "T0");  // the 30 V top rate
  all &= stepRun(1, 2300, true, 2, "T0");         // 1 pulse/s: one 1 s period
  // Stops inside chosen phases at 1 kHz, every pulse's LOW timed: 10 each
  // mid-pulse (Z1, the hardware stop), just after a pulse (Z2) and just
  // before the next (Z3); then 20 at random phases at 55 733/s (period,
  // rest level, silence after the stop).
  uint8_t stopsOk = 0;
  stepQuiet = true;
  for (uint8_t i = 0; i < 50; i++) {
    bool ok;
    if (i < 30) {
      const char *zones[3] = {"Z1", "Z2", "Z3"};
      ok = stepRun(1000, 5 + random(0, 15), true, 1, zones[i / 10]);
      for (uint8_t retry = 0; !ok && stepInconclusive && retry < 2; retry++) {
        ok = stepRun(1000, 5 + random(0, 15), true, 1, zones[i / 10]);
      }
    } else {
      ok = stepRun(55733, 2 + random(0, 18), false, 1, "T0");
    }
    if (ok) stopsOk++;
    delayMicroseconds(random(0, 2000));
  }
  stepQuiet = false;
  all &= stopsOk == 50;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, all);
  Serial.print(F("step: D4 STEP at 1, 1000, 44600, 55733/s; 5 us low; "));
  Serial.print(stopsOk);
  Serial.println(F("/50 stops clean (30 inside chosen phases, 20 random; HIGH, no cut pulse, quiet)"));
}

// ---- Slice 11 step 3: the hardware pulse count ------------------------------

// One run for the count check: read the R4's count (N), start `rate` with
// this sketch already capturing, run `windowMs`, stop with `stopBody`, read
// N again. The R4's difference must equal the edges captured here. The R3's
// own count is trusted only when no interval is 1.5 periods or longer (a
// missed edge doubles one); a run where the R3 lost an edge is reported as
// such, not as an R4 error.
bool countRun(uint32_t rate, uint16_t windowMs, const char *stopBody, long &r3Total,
              long &r4Total) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  captureStart(false);
  char body[12];
  snprintf(body, sizeof(body), "T%lu", (unsigned long)rate);
  if (!request(body, 500) || !replyIs("T ") || field(1) <= 0) {
    captureStop();
    return false;
  }
  const long counts = field(1);
  delay(windowMs);
  const bool zoneStop = stopBody[0] == 'Z';
  // A zone stop must name its zone: "Z ERR" (zone not reached) is no stop.
  const bool stopped = request(stopBody, 1000) && replyIs(zoneStop ? "Z " : "T 0") &&
                       (!zoneStop || field(1) > 0);
  delay(5);
  captureStop();
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  const bool r3Clean = capN < 2 || (long)capMax * 24L < counts * 3L / 2L;
  r3Total += r3;
  r4Total += r4;
  const bool pass = stopped && r3Clean && r3 == r4;
  if (!pass) {
    Serial.print(F("  count T "));
    Serial.print(rate);
    Serial.print(F("/s "));
    Serial.print(stopBody);
    Serial.print(F(": R4 "));
    Serial.print(r4);
    Serial.print(F(" R3 "));
    Serial.print(r3);
    // An extra R3 edge (noise) shows as a short interval; an edge the R4
    // did not count leaves every interval normal.
    Serial.print(F(" interval "));
    Serial.print(capMin / 2.0, 1);
    Serial.print(F(".."));
    Serial.print(capMax / 2.0, 1);
    Serial.print(F("us"));
    Serial.println(r3Clean ? F("") : F(" (R3 missed an edge)"));
  }
  return pass;
}

// One stop burst (Slice 11 follow-up): the R4 runs `cycles` start/stop
// cycles at 55 733/s, each stopped at a random phase (H<cycles>), and reports
// where each stop froze its counter. Here: every edge captured over the
// burst against the R4's count. A pulse cut by a stop is an edge the R3 sees
// and the R4 never counts (it counts pulse ends), so R3 = R4 + cuts.
bool stopBurst(uint16_t cycles, long &cutTotal, long &cycleTotal) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  captureStart(false);
  char body[8];
  snprintf(body, sizeof(body), "H%u", cycles);
  const bool answered = request(body, 5000) && replyIs("H ");
  char burstReply[64];
  strncpy(burstReply, reply, sizeof(burstReply));
  const long done = field(1), cut = field(2), atEnd = field(3), between = field(4);
  const long cutMax = field(5), betweenMax = field(6), period = field(7), r4 = field(8);
  delay(5);
  captureStop();
  const long r3 = capN;
  cutTotal += cut;
  cycleTotal += done;
  const bool pass = answered && done == cycles && cut == 0 && r3 == r4 && atEnd + between == done;
  Serial.print(F("  stop burst "));
  Serial.print(done);
  Serial.print(F(" cycles at 55733/s: cut "));
  Serial.print(cut);
  Serial.print(F(" (max frozen "));
  Serial.print(cutMax);
  Serial.print(F(") at a pulse end "));
  Serial.print(atEnd);
  Serial.print(F(" between pulses "));
  Serial.print(between);
  Serial.print(F(" (max frozen "));
  Serial.print(betweenMax);
  Serial.print(F(" of period "));
  Serial.print(period);
  Serial.print(F(") pulses R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F(" min interval "));
  Serial.print(capMin / 2.0, 1);
  Serial.print(F("us; R4: "));
  Serial.println(answered ? burstReply : "no reply");
  return pass;
}

void checkCount() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  long r3Total = 0, r4Total = 0;
  uint8_t ok = 0;
  const uint8_t runs = 20;
  for (uint8_t i = 0; i < runs; i++) {
    const char *zones[3] = {"Z1", "Z2", "Z3"};
    bool pass;
    if (i < 9) {
      pass = countRun(1000, 50 + random(0, 50), zones[i % 3], r3Total, r4Total);
    } else if (i < 12) {
      pass = countRun(44600, 200, "T0", r3Total, r4Total);
    } else if (i < 15) {
      pass = countRun(55733, 200, "T0", r3Total, r4Total);
    } else {
      pass = countRun(55733, 1 + random(0, 30), zones[i % 3], r3Total, r4Total);
    }
    if (pass) ok++;
  }
  // 5 bursts of 2 000 stops at random phases: no stop may cut a pulse.
  long cutTotal = 0, cycleTotal = 0;
  uint8_t burstsOk = 0;
  for (uint8_t i = 0; i < 5; i++) {
    if (stopBurst(2000, cutTotal, cycleTotal)) burstsOk++;
  }
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == runs && r3Total == r4Total && burstsOk == 5);
  Serial.print(F("count: R4 hardware count = R3 edges in "));
  Serial.print(ok);
  Serial.print(F("/"));
  Serial.print(runs);
  Serial.print(F(" start/stop runs (1000, 44600, 55733/s; Z1-Z3 and T0 stops); totals R4 "));
  Serial.print(r4Total);
  Serial.print(F(" R3 "));
  Serial.print(r3Total);
  Serial.print(F("; stop bursts "));
  Serial.print(burstsOk);
  Serial.print(F("/5, "));
  Serial.print(cutTotal);
  Serial.print(F(" cut pulses in "));
  Serial.print(cycleTotal);
  Serial.println(F(" stops"));
}

// ---- Slice 11 step 4: the 2 ms tick and its latch ---------------------------

// One tick run: the R4 runs `rate` for 60 ticks (Y), checking each tick's
// latched count against the previous tick's prediction, then stops. Here:
// the R4's whole-run count (N before/after) against the edges captured,
// and its per-tick changes against rate x 2 ms. The tick period itself is
// the R4's own claim in this step (no tick pin reaches the R3).
bool tickRun(char letter, uint32_t rate, long ticks, uint16_t timeoutMs) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  captureStart(false);
  char body[12];
  snprintf(body, sizeof(body), "%c%lu", letter, (unsigned long)rate);
  const bool answered = request(body, timeoutMs) && replyIs("Y ");
  char tickReply[64];
  strncpy(tickReply, reply, sizeof(tickReply));
  const long checked = field(1), wrong = field(2), skipped = field(3);
  const long dmin = field(4), dmax = field(5), span = field(6);
  delay(5);
  captureStop();
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  // Pulses per 2 ms tick: floor and ceiling of rate x 0.002.
  const long lo = (long)(rate * 2UL / 1000UL);
  const long hi = (long)((rate * 2UL + 999UL) / 1000UL);
  const bool pass = answered && checked >= ticks - 10 && wrong == 0 && skipped == 0 && span == ticks - 1 &&
                    dmin >= lo && dmax <= hi && r3 == r4;
  Serial.print(letter == 'V' ? F("  tick stress ") : F("  tick "));
  Serial.print(rate);
  Serial.print(F("/s: predictions "));
  Serial.print(checked - wrong);
  Serial.print(F("/"));
  Serial.print(checked);
  Serial.print(F(" exact, per tick "));
  Serial.print(dmin);
  Serial.print(F(".."));
  Serial.print(dmax);
  Serial.print(F(" (want "));
  Serial.print(lo);
  Serial.print(F(".."));
  Serial.print(hi);
  Serial.print(F("), skipped "));
  Serial.print(skipped);
  Serial.print(F(", span "));
  Serial.print(span);
  Serial.print(F(", run R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  if (r3 >= 2) {
    Serial.print(F(" interval "));
    Serial.print(capMin / 2.0, 1);
    Serial.print(F(".."));
    Serial.print(capMax / 2.0, 1);
    Serial.print(F("us"));
  }
  Serial.print(F("; R4: "));
  Serial.println(answered ? tickReply : "no reply");
  return pass;
}

void checkTick() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  const uint32_t rates[5] = {0, 1000, 44600, 55733, 12345};
  uint8_t ok = 0;
  for (uint8_t i = 0; i < 5; i++) {
    if (tickRun('Y', rates[i], 60, 1500)) ok++;
  }
  // 5 s stress runs at the rig's top rates: 2 500 predictions each.
  if (tickRun('V', 44600, 2500, 7000)) ok++;
  if (tickRun('V', 55733, 2500, 7000)) ok++;
  // 44 486/s (1 079 counts) walks the phase through every value within
  // 2.2 s, so every pulse-end/tick boundary is met in each run.
  if (tickRun('V', 44486, 2500, 7000)) ok++;
  if (tickRun('V', 44486, 2500, 7000)) ok++;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 9);
  Serial.print(F("tick: 2 ms latch, predictions exact at 0-55733/s in "));
  Serial.print(ok);
  Serial.println(F("/9 runs; R4 count = R3 edges"));
}

// ---- Slice 11 step 5a: rate changes applied at the tick ---------------------

// One queue run: the R4 queues a rate after every tick (Q<mode>) and checks
// every prediction itself. Here: its whole-run count against the edges
// captured, no interval under 7.5 us (low + minimum high), and for mode 1
// the tick grid from outside: each switch to 44 600/s puts a pulse 2.5 us
// after a tick, so the burst starts must be exactly 2 ticks (4 ms) apart.
bool queueRun(uint8_t mode, uint16_t wantTicks, uint16_t timeoutMs) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  capBursts = mode == 1;
  captureStart(false);
  char body[8];
  snprintf(body, sizeof(body), "Q%u", mode);
  const bool answered = request(body, timeoutMs) && replyIs("Q ");
  char queueReply[64];
  strncpy(queueReply, reply, sizeof(queueReply));
  const long ticks = field(1), applied = field(2), countMiss = field(3);
  const long phaseMiss = field(4), queueErr = field(5), span = field(6), oldFirst = field(7);
  delay(5);
  captureStop();
  capBursts = 0;
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  bool pass = answered && ticks == wantTicks && applied == ticks && countMiss == 0 &&
              phaseMiss == 0 && queueErr == 0 && span == ticks && r3 == r4 && capMin >= 15;
  // Burst spacing: 4 ms = 8000 ticks, +-(0.5 % + 2 ticks).
  uint32_t gapMin = 0xFFFFFFFF, gapMax = 0;
  for (uint8_t i = 1; i < burstN; i++) {
    const uint32_t gap = burstAt[i] - burstAt[i - 1];
    if (gap < gapMin) gapMin = gap;
    if (gap > gapMax) gapMax = gap;
  }
  if (mode == 1) pass = pass && burstN >= 18 && gapMin >= 7958 && gapMax <= 8042;
  // Mode 3 must also have met the race: old pulse before the DTC write.
  if (mode == 3) pass = pass && oldFirst >= 1;
  Serial.print(F("  queue Q"));
  Serial.print(mode);
  Serial.print(F(": ticks "));
  Serial.print(ticks);
  Serial.print(F(" applied "));
  Serial.print(applied);
  Serial.print(F(" count/phase misses "));
  Serial.print(countMiss);
  Serial.print(F("/"));
  Serial.print(phaseMiss);
  Serial.print(F(" old-pulse-first "));
  Serial.print(oldFirst);
  Serial.print(F(" queue errors "));
  Serial.print(queueErr);
  Serial.print(F(" min interval "));
  Serial.print(capMin / 2.0, 1);
  Serial.print(F("us run R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  if (mode == 1) {
    Serial.print(F(" bursts "));
    Serial.print(burstN);
    Serial.print(F(" spacing "));
    Serial.print(gapMin / 2.0, 1);
    Serial.print(F(".."));
    Serial.print(gapMax / 2.0, 1);
    Serial.print(F("us (want 4000)"));
  }
  Serial.print(F("; R4: "));
  Serial.println(answered ? queueReply : "no reply");
  return pass;
}

void checkQueue() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  uint8_t ok = 0;
  if (queueRun(1, 40, 3000)) ok++;
  if (queueRun(1, 40, 3000)) ok++;
  if (queueRun(2, 37, 3000)) ok++;
  if (queueRun(2, 37, 3000)) ok++;
  if (queueRun(3, 2500, 7000)) ok++;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 5);
  Serial.print(F("queue: applied at the tick, exact, in "));
  Serial.print(ok);
  Serial.println(F("/5 runs"));
}

// ---- Slice 11 step 5b: direction changes at the tick -------------------------

// One reversal run: the R4 queues signed rates (W<mode>) and checks every
// reversal against the DIR rule from its own read-back. Here: its count
// against the edges captured, and its position change against DIR as read
// on D7 at every STEP pulse (forward = DIR HIGH): R4 position change must
// equal pulses with DIR HIGH minus pulses with DIR LOW.
bool reverseRun(uint8_t mode, long wantTicks, uint16_t timeoutMs) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  capDir = 1;
  captureStart(false);
  char body[8];
  snprintf(body, sizeof(body), "W%u", mode);
  const bool answered = request(body, timeoutMs) && replyIs("W ");
  char reverseReply[64];
  strncpy(reverseReply, reply, sizeof(reverseReply));
  const long ticks = field(1), applied = field(2), countMiss = field(3), phaseMiss = field(4);
  const long queueErr = field(5), reversals = field(6), violations = field(9);
  const long inFlight = field(7), oldFirst = field(8);
  // The position change is the last field, "M <n>" when negative.
  long position = field(12);
  if (position < 0) position = -field(13);
  delay(5);
  captureStop();
  capDir = 0;
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  const long signedR3 = (long)dirHighCount - (long)dirLowCount;
  // A DIR read taken after its pulse ended may see the new direction: each
  // such misread moves the signed count by 2, so the R3's figure may differ
  // by at most twice the uncertain reads (exact when there were none).
  const long uncertain = dirUncertain;
  bool pass = answered && ticks == wantTicks && countMiss == 0 && phaseMiss == 0 &&
              queueErr == 0 && violations == 0 && r3 == r4 &&
              labs(signedR3 - position) <= 2 * uncertain && capMin >= 15;
  // Mode 1 must meet both hazards the delayed DIR write exists for: a pulse
  // LOW at the tick, and one fired inside the write latency. Mode 3 must
  // reverse while running at least once. (Mode 2 reverses through idle.)
  if (mode == 1) pass = pass && reversals == wantTicks && inFlight > 0 && oldFirst > 0;
  if (mode == 3) pass = pass && reversals > 0;
  Serial.print(F("  reverse W"));
  Serial.print(mode);
  Serial.print(F(": applied "));
  Serial.print(applied);
  Serial.print(F(" reversals "));
  Serial.print(reversals);
  Serial.print(F(" (in flight "));
  Serial.print(inFlight);
  Serial.print(F(", old first "));
  Serial.print(oldFirst);
  Serial.print(F(") run R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F(" position R4 "));
  Serial.print(position);
  Serial.print(F(" R3 "));
  Serial.print(signedR3);
  Serial.print(F(" (uncertain reads "));
  Serial.print(uncertain);
  Serial.print(F(") min interval "));
  Serial.print(capMin / 2.0, 1);
  Serial.print(F("us; R4: "));
  Serial.println(answered ? reverseReply : "no reply");
  return pass;
}

void checkReverse() {
  pinMode(STEP_IN, INPUT_PULLUP);
  pinMode(DIR_IN, INPUT);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  uint8_t ok = 0;
  if (reverseRun(1, 2000, 7000)) ok++;
  if (reverseRun(2, 80, 2000)) ok++;
  if (reverseRun(3, 610, 4000)) ok++;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 3);
  Serial.print(F("reverse: DIR rule held, position = DIR-signed pulses in "));
  Serial.print(ok);
  Serial.println(F("/3 runs"));
}

// ---- Slice 11 follow-up: ticks passed over (missed-tick recovery) -----------

// One skip run: the R4 stalls its poll loop on purpose (k<mode>), so queued
// changes land on ticks it never polls. Modes 1 and 2: every stall must be
// recovered (the next queue accepted), every later prediction exact, and
// the R4's count and position must match the edges and DIR seen here — in
// mode 2 every stall passes over a reversal. Mode 3: an idle generator must
// stop about 1 s after its last pulse even after a stall, with no pulse
// after idle beyond the one LOW at the idle tick.
bool skipRun(uint8_t mode, uint16_t timeoutMs) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  capDir = 1;
  captureStart(false);
  char body[8];
  snprintf(body, sizeof(body), "k%u", mode);
  const bool answered = request(body, timeoutMs) && replyIs("k ");
  char skipReply[64];
  strncpy(skipReply, reply, sizeof(skipReply));
  bool pass = false;
  long position = 0;
  if (mode == 3) {
    const long stopped = field(1), ticks = field(2), after = field(3);
    position = field(4);
    if (position < 0) position = -field(5);
    // 1 s = 500 ticks after the last pulse; the last pulse is at most one
    // 1 000/s period before the idle tick.
    pass = answered && stopped == 1 && ticks >= 499 && ticks <= 503 && after <= 1;
  } else {
    // A stall ending just after a tick must be recovered in the same poll;
    // one ending anywhere may wait one tick (a STEP overflow since the tick
    // hides the period that was in progress at it).
    const long stalls = field(1), recovered = field(2), worstLate = field(3), worstAligned = field(4);
    const long queueErr = field(5), phaseMiss = field(6), countMiss = field(7), unsynced = field(9);
    position = field(11);
    if (position < 0) position = -field(12);
    pass = answered && stalls == 20 && recovered == stalls && worstAligned == 0 && worstLate <= 1 &&
           queueErr == 0 && phaseMiss == 0 && countMiss == 0 && unsynced == 0;
  }
  delay(5);
  captureStop();
  capDir = 0;
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  const long signedR3 = (long)dirHighCount - (long)dirLowCount;
  const long uncertain = dirUncertain;
  pass = pass && r3 == r4 && labs(signedR3 - position) <= 2 * uncertain && capMin >= 15;
  Serial.print(F("  skip k"));
  Serial.print(mode);
  Serial.print(F(": run R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F(" position R4 "));
  Serial.print(position);
  Serial.print(F(" R3 "));
  Serial.print(signedR3);
  Serial.print(F(" (uncertain reads "));
  Serial.print(uncertain);
  Serial.print(F("); R4: "));
  Serial.println(answered ? skipReply : "no reply");
  return pass;
}

void checkSkip() {
  pinMode(STEP_IN, INPUT_PULLUP);
  pinMode(DIR_IN, INPUT);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  uint8_t ok = 0;
  if (skipRun(1, 3000)) ok++;
  if (skipRun(2, 3000)) ok++;
  if (skipRun(3, 4000)) ok++;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 3);
  Serial.print(F("skip: recovered, count and position = R3 in "));
  Serial.print(ok);
  Serial.println(F("/3 runs"));
}

// ---- Cart-pole dogfooding fix: DIR set-up before the first pulse -----------

const uint8_t DIR_BIT = _BV(PD7);  // DIR_IN, D7

// One DIR read in timeDirLeads' wait: in, andi, cp, brne (4 cycles, 0.25 us).
#define DIR_POLL if ((PIND & DIR_BIT) != dir) goto dirChanged;

// Times DIR changes on D7 against the next STEP falling edge on D8, with
// interrupts off and Timer1 at clk/1 (62.5 ns): up to `want` changes, or
// until `timeoutOvf` Timer1 overflows (4.096 ms each). The edge time is the
// hardware capture (ICR1). DIR is polled; in the compiled loop the reads
// are 4 cycles apart, 9 across a pass and 20 once per Timer1 overflow, and
// Timer1 is read 5-6 cycles after the read that saw the change. With the
// pin synchronizer, a time reads up to 25 cycles (1.56 us) short, never
// long. A STEP edge already flagged when the change is seen reads 0. The flag is cleared once per pass; the generator is stopped or
// idle before every change, so the last pulse is microseconds older. Returns
// the changes timed.
uint8_t timeDirLeads(uint16_t *gaps, uint8_t want, uint16_t timeoutOvf) {
  uint8_t n = 0;
  uint16_t ovf = 0;
  noInterrupts();
  TIMSK1 = 0;
  TCCR1A = 0;
  TCCR1B = _BV(CS10);  // clk/1; ICES1 = 0: falling edge; no noise canceler
  TIFR1 = _BV(ICF1) | _BV(TOV1);
  uint8_t dir = PIND & DIR_BIT;
  while (n < want) {
    for (;;) {
      TIFR1 = _BV(ICF1);  // an edge of the train before the change
      DIR_POLL DIR_POLL DIR_POLL DIR_POLL DIR_POLL DIR_POLL DIR_POLL DIR_POLL
      if (TIFR1 & _BV(TOV1)) {
        TIFR1 = _BV(TOV1);
        if (++ovf >= timeoutOvf) goto dirDone;
      }
    }
  dirChanged:
    const uint16_t t0 = TCNT1;
    const bool early = TIFR1 & _BV(ICF1);
    dir = PIND & DIR_BIT;
    while (!(TIFR1 & _BV(ICF1)) && ovf < timeoutOvf) {
      if (TIFR1 & _BV(TOV1)) {
        TIFR1 = _BV(TOV1);
        ovf++;
      }
    }
    if (!(TIFR1 & _BV(ICF1))) break;
    gaps[n++] = early ? 0 : (uint16_t)(ICR1 - t0);
    TIFR1 = _BV(ICF1);
  }
dirDone:
  interrupts();
  return n;
}

// One d<mode> run (the R4's dir_lead_run): ten starts alternating forward
// and back, beginning forward, so DIR changes 9 times, or 10 when it was
// LOW (backward) before. Each change's time to the next STEP pulse must be
// at least the 5 us DIR set-up, and at most 7 us, for starts from `run`
// (modes 1-2, and mode 4's first start); at least the 40 us queue guard
// plus the 2.5 us minimum HIGH for starts through `queue` (mode 3, and
// mode 4's direction changes through idle). The lower thresholds allow
// the 1.56 us the polled DIR can read short; the old `run`, which waited
// only the 2.5 us minimum HIGH, stays below the run threshold. The R4
// checks every prediction itself.
bool dirLeadRun(uint8_t mode) {
  const bool startLow = digitalRead(DIR_IN) == LOW;
  const uint8_t want = 9 + (startLow ? 1 : 0);
  char body[8];
  snprintf(body, sizeof(body), "d%u", mode);
  while (Serial.available()) Serial.read();
  Serial.write(STX);
  Serial.print(body);
  Serial.write(ETX);
  Serial.flush();
  uint16_t gaps[10];
  const uint8_t n = timeDirLeads(gaps, want, 400);
  const bool answered = awaitReply(3000, false) && replyIs("d ");
  // Timer1 counts, 16 per us.
  const uint16_t RUN_MIN = 56;     // 5 us set-up, less 1.5 us
  const uint16_t RUN_MAX = 112;    // 7 us: the wait is the set-up, not more
  const uint16_t QUEUE_MIN = 656;  // 40 us guard + 2.5 us min HIGH, less 1.5 us
  uint16_t runMin = 0xFFFF, runMax = 0, queueMin = 0xFFFF;
  uint8_t runs = 0, queues = 0;
  bool leadsOk = true;
  for (uint8_t i = 0; i < n; i++) {
    const bool fromRun = mode <= 2 || (mode == 4 && i == 0 && startLow);
    if (fromRun) {
      runs++;
      if (gaps[i] < runMin) runMin = gaps[i];
      if (gaps[i] > runMax) runMax = gaps[i];
      leadsOk = leadsOk && gaps[i] >= RUN_MIN && gaps[i] <= RUN_MAX;
    } else {
      queues++;
      if (gaps[i] < queueMin) queueMin = gaps[i];
      leadsOk = leadsOk && gaps[i] >= QUEUE_MIN;
    }
  }
  const long starts = field(1), errors = field(2), phaseMiss = field(3), countMiss = field(4);
  const long checked = field(5);
  const bool pass = answered && n == want && leadsOk && starts == 10 && errors == 0 &&
                    phaseMiss == 0 && countMiss == 0 && checked >= 20;
  Serial.print(F("  dirlead d"));
  Serial.print(mode);
  Serial.print(F(": DIR changes "));
  Serial.print(n);
  Serial.print(F("/"));
  Serial.print(want);
  if (runs) {
    Serial.print(F(", run lead "));
    Serial.print(runMin / 16.0, 2);
    Serial.print(F(".."));
    Serial.print(runMax / 16.0, 2);
    Serial.print(F("us"));
  }
  if (queues) {
    Serial.print(F(", queue lead >= "));
    Serial.print(queueMin / 16.0, 2);
    Serial.print(F("us"));
  }
  Serial.print(F("; R4: "));
  Serial.println(answered ? reply : "no reply");
  return pass;
}

void checkDirLead() {
  pinMode(STEP_IN, INPUT_PULLUP);
  pinMode(DIR_IN, INPUT);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  uint8_t ok = 0;
  for (uint8_t mode = 1; mode <= 4; mode++) {
    if (dirLeadRun(mode)) ok++;
  }
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 4);
  Serial.print(F("dirlead: DIR set-up waited, exact, in "));
  Serial.print(ok);
  Serial.println(F("/4 modes"));
}

// ---- Count stop: moves of an exact pulse count -----------------------------

// One n<mode> run (the R4's count_stop_run): every move must end at exactly
// its count, with STEP resting HIGH, and the R4's count change over the
// request must equal the STEP edges captured here.
bool countStopRun(uint8_t mode, long wantMoves, long wantRefusals) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  captureStart(false);
  char body[3] = {'n', (char)('0' + mode), 0};
  const bool answered = request(body, 6000) && replyIs("n ");
  char countReply[48];
  strncpy(countReply, reply, sizeof(countReply));
  countReply[sizeof(countReply) - 1] = 0;
  const long moves = field(1), exact = field(2), errors = field(3), refusals = field(4);
  const long wakes = field(5), pulses = field(6);
  delay(5);
  captureStop();
  const bool restHigh = digitalRead(STEP_IN) == HIGH;
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  const bool pass = answered && moves == wantMoves && exact == moves && errors == 0 &&
                    refusals == wantRefusals && pulses == r4 && r3 == r4 && restHigh &&
                    wakes >= 1;
  Serial.print(F("  count n"));
  Serial.print(mode);
  Serial.print(F(": R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F("; R4: "));
  Serial.println(answered ? countReply : "none");
  return pass;
}

void checkCountStop() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  uint8_t ok = 0;
  if (countStopRun(1, 5, 0)) ok++;
  if (countStopRun(2, 5, 0)) ok++;
  if (countStopRun(3, 4, 12)) ok++;
  if (countStopRun(4, 1, 0)) ok++;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 4);
  Serial.print(F("countstop: exact counts, R4 = R3, in "));
  Serial.print(ok);
  Serial.println(F("/4 modes"));
}

// ---- Count stop cancelled while its match fires ----------------------------

// The R4 arms a stop 4 pulses ahead at 55 733/s and cancels it at swept
// moments around the match, 2 000 times (c). Each cancel must leave either
// a stop exactly at the count, reported, or a train still running, not
// reported. Both outcomes must occur (the sweep crossed the match), and
// the R4's count must equal the edges captured here.
void checkCancelRace() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  bool pass = false;
  char raceReply[64] = "no reply";
  long r4 = -1, r3 = -1;
  if (request("N", 500) && replyIs("N ")) {
    const long before = field(1);
    captureStart(false);
    const bool answered = request("c", 8000) && replyIs("c ");
    if (answered) strncpy(raceReply, reply, sizeof(raceReply) - 1);
    const long attempts = field(1), stops = field(2), cancels = field(3);
    const long violations = field(4), errors = field(6), pulses = field(8);
    delay(5);
    captureStop();
    if (request("N", 500) && replyIs("N ")) {
      r4 = field(1) - before;
      r3 = capN;
      pass = answered && violations == 0 && errors == 0 && stops > 0 && cancels > 0 &&
             stops + cancels == attempts && pulses == r4 && r3 == r4;
    }
  }
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, pass);
  Serial.print(F("cancelrace: R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F("; R4: "));
  Serial.println(raceReply);
}

// ---- A long sleep at a steady rate, then a queue ---------------------------

// The R4 runs at a steady rate, makes no call for 100 ms, 500 ms or 3 s,
// then polls once and queues a new rate (s1-s3; s2 reverses). The first
// poll must report the ticks passed over, queue must work within 3 polls,
// every later prediction must be exact, and the count and DIR-signed
// position must match what D8 and D7 saw here.
bool longSleepRun(uint8_t mode, long minSkipped, uint16_t timeoutMs) {
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long before = field(1);
  capDir = 1;
  captureStart(false);
  char body[3] = {'s', (char)('0' + mode), 0};
  const bool answered = request(body, timeoutMs) && replyIs("s ");
  char sleepReply[64];
  strncpy(sleepReply, reply, sizeof(sleepReply));
  sleepReply[sizeof(sleepReply) - 1] = 0;
  const long skipped = field(1), polls = field(2), errors = field(3);
  const long phaseMiss = field(4), countMiss = field(5), checked = field(6), pulses = field(7);
  // The position change is the last field, "M <n>" when negative.
  long position = field(8);
  if (position < 0) position = -field(9);
  delay(5);
  captureStop();
  capDir = 0;
  if (!request("N", 500) || !replyIs("N ")) return false;
  const long r4 = field(1) - before;
  const long r3 = capN;
  const long signedR3 = (long)dirHighCount - (long)dirLowCount;
  const long uncertain = dirUncertain;
  const bool pass = answered && skipped >= minSkipped && polls <= 3 && errors == 0 &&
                    phaseMiss == 0 && countMiss == 0 && checked >= 15 && pulses == r4 &&
                    r3 == r4 && labs(signedR3 - position) <= 2 * uncertain;
  Serial.print(F("  sleep s"));
  Serial.print(mode);
  Serial.print(F(": R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F(" position R4 "));
  Serial.print(position);
  Serial.print(F(" R3 "));
  Serial.print(signedR3);
  Serial.print(F("; R4: "));
  Serial.println(answered ? sleepReply : "none");
  return pass;
}

// ---- Boot-time setup: another timer, a long DIR set-up, free ---------------

// While the R4 boots it builds a second step setup (STEP on its D3 = GPT321
// output A, DIR on D5 with a 200 us set-up), changes direction four times
// through queue, frees the generator while it runs, and builds the main
// one from what free gave back (b). Here: every STEP pulse start on D3
// (INT1, falling) and every DIR change on D7 (PCINT23), timestamped with
// micros(). Each DIR change must lead the next pulse by the set-up time
// (at least 190 us as read here) and by no more than 500 us: the R4 queues
// each change 60 us before a tick, so a long lead means the case was not
// exercised. The falls must equal the R4's count after free (no cut pulse,
// no pulse after it).
volatile uint16_t bootFalls = 0;
volatile uint8_t bootDirChanges = 0, bootDirPending = 0, bootDirLevel = 1, bootLeads = 0;
volatile uint32_t bootDirAt = 0, bootLeadMin = 0xFFFFFFFF, bootLeadMax = 0;

void bootStepIsr() {
  bootFalls++;
  if (bootDirPending) {
    const uint32_t lead = micros() - bootDirAt;
    if (lead < bootLeadMin) bootLeadMin = lead;
    if (lead > bootLeadMax) bootLeadMax = lead;
    bootLeads++;
    bootDirPending = 0;
  }
}

ISR(PCINT2_vect) {
  const uint8_t level = (PIND & DIR_BIT) ? 1 : 0;
  if (level == bootDirLevel) return;
  bootDirLevel = level;
  bootDirAt = micros();
  bootDirPending = 1;
  bootDirChanges++;
}

void checkBootFree() {
  // Pull-ups hold both lines HIGH while the R4 resets and its pins float;
  // HIGH is the STEP idle level and DIR's forward level.
  pinMode(SHARED, INPUT_PULLUP);
  pinMode(DIR_IN, INPUT_PULLUP);
  attachInterrupt(digitalPinToInterrupt(SHARED), bootStepIsr, FALLING);
  // An edge latched in EIFR while INT1 was off fires once on attach.
  delay(2);
  noInterrupts();
  bootFalls = 0;
  bootDirChanges = 0;
  bootDirPending = 0;
  bootDirLevel = (PIND & DIR_BIT) ? 1 : 0;
  bootLeads = 0;
  bootLeadMin = 0xFFFFFFFF;
  bootLeadMax = 0;
  PCIFR = _BV(PCIF2);
  PCMSK2 |= _BV(PCINT23);
  PCICR |= _BV(PCIE2);
  interrupts();
  pulse();  // the R4 restarts and runs its boot-time checks
  delay(3000);
  bool back = false;
  for (uint8_t i = 0; i < 10 && !back; i++) back = request("I", 500) && replyIs("ID ");
  noInterrupts();
  PCICR &= ~_BV(PCIE2);
  PCMSK2 &= ~_BV(PCINT23);
  interrupts();
  detachInterrupt(digitalPinToInterrupt(SHARED));
  const bool high = digitalRead(SHARED) == HIGH;
  pinMode(DIR_IN, INPUT);
  const bool answered = back && request("b", 500) && replyIs("b ");
  const long ran = field(1), starts = field(2), errors = field(3), phaseMiss = field(4);
  const long countMiss = field(5), checked = field(6), freed = field(7), pinsInput = field(8);
  const long stopped = field(9), count = field(10);
  // ENA: enable() to is_enabled() in R4 PCLKD counts, against the lead.
  const long enaCounts = field(11), leadCounts = field(12);
  const long falls = bootFalls;
  const bool pass = answered && ran == 1 && starts == 4 && errors == 0 && phaseMiss == 0 &&
                    countMiss == 0 && checked >= 10 && freed == 1 && pinsInput == 1 &&
                    stopped == 1 && bootDirChanges == 4 && bootLeads == 4 &&
                    bootLeadMin >= 190 && bootLeadMax <= 500 && falls == count && high &&
                    leadCounts > 0 && enaCounts >= leadCounts && enaCounts <= leadCounts + 200;
  verdict(11, pass);
  Serial.print(F("boot: STEP D3 falls R3 "));
  Serial.print(falls);
  Serial.print(F(" R4 "));
  Serial.print(count);
  Serial.print(F(", DIR changes "));
  Serial.print(bootDirChanges);
  Serial.print(F(" lead "));
  Serial.print(bootLeadMin);
  Serial.print(F(".."));
  Serial.print(bootLeadMax);
  Serial.print(F(" us; R4: "));
  Serial.println(answered ? reply : "none");
}

// ---- Slice 12: the wake timer ----------------------------------------------

// The R4 times its wake timer against a free-running GPT itself (t1:
// every wait at least its length, within one prescaler step plus 10 us,
// one WFI wake each; t2: once, restart, cancel, zero, too long). Then it
// marks a 1 ms and a 100 ms wait on D3, LOW from start to wake (t3), and
// this sketch times both LOW widths on INT1 with micros().
volatile uint32_t wakeFallAt = 0;
volatile uint8_t wakeFallKnown = 0, wakeWidthN = 0;
volatile uint32_t wakeWidths[2];

void wakeEdgeIsr() {
  const uint32_t now = micros();
  if (PIND & _BV(PIND3)) {
    if (wakeFallKnown && wakeWidthN < 2) wakeWidths[wakeWidthN++] = now - wakeFallAt;
    wakeFallKnown = 0;
  } else {
    wakeFallAt = now;
    wakeFallKnown = 1;
  }
}

void checkWakeTimer() {
  char lengths[48] = "none", behaviour[32] = "none";
  const bool lengthsOk = request("t1", 5000) && replyIs("t1 ") && field(1) == 9 &&
                         field(2) == 0 && field(3) == 0 && field(6) == 1;
  if (replyIs("t1 ")) strncpy(lengths, reply, sizeof(lengths) - 1);
  const bool behaviourOk = request("t2", 5000) && replyIs("t2 ") && field(1) == 1 &&
                           field(2) == 1 && field(3) == 1 && field(4) == 1 &&
                           field(5) == 1 && field(6) == 1;
  if (replyIs("t2 ")) strncpy(behaviour, reply, sizeof(behaviour) - 1);
  pinMode(SHARED, INPUT_PULLUP);
  attachInterrupt(digitalPinToInterrupt(SHARED), wakeEdgeIsr, CHANGE);
  // An edge latched in EIFR while INT1 was off fires once on attach.
  delay(2);
  noInterrupts();
  wakeWidthN = 0;
  wakeFallKnown = 0;
  interrupts();
  const bool marked = request("t3", 3000) && replyIs("t3 ");
  detachInterrupt(digitalPinToInterrupt(SHARED));
  pinMode(SHARED, INPUT_PULLUP);
  const long w1 = wakeWidthN > 0 ? (long)wakeWidths[0] : -1;
  const long w2 = wakeWidthN > 1 ? (long)wakeWidths[1] : -1;
  // micros() steps 4 us at each end, so allow 8 us short. Long: this
  // board's clock reads the R4's time up to 0.1 % long (S1: 20 ms phases
  // read 20 012-20 020 us), and the R4 sets D3 LOW before start() and HIGH
  // after the wake, so allow 0.3 % plus 30 us.
  const bool wireOk = marked && wakeWidthN == 2 && w1 >= 992 && w1 <= 1033 &&
                      w2 >= 99992 && w2 <= 100330;
  verdict(12, lengthsOk && behaviourOk && wireOk);
  Serial.print(F("wake: D3 LOW "));
  Serial.print(w1);
  Serial.print(F(" us / "));
  Serial.print(w2);
  Serial.print(F(" us; R4: "));
  Serial.print(lengths);
  Serial.print(F("; "));
  Serial.println(behaviour);
}

void checkLongSleep() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  uint8_t ok = 0;
  if (longSleepRun(1, 45, 3000)) ok++;
  if (longSleepRun(2, 245, 4000)) ok++;
  if (longSleepRun(3, 1495, 8000)) ok++;
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, ok == 3);
  Serial.print(F("sleep: recovered, queue worked, R4 = R3, in "));
  Serial.print(ok);
  Serial.println(F("/3 modes"));
}

// ---- Slice 11 follow-up: the tick wakes the CPU from WFI -------------------

// The R4 runs 2 500 ticks of random fast rates with its CPU asleep in WFI,
// woken by the tick's interrupt (J), and checks every tick itself. Here:
// its count against the edges captured, no interval under 15 counts, and
// its timing: every tick seen, its queue done long before the 40 us guard.
void checkWake() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  bool pass = false;
  char wakeReply[64] = "no reply";
  long r4 = -1, r3 = -1;
  if (request("N", 500) && replyIs("N ")) {
    const long before = field(1);
    captureStart(false);
    const bool answered = request("J", 8000) && replyIs("J ");
    if (answered) strncpy(wakeReply, reply, sizeof(wakeReply));
    const long ticks = field(1), applied = field(2), countMiss = field(3), phaseMiss = field(4);
    const long queueErr = field(5), skipped = field(6), doneMax = field(9);
    delay(5);
    captureStop();
    if (request("N", 500) && replyIs("N ")) {
      r4 = field(1) - before;
      r3 = capN;
      // The queue guard starts 1 920 counts before the next tick (96 000).
      pass = answered && ticks == 2500 && applied == ticks && countMiss == 0 && phaseMiss == 0 &&
             queueErr == 0 && skipped == 0 && doneMax < 96000L - 1920L && r3 == r4 && capMin >= 15;
    }
  }
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  verdict(11, pass);
  Serial.print(F("wake: 2500 ticks woke WFI, exact; run R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(r3);
  Serial.print(F("; R4: "));
  Serial.println(wakeReply);
}

// ---- Slice 11 step 5c: rate 0, held ticks, idle stop, start, trip -----------

// The R4 plays a fixed scenario (O): ramp, rate 0, ticks without a queued
// rate (held), a restart from idle, 600 idle ticks (it stops itself after
// 1 s), a hardware start from stopped, and a trip stop part-way into a tick.
// It checks every prediction itself. Here: its count against the edges
// captured, every pulse's LOW (no cut pulse, the trip's last one included),
// no interval under 7.5 us, and the line HIGH and silent after the trip.
void checkScenario() {
  pinMode(STEP_IN, INPUT_PULLUP);
  const uint8_t savedA = TCCR1A, savedB = TCCR1B, savedMask = TIMSK1;
  bool pass = false;
  long r4 = -1;
  char scenarioReply[64] = "no reply";
  if (request("N", 500) && replyIs("N ")) {
    const long before = field(1);
    captureStart(true);
    const bool answered = request("O", 4000) && replyIs("O ");
    strncpy(scenarioReply, reply, sizeof(scenarioReply));
    const long ticks = field(1), applied = field(2), countMiss = field(3);
    const long phaseMiss = field(4), queueErr = field(5), idleStop = field(6), started = field(7);
    const uint32_t atStop = capturedCount();
    delay(5);
    captureStop();
    const bool quiet = capN == atStop;
    const bool restHigh = digitalRead(STEP_IN) == HIGH;
    if (request("N", 500) && replyIs("N ")) r4 = field(1) - before;
    pass = answered && ticks == 618 && applied == 13 && countMiss == 0 && phaseMiss == 0 &&
           queueErr == 0 && idleStop == 1 && started == 1 && r4 == (long)capN && capMin >= 15 &&
           lowMin >= 9 && lowMax <= 14 && lowLate <= capN / 10 + 2 && quiet && restHigh &&
           (lastLate || (lastLow >= 9 && lastLow <= 14));
  }
  noInterrupts();
  TIMSK1 = savedMask;
  TCCR1A = savedA;
  TCCR1B = savedB;
  interrupts();
  Serial.print(F("  scenario: run R4 "));
  Serial.print(r4);
  Serial.print(F(" R3 "));
  Serial.print(capN);
  Serial.print(F(" min interval "));
  Serial.print(capMin / 2.0, 1);
  Serial.print(F("us low "));
  Serial.print(lowMin / 2.0, 1);
  Serial.print(F(".."));
  Serial.print(lowMax / 2.0, 1);
  Serial.print(F("us late "));
  Serial.print(lowLate);
  Serial.print(F(" last "));
  Serial.print(lastLate ? -1.0 : lastLow / 2.0, 1);
  Serial.print(F("; R4: "));
  Serial.println(scenarioReply);
  verdict(11, pass);
  Serial.println(F("scenario: idle, hold, restart, idle stop, start, trip stop"));
}

// ---- Slice 11 step 1: the ELC, R4-internal ----------------------------------

// The R4 routes ELC software event 0 to GPT166's count-up input and fires
// n events; GPT321 must count exactly n. Then it removes the route and
// fires 10 more, which must not count (the control: the count came
// through the ELC). Nothing leaves the R4, so this check reads its reply.
void checkElc() {
  const uint16_t counts[3] = {1, 100, 10000};
  bool all = true;
  Serial.print(F("S11 "));
  char body[8];
  long got[3];
  for (uint8_t i = 0; i < 3; i++) {
    snprintf(body, sizeof(body), "L%u", counts[i]);
    if (!request(body, 2000) || !replyIs("L ")) {
      got[i] = -1;
      all = false;
      continue;
    }
    got[i] = field(2);
    all = all && field(1) == (long)counts[i] && got[i] == (long)counts[i] && field(3) == got[i];
  }
  Serial.print(all ? F("PASS ") : F("FAIL "));
  if (all) passes++;
  else failures++;
  Serial.print(F("elc: software events counted by GPT166"));
  for (uint8_t i = 0; i < 3; i++) {
    Serial.print(F(" "));
    Serial.print(got[i]);
    Serial.print(F("/"));
    Serial.print(counts[i]);
  }
  Serial.println(all ? F("; unrouted events not counted") : F(""));
}

#endif  // R3_PART >= 2

#if R3_PART == 1

char watchCommand = 0;  // the last x or ! the PC sent during the watch

// One K poll. Unlike request(), PC command bytes that arrive outside a
// frame are kept in watchCommand instead of discarded.
bool pollBootLog(uint32_t timeoutMs) {
  Serial.write(STX);
  Serial.print("K");
  Serial.write(ETX);
  const uint32_t start = millis();
  bool inFrame = false;
  replyLen = 0;
  while (millis() - start < timeoutMs) {
    if (!Serial.available()) continue;
    const int c = Serial.read();
    if (c == STX) {
      inFrame = true;
      replyLen = 0;
    } else if (c == ETX && inFrame) {
      reply[replyLen] = 0;
      return true;
    } else if (inFrame) {
      if (replyLen < sizeof(reply) - 1) reply[replyLen++] = c;
      else inFrame = false;
    } else if (c == 'x' || c == '!') {
      watchCommand = c;
    }
  }
  reply[0] = 0;
  return false;
}

// The boot-counter watch. The first reply is the baseline (the R4 was
// already running). After that every change of the counter is one boot
// event: good when the log reads back intact and the counter grew (or
// restarted at 1 after a full log). A run of unanswered polls is a GAP
// (R4 unpowered or restarting).
void watchBoots() {
  long base = -1, last = -1;
  bool seen = false, gap = false;
  uint16_t boots = 0, bad = 0;
  watchCommand = 0;
  Serial.println(F("S10 WATCH polling K; x ends, ! resets the R4"));
  for (;;) {
    while (Serial.available()) {
      const int c = Serial.read();
      if (c == 'x' || c == '!') watchCommand = c;
    }
    if (watchCommand == 'x') break;
    if (watchCommand == '!') {
      watchCommand = 0;
      pulse();
      Serial.println(F("RESET 1"));
    }
    if (pollBootLog(150) && replyIs("K ")) {
      const long count = field(1);
      const long intact = field(2);
      if (count < 0) {
        bad++;
        Serial.print(F("S10 ERR "));
        Serial.println(reply);
      } else if (!seen) {
        seen = true;
        base = count;
        if (intact != 1) bad++;
        Serial.print(F("S10 BASE "));
        Serial.print(count);
        Serial.print(F(" "));
        Serial.println(intact);
      } else if (count != last) {
        const bool good = intact == 1 && (count > last || count == 1);
        boots++;
        if (!good) bad++;
        Serial.print(F("S10 BOOT "));
        Serial.print(count);
        Serial.print(F(" "));
        Serial.print(intact);
        Serial.print(F(" "));
        Serial.print(count - last);
        Serial.print(F(" "));
        Serial.println(gap ? 1 : 0);
      }
      if (count >= 0) last = count;
      gap = false;
    } else if (seen && !gap) {
      gap = true;
      Serial.println(F("S10 GAP"));
    }
    delay(100);
  }
  verdict(10, seen && boots > 0 && bad == 0);
  Serial.print(F("watch: "));
  Serial.print(boots);
  Serial.print(F(" boot events, "));
  Serial.print(bad);
  Serial.print(F(" bad, counter "));
  Serial.print(base);
  Serial.print(F(" -> "));
  Serial.println(last);
}

// ---- Slice 9 and status ----------------------------------------------------

void takeUsb() {
  if (!request("U", 1000)) { noReply(9); return; }
  Serial.print(F("S9 "));
  Serial.println(replyIs("U 1")
                     ? F("TAKEN native CDC 2341:006D; host checks it")
                     : F("ALREADY TAKEN (reset the R4 to take again)"));
}

#endif  // R3_PART == 1

void status() {
  if (!request("S", 500)) {
    Serial.println(F("STATUS no reply from R4"));
    return;
  }
  Serial.print(F("STATUS usb="));
  Serial.print(field(1));
  Serial.print(F(" taken="));
  Serial.print(field(2));
  Serial.print(F(" up="));
  Serial.print(field(3));
  Serial.print(F("s ov="));
  Serial.print(field(4));
  Serial.print(F(" er="));
  Serial.println(field(5));
}

void identify() {
  Serial.print(F("ID "));
  Serial.println(request("I", 500) ? reply : "no reply from R4");
}

void runAll() {
  passes = failures = 0;
#if R3_PART == 1
  checkLink();
  checkClock();
  checkInput();
  checkButton();
  checkSwitchWake();
  checkExternalPullUp();
  checkMonitor();
  checkBurst();
  checkTraits();
  checkAt();
  checkPwm();
  checkAdc();
  checkFlash();
#elif R3_PART == 2
  checkStep();
  checkCount();
  checkTick();
  checkQueue();
  checkScenario();
  checkReverse();
  checkWake();
  checkSkip();
  checkElc();
#else
  checkDirLead();
  checkCountStop();
  checkCancelRace();
  checkLongSleep();
  checkWakeTimer();
  checkBootFree();
#endif
  Serial.print(F("DONE "));
  Serial.print(passes);
  Serial.print(F(" PASS "));
  Serial.print(failures);
  Serial.println(F(" FAIL"));
}

void help() {
  // The commands are listed in this sketch's header comment. The validate
  // CLI reads the part from this line.
  Serial.print(F("r3-validator part "));
  Serial.print(R3_PART);
  Serial.println(F(" of 3: see header"));
}

void setup() {
  digitalWrite(RESET_PIN, LOW);
  pinMode(RESET_PIN, INPUT);
  pinMode(SHARED, INPUT_PULLUP);
  pinMode(PWM_IN, INPUT);
  pinMode(STEP_IN, INPUT_PULLUP);
  pinMode(ANALOG_OUT, OUTPUT);
  digitalWrite(ANALOG_OUT, LOW);
  Serial.begin(9600);
  help();
}

void loop() {
  static bool inFrame = false;
  if (!Serial.available()) return;
  const int c = Serial.read();
  // A stray R4 frame (a late reply) is skipped whole, never obeyed.
  if (c == STX) { inFrame = true; return; }
  if (inFrame) { if (c == ETX) inFrame = false; return; }
  switch (c) {
    case '?': help(); break;
    case 'i': identify(); break;
    case 'a': runAll(); break;
    case 's': status(); break;
    case '!': pulse(); Serial.println(F("RESET 1")); break;
    case '@': pulse(); delay(200); pulse(); Serial.println(F("RESET 2")); break;
#if R3_PART == 1
    case 'l': checkLink(); break;
    case 'c': checkClock(); break;
    case 'g': checkInput(); break;
    case 'b': checkButton(); break;
    case 'm': checkMonitor(); break;
    case 'e': checkBurst(); break;
    case 't': checkTraits(); break;
    case 'w': checkAt(); break;
    case 'p': checkPwm(); break;
    case 'n': checkAdc(); break;
    case 'u': takeUsb(); break;
    case 'f': checkFlash(); break;
    case 'k': watchBoots(); break;
    case 'W': checkSwitchWake(); break;
    case 'E': checkExternalPullUp(); break;
#elif R3_PART == 2
    case 'd': checkStep(); break;
    case 'v': checkElc(); break;
    case 'q': checkCount(); break;
    case 'j': checkTick(); break;
    case 'h': checkQueue(); break;
    case 'o': checkScenario(); break;
    case 'r': checkReverse(); break;
    case 'z': checkWake(); break;
    case 'K': checkSkip(); break;
#else
    case 'D':
      Serial.print(F("DELAY "));
      Serial.println(request("m", 15000) ? reply : "no reply from R4");
      break;
    case 'L': checkDirLead(); break;
    case 'M': checkCountStop(); break;
    case 'C': checkCancelRace(); break;
    case 'S': checkLongSleep(); break;
    case 'B': checkBootFree(); break;
    case 'T': checkWakeTimer(); break;
#endif
    default: break;  // anything else (incl. the R4's unframed burst) is ignored
  }
}
