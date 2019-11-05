use futures::{future, Future, Stream};
use gotham::{
    handler::{HandlerFuture, IntoHandlerError},
    helpers::http::response::create_empty_response,
    state::State,
};
use hyper::{Body, HeaderMap, Method, StatusCode, Uri, Version};
use reqwest::header::HOST;

use super::net::Req;
use std::{str, sync::Mutex};

lazy_static! {
    static ref BRD: Mutex<Brd> = {
        let (tx, rx) = bounded::<String>(2);
        Mutex::new(Brd { tx: tx, rx: rx })
    };
}

pub const PASS_REQ: &str = "[[PASS-THIS-REQ]]";
pub const DROP_REQ: &str = "[[DROP-THIS-REQ]]";
pub const REQ_HANDLE: &str = "[[REQ_HANDLE]]";
pub const REQ_SERVER: &str = "[[REQ_SERVER]]";

use crossbeam_channel::{bounded, Receiver as MReceiver, Sender as MSender};

use colored::Colorize;
pub type S<T> = MSender<T>;
pub type R<T> = MReceiver<T>;
// type SS<T> = Sender<S<T>>;
// type RS<T> = Receiver<S<T>>;

pub struct Brd {
    tx: S<String>,
    rx: R<String>,
}

impl Brd {
    pub fn regist_brd(&self) -> (S<String>, R<String>) {
        (self.tx.clone(), self.rx.clone())
    }
    pub fn recv_with(rx: R<String>, tx: S<String>, name: &str) -> Option<String> {
        let tag = format!("[[{}]]", name);
        loop {
            let m = rx.recv().expect("recv failed");
            if m.starts_with(&tag) {
                return Some(m.replace(&tag, ""));
            } else {
                tx.send(m).expect("try again");
            }
        }
    }

    pub fn send_to(tx: S<String>, name: &str, msg: &str) {
        // let m = rx.recv().expect("recv failed");
        let content = format!("[[{}]]{}", name, msg);
        tx.send(content).expect("send to failed");
    }
}

pub fn global_brd() -> (S<String>, R<String>) {
    let s = BRD.lock().expect("lock failed");
    s.regist_brd()
}

pub fn brd_once(msg: &str) {
    let b = BRD.lock().unwrap();
    match b.tx.send(msg.to_string()) {
        Ok(_) => {}
        _ => {
            println!("{}", "filaed");
        }
    }
}

fn state_to_req(state: &State, body: &[u8]) -> String {
    let method = state.borrow::<Method>().clone();
    let uri = state.borrow::<Uri>().clone();
    let http_version = state.borrow::<Version>();
    let mut headers_str = String::new();
    let mut headers = state.borrow::<HeaderMap>().clone();
    let content = str::from_utf8(body).unwrap().to_string();
    headers.insert(HOST, uri.host().unwrap().parse().unwrap());
    for (k, v) in headers {
        headers_str.push_str(&format!("{}: {}\r\n", k.unwrap(), v.to_str().unwrap()));
    }
    format!(
        "{} {} {:?}\r\n{}\r\n\r\n{}",
        method.to_string(),
        uri,
        http_version,
        headers_str,
        &content
    )
}

pub fn sniff(mut state: State) -> Box<HandlerFuture> {
    let v = state
        .take::<Body>()
        .concat2()
        .then(move |chunk| match chunk {
            Ok(chunk) => {
                let req_str = state_to_req(&state, &chunk.into_bytes());
                // let method = state.borrow::<Method>().clone();
                // let uri = state.borrow::<Uri>().clone();
                // let http_version = state.borrow::<Version>();

                let mut res = create_empty_response(&state, StatusCode::OK);
                let (tx, rx) = global_brd();
                Brd::send_to(tx.clone(), REQ_HANDLE, &req_str);
                if let Some(msg) = Brd::recv_with(rx, tx, REQ_SERVER) {
                    // println!("rx: {}", msg.green());
                    if msg.starts_with(PASS_REQ) {
                        let req = msg.replace(PASS_REQ, "");
                        let mut req_res = req.to_req().unwrap().send();
                        if !req_res.status().is_success() {
                            return future::ok((state, res));
                            // res  = create_empty_response(&state, StatusCode::NOT_FOUND);
                        }
                        let back_headers = res.headers_mut();
                        for (k, v) in req_res.headers() {
                            back_headers.insert(k, v.clone());
                        }

                        let b = Body::from(req_res.text().unwrap());
                        *res.body_mut() = b.into();
                    } else {
                        *res.body_mut() = Body::from("DROPED this req").into();
                    }
                }

                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(v)
}

fn proxy(state: State) -> Box<HandlerFuture> {
    sniff(state)
}

pub fn how_to_handler_sniff<F>(closure: F)
where
    F: Fn(&str) -> Option<String>,
{
    let (tx, rx) = global_brd();
    if let Some(msg) = Brd::recv_with(rx, tx.clone(), REQ_HANDLE) {
        if let Some(output) = closure(&msg) {
            Brd::send_to(tx, REQ_SERVER, &output);
        } else {
            Brd::send_to(tx, REQ_SERVER, DROP_REQ);
        }
    }
}

/// Start a server and use a `Router` to dispatch requests
pub fn server_start(addr: &str) {
    // let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr.yellow());
    // gotham::start(addr, || Ok(print_request_elements));
    let ad: &'static str = Box::leak(addr.to_string().into_boxed_str());
    // gotham::test::TestServer::with_timeout()
    // gotham.
    gotham::start(ad, || Ok(proxy));
}
