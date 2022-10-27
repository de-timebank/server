use super::rpc::ServiceRequestRpc;
use super::SupabaseClient;
use crate::proto::timebank::servicerequest::create;

use postgrest::Builder;
use reqwest::{Error, Response};
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

    async fn create(
        &self,
        requestor: &str,
        request_data: create::TNewServiceRequest,
    ) -> Result<Response, Error> {
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

    async fn delete(&self, request_id: &str) -> Result<Response, Error> {
        self.client
            .rpc(
                ServiceRequestRpc::Delete,
                json!({ "_request_id": request_id }).to_string(),
            )
            .execute()
            .await
    }

    async fn select_bid(&self) -> Result<Response, Error> {
        self.client
            .rpc(ServiceRequestRpc::SelectBid, json!({}).to_string())
            .execute()
            .await
    }

    async fn complete_service(&self) -> Result<Response, Error> {
        self.client
            .rpc(ServiceRequestRpc::CompleteService, json!({}).to_string())
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

    async fn get_commitment_of<T>(&self, id: T)
    where
        T: AsRef<str>,
    {
        todo!()
    }

    async fn get_by_id<T>(&self, id: T)
    where
        T: AsRef<str>,
    {
        todo!()
    }

    fn table(&self) -> Builder {
        self.client.from("service_request")
    }
}
