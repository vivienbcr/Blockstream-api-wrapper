/// A custom Reqwest Client with API URL
#[derive(Debug)]
pub struct ApiClient {
    pub url: String,
    pub reqwest: reqwest::blocking::Client,
}
/// List of option for custom Reqwest usage
#[derive(Debug)]
pub struct ClientOptions {
    pub headers: Option<HeadersOptions>,
}
/// Headers options can be used to use authorization header
#[derive(Debug)]
pub struct HeadersOptions {
    pub authorization: Option<String>,
}
impl ApiClient {
    /// Instanciate client from endpoint API URL, and options struct.
    pub fn new(
        url: &str,
        options: Option<ClientOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_builder = reqwest::blocking::ClientBuilder::new();
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
        let build = client_builder
            .build()
            .unwrap_or(reqwest::blocking::Client::new());

        Ok(ApiClient {
            url: url.to_string(),
            reqwest: build,
        })
    }
    pub fn new_from_config(
        url: &str,
        client: reqwest::blocking::Client
    )->Result<Self, Box<dyn std::error::Error>> {
        Ok(ApiClient {
            url: url.to_string(),
            reqwest: client,
        })
    }
}
