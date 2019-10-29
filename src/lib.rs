extern crate web_view;

use serde_json;
use log;
use colog;
use chrono::Local;
use std::io::Write;
use colored::Colorize;

mod view;
#[macro_use]
mod widgets;

pub struct UI{
    html:String,
    css: String,
    js:String,
    theme:web_view::Color,
}

impl Default for UI {
    fn default() -> Self{
        let d = ele!{ "search"
            (I "search")
            | r#"
            margin-top: 10px;
            width:96%;
            border-radius: 4px;
            left: 8px;
        "#};
        Self{
            html: d,
            css : r#"
#search{
    position:absolute;
    bottom:1%;
    width:100%;
}
    "#.to_string(),
            js: "".to_string(),
            theme: web_view::Color::from((5, 44, 92)),
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

    pub fn to_string(&self) -> String{
        let mut m = String::from(format!(r#"<style type="text/csstyle">{}</style>"#, &self.css));
        m.push_str(&self.html);
        m.push_str(&format!(r#"<script type="text/javascript">{}</script>"#, &self.js));
        m
    }
}

#[macro_export]
macro_rules! with_html {
    ( $( ($ele:ident $id:tt) ),* @css $($style:tt)* ) => {
        {
            let mut ui = UI::default();
            let h = ele!("main" $( ($ele $id) ),* );
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

fn log_inif(){
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

pub fn with_search_extend<F>(how_handle: F, html:&UI)
where F: Fn(&str, &str, &str){
    search_box(how_handle, html);
}

pub fn with_search<F>(how_handle: F)
where F: Fn(&str, &str, &str){
    let html = UI::default();
    search_box(how_handle, &html);
}

fn search_box<F> (how_handle: F, html:&UI)
where F: Fn(&str, &str, &str)  {
    // log_inif();
    log_inif();
    
    let _ = view::with_build(&html.html, &html.css,&html.js, html.theme ,|_, arg|{
        let value:serde_json::Value = match serde_json::from_str(arg){
            Ok(a) => a,
            Err(e) => {
                log::info!("err {}",e.to_string().red());
                serde_json::from_str(r#"{}"#).unwrap()
            }
        };
        if let Some(value) = value.get("text"){
            let from_id = value.get("id").unwrap().as_str().unwrap();
            let content = value.get("content").unwrap().as_str().unwrap();
            log::info!("edit |{}| : {}", from_id.green(),content.yellow());
            how_handle("text",from_id, content);
        }else if let Some(value) = value.get("btn"){
            let from_id = value.get("id").unwrap().as_str().unwrap();
            let content = value.get("content").unwrap().as_str().unwrap();
            log::info!("btn |{}| : {}", from_id.green(),content.yellow());
            how_handle("btn",from_id, content);
        }else{
            log::info!(" {:?}", value);
        }
        Ok(())
    });
}



#[test]
fn test_macro(){

    let mut h = with_html!{
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