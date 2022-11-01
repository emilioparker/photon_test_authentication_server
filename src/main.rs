
use std::collections::HashMap;
use std::{sync::Arc};

use hyper::{Request, body, server::conn::AddrStream};
use serde::{Deserialize, Serialize};
use tokio::time::error::Elapsed;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};


#[derive(Deserialize, Serialize, Debug)]
struct PlayerRequest {

    user: String,
    pass: String, //create
}

// Success: { "ResultCode": 1, "UserId": <userId> }
// Failure: { "ResultCode": 2, "Message": "Authentication failed. Wrong credentials." }

#[derive(Deserialize, Serialize, Debug)]
struct PlayerResponse {
    ResultCode: u32,
    UserId: String,
}

#[derive(Clone)]
struct AppContext {
    data_base : HashMap<String, String>
}


#[tokio::main()]
async fn main() {

    let mut data_base = HashMap::new();
    data_base.insert("joe".to_owned(), "<CTO>joe<CTO>".to_owned());
    data_base.insert("parker".to_owned(), "<Developer><Developer>".to_owned());
    let context = AppContext {
        data_base
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    let make_service = make_service_fn(move |conn: &AddrStream| {
        let context = context.clone();
        let _addr = conn.remote_addr();
        let service = service_fn(move |req| {
            handle(context.clone(), req)
        });

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle(context: AppContext, mut req: Request<Body>) -> Result<Response<Body>, Infallible> {

    // let body = req.body_mut();

    let query = req.uri().query().unwrap();

    let result = querystring::querify(query);

    // let data = body::to_bytes(body).await.unwrap();
    // let data: PlayerRequest = serde_json::from_slice(&data).unwrap();
    println!("got some data {:?}", result);

    let mut user_name : Option<String> = None;
    for data in result{
        if data.0.eq("user") {
            user_name = Some(data.1.to_owned());
        }
    }

    if let Some(user_name) = user_name {
        let assigned_user_name = context.data_base.get(&user_name);
        if let Some(assigned_user_name) = assigned_user_name {

            let player_response = PlayerResponse {
                ResultCode :1,
                UserId : assigned_user_name.to_owned()
            };
            let response = serde_json::to_vec(&player_response).unwrap();

            let response = Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from(response))
            .unwrap();

            return Ok(response);
        }
    }

    let player_response = PlayerResponse {
        ResultCode :2,
        UserId : " ".to_owned()
    };
    let response = serde_json::to_vec(&player_response).unwrap();

    let response = Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(Body::from(response))
    .unwrap();

    Ok(response)
    // Ok(Response::new(Body::from(response)))
}

