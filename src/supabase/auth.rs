use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SignUpResponse {
    pub id: String,
    pub app_metadata: Value,
    pub user_metadata: Value,
    pub aud: String,
    pub confirmation_sent_at: Option<String>,
    pub recovery_sent_at: Option<String>,
    pub email_change_sent_at: Option<String>,
    pub new_email: Option<String>,
    pub invited_at: Option<String>,
    pub action_link: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: String,
    pub confirmed_at: Option<String>,
    pub email_confirmed_at: Option<String>,
    pub phone_confirmed_at: Option<String>,
    pub last_sign_in_at: Option<String>,
    pub role: Option<String>,
    pub updated_at: Option<String>,
    pub identities: Option<Vec<UserIdentity>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UserIdentity {
    pub id: String,
    pub user_id: String,
    pub identity_data: Value,
    pub provider: String,
    pub created_at: String,
    pub last_sign_in_at: String,
    pub updated_at: Option<String>,
}

pub struct AuthClient {
    client: reqwest::Client,
}

impl AuthClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn sign_up<T, U>(
        &self,
        email: T,
        password: U,
    ) -> Result<SignUpResponse, reqwest::Error>
    where
        T: Serialize,
        U: Serialize,
    {
        let apikey = dotenv::var("SUPABASE_API_KEY").expect("missing supabase apikey");
        let url = dotenv::var("SUPABASE_AUTH_ENDPOINT").expect("missing supabase auth endpoint");
        let url = format!("{}/signup", url);

        let res = self
            .client
            .post(url)
            .header("apikey", apikey)
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<SignUpResponse>()
            .await?;
        Ok(res)
    }
}
