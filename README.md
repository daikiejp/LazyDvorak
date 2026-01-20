# LazyDvorak

<div align="center">

![LazyDvorak Banner](https://img.shields.io/badge/LazyDvorak-v0.1.0-cyan?style=for-the-badge)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

A typing practice application designed for programmers using Dvorak and QWERTY layouts.

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Contributing](#contributing) • [License](#license)

</div>

---

[LazyDvorak](https://utfs.io/f/DBjkFwb5pfbRtJVPsjyyr8n5dGbEgTcieAKoYQzaRIDHmx0J)

## ⚠️ Disclaimer

The project is in an early stage, so there may be some bugs and errors. I apologize for them and will be continuously reviewing and improving it.

## 🎯 Features

- **Multiple Keyboard Layouts**: Support for both Dvorak Programmer and QWERTY layouts
- **Programming-Focused Practice**:
  - Vim commands practice
  - Language-specific keywords (Lua, Ruby, Rust, TypeScript, Python)
  - Real code snippets from common algorithms
- **Progressive Difficulty Levels**:
  - **Basic**: Words & Commands
  - **Intermediate**: Sentences
  - **Advanced**: Real code tests
- **Real-time Statistics**: Track WPM, accuracy, errors, and correct keystrokes
- **Visual Keyboard**: Interactive keyboard visualization with key highlighting
- **Multi-language UI**: English, Spanish, and Japanese support
- **Customizable Sessions**: Choose between unlimited practice or set specific exercise counts (10, 25, 50, 100)
- **Terminal-based UI**: Built with Ratatui for a smooth terminal experience

## 📋 TODO

[ ] Analyze whether the WPM calculation is correct

[ ] Shift and Caps only work when used in combination; they should be displayed when switching layouts

[ ] Increase test coverage for the Real Code Test

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### From Source

1. Clone the repository:

```bash
git clone https://github.com/daikiejp/lazydvorak.git
cd lazydvorak
```

2. Build the project:

```bash
cargo build --release
# or
# check your architecture at:
rustc -vV
# windows
cargo build --release --target x86_64-pc-windows-gnu
# macos
cargo build --release --target aarch64-apple-darwin
# linux
cargo build --release --target x86_64-unknown-linux-gnu
```

3. Run the application:

```bash
cargo run --release
```

### Install Globally

```bash
cargo install --path .
```

Then run from anywhere:

```bash
lazydvorak
```

## 🚀 Usage

### Basic Usage

Start the application with default settings (English, Dvorak layout):

```bash
lazydvorak
```

### Command Line Options

```bash
lazydvorak [OPTIONS]

Options:
  -l, --lang <LANG>        Language for UI [default: en] [possible values: en, es, ja]
  -k, --layout <LAYOUT>    Keyboard layout [default: dvorak] [possible values: dvorak, qwerty]
  -h, --help              Print help
  -V, --version           Print version
```

### Examples

Start with Spanish UI and QWERTY layout:

```bash
lazydvorak --lang es --layout qwerty
```

Start with Japanese UI and Dvorak layout:

```bash
lazydvorak -l ja -k dvorak
```

### Navigation

#### Main Menu

- `↑/↓`: Navigate through options
- `Enter`: Select an option
- `q`: Quit the application

#### Practice Mode

- Type the characters shown in the target text
- `Esc`: Return to menu
- `Backspace`: Delete last character

#### Settings

- `↑/↓`: Navigate through settings
- `Enter/←/→`: Toggle setting
- `Esc`: Return to menu

## 🎮 Practice Modes

### 1. Words & Commands (Basic)

- **Simple Words**: Common English words
- **Vim Commands**: Essential Vim navigation and editing commands
- **Words by Language**: Language-specific keywords
  - Lua (function, local, end, require, etc.)
  - Ruby (def, class, module, etc.)
  - TypeScript (const, interface, async, etc.)
  - Rust (fn, let, mut, impl, etc.)
  - Python (def, class, import, etc.)

### 2. Sentences (Intermediate)

- **Normal Sentences**: Programming-related sentences
- **Dvorak Sentences**: Texts optimized for Dvorak practice
- **QWERTY Sentences**: Texts optimized for QWERTY practice

### 3. Real Code Test (Advanced)

Practice with actual code snippets implementing common algorithms:

- Fibonacci sequence
- Quicksort
- Factorial
- Bubble sort (Python only)

Available in: Lua, Ruby, Rust, TypeScript, and Python

## 📊 Statistics

The application tracks and displays:

- ✓ **Correct keystrokes**: Number of correctly typed characters
- ✗ **Errors**: Number of typing mistakes
- 📊 **Accuracy**: Percentage of correct keystrokes
- ⚡ **WPM**: Words per minute (calculated using the Monkeytype formula)

## 🛠️ Project Structure

```
lazydvorak/
├── src/
│   ├── exercises/        # Exercises tests folder
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── app.rs            # Core application logic
│   ├── keyboard.rs       # Keyboard rendering
│   ├── stats.rs          # Statistics tracking
│   ├── translations.rs   # Multi-language support
│   ├── types.rs          # Type definitions
│   └── ui.rs             # User interface rendering
├── Cargo.toml
├── LICENSE
└── README.md
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/daikiejp/lazydvorak/issues)
2. If not, create a new issue with:
   - Clear description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment (OS, Rust version, terminal)

### Suggesting Features

Open an issue with the tag `enhancement` and describe:

- The feature you'd like to see
- Why it would be useful
- How it might work

### Pull Requests

1. Fork the repository
2. Create a new branch:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. Make your changes
4. Run tests and formatting:
   ```bash
   cargo test
   cargo fmt
   cargo clippy
   ```
5. Commit your changes:
   ```bash
   git commit -m 'Add some amazing feature'
   ```
6. Push to your branch:
   ```bash
   git push origin feature/amazing-feature
   ```
7. Open a Pull Request

### Development Guidelines

- Follow Rust coding conventions
- Add tests for new features
- Update documentation as needed
- Keep commits atomic and well-described
- Ensure all tests pass before submitting PR

### Adding New Languages

To add support for a new language:

1. Add language-specific words in `src/exercises.rs`
2. Add translations in `src/translations.rs`
3. Update the language selection logic in `src/app.rs`

### Adding New Practice Modes

1. Define new `AppMode` variant in `src/types.rs`
2. Add exercise content in `src/exercises.rs`
3. Update menu handling in `src/app.rs`
4. Add UI rendering in `src/ui.rs`

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Danny Davila (@daikiejp)**

- Website: [https://daikie.jp](https://daikie.jp)
- GitHub: [@daikiejp](https://github.com/daikiejp)

## 🙏 Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- Inspired by typing practice tools like Monkeytype

## 📮 Support

If you encounter any issues or have questions:

- Open an [issue](https://github.com/daikiejp/lazydvorak/issues)
- Visit [daikie.jp](https://daikie.jp) for contact information

---

<div align="center">

Made with ❤️ by DaikieJP with Rust and Neovim my favorites tools from 2025

⭐ Star this repository if you find it useful!

</div>
