use std::env;
use colored::{Color, ColoredString, Colorize};

fn paint_red(text: &str, true_color: bool) -> ColoredString {
    if (true_color) {
        text.bold().truecolor(184, 20, 56)
    } else {
        text.bold().color(Color::Red)
    }
}

fn paint_green(text: &str, true_color: bool) -> ColoredString {
    if true_color {
        text.truecolor(134, 186, 144)
    } else {
        text.color(Color::Green)
    }
}

fn paint_blue(text: &str, true_color: bool) -> ColoredString {
    if true_color {
        text.bold().truecolor(82, 139, 183)
    } else {
        text.bold().color(Color::Blue)
    }
}


/// Prints welcome message
#[rustfmt::skip]
pub fn welcome() {

    let mut true_color = true;

    match env::var("COLORTERM") {
        Ok(val) => if val != "24bit" && val != "truecolor" {
            true_color = false;
        },
        Err(_) => true_color = false,
    }

    println!("{}", paint_red(r#"           _                 _    "#, true_color));
    println!("{}", paint_red(r#"  __ _  __| |_ __ __ _ _ __ | |_  "#, true_color));
    println!("{}", paint_red(r#" / _` |/ _` | '__/ _` | '_ \| __| "#, true_color));
    println!("{}", paint_red(r#"| (_| | (_| | | | (_| | | | | |_  "#, true_color));
    println!("{}", paint_red(r#" \__, |\__,_|_|  \__,_|_| |_|\__| "#, true_color));
    println!("{}", paint_red(r#"    |_|                           "#, true_color));
    println!();


    println!(
        "{} {}",
        paint_green("Version:", true_color),
        paint_blue(env!("CARGO_PKG_VERSION"), true_color),
    );


    println!(
        "{} {}",
        paint_green("Access web UI at", true_color),
        paint_blue("test", true_color).underline()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome() {
        welcome();
    }
}
