extern crate search_ui;

use search_ui::proxy::{
    server_start,
    brd_once
};
use std::thread;
use std::io::{stdin,stdout,Write};
    
fn input() -> String{
    let mut s=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

fn main() {
    thread::spawn(move ||{
        server_start();
    });
    
    loop {
        let s = input();
        brd_once(&s);
    }

}