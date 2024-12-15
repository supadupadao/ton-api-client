pub type TonApiResult<T, E = TonApiError> = Result<T, E>;

#[derive(Debug)]
pub enum TonApiError {
    UnknownError(String), // TODO
}
