extern crate search_ui;
fn main() {
    search_ui::with_search(|tp, id, content|{
        println!("input text : {}, {}, {}", tp, id , content);
    });
}