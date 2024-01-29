use reqwest::{Client, Response};

/// Base object to query http request.
#[derive(Default)]
pub struct HttpQuerier {
    url: String,
    client: Client,
}

impl HttpQuerier {
    /// Create a new HttpQuerier.
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: Client::new(),
        }
    }

    pub async fn query(&self, path: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}{}", self.url, path);
        self.client.get(&url).send().await
    }
}
