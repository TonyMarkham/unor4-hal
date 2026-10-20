// Working Arduino-core SCI9 reference. D0/D1 remain inputs so the UNO
// can be uploaded without disconnecting the UART wires.
void setup() {
  pinMode(0, INPUT);
  pinMode(1, INPUT);
  Serial.begin(9600);
}

void loop() {
  static unsigned long previous = 0;
  if (millis() - previous >= 2000) {
    previous = millis();
    Serial.println("CORE SCI9");
    Serial.print("VTOR "); Serial.println(SCB->VTOR, HEX);
    Serial.print("ISER "); Serial.println(NVIC->ISER[0], HEX);
    Serial.print("ISPR "); Serial.println(NVIC->ISPR[0], HEX);
    for (unsigned i = 0; i < 32; ++i) {
      Serial.print("I "); Serial.print(i);
      Serial.print(' '); Serial.print(R_ICU->IELSR[i], HEX);
      Serial.print(' '); Serial.println(NVIC->IP[i], HEX);
    }
    Serial.print("SCR "); Serial.println(R_SCI9->SCR, HEX);
    Serial.print("SSR "); Serial.println(R_SCI9->SSR, HEX);
    Serial.print("SMR "); Serial.println(R_SCI9->SMR, HEX);
    Serial.print("SEMR "); Serial.println(R_SCI9->SEMR, HEX);
    Serial.print("SCMR "); Serial.println(R_SCI9->SCMR, HEX);
  }
  while (Serial.available()) {
    int c = Serial.read();
    Serial.write(c >= 'a' && c <= 'z' ? c - 'a' + 'A' : c);
  }
}
