#[macro_use]
extern crate search_ui;
use search_ui::UI;


fn main(){

    let mut h = with_html!{
        (B "check it")
        @css
        button#check-it{
            position: absolute;
            bottom: 1%;
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
    search_ui::with_search_extend(|tp, id, content|{
        println!("rpc handle here : {} {} {} ", tp, id ,content);
    }, &h);
    
}