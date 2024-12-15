use url::Url;

#[derive(Debug)]
pub enum Server {
    MainNet,
    TestNet,
    Custom(Url),
}

impl Default for Server {
    fn default() -> Server {
        Server::MainNet
    }
}
