use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Debug)]
pub(crate) enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Option,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "patch" => Ok(HttpMethod::Patch),
            "delete" => Ok(HttpMethod::Delete),
            "option" => Ok(HttpMethod::Option),
            _ => Err(()),
        }
    }
}
