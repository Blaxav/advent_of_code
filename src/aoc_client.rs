use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest::Client;
use std::fs;

struct AoCClient {
    token: String,
    client: Client,
}

impl AoCClient {
    const BASE_URL: &str = "https://adventofcode.com/";

    fn new() -> AoCClient {
        let token = match fs::read_to_string(".auth") {
            Ok(t) => t,
            Err(e) => panic!("Unable to read token file: {}", e),
        };

        AoCClient {
            token,
            client: Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let cookie_value = format!("session={};", self.token);

        // Set up headers and add the Cookie header
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_str(&cookie_value).unwrap());
        headers
    }

    async fn get(self, endpoint: &str, data_path: &str) {
        let url = format!("{}{}", AoCClient::BASE_URL, endpoint);
        let response = self.client.get(&url).headers(self.headers()).send().await;

        match response {
            Ok(r) => match r.text().await {
                Ok(content) => {
                    match fs::write(data_path, content) {
                        Ok(_) => println!("Result written to {}", data_path),
                        Err(e) => println!("unable to write data: {}", e),
                    };
                }
                Err(e) => println!("Error: {}", e),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}

pub async fn get_data(year: u64, day: u64, data_path: Option<String>) {
    let client = AoCClient::new();

    let data_path: String = match data_path {
        Some(path) => path.clone(),
        None => format!("./data/{}/{}.txt", year, day),
    };

    let _ = client
        .get(&format!("{}/day/{}/input", year, day), &data_path)
        .await;
}
