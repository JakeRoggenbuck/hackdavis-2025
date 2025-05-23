use crate::ir::{Command, Program};

pub fn generate_arduino_code(program: &Program) -> Result<String, String> {
    let mut output = String::new();

    // Add the motor pin definitions and setup code
    output.push_str(
        r#"// Automatically Generated From IR
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
    main_loop();
}

"#,
    );

    // Generate functions for each section
    for section in &program.sections {
        let section_name = if section.name == "main" {
            "main_loop"
        } else {
            &section.name
        };
        output.push_str(&format!("void {}() {{\n", section_name));

        for command in &section.commands {
            match command {
                Command::Move { r#type, amount } => match r#type.as_str() {
                    "forward" => {
                        output.push_str(&format!("    forward({});\n", amount));
                    }
                    "backward" => {
                        output.push_str(&format!("    backwards({});\n", amount));
                    }
                    "direction" => match amount {
                        1 => output.push_str("    left();\n"),
                        2 => output.push_str("    right();\n"),
                        0 => output.push_str("    straight();\n"),
                        _ => return Err(format!("Invalid direction value: {}", amount)),
                    },
                    "wait" => {
                        output.push_str(&format!("    wait({});\n", amount));
                    }
                    _ => return Err(format!("Unknown command type: {}", r#type)),
                },
                Command::Jump { label } => {
                    let target_name = if label == "main" { "main_loop" } else { label };
                    output.push_str(&format!("    {}();\n", target_name));
                }
            }
        }

        output.push_str("}\n\n");
    }

    // Add the motor control functions
    output.push_str(
        r#"void forward(int time){
        	delay(500);
	digitalWrite(in2, HIGH);
	digitalWrite(in1, LOW);
	delay(time*1000);
	digitalWrite(in1, LOW);
	digitalWrite(in2, LOW);
	delay(500);

}

void backwards(int time){
	delay(500);
	digitalWrite(in1, HIGH);
	digitalWrite(in2, LOW);
	delay(time*1000);	
	digitalWrite(in1, LOW);
	digitalWrite(in2, LOW);
	delay(500);
}

void right(){
    	straight();
	delay(500);
	digitalWrite(in3, LOW);
	digitalWrite(in4, HIGH);
	delay(500);
}

void wait(int time){
    	digitalWrite(in1, LOW);
	digitalWrite(in2, LOW);
	// float delayTime = time*1000;
	// long delayLong = (long)delayTime;
	delay(time*1000);
}

void left(){
    	straight();
	delay(500);
	digitalWrite(in4, LOW);
	digitalWrite(in3, HIGH);
	delay(500);
}

void straight(){
    	digitalWrite(in4, LOW);
	digitalWrite(in3, LOW);
	delay(500);
}
"#,
    );

    Ok(output)
}
