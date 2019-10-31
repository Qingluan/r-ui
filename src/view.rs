use web_view::*;
use colored::Colorize;
use super::{
    UI,
    View
};
use std::sync::{
    Mutex,
    mpsc::{
        Sender,
        Receiver,
        channel,
    }
};


pub const MSG_SEP:&str =  " -|- ";
pub type R = Receiver<String>;
pub type S = Sender<String>;

lazy_static!{
    static ref POOL:Mutex<threadpool::ThreadPool> = {
        Mutex::new(
            threadpool::ThreadPool::new(10)
        )
    };
}


pub fn with_build<F>(ui:&UI, invok_handler:F ) -> WVResult
where F: FnOnce(S, &R) + Send +'static
 {
    let default_css  = include_str!("template/default.css");
    let css_boot  = include_str!("template/bootstrap-4/css/bootstrap.min.css");
    let jquery = include_str!("template/jquery.min.js");
    let default_js  = include_str!("template/default.js");
    let theme = ui.theme;
    let body = ui.html.clone();
    let js = ui.js.clone();
    let style = ui.css.clone();
    let size = ui.size;
    let mut theme_css = format!(r#"
	background-color: rgb{theme};
    "#, theme=&format!("({},{},{})",theme.r,theme.g,theme.b));
    theme_css = theme_css.replace("++", "{");
    theme_css = theme_css.replace("--", "}");
    let html = format!(r#"
<!doctype html>
	<html style = "{theme}">
    <head>
        <style type="text/css" >
        {default_css}
        </style>
        <style type="text/css" >{css}</style>
    </head>
    <body style="{theme}">
        {body}
        <script type="text/javascript">{jquery}</script>
        <script type="text/javascript">{scripts}</script>
    </body>
</html>
"#, body=body,jquery=jquery,css=&style, default_css=&format!("{}\n{}",css_boot , default_css), theme=theme_css, scripts=&format!("{}\n{}",default_js , &js));
    
    
    let pool = POOL.lock().expect("lock thread failed");
    let (stx, srx) = channel::<String>();
    let (tx, rx) = channel::<String>();

    let mut webview= web_view::builder()
        .title("example")
        .content(Content::Html(html))
        .size(size.0, size.1)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_,arg|{
            let tx = tx.clone();
            let value:serde_json::Value = match serde_json::from_str(arg){
            Ok(a) => a,
            Err(e) => {
                    log::info!("err {}",e.to_string().red());
                    serde_json::from_str(r#"{}"#).expect("no such json")
                }
            };
            if let Some(tp) = value.get("tp"){
                if let Some(from_id) = value.get("id"){
                    if let Some(content) = value.get("content"){
                        
                        let id = from_id.as_str().unwrap().to_string();
                        let msg = content.as_str().unwrap().to_string();
                        let tp = tp.as_str().unwrap().to_string();
                        log::info!("recv |{}| : {}", from_id.as_str().unwrap().green(),content.as_str().unwrap().yellow());
                        tx.send(vec![id, tp, msg].join(MSG_SEP)).expect("send failed");
                    } 
                }
            }else{
                log::info!(" {:?}", value);
            }
            Ok(())
        })
        .build().unwrap();
    
    let handle = webview.handle();
    pool.execute(move||{
        let tx = stx.clone();
        println!("[{}] <--- start job ", "ok".green());
        invok_handler(tx, &rx);
    });
    pool.execute(move || {
        // println!("lock 30 get_job {}", "ok".green());
        println!("---> start render [{}]", "ok".green());    
        loop {
            let msg = srx.recv().expect("get error");
            handle.dispatch(move|web|{
                println!("---> patch background [{}]", "ok".green());
                web.render_with_json(&msg);
                Ok(())
            }).unwrap();
        }
    //     thread::sleep(Duration::from_secs(1));
    });
    webview.set_color(ui.title_color);
    webview.run()
}


