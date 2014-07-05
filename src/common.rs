// rfc7230, section 3.1:
//     start-line     = request-line / status-line
//
// rfc7230, section 3.1.1:
//     request-line   = method SP request-target SP HTTP-version CRLF

// rfc7230, section 2.6:
//     HTTP-version  = HTTP-name "/" DIGIT "." DIGIT
//     HTTP-name     = %x48.54.54.50 ; "HTTP", case-sensitive

// rfc7230 section 3.1.2:
//     status-line = HTTP-version SP status-code SP reason-phrase CRLF
//
//     status-code    = 3DIGIT
//     reason-phrase  = *( HTAB / SP / VCHAR / obs-text )
//
// rfc5234
//     HTAB           =  %x09
//                                ; horizontal tab
//
//     SP             =  %x20
//
//     VCHAR          =  %x21-7E
//                                ; visible (printing) characters

// rfc7230 section 3.2.6:
//     obs-text       = %x80-FF

struct HttpVersion {
    major: u8,
    minor: u8,
}

impl HttpVersion {
    // TODO: is String what we want here?
    fn render(&self) -> String {
        format!("HTTP/{}.{}", self.major, self.minor)
    }

}

// rfc7230, section 3.2
//     header-field   = field-name ":" OWS field-value OWS

//     field-name     = token
//     field-value    = *( field-content / obs-fold )
//     field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
//     field-vchar    = VCHAR / obs-text

//     obs-fold       = CRLF 1*( SP / HTAB )
//                    ; obsolete line folding

// rfc7230, section 3.2.3
//     OWS            = *( SP / HTAB )
//                    ; optional whitespace

// rfc 7230, section 3.2.6
//     token          = 1*tchar

//     tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*"
//                    / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
//                    / DIGIT / ALPHA
//                    ; any VCHAR, except delimiters

// rfc7230, section 3.2 
//   Each header field consists of a case-insensitive field name followed
//   by a colon (":"), optional leading whitespace, the field value, and
//   optional trailing whitespace.
struct Header {
    fields: HashMap<SendStr, SendStr>,
}

// rfc7230, section 3
//
//     HTTP-message   = start-line
//                      *( header-field CRLF )
//                      CRLF
//                      [ message-body ]


/* From 3.1.1:
 
     Recipients of an invalid request-line SHOULD respond with either a
   400 (Bad Request) error or a 301 (Moved Permanently) redirect with
   the request-target properly encoded.  A recipient SHOULD NOT attempt
   to autocorrect and then process the request without a redirect, since
   the invalid request-line might be deliberately crafted to bypass
   security filters along the request chain.

   TODO
*/

