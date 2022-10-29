use super::{error_for_status, rpc::RatingRpc, ClientErrorKind, SupabaseClient};
use crate::proto::timebank::rating::{create::NewRatingData, RatingData};

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

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<RatingData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

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

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<RatingData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

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
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<RatingData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values)
    }

    pub async fn update<T, U>(&self, id: T, body: U) -> Result<RatingData, ClientErrorKind>
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
            .json::<Vec<RatingData>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub async fn delete<T>(&self, id: T) -> Result<(), ClientErrorKind>
    where
        T: AsRef<str>,
    {
        let res = self
            .table()
            .eq("id", id)
            .delete()
            .execute()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;
        error_for_status(res).await?;
        Ok(())
    }

    fn table(&self) -> Builder {
        self.client.postgrest_client.from("rating")
    }
}
