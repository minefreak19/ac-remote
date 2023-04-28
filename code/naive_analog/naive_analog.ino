#define INPUT_PIN A0

int value;

void setup() {
  Serial.begin(230400);

  pinMode(INPUT_PIN, INPUT);

  Serial.println("Initialised");
  
  while (1) {
    value = analogRead(INPUT_PIN);

    Serial.println(value);
  }
}

void loop() {
}
