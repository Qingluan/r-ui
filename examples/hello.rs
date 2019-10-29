extern crate search_ui;
fn main() {
    search_ui::with_search(&mut |tp, id, content, web_view|{
        println!("input text : {}, {}, {}", tp, id , content);
        let _ = web_view.set_title(&format!("input text : {}, {}", tp, id ));
    });
}