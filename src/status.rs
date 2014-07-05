// From http://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
// Retried 04 July 2014
//
// Also added status code 418 from RFC2324, because how can you not?

enum StandardStatus {
    Continue = 100, // RFC7231, Section 6.2.1
    SwitchingProtocols = 101, // RFC7231, Section 6.2.2
    Processing = 102, // RFC2518

    OK = 200, // RFC7231, Section 6.3.1
    Created = 201, // RFC7231, Section 6.3.2
    Accepted = 202, // RFC7231, Section 6.3.3
    NonAuthoritativeInformation = 203, // RFC7231, Section 6.3.4
    NoContent = 204, // RFC7231, Section 6.3.5
    ResetContent = 205, // RFC7231, Section 6.3.6
    PartialContent = 206, // RFC7233, Section 4.1
    MultiStatus = 207, // RFC4918
    AlreadyReported = 208, // RFC5842
    IMUsed = 226, // RFC3229

    MultipleChoices = 300, // RFC7231, Section 6.4.1
    MovedPermanently = 301, // RFC7231, Section 6.4.2
    Found = 302, // RFC7231, Section 6.4.3
    SeeOther = 303, // RFC7231, Section 6.4.4
    NotModified = 304, // RFC7232, Section 4.1
    UseProxy = 305, // RFC7231, Section 6.4.5
    //(Unused) = 306, // RFC7231, Section 6.4.6
    TemporaryRedirect = 307, // RFC7231, Section 6.4.7
    PermanentRedirect = 308, // RFC7238

    BadRequest = 400, // RFC7231, Section 6.5.1
    Unauthorized = 401, // RFC7235, Section 3.1
    PaymentRequired = 402, // RFC7231, Section 6.5.2
    Forbidden = 403, // RFC7231, Section 6.5.3
    NotFound = 404, // RFC7231, Section 6.5.4
    MethodNotAllowed = 405, // RFC7231, Section 6.5.5
    NotAcceptable = 406, // RFC7231, Section 6.5.6
    ProxyAuthenticationRequired = 407, // RFC7235, Section 3.2
    RequestTimeout = 408, // RFC7231, Section 6.5.7
    Conflict = 409, // RFC7231, Section 6.5.8
    Gone = 410, // RFC7231, Section 6.5.9
    LengthRequired = 411, // RFC7231, Section 6.5.10
    PreconditionFailed = 412, // RFC7232, Section 4.2
    PayloadTooLarge = 413, // RFC7231, Section 6.5.11
    URITooLong = 414, // RFC7231, Section 6.5.12
    UnsupportedMediaType = 415, // RFC7231, Section 6.5.13
    RequestedRangeNotSatisfiable = 416, // RFC7233, Section 4.4
    ExpectationFailed = 417, // RFC7231, Section 6.5.14
    ImATeapot = 418, // RFC2324
    UnprocessableEntity = 422, // RFC4918
    Locked = 423, // RFC4918
    FailedDependency = 424, // RFC4918
    UpgradeRequired = 426, // RFC7231, Section 6.5.15
    PreconditionRequired = 428, // RFC6585
    TooManyRequests = 429, // RFC6585
    RequestHeaderFieldsTooLarge = 431, // RFC6585

    InternalServerError = 500, // RFC7231, Section 6.6.1
    NotImplemented = 501, // RFC7231, Section 6.6.2
    BadGateway = 502, // RFC7231, Section 6.6.3
    ServiceUnavailable = 503, // RFC7231, Section 6.6.4
    GatewayTimeout = 504, // RFC7231, Section 6.6.5
    HTTPVersionNotSupported = 505, // RFC7231, Section 6.6.6
    VariantAlsoNegotiates = 506, // RFC2295
    InsufficientStorage = 507, // RFC4918
    LoopDetected = 508, // RFC5842
    NotExtended = 510, // RFC2774
    NetworkAuthenticationRequired = 511, // RFC6585
}
