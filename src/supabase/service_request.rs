use super::{error_for_status, rpc::ServiceRequestRpc, ClientErrorKind, SupabaseClient};
use crate::proto::timebank::servicerequest::{create, ServiceRequestData};

use postgrest::Builder;
use serde::Serialize;
use serde_json::json;

pub struct ServiceRequestClient {
    client: SupabaseClient,
}

impl ServiceRequestClient {
    pub fn new() -> Self {
        Self {
            client: SupabaseClient::new(),
        }
    }

    pub async fn create<T>(
        &self,
        requestor: T,
        request_data: create::NewServiceRequestData,
    ) -> Result<ServiceRequestData, ClientErrorKind>
    where
        T: Serialize,
    {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::Create,
                json!({
                    "_requestor": requestor,
                    "_request": request_data
                })
                .to_string(),
            )
            .await?;

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<ServiceRequestData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn get<T, U>(
        &self,
        column: T,
        filter: U,
    ) -> Result<Vec<ServiceRequestData>, ClientErrorKind>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let res = self
            .table()
            .eq(column, filter)
            .execute()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<ServiceRequestData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values)
    }

    pub async fn update<T, U>(&self, id: T, body: U) -> Result<ServiceRequestData, ClientErrorKind>
    where
        T: AsRef<str>,
        U: Into<String>,
    {
        let res = self
            .table()
            .eq("id", id)
            .update(body)
            .execute()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<ServiceRequestData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn delete<T>(&self, id: T) -> Result<(), ClientErrorKind>
    where
        T: Serialize,
    {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::Delete,
                json!({ "_request_id": id }).to_string(),
            )
            .await?;
        error_for_status(res).await?;
        Ok(())
    }

    pub async fn apply_as_provider<T, U>(&self, id: T, provider: U) -> Result<(), ClientErrorKind>
    where
        T: Serialize,
        U: Serialize,
    {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::ApplyProvider,
                json!({
                    "_request_id": id,
                    "_provider": provider
                })
                .to_string(),
            )
            .await?;
        error_for_status(res).await?;
        Ok(())
    }

    pub async fn select_provider<T, U, V>(
        &self,
        id: T,
        provider: U,
        user: V,
    ) -> Result<(), ClientErrorKind>
    where
        T: Serialize,
        U: Serialize,
        V: Serialize,
    {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::SelectProvider,
                json!({
                    "_caller": user,
                    "_request_id": id,
                    "_provider": provider,
                })
                .to_string(),
            )
            .await?;
        error_for_status(res).await?;
        Ok(())
    }

    pub async fn complete_service<T, U>(&self, id: T, requestor: U) -> Result<(), ClientErrorKind>
    where
        T: Serialize,
        U: Serialize,
    {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::CompleteService,
                json!({
                    "_request_id": id,
                    "_user_id": requestor,
                })
                .to_string(),
            )
            .await?;
        error_for_status(res).await?;
        Ok(())
    }

    #[allow(unused)]
    pub async fn get_commitment_of<T>(&self, id: T)
    where
        T: AsRef<str>,
    {
        todo!()
    }

    fn table(&self) -> Builder {
        self.client.from("service_request")
    }
}
