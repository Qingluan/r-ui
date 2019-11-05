#[macro_use]
extern crate r_ui;

use colored::Colorize;

fn main() {
    let h = ele! {RAW
        div "hello" (
            // "hello"
            div "sub" ( "world2".yellow() )
            ul "list" (
                li "l1" ("one".blue())
                li "l2" ("one".blue())
                li "l2" ("one".blue())
            )
        )
        p "sub" ("world3 ".green() )
        M "hello-modal"
    };
    println!("{}", &h);
}
