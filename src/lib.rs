extern crate web_view;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate threadpool;

extern crate base64;
extern crate futures;
extern crate gotham;
// #[macro_use]
extern crate gotham_derive;
// extern crate hyper;
extern crate mime;

extern crate crossbeam;

// #[macro_use]
extern crate crossbeam_channel;

// use serde_json;
// use log;
use chrono::Local;
use colog;
use std::io::Write;

// use colored::Colorize;

// use serde_json::Value;
// use serde::{Deserialize, Serialize};
// mod backend;
pub mod view;
#[macro_use]
pub mod widgets;
pub mod action;
pub mod net;
pub mod proxy;
pub mod utils;

#[allow(unused_imports)]
// use view::SP2;

pub trait SP2 {
    fn split_twice(&self, sep: &str) -> (String, String, String);
    fn split_once(&self, sep: &str) -> (String, String);
}

impl SP2 for String {
    fn split_twice(&self, sep: &str) -> (String, String, String) {
        let mut f = self.split(sep);
        (
            f.next().unwrap().to_string(),
            f.next().unwrap().to_string(),
            f.next().unwrap().to_string(),
        )
    }
    fn split_once(&self, sep: &str) -> (String, String) {
        let mut f = self.split(sep);
        (f.next().unwrap().to_string(), f.next().unwrap().to_string())
    }
}

impl<'a> SP2 for &'a str {
    fn split_twice(&self, sep: &str) -> (String, String, String) {
        let mut f = self.split(sep);
        (
            f.next().unwrap().to_string(),
            f.next().unwrap().to_string(),
            f.next().unwrap().to_string(),
        )
    }
    fn split_once(&self, sep: &str) -> (String, String) {
        let mut f = self.split(sep);
        (f.next().unwrap().to_string(), f.next().unwrap().to_string())
    }
}

pub struct UI {
    html: String,
    css: String,
    js: String,
    theme: web_view::Color,
    size: (i32, i32),
    title_color: web_view::Color,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            html: r#""#.to_string(),
            css: r#""#.to_string(),
            size: (360, 600),
            js: "".to_string(),
            theme: web_view::Color::from((5, 44, 92)),
            title_color: web_view::Color::from((255, 255, 255)),
        }
    }
}

impl UI {
    #[allow(unused)]
    fn new(html: &str) -> Self {
        // let h = html_file.read
        Self {
            html: html.to_string(),
            ..Self::default()
        }
    }

    #[allow(dead_code)]
    pub fn set_size(&mut self, w: i32, h: i32) {
        self.size = (w, h);
    }

    #[allow(dead_code)]
    pub fn set_theme(&mut self, r: u8, g: u8, b: u8) {
        self.theme = web_view::Color::from((r, g, b));
    }

    #[allow(dead_code)]
    pub fn add_html(&mut self, new: &str) {
        self.html.push_str("\n");
        self.html.push_str(new);
    }
    #[allow(dead_code)]
    pub fn add_css(&mut self, new: &str) {
        self.css.push_str("\n");
        self.css.push_str(new);
    }
    #[allow(dead_code)]
    pub fn add_js(&mut self, new: &str) {
        self.js.push_str("\n");
        self.js.push_str(new);
    }

    pub fn chain(&mut self, other_ui: &UI) {
        self.add_html(&other_ui.html);
        self.add_css(&other_ui.css);
        self.add_js(&other_ui.js);
    }

    pub fn to_string(&self) -> String {
        let mut m = String::from(format!(
            r#"<style type="text/csstyle">{}</style>"#,
            &self.css
        ));
        m.push_str(&self.html);
        m.push_str(&format!(
            r#"<script type="text/javascript">{}</script>"#,
            &self.js
        ));
        m
    }
}

#[allow(dead_code)]
fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

#[allow(dead_code)]
fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

