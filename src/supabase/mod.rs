pub mod auth;
pub mod rating;
pub(self) mod rpc;
pub mod service_request;
pub mod user;

use core::fmt;
use postgrest::{Builder, Postgrest};
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug)]
pub enum ClientErrorKind {
    SupabaseError(SupabaseError),
    InternalError(Box<dyn std::error::Error + 'static>),
}

impl fmt::Display for ClientErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientErrorKind::SupabaseError(e) => e.fmt(f),
            ClientErrorKind::InternalError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl std::error::Error for ClientErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientErrorKind::SupabaseError(e) => Some(e),
            ClientErrorKind::InternalError(e) => Some(e.as_ref()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SupabaseError {
    pub code: String,
    pub details: Option<String>,
    pub hint: Option<String>,
    pub message: Option<String>,
}

impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "supabase error : {}",
            self.message
                .as_ref()
                .unwrap_or(&String::from("unknown error"))
        )
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

    async fn rpc<T, U>(&self, function: T, params: U) -> Result<Response, reqwest::Error>
    where
        T: rpc::RpcMethod,
        U: Into<String>,
    {
        self.postgrest_client
            .rpc(function.name(), params)
            .execute()
            .await?
            .error_for_status()
        // .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        // if !res.status().is_success() {
        //     let err = res
        //         .json::<SupabaseError>()
        //         .await
        //         .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;
        //     Err(ClientErrorKind::SupabaseError(err))
        // } else {
        //     Ok(res)
        // }
    }

    fn from<T>(&self, table: T) -> Builder
    where
        T: AsRef<str>,
    {
        self.postgrest_client.from(table)
    }
}

// A utility function for converting a response into an error if its an error-type response.
// pub(self) async fn error_for_status(res: Response) -> Result<Response, ClientErrorKind> {
//     if res.status().is_client_error() || res.status().is_server_error() {
//         let code = res.status();
//         let body = res
//             .json::<String>()
//             .await
//             .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;
//         Err(ClientErrorKind::RequestError { code, body })
//     } else {
//         Ok(res)
//     }
// }
