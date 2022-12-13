pub mod auth;
pub mod graphql;
pub mod rating;
pub(self) mod rpc;
pub mod service_request;
pub mod user;

use core::fmt;
use postgrest::{Builder, Postgrest};
use reqwest::Response;
use serde::{Deserialize, Serialize};

use self::rpc::RpcMethod;

#[derive(Debug)]
pub enum InternalErrorKind {
    ParsingError(String),
    RequestError(String),
}

impl std::error::Error for InternalErrorKind {}

impl fmt::Display for InternalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalErrorKind::ParsingError(s) => write!(f, "parsing error : {s}"),
            InternalErrorKind::RequestError(s) => write!(f, "request error : {s}"),
        }
    }
}

#[derive(Debug)]
pub enum ClientError {
    SupabaseError(PostgrestError),
    InternalError(InternalErrorKind),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::SupabaseError(e) => e.fmt(f),
            ClientError::InternalError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientError::SupabaseError(e) => Some(e),
            ClientError::InternalError(e) => Some(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgrestError {
    pub code: String,
    pub details: Option<String>,
    pub hint: Option<String>,
    pub message: Option<String>,
}

impl fmt::Display for PostgrestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl std::error::Error for PostgrestError {}

pub(self) struct Client {
    postgrest: Postgrest,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    fn new() -> Self {
        let uri = dotenv::var("SUPABASE_ENDPOINT").expect("MISSING SUPABASE POSTGREST ENDPOINT!");
        let apikey = dotenv::var("SUPABASE_API_KEY").expect("MISSING SUPABASE API KEY!");

        Self {
            postgrest: Postgrest::new(uri)
                .insert_header("apikey", &apikey)
                .insert_header("Authorization", format!("Bearer {apikey}")),
        }
    }

    async fn rpc<T, U>(&self, function: T, params: U) -> Result<Response, ClientError>
    where
        T: rpc::RpcMethod,
        U: Into<String>,
    {
        let res = self
            .postgrest
            .rpc(function.name(), params)
            .execute()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if !res.status().is_success() {
            let err = res.json::<PostgrestError>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientError::SupabaseError(err))
        } else {
            Ok(res)
        }
    }

    fn from<T>(&self, table: T) -> Builder
    where
        T: AsRef<str>,
    {
        self.postgrest.from(table)
    }
}

// what to have in the `schema` trait
// - table() - to access the schema postgres table
// - rpc() - to access rpc methods related to that schema (based on the naming of the rpc methods)

#[tonic::async_trait]
trait Schema {
    type Method: RpcMethod;

    fn table(&self) -> Builder;

    async fn rpc<T: Into<String> + std::marker::Send>(
        &self,
        method: Self::Method,
        params: T,
    ) -> Result<Response, ClientError>;
}
