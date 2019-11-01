use reqwest;
use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap,HeaderValue, HeaderName};
// use std::collections::HashMap;
use std::str::from_utf8;

pub struct Payload {
    url: String,
    host: String, 
    headers: HeaderMap,
    method: String,
    data: String
}

impl Default for Payload {
    fn default() -> Self {
        Self{
            url:String::new(),
            host: "".to_string(),
            headers: HeaderMap::new(),
            method: "GET".to_string(),
            data: "".to_string(),
        }
    }
}

impl Payload{
    pub fn send(&self) -> reqwest::Response{
        let cli = reqwest::Client::new();
        let body = self.data.clone();
        match self.method.as_str() {
            "GET" => {
                cli.get(&self.url)
                    .headers(self.headers.clone())
                    .body( body)
                    .send().expect("send failed")
            },
            "POST" => {
                cli.post(&self.url)
                    .headers(self.headers.clone())
                    .body( body)
                    .send().expect("send failed")
            },
            _ => {
                cli.head(&self.url)
                    .headers(self.headers.clone())
                    .body( body)
                    .send().expect("send failed")

            }
        }
        
    }

    pub fn ua(&mut self, new_ua:&str) {
        self.headers.insert(USER_AGENT,HeaderValue::from_str(new_ua).expect("tr faild"));
    }
}

pub trait Req {
    fn to_req(&self) -> Option<Payload>;
} 

impl <'a>Req for &'a str{
    fn to_req(&self) -> Option<Payload>{
        if !self.contains("host"){
            return None;
        }
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(self.as_bytes()).unwrap();
        if res.is_complete() {
            let mut pay = Payload{
                method:req.method.unwrap().to_string(),
                ..Payload::default()
            };
            let _ = req.headers.iter().map(|v|{
                if v.name == "Host"{
                    pay.url = format!("http://{}{}", from_utf8(v.value).expect("decode utf8 failed"), req.path.unwrap());
                    pay.host = from_utf8(v.value).expect("dec utf8 failed").to_string();
                }
                pay.headers.insert(HeaderName::from_bytes(v.name.as_bytes()).unwrap(), HeaderValue::from_bytes(v.value).expect("dec utf8"));
            }).collect::<Vec<_>>();
            return Some(pay);
        }

        None

    }
}

impl Req for String {
    
    fn to_req(&self) -> Option<Payload>{
        self.as_str().to_req()
    }

}