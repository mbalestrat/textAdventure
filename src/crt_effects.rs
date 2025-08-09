use std::io::{self, Result, Write};
use std::thread;
use std::time::Duration;
use rand::Rng;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, Attribute, SetAttribute},
    terminal,
};

// Colors for phosphor glow (different brightness levels)
const PHOSPHOR_BRIGHT: Color = Color::Rgb { r: 144, g: 238, b: 144 }; // Bright green phosphor
const PHOSPHOR_MEDIUM: Color = Color::Rgb { r: 85, g: 160, b: 85 };   // Medium brightness
const PHOSPHOR_DIM: Color = Color::Rgb { r: 40, g: 80, b: 40 };       // Dim/fading phosphor

// Amber phosphor colors (alternative color scheme)
const AMBER_BRIGHT: Color = Color::Rgb { r: 255, g: 176, b: 0 };
const AMBER_MEDIUM: Color = Color::Rgb { r: 180, g: 120, b: 0 };
const AMBER_DIM: Color = Color::Rgb { r: 100, g: 70, b: 0 };

// Blue phosphor colors (like old IBM terminals)
const BLUE_BRIGHT: Color = Color::Rgb { r: 100, g: 180, b: 255 };
const BLUE_MEDIUM: Color = Color::Rgb { r: 65, g: 120, b: 180 };
const BLUE_DIM: Color = Color::Rgb { r: 30, g: 60, b: 120 };

// Phosphor color scheme enum
#[derive(Clone, Copy)]
pub enum PhosphorType {
    Green,
    Amber,
    Blue,
}

// Get phosphor colors based on type
fn get_phosphor_colors(phosphor_type: PhosphorType) -> (Color, Color, Color) {
    match phosphor_type {
        PhosphorType::Green => (PHOSPHOR_BRIGHT, PHOSPHOR_MEDIUM, PHOSPHOR_DIM),
        PhosphorType::Amber => (AMBER_BRIGHT, AMBER_MEDIUM, AMBER_DIM),
        PhosphorType::Blue => (BLUE_BRIGHT, BLUE_MEDIUM, BLUE_DIM),
    }
}

// Print text with phosphor persistence effect
pub fn print_with_phosphor(
    text: &str, 
    x: u16, 
    y: u16, 
    phosphor_type: PhosphorType,
    glow_delay_ms: u64
) -> Result<()> {
    let mut stdout = io::stdout();
    let (bright, medium, dim) = get_phosphor_colors(phosphor_type);
    
    // First pass - bright phosphor (initial activation)
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(bright),
        Print(text),
        ResetColor
    )?;
    stdout.flush()?;
    
    thread::sleep(Duration::from_millis(glow_delay_ms));
    
    // Second pass - medium phosphor (initial fade)
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(medium),
        Print(text),
        ResetColor
    )?;
    stdout.flush()?;
    
    thread::sleep(Duration::from_millis(glow_delay_ms));
    
    // Third pass - dim phosphor (final state)
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(dim),
        Print(text),
        ResetColor
    )?;
    stdout.flush()?;
    
    // Optionally, you can add a fourth pass to return to normal text color
    thread::sleep(Duration::from_millis(glow_delay_ms));
    
    // Return to normal color (terminal default or specified final color)
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(Color::White),
        Print(text),
        ResetColor
    )?;
    
    Ok(())
}

// Print a character by character with phosphor glow
pub fn print_slowly_with_phosphor(
    text: &str, 
    x: u16, 
    y: u16, 
    phosphor_type: PhosphorType,
    char_delay_ms: u64
) -> Result<()> {
    let mut stdout = io::stdout();
    let (bright, medium, dim) = get_phosphor_colors(phosphor_type);
    let mut current_x = x;
    
    for c in text.chars() {
        // Print the current character with bright phosphor
        execute!(
            stdout,
            cursor::MoveTo(current_x, y),
            SetForegroundColor(bright),
            Print(c),
            ResetColor
        )?;
        stdout.flush()?;
        
        // If not the first character, update the previous character to medium glow
        if current_x > x {
            execute!(
                stdout,
                cursor::MoveTo(current_x - 1, y),
                SetForegroundColor(medium),
                Print(text.chars().nth((current_x - x - 1) as usize).unwrap_or(' ')),
                ResetColor
            )?;
        }
        
        // If not the first or second character, update the character before previous to dim
        if current_x > x + 1 {
            execute!(
                stdout,
                cursor::MoveTo(current_x - 2, y),
                SetForegroundColor(dim),
                Print(text.chars().nth((current_x - x - 2) as usize).unwrap_or(' ')),
                ResetColor
            )?;
        }
        
        // If not among the first three characters, return earlier characters to normal
        if current_x > x + 2 {
            execute!(
                stdout,
                cursor::MoveTo(current_x - 3, y),
                SetForegroundColor(Color::White),
                Print(text.chars().nth((current_x - x - 3) as usize).unwrap_or(' ')),
                ResetColor
            )?;
        }
        
        current_x += 1;
        thread::sleep(Duration::from_millis(char_delay_ms));
    }
    
    // Fade out the last few characters after completing the text
    thread::sleep(Duration::from_millis(char_delay_ms));
    
    // Fade the last two characters
    for i in 0..std::cmp::min(3, text.len() as u16) {
        let pos = current_x - 1 - i;
        let char_idx = (current_x - x - 1 - i) as usize;
        let c = text.chars().nth(char_idx).unwrap_or(' ');
        
        // Medium glow
        execute!(
            stdout,
            cursor::MoveTo(pos, y),
            SetForegroundColor(medium),
            Print(c),
            ResetColor
        )?;
    }
    
    thread::sleep(Duration::from_millis(char_delay_ms));
    
    // Final dim glow on last character
    for i in 0..std::cmp::min(3, text.len() as u16) {
        let pos = current_x - 1 - i;
        let char_idx = (current_x - x - 1 - i) as usize;
        let c = text.chars().nth(char_idx).unwrap_or(' ');
        
        execute!(
            stdout,
            cursor::MoveTo(pos, y),
            SetForegroundColor(dim),
            Print(c),
            ResetColor
        )?;
    }
    
    thread::sleep(Duration::from_millis(char_delay_ms));
    
    // Return all characters to normal color
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(Color::White),
        Print(text),
        ResetColor
    )?;
    
    // Add a newline at the end
    println!();
    
    Ok(())
}

