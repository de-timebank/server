use crate::proto::servicerequest::{create, get_by_id, get_summary_for_user, ServiceRequestData};
use crate::supabase::{
    self, rpc::ServiceRequestRpc, ClientError, InternalErrorKind, PostgrestError,
};

use postgrest::Builder;
use serde::Serialize;
use serde_json::json;

pub struct ServiceRequestClient {
    client: supabase::Client,
}

impl ServiceRequestClient {
    pub fn new() -> Self {
        Self {
            client: supabase::Client::new(),
        }
    }

    pub async fn create<T>(
        &self,
        requestor: T,
        request_data: create::NewServiceRequestData,
    ) -> Result<ServiceRequestData, ClientError>
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

        let values = res.json::<Vec<ServiceRequestData>>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn get<T, U>(
        &self,
        column: T,
        filter: U,
    ) -> Result<Vec<ServiceRequestData>, ClientError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let res = self
            .table()
            .eq(column, filter)
            .execute()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<ServiceRequestData>>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values)
        } else {
            let err = res.json::<PostgrestError>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientError::SupabaseError(err))
        }
    }

    pub async fn get_by_id<T>(&self, request_id: T) -> Result<get_by_id::Response, ClientError>
    where
        T: Serialize,
    {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::GetById,
                json!({
                    "_request_id": request_id,
                })
                .to_string(),
            )
            .await?;

        let value = res.json::<get_by_id::Response>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(value)
    }

    pub async fn update<T, U>(&self, id: T, body: U) -> Result<Vec<ServiceRequestData>, ClientError>
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
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<ServiceRequestData>>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values)
        } else {
            let err = res.json::<PostgrestError>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientError::SupabaseError(err))
        }
    }

    pub async fn delete<T>(&self, id: T) -> Result<(), ClientError>
    where
        T: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::Delete,
                json!({ "_request_id": id }).to_string(),
            )
            .await?;
        Ok(())
    }

    pub async fn apply_as_provider<T, U>(&self, id: T, provider: U) -> Result<(), ClientError>
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
            .await?;
        Ok(())
    }

    pub async fn select_provider<T, U, V>(
        &self,
        id: T,
        provider: U,
        user: V,
    ) -> Result<(), ClientError>
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
            .await?;
        Ok(())
    }

    pub async fn start_service<T, U>(&self, request_id: T, user_id: U) -> Result<(), ClientError>
    where
        T: Serialize,
        U: Serialize,
    {
        self.client
            .rpc(
                ServiceRequestRpc::StartService,
                json!({
                    "_request_id": request_id,
                    "_user_id": user_id
                })
                .to_string(),
            )
            .await?;
        Ok(())
    }

    pub async fn complete_service<T, U>(&self, id: T, requestor: U) -> Result<(), ClientError>
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
            .await?;
        Ok(())
    }

    /// Fetch all service requests that is in the pending state.
    pub async fn get_available<T, U>(
        &self,
        filter_by: T,
        filter_value: U,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<ServiceRequestData>, ClientError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let res = self
            .table()
            .select("*")
            .eq("state", "0")
            .eq(filter_by, filter_value)
            .range(offset, offset + limit)
            .execute()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        let values = res.json::<Vec<ServiceRequestData>>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values)
    }

    fn table(&self) -> Builder {
        self.client.from("service_requests")
    }

    pub async fn get_summary_for_user<T: Serialize>(
        &self,
        user_id: T,
    ) -> Result<get_summary_for_user::Response, ClientError> {
        let res = self
            .client
            .rpc(
                ServiceRequestRpc::GetSummaryForUser,
                json!({ "_user_id": user_id }).to_string(),
            )
            .await?
            .json::<get_summary_for_user::Response>()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

        Ok(res)
    }
}
