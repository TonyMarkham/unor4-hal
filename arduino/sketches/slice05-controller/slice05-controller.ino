// UNO R3 autonomous full-duplex checker for slice05_rig.
// Cross D0/D1 to the R4, connect D3 straight through, and share GND.
// The UNO's hardware Serial is shared with its USB adapter: the host
// ONLY READS COM4. Writing host commands would fight the R4's TX line.
// Opening COM4 resets this controller; tests start four seconds later.
const unsigned TEST_BYTES = 4096;
const unsigned TEST_ROUNDS = 8;

uint8_t pattern(unsigned index) {
  return 'a' + ((index * 7U + index / 26U) % 26U);
}

void setup() {
  digitalWrite(3, HIGH);
  pinMode(3, OUTPUT);
  Serial.begin(115200);
  delay(4000);
}

void loop() {
  for (unsigned round = 1; round <= TEST_ROUNDS; ++round) {
    while (Serial.available()) Serial.read();
    digitalWrite(3, LOW);
    delay(100);
    unsigned sent = 0, received = 0, mismatches = 0;
    unsigned long began = millis();
    // Saturate TX while servicing RX concurrently: a continuous 4 KiB
    // UART burst, not a host-paced or stop-and-wait transfer.
    while ((sent < TEST_BYTES || received < TEST_BYTES) &&
           millis() - began < 5000) {
      if (sent < TEST_BYTES && Serial.availableForWrite() > 0) {
        Serial.write(pattern(sent++));
      }
      while (Serial.available()) {
        uint8_t byte = Serial.read();
        if (received >= TEST_BYTES || byte != pattern(received) - 'a' + 'A') {
          ++mismatches;
        }
        ++received;
      }
    }
    Serial.flush();
    digitalWrite(3, HIGH);  // R4 stops echo before the summary arrives.
    delay(100);
    Serial.print("\r\nRESULT round="); Serial.print(round);
    Serial.print(" sent="); Serial.print(sent);
    Serial.print(" received="); Serial.print(received);
    Serial.print(" mismatches="); Serial.println(mismatches);
    Serial.flush();
    delay(500);
  }
  Serial.println("DONE");
  Serial.flush();
  for (;;) {}
}
