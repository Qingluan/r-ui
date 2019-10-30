#[macro_use]
extern crate search_ui;
use search_ui::View;
use search_ui::UI;


fn main(){
    search_ui::log_init();
    let mut h = with_html!{@li
        (P "hello world"),
        (P "hello world"),
        (P "hello world")
        @css
        ul {
            height: 80%;
        }
        ul>li{
            border-radius:8px;
        }
    };
    let h2 = with_html!{@div
        (I "t")
        @css
        div#main{
            margin-top:10px;
        }
        button#new{
            position: absolute;
            bottom: 10%;
        }
        input{
            position: fixed;
            bottom: 1%;
        }
        @js
        handle_json = function(obj){
            list_add(obj.content);
        }
    };

    h.chain(&h2);
    search_ui::with_search_extend(&mut |tp, id, content, webview|{
        println!("rpc handle here : {} {} {} ", tp, id ,content);
        // let m:&str = content
        webview.render(id, content);
        let _ = webview.set_title(&format!("input text : {}, {}", tp, id ));
    }, &h);
    
}