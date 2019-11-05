extern crate search_ui;

use colored::Colorize;
use search_ui::utils::ToFils;

fn main() {
    "~/Documents"
        .t()
        .with(|f| ".png$".re().is_match(f), &mut |f| {
            println!("found png: {}", f.name().green())
        });
}
