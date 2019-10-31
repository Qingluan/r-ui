extern crate search_ui;

use search_ui::utils::{ToFils};
use colored::Colorize;

fn  main() {
    "~/Documents".t().with(|f| ".png$".re().is_match(f),&mut |f|{
        println!("found png: {}", f.name().green())
    });
}