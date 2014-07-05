extern crate url;

use url::Url;

// rfc7231, section 4
enum StandardizedRequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl StandardizedRequestMethod {
    fn to_str(&self) {
        match *self {
            GET => "GET",
            HEAD => "HEAD",
            POST => "POST",
            PUT => "PUT",
            DELETE => "DELETE",
            CONNECT => "CONNECT",
            OPTIONS => "OPTIONS",
            TRACE => "TRACE",
        }
    }
}


struct Request<T> {
    method: SendStr,
    target: SendStr,
    version: HttpVersion,
    header: Header,
    body: T,
}
