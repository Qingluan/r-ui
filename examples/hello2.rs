#[macro_use]
extern crate search_ui;
use search_ui::View;
use search_ui::UI;
use search_ui::utils::ToFils;
use threadpool;

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
            list_add_all(obj.content);
        }
    };

    h.chain(&h2);
    let pool = threadpool::ThreadPool::new(5);
    search_ui::with_search_extend(&mut |tp, id, content, webview|{
        println!("rpc handle here : {} {} {} ", tp, id ,content);
        
        if content.len() >= 3{
            let t = content.re();
            let handler = webview.handle();
            pool.execute(move||{
                handler.dispatch(move |webview| {
                // let tx = search_ui::view::get_sender();
                println!("-> job {} ", "try");
                
                let a = "./".as_file().ein(move |f| t.is_match(f));
                webview.render_with_list(&a);
                println!("-> job [{}] ", a.len());
                Ok(())
                }).unwrap();
            });    
        }
        
        
    }, &h);
    
}