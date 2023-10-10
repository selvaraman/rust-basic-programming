use actix_web::{test, get, App, HttpServer, Responder };
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub support: Support,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: i64,
    pub email: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub avatar: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Support {
    pub url: String,
    pub text: String,
}

async fn get_downstream_data(base_url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::builder().build().unwrap();
    let api_url = "/api/users/2";
    let url = format!("{}{}", base_url, api_url);
    let response = client.get(url).send().await;
    response
}

#[get("/hello")]
async fn index() -> impl Responder {
    let base_url = "https://reqres.in";
    let response = get_downstream_data(base_url).await;
    if response.is_ok() {
        //println!("{:?}", response.unwrap());
        let result = response.unwrap().json::<Root>();

    }
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started...........");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, ResponseTemplate, Mock, matchers::{method, path}};

    #[actix_web::test]
    async fn test_api() {
        let mock_server = MockServer::start().await;
        let response_template = ResponseTemplate::new(200).set_body_string("Mocked response");
        Mock::given(method("GET"))
        .and(path("/api/users/2"))
        .respond_with(response_template)
        .mount(&mock_server)
        .await;
        let base_uri = mock_server.uri();
        println!("0000000000000000000000000000000000000");
        let res = get_downstream_data(base_uri.as_str()).await;
        println!("Result: {:?}", res);
        println!("sample case");
    }
}
