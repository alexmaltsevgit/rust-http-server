use std::collections::HashMap;

pub(crate) struct Request {
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new(headers: HashMap<String, String>) -> Self {
        Request { headers }
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}
