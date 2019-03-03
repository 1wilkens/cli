use kapitalist_types::request::UserCreationRequest;
use reqwest::{Client, Url};

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
        let resp = self
            .client
            .post(url)
            .json(&request)
            .send()
            // XXX: Better error handling
            .expect("Failed to send request to the kapitalist backend");
        match resp.status() {
            s if s.is_success() => println!("Successfully registered new user"),
            s if s.is_client_error() => println!("4XX: Client error"),
            s if s.is_server_error() => println!("5XX: Server error"),
            s => println!("{}: unknown error", s.as_str()),
        }
    }
}
