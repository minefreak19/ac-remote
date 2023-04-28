void setup() {
  Serial.begin(230400);

  Serial.println("Initialised");
}

void loop() {
  Serial.println(millis());
}
