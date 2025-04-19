# HackDavis 2025 Project

![image](https://github.com/user-attachments/assets/d63eeed3-24af-4aa6-89f1-f2c9bf6aeb25)

A modern web-based IDE with a custom compiler backend, built for HackDavis 2025.

## Features

### Frontend (Next.js)
- Split-screen interface with code editor and terminal
- Monaco Editor for code editing
- XTerm.js terminal emulator
- Real-time compilation status indicators
- Modern UI with Tailwind CSS
- Responsive design

### Backend (Rust)
- Custom compiler implementation
- Lexer and parser for custom language
- Unicode support for identifiers
- Section-based code organization
- Command parsing and validation

## Tech Stack

### Frontend
- Next.js 15
- TypeScript
- Tailwind CSS
- Monaco Editor
- XTerm.js
- React Icons

### Backend
- Rust
- Custom lexer and parser
- Unicode support

## Getting Started

### Prerequisites
- Node.js 18+ and npm
- Rust and Cargo
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/hackdavis-2025.git
cd hackdavis-2025
```

2. Install frontend dependencies:
```bash
npm install
```

3. Install backend dependencies:
```bash
cd hackdavis-2025-compiler
cargo build
```

### Running the Project

1. Start the frontend development server:
```bash
npm run dev
```

2. Run the compiler tests:
```bash
cd hackdavis-2025-compiler
cargo test
```

## Project Structure

```
hackdavis-2025/
├── src/
│   ├── app/
│   │   ├── page.tsx
│   │   └── globals.css
│   └── components/
│       └── IDE.tsx
├── hackdavis-2025-compiler/
│   └── src/
│       ├── lib.rs
│       ├── lexer.rs
│       └── parser.rs
└── README.md
```

## Development

### Frontend Development
The frontend is built with Next.js and uses:
- Client-side components for the IDE interface
- Dynamic imports for Monaco Editor and XTerm.js
- Tailwind CSS for styling
- React hooks for state management

### Backend Development
The backend compiler is written in Rust and includes:
- Lexer for tokenizing source code
- Parser for building abstract syntax trees
- Unicode support for identifiers
- Section-based code organization

## Testing

### Frontend Tests
```bash
npm test
```

### Backend Tests
```bash
cd hackdavis-2025-compiler
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- HackDavis 2025 organizers
- Next.js team
- Rust community
- Monaco Editor and XTerm.js maintainers
