# r-ui

#### doc
a rust  desktop Gui  based on web_view.

#### mvc

#### install

```rs
[dependencies]
search-ui = {git = "https://gitee.com/dark.H/r-ui.git"}
```

#### todo

[x] finish base ui.
[x] add list template.
[x] action listener
[ ] add preview mode.

#### usage

> a verify simple code
```rs
extern crate search_ui;
fn main() {
    search_ui::with_search(move |_, rx|{
        let (tp, id ,content) = search_ui::rpc_from(rx);
        println!("input text : {}, {}, {}", tp, id , content);
    });
}
```

```bash
cargo run --examples hello
```

![png](./screen.png)


```bash
cargo run --examples action

```

![png](./screen2.png)


```rs
#[macro_use]
extern crate search_ui;
// use search_ui::View;
use search_ui::{
    UI,
    utils::ToFils
};
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
    search_ui::with_search_extend(&h, move |tx, rx|{
        loop{
            let (tp, id ,content) = search_ui::rpc_from(rx);
            println!("rpc handle here : {} {} {} ", &tp, &id ,&content);        
            if content.len()>3{
                let t = content.t().re().unwrap();
                let vv = "./".t().ein(move|f| t.is_match(f));
                search_ui::rpc_to(&id, &tp, &vv, tx.clone());
            }
        }        
    });
    
}
```

#### author

qingluan


