void setup() {
  Serial.begin(9600);

  Serial.println("Initialised");
}

void loop() {
  Serial.println(millis());
}
