use super::{ClientErrorKind, SupabaseClient};
use crate::proto::timebank::user::{NewUserProfile, UserProfile};

use postgrest::Builder;
use serde::Serialize;
use serde_json::json;

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

        // let res = error_for_status(res).await?;
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

        // let res = error_for_status(res).await?;
        let values = res
            .json::<Vec<UserProfile>>()
            .await
            .map_err(|e| ClientErrorKind::InternalError(Box::new(e)))?;

        Ok(values.into_iter().next().unwrap_or_default())
    }

    pub(crate) async fn create_profile<T>(
        &self,
        user_id: T,
        profile: NewUserProfile,
    ) -> Result<UserProfile, reqwest::Error>
    where
        T: AsRef<str> + Serialize + Copy + Clone,
    {
        let res = self
            .table()
            .eq("user_id", user_id)
            .insert(
                json!([{
                    "user_id": user_id,
                    "name": profile.name,
                    "gender": profile.gender,
                    "matric_number": profile.matric_number,
                    "skills": profile.skills,
                    "contacts":profile.contacts
                }])
                .to_string(),
            )
            .execute()
            .await?
            .error_for_status()?
            .json::<Vec<UserProfile>>()
            .await?;

        Ok(res.into_iter().next().unwrap_or_default())
    }

    fn table(&self) -> Builder {
        self.client.from("user_profile")
    }
}
