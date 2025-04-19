# HackDavis 2025 Compiler

A Rust-based compiler that translates a custom language into Arduino C++ code for controlling a robot. This compiler takes a high-level description of robot movements and generates the corresponding Arduino code that can be uploaded to control the robot's motors.

## Architecture

The compiler is built in Rust and consists of several key components:

1. **Lexer** (`src/lexer.rs`): Tokenizes the input text into a stream of tokens
2. **Parser** (`src/parser.rs`): Converts the token stream into an Abstract Syntax Tree (AST)
3. **Intermediate Representation** (`src/ir.rs`): Defines the data structures for the program representation
4. **Code Generator** (`src/codegen.rs`): Converts the IR into Arduino C++ code

## Input Language

The input language is designed to be simple and intuitive for describing robot movements. Here's an example:

```rust
circle:
    mov direction, 1    # Turn left
    mov forward, 4      # Move forward for 4 seconds
    mov direction, 0    # Go straight

main:
    jal circle         # Call the circle function
    mov forward, 10    # Move forward for 10 seconds
    jal circle         # Call the circle function again
```

### Language Features

- **Sections**: Named blocks of code (like functions)
- **Movement Commands**:
  - `mov direction, <value>`: Controls turning (0=straight, 1=left, 2=right)
  - `mov forward, <seconds>`: Move forward for specified time
  - `mov backward, <seconds>`: Move backward for specified time
  - `mov wait, <seconds>`: Wait for specified time
- **Jump Commands**:
  - `jal <label>`: Jump to another section

## Intermediate Representation

The IR is defined in `src/ir.rs` and consists of:

```rust
pub enum Command {
    Move {
        r#type: String,
        amount: i32,
    },
    Jump {
        label: String,
    },
}

pub struct Section {
    pub name: String,
    pub commands: Vec<Command>,
}

pub struct Program {
    pub sections: Vec<Section>,
}
```

This represents the program structure after parsing, making it easier to generate the final Arduino code.

## Generated Arduino Code

The compiler generates Arduino C++ code that includes:

1. Pin definitions for motor control
2. Setup function for initializing pins
3. Movement functions (forward, backward, left, right, etc.)
4. The main program logic

Example generated code:
```cpp
// Motor pin definitions
int enA = 9;
int in1 = 3;
int in2 = 4;
int enB = 10;
int in3 = 5;
int in4 = 6;

void setup() {
    // Initialize pins
    pinMode(enA, OUTPUT);
    pinMode(enB, OUTPUT);
    // ... other pin setups
}

void loop() {
    main_loop();
}

void circle() {
    left();
    forward(4);
    straight();
}

void main_loop() {
    circle();
    forward(10);
    circle();
}

// Motor control functions
void forward(float time) {
    digitalWrite(in1, HIGH);
    digitalWrite(in2, LOW);
    delay(time * 1000);
    digitalWrite(in1, LOW);
    digitalWrite(in2, LOW);
}

// ... other motor control functions
```

## Testing

The project includes comprehensive tests in `tests/integration.rs`:

1. **Simple Program Test**: Tests basic program compilation
2. **Arduino Code Generation Test**: Verifies the generated Arduino code
3. **Multiple Sections Test**: Tests programs with multiple sections
4. **Empty Section Test**: Tests handling of empty sections
5. **Invalid Syntax Test**: Tests error handling for invalid input

Example test:
```rust
#[test]
fn test_compile_simple_program() {
    let input = r#"
    circle:
        mov direction, 1
        mov forward, 4
        mov direction, 0
    "#.to_string();

    let output = compile(input);
    // ... test assertions
}
```

## Usage

1. Write your robot program in the custom language
2. Compile it using the Rust compiler:
   ```bash
   cargo run -- input.txt
   ```
3. The compiler will generate Arduino C++ code
4. Upload the generated code to your Arduino

## Dependencies

- Rust (latest stable version)
- Arduino IDE (for uploading the generated code)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the tests: `cargo test`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 