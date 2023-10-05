use actix_web::{test, get, App, HttpServer, Responder };
use reqwest::Client;

async fn get_downstream_data() -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::builder().build();
    let api_url = "https://reqres.in/api/users/2";
    let response = client.expect("REASON").get(api_url).send().await;
    response
}

#[get("/hello")]
async fn index() -> impl Responder {
    let response = get_downstream_data().await;
    if response.is_ok() {
        println!("{:?}", response.unwrap());
    }
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_api() {
        let mut server = mockito::Server::new();
        let host = server.host_with_port();
        let url = server.url();
        let mock = server
            .mock("GET", "/api/users/2")
            .with_status(201)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body("world")
            .create();
        let res = get_downstream_data().await;
        println!("{:?}", res);
        mock.assert();
    }
}