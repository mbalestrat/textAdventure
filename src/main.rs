use std::io::{self, Result};
use std::thread;
use std::time::Duration;
use chrono::Local;
use crossterm::{
    cursor,
    execute,
    style::{Color, ResetColor},
    terminal::{self, Clear, ClearType},
};

mod narrative;
use narrative::{
    clear_screen, print_slowly, print_title, run_game
};

fn main() -> Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide
    )?;

    // Game state
    let mut hours = 12;
    let mut stand = false;
    let mut who = false;

    // Introduction
    clear_screen()?;
    print_title();
    
    thread::sleep(Duration::from_millis(1000));
    
    print_slowly("WELCOME, USER. CURRENT SYSTEM TIME: ", Color::Cyan)?;
    print_slowly(&format!("{}", Local::now().format("%a %b %e %T %Y")), Color::Green)?;
    print_slowly("REMOTE LINK SUCCESSFUL. ===================================== ", Color::Cyan)?;
    print_slowly("BEGIN.", Color::Cyan)?;

    thread::sleep(Duration::from_millis(2000));

    // Main game loop
    let result = run_game(&mut hours, &mut stand, &mut who);

    // Clean up terminal
    execute!(
        stdout,
        ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    
    result
}