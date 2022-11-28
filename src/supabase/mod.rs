pub mod auth;
pub mod rating;
pub(self) mod rpc;
pub mod service_request;
pub mod user;

use core::fmt;
use postgrest::{Builder, Postgrest};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum InternalErrorKind {
    ParsingError(String),
    RequestError(String),
}

impl std::error::Error for InternalErrorKind {}

impl fmt::Display for InternalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalErrorKind::ParsingError(s) => write!(f, "parsing error : {}", s),
            InternalErrorKind::RequestError(s) => write!(f, "request error : {}", s),
        }
    }
}

#[derive(Debug)]
pub enum ClientErrorKind {
    SupabaseError(SupabaseError),
    InternalError(InternalErrorKind),
}

impl fmt::Display for ClientErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientErrorKind::SupabaseError(e) => e.fmt(f),
            ClientErrorKind::InternalError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ClientErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientErrorKind::SupabaseError(e) => Some(e),
            ClientErrorKind::InternalError(e) => Some(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseError {
    pub code: String,
    pub details: Option<String>,
    pub hint: Option<String>,
    pub message: Option<String>,
}

impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl std::error::Error for SupabaseError {}

pub(self) struct SupabaseClient {
    postgrest_client: Postgrest,
}

impl SupabaseClient {
    fn new() -> Self {
        Self {
            postgrest_client: Postgrest::new(
                dotenv::var("SUPABASE_ENDPOINT").expect("MISSING SUPABASE POSTGREST ENDPOINT!"),
            )
            .insert_header(
                "apikey",
                dotenv::var("SUPABASE_API_KEY").expect("MISSING SUPABASE API KEY!"),
            ),
        }
    }

    async fn rpc<T, U>(&self, function: T, params: U) -> Result<Response, ClientErrorKind>
    where
        T: rpc::RpcMethod,
        U: Into<String>,
    {
        let res = self
            .postgrest_client
            .rpc(function.name(), params)
            .execute()
            .await
            .map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if !res.status().is_success() {
            let err = res.json::<SupabaseError>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientErrorKind::SupabaseError(err))
        } else {
            Ok(res)
        }
    }

    fn from<T>(&self, table: T) -> Builder
    where
        T: AsRef<str>,
    {
        self.postgrest_client.from(table)
    }
}
