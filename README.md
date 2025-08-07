# Text Adventure

A simple text-based adventure game I wrote about consciousness back in 2017, ported from C to Rust.

Inspired by InfoCom narrative games from the 80's, I wanted to make a short story about being alive.

## Features
- **Rich Terminal UI**: Custom colors, ASCII art title, and animated text
- **Interactive Experience**: Keyboard-driven navigation
- **Visual Storytelling**: Color-coded messages and gradient effects
- **Cross-Platform**: Works on macOS, Windows, and Linux
- **Narrative-Focused**: A philosophical short story about consciousness

## How to Run

### Easy Launch (Recommended)
Simply double-click the appropriate launcher for your operating system:
- **macOS**: `launch.command`
- **Windows**: `launch.bat`
- **Linux**: `launch.sh`

This will open the game in its own terminal window with the proper settings.

### Manual Launch
1. Make sure you have Rust installed. If not, install it from [https://rustup.rs/](https://rustup.rs/)
2. Navigate to the project directory
3. Run the game with:
```
cargo run
```

## Technology
The enhanced version uses:
- `crossterm` for terminal manipulation and input handling
- `colored` for text styling
- `chrono` for date/time handling

## Original Version
The original C version is preserved in `main.c`.