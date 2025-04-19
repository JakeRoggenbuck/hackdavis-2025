use crate::ir::{Program, Section, Command};

pub fn generate_arduino_code(program: &Program) -> String {
    let mut output = String::new();
    
    // Add the motor pin definitions and setup code
    output.push_str(r#"// Automatically Generated From IR
// Motor A connections
int enA = 9;
int in1 = 3;
int in2 = 4;
// Motor B connections
int enB = 10;
int in3 = 5;
int in4 = 6;

void setup() {
    // Set all the motor control pins to outputs
    pinMode(enA, OUTPUT);
    pinMode(enB, OUTPUT);
    pinMode(in1, OUTPUT);
    pinMode(in2, OUTPUT);
    pinMode(in3, OUTPUT);
    pinMode(in4, OUTPUT);
    
    // Turn off motors - Initial state
    digitalWrite(in1, LOW);
    digitalWrite(in2, LOW);
    digitalWrite(in3, LOW);
    digitalWrite(in4, LOW);
    analogWrite(enA, 255);
    analogWrite(enB, 255);
}

void loop() {
"#);

    // Generate code for each section
    for section in &program.sections {
        for command in &section.commands {
            match command.r#type.as_str() {
                "forward" => {
                    output.push_str(&format!("    forward({});\n", command.amount));
                }
                "backward" => {
                    output.push_str(&format!("    backwards({});\n", command.amount));
                }
                "direction" => {
                    match command.amount {
                        1 => output.push_str("    left();\n"),
                        2 => output.push_str("    right();\n"),
                        _ => panic!("Invalid direction value: {}", command.amount),
                    }
                }
                "straight" => {
                    output.push_str("    straight();\n");
                }
                "wait" => {
                    output.push_str(&format!("    wait({});\n", command.amount));
                }
                _ => panic!("Unknown command type: {}", command.r#type),
            }
        }
    }

    // Add the motor control functions
    output.push_str(r#"}

void forward(float time){
    digitalWrite(in1, HIGH);
    digitalWrite(in2, LOW);
    float delayTime = time*1000;
    long delayLong = (long)delayTime;
    delay(delayLong);    
    digitalWrite(in1, LOW);
    digitalWrite(in2, LOW);
}

void backwards(float time){
    digitalWrite(in2, HIGH);
    digitalWrite(in1, LOW);
    float delayTime = time*1000;
    long delayLong = (long)delayTime;
    delay(delayLong);
    digitalWrite(in1, LOW);
    digitalWrite(in2, LOW);
}

void right (){
    digitalWrite(in3, LOW);
    digitalWrite(in4, HIGH);
}

void wait (float time){
    digitalWrite(in1, LOW);
    digitalWrite(in2, LOW);
    float delayTime = time*1000;
    long delayLong = (long)delayTime;
    delay(delayLong);
}

void left (){
    digitalWrite(in4, LOW);
    digitalWrite(in3, HIGH);
}

void straight(){
    digitalWrite(in4, LOW);
    digitalWrite(in3, LOW);
}
"#);

    output
} 
