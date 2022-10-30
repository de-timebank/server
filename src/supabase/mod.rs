pub mod rating;
pub(self) mod rpc;
pub mod service_request;
pub mod user;

use core::fmt;
use postgrest::{Builder, Postgrest};
use reqwest::Response;

#[derive(Debug)]
pub enum ClientErrorKind {
    RequestError {
        code: reqwest::StatusCode,
        body: String,
    },
    InternalError(Box<dyn std::error::Error + 'static>),
}

impl fmt::Display for ClientErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientErrorKind::RequestError { .. } => {
                write!(f, "request error")
            }
            ClientErrorKind::InternalError(_) => {
                write!(f, "internal error")
            }
        }
    }
}

impl std::error::Error for ClientErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientErrorKind::RequestError { .. } => None,
            ClientErrorKind::InternalError(e) => Some(e.as_ref()),
        }
    }
}

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
        self.postgrest_client
            .rpc(function.name(), params)
            .execute()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))
    }

    fn from<T>(&self, table: T) -> Builder
    where
        T: AsRef<str>,
    {
        self.postgrest_client.from(table)
    }
}

/// A utility function for converting a response into an error if its an error-type response.
pub(self) async fn error_for_status(res: Response) -> Result<Response, ClientErrorKind> {
    if res.status().is_client_error() || res.status().is_server_error() {
        let code = res.status();
        let body = res
            .json::<String>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;
        Err(ClientErrorKind::RequestError { code, body })
    } else {
        Ok(res)
    }
}
