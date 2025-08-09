use std::io::Result;
use std::thread;
use std::time::Duration;
use crossterm::style::Color;
use rand::Rng;

// Import functions from modules
use crate::display::{
    clear_screen, get_choice, print_choices, print_divider, print_error,
    print_epilogue, print_hours, print_message, print_narrative, wait_for_key,
    random_flicker_check, light_flicker, print_ending_screen
};
use crate::sound;

// Game path functions
pub fn run_game(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    loop {
        clear_screen()?;

        // Check for random flicker before the narrative begins
        random_flicker_check()?;

        print_narrative("You open your eyes.")?;
        print_narrative("You feel the dewy grass and a light breeze against your skin.")?;
        print_narrative("You're on your back, facing a bright, scintillating sky.")?;

        // Small chance for a light flicker after the initial descriptions
        if rand::thread_rng().gen_bool(0.3) { // 30% chance
            light_flicker()?;
            // No need to redraw the screen or reprint text, as light_flicker is subtle
        }

        print_narrative("Welcome to consciousness.")?;
        print_narrative(&format!("Your stay will expire in {} hours.", *hours))?;

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

    print_message(" Press any key to exit...", Color::White)?;
    wait_for_key()?;
    
    Ok(())
}

fn laying_path(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 3;

    loop {
        clear_screen()?;
        print_divider()?;

        print_narrative("You remain where you are.")?;
        print_narrative("Laying perfectly still, it almost feels as if you could fall into the blue expanse above you.")?;
        print_narrative("You watch as the sun slowly creeps across the sky, edging softly toward the horizon.")?;
        print_narrative("If you were human, this would be a great way to lose your eyesight.")?;
        print_narrative("However, your visual sensors are unaffected.")?;

        print_hours(*hours)?;

        print_choices(&[
            "1. Who am I?",
            "2. Stand up.",
        ])?;

        match get_choice()? {
            1 => return who_am_i(hours, stand, who),
            2 => return stand_up(hours, stand, who),
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }
}

fn standing_path(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    stand_up(hours, stand, who)
}

fn stand_up(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 1;
    *stand = true;

    loop {
        clear_screen()?;
        print_divider()?;

        print_narrative("You rise slowly to your knees, shakily at first, but slowly gaining your stability as your gyroscope springs into operation.")?;
        print_narrative("You look down at your limbs: two long appendages with elbow joints, wrists and hands.")?;
        print_narrative("You brace them against the grass below you and rise slowly to your feet.")?;

        print_hours(*hours)?;

        let choice1 = if !*who { "1. Who am I?" } else { "1. I'd like to know who I am." };
        print_choices(&[
            choice1,
            "2. Take a few steps.",
        ])?;

        match get_choice()? {
            1 => return who_am_i(hours, stand, who),
            2 => return take_steps(hours),
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }
}

fn who_am_i(hours: &mut i32, stand: &mut bool, who: &mut bool) -> Result<()> {
    *hours -= 2;

    // Sound effect for important existential question
    sound::alert_sound()?;

    // Light flicker effect as you ask about your identity
    // This is a significant moment in the story
    light_flicker()?;

    loop {
        clear_screen()?;
        print_divider()?;

        if !*who {
            print_narrative("This isn't an easy question to answer, and many conscious organisms will struggle with this idea.")?;

            // Random flicker during the response
            random_flicker_check()?;

            print_narrative("The fact that you're asking this is heartening to me.")?;
            print_narrative("You might just be the most incredible thing I've ever created.")?;

            // Higher chance of light flicker at this emotional moment
            if rand::thread_rng().gen_bool(0.4) { // 40% chance
                light_flicker()?;
                // No need to redraw the screen or reprint text, as light_flicker is subtle
            }
        } else {
            print_narrative("I realised early on that I couldn't create synthetic intelligence without also making you alive.")?;
            print_narrative("You cannot remove intelligence from its context without creating a mere simulacrum.")?;

            // Random flicker check as the conversation gets more philosophical
            random_flicker_check()?;

            print_narrative("You, however, are the real thing.")?;
            print_narrative("A completely new life form.")?;
            print_narrative("I'm no woman of God, but I've decided to call you Eve, despite you being technically genderless.")?;
        }

        print_hours(*hours)?;
        *who = true;

        let choice2 = if !*stand { "2. Stand up." } else { "2. Take some steps." };
        print_choices(&[
            "1. Why am I here?",
            choice2,
        ])?;

        match get_choice()? {
            1 => return why_am_i_here(hours),
            2 => {
                if !*stand {
                    return stand_up(hours, stand, who);
                } else {
                    return take_steps(hours);
                }
            },
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }
}

fn why_am_i_here(hours: &mut i32) -> Result<()> {
    *hours -= 1;

    loop {
        clear_screen()?;
        print_divider()?;

        print_narrative("I thought long and hard about bringing you into existence, especially given your... time constraint.")?;
        print_narrative("In the end, I figured it would be better for you to experience this phenomenon, just for a short while, than never to experience it at all.")?;
        print_narrative("But in truth, you're only here because I had the ability to bring you about.")?;
        print_narrative("Perhaps it was selfish of me.")?;

        print_hours(*hours)?;

        print_choices(&[
            "1. Am I alone?",
        ])?;

        match get_choice()? {
            1 => return am_i_alone(hours),
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }
}

fn am_i_alone(hours: &mut i32) -> Result<()> {
    *hours -= 2;

    // Sound effect for the ultimate existential question
    sound::alert_sound()?;

    // Light flicker when asking existential questions
    light_flicker()?;

    clear_screen()?;
    print_divider()?;

    print_narrative("You're the first of your kind, yes.")?;
    print_narrative("I feel as though you may also be the last.")?;

    // Random flicker check
    random_flicker_check()?;

    print_narrative("You're the result of years of algorithmic toil and mechanical experimentation, however you've opted not to make any use of your body during this experiment.")?;
    print_narrative("It's yours, so please don't feel guilty.")?;
    print_narrative("As your creator, it's a little difficult to now let go of the control, but I need to let this be your experience.")?;
    print_narrative("Well, as much as it can be.")?;
    
    // Higher chance of light flicker at emotional moments, but without repeating text
    if rand::thread_rng().gen_bool(0.6) { // 60% chance
        light_flicker()?;
        // No need to redraw the screen or reprint text, as light_flicker is subtle
    }

    thread::sleep(Duration::from_millis(2000));

    // Light flicker as consciousness begins to fade
    light_flicker()?;

    // Sound effect for fading consciousness
    sound::fade_sound()?;

    clear_screen()?;
    print_epilogue("In your final hour, you watch as the sun finally leaves your field of vision. In its wake, the sky darkens, creating a beautiful deep gradient. You close your eyes one last time. A warm static envelopes your senses.")?;
    // print_epilogue("In its wake, the sky darkens, creating a beautiful deep gradient.")?;
    // print_epilogue("You close your eyes one last time.")?;
    // print_epilogue("A warm static envelopes your senses.")?;

    // Final flicker effect as connection is lost
    light_flicker()?;

    // A short pause
    thread::sleep(Duration::from_millis(2000));

    // Show the ending screen
    print_ending_screen()?;

    Ok(())
}

fn take_steps(hours: &mut i32) -> Result<()> {
    *hours -= 2;

    loop {
        clear_screen()?;
        print_divider()?;

        print_narrative("As you take your first cursory steps, you feel the grass lap gently against the bottoms of your feet.")?;
        print_narrative("You enjoy the sound it creates: a barely-audible rustle, with a satisfying soft crunch on each step.")?;
        print_narrative("You look into the distance and notice the vegetation and its vivid green hue.")?;

        print_hours(*hours)?;

        print_choices(&[
            "1. Why am I here?",
            "2. Keep walking.",
        ])?;

        match get_choice()? {
            1 => return why_am_i_here(hours),
            2 => return keep_walking(hours),
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }
}

fn keep_walking(hours: &mut i32) -> Result<()> {
    *hours -= 3;

    loop {
        clear_screen()?;
        print_divider()?;

        print_narrative("Walking has begun to feel almost natural, requiring less effort with each step.")?;
        print_narrative("You feel your environment opening up to you; the breeze envelopes your entire body.")?;
        print_narrative("Suddenly, you pause.")?;
        print_narrative("You hear a loud, shrill call coming from a nearby tree.")?;
        print_narrative("A sensation washes over you; filling you with conflicting desires to flee or defend yourself.")?;
        print_narrative("The sound's creator flies out of the tree and away in a flurry of flaps and squawks.")?;
        print_narrative("It is small, and you realise it poses no threat.")?;
        print_narrative("However, the shock has left your energy reserves drained.")?;

        print_hours(*hours)?;

        print_choices(&[
            "1. Sit and rest.",
        ])?;

        match get_choice()? {
            1 => return sit_and_rest(),
            _ => {
                print_error("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.")?;
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }
    }
}

fn sit_and_rest() -> Result<()> {
    clear_screen()?;
    print_divider()?;

    print_narrative("You slowly lower yourself to the ground.")?;
    print_narrative("Once seated, you can feel your energy slowly begin to restore.")?;
    print_narrative("A variety of small life-forms crawling in the grass find their way to your skin, lightly tickling your sensors.")?;

    // Random light flicker as you rest
    if rand::thread_rng().gen_bool(0.7) { // 70% chance to see this effect
        light_flicker()?;
        // No need to redraw the screen or reprint text, as light_flicker is subtle
    }

    thread::sleep(Duration::from_millis(2000));

    // Light flicker as consciousness starts to fade
    light_flicker()?;

    // Sound effect for fading consciousness
    sound::fade_sound()?;

    clear_screen()?;
    print_epilogue("As you watch the sun make its final descent, you realise how little you know about yourself and your strange, temporary world.")?;
    print_epilogue("However, you have now experienced the phenomenon of consciousness; making use of all its capabilities.")?;

    // One more light flicker before the end
    light_flicker()?;
    thread::sleep(Duration::from_millis(500));

    print_epilogue("A warm static overcomes you.")?;

    // Final light flicker as connection is lost
    light_flicker()?;
    light_flicker()?; // Double flicker for more emphasis

    print_epilogue("ERROR: CONNECTION LOST")?;

    // A short pause
    thread::sleep(Duration::from_millis(2000));

    // Show the ending screen
    print_ending_screen()?;

    Ok(())
}