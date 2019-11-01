extern crate web_view;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate threadpool;
// use serde_json;
use log;
use colog;
use chrono::Local;
use std::io::Write;
use colored::Colorize;

// use serde_json::Value;
// use serde::{Deserialize, Serialize};
// mod backend;
pub mod view;
#[macro_use]
pub mod widgets;
pub mod utils;
pub mod action;
pub mod net;

#[allow(unused_imports)]
// use view::SP2;


pub trait SP2 {
    fn split_twice(&self, sep:&str) -> (String, String, String);
    fn split_once(&self, sep:&str) -> (String, String);
}

impl SP2 for String{
    fn split_twice(&self, sep:&str) -> (String, String, String){
        let mut f = self.split(sep);
        (f.next().unwrap().to_string(),f.next().unwrap().to_string(),f.next().unwrap().to_string())
    }
    fn split_once(&self, sep:&str) -> (String, String){
        let mut f = self.split(sep);
        (f.next().unwrap().to_string(),f.next().unwrap().to_string())
    }
}

impl <'a>SP2 for &'a str{
    fn split_twice(&self, sep:&str) -> (String, String, String){
        let mut f = self.split(sep);
        (f.next().unwrap().to_string(),f.next().unwrap().to_string(),f.next().unwrap().to_string())
    }
    fn split_once(&self, sep:&str) -> (String, String){
        let mut f = self.split(sep);
        (f.next().unwrap().to_string(),f.next().unwrap().to_string())
    }
}


pub struct UI{
    html:String,
    css: String,
    js:String,
    theme:web_view::Color,
    size:(i32, i32),
    title_color:web_view::Color,
}

impl Default for UI {
    fn default() -> Self{
        // let d = ele!{ "search"
        //     (I "search")
        //     | r#""#};
        Self{
            html: r#""#.to_string(),
            css : r#""#.to_string(),
            size:(360,600),
            js: "".to_string(),
            theme: web_view::Color::from((5, 44, 92)),
            title_color:web_view::Color::from((255,255,255)),
            // theme: web_view::Color::from((5, 4, 5)),
        }
    }

}
impl UI {
    #[allow(dead_code)]
    pub fn add_html(&mut self, new:&str) {
        self.html.push_str("\n");
        self.html.push_str(new);
    }
    #[allow(dead_code)]
    pub fn add_css(&mut self, new:&str) {
        self.css.push_str("\n");
        self.css.push_str(new);
    }
    #[allow(dead_code)]
    pub fn add_js(&mut self, new:&str) {
        self.js.push_str("\n");
        self.js.push_str(new);
    }

    pub fn chain(&mut self, other_ui:&UI){
        self.add_html(&other_ui.html);
        self.add_css(&other_ui.css);
        self.add_js(&other_ui.js);
    }

