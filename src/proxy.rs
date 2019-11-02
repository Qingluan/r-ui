use futures::{future, Future, Stream};
use hyper::{ 
    HeaderMap, Method, Body, Uri,
    Version,
    StatusCode,
    // Response
    };
use reqwest::header::HOST;
use gotham::{
    state::{
        // FromState,
        State
    },
    handler::{
        IntoHandlerError,
        HandlerFuture,
    },
    helpers::http::response::{
        // create_response,
        create_empty_response
    }
};

use super::net::{
    Payload,
    Req,
    // Res
};
use std::{
    str,
    sync::{
        Mutex,
        mpsc::{
            Sender,Receiver,
            channel
        }
    }
};

use colored::Colorize;

pub struct Brd{
    tx: Sender<String>,
    // rx: Receiver<String>
}
impl Brd{
    fn new_brd(&mut self)  {
        // let mut s = BRD.lock().unwrap();
        let (tx,rx) = channel::<String>();
        self.tx = tx;
        // self.rx = rx;
    } 
}

lazy_static!{
    static ref BRD:Mutex<Brd> = {
        let (tx,_) = channel::<String>();
        Mutex::new(
            Brd{
                tx:tx,
                // rx:rx
            }
        )
    };
}

pub fn brd_once(msg:&str) {
    let b = BRD.lock().unwrap();
    b.tx.send(msg.to_string()).expect("send error");
}

fn regist_brd() -> Receiver<String>{
    let (tx, rx) = channel::<String>();
    let mut b = BRD.lock().unwrap();
    b.tx  = tx;
    rx
}



/// Extract the main elements of the request except for the `Body`
fn print_request_elements(mut state: State) ->  Box<HandlerFuture>  {
    let v = state.take::<Body>().concat2().then(move |chunk| match chunk {
        Ok(chunk) => {
            let method = state.borrow::<Method>().clone();
            let uri = state.borrow::<Uri>().clone();
            let http_version = state.borrow::<Version>();

            let mut res  = create_empty_response(&state, StatusCode::OK);
                
            let mut headers = state.borrow::<HeaderMap>().clone();
            let content = str::from_utf8(&chunk.into_bytes()).unwrap().to_string();
            if uri.scheme_str() == Some("http"){
            
                headers.insert(HOST, uri.host().unwrap().parse().unwrap());
                let mut headers_str = String::new();
            
                for (k,v ) in headers {
                    headers_str.push_str(&format!("{}: {}\r\n", k.unwrap(), v.to_str().unwrap() ) );        
                }
                let req_str = format!("{} {} {:?}\r\n{}\r\n\r\n{}",method.to_string(), uri, http_version,headers_str, &content);
                println!("{}", req_str.yellow());
                
                let mut req_res = req_str.to_req().unwrap().send();
                // let res_copy = req_res.clone();
                if !req_res.status().is_success() {
                    res  = create_empty_response(&state, StatusCode::NOT_FOUND);
                
                }
                let back_headers = res.headers_mut();
                for  (k,v) in req_res.headers(){
                    back_headers.insert(k, v.clone());
                }

                let b= Body::from(req_res.text().unwrap()); 
                *res.body_mut() = b.into(); 


                let rx = regist_brd();
                if &rx.recv().expect("no recv") != "ok"{
                    res = create_empty_response(&state, StatusCode::OK);
                }

                
                // println!("{}", Payload::res_to_res_and_str(&res).green());
            }
                
            future::ok((state, res))
        },
        Err(e) => future::err((state, e.into_handler_error() ))

    });
    Box::new(v)
}



/// Create a `Router`
// fn router() -> Router {
//     let (chain, pipelines) = single_pipeline(new_pipeline().add(CookieParser).build());
//     build_router(chain, pipelines, |route| {
//         route.get("/").to(handler);
//     })
// }

/// Start a server and use a `Router` to dispatch requests
pub fn server_start() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(print_request_elements));
}