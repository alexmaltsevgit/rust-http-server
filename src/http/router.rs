use crate::http::http_method::HttpMethod;
use crate::http::request::Request;
use crate::http::response::Response;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Debug)]
pub(crate) struct ResourcePath(String);

impl From<&str> for ResourcePath {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub(crate) struct RouterPath {
    http_method: HttpMethod,
    resource_path: ResourcePath,
}

impl FromStr for RouterPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (http_method, resource) = s.split_once(" ").unwrap();
        Ok(RouterPath {
            http_method: http_method.parse()?,
            resource_path: ResourcePath(resource.to_string()),
        })
    }
}

type Handler = dyn Fn(&Request, &mut Response);

type MiddlewareChain<'a> = Vec<&'a Handler>;

pub(crate) struct Router<'a> {
    chains: HashMap<RouterPath, MiddlewareChain<'a>>,
}

macro_rules! define_method {
    ($name:ident, $http_method:expr) => {
        pub fn $name<'b>(&'b mut self, path: &str, handlers: &[&'a Handler]) {
            let path = RouterPath {
                http_method: $http_method,
                resource_path: path.into(),
            };

            let handlers_list: Vec<&'a Handler> = Vec::from(handlers);
            self.chains.insert(path, handlers_list);
        }
    };
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
        }
    }

    define_method!(get, HttpMethod::Get);
    define_method!(post, HttpMethod::Post);
    define_method!(put, HttpMethod::Put);
    define_method!(patch, HttpMethod::Patch);
    define_method!(delete, HttpMethod::Delete);

    pub fn run_route_middleware_chain(
        &self,
        router_path: &RouterPath,
        req: &Request,
        res: &mut Response,
    ) {
        let Some(chain) = self.chains.get(router_path) else {
            return;
        };

        chain.iter().for_each(|&handler| handler(req, res));
    }
}
