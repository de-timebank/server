use super::{rpc::ServiceRequestRpc, SupabaseClient};
use crate::proto::timebank::servicerequest::create;

use postgrest::Builder;
use reqwest::{Error, Response};
use serde::Serialize;
use serde_json::json;

pub struct ServiceRequest {
    client: SupabaseClient,
}

impl ServiceRequest {
    fn new() -> Self {
        Self {
            client: SupabaseClient::new(),
        }
    }

    async fn create<T>(
        &self,
        requestor: T,
        request_data: create::NewServiceRequestData,
    ) -> Result<Response, Error>
    where
        T: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::Create,
                json!({
                    "_requestor": requestor,
                    "_request": request_data
                })
                .to_string(),
            )
            .execute()
            .await
    }

    async fn get<T, U>(&self, column: T, filter: U) -> Result<Response, Error>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        self.table().eq(column, filter).execute().await
    }

    async fn update<T, U>(&self, id: T, body: U) -> Result<Response, Error>
    where
        T: AsRef<str>,
        U: Into<String>,
    {
        self.table().eq("id", id).update(body).execute().await
    }

    async fn delete<T>(&self, id: T) -> Result<Response, Error>
    where
        T: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::Delete,
                json!({ "_request_id": id }).to_string(),
            )
            .execute()
            .await
    }

    async fn apply_as_provider<T, U>(&self, id: T, provider: U) -> Result<Response, Error>
    where
        T: Serialize,
        U: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::ApplyProvider,
                json!({
                    "_request_id": id,
                    "_provider": provider
                })
                .to_string(),
            )
            .execute()
            .await
    }

    async fn select_provider<T, U, V>(&self, id: T, provider: U, user: V) -> Result<Response, Error>
    where
        T: Serialize,
        U: Serialize,
        V: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::SelectProvider,
                json!({
                    "_caller": user,
                    "_request_id": id,
                    "_provider": provider,
                })
                .to_string(),
            )
            .execute()
            .await
    }

    async fn complete_service<T, U>(&self, id: T, requestor: U) -> Result<Response, Error>
    where
        T: Serialize,
        U: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::CompleteService,
                json!({
                    "_request_id": id,
                    "_user_id": requestor,
                })
                .to_string(),
            )
            .execute()
            .await
    }

    async fn get_by_id<T>(&self, id: T) -> Result<Response, Error>
    where
        T: AsRef<str>,
    {
        self.get("id", id).await
    }

    async fn get_commitment_of<T>(&self, id: T)
    where
        T: AsRef<str>,
    {
        todo!()
    }

    fn table(&self) -> Builder {
        self.client.from("service_request")
    }
}
