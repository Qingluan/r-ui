extern crate r_ui;
fn main() {
    r_ui::with_search(move |_, rx| {
        let (tp, id, content) = r_ui::rpc_from(rx);
        println!("input text : {}, {}, {}", tp, id, content);
    });
}
