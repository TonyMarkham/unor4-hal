// UART relay: anything the R4 sends to pin 0 is forwarded to the PC,
// and anything typed in the PC monitor goes out pin 1 to the R4.
void setup() {
  pinMode(0, INPUT_PULLUP);
  Serial.begin(115200);
}

void loop() {
  while (Serial.available()) {
    Serial.write(Serial.read());
  }
}