use crate::http::http_shared::HttpMethod;
use crate::http::request::Request;
use crate::http::response::Response;
use std::collections::BTreeMap;
use std::ops::Add;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub(crate) struct RoutePath(String);

impl From<&str> for RoutePath {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Ord, PartialOrd)]
pub(crate) struct Route {
    path: RoutePath,
    method: HttpMethod,
}

impl FromStr for Route {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (http_method, resource) = s.split_once(" ").unwrap();
        Ok(Route {
            method: HttpMethod::from_str(http_method)?,
            path: RoutePath(resource.to_string()),
        })
    }
}

type Handler = dyn Fn(&Request, &mut Response);

type MiddlewareChain<'a> = Vec<&'a Handler>;

pub(crate) struct Router<'a> {
    prefix: String,
    chains: BTreeMap<Route, MiddlewareChain<'a>>,
}

macro_rules! define_method {
    ($name:ident, $http_method:expr) => {
        pub fn $name<'b>(&'b mut self, path: &str, handlers: &[&'a Handler]) -> &mut Self {
            let path = self.prefix.clone().add(path);
            let route = Route {
                method: $http_method,
                path: path.as_str().into(),
            };

            let handlers_list: Vec<&'a Handler> = Vec::from(handlers);
            self.chains.insert(route, handlers_list);

            self
        }
    };
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Self {
            prefix: "".to_string(),
            chains: BTreeMap::new(),
        }
    }

    pub fn new_prefixed(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            chains: BTreeMap::new(),
        }
    }

    pub fn join_other(&mut self, other: Router<'a>) -> &mut Self {
        let group_entries = other.chains.into_iter().map(|(route, chain)| {
            let path = self.prefix.clone().add(route.path.0.as_str());
            let prefixed_path = Route {
                method: route.method,
                path: path.as_str().into(),
            };

            (prefixed_path, chain)
        });

        self.chains.extend(group_entries);

        self
    }

    pub fn routes(&self) -> Vec<&Route> {
        self.chains.keys().collect()
    }

    pub fn formatted_routes(&self) -> String {
        let routes = self.routes();
        let mut output = String::with_capacity(32);
        routes.iter().for_each(|x| {
            output.push_str(format!("{:8} {}\n", x.method, x.path.0).as_str());
        });

        output
    }

    define_method!(get, HttpMethod::Get);
    define_method!(post, HttpMethod::Post);
    define_method!(put, HttpMethod::Put);
    define_method!(patch, HttpMethod::Patch);
    define_method!(delete, HttpMethod::Delete);

    pub fn run_route_middleware_chain(
        &self,
        router_path: &Route,
        req: &Request,
        res: &mut Response,
    ) {
        let Some(chain) = self.chains.get(router_path) else {
            return;
        };

        chain.iter().for_each(|&handler| {
            if *res.is_finished() {
                return;
            }

            handler(req, res)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_route() {
        let mut router = Router::new();
        router.get("/", &[]);
        let routes = router.routes();
        assert_eq!(
            routes,
            vec![&Route {
                path: "/".into(),
                method: HttpMethod::Get
            }]
        );
    }

    #[test]
    fn grouped_routes() {
        let mut router = Router::new();

        router.get("/", &[]);

        let mut user_router = Router::new_prefixed("/user");
        user_router.get("/", &[]).post("/", &[]).get("/:id", &[]);

        router.join_other(user_router);

        let routes = router.routes();

        println!("{}", router.formatted_routes())

        // assert_eq!(
        //     routes,
        //     vec![
        //         &Route {
        //             path: "/".into(),
        //             method: HttpMethod::Get
        //         },
        //         &Route {
        //             path: "/".into(),
        //             method: HttpMethod::Get
        //         },
        //         &Route {
        //             path: "/".into(),
        //             method: HttpMethod::Get
        //         }
        //     ]
        // );
    }
}
