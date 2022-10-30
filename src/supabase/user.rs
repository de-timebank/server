use super::{error_for_status, ClientErrorKind, SupabaseClient};
use crate::proto::timebank::user::UserProfile;

use postgrest::Builder;

pub struct UserClient {
    client: SupabaseClient,
}

impl UserClient {
    pub fn new() -> Self {
        Self {
            client: SupabaseClient::new(),
        }
    }

    pub async fn get<T, U>(&self, column: T, filter: U) -> Result<Vec<UserProfile>, ClientErrorKind>
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
            .json::<Vec<UserProfile>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values)
    }

    pub async fn update<T, U>(&self, user_id: T, body: U) -> Result<UserProfile, ClientErrorKind>
    where
        T: AsRef<str>,
        U: Into<String>,
    {
        let res = self
            .table()
            .eq("user_id", user_id)
            .update(body)
            .execute()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<UserProfile>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    fn table(&self) -> Builder {
        self.client.from("user_profile")
    }
}
