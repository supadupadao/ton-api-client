use url::Url;

#[derive(Debug, Default)]
pub enum Server {
    #[default]
    MainNet,
    TestNet,
    Custom(Url),
}
