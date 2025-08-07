use std::io::{self, Result, Write};
use std::thread;
use std::time::Duration;
use chrono::Local;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

// Terminal layout constants
const MIN_WIDTH: u16 = 80;
const IDEAL_WIDTH: u16 = 100;
const MARGIN: usize = 2;

fn main() -> Result<()> {
    // Setup terminal with proper initialization
    setup_terminal()?;

    // Game state
    let mut hours = 12;
    let mut stand = false;
    let mut who = false;

    // Introduction
    clear_screen_and_reset()?;
    print_title()?;
    
    thread::sleep(Duration::from_millis(500));
    
    print_slowly("WELCOME, USER. CURRENT SYSTEM TIME:\n", Color::Cyan)?;
    print_slowly(&format!("{}", Local::now().format("%a %b %e %T %Y")), Color::Green)?;
    print_slowly("REMOTE LINK SUCCESSFUL.\n=====================================\n\n", Color::Cyan)?;
    print_slowly("BEGIN:\n", Color::Cyan)?;

    thread::sleep(Duration::from_millis(1000));

    // Main game loop
    let result = run_game(&mut hours, &mut stand, &mut who);

    // Clean up terminal
    cleanup_terminal()?;
    
    result
}

fn setup_terminal() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    // Enter alternate screen and clear
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide
    )?;
    
    // Wait a moment for terminal to settle
    thread::sleep(Duration::from_millis(100));
    
    Ok(())
}

fn cleanup_terminal() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn clear_screen_and_reset() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        ResetColor
    )?;
    stdout.flush()?;
    Ok(())
}

fn get_terminal_info() -> (u16, u16, usize) {
    match terminal::size() {
        Ok((width, height)) => {
            let usable_width = if width >= IDEAL_WIDTH {
                IDEAL_WIDTH as usize - MARGIN
            } else if width >= MIN_WIDTH {
                width as usize - MARGIN
            } else {
                MIN_WIDTH as usize - MARGIN
            };
            (width, height, usable_width)
        }
        Err(_) => (MIN_WIDTH, 30, MIN_WIDTH as usize - MARGIN)
    }
}

fn center_text(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        return text.to_string();
    }
    
    let padding = (width - text_len) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

