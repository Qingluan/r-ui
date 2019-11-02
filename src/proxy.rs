use futures::{future, Future, Stream};
use hyper::{ 
    HeaderMap, Method, Body, Uri,
    Version,
    StatusCode
    };
use reqwest::header::HOST;
// use gotham::handler::{HandlerFuture, IntoHandlerError};
// use gotham::helpers::http::response::create_empty_response;
// use gotham::router::builder::{bulild_simple_router, DefineSingleRoute, DrawRoutes};

// use gotham::router::builder::{StateData, StaticResponseExtender};
use gotham::{
    state::{
        // FromState,
        State
    },
    handler::{
        IntoHandlerError,
        HandlerFuture,
    },
    helpers::http::response::create_response
};

// use super::net::Payload;
use std::str;
// use std::io::BufWriter;

use colored::Colorize;


/// Extract the main elements of the request except for the `Body`
fn print_request_elements(mut state: State) ->  Box<HandlerFuture>  {
    
    // let method = Method::borrow_from(&state);
    // let uri = Uri::borrow_from(&state);
    // let http_version = Version::borrow_from(&state);
    // let mut headers = HeaderMap::borrow_from(&state).clone();
    let mut res = String::new(); 
    let v = state.take::<Body>().concat2().then(move |chunk| match chunk {
        Ok(chunk) => {
            let method = state.borrow::<Method>().clone();
            let uri = state.borrow::<Uri>().clone();
            let http_version = state.borrow::<Version>();
    
            let mut headers = state.borrow::<HeaderMap>().clone();
            let content = str::from_utf8(&chunk.into_bytes()).unwrap().to_string();
            if uri.scheme_str() == Some("http"){
            
                headers.insert(HOST, uri.host().unwrap().parse().unwrap());
                let mut headers_str = String::new();
            
                for (k,v ) in headers {
                    headers_str.push_str(&format!("{:?}: {:?}\r\n", k.unwrap(),v.to_str()) );        
                }
                println!("{} {} {:?}\r\n{}\r\n\r\n{}",method.to_string().yellow(), uri, http_version,headers_str.blue(), &content.green() );
        
            }
            let res = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, content);
                
            future::ok((state, res))
        },
        Err(e) => future::err((state, e.into_handler_error() ))

        //future::err((state, "nothing"))  e.into_handler_error()
    });
    // Box::new(v)

    // headers.insert(HOST, uri.host().unwrap().parse().unwrap());
    // if uri.scheme_str() == Some("http"){
        // let f = Body::borrow_from(&state);
        
        // let mut buf: Vec<&u8> = vec![];
        // buf.write_all(f);
        // body.to_string();
        // let strea = body.concat2();
        // let s = f.concat2();
        //     .then(|full_body| match full_body {
        //     Ok(valid_body) => {
        //         let body_content = valid_body.into_bytes();
        //         // Perform decoding on request body
        //         println!("{}", &str::from_utf8(&body_content).expect("to utf8 X"));
        //         future::ok((state, ()))
        //     }
        //     Err(e) => future::err((state, e.into_handler_error())),
        // });
        // let pay = Payload::{
        //     uri: uri.to_string(),
        //     headers: headers.clone(),

        // }
    // }
    
    // println!("path {:?}", path.parts);
    // (&state, res)
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