// slice08-source — Slice 8 stimulus: PWM duty staircase into R4 A0.
//
// Bench wiring (see the repo's AGENTS.md): R3 D5 -> R4 A0, and the
// existing gate line R3 D3 -> R4 D3. The R4 runs `slice08_adc` and
// averages its ADC readings over 1.5 s windows aligned to this sketch's
// duty switches.
//
// Choreography per round (repeats forever):
//   1. duty 0, gate HIGH for 1 s        — R4 is in its live-print phase
//   2. gate LOW for 1 s                 — R4 prints "gate open" (falling edge)
//   3. gate HIGH, duty staircase starts — rising edge starts R4's windows
//      0, 64, 127, 191, 255 (~0/25/50/75/100 %), 2 s per step
//   4. duty 0, 2 s pause, then the next round
//
// The commanded duty is printed on this board's USB serial so the
// transcript pairs with the R4's `W<n> mean=...` lines: the mean of a
// PWM wave over whole periods is duty/255 x VREF, so
//   duty 64  -> ~1255 mV,  127 -> ~2490 mV,  191 -> ~3745 mV,  255 -> ~5000 mV
// (at a 5000 mV reference — OQ-A1 checks that assumption).
//
// D5/D6 are ATmega328p Timer0 PWM (~976 Hz phase-correct in the Arduino
// core — the R4's min/max spread inside each window shows the ripple,
// which is how the period claim gets checked from the other side).

const int GATE_PIN = 3;
const int OUT_PIN = 5;

// 8-bit duty staircase: 0, 25, 50, 75, 100 %.
const uint8_t DUTIES[5] = {0, 64, 127, 191, 255};

void setup() {
  pinMode(GATE_PIN, OUTPUT);
  digitalWrite(GATE_PIN, HIGH); // idle: gate released
  pinMode(OUT_PIN, OUTPUT);
  digitalWrite(OUT_PIN, LOW);   // idle: stimulus at 0 V, no float on A0
  Serial.begin(115200);
}

void loop() {
  Serial.println("round: duty 0, gate high");
  digitalWrite(OUT_PIN, LOW);
  delay(1000);

  digitalWrite(GATE_PIN, LOW); // falling edge arms the R4
  Serial.println("gate low");
  delay(1000);

  digitalWrite(GATE_PIN, HIGH); // rising edge: R4 starts its windows
  Serial.println("gate high, staircase");
  for (int i = 0; i < 5; i++) {
    analogWrite(OUT_PIN, DUTIES[i]);
    Serial.print("duty ");
    Serial.println(DUTIES[i]);
    delay(2000);
  }

  Serial.println("round done");
  digitalWrite(OUT_PIN, LOW);
  delay(2000);
}
