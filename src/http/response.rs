use crate::http::http_shared::{HttpBasicHeader, HttpContentType};
use derive_getters::Getters;
use std::collections::HashMap;

#[derive(Getters)]
pub(crate) struct Response {
    is_finished: bool,

    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            is_finished: false,
            status: 200,
            headers: HashMap::from_iter([
                (
                    HttpBasicHeader::Server.to_string(),
                    "RUST-HTTP-9000".to_string(),
                ),
                (
                    HttpBasicHeader::ContentType.to_string(),
                    HttpContentType::Json.to_string(),
                ),
            ]),
            body: String::from(""),
        }
    }
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }

    pub fn json(&mut self, payload: String) {
        self.headers.insert(
            HttpBasicHeader::ContentType.to_string(),
            HttpContentType::Json.to_string(),
        );
        self.body = payload;

        self.is_finished = true;
    }

    pub fn text(&mut self, payload: String) {
        self.headers.insert(
            HttpBasicHeader::ContentType.to_string(),
            HttpContentType::PlainText.to_string(),
        );
        self.body = payload;

        self.is_finished = true;
    }

    pub fn html(&mut self, payload: String) {
        self.headers.insert(
            HttpBasicHeader::ContentType.to_string(),
            HttpContentType::Html.to_string(),
        );
        self.body = payload;

        self.is_finished = true;
    }
}
