use kapitalist_types::request::UserCreationRequest;
use kapitalist_types::response::ErrorResponse;
use reqwest::{Client, StatusCode, Url};

pub(crate) struct Api {
    base_uri: Url,
    client: Client,
}

impl Api {
    pub(crate) fn new(base_uri: &str) -> Option<Self> {
        let url = Url::parse(base_uri);
        // XXX: Use `?` here
        let url = if let Ok(u) = url { u } else { return None };
        // XXX: Validate base_uri and set version in Api?
        Some(Self {
            base_uri: url,
            client: Client::new(),
        })
    }

    pub(crate) fn register(&mut self, email: String, password: String) {
        let request = UserCreationRequest {
            email: email,
            password: password,
            name: None,
        };

        // XXX: Currently can fail, but once we validate the base_uri above this should be safe
        let url = self.base_uri.join("register").unwrap();
        let mut resp = self
            .client
            .post(url)
            .json(&request)
            .send()
            // XXX: Better error handling
            .expect("Failed to send request to the kapitalist backend");

        let result = match resp.status() {
            s if s.is_success() => format!("[SUCCESS] Successfully registered new user"),
            StatusCode::UNAUTHORIZED => {
                format!("[ERROR] A user with that email address already exists")
            }
            s if s.is_client_error() => {
                let err: ErrorResponse = resp.json().expect("Got invalid response from backend");
                format!("[ERROR] 4XX: Client error: {}", err.error)
            }
            s if s.is_server_error() => format!("[ERROR] 5XX: Server error"),
            s => format!("[ERROR] {}: unknown error", s.as_str()),
        };
        println!("{}", result);
    }
}
