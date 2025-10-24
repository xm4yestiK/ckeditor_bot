use anyhow::{Context, Result};
use colored::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, KeyboardControllable};
use figlet_rs::FIGfont;
use std::io::{self, BufRead, Write};
use std::process;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};
use tracing_subscriber;

const POLL_INTERVAL: Duration = Duration::from_millis(50);
const PRE_TYPE_DELAY: Duration = Duration::from_secs(3);
const COUNTDOWN_STEP: Duration = Duration::from_millis(200);
const MAX_WORDS: usize = 1000;
const BASE_TYPE_DELAY: Duration = Duration::from_millis(5);
const MAX_TYPE_DELAY: Duration = Duration::from_millis(40);

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_target(false)
        .compact()
        .init();

    if let Err(e) = run() {
        eprintln!("❌ Program crashed: {:?}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    display_banner();
    loop {
        let text_to_type = get_user_text().context("Failed to read user input")?;
        let word_count = count_words(&text_to_type);
        info!(
            "Captured {} word(s). Focus the target app and press F8 to type. Press F10 to quit.",
            word_count
        );
        if !event_loop(&text_to_type)? {
            process::exit(0);
        }
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

fn get_user_text() -> Result<String> {
    eprintln!(
        "{}",
        format!(
            "🤖 Enter the text you want me to type (max {} words).",
            MAX_WORDS
        )
        .cyan()
        .bold()
    );
    eprintln!(
        "{}",
        "(Type multiple lines. Type !!END on a new line to finish)"
            .bright_black()
    );
    io::stdout().flush().context("Failed to flush stdout")?;

    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    let mut total_words = 0;

    for line_result in stdin.lock().lines() {
        let line = line_result.context("Failed to read line from stdin")?;

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
            let truncated_line = line_for_counting
                .split_whitespace()
                .take(remaining)
                .collect::<Vec<&str>>()
                .join(" ");
            lines.push(truncated_line);
            total_words += remaining;
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
            print!(
                "{}",
                format!(
                    "\rTyping in {}.{:03}s... (switch to target window now)",
                    secs, ms
                )
                .yellow()
            );
            let _ = io::stdout().flush();
            thread::sleep(COUNTDOWN_STEP);
            elapsed = start.elapsed().as_millis() as u64;
        }
        println!();
    }

    let delay = calculate_delay(text.len());
    let mut enigo = Enigo::new();
    warn!("🔥 F8 detected — typing now...");

    for ch in text.chars() {
        if ch == '\n' {
            enigo.key_click(enigo::Key::Return);
        } else {
            enigo.key_click(enigo::Key::Layout(ch));
        }
        thread::sleep(delay);
    }

    info!("✅ Done typing. Returning to prompt...");
}

fn calculate_delay(text_len: usize) -> Duration {
    let factor = (text_len as f64 / (MAX_WORDS * 6) as f64).min(1.0);
    let delay_ms = BASE_TYPE_DELAY.as_millis() as f64
        + (MAX_TYPE_DELAY.as_millis() as f64 - BASE_TYPE_DELAY.as_millis() as f64) * factor;
    Duration::from_millis(delay_ms as u64)
}

fn event_loop(text_to_type: &str) -> Result<bool> {
    let device_state = DeviceState::new();
    let mut prev_f8_pressed = false;

    loop {
        let keys = device_state.get_keys();
        debug!("Pressed keys: {:?}", keys);

        if keys.contains(&Keycode::F10) {
            warn!("F10 pressed -> quitting");
            return Ok(false);
        }

        let now_f8_pressed = keys.contains(&Keycode::F8);

        if now_f8_pressed && !prev_f8_pressed {
            type_text(text_to_type);

            while device_state.get_keys().contains(&Keycode::F8) {
                thread::sleep(POLL_INTERVAL);
            }

            return Ok(true);
        }

        prev_f8_pressed = now_f8_pressed;
        thread::sleep(POLL_INTERVAL);
    }
}