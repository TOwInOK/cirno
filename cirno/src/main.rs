//! # Cirno
//! She will always support you.
//! - This app to display random [PHRASE]
//!
//! ## App contains arguments:
//!
//! - `--compact` - return compact display variant
//! - `--only_japan` - return only japan string
//! - `--only_russian` - return only russian string
//!
//! ## Display output example
//! - '=' должен распространяться на всю ширину экрана
//! - "||" находится по краям
//!
//! ```
//! ==================================
//! ||                              ||
//! ||          諦めないで             ||
//! ||                              ||
//! ||         Не сдавайся          ||
//! ||                              ||
//! ==================================
//! ```
//!
//! ## Compact display output example
//!
//!```
//! 大丈夫です
//! Всё будет хорошо
//! ```

use cirno_store::get_random_phrase;
use clap::Parser;

use crate::{
    claping::Cli,
    display::{print_framed_phrase, print_minimized_phrase},
    localisation::MESSAGE_TTY_NOT_FOUND,
};

pub(crate) mod claping;
mod display;
mod localisation;

fn main() {
    let args = Cli::parse();

    if let Some((w, _)) = terminal_size::terminal_size() {
        let phrase = get_random_phrase();
        match args.display {
            claping::DisplayType::Framed => print_framed_phrase(phrase, w.0 as usize, &args.show),
            claping::DisplayType::Compact => print_minimized_phrase(phrase, &args.show),
        }
    } else {
        println!("{}", MESSAGE_TTY_NOT_FOUND);
    }
}
