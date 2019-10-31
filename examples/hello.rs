extern crate search_ui;
fn main() {
    search_ui::with_search(move |_, rx|{
        let (tp, id ,content) = search_ui::rpc_from(rx);
        println!("input text : {}, {}, {}", tp, id , content);
    });
}