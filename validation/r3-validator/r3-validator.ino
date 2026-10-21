// r3-validator — R3 side of the all-in-one unor4-hal validation pair.
// Peer: crates/test-firmware/src/bin/r4_validator.rs (one R4 image for Slices 1-9).
// Host: validation/validate.ps1 drives this sketch, and also checks the
// two slices only a PC can see (4: the bridge monitor, 9: native USB).
//
// Wiring (the bench in AGENTS.md, unchanged):
//   R3 D0/D1 <-> R4 D1/D0  request link, 9600 8N1 (shared with this USB port)
//   R3 D2    <-  R4 D6     Slice 7 PWM, measured on INT0
//   R3 D3    <-> R4 D3     Slice 2 input + button (R3 drives) / Slice 1 timing (R4 drives)
//   R3 D4    ->  R4 RESET  LOW pulses only, released to input, never HIGH
//   R3 D5    ->  R4 A0     Slice 8 stimulus (Timer0 PWM, ~976 Hz)
//   R3 GND   <-> R4 GND
// R4 D2 is free/open: the Slice 2 button is this sketch pressing D3.
// Keep the R3 powered whenever D4 is connected to the R4's RESET.
// D3 is INPUT_PULLUP on both boards except while one side drives it for a
// test it asked for, so the boards never drive it against each other.
//
// PC commands (9600 8N1, no line ending; lowercase so no R4 reply can
// ever look like one):
//   ?  help                     i  identify the R4
//   a  run every automatic check (S3, S1, S2 input, S2 button, S4,
//      S5 burst, S5 traits, S6-S8)
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

const uint8_t PWM_IN = 2;      // INT0, from R4 D6
const uint8_t SHARED = 3;      // INT1, R4 D3
const uint8_t RESET_PIN = 4;   // to R4 RESET
const uint8_t ANALOG_OUT = 5;  // to R4 A0
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
void checkButton() {
  pinMode(SHARED, INPUT_PULLUP);
  while (Serial.available()) Serial.read();
  Serial.write(STX);
  Serial.print("B");
  Serial.write(ETX);
  Serial.flush();  // the request, and any queued report text, has left
  delay(100);      // the R4 is now watching D3
  digitalWrite(SHARED, LOW);
  pinMode(SHARED, OUTPUT);
  delay(200);
  pinMode(SHARED, INPUT_PULLUP);
  if (!awaitReply(3000, false)) { noReply(2); return; }
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

// ---- Slice 9 and status ----------------------------------------------------

void takeUsb() {
  if (!request("U", 1000)) { noReply(9); return; }
  Serial.print(F("S9 "));
  Serial.println(replyIs("U 1")
                     ? F("TAKEN native CDC 2341:006D now; host checks echo/ticks; 1200-baud open returns the bridge")
                     : F("ALREADY TAKEN (reset the R4 to take again)"));
}

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
  checkLink();
  checkClock();
  checkInput();
  checkButton();
  checkMonitor();
  checkBurst();
  checkTraits();
  checkAt();
  checkPwm();
  checkAdc();
  Serial.print(F("DONE "));
  Serial.print(passes);
  Serial.print(F(" PASS "));
  Serial.print(failures);
  Serial.println(F(" FAIL"));
}

void help() {
  Serial.println(F("r3-validator: a all, l S3, c S1, g S2, b button, m S4, e/t S5,"));
  Serial.println(F("  w S6, p S7, n S8, u S9, s status, i id, ! reset, @ bootloader"));
}

void setup() {
  digitalWrite(RESET_PIN, LOW);
  pinMode(RESET_PIN, INPUT);
  pinMode(SHARED, INPUT_PULLUP);
  pinMode(PWM_IN, INPUT);
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
    case 's': status(); break;
    case '!': pulse(); Serial.println(F("RESET 1")); break;
    case '@': pulse(); delay(200); pulse(); Serial.println(F("RESET 2")); break;
    default: break;  // anything else (incl. the R4's unframed burst) is ignored
  }
}
