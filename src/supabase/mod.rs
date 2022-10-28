pub mod rating;
pub(self) mod rpc;
pub mod service_request;

use postgrest::{Builder, Postgrest};

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

    fn rpc<T, U>(&self, function: T, params: U) -> Builder
    where
        T: rpc::RpcMethod,
        U: Into<String>,
    {
        self.postgrest_client.rpc(function.name(), params)
    }

    fn from<T>(&self, table: T) -> Builder
    where
        T: AsRef<str>,
    {
        self.postgrest_client.from(table)
    }
}