#[macro_export]
macro_rules! with_html {
    (@e $ui:ident ($tag:expr)) => {
        {
            let html:String = $tag;
            $ui.add_html(&html);
        }
    };
    (@e $ui:ident ($tag:tt $id:tt)) => {
        {
            let html = ele!( ($tag $id) );
            $ui.add_html(&html);
        }
    };
    (@file $base:tt @css $($css:tt)*  @js $($js:tt)* ) => {
        {
            use std::io::Read;
            use std::fs;
            let mut buf = String::new();
            let mut f = fs::File::open($base).unwrap();
            f.read_to_alll(&mut buf);
            let mut csss = String::new();
            let mut jss = String::new();
            $(
                csss.push_str(&inline_style(&include_str!($css)));

            )*
            $(
                jss.push_str(&inline_script(&include_str!($js)));
            )*
            buf = buf.replace("</head>", format!("{}\n</head>", &csss));
            buf = buf.replace("</body>", format!("{}\n</body>", &jss));
            buf
        }
    };
    (@html $(($tag:tt $($id:tt)* )),* @css $($style:tt)* ) =>{
        {
            use r_ui::UI;
            let mut ui = UI::default();
            $(
                with_html!{@e ui
                    ($tag $($id)* )
                };
            )*

            // ui.add_html(&h);
            let css = stringify!($($style)*);
            if css.contains("@ js"){
                let fs:Vec<&str>  = css.split("@ js").collect();
                let ncss = fs[0].to_string().replace(" ", "");
                let js = fs.last().unwrap();
                ui.add_css(&ncss);
                ui.add_js(&js);
            }else{
                let ncss = css.replace(" ","");
                ui.add_css(&ncss);
            }
            ui
        }
    };

    (@html $(($expr:expr)),* @css $($style:tt)* ) =>{
        {
            use r_ui::UI;
            let mut ui = UI::default();
            $(
                // let html:String = $expr;
                // ui.add_html(&html);
                with_html!(@e ui ($expr));
            )*
            let css = stringify!($($style)*);
            if css.contains("@ js"){
                let fs:Vec<&str>  = css.split("@ js").collect();
                let ncss = fs[0].to_string().replace(" ", "");
                let js = fs.last().unwrap();
                ui.add_css(&ncss);
                ui.add_js(&js);
            }else{
                let ncss = css.replace(" ","");
                ui.add_css(&ncss);
            }
            ui
        }
    };
    (@li $(( $e:ident $sid:tt )),*  @css $($style:tt)* ) => {
        {
            use r_ui::UI;
            let mut ui = UI::default();
            let h = ele!(L "list-container" $( ($e $sid  ) ),*  );
            ui.add_html(&h);
            let css = stringify!($($style)*);
            if css.contains("@ js"){
                let fs:Vec<&str>  = css.split("@ js").collect();
                let ncss = fs[0].to_string().replace(" ", "");
                let js = fs.last().unwrap();
                ui.add_css(&ncss);
                ui.add_js(&js);
            }else{
                let ncss = css.replace(" ","");
                ui.add_css(&ncss);
            }


            ui
        }
    };
    (@div $( ($ele:ident $id:tt  ) ),* @css $($style:tt)* ) => {
        {
            use r_ui::UI;
            let mut ui = UI::default();
            let h = ele!("main" $( ($ele $id  ) ),* );
            ui.add_html(&h);
            let css = stringify!($($style)*);
            if css.contains("@ js"){
                let fs:Vec<&str>  = css.split("@ js").collect();
                let ncss = fs[0].to_string().replace(" ", "");
                let js = fs.last().unwrap();
                ui.add_css(&ncss);
                ui.add_js(&js);
            }else{
                let ncss = css.replace(" ","");
                ui.add_css(&ncss);
            }


            ui
        }
    };
}
#[allow(dead_code)]
pub fn log_init() {
    let mut clog = colog::builder();
    clog.format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    });
    // clog.filter(None, log::LevelFilter::Warn);
    clog.init();
}

pub fn with_search_extend<'a, F>(html: &UI, how_handle: F)
where
    F: Fn(view::S, &view::R) + Send + 'static,
{
    search_box(how_handle, html);
}

pub fn with_search<'a, F>(how_handle: F)
where
    F: Fn(view::S, &view::R) + Send + 'static,
{
    let html = UI::default();
    search_box(how_handle, &html);
}

pub trait View<'a> {
    fn render(&mut self, id: &str, content: &str);
    fn render_with_json(&mut self, json_data: &str);
    fn render_with_list(&mut self, list: &Vec<String>);
}

#[derive(Serialize, Deserialize)]
pub struct Rpc {
    content: Vec<String>,
    id: String,
    tp: String,
    progress: String,
}

pub fn rpc_from(rx: &view::R) -> (String, String, String) {
    let m = rx.recv().expect("recv failed");
    m.split_twice(view::MSG_SEP)
}