    pub fn to_string(&self) -> String{
        let mut m = String::from(format!(r#"<style type="text/csstyle">{}</style>"#, &self.css));
        m.push_str(&self.html);
        m.push_str(&format!(r#"<script type="text/javascript">{}</script>"#, &self.js));
        m
    }
}

#[macro_export]
macro_rules! with_html {
    (@html $(($expr:expr)),* @css $($style:tt)* ) =>{
        {
            use search_ui::UI;
            let mut ui = UI::default();
            $(
                let html:String = $expr;
                ui.add_html(&html);
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
            use self::UI;
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
pub fn log_init(){
    let mut clog = colog::builder();
    clog.format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
    });
    // clog.filter(None, log::LevelFilter::Warn);
    clog.init();
}

pub fn with_search_extend<'a,F>(html:&UI,how_handle:F)
where F: Fn(view::S, &view::R) +Send +'static {
    search_box(how_handle, html);
}

pub fn with_search<'a,F>(how_handle:F)
where F: Fn(view::S, &view::R) +Send +'static {
    let html = UI::default();
    search_box(how_handle, &html);
}

pub trait View<'a>{
    fn render(&mut self, id:&str, content:&str);
    fn render_with_json(&mut self, json_data:&str);
    fn render_with_list(&mut self, list:&Vec<String>);
}


#[derive(Serialize, Deserialize)]
pub struct Rpc {
    content: Vec<String>,
    id: String,
    tp:String,
    progress:String,
}

pub fn rpc_from(rx:&view::R) -> (String,String, String){
    let m = rx.recv().expect("recv failed");
    m.split_twice(view::MSG_SEP)
}

pub fn rpc_msg_progress(id:&str, tp:&str, content:&str, pro:usize,tx:view::S){
    let r = Rpc{
        id:id.to_string(),
        tp:tp.to_string(),
        progress:format!("{}",pro),
        content:vec![content.to_string()],
    };
    let c = serde_json::to_string(&r).expect("trans to json failed!");
    tx.send(c).expect("send to view failed");
}


pub fn rpc_msg(id:&str, tp:&str, content:&str, tx:view::S){
    let r = Rpc{
        id:id.to_string(),
        tp:tp.to_string(),
        progress:"none".to_string(),
        content:vec![content.to_string()],
    };
    let c = serde_json::to_string(&r).expect("trans to json failed!");
    tx.send(c).expect("send to view failed");
}

pub fn rpc_list_pro(id:&str, tp:&str, pro:usize, content:&Vec<String>, tx:view::S){
    let r = Rpc{
        id:id.to_string(),
        tp:tp.to_string(),
        progress:format!("{}",pro),
        content:content.clone(),
    };

    let c = serde_json::to_string(&r).expect("trans to json failed!");
    tx.send(c).expect("send to view failed");
}


pub fn rpc_list(id:&str, tp:&str, content:&Vec<String>, tx:view::S){
    let r = Rpc{
        id:id.to_string(),
        tp:tp.to_string(),
        progress:"none".to_string(),
        content:content.clone(),
    };

    let c = serde_json::to_string(&r).expect("trans to json failed!");
    tx.send(c).expect("send to view failed");
}


impl Rpc{
    pub fn to_msg(id:&str,tp:&str, content:&str) -> String{
        let r = Rpc{
            id:id.to_string(),
            tp:tp.to_string(),
            progress:"none".to_string(),
            content:vec![content.to_string()],
        };
        serde_json::to_string(&r).unwrap()
    }
}
impl Default for Rpc{
    fn default()->Self{
        Self{
            id:"send".to_string(),
            tp:"list".to_string(),
            progress:"none".to_string(),
            content:vec![]
        }
    }
}

impl <'a>View<'a> for web_view::WebView<'a,()> {
    fn render_with_list(&mut self, list:&Vec<String>){
        let v = Rpc{
            content: list.clone(),
            ..Rpc::default()
        };
        
        let vv = serde_json::to_string(&v).unwrap();
        self.render_with_json(&vv);
    }

    fn render(&mut self, id:&str, content:&str){
        let r = Rpc{
            id:id.to_string(),
            content:vec![content.to_string()],
            progress:"none".to_string(),
            tp:"1".to_string(),
        };
        let rpc_str = serde_json::to_string(&r).unwrap();
        let _ = self.eval(&format!("rpc.render('{}')", &rpc_str));
    }

    fn render_with_json(&mut self, json_data:&str){
        if json_data.contains("{") && json_data.contains("}"){
            let _ = self.eval(&format!("rpc.render('{}')", json_data));
        }else{
            log::error!("no valid data to pass {}", json_data.red());
        }
        
    }
}

#[allow(dead_code)]
pub fn to_rpc_msg(id:&str, msg:&str) -> String{
    Rpc::to_msg(id, "normal",msg)
}


fn search_box<'a,F> (how_handle:F, html:&UI)
where F: FnOnce(view::S, &view::R) + Send +'static{
    // log_inif();
    // view::job_with(how_handle);
    let _ = view::with_build(&html,how_handle);

}



#[test]
fn test_macro(){

    let mut h = with_html!{@li
        (I "hello"),
        (B "check it")
        @css
        button#check-it{
            position: absolute;
            bottom: 1%;
        }
        @js
        console.log("hello")
        
    };
    h.add_js("console.log('hllo');");
    println!("html {}",h.html);
    println!("css {}",h.css);
    println!("js {}",h.js);
}



#[test]
fn file(){
    // use utils::FileSystem;
    // // let ffff:Box<dyn Fn(&str) -> bool> = "screen.png$".to_regex_filter();
    // // assert!(ffff("screen.png"), true);
    // let f = "screen.png$".re();
    // // assert!(f("screen.png") == true, true);
    // let fs =  "./".ein(f);
    // assert!(fs.len() == 2, true);
}