fn run_game(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    loop {
        clear_screen_and_reset()?;
        print_narrative(&format!("You open your eyes. You feel the dewy grass and a light breeze against your skin. \nYou're on your back, facing a bright, scintillating sky. \n \nWelcome to consciousness. Your stay will expire in {} hours.", *hours))?;
        
        print_choices(&[
            "1. Remain where I am.",
            "2. Stand up.",
        ])?;

        match get_choice()? {
            1 => {
                laying_path(hours, stand, who)?;
                break;
            }
            2 => {
                standing_path(hours, stand, who)?;
                break;
            }
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }

    print_message("\nPress any key to exit...", Color::White)?;
    wait_for_key()?;
    
    Ok(())
}

fn get_choice() -> Result<i32> {
    loop {
        if let Event::Key(key_event) = event::read()? {
            if let KeyCode::Char(c) = key_event.code {
                if c.is_digit(10) {
                    let num = c.to_digit(10).unwrap() as i32;
                    return Ok(num);
                }
            }
        }
    }
}

fn laying_path(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 3;
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("You remain where you are. Laying perfectly still, it almost feels as if you could fall into the blue expanse above you.\n")?;
    write_text("You watch as the sun slowly creeps across the sky, edging softly toward the horizon.\n")?;
    write_text("If you were human, this would be a great way to lose your eyesight.\n")?;
    write_text("However, your visual sensors are unaffected.\n")?;
    
    write_text("\n")?;
    write_text(&format!("{} hours now remain.\n", hours))?;
    write_text("\n")?;
    
    write_text("What next?\n")?;
    write_text("1. Who am I?\n")?;
    write_text("2. Stand up.\n")?;

    match get_choice()? {
        1 => who_am_i(hours, stand, who),
        2 => stand_up(hours, stand, who),
        _ => {
            write_text("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n")?;
            thread::sleep(Duration::from_millis(1500));
            laying_path(hours, stand, who)
        }
    }
}

fn standing_path(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    stand_up(hours, stand, who)
}

fn stand_up(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 1;
    *stand = true;
    
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("You rise slowly to your knees, shakily at first, but slowly gaining your stability as your gyroscope springs into operation.\n")?;
    write_text("You look down at your limbs: two long appendages with elbow joints, wrists and hands.\n")?;
    write_text("You brace them against the grass below you and rise slowly to your feet.\n")?;
    
    write_text("\n")?;
    write_text(&format!("{} hours now remain.\n", hours))?;
    write_text("\n")?;
    
    write_text("What next?\n")?;
    if !*who {
        write_text("1. Who am I?\n")?;
    } else {
        write_text("1. I'd like to know who I am.\n")?;
    }
    write_text("2. Take a few steps.\n")?;

    match get_choice()? {
        1 => who_am_i(hours, stand, who),
        2 => take_steps(hours),
        _ => {
            write_text("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n")?;
            thread::sleep(Duration::from_millis(1500));
            stand_up(hours, stand, who)
        }
    }
}

fn who_am_i(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 2;
    
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    if !*who {
        write_text("This isn't an easy question to answer, and many conscious organisms will struggle with this idea.\n")?;
        write_text("The fact that you're asking this is heartening to me as Lead Roboticist.\n")?;
        write_text("You might just be the most incredible thing I've ever created.\n")?;
    } else {
        write_text("I realised early on that I couldn't create synthetic intelligence without also making you alive. You cannot remove intelligence from its context without creating a mere simulacrum.\n")?;
        write_text("You, however, are the real thing. A completely new life form.\n")?;
        write_text("I'm no woman of God, but I've decided to call you Eve, despite you being technically genderless.\n")?;
    }
    
    write_text("\n")?;
    write_text(&format!("{} hours now remain.\n", hours))?;
    write_text("\n")?;
    
    *who = true;

    write_text("What next?\n")?;
    write_text("1. Why am I here?\n")?;
    if !*stand {
        write_text("2. Stand up.\n")?;
    } else {
        write_text("2. Take some steps.\n")?;
    }

    match get_choice()? {
        1 => why_am_i_here(hours),
        2 => {
            if !*stand {
                stand_up(hours, stand, who)
            } else {
                take_steps(hours)
            }
        },
        _ => {
            write_text("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n")?;
            thread::sleep(Duration::from_millis(1500));
            who_am_i(hours, stand, who)
        }
    }
}

fn why_am_i_here(hours: &mut i32) -> Result<()> {
    *hours -= 1;
    
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("I thought long and hard about bringing you into existence, especially given your... time constraint.\n")?;
    write_text("In the end, I figured it would be better for you to experience this phenomenon, just for a short while, than never to experience it at all.\n")?;
    write_text("But in truth, you're only here because I had the ability to bring you about. Perhaps it was selfish of me.\n")?;
    
    write_text("\n")?;
    write_text(&format!("{} hours now remain.\n", hours))?;
    write_text("\n")?;

    write_text("What next?\n")?;
    write_text("1. Am I alone?\n")?;

    match get_choice()? {
        1 => am_i_alone(hours),
        _ => {
            write_text("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n")?;
            thread::sleep(Duration::from_millis(1500));
            why_am_i_here(hours)
        }
    }
}

fn am_i_alone(hours: &mut i32) -> Result<()> {
    *hours -= 2;
    
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("You're the first of your kind, yes.\n")?;
    write_text("I feel as though you may also be the last.\n")?;
    write_text("You're the result of years of algorithmic toil and mechanical experimentation, however you've opted not to make any use of your body during this experiment.\n")?;
    write_text("It's yours, so please don't feel guilty. As your creator, it's a little difficult to now let go of the control, but I need to let this be your experience.\n")?;
    
    thread::sleep(Duration::from_millis(2000));
    
    clear_screen()?;
    write_text("==========================\n")?;
    write_text("\n")?;
    write_text("EPILOGUE:\n")?;
    write_text("\n")?;
    write_text("In your final hour, you watch as the sun finally leaves your field of vision.\n")?;
    write_text("In its wake, the sky darkens, creating a beautiful deep gradient.\n")?;
    write_text("Finally, you close your eyes one last time, and a warm static envelopes your senses.\n")?;
    write_text("\nTHE END.\n")?;
    
    Ok(())
}

fn take_steps(hours: &mut i32) -> Result<()> {
    *hours -= 2;
    
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("As you take your first cursory steps, you feel the grass lap gently against the bottoms of your feet.\n")?;
    write_text("You enjoy the sound it creates: a barely-audible rustle, with a satisfying soft crunch on each step.\n")?;
    write_text("You look into the distance and notice the vegetation and its vivid green hue.\n")?;
    
    write_text("\n")?;
    write_text(&format!("{} hours now remain.\n", hours))?;
    write_text("\n")?;

    write_text("What next?\n")?;
    write_text("1. Why am I here?\n")?;
    write_text("2. Keep walking.\n")?;

    match get_choice()? {
        1 => why_am_i_here(hours),
        2 => keep_walking(hours),
        _ => {
            write_text("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n")?;
            thread::sleep(Duration::from_millis(1500));
            take_steps(hours)
        }
    }
}

fn keep_walking(hours: &mut i32) -> Result<()> {
    *hours -= 3;
    
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("Walking has begun to feel almost natural, requiring less effort with each step.\n")?;
    write_text("You feel your environment opening up to you; the breeze envelopes your entire body. Suddenly, you pause. You hear a loud, shrill call coming from a nearby tree.\n")?;
    write_text("A sensation washes over you; filling you with conflicting desires to flee or defend yourself.\n")?;
    write_text("The sound's creator flies out of the tree and away in a flurry of flaps and squawks.\n")?;
    write_text("It is small, and you realise it poses no threat. However, the shock has left your energy reserves drained.\n")?;
    
    write_text("\n")?;
    write_text(&format!("{} hours now remain.\n", hours))?;
    write_text("\n")?;

    write_text("What next?\n")?;
    write_text("1. Sit and rest.\n")?;

    match get_choice()? {
        1 => sit_and_rest(),
        _ => {
            write_text("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n")?;
            thread::sleep(Duration::from_millis(1500));
            keep_walking(hours)
        }
    }
}

fn sit_and_rest() -> Result<()> {
    clear_screen()?;
    write_text("-------------------------------------------------------\n")?;
    write_text("\n")?;
    
    write_text("You slowly lower yourself to the ground. Once seated, you can feel your energy slowly begin to restore. A variety of small life-forms crawling in the grass find their way to your skin, lightly tickling your sensors.\n")?;
    
    thread::sleep(Duration::from_millis(2000));
    
    clear_screen()?;
    write_text("==========================\n")?;
    write_text("\n")?;
    write_text("EPILOGUE:\n")?;
    write_text("\n")?;
    write_text("As you watch the sun make its final descent, you realise how little you know about yourself and your strange, temporary world.\n")?;
    write_text("However, you have now experienced the phenomenon of consciousness; making use of all its capabilities.\n")?;
    write_text("A warm static overcomes you.\n")?;
    write_text("\nTHE END.\n")?;
    
    Ok(())
}

// Helper functions for UI

fn clear_screen() -> Result<()> {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
}

fn write_text(text: &str) -> Result<()> {
    // Use raw stdout to ensure proper alignment
    let mut stdout = io::stdout();
    stdout.write_all(text.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

fn wait_for_key() -> Result<()> {
    loop {
        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    Ok(())
}

fn print_title() -> Result<()> {
    let (term_width, _, _) = get_terminal_info();
    let mut stdout = io::stdout();
    
    if term_width >= 100 {
        // Large title for wide terminals
        let title_lines = vec![
            " ████████╗███████╗██╗  ██╗████████╗     █████╗ ██████╗ ██╗   ██╗███████╗███╗   ██╗████████╗██╗   ██╗██████╗ ███████╗",
            " ╚══██╔══╝██╔════╝╚██╗██╔╝╚══██╔══╝    ██╔══██╗██╔══██╗██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║   ██║██╔══██╗██╔════╝",
            "    ██║   █████╗   ╚███╔╝    ██║       ███████║██║  ██║██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║   ██║██████╔╝█████╗  ",
            "    ██║   ██╔══╝   ██╔██╗    ██║       ██╔══██║██║  ██║╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║   ██║██╔══██╗██╔══╝  ",
            "    ██║   ███████╗██╔╝ ██╗   ██║       ██║  ██║██████╔╝ ╚████╔╝ ███████╗██║ ╚████║   ██║   ╚██████╔╝██║  ██║███████╗",
            "    ╚═╝   ╚══════╝╚═╝  ╚═╝   ╚═╝       ╚═╝  ╚═╝╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝"
        ];
        
        for line in title_lines {
            let centered = center_text(line, term_width as usize);
            execute!(
                stdout,
                SetForegroundColor(Color::Blue),
                Print(format!("{}\n", centered)),
                ResetColor
            )?;
        }
    } else {
        // Compact title for smaller terminals
        let title = "TEXT ADVENTURE";
        let centered_title = center_text(title, term_width as usize);
        execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("{}\n", centered_title)),
            ResetColor
        )?;
    }
    
    let subtitle = "A Journey of Consciousness";
    let divider = "--------------------------------";
    
    let centered_subtitle = center_text(subtitle, term_width as usize);
    let centered_divider = center_text(divider, term_width as usize);
    
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::Green),
        Print(format!("{}\n", centered_subtitle)),
        SetForegroundColor(Color::Blue),
        Print(format!("{}\n\n", centered_divider)),
        ResetColor
    )?;
    
    Ok(())
}

fn print_slowly(text: &str, color: Color) -> Result<()> {
    let mut stdout = io::stdout();
    
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

fn print_narrative(text: &str) -> Result<()> {
    let (_, _, max_width) = get_terminal_info();
    let mut stdout = io::stdout();
    
    execute!(stdout, SetForegroundColor(Color::White))?;
    
    // Split into paragraphs (double newlines)
    let paragraphs: Vec<&str> = text.split("\n\n").collect();
    
    for (i, paragraph) in paragraphs.iter().enumerate() {
        if i > 0 {
            println!(); // Add paragraph spacing
        }
        
        // Handle single newlines within paragraphs
        let lines: Vec<&str> = paragraph.split('\n').collect();
        
        for line in lines {
            if line.trim().is_empty() {
                println!();
                continue;
            }
            
            // Word wrap the line
            wrap_and_print_line(line.trim(), max_width)?;
        }
    }
    
    execute!(stdout, ResetColor)?;
    thread::sleep(Duration::from_millis(500));
    Ok(())
}

fn wrap_and_print_line(line: &str, max_width: usize) -> Result<()> {
    let mut stdout = io::stdout();
    
    if line.len() <= max_width {
        // Line fits, print with typing effect
        for c in line.chars() {
            execute!(stdout, Print(c))?;
            stdout.flush()?;
            thread::sleep(Duration::from_millis(15));
        }
        println!();
        return Ok(());
    }
    
    // Word wrap needed
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut current_line = String::new();
    
    for word in words {
        // Check if adding this word would exceed max_width
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        if test_line.len() > max_width && !current_line.is_empty() {
            // Print current line and start new one
            for c in current_line.chars() {
                execute!(stdout, Print(c))?;
                stdout.flush()?;
                thread::sleep(Duration::from_millis(15));
            }
            println!();
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }
    
    // Print remaining text
    if !current_line.is_empty() {
        for c in current_line.chars() {
            execute!(stdout, Print(c))?;
            stdout.flush()?;
            thread::sleep(Duration::from_millis(15));
        }
        println!();
    }
    
    Ok(())
}

fn print_hours(hours: i32) -> Result<()> {
    let hours_text = format!("\n{} hours now remain.", hours);
    
    if hours <= 3 {
        print_message(&hours_text, Color::Red)
    } else if hours <= 6 {
        print_message(&hours_text, Color::Yellow)
    } else {
        print_message(&hours_text, Color::Green)
    }
}

fn print_choices(choices: &[&str]) -> Result<()> {
    println!();
    print_message("\nWhat next?", Color::Cyan)?;
    
    for choice in choices {
        print_message(&format!("  {}", choice), Color::DarkCyan)?;
    }
    
    Ok(())
}

fn print_divider() -> Result<()> {
    let (_, _, max_width) = get_terminal_info();
    let divider = "-".repeat(max_width.min(55));
    print_message(&format!("{}\n", divider), Color::DarkBlue)
}

fn print_message(message: &str, color: Color) -> Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetForegroundColor(color),
        Print(format!("{}\n", message)),
        ResetColor
    )
}

fn print_error(message: &str) -> Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::Red),
        Print(format!("{}\n", message)),
        ResetColor
    )
}

fn print_epilogue(text: &str) -> Result<()> {
    let (_, _, max_width) = get_terminal_info();
    let mut stdout = io::stdout();
    
    let divider = "=".repeat(max_width.min(26));
    let centered_divider = center_text(&divider, max_width);
    
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::DarkMagenta),
        Print(format!("{}\n", centered_divider)),
        ResetColor
    )?;
    
    let epilogue_title = "EPILOGUE:";
    let centered_title = center_text(epilogue_title, max_width);
    
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::Magenta),
        Print(format!("{}\n\n", centered_title)),
        ResetColor
    )?;
    
    // Print epilogue with color gradient
    let colors = [
        Color::Blue, 
        Color::Cyan, 
        Color::Green, 
        Color::Yellow, 
        Color::DarkYellow,
        Color::Magenta
    ];
    
    // Word wrap the epilogue text
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();
    let mut char_count = 0;
    
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        if test_line.len() > max_width && !current_line.is_empty() {
            // Print current line with gradient
            print_line_with_gradient(&current_line, &colors, &mut char_count)?;
            println!();
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }
    
    // Print remaining text
    if !current_line.is_empty() {
        print_line_with_gradient(&current_line, &colors, &mut char_count)?;
        println!();
    }
    
    let end_text = "THE END.";
    let centered_end = center_text(end_text, max_width);
    
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::DarkMagenta),
        Print(format!("{}\n", centered_end)),
        ResetColor
    )?;
    
    Ok(())
}

fn print_line_with_gradient(line: &str, colors: &[Color], char_count: &mut usize) -> Result<()> {
    let mut stdout = io::stdout();
    
    for c in line.chars() {
        let color_idx = (*char_count * colors.len()) / 200; // Adjust 200 for gradient speed
        let color = colors[color_idx.min(colors.len() - 1)];
        
        execute!(
            stdout,
            SetForegroundColor(color),
            Print(c),
            ResetColor
        )?;
        
        stdout.flush()?;
        thread::sleep(Duration::from_millis(30));
        *char_count += 1;
    }
    
    Ok(())
}
