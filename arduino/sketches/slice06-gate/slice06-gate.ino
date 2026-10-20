// Slice 6 gate cycler. The bench test-gate line (D3, straight through
// to the R4) is what the R4's slice06_at firmware waits on: D3 low
// releases the start gate, and the R4 re-arms when the line rises
// again. Cycling the line makes the slice re-runnable without
// re-flashing either board, and without an operator.
//
// Cycle: 30 s high (idle / re-arm window), then 50 s low (one full
// slice06_at run: ~2 s settle + 6 rounds x 5 s + report). The host
// attaches its COM3 reader during a high phase.
//
// Upload order: the R4 first — its D0/D1 stay inputs in slice06 (SCI2
// is unused), so the R3's upload over its own D0/D1 (shared with this
// board's USB adapter, crossing to the R4) cannot be corrupted.
// COM4 is read-only for the host: writing to it would fight the
// cross-wired UART lines, so this sketch prints nothing.
void setup() {
  digitalWrite(3, HIGH); // line defined high before the driver engages
  pinMode(3, OUTPUT);
}

void loop() {
  delay(30000);           // high: re-arm window
  digitalWrite(3, LOW);   // falling edge: run one AT test
  delay(50000);
  digitalWrite(3, HIGH);  // rise: R4 re-arms
}
