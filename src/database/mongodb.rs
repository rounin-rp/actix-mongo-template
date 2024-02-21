use crate::handlers::error_handler::Errors;
use mongodb::Client;

#[derive(Default, Clone)]
pub struct Url(String);

impl Url {
    pub fn new(url: impl Into<String>) -> Self {
        Self(url.into())
    }
}

#[derive(Default, Clone)]
pub struct NoUrl;

#[derive(Default, Clone)]
pub struct DbName(String);

impl DbName {
    pub fn new(db_name: impl Into<String>) -> Self {
        Self(db_name.into())
    }
}

#[derive(Default, Clone)]
pub struct NoDbName;

#[derive(Clone)]
pub struct MongoClient {
    pub url: String,
    pub db_name: String,
    pub client: Client,
}

#[derive(Default, Clone)]
pub struct MongoClientBuilder<U, D> {
    pub url: U,
    pub db_name: D,
    pub client: Option<Client>,
}

impl MongoClientBuilder<NoUrl, NoDbName> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MongoClientBuilder<Url, DbName> {
    pub async fn url(url: Url, db_name: DbName) -> Result<Self, Errors> {
        let client = Client::with_uri_str(url.0.clone())
            .await
            .map_err(|_| Errors::InternalError(String::from("Invalid Mongo Uri!")))?;

        Ok(Self {
            client: Some(client),
            db_name,
            url,
        })
    }

    pub fn build(self) -> Result<MongoClient, Errors> {
        Ok(MongoClient {
            url: self.url.0,
            db_name: self.db_name.0,
            client: self.client.unwrap(),
        })
    }
}
