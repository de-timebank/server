use crate::proto::user::{NewUserProfile, ProfileSummary, UserProfile};
use crate::supabase::{self, rpc::UserRpc, ClientError, InternalErrorKind, PostgrestError};

use postgrest::Builder;
use serde::Serialize;
use serde_json::json;

pub struct UserClient {
    client: supabase::Client,
}

impl UserClient {
    pub fn new() -> Self {
        Self {
            client: supabase::Client::new(),
        }
    }

    pub async fn get<T, U>(&self, column: T, filter: U) -> Result<Vec<UserProfile>, ClientError>
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

        let values = res.json::<Vec<UserProfile>>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(values)
    }

    pub async fn update<T, U>(&self, user_id: T, body: U) -> Result<UserProfile, ClientError>
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
            .map_err(|e| {
                ClientError::InternalError(InternalErrorKind::RequestError(e.to_string()))
            })?;

        if res.status().is_success() {
            let values = res.json::<Vec<UserProfile>>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Ok(values.into_iter().next().unwrap_or_default())
        } else {
            let err = res.json::<PostgrestError>().await.map_err(|e| {
                ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
            })?;

            Err(ClientError::SupabaseError(err))
        }
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<ProfileSummary, ClientError> {
        let res = self
            .client
            .rpc(
                UserRpc::GetProfile,
                json!({ "_user_id": user_id }).to_string(),
            )
            .await?;

        res.json::<ProfileSummary>()
            .await
            .map_err(|e| ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string())))
    }

    pub(crate) async fn create_new_profile<T>(
        &self,
        user_id: T,
        profile: NewUserProfile,
    ) -> Result<Vec<UserProfile>, ClientError>
    where
        T: Serialize,
    {
        let res = self
            .client
            .rpc(
                UserRpc::HandleNewUser,
                json!({
                    "_user_id": user_id,
                    "_profile": profile
                })
                .to_string(),
            )
            .await?;

        res.json::<Vec<UserProfile>>()
            .await
            .map_err(|e| ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string())))
    }

    pub async fn check_if_email_exist<T>(&self, id: T) -> Result<bool, ClientError>
    where
        T: Serialize,
    {
        let res = self
            .client
            .rpc(
                UserRpc::CheckIfEmailExist,
                json!({ "_email": id }).to_string(),
            )
            .await?;

        let value = res.json::<bool>().await.map_err(|e| {
            ClientError::InternalError(InternalErrorKind::ParsingError(e.to_string()))
        })?;

        Ok(value)
    }

    fn table(&self) -> Builder {
        self.client.from("profiles")
    }
}
