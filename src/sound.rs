use rodio::{OutputStream, Sink, Source};
use std::io;
use std::thread;
use std::time::Duration;
use rand::Rng;

// PC Speaker tones frequencies (in Hz)
const PC_BEEP_FREQ: f32 = 800.0;  // Standard PC beep
const PC_ERROR_FREQ: f32 = 400.0;  // Lower tone for errors
const PC_ALERT_FREQ: f32 = 1200.0; // Higher tone for alerts
const PC_SUCCESS_FREQ: f32 = 1000.0; // Success tone

// Helper function to play a tone at specified frequency and duration
fn play_tone(frequency: f32, duration_ms: u64) -> io::Result<()> {
    // Try to get an output stream handle and sink
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(result) => result,
        Err(_) => return Ok(()), // Silently fail if audio isn't available
    };
    
    let sink = match Sink::try_new(&stream_handle) {
        Ok(sink) => sink,
        Err(_) => return Ok(()), // Silently fail if sink creation fails
    };

    // Create a source with the PC speaker-like square wave
    let source = rodio::source::SineWave::new(frequency)
        .take_duration(Duration::from_millis(duration_ms))
        .amplify(0.20); // Lower volume to avoid being too loud
    
    // Play the source
    sink.append(source);
    
    // Wait for the sound to finish playing
    thread::sleep(Duration::from_millis(duration_ms));
    
    Ok(())
}

// Classic PC beep (higher pitch, short duration)
pub fn beep() -> io::Result<()> {
    play_tone(PC_BEEP_FREQ, 150)
}

// Error beep (lower tone)
pub fn error_sound() -> io::Result<()> {
    play_tone(PC_ERROR_FREQ, 300)
}

// Multiple beeps with delay - classic PC XT style
pub fn multi_beep(count: u8, delay_ms: u64) -> io::Result<()> {
    for _ in 0..count {
        beep()?;
        thread::sleep(Duration::from_millis(delay_ms));
    }
    Ok(())
}

// Alert sound - higher pitch beeps
pub fn alert_sound() -> io::Result<()> {
    play_tone(PC_ALERT_FREQ, 100)?;
    thread::sleep(Duration::from_millis(70));
    play_tone(PC_ALERT_FREQ, 100)
}

// Success sound - ascending tones
pub fn success_sound() -> io::Result<()> {
    play_tone(PC_BEEP_FREQ, 100)?;
    thread::sleep(Duration::from_millis(50));
    play_tone(PC_SUCCESS_FREQ, 150)
}

// Processing sound - repeated tones
pub fn processing_sound() -> io::Result<()> {
    for i in 0..3 {
        // Slightly increase pitch for each beep
        play_tone(PC_BEEP_FREQ + (i as f32 * 50.0), 100)?;
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

// Sound for terminal/connection flickering - random tones
pub fn flicker_sound() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    // Random frequency between 500-1000 Hz for electrical interference feel
    let freq = 500.0 + (rng.gen::<f32>() * 500.0);
    play_tone(freq, 50)
}

// Sound for when consciousness is fading - descending tones
pub fn fade_sound() -> io::Result<()> {
    // Series of progressively lower tones
    play_tone(1000.0, 200)?;
    thread::sleep(Duration::from_millis(100));
    play_tone(800.0, 200)?;
    thread::sleep(Duration::from_millis(150));
    play_tone(600.0, 250)?;
    thread::sleep(Duration::from_millis(200));
    play_tone(400.0, 300)?;
    Ok(())
}

// Dramatic sound for ending
pub fn ending_sound() -> io::Result<()> {
    // Dramatic arpeggio down
    play_tone(1200.0, 200)?;
    thread::sleep(Duration::from_millis(100));
    play_tone(900.0, 200)?;
    thread::sleep(Duration::from_millis(100));
    play_tone(600.0, 200)?;
    thread::sleep(Duration::from_millis(100));
    play_tone(300.0, 400)?;
    Ok(())
}

// Classic boot-up chime sound
pub fn boot_sound() -> io::Result<()> {
    // Classic PC start sound
    play_tone(800.0, 150)?;
    thread::sleep(Duration::from_millis(50));
    play_tone(1000.0, 150)?;
    thread::sleep(Duration::from_millis(50));
    play_tone(1200.0, 200)?;
    Ok(())
}

// Error/crash sound
pub fn crash_sound() -> io::Result<()> {
    play_tone(800.0, 100)?;
    play_tone(700.0, 100)?;
    play_tone(600.0, 100)?;
    play_tone(500.0, 100)?;
    play_tone(400.0, 400)?;
    Ok(())
}

// A dial-up modem-like sound for "establishing connection"
pub fn connection_sound() -> io::Result<()> {
    // Brief dial-up modem simulation
    for i in 0..5 {
        let freq = 800.0 + (i as f32 * 100.0);
        play_tone(freq, 70)?;
        thread::sleep(Duration::from_millis(30));
    }
    
    // Connection established tone
    thread::sleep(Duration::from_millis(200));
    play_tone(1200.0, 200)?;
    
    Ok(())
}