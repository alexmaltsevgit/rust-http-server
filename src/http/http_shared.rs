use strum_macros::{Display, EnumString};

#[derive(Hash, PartialEq, Eq, Debug, EnumString)]
pub enum HttpMethod {
    #[strum(ascii_case_insensitive)]
    Get,
    #[strum(ascii_case_insensitive)]
    Post,
    #[strum(ascii_case_insensitive)]
    Put,
    #[strum(ascii_case_insensitive)]
    Patch,
    #[strum(ascii_case_insensitive)]
    Delete,
    #[strum(ascii_case_insensitive)]
    Option,
}

#[derive(EnumString, Display)]
pub enum HttpContentType {
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "application/json")]
    Json,
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "text/plain")]
    PlainText,
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "text/html")]
    Html,
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "multipart/form-data")]
    FormData,
}

#[derive(EnumString, Display)]
pub enum HttpBasicHeader {
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "Server")]
    Server,
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "Content-Type")]
    ContentType,
}
