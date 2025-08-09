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

// Include project modules
mod display;
mod narrative;
mod sound;
mod crt_effects;

// Import necessary functions from modules
use display::{clear_screen, print_slowly, print_title};
use narrative::run_game;
use sound::{beep, connection_sound, boot_sound};
use crt_effects::{PhosphorType, crt_power_on, print_slowly_with_phosphor};

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

    // Classic PC XT boot sound with CRT power-on effect
    boot_sound()?;
    crt_power_on(PhosphorType::Green)?;
    print_title();

    thread::sleep(Duration::from_millis(1000));

    // Welcome message with phosphor persistence effect
    print_slowly_with_phosphor("WELCOME, USER. CURRENT SYSTEM TIME: ", 2, 10, PhosphorType::Green, 30)?;

    let time_str = format!("{}", Local::now().format("%a %b %e %T %Y"));
    print_slowly_with_phosphor(&time_str, 2, 11, PhosphorType::Blue, 30)?;

    // Modem-like connection sound for establishing link
    print_slowly_with_phosphor("ESTABLISHING REMOTE LINK ", 2, 13, PhosphorType::Green, 30)?;
    connection_sound()?;
    print_slowly_with_phosphor("===================================== ", 25, 13, PhosphorType::Green, 10)?;

    // Success beep when connection is complete
    beep()?;
    print_slowly_with_phosphor("BEGIN.", 2, 15, PhosphorType::Amber, 50)?;

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