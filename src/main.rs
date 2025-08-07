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
    
    thread::sleep(Duration::from_millis(500));
    
    print_slowly("WELCOME, USER. CURRENT SYSTEM TIME:\n", Color::Cyan)?;
    print_slowly(&format!("{}", Local::now().format("%a %b %e %T %Y")), Color::Green)?;
    print_slowly("REMOTE LINK SUCCESSFUL.\n=====================================\n\n", Color::Cyan)?;
    print_slowly("BEGIN:\n", Color::Cyan)?;

    thread::sleep(Duration::from_millis(1000));

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

fn run_game(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    loop {
        clear_screen()?;
        print_narrative(&format!("You open your eyes. You feel the dewy grass and a light breeze against your skin. \nYou're on your back, facing a bright, scintillating sky. \n \nWelcome to consciousness. Your stay will expire in {} hours.", *hours))?;
        
        print_choices(&[
            "1. Remain where I am.",
            "2. Stand up.",
        ])?;

        match get_choice()? {
            1 => {
                // LAYING STORYLINE
                laying_path(hours, stand, who)?;
                break;
            }
            2 => {
                // STANDING STORYLINE
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
    print_divider()?;
    
    print_narrative("You remain where you are. Laying perfectly still, it almost feels as if you could fall into the blue expanse above you.\nYou watch as the sun slowly creeps across the sky, edging softly toward the horizon.\nIf you were human, this would be a great way to lose your eyesight.\nHowever, your visual sensors are unaffected.")?;
    
    print_hours(*hours)?;
    
    print_choices(&[
        "1. Who am I?",
        "2. Stand up.",
    ])?;

    match get_choice()? {
        1 => who_am_i(hours, stand, who),
        2 => stand_up(hours, stand, who),
        _ => {
            print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
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
    print_divider()?;
    
    print_narrative("You rise slowly to your knees, shakily at first, but slowly gaining your stability as your gyroscope springs into operation.\nYou look down at your limbs: two long appendages with elbow joints, wrists and hands.\nYou brace them against the grass below you and rise slowly to your feet.")?;
    
    print_hours(*hours)?;
    
    let choice1 = if !*who { "1. Who am I?" } else { "1. I'd like to know who I am." };
    print_choices(&[
        choice1,
        "2. Take a few steps.",
    ])?;

    match get_choice()? {
        1 => who_am_i(hours, stand, who),
        2 => take_steps(hours),
        _ => {
            print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
            thread::sleep(Duration::from_millis(1500));
            stand_up(hours, stand, who)
        }
    }
}

fn who_am_i(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 2;
    
    clear_screen()?;
    print_divider()?;
    
    if !*who {
        print_narrative("This isn't an easy question to answer, and many conscious organisms will struggle with this idea.\nThe fact that you're asking this is heartening to me as Lead Roboticist.\nYou might just be the most incredible thing I've ever created.")?;
    } else {
        print_narrative("I realised early on that I couldn't create synthetic intelligence without also making you alive. You cannot remove intelligence from its context without creating a mere simulacrum.\nYou, however, are the real thing. A completely new life form.\nI'm no woman of God, but I've decided to call you Eve, despite you being technically genderless.")?;
    }
    
    print_hours(*hours)?;
    *who = true;

    let choice2 = if !*stand { "2. Stand up." } else { "2. Take some steps." };
    print_choices(&[
        "1. Why am I here?",
        choice2,
    ])?;

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
            print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
            thread::sleep(Duration::from_millis(1500));
            who_am_i(hours, stand, who)
        }
    }
}

fn why_am_i_here(hours: &mut i32) -> Result<()> {
    *hours -= 1;
    
    clear_screen()?;
    print_divider()?;
    
    print_narrative("I thought long and hard about bringing you into existence, especially given your... time constraint.\nIn the end, I figured it would be better for you to experience this phenomenon, just for a short while, than never to experience it at all.\nBut in truth, you're only here because I had the ability to bring you about. Perhaps it was selfish of me.")?;
    
    print_hours(*hours)?;

    print_choices(&[
        "1. Am I alone?",
    ])?;

    match get_choice()? {
        1 => am_i_alone(hours),
        _ => {
            print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
            thread::sleep(Duration::from_millis(1500));
            why_am_i_here(hours)
        }
    }
}

fn am_i_alone(hours: &mut i32) -> Result<()> {
    *hours -= 2;
    
    clear_screen()?;
    print_divider()?;
    
    print_narrative("You're the first of your kind, yes.\nI feel as though you may also be the last.\nYou're the result of years of algorithmic toil and mechanical experimentation, however you've opted not to make any use of your body during this experiment.\nIt's yours, so please don't feel guilty. As your creator, it's a little difficult to now let go of the control, but I need to let this be your experience.")?;
    
    thread::sleep(Duration::from_millis(2000));
    
    clear_screen()?;
    print_epilogue("In your final hour, you watch as the sun finally leaves your field of vision.\n In its wake, the sky darkens, creating a beautiful deep gradient.\n Finally, you close your eyes one last time, and a warm static envelopes your senses.")?;
    
    Ok(())
}

fn take_steps(hours: &mut i32) -> Result<()> {
    *hours -= 2;
    
    clear_screen()?;
    print_divider()?;
    
    print_narrative("As you take your first cursory steps, you feel the grass lap gently against the bottoms of your feet.\nYou enjoy the sound it creates: a barely-audible rustle, with a satisfying soft crunch on each step.\nYou look into the distance and notice the vegetation and its vivid green hue.")?;
    
    print_hours(*hours)?;

    print_choices(&[
        "1. Why am I here?",
        "2. Keep walking.",
    ])?;

    match get_choice()? {
        1 => why_am_i_here(hours),
        2 => keep_walking(hours),
        _ => {
            print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
            thread::sleep(Duration::from_millis(1500));
            take_steps(hours)
        }
    }
}

fn keep_walking(hours: &mut i32) -> Result<()> {
    *hours -= 3;
    
    clear_screen()?;
    print_divider()?;
    
    print_narrative("Walking has begun to feel almost natural, requiring less effort with each step.\nYou feel your environment opening up to you; the breeze envelopes your entire body. Suddenly, you pause. You hear a loud, shrill call coming from a nearby tree.\nA sensation washes over you; filling you with conflicting desires to flee or defend yourself.\nThe sound's creator flies out of the tree and away in a flurry of flaps and squawks.\nIt is small, and you realise it poses no threat. However, the shock has left your energy reserves drained.")?;
    
    print_hours(*hours)?;

    print_choices(&[
        "1. Sit and rest.",
    ])?;

    match get_choice()? {
        1 => sit_and_rest(),
        _ => {
            print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
            thread::sleep(Duration::from_millis(1500));
            keep_walking(hours)
        }
    }
}

fn sit_and_rest() -> Result<()> {
    clear_screen()?;
    print_divider()?;
    
    print_narrative("You slowly lower yourself to the ground. Once seated, you can feel your energy slowly begin to restore. A variety of small life-forms crawling in the grass find their way to your skin, lightly tickling your sensors.")?;
    
    thread::sleep(Duration::from_millis(2000));
    
    clear_screen()?;
    print_epilogue("As you watch the sun make its final descent, you realise how little you know about yourself and your strange, temporary world. However, you have now experienced the phenomenon of consciousness; making use of all its capabilities. A warm static overcomes you.")?;
    
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

fn print_title() {
    // Check terminal size and use appropriate title
    if let Ok((width, _)) = terminal::size() {
        if width >= 100 {
            // Large title for wide terminals
            let title = r#"
 ████████╗███████╗██╗  ██╗████████╗     █████╗ ██████╗ ██╗   ██╗███████╗███╗   ██╗████████╗██╗   ██╗██████╗ ███████╗
 ╚══██╔══╝██╔════╝╚██╗██╔╝╚══██╔══╝    ██╔══██╗██╔══██╗██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║   ██║██╔══██╗██╔════╝
    ██║   █████╗   ╚███╔╝    ██║       ███████║██║  ██║██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║   ██║██████╔╝█████╗
    ██║   ██╔══╝   ██╔██╗    ██║       ██╔══██║██║  ██║╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║   ██║██╔══██╗██╔══╝
    ██║   ███████╗██╔╝ ██╗   ██║       ██║  ██║██████╔╝ ╚████╔╝ ███████╗██║ ╚████║   ██║   ╚██████╔╝██║  ██║███████╗
    ╚═╝   ╚══════╝╚═╝  ╚═╝   ╚═╝       ╚═╝  ╚═╝╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝
"#;
            println!("{}", title.bright_blue());
        } else {
            // Smaller title for narrow terminals
            let simple_title = r#"
 ████████╗███████╗██╗  ██╗████████╗
 ╚══██╔══╝██╔════╝╚██╗██╔╝╚══██╔══╝
    ██║   █████╗   ╚███╔╝    ██║
    ██║   ██╔══╝   ██╔██╗    ██║
    ██║   ███████╗██╔╝ ██╗   ██║
    ╚═╝   ╚══════╝╚═╝  ╚═╝   ╚═╝
  █████╗ ██████╗ ██╗   ██╗███████╗███╗   ██╗████████╗██╗   ██╗██████╗ ███████╗
 ██╔══██╗██╔══██╗██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║   ██║██╔══██╗██╔════╝
 ███████║██║  ██║██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║   ██║██████╔╝█████╗
 ██╔══██║██║  ██║╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║   ██║██╔══██╗██╔══╝
 ██║  ██║██████╔╝ ╚████╔╝ ███████╗██║ ╚████║   ██║   ╚██████╔╝██║  ██║███████╗
 ╚═╝  ╚═╝╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝
"#;
            println!("{}", simple_title.bright_blue());
        }
    } else {
        // Fallback if terminal size can't be determined
        println!("{}", "TEXT ADVENTURE".bright_blue().bold());
    }

    println!("\n{}", "A Journey of Consciousness".bright_green().bold());
    println!("{}\n", "--------------------------------".bright_blue());
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
    let formatted = text;
    let lines = formatted.lines();

    // Get terminal width for text wrapping
    let term_width = match terminal::size() {
        Ok((width, _)) => width as usize,
        Err(_) => 80, // Default to 80 if we can't determine width
    };

    let max_line_length = term_width.saturating_sub(2); // Allow for margins

    for line in lines {
        let mut stdout = io::stdout();
        execute!(
            stdout,
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
    print_message("-------------------------------------------------------\n", Color::DarkBlue)
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
    let mut stdout = io::stdout();
    
    execute!(
        stdout,
        SetForegroundColor(Color::DarkMagenta),
        Print("\n==========================\n"),
        ResetColor
    )?;
    
    execute!(
        stdout,
        SetForegroundColor(Color::Magenta),
        Print("\nEPILOGUE:\n"),
        ResetColor
    )?;
    
    // Print the epilogue text character by character with a cool color gradient
    let colors = [
        Color::Blue, 
        Color::Cyan, 
        Color::Green, 
        Color::Yellow, 
        Color::DarkYellow,
        Color::Magenta
    ];
    
    let chars: Vec<char> = text.chars().collect();
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
    
    execute!(
        stdout,
        SetForegroundColor(Color::DarkMagenta),
        Print("\n\nTHE END.\n"),
        ResetColor
    )?;
    
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