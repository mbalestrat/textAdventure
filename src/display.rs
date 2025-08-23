use std::io::{self, Result, Write};
use std::thread;
use std::time::Duration;
use colored::*;
use rand::Rng;
use crate::sound; // Import the sound module
use crate::crt_effects::{self, print_slowly_with_phosphor, PhosphorType}; // Import the CRT effects module
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

// UI Helper functions
pub fn get_choice() -> Result<i32> {
    // Drain any pending events in the queue before waiting for input
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }

    // Small delay to ensure terminal is ready for input
    thread::sleep(Duration::from_millis(100));

    // Now wait for a valid key press
    loop {
        if let Event::Key(key_event) = event::read()? {
            if let KeyCode::Char(c) = key_event.code {
                if c.is_digit(10) {
                    let num = c.to_digit(10).unwrap() as i32;
                    if num > 0 {  // Only return digits 1-9, not 0
                        return Ok(num);
                    }
                }
            }
        }
    }
}

pub fn clear_screen() -> Result<()> {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
}

pub fn print_title() {
    let indent = 2; // Consistent with narrative text indentation
    let mut stdout = io::stdout();

    // Apply scan lines effect for CRT look
    crt_effects::draw_scan_lines(PhosphorType::Green).unwrap_or(());

    // First show the logo (SYN-TEC ASCII art)
    println!(); // Extra spacing at the top
    if let Ok((_, _)) = terminal::size() {
        // Simple text-based title for "SYN-TEC"
        execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
        println!("{}", "   ███████╗ ██╗   ██╗ ███╗   ██╗       ████████╗ ███████╗  ██████╗ ".bright_blue());
        execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
        println!("{}", "   ██╔════╝ ╚██╗ ██╔╝ ████╗  ██║       ╚══██╔══╝ ██╔════╝ ██╔════╝ ".bright_blue());
        execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
        println!("{}", "   ███████╗  ╚████╔╝  ██╔██╗ ██║ █████╗   ██║    █████╗   ██║      ".bright_blue());
        execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
        println!("{}", "   ╚════██║   ╚██╔╝   ██║╚██╗██║ ╚════╝   ██║    ██╔══╝   ██║      ".bright_blue());
        execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
        println!("{}", "   ███████║    ██║    ██║ ╚████║          ██║    ███████╗ ╚██████╗ ".bright_blue());
        execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
        println!("{}", "   ╚══════╝    ╚═╝    ╚═╝  ╚═══╝          ╚═╝    ╚══════╝  ╚═════╝ ".bright_blue());

        // Add random phosphor noise around the logo for authentic CRT look
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.2) { // 80% chance of noise
            let logo_y_position = 1; // Approximate line where logo starts
            crt_effects::phosphor_noise(indent as u16, logo_y_position, 80, 6, PhosphorType::Blue, 0.03).unwrap_or(());
        }
    }

    println!(); // Spacing after logo

    // Second, print the initializing text with phosphor glow
    let y_pos = match cursor::position() {
        Ok((_, y)) => y,
        Err(_) => 9, // Default position if cursor position can't be determined
    };

    let init_text = "Initialising...";
    crt_effects::print_with_phosphor(init_text, indent as u16, y_pos, PhosphorType::Green, 50).unwrap_or(());

    // No divider here as per requested sequence
    println!(); // Just add spacing after initialization text
}

// pub fn print_slowly(text: &str, color: Color) -> Result<()> {
//     let indent = 2; // Consistent with narrative text indentation
//     let mut stdout = io::stdout();

//     // Position at the start of the line with proper indentation
//     execute!(
//         stdout,
//         cursor::MoveToColumn(indent)
//     )?;

//     for c in text.chars() {
//         execute!(
//             stdout,
//             SetForegroundColor(color),
//             Print(c),
//             ResetColor
//         )?;
//         stdout.flush()?;
//         thread::sleep(Duration::from_millis(20));
//     }
//     println!();
//     Ok(())
// }

pub fn print_narrative(text: &str) -> Result<()> {
    print_narrative_with_phosphor(text, PhosphorType::Green)
}

// Enhanced version with phosphor glow effect
pub fn print_narrative_with_phosphor(text: &str, phosphor_type: PhosphorType) -> Result<()> {
    let formatted = text;
    let lines = formatted.lines();

    // Get terminal width for text wrapping
    let term_width = match terminal::size() {
        Ok((width, _)) => width as usize,
        Err(_) => 80, // Default to 80 if we can't determine width
    };

    let max_line_length = term_width.saturating_sub(6); // Allow for margins and indentation
    let indent = 2; // Number of spaces to indent each line

    // Get current cursor position for starting line
    let mut y_position = match cursor::position() {
        Ok((_, y)) => y,
        Err(_) => 0,
    };

    for line in lines {
        let mut stdout = io::stdout();

        // Set indentation for each line
        execute!(
            stdout,
            cursor::MoveToColumn(indent),
            SetForegroundColor(Color::White)
        )?;

        // Word wrapping for lines that are too long
        if line.len() > max_line_length {
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                // Check if adding this word would exceed the max length
                if current_line.len() + word.len() + 1 > max_line_length && !current_line.is_empty() {
                    // Use phosphor effect for the current line
                    print_slowly_with_phosphor(&current_line, indent as u16, y_position, phosphor_type, 15)?;

                    // Move to next line with additional indent for wrapped lines
                    y_position += 1;
                    current_line = word.to_string();
                } else {
                    // Add word to current line
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }

            // Print any remaining text
            if !current_line.is_empty() {
                print_slowly_with_phosphor(&current_line, indent as u16, y_position, phosphor_type, 15)?;
                y_position += 1;
            }
        } else {
            // For lines that don't need wrapping, use phosphor effect directly
            print_slowly_with_phosphor(line, indent as u16, y_position, phosphor_type, 15)?;
            y_position += 1;
        }
    }

    // Ensure cursor is positioned correctly after all text
    execute!(io::stdout(), cursor::MoveTo(0, y_position))?;

    // Add random phosphor noise effect (subtle static) after the text
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.3) { // 30% chance of noise
        let line_count = text.lines().count();
        crt_effects::phosphor_noise(indent as u16, y_position.saturating_sub(line_count as u16),
                                term_width as u16 - (indent as u16 * 2), line_count as u16,
                                phosphor_type, 0.05)?;
    }

    thread::sleep(Duration::from_millis(500));
    Ok(())
}

pub fn print_hours(hours: i32) -> Result<()> {
    let hours_text = format!("{} hours now remain.", hours);
    let indent = 2; // Consistent with narrative text indentation

    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveToColumn(indent))?;

    if hours <= 3 {
        print_message(&hours_text, Color::Red)
    } else if hours <= 6 {
        print_message(&hours_text, Color::Yellow)
    } else {
        print_message(&hours_text, Color::Green)
    }
}

pub fn print_choices(choices: &[&str]) -> Result<()> {
    let indent = 2; // Consistent with narrative text indentation
    let mut stdout = io::stdout();

    println!();

    // "What next?" prompt
    execute!(stdout, cursor::MoveToColumn(indent))?;
    print_message("What next?", Color::Cyan)?;

    // Print each choice with consistent indentation
    for choice in choices {
        println!();
        execute!(stdout, cursor::MoveToColumn(indent + 2))?; // Additional indent for choices
        print_message(choice, Color::DarkCyan)?;
    }

    println!();
    Ok(())
}

pub fn print_divider() -> Result<()> {
    println!(); // Space before divider
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveToColumn(0))?; // Divider starts at column 0
    print_message("------------------------------------------------------- ", Color::DarkBlue)?;
    println!();
    println!(); // Extra space after divider
    Ok(())
}

pub fn print_message(message: &str, color: Color) -> Result<()> {
    // This function doesn't control indentation; caller should position cursor
    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetForegroundColor(color),
        Print(message),
        ResetColor
    )
}

pub fn print_error(message: &str) -> Result<()> {
    let indent = 2; // Consistent with narrative text indentation
    let mut stdout = io::stdout();

    // Play error sound
    sound::error_sound()?;

    execute!(
        stdout,
        cursor::MoveToColumn(indent),
        SetForegroundColor(Color::Red),
        Print(message),
        ResetColor
    )?;
    println!();
    Ok(())
}

pub fn print_epilogue(text: &str) -> Result<()> {
    let mut stdout = io::stdout();
    let indent = 2; // Number of spaces to indent each line

    // Play a subtle sound for epilogue
    sound::beep()?;
    thread::sleep(Duration::from_millis(300));

    execute!(
        stdout,
        SetForegroundColor(Color::DarkMagenta),
        Print(" ========================== "),
        ResetColor
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::Magenta),
        Print(" EPILOGUE: "),
        ResetColor
    )?;

    println!();

    // Print the epilogue text character by character with a cool color gradient
    let colors = [
        Color::Blue,
        Color::Cyan,
        Color::Green,
        Color::Yellow,
        Color::DarkYellow,
        Color::Magenta
    ];

    // Split text by newlines so we can handle each line separately
    let lines = text.split('\n');

    for line in lines {
        // Move to the indented position
        execute!(
            stdout,
            cursor::MoveToColumn(indent)
        )?;

        let chars: Vec<char> = line.chars().collect();
        let total_chars = chars.len();

        for (i, c) in chars.iter().enumerate() {
            // Calculate color index based on position in text
            let color_idx = (i * colors.len()) / total_chars;
            let color = colors[color_idx];

            execute!(
                stdout,
                SetForegroundColor(color),
                Print(c),
                ResetColor
            )?;

            stdout.flush()?;
            thread::sleep(Duration::from_millis(30));
        }

        // New line after each line of text
        println!();
    }
    
    execute!(
        stdout,
        SetForegroundColor(Color::DarkMagenta),
        Print("  CONNECTION LOST. "),
        ResetColor
    )?;
    
    Ok(())
}

pub fn wait_for_key() -> Result<()> {
    loop {
        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    Ok(())
}

pub fn print_ending_screen() -> Result<()> {
    let indent = 2; // Consistent with narrative text indentation
    let mut stdout = io::stdout();

    // Play dramatic ending sound
    sound::ending_sound()?;

    // Clear the screen first
    clear_screen()?;

    // Add some spacing
    println!("\n\n");

    // Display the SYN-TEC logo in a different color
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "   ███████╗ ██╗   ██╗ ███╗   ██╗       ████████╗ ███████╗  ██████╗ ".bright_magenta());
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "   ██╔════╝ ╚██╗ ██╔╝ ████╗  ██║       ╚══██╔══╝ ██╔════╝ ██╔════╝ ".bright_magenta());
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "   ███████╗  ╚████╔╝  ██╔██╗ ██║ █████╗   ██║    █████╗   ██║      ".bright_magenta());
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "   ╚════██║   ╚██╔╝   ██║╚██╗██║ ╚════╝   ██║    ██╔══╝   ██║      ".bright_magenta());
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "   ███████║    ██║    ██║ ╚████║          ██║    ███████╗ ╚██████╗ ".bright_magenta());
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "   ╚══════╝    ╚═╝    ╚═╝  ╚═══╝          ╚═╝    ╚══════╝  ╚═════╝ ".bright_magenta());

    // Add text below with sound
    println!("\n");
    sound::beep()?;
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "C O N S C I O U S N E S S   T E R M I N A T E D".bright_red());

    // Add separator
    println!();
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "═════════════════════════════════════════════════════════════".bright_cyan());

    // Add connection information
    println!();
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "SYN-TEC INDUSTRIES - SYNTHETIC LIFE EXPERIMENT V1.0 (BETA)".bright_white());

    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "CONNECTION TERMINATED - SESSION LOGS ARCHIVED".bright_white());

    // Add date and time with final beep
    println!();
    sound::beep()?;
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}", format!("SYSTEM TIME: {}", timestamp).bright_green());

    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "Press any key to exit...".bright_white());

    // Wait for a key press
    wait_for_key()?;

    Ok(())
}

// Terminal flicker effects
// Lighter flicker that just flashes specific characters
pub fn light_flicker() -> Result<()> {
    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();
    let (cols, rows) = terminal::size()?;

    // Play flicker sound with 80% probability (so not every visual flicker has sound)
    if rng.gen_bool(0.8) {
        sound::flicker_sound()?;
    }

    // Number of characters to flicker
    let num_flickers = rng.gen_range(3..10);

    // Flicker random characters on screen
    for _ in 0..num_flickers {
        let x = rng.gen_range(0..cols);
        let y = rng.gen_range(0..rows);
        let flicker_char = match rng.gen_range(0..4) {
            0 => '█',
            1 => '▓',
            2 => '▒',
            _ => '░',
        };

        execute!(
            stdout,
            cursor::MoveTo(x, y),
            SetForegroundColor(Color::White),
            Print(flicker_char),
            ResetColor
        )?;
    }

    // Short pause to see the flicker
    thread::sleep(Duration::from_millis(50));

    // Caller will need to redraw content

    Ok(())
}

// Main function that randomly decides whether to flicker
pub fn random_flicker_check() -> Result<()> {
    let mut rng = rand::thread_rng();

    // 5% chance of a flicker effect occurring
    if rng.gen_bool(0.05) {
        // Only use the light flicker effect
        light_flicker()?;

        // No need to clear the screen after a light flicker
        // as it doesn't disrupt the whole display
    }

    Ok(())
}