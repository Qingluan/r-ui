#[macro_use]
extern crate search_ui;
// use search_ui::UI;
use search_ui::utils::ToFils;
use search_ui::action::ListData;
fn main() {
    let mut data = ListData::default();
    data.update( &"./".t().ein(|f| f.ends_with("png")));
    let h = with_html!{
        @html (data.to_html())
        @css 
        ul {
            height: 80%;
        }
    };
    search_ui::with_search_extend(&h, |tx,rx|loop{
        let (id, tp, msg) = search_ui::rpc_from(rx);

        println!("click id:{}| content:{}", &id, &msg);

    });
}