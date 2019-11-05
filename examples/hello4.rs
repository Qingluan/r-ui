#[macro_use]
extern crate search_ui;
use search_ui::action::ListData;
use search_ui::utils::ToFils;
use search_ui::widgets;
fn main() {
    let mut data = ListData::default();
    data.update(&"./".t().ein(|f| f.ends_with("png")));

    let h = with_html! {
        @html
        (data.to_html()),
        (ele!{ "bar"
            (P "search"),
            (I "some" )
            |
            r#""#})
        @css
        ul {
            height: 80%;
        }
        @js
        handle_json = function(obj){
            if (obj.tp == "pro"){
                rpc.progress(obj);
            }else{
                list_add_all(obj.content);
            }

        }
    };
    search_ui::with_search_extend(&h, |tx, rx| loop {
        let (id, tp, msg) = search_ui::rpc_from(rx);

        println!("click id:{}| content:{}", &id, &msg);
        if msg.contains("pro") {
            search_ui::rpc_msg(&id, "pro", &format!("{}", msg.len()), tx.clone());
        }
    });
}
