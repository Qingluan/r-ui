extern crate r_ui;

use colored::Colorize;
use r_ui::utils::ToFils;

fn main() {
    "~/Documents"
        .t()
        .with(|f| ".png$".re().is_match(f), &mut |f| {
            println!("found png: {}", f.name().green())
        });
}
