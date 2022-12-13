use crate::proto::rating::{create::NewRatingData, RatingData};
use crate::supabase::{
    self, rpc::RatingRpc, ClientError, InternalErrorKind, PostgrestError, Schema,
};

use postgrest::Builder;
use serde_json::json;

#[derive(Default)]
pub struct RatingClient {
    client: supabase::Client,
}

#[tonic::async_trait]
impl Schema for RatingClient {
    type Method = RatingRpc;

    fn table(&self) -> Builder {
        self.client.from("ratings")
    }

    async fn rpc<T: Into<String> + std::marker::Send>(
        &self,
        method: Self::Method,
        params: T,
    ) -> Result<reqwest::Response, ClientError> {
        self.client.rpc(method, params).await
    }
}

impl RatingClient {
    pub fn new() -> Self {
        Self {
            client: supabase::Client::new(),
        }
    }

    pub async fn create_for_requestor(
        &self,
        rating: NewRatingData,
    ) -> Result<RatingData, ClientError> {
        let NewRatingData {
            request_id,
            author,
            value,
            comment,
        } = rating;

        let res = self
            .rpc(
                RatingRpc::CreateForRequestor,
                json!({
                    "_request_id": request_id,
                    "_author": author,
                    "_value": value,
                    "_comment": comment,
                })
                .to_string(),
            )
            .await?;

        let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn create_for_provider(
        &self,
        rating: NewRatingData,
    ) -> Result<RatingData, ClientError> {
        let NewRatingData {
            request_id,
            author,
            value,
            comment,
        } = rating;

        let res = self
            .rpc(
                RatingRpc::CreateForProvider,
                json!({
                    "_request_id": request_id,
                    "_author": author,
                    "_value": value,
                    "_comment": comment,
                })
                .to_string(),
            )
            .await?;

        let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn get<T, U>(&self, column: T, filter: U) -> Result<Vec<RatingData>, ClientError>
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
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
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

    pub async fn get_for_request<T: AsRef<str>>(
        &self,
        request_id: T,
    ) -> Result<Vec<RatingData>, ClientError> {
        let res = self
            .table()
            .eq("request_id", request_id)
            .execute()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
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

    pub async fn get_by_id<T, U>(
        &self,
        request_id: T,
        rating_for: U,
    ) -> Result<Vec<RatingData>, ClientError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let res = self
            .table()
            .eq("request_id", request_id)
            .eq("rating_for", rating_for)
            .execute()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
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

    pub async fn update<T, U, V>(
        &self,
        request_id: T,
        rating_for: U,
        body: V,
    ) -> Result<Vec<RatingData>, ClientError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
        V: Into<String>,
    {
        let res = self
            .table()
            .eq("request_id", request_id)
            .eq("rating_for", rating_for)
            .update(body)
            .execute()
            .await
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
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

    pub async fn delete<T, U>(&self, request_id: T, rating_for: U) -> Result<(), ClientError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let res = self
            .table()
            .eq("request_id", request_id)
            .eq("rating_for", rating_for)
            .delete()
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
            Ok(())
        }
    }
}
