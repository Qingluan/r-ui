#[macro_use]
extern crate r_ui;
use r_ui::action::ListData;
use r_ui::utils::ToFils;
// use r_ui::widgets;
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
    r_ui::with_search_extend(&h, |tx, rx| loop {
        let (id, _tp, msg) = r_ui::rpc_from(rx);

        println!("click id:{}| content:{}", &id, &msg);
        if msg.contains("pro") {
            r_ui::rpc_msg(&id, "pro", &format!("{}", msg.len()), tx.clone());
        }
    });
}
