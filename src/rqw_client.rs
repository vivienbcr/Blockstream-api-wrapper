pub struct Client {
    pub url: String,
    
}
impl Client {
    pub fn new(url: &str) -> Self {
        Client {
            url: url.to_string(),
        }
    }
}
