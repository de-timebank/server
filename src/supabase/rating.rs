use super::{rpc::RatingRpc, SupabaseClient};
use crate::proto::timebank::rating::create::NewRatingData;

use postgrest::Builder;
use reqwest::{Error, Response};
use serde::Serialize;
use serde_json::json;

pub struct Rating {
    client: SupabaseClient,
}

impl Rating {
    fn new() -> Self {
        Self {
            client: SupabaseClient::new(),
        }
    }

    async fn createForRequestor(&self, rating: NewRatingData) -> Result<Response, Error> {
        let NewRatingData {
            request_id,
            author,
            value,
            comment,
        } = rating;

        self.client
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
            .execute()
            .await
    }

    async fn createForProvider(&self, rating: NewRatingData) -> Result<Response, Error> {
        let NewRatingData {
            request_id,
            author,
            value,
            comment,
        } = rating;

        self.client
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
        T: AsRef<str>,
    {
        self.table().eq("id", id).delete().execute().await
    }

    fn table(&self) -> Builder {
        self.client.postgrest_client.from("service_rating")
    }
}
