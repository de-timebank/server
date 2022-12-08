use super::{rpc::RatingRpc, ClientErrorKind, InternalErrorKind, SupabaseClient, SupabaseError};
use crate::proto::rating::{create::NewRatingData, RatingData};

use postgrest::Builder;
use serde_json::json;

pub struct RatingClient {
    client: SupabaseClient,
}

impl RatingClient {
    pub fn new() -> Self {
        Self {
            client: SupabaseClient::new(),
        }
    }

    pub async fn create_for_requestor(
        &self,
        rating: NewRatingData,
    ) -> Result<RatingData, ClientErrorKind> {
        let NewRatingData {
            request_id,
            author,
            value,
            comment,
        } = rating;

        let res = self
            .client
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
            ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn create_for_provider(
        &self,
        rating: NewRatingData,
    ) -> Result<RatingData, ClientErrorKind> {
        let NewRatingData {
            request_id,
            author,
            value,
            comment,
        } = rating;

        let res = self
            .client
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
            ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn get<T, U>(&self, column: T, filter: U) -> Result<Vec<RatingData>, ClientErrorKind>
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
                ClientErrorKind::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values)
        } else {
            let err = res.json::<SupabaseError>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientErrorKind::SupabaseError(err))
        }
    }

    pub async fn get_for_request<T: AsRef<str>>(
        &self,
        request_id: T,
    ) -> Result<Vec<RatingData>, ClientErrorKind> {
        let res = self
            .table()
            .eq("request_id", request_id)
            .execute()
            .await
            .map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values)
        } else {
            let err = res.json::<SupabaseError>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientErrorKind::SupabaseError(err))
        }
    }

    pub async fn get_by_id<T, U>(
        &self,
        request_id: T,
        rating_for: U,
    ) -> Result<Vec<RatingData>, ClientErrorKind>
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
                ClientErrorKind::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values)
        } else {
            let err = res.json::<SupabaseError>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientErrorKind::SupabaseError(err))
        }
    }

    pub async fn update<T, U, V>(
        &self,
        request_id: T,
        rating_for: U,
        body: V,
    ) -> Result<Vec<RatingData>, ClientErrorKind>
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
                ClientErrorKind::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<RatingData>>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values)
        } else {
            let err = res.json::<SupabaseError>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientErrorKind::SupabaseError(err))
        }
    }

    pub async fn delete<T, U>(&self, request_id: T, rating_for: U) -> Result<(), ClientErrorKind>
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
                ClientErrorKind::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if !res.status().is_success() {
            let err = res.json::<SupabaseError>().await.map_err(|e| {
                ClientErrorKind::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientErrorKind::SupabaseError(err))
        } else {
            Ok(())
        }
    }

    fn table(&self) -> Builder {
        self.client.postgrest_client.from("ratings")
    }
}
