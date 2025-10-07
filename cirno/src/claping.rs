use std::fmt::Display;

use clap::{Parser, ValueEnum, command};

#[derive(Parser)]
#[command(name = "cirno")]
#[command(about = "She will always support you.", long_about = None)]
pub struct Cli {
    #[arg(long, short, default_value_t = ShowType::All)]
    /// Choose that strokes to show
    pub show: ShowType,
    #[arg(long, short, default_value_t = DisplayType::Framed)]
    /// Choose output display type
    pub display: DisplayType,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]

pub enum ShowType {
    #[default]
    All,
    Japan,
    Russina,
}

impl Display for ShowType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ShowType::All => write!(f, "all"),
            ShowType::Japan => write!(f, "japan"),
            ShowType::Russina => write!(f, "russina"),
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]

pub enum DisplayType {
    #[default]
    Framed,
    Compact,
}

impl Display for DisplayType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            DisplayType::Framed => write!(f, "framed"),
            DisplayType::Compact => write!(f, "compact"),
        }
    }
}
