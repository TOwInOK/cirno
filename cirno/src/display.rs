use cirno_store::PHRASE;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::claping::ShowType;
/// Print framed phrase pack
pub fn print_framed_phrase(
    phrase: PHRASE,
    width: usize,
    show: &ShowType,
) {
    let braces = "||".cyan();
    let border = "=";

    let space = format!("{braces}{}{braces}", " ".repeat(width - 4));
    let border = border.repeat(width).cyan();
    print!(
        "{}",
        match show {
            ShowType::All => {
                let f = phrase.0.italic().bold().blue();
                let s = phrase.1.bold().italic().purple();

                let f_len = f.width();
                let s_len = s.width();

                let f_padding = (width - f_len - 4) / 2;
                let s_padding = (width - s_len - 4) / 2;

                let f_right = width - f_len - f_padding - 4;
                let s_right = width - s_len - s_padding - 4;

                let f = format!(
                    "{braces}{}{f}{}{braces}",
                    " ".repeat(f_padding),
                    " ".repeat(f_right)
                );
                let s = format!(
                    "{braces}{}{s}{}{braces}",
                    " ".repeat(s_padding),
                    " ".repeat(s_right)
                );
                format!("{border}\n{space}\n{f}\n{space}\n{s}\n{space}\n{border}\n")
            }
            ShowType::Japan => {
                let f = phrase.0.italic().bold().blue();

                let f_len = f.width();

                let f_padding = (width - f_len - 4) / 2;

                let f_right = width - f_len - f_padding - 4;

                let f = format!(
                    "{braces}{}{f}{}{braces}",
                    " ".repeat(f_padding),
                    " ".repeat(f_right)
                );

                format!("{border}\n{space}\n{f}\n{space}\n{border}\n")
            }
            ShowType::Russina => {
                let s = phrase.1.bold().italic().purple();

                let s_len = s.width();

                let s_padding = (width - s_len - 4) / 2;

                let s_right = width - s_len - s_padding - 4;

                let s = format!(
                    "{braces}{}{s}{}{braces}",
                    " ".repeat(s_padding),
                    " ".repeat(s_right)
                );
                format!("{border}\n{space}\n{s}\n{space}\n{border}\n")
            }
        }
    )
}

/// Print minimized phrase pack
pub fn print_minimized_phrase(
    phrase: PHRASE,
    show: &ShowType,
) {
    match show {
        ShowType::All => {
            let f = phrase.0.italic().bold().blue();
            let s = phrase.1.bold().italic().purple();
            print!("{f}\n{s}");
        }
        ShowType::Japan => {
            let f = phrase.0.italic().bold().blue();
            print!("{f}");
        }
        ShowType::Russina => {
            let s = phrase.1.bold().italic().purple();
            print!("{s}");
        }
    }
}