// Simulate random phosphor noise (slight static/interference)
pub fn phosphor_noise(
    x: u16, 
    y: u16, 
    width: u16, 
    height: u16, 
    phosphor_type: PhosphorType,
    intensity: f32  // 0.0 to 1.0, how much noise to display
) -> Result<()> {
    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();
    let (bright, medium, dim) = get_phosphor_colors(phosphor_type);
    
    // Save cursor position
    execute!(stdout, cursor::SavePosition)?;
    
    let noise_chars = vec!['·', ':', '·', '`', '.', ' '];
    
    // Generate random phosphor noise
    for _ in 0..((width * height) as f32 * intensity) as u16 {
        let noise_x = x + rng.gen_range(0..width);
        let noise_y = y + rng.gen_range(0..height);
        let noise_char = noise_chars[rng.gen_range(0..noise_chars.len())];
        
        // Pick a random phosphor intensity
        let color = match rng.gen_range(0..10) {
            0..=2 => bright,   // 30% chance of bright
            3..=6 => medium,   // 40% chance of medium
            _ => dim,          // 30% chance of dim
        };
        
        execute!(
            stdout,
            cursor::MoveTo(noise_x, noise_y),
            SetForegroundColor(color),
            //Print(noise_char),
            ResetColor
        )?;
    }
    
    // Restore cursor position
    execute!(stdout, cursor::RestorePosition)?;
    
    Ok(())
}

// Simulate scan lines effect
pub fn draw_scan_lines(phosphor_type: PhosphorType) -> Result<()> {
    let (cols, rows) = terminal::size()?;
    let mut stdout = io::stdout();
    let (_, _, dim) = get_phosphor_colors(phosphor_type);
    
    // Save cursor position
    execute!(stdout, cursor::SavePosition)?;
    
    // Draw scan lines (every other row)
    for y in (0..rows).step_by(2) {
        execute!(
            stdout,
            cursor::MoveTo(0, y),
            SetForegroundColor(dim)
        )?;
        
        // Draw a faint line across the screen
        for _ in 0..cols {
            execute!(stdout, Print("░"))?;
        }
    }
    
    // Restore cursor position
    execute!(stdout, cursor::RestorePosition, ResetColor)?;
    Ok(())
}

// Create a CRT power-on effect with phosphor glow
pub fn crt_power_on(phosphor_type: PhosphorType) -> Result<()> {
    let (cols, rows) = terminal::size()?;
    let mut stdout = io::stdout();
    let (bright, medium, dim) = get_phosphor_colors(phosphor_type);
    
    // Clear screen
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    
    // First: horizontal line flash
    execute!(stdout, cursor::MoveTo(0, rows / 2))?;
    for x in 0..cols {
        execute!(
            stdout,
            cursor::MoveTo(x, rows / 2),
            SetForegroundColor(bright),
            Print("═"),
            ResetColor
        )?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(1));
    }
    thread::sleep(Duration::from_millis(100));
    
    // Screen dim glow
    for y in 0..rows {
        execute!(
            stdout,
            cursor::MoveTo(0, y),
            SetForegroundColor(dim)
        )?;
        
        for _ in 0..cols {
            execute!(stdout, Print(" "))?;
        }
        
        stdout.flush()?;
        thread::sleep(Duration::from_millis(5));
    }
    
    // Vertical line sweep
    for x in 0..cols {
        for y in 0..rows {
            execute!(
                stdout,
                cursor::MoveTo(x, y),
                SetForegroundColor(medium),
                Print(" "),
                ResetColor
            )?;
        }
        stdout.flush()?;
        thread::sleep(Duration::from_millis(1));
    }
    
    // Clear and reset
    execute!(stdout, terminal::Clear(terminal::ClearType::All), ResetColor)?;
    
    Ok(())
}

// Simulate a screen with slight phosphor burn-in
pub fn phosphor_burn_in(text: &str, x: u16, y: u16, phosphor_type: PhosphorType) -> Result<()> {
    let mut stdout = io::stdout();
    let (_, _, dim) = get_phosphor_colors(phosphor_type);
    
    // Save cursor position
    execute!(stdout, cursor::SavePosition)?;
    
    // Draw burn-in text (very dim)
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(dim),
        Print(text),
        ResetColor
    )?;
    
    // Restore cursor position
    execute!(stdout, cursor::RestorePosition)?;
    
    Ok(())
}