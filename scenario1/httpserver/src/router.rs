use super::handler::{
    Handler, PageNotFoundHandler,
    StaticPageHandler, WebServiceHandler,
};
use http::{httprequest::{self, HttpRequest, Method, Resource, Version},
    httpresponse::HttpResponse,
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::GET => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // handle api
                        "api" => {
                            let res: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = res.send_response(stream);
                        }
                        _ => {
                            let res: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = res.send_response(stream);
                        }
                    }
                }
            },
            // not GET request => send 404
            _ => {
                let res: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = res.send_response(stream);
            }
        }
    }
}
