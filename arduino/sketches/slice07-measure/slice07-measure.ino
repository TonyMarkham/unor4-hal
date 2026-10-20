// slice07-measure — R3-side rig for unor4-hal Slice 7 (PWM).
//
// Wiring (additive bench lines, see Research/slice-07-research.md §8):
//   R4 D6 (GPT PWM out, 1 kHz, 3.3 V logic) -> R3 D2 (INT0)
//   R3 D3 -> R4 D3 (the bench gate line: low = start a staircase round)
//
// The R4's firmware (slice07_breathe) runs, per gate falling edge:
//   1 s settle, then a duty staircase (0/25/50/75/100%, 2 s each),
//   then sine breathing (2.048 s cycle) until the next gate edge.
//
// This sketch drives one gate round, then measures three consecutive
// 10 s windows edge-by-edge on INT0:
//   - window 1 aligns with the staircase,
//   - windows 2-3 catch the sine breathing.
// Each window reports:
//   - twenty 500 ms duty samples (per-mille of the last completed
//     period) — the staircase and the sine shape show up directly,
//   - aggregate duty (per-mille) with per-period min/max,
//   - mean period in microseconds and the edge count,
//   - "flat HIGH/LOW" when a window has no edges at all (the 0%/100%
//     mechanisms produce DC lines, which is itself the evidence).
//
// 3.3 V into an ATmega328P input is in spec: VIH = 0.6 x VCC = 3.0 V
// at 5 V VCC.

const uint8_t PWM_PIN = 2;   // INT0, from R4 D6
const uint8_t GATE_PIN = 3;  // to R4 D3 (bench gate line)
const unsigned long GATE_HOLD_MS = 1000;
const unsigned long WINDOW_MS = 10000;
const unsigned long SAMPLE_MS = 500;
const uint8_t WINDOWS_PER_ROUND = 3;

// ISR state (volatile; 32-bit reads in loop() are wrapped in
// noInterrupts/interrupts — the AVR reads these byte-wise).
volatile uint32_t g_rise = 0, g_fall = 0;
volatile uint8_t g_armed = 0, g_fell = 0;
volatile uint32_t g_sumHigh = 0, g_sumPeriod = 0, g_periods = 0;
volatile uint32_t g_dutyMin = 0xFFFFFFFF, g_dutyMax = 0, g_lastDuty = 0;

void pwmIsr() {
  uint32_t now = micros();
  if (PIND & _BV(PIND2)) {  // rising edge: one full period just ended
    if (g_armed) {
      uint32_t period = now - g_rise;
      // No fall since the rise = the pin was high the whole period
      // (the 100% duty mechanism never matches, so never falls).
      uint32_t high = g_fell ? (g_fall - g_rise) : period;
      g_sumPeriod += period;
      g_sumHigh += high;
      g_periods++;
      // Per-mille; 1 kHz periods are ~1000 us so high*1000 fits u32.
      uint32_t dutyPm = period ? (high * 1000UL) / period : 0;
      if (dutyPm < g_dutyMin) {
        g_dutyMin = dutyPm;
      }
      if (dutyPm > g_dutyMax) {
        g_dutyMax = dutyPm;
      }
      g_lastDuty = dutyPm;
    }
    g_rise = now;
    g_armed = 1;
    g_fell = 0;
  } else {  // falling edge
    g_fall = now;
    g_fell = 1;
  }
}

void resetWindow() {
  noInterrupts();
  g_armed = 0;
  g_fell = 0;
  g_sumHigh = 0;
  g_sumPeriod = 0;
  g_periods = 0;
  g_dutyMin = 0xFFFFFFFF;
  g_dutyMax = 0;
  g_lastDuty = 0;
  interrupts();
}

void reportWindow(uint32_t index) {
  noInterrupts();
  uint32_t sumHigh = g_sumHigh, sumPeriod = g_sumPeriod, periods = g_periods;
  uint32_t dutyMin = g_dutyMin, dutyMax = g_dutyMax;
  interrupts();

  Serial.print("W");
  Serial.print(index);
  if (periods == 0) {
    // No edges at all: DC line. Which level?
    Serial.print(" flat ");
    Serial.println(digitalRead(PWM_PIN) ? "HIGH (100%)" : "LOW (0%)");
    return;
  }
  uint32_t dutyMean = (uint32_t)(((uint64_t)sumHigh * 1000ULL) / sumPeriod);
  uint32_t periodMean = sumPeriod / periods;
  Serial.print(" duty=");
  Serial.print(dutyMean);
  Serial.print("/1000 (min ");
  Serial.print(dutyMin);
  Serial.print(" max ");
  Serial.print(dutyMax);
  Serial.print(") period=");
  Serial.print(periodMean);
  Serial.print("us edges=");
  Serial.println(periods);
}

void runWindow(uint32_t index) {
  Serial.print("W");
  Serial.print(index);
  Serial.println(" duty samples (pm, 500 ms apart; H/L = flat high/low, no edges):");
  uint32_t prevPeriods = 0;
  bool firstSample = true;
  for (uint8_t s = 0; s < (uint8_t)(WINDOW_MS / SAMPLE_MS); s++) {
    delay(SAMPLE_MS);
    noInterrupts();
    uint32_t d = g_lastDuty;
    uint32_t periods = g_periods;
    interrupts();
    if (!firstSample && periods == prevPeriods) {
      // No edge completed in this sample interval: the line is DC.
      // (The 0%/100% duty mechanisms output flat levels, so "no new
      // periods" is the measurement, not a stale value.)
      Serial.print(digitalRead(PWM_PIN) ? "H" : "L");
    } else {
      Serial.print(d);
    }
    Serial.print(s == (WINDOW_MS / SAMPLE_MS - 1) ? '\n' : ' ');
    prevPeriods = periods;
    firstSample = false;
  }
  reportWindow(index);
}

void setup() {
  Serial.begin(115200);
  pinMode(PWM_PIN, INPUT);
  pinMode(GATE_PIN, OUTPUT);
  digitalWrite(GATE_PIN, HIGH);  // idle high; low = round trigger
  attachInterrupt(digitalPinToInterrupt(PWM_PIN), pwmIsr, CHANGE);
  Serial.println("slice07-measure: measuring R4 D6 PWM on D2/INT0");
}

void loop() {
  // Trigger one staircase round on the R4 and measure it.
  digitalWrite(GATE_PIN, LOW);
  delay(GATE_HOLD_MS);
  digitalWrite(GATE_PIN, HIGH);

  static uint32_t windowIndex = 0;
  for (uint8_t w = 0; w < WINDOWS_PER_ROUND; w++) {
    resetWindow();
    runWindow(windowIndex++);
  }
}
