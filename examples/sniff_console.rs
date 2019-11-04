extern crate search_ui;

use search_ui::proxy::{
    server_start,
    PASS_REQ,
    how_to_handler_sniff
};
use std::thread;
use std::io::{stdin,stdout,Write};
use colored::Colorize;

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
    // print!("\r");
    s
}

fn main() {
    thread::spawn(move ||{
        server_start("localhost:7878");
    });
    
    loop {
        how_to_handler_sniff(|msg|{
            println!("{}", msg.yellow());
            let f = input();
            if f == "ok" || f.len() ==0{
                format!("{}{}",PASS_REQ, msg)
            }else{
                format!("{}{}","no", msg)
            }
        })
        
    }

}