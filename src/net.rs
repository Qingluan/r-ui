// use futures::{future, Future, Stream};
use hyper::{body::Body, Response as HRes};
use reqwest;
use reqwest::{
    header::USER_AGENT,
    header::{HeaderMap, HeaderName, HeaderValue},
    Response,
};
// use std::collections::HashMap;
use std::str::from_utf8;

pub struct Payload {
    url: String,
    host: String,
    headers: HeaderMap,
    method: String,
    data: String,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            url: String::new(),
            host: "".to_string(),
            headers: HeaderMap::new(),
            method: "GET".to_string(),
            data: "".to_string(),
        }
    }
}

impl Payload {
    pub fn send(&self) -> reqwest::Response {
        let cli = reqwest::Client::new();
        let body = self.data.clone();
        // println!("---> {}" ,&self.url);
        match self.method.as_str() {
            "GET" => cli
                .get(&self.url)
                .headers(self.headers.clone())
                .body(body)
                .send()
                .expect("send failed"),
            "POST" => cli
                .post(&self.url)
                .headers(self.headers.clone())
                .body(body)
                .send()
                .expect("send failed"),
            _ => cli
                .head(&self.url)
                .headers(self.headers.clone())
                .body(body)
                .send()
                .expect("send failed"),
        }
    }

    pub fn ua(&mut self, new_ua: &str) {
        self.headers
            .insert(USER_AGENT, HeaderValue::from_str(new_ua).expect("tr faild"));
    }

    pub fn res_to_res_and_str(res: &HRes<Body>) -> String {
        // let b = res.body();
        // b.concat2();
        let mut buf = String::from(&format!("{:?} {} ok\r\n", res.version(), res.status()));
        for (k, v) in res.headers() {
            buf.push_str(&format!("{}: {}\r\n", k, v.to_str().unwrap()))
        }
        // buf.push_str(&format!("\r\n{:?}", res.body()));
        buf
    }
}

pub trait Req {
    fn to_req(&self) -> Option<Payload>;
}

impl<'a> From<&'a str> for Payload {
    fn from(w: &'a str) -> Payload {
        w.to_req().unwrap()
    }
}

pub trait Res {
    fn to_string(&mut self) -> String;
}

impl Res for Response {
    fn to_string(&mut self) -> String {
        let mut back = String::from(&format!("{:?} {:?} ok\r\n", self.version(), self.status()));
        // let code =
        println!("{:?}", self);
        for (k, v) in self.headers() {
            back.push_str(&format!("{}: {}\r\n", k, v.clone().to_str().unwrap()));
        }
        if let Ok(body) = self.text() {
            if body.len() > 0 {
                back.push_str(&format!("\r\n\r\n{}", body));
            }
        }
        back
    }
}

impl<'a> Req for &'a str {
    fn to_req(&self) -> Option<Payload> {
        if !self.contains("host") {
            return None;
        }
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(self.as_bytes()).unwrap();
        if res.is_complete() {
            let mut pay = Payload {
                method: req.method.unwrap().to_string(),
                ..Payload::default()
            };
            let _ = req
                .headers
                .iter()
                .map(|v| {
                    if v.name == "host" {
                        pay.url = format!("{}", req.path.unwrap());
                        pay.host = from_utf8(v.value).expect("dec utf8 failed").to_string();
                    } else {
                    }
                    pay.headers.insert(
                        HeaderName::from_bytes(v.name.as_bytes()).unwrap(),
                        HeaderValue::from_bytes(v.value).expect("dec utf8"),
                    );
                })
                .collect::<Vec<_>>();
            // println!("u:{} | h:{}", &pay.url, &pay.host);
            return Some(pay);
        }

        None
    }
}

impl Req for String {
    fn to_req(&self) -> Option<Payload> {
        self.as_str().to_req()
    }
}
