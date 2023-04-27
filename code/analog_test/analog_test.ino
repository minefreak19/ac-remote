#define INPUT_PIN A0
#define SWITCH_PIN 2

#define READINGS_CAP 1536
u8 readings[READINGS_CAP] = {0};
u16 readings_count = 0;

#define ZEROES_LIMIT (READINGS_CAP + 1)

#define IS_ZERO(x) (x < 5)

void setup() {
  Serial.begin(9600);

  pinMode(INPUT_PIN, INPUT);
  pinMode(SWITCH_PIN, INPUT);

  Serial.println("Initialised");
}

bool saving = false;
int value = 0;
int zeroes = 0;
int swValue = 0;
u16 i = 0;

inline void clear_readings() {
  for (i = 0; i < readings_count; i++) {
    Serial.print(readings[i]);
    Serial.println();
  }

  readings_count = 0;
}

void loop() {
  value = analogRead(INPUT_PIN);

  if (!IS_ZERO(value)) {
    zeroes = 0;
    saving = true;
  }

  if (saving) {
    if (readings_count >= READINGS_CAP) {
      clear_readings();
    }

    readings[readings_count++] = (u8) (value >> 2);

    if (IS_ZERO(value)) zeroes++;

    if (zeroes >= ZEROES_LIMIT) saving = false;
    return;
  }

  swValue = digitalRead(SWITCH_PIN);
  if (swValue) {
    clear_readings();
  }
}
