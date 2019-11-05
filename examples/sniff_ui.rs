#[macro_use]
extern crate r_ui;

use colored::Colorize;
use r_ui::{
    net::{Req, Res},
    proxy::{how_to_handler_sniff, server_start, PASS_REQ},
};
use std::thread;

fn build_ui() -> r_ui::UI {
    let mut ui = with_html! {
        @html
        (ele!{RAW
            div "hidden" (
                M "req-show" r#"<pre id="show-res"><code id="show-area"></code></pre>"#
            )
        }),
        (r#"
<div class="show-area">
    <pre id="wait-area" contenteditable="true" data-language="json">
        <code id="req" > </code>
    </pre>
    
</div>
<button class="btn btn-suss" id="pass"> Pass it </button>
<button class="btn btn-info" id="tryreq"> try it </button>
       "#.to_string())
        @css
        input#req{
            width:360px;
        }
        button#pass{
            position:fixed;
            bottom:1%;
        }
        button#tryreq{
            position:fixed;
            bottom:1%;
            right:5%;
        }
        pre {
            background-color: white;
            margin:8px;
            border-radius:3px;
        }
        pre#wait-area{
            color:#0a9d20;
        }

        pre#show-res{
            color:black;
        }
        code#show-area{
            color:#f1cb96
            border: 2
        }
        @js

        $("button#pass").click(function(){
             let v = $("code#req").html();
             console.log(v);
             rpc.invoke({
                 tp:"btn",
                 id: "pass",
                 content: v
             });
             $("code#req").html("");
        });

        $("button#tryreq").click(function(){
             let v = $("code#req").html();
             console.log(v);
             rpc.invoke({
                 tp:"btn",
                 id: "try",
                 content: v
             })
        });
        handle_json = function(obj){
             console.log(obj);
             if (obj.tp == "show"){
                 $("code#req").text(obj.content);
             }else if (obj.tp == "try"){
                 $("code#show-area").text(obj.content);
             }

        }
    };
    ui.set_size(720, 600);
    ui.set_theme(226, 229, 231);
    ui
}

fn main() {
    let ui = build_ui();

    // start http proxy
    thread::spawn(move || {
        server_start("localhost:7878");
    });

    r_ui::with_search_extend(&ui, |tx, rx| loop {
        how_to_handler_sniff(|msg| {
            r_ui::rpc_msg_progress("list", "show", &msg, 1, tx.clone());
            let (id, tp, nmsg) = r_ui::rpc_from(rx);
            println!("{} {} {}", id, tp, nmsg);
            if id == "pass" {
                Some(format!("{}{}", PASS_REQ, &nmsg))
            } else {
                if id == "try" {
                    println!("[{}]\n {} ", "test".red(), &nmsg);
                    if let Some(req_pay) = nmsg.to_req() {
                        println!("[{}]\n {} ", "ready".yellow(), &nmsg);
                        let mut res = req_pay.send();
                        let msg = res.to_string();
                        // let msg =format!("<pre ><code>{}</code></pre>", &res.to_string());
                        println!("[{}]\n {}... ", "ok".green(), &msg[..20]);
                        r_ui::rpc_msg_progress("add", "try", &msg, 100, tx.clone());
                        println!("tr {} {} ", id, tp);
                    }
                }
                None
            }
        });
    })
}
