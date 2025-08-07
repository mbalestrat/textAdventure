use std::io::{self, Result, Write};
use std::thread;
use std::time::Duration;
use colored::*;
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

    // Check terminal size and use appropriate title
    if let Ok((_, _)) = terminal::size() {
        // Simple text-based title for "SYN-TEC"
        println!(""); // Empty line for spacing
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
    }

    // Initializing text with proper indentation
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "Initialising...".bright_green().bold());

    // Divider with proper indentation
    execute!(stdout, cursor::MoveToColumn(indent)).unwrap();
    println!("{}", "--------------------------------".bright_blue());
}

pub fn print_slowly(text: &str, color: Color) -> Result<()> {
    let indent = 2; // Consistent with narrative text indentation
    let mut stdout = io::stdout();

    // Position at the start of the line with proper indentation
    execute!(
        stdout,
        cursor::MoveToColumn(indent)
    )?;

    for c in text.chars() {
        execute!(
            stdout,
            SetForegroundColor(color),
            Print(c),
            ResetColor
        )?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(20));
    }
    println!();
    Ok(())
}

pub fn print_narrative(text: &str) -> Result<()> {
    let formatted = text;
    let lines = formatted.lines();

    // Get terminal width for text wrapping
    let term_width = match terminal::size() {
        Ok((width, _)) => width as usize,
        Err(_) => 80, // Default to 80 if we can't determine width
    };

    let max_line_length = term_width.saturating_sub(6); // Allow for margins and indentation
    let indent = 2; // Number of spaces to indent each line

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
                    // Print the current line and start a new one
                    for c in current_line.chars() {
                        execute!(stdout, Print(c))?;
                        stdout.flush()?;
                        thread::sleep(Duration::from_millis(15));
                    }
                    println!();
                    // Set indentation for continuation line
                    execute!(stdout, cursor::MoveToColumn(indent + 2))?; // Additional indent for wrapped lines
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
                for c in current_line.chars() {
                    execute!(stdout, Print(c))?;
                    stdout.flush()?;
                    thread::sleep(Duration::from_millis(15));
                }
                println!();
            }
        } else {
            // Original behavior for lines that don't need wrapping
            for c in line.chars() {
                execute!(stdout, Print(c))?;
                stdout.flush()?;
                thread::sleep(Duration::from_millis(15));
            }
            println!();
         }
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
    println!();
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveToColumn(0))?; // Divider starts at column 0
    print_message("------------------------------------------------------- ", Color::DarkBlue)?;
    println!();
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
        Print("  THE END. "),
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