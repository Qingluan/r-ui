extern crate search_ui;

use search_ui::utils::{ToFils};
use colored::Colorize;

fn  main() {
    "~/Documents".as_file().with(|f| ".png$".re().is_match(f),&mut |f|{
        println!("found png: {}", f.name().green())
    });
}