use std::io::{self, BufRead, Write};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use colored::*;
use enigo::{Enigo, KeyboardControllable, Key};
use figlet_rs::FIGfont;
use rand::Rng;
use rdev::{listen, Event, EventType, Key as RKey};
use tracing::{info, warn};
use tracing_subscriber;

const PRE_TYPE_DELAY: Duration = Duration::from_secs(1);
const COUNTDOWN_STEP: Duration = Duration::from_millis(100);
const MAX_WORDS: usize = 1000;

fn main() {
    tracing_subscriber::fmt().with_writer(io::stderr).with_target(false).compact().init();
    if let Err(e) = run() {
        eprintln!("❌ Program crashed: {:?}", e);
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    display_banner();
    loop {
        let text_to_type = get_user_text()?;
        let word_count = count_words(&text_to_type);
        info!("Captured {} word(s). Press F8 to type, Esc to quit.", word_count);
        event_loop(text_to_type.clone())?;
    }
}

fn display_banner() {
    if let Ok(font) = FIGfont::standard() {
        if let Some(fig) = font.convert("CKEDITOR BOT") {
            println!("{}", fig.to_string().green().bold());
        } else {
            println!("{}", "CKEDITOR BOT".green().bold());
        }
    } else {
        println!("{}", "CKEDITOR BOT".green().bold());
    }
    println!("{}", "\n; ================================================================\n; CKEDITOR BOT © 2025 m4yestiK\n; Licensed under the MIT License\n; https://github.com/xm4yestiK/ckeditor_bot\n; ================================================================\n".bright_black());
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn get_user_text() -> anyhow::Result<String> {
    eprintln!("{}", format!("🤖 Enter the text you want me to type (max {} words).", MAX_WORDS).cyan().bold());
    eprintln!("{}", "(Type multiple lines. Type !!END on a new line to finish)".bright_black());
    io::stdout().flush()?;
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    let mut total_words = 0;
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.trim() == "!!END" {
            eprintln!("⚠️ Input ended by !!END");
            break;
        }
        let line_for_counting = line.trim();
        let word_count = line_for_counting.split_whitespace().count();
        if word_count == 0 {
            lines.push(line);
            continue;
        }
        if total_words + word_count > MAX_WORDS {
            let remaining = MAX_WORDS - total_words;
            let truncated_line = line_for_counting.split_whitespace().take(remaining).collect::<Vec<&str>>().join(" ");
            lines.push(truncated_line);
            eprintln!("⚠️ Reached max word limit, truncating input.");
            break;
        } else {
            lines.push(line);
            total_words += word_count;
        }
    }
    if lines.is_empty() && total_words == 0 {
        return Err(anyhow::anyhow!("Text cannot be empty"));
    }
    Ok(lines.join("\n"))
}

fn type_text(text: &str) {
    if PRE_TYPE_DELAY.as_secs() > 0 {
        let start = Instant::now();
        let total_ms = PRE_TYPE_DELAY.as_millis() as u64;
        let mut elapsed = 0u64;
        while elapsed < total_ms {
            let remaining = total_ms.saturating_sub(elapsed);
            let secs = remaining / 1000;
            let ms = remaining % 1000;
            print!("\r{}", format!("Typing in {}.{:03}s...", secs, ms).yellow());
            let _ = io::stdout().flush();
            thread::sleep(COUNTDOWN_STEP);
            elapsed = start.elapsed().as_millis() as u64;
        }
        println!();
    }
    let mut enigo = Enigo::new();
    let mut rng = rand::rng();
    warn!("🔥 F8 detected — typing now...");
    thread::sleep(Duration::from_millis(200));
    for ch in text.chars() {
        match ch {
            '\n' => {
                enigo.key_click(Key::Return);
                thread::sleep(Duration::from_millis(2));
            }
            ' ' => {
                enigo.key_click(Key::Space);
                thread::sleep(Duration::from_millis(1));
            }
            _ => {
                enigo.key_sequence(&ch.to_string());
                let delay = rng.random_range(1..2);
                thread::sleep(Duration::from_millis(delay));
            }
        }
    }
    info!("✅ Done typing. Returning to prompt...");
}

fn event_loop(text_to_type: String) -> anyhow::Result<()> {
    let is_typing = Arc::new(Mutex::new(false));
    let typing_flag = is_typing.clone();
    let _ = listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            if key == RKey::F8 {
                let mut flag = typing_flag.lock().unwrap();
                if !*flag {
                    *flag = true;
                    let text = text_to_type.clone();
                    let typing_flag_clone = typing_flag.clone();
                    thread::spawn(move || {
                        type_text(&text);
                        *typing_flag_clone.lock().unwrap() = false;
                    });
                }
            } else if key == RKey::Escape {
                std::process::exit(0);
            }
        }
    });
    Ok(())
}

