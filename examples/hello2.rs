#[macro_use]
extern crate search_ui;
use search_ui::UI;


fn main(){
    search_ui::log_init();
    let mut h = with_html!{
        (B "check it"),
        (B "new")
        @css
        button#check-it{
            position: absolute;
            bottom: 1%;
        }
        button#new{
            position: absolute;
            bottom: 10%;
        }
        input{
            position: absolute;
            top: 1%;
        }
        @js
        document.getElementById("check-it").addEventListener("click", function(){
            alert("hello 2")
        });
    };
    h.add_js("console.log('hllo');");
    println!("html {}",&h.to_string());
    search_ui::with_search_extend(&mut |tp, id, content, webview|{
        println!("rpc handle here : {} {} {} ", tp, id ,content);
        let _ = webview.set_title(&format!("input text : {}, {}", tp, id ));
    }, &h);
    
}