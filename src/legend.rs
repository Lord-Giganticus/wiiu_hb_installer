use crate::consts::*;
use console::style;

pub fn print_legend() {
    let green = style("Green: Safe to use.").fg(COLORS["Green"]);
    let red = style("Red: Either unsafe, or not possible to use.")
    .fg(COLORS["Red"]);
    let blue = style("Blue: The community recommended method.")
    .fg(COLORS["Blue"]);
    println!("Legend for colors:\n{}\n{}\n{}", green, red, blue);
}

pub fn print_options() {
    let tira = style(NAMES[&0]).fg(COLORS["Blue"]);
    let idx = style(NAMES[&1]).fg(COLORS["Green"]);
    let clr = match eshop_down() {
        true => COLORS["Red"],
        false => COLORS["Green"]
    };
    let hax = style(NAMES[&2]).fg(clr);
    let cbhc = style(NAMES[&3]).fg(clr);
    println!("1. {}\n2. {}\n3. {}\n4. {}", tira, idx, hax, cbhc);
}