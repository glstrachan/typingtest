use crossterm::terminal::{disable_raw_mode, EnableLineWrap};
use serde_json::Value;
use std::time::Instant;
use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, DisableFocusChange, Event, KeyCode},
    style::{self, Stylize},
    terminal::{self, enable_raw_mode},
    ExecutableCommand, QueueableCommand, Result,
};

fn main() -> Result<()> {
    let mut terms = String::from("");
    urban_terms(10, &mut terms);

    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::Print(terms.clone()))?
        .flush()?;

    enable_raw_mode()?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    let mut i = 0;
    let mut start = Instant::now();
    let mut elapsed;

    while i < terms.len() - 1 {
        match read()? {
            Event::Key(event) => {
                if (event.code == KeyCode::Esc) {
                    break;
                }

                if (event.code == KeyCode::Backspace || event.code == KeyCode::Delete) {
                    i = if i > 0 { i - 1 } else { i };
                    let c = terms.clone().chars().nth(i).unwrap();

                    stdout.execute(cursor::MoveTo(i.try_into().unwrap(), 0))?;
                    stdout.execute(style::Print(c))?;
                    stdout.execute(cursor::MoveTo(i.try_into().unwrap(), 0))?;

                    continue;
                }

                if(i == 0) { start = Instant::now(); }

                stdout.execute(cursor::MoveTo(i.try_into().unwrap(), 0))?;

                let mut c = terms.clone().chars().nth(i).unwrap().bold();

                if (KeyCode::Char(terms.clone().chars().nth(i).unwrap()) != event.code) {
                    c = c.red();
                }

                stdout.execute(style::PrintStyledContent(c))?;
                i += 1;
            }
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }

    elapsed = Instant::now().duration_since(start);

    stdout.execute(cursor::MoveTo(0, 2))?;

    let message = &elapsed.as_secs().to_string();
    stdout.execute(style::Print("Time "))?;
    stdout.execute(style::Print(message))?;
    stdout.execute(style::Print("s "))?;

    disable_raw_mode()?;
    stdout.execute(EnableLineWrap)?;
    stdout.flush()?;

    Ok(())
}

fn urban_terms(size: u32, urban: &mut String) {
    let mut i: u32 = 0;

    while i < size {
        let resp = reqwest::blocking::get("https://api.urbandictionary.com/v0/random")
            .unwrap()
            .text()
            .unwrap();
        let v: Value = serde_json::from_str(&resp).unwrap();
        let terms = &v["list"].as_array().unwrap();

        let mut o = 0;
        'outer: while o < terms.len() && i < size {
            let term = terms[o]["word"].as_str().unwrap();

            for c in term.chars() {
                if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == ' ') {
                    i = if i > 0 { i - 1 } else { i };
                    o += 1;
                    continue 'outer;
                }
            }

            urban.push_str(&term.to_lowercase());
            urban.push_str(" ");

            i += 1;
            o += 1;
        }
    }
}
