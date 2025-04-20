# 🚀 RoboCode IDE - HackDavis 2025

<div align="center">

![image](https://github.com/user-attachments/assets/d63eeed3-24af-4aa6-89f1-f2c9bf6aeb25)

A modern web-based IDE with a custom compiler backend, built for HackDavis 2025.

![RoboCode IDE](robot-asm.png)

**A revolutionary web-based IDE with a custom assembly-like language for robotics programming!**

[![Built for HackDavis](https://img.shields.io/badge/Built%20for-HackDavis%202025-blue?style=for-the-badge)](https://hackdavis.io)
[![Made with Next.js](https://img.shields.io/badge/Made%20with-Next.js-000000?style=for-the-badge&logo=next.js)](https://nextjs.org)
[![Powered by Rust](https://img.shields.io/badge/Powered%20by-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)

</div>

## 🌟 Overview

RoboCode IDE is a cutting-edge development environment that combines the power of modern web technologies with a custom-built compiler for robotics programming. Whether you're a robotics enthusiast, student, or professional developer, our platform provides an intuitive interface for writing and executing robot control code.

### ✨ Key Features

- 🎯 **Interactive Grid Visualization** - Real-time visual feedback of your robot's movements
- 💻 **Advanced Code Editor** - Powered by Monaco Editor with syntax highlighting
- 🖥️ **Integrated Terminal** - Built-in command execution with XTerm.js
- ⚡ **Real-time Compilation** - Instant feedback on your code
- 🎨 **Modern UI/UX** - Sleek interface with Tailwind CSS
- 🔧 **Custom Assembly Language** - Simplified robotics programming

## 🛠️ Tech Stack

### Frontend Powerhouse
- ⚛️ **Next.js 15** - React framework for production
- 📝 **TypeScript** - Type-safe development
- 🎨 **Tailwind CSS** - Utility-first styling
- 📊 **Monaco Editor** - VS Code-like editing experience
- 🖥️ **XTerm.js** - Terminal emulation
- 🎯 **React Icons** - Beautiful iconography

### Robust Backend
- 🦀 **Rust** - Systems programming language
- 🔧 **Custom Compiler** - Purpose-built for robotics
- 🌐 **Unicode Support** - International character compatibility
- 📦 **Module System** - Organized code structure

## 🚀 Getting Started

### Prerequisites
- Node.js 18+
- Rust and Cargo
- Git

### Quick Start

1. **Clone & Install**
   ```bash
   git clone https://github.com/yourusername/hackdavis-2025.git
   cd hackdavis-2025
   ```

2. **Frontend Setup**
   ```bash
   cd client
   npm install
   npm run dev
   ```

3. **Compiler Setup**
   ```bash
   cd compiler
   cargo build
   cargo test
   ```

## 📁 Project Structure

```
hackdavis-2025/
├── client/                  # Frontend application
│   ├── app/                # Next.js app directory
│   │   ├── components/     # React components
│   │   ├── page.tsx       # Main page
│   │   └── globals.css    # Global styles
│   └── package.json       # Frontend dependencies
│
├── compiler/               # Rust compiler backend
│   ├── src/               # Source code
│   │   ├── lexer.rs      # Token analysis
│   │   ├── parser.rs     # AST generation
│   │   └── lib.rs        # Core functionality
│   └── Cargo.toml        # Rust dependencies
│
└── README.md              # Project documentation
```

## 🎮 Usage Example

```assembly
# Create a square pattern
circle:
    mov direction, 1    # Set direction
    mov forward, 4      # Move 4 units
    mov direction, 0    # Reset direction

main:
    jal circle          # Jump to circle routine
    mov forward, 4      # Move forward
    jal circle          # Repeat pattern
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- HackDavis 2025 organizers and mentors
- The amazing Next.js and Rust communities
- All contributors and supporters

---

<div align="center">

**Built with ❤️ at HackDavis 2025**

[Report Bug](https://github.com/yourusername/hackdavis-2025/issues) · [Request Feature](https://github.com/yourusername/hackdavis-2025/issues)

</div>
