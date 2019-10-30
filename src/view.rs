use web_view::*;

// use web_view::Color;
// use colored::Colorize;
use super::UI;
// use super::View;
// use super::backend;
// use serde_json::Result;
// use std::time::Duration;
// use std::thread;
// use std::sync::Mutex;
// use std::sync::Arc;
use std::sync::mpsc::{
    Sender,
    // Receiver,
    // channel
    };

// use threadpool::ThreadPool;

pub type S = Sender<String>;
// type R = Receiver<String>;
// pub fn with_build<F>(body:&str,style:&str,js:&str, theme:Color, size:(i32,i32), invok_handler:F ) -> WVResult
pub fn with_build<F>(ui:&UI, invok_handler:F ) -> WVResult
where F: FnMut(&mut web_view::WebView<'_, ()>, &str) -> WVResult
 {
    let default_css  = include_str!("template/default.css");
    let css_boot  = include_str!("template/bootstrap-4/css/bootstrap.min.css");
    let jquery = include_str!("template/jquery.min.js");
    let default_js  = include_str!("template/default.js");
    // css_boot.push_str(&default_css);
    // let default_html = include_str!("template/index.html");
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
    let mut webview = web_view::builder()
        .title("example")
        .content(Content::Html(html))
        .size(size.0, size.1)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(invok_handler)
        .build()?;
    
    // let handle = webview.handle();
    
    // thread::spawn(move || {
    //     let lock_channel = CHANNEL.lock().unwrap();
    //     println!("---> start background [{}]", "ok".green());
    //     loop {
    //         let recv_data =  match lock_channel.rx.recv(){
    //             Ok(a) => a,
    //             _ => {
    //                 println!("---> end background [{}]", "end".red());
    //                 break
    //             }
    //         };

    //         handle
    //             .dispatch(move |webview| {
    //                 let data_shared = recv_data.clone();
    //                 webview.render_with_json(&data_shared);
    //                 Ok(())
    //             })
    //             .unwrap();
            
    //     }
    //     thread::sleep(Duration::from_secs(1));
    // });

    webview.set_color(ui.title_color);
    webview.run()
}





// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// struct ChannelHandler {
//     tx:S,
//     // rx:R,
//     // register:FnOnce(S) + Send +'static,
// }

// impl Default for ChannelHandler {
//     fn default() -> Self{
//         let (tx,rx) = channel::<String>();
//         Self{
//             tx:tx,
//             // rx:rx,
//             // register: nil
//             // register:vec![]
//         }
//     }
// }
// impl ChannelHandler{
//     #[allow(dead_code)]
//     pub fn get_sender(&self) -> S{
//         self.tx.clone()
//     }

// }

// lazy_static!{
//     static ref CHANNEL:Arc<Mutex<ChannelHandler>> = {
//         Arc::new(Mutex::new(
//             ChannelHandler::default()
//         ))
//     };

//     static ref POOL:Arc<Mutex<ThreadPool>> = {
//         Arc::new(Mutex::new(
//             ThreadPool::new(10)
//         ))
//     };
// }

// #[allow(dead_code)]
// pub fn get_sender() -> S{
//     let a = CHANNEL.lock().unwrap();
//     a.get_sender()
// }

// #[allow(dead_code)]
// pub fn job_with<F> (handle:F) 
// where F: FnOnce(S) + Send + 'static
// {
//     let pool = POOL.lock().unwrap();
//     println!("run back [{}]","now".yellow());
//     pool.execute(move||{
//         let tx = get_sender();
//         println!("run back [{}]","ready".yellow());
//         handle(tx);
//     });
//     // pool.join();
// }
