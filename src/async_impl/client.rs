#[derive(Debug)]
pub struct ApiClient {
    pub url: String,
    pub reqwest: reqwest::Client,
}
#[derive(Debug)]
pub struct ClientOptions {
    pub headers: Option<HeadersOptions>,
}
#[derive(Debug)]
pub struct HeadersOptions {
    pub authorization: Option<String>,
}
impl ApiClient {
    pub fn new(
        url: &str,
        options: Option<ClientOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_builder = reqwest::ClientBuilder::new();
        // Find options
        match options {
            // Build headers
            Some(ClientOptions { headers, .. }) => {
                let mut headers_map = reqwest::header::HeaderMap::new();
                match headers {
                    // header::AUTHORIZATION
                    Some(HeadersOptions {
                        authorization: Some(authorization),
                    }) => {
                        headers_map.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(&authorization).unwrap(),
                        );
                    }
                    _ => (),
                }
                client_builder = client_builder.default_headers(headers_map);
            }
            None => (),
        }
        let build = client_builder.build().unwrap_or(reqwest::Client::new());

        Ok(ApiClient {
            url: url.to_string(),
            reqwest: build,
        })
    }
    pub fn new_from_config(
        url: &str,
        client: reqwest::Client
    )->Result<Self, Box<dyn std::error::Error>> {
        Ok(ApiClient {
            url: url.to_string(),
            reqwest: client,
        })
    }
}
