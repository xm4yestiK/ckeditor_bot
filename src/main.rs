use anyhow::{Context, Result};
use colored::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, KeyboardControllable};
use figlet_rs::FIGfont;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use tracing_subscriber;

const POLL_INTERVAL: Duration = Duration::from_millis(50);
const TYPE_DELAY: Duration = Duration::from_millis(100);

fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    if let Err(e) = run() {
        error!("❌ Program crashed: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    display_banner();

    loop {
        let text_to_type = get_user_text().context("Failed to read user input")?;
        info!("Bot ready — focus the target app and press F8 to type. Press F10 to quit.");
        if !event_loop(&text_to_type)? {
            info!("Exiting by user request.");
            break;
        }
    }

    Ok(())
}

fn display_banner() {
    match FIGfont::standard() {
        Ok(font) => {
            if let Some(fig) = font.convert("CKEDITOR BOT") {
                println!("{}", fig.to_string().green().bold());
            } else {
                println!("{}", "CKEDITOR BOT".green().bold());
            }
        }
        Err(e) => {
            eprintln!("{} Failed to load FIGfont: {}", "⚠️".yellow(), e);
            println!("{}", "CKEDITOR BOT".green().bold());
        }
    }

    println!(
        "{}",
        "\n; ================================================================\n\
         ; BYPASS CKEDITOR COPY PASTE BLOCKER, HAPPY HACKING!\n\
         ; ================================================================\n"
        .bright_black()
    );
    println!(
        "{}",
        "\n; ================================================================\n\
         ; CKEDITOR BOT © 2025 m4yestiK\n\
         ; Licensed under the MIT License\n\
         ; https://github.com/xm4yestiK/ckeditor-bot\n\
         ; ================================================================\n"
        .bright_black()
    );
}

fn get_user_text() -> Result<String> {
    print!("{}", "🤖 Yo! Gimme the text u want me to type, then hit Enter: ".cyan().bold());
    io::stdout().flush().context("Failed to flush stdout")?;

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .context("Failed to read line from stdin")?;

    let trimmed = user_input.trim();
    if trimmed.is_empty() {
        Err(anyhow::anyhow!("Text cannot be empty"))
    } else {
        Ok(trimmed.to_owned())
    }
}

fn type_text(text: &str) {
    let mut enigo = Enigo::new();
    warn!("🔥 F8 detected — typing now...");
    thread::sleep(TYPE_DELAY);
    enigo.key_sequence(text);
    info!("✅ Done typing. Returning to prompt...");
}

fn event_loop(text_to_type: &str) -> Result<bool> {
    let device_state = DeviceState::new();
    let mut prev_f8 = false;

    loop {
        let keys = device_state.get_keys();
        debug!("Pressed keys: {:?}", keys);

        if keys.contains(&Keycode::F10) {
            warn!("F10 pressed -> quitting");
            return Ok(false);
        }

        let now_f8 = keys.contains(&Keycode::F8);

        if now_f8 && !prev_f8 {
            let res = std::panic::catch_unwind(|| {
                type_text(text_to_type);
            });
            if let Err(e) = res {
                error!("Panic while typing: {:?}", e);
            }

            loop {
                let keys_after = device_state.get_keys();
                if !keys_after.contains(&Keycode::F8) {
                    break;
                }
                thread::sleep(POLL_INTERVAL);
            }

            return Ok(true);
        }

        prev_f8 = now_f8;
        thread::sleep(POLL_INTERVAL);
    }
}
