struct Response<'a, T> {
    code: uint,
    reason: Option<SendStr>,
    headers: Vec<HeaderField>,
    body: T,
}