pub fn rpc_msg_progress(id: &str, tp: &str, content: &str, pro: usize, tx: view::S) {
    let r = Rpc {
        id: id.to_string(),
        tp: tp.to_string(),
        progress: format!("{}", pro),
        content: vec![content.to_string()],
    };
    let c = serde_json::to_string(&r).expect("trans to json failed!");
    let c_json = base64::encode(c.as_bytes());
    tx.send(c_json).expect("send to view failed");
}

pub fn rpc_msg(id: &str, tp: &str, content: &str, tx: view::S) {
    let r = Rpc {
        id: id.to_string(),
        tp: tp.to_string(),
        progress: "none".to_string(),
        content: vec![content.to_string()],
    };
    let c = serde_json::to_string(&r).expect("trans to json failed!");
    let c_json = base64::encode(c.as_bytes());
    tx.send(c_json).expect("send to view failed");
}

pub fn rpc_list_pro(id: &str, tp: &str, pro: usize, content: &Vec<String>, tx: view::S) {
    let r = Rpc {
        id: id.to_string(),
        tp: tp.to_string(),
        progress: format!("{}", pro),
        content: content.clone(),
    };

    let c = serde_json::to_string(&r).expect("trans to json failed!");
    tx.send(c).expect("send to view failed");
}

pub fn rpc_list(id: &str, tp: &str, content: &Vec<String>, tx: view::S) {
    let r = Rpc {
        id: id.to_string(),
        tp: tp.to_string(),
        progress: "none".to_string(),
        content: content.clone(),
    };

    let c = serde_json::to_string(&r).expect("trans to json failed!");
    tx.send(c).expect("send to view failed");
}

pub trait B64 {
    fn b64(&self) -> Vec<String>;
}

impl B64 for Vec<String> {
    fn b64(&self) -> Vec<String> {
        self.iter().map(|s| base64::encode(s.as_bytes())).collect()
    }
}

impl Rpc {
    pub fn to_msg(id: &str, tp: &str, content: &str) -> String {
        let r = Rpc {
            id: id.to_string(),
            tp: tp.to_string(),
            progress: "none".to_string(),
            content: vec![content.to_string()],
        };
        serde_json::to_string(&r).unwrap()
    }
}

impl Default for Rpc {
    fn default() -> Self {
        Self {
            id: "send".to_string(),
            tp: "list".to_string(),
            progress: "none".to_string(),
            content: vec![],
        }
    }
}

impl<'a> View<'a> for web_view::WebView<'a, ()> {
    fn render_with_list(&mut self, list: &Vec<String>) {
        let v = Rpc {
            content: list.clone(),
            ..Rpc::default()
        };

        let vv = serde_json::to_string(&v).unwrap();
        self.render_with_json(&vv);
    }

    fn render(&mut self, id: &str, content: &str) {
        let r = Rpc {
            id: id.to_string(),
            content: vec![content.to_string()],
            progress: "none".to_string(),
            tp: "1".to_string(),
        };
        let rpc_str = serde_json::to_string(&r).unwrap();
        let _ = self.eval(&format!("rpc.render('{}')", &rpc_str));
    }

    fn render_with_json(&mut self, json_data: &str) {
        // if json_data.contains("{") && json_data.contains("}"){
        let _ = self.eval(&format!("rpc.render('{}')", json_data));
        // }else{
        // log::error!("no valid data to pass {}", json_data.red());
        // }
    }
}

#[allow(dead_code)]
pub fn to_rpc_msg(id: &str, msg: &str) -> String {
    Rpc::to_msg(id, "normal", msg)
}

fn search_box<'a, F>(how_handle: F, html: &UI)
where
    F: FnOnce(view::S, &view::R) + Send + 'static,
{
    // log_inif();
    // view::job_with(how_handle);
    let _ = view::with_build(&html, how_handle);
}

#[test]
fn test_macro() {

    // let mut h = with_html!{@li
    //     (I "hello"),
    //     (B "check it")
    //     @css
    //     button#check-it{
    //         position: absolute;
    //         bottom: 1%;
    //     }
    //     @js
    //     console.log("hello")

    // };
    // h.add_js("console.log('hllo');");
    // println!("html {}",h.html);
    // println!("css {}",h.css);
    // println!("js {}",h.js);
}

#[test]
fn file() {
    // use utils::FileSystem;
    // // let ffff:Box<dyn Fn(&str) -> bool> = "screen.png$".to_regex_filter();
    // // assert!(ffff("screen.png"), true);
    // let f = "screen.png$".re();
    // // assert!(f("screen.png") == true, true);
    // let fs =  "./".ein(f);
    // assert!(fs.len() == 2, true);
}
