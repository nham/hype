extern crate url;

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
    target: RequestTarget,
    version: HttpVersion,
    header: Header,
    body: T,
}

enum RequestTarget {
    OriginForm(Vec<String>),
    AbsoluteForm(AbsoluteURI),
    AuthorityForm(Authority),
    AsteriskForm,
}

struct AbsoluteURI {
    scheme: String,
    hier_part: HierPart,
    query: Option<String>,
}

enum HierPart {
    WithAuthority(Authority, Vec<String>),
    Absolute(Vec<String>),
    Rootless(String, Vec<String>),
    Empty,
}

struct Authority {
    user: Option<String>,
    host: String,
    port: Option<uint>,
}

/* rfc7230 section 5.3

     request-target = origin-form
                    / absolute-form
                    / authority-form
                    / asterisk-form

     origin-form    = absolute-path [ "?" query ]
     absolute-form  = absolute-URI
     authority-form = authority
     asterisk-form  = "*"

     absolute-path = 1*( "/" segment )
     segment = <segment, see [RFC3986], Section 3.3>
     query = <query, see [RFC3986], Section 3.4>

     absolute-URI = <absolute-URI, see [RFC3986], Section 4.3>
     authority = <authority, see [RFC3986], Section 3.2>
*/

/* rfc3986

     absolute-URI  = scheme ":" hier-part [ "?" query ]
     scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

     hier-part     = "//" authority path-abempty
                   / path-absolute
                   / path-rootless
                   / path-empty

     authority     = [ userinfo "@" ] host [ ":" port ]

     userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
     host          = IP-literal / IPv4address / reg-name
     port          = *DIGIT

     IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"

     path-abempty  = *( "/" segment )
     path-absolute = "/" [ segment-nz *( "/" segment ) ]
     path-rootless = segment-nz *( "/" segment )
     path-empty    = 0<pchar>

     segment       = *pchar
     segment-nz    = 1*pchar
     pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
     unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
     pct-encoded   = "%" HEXDIG HEXDIG
     sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
                   / "*" / "+" / "," / ";" / "="

     query         = *( pchar / "/" / "?" )

     http-URI = "http:" "//" authority path-abempty [ "?" query ]
                [ "#" fragment ]
*/
