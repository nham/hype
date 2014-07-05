struct Response<T> {
    code: uint,
    reason: Option<SendStr>,
    version: HttpVersion,
    headers: Header,
    body: T,
}
