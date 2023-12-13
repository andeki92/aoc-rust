//! [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[94m";
pub const WHITE: &str = "\x1b[97m";
pub const HOME: &str = "\x1b[H";
pub const CLEAR: &str = "\x1b[J";
