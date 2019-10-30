#[macro_use]
extern crate search_ui;
use search_ui::UI;


fn main(){
    let mut h = with_html!{@li
        (P "hello world"),
        (P "hello world"),
        (P "hello world")
        @css
        ul>li{
            margin:5px;
            border-radius:8px;
        }
    };
    let h2 = with_html!{@div
        (I "t")
        @css
        div#main{
            margin-top:10px;
        }

        input{
            position: absolute;
            bottom: 1%;
        }
    };
    h.chain(&h2);
    search_ui::with_search_extend(|tp, id, content|{
        println!("rpc handle here : {} {} {} ", tp, id ,content);
    }, &h);
    
}