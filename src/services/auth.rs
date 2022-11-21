use tonic::{Request, Response, Status};

use crate::proto::auth::auth_server::Auth;
pub use crate::proto::auth::auth_server::AuthServer;
use crate::proto::auth::sign_up;
use crate::services::Result;
use crate::supabase::ClientErrorKind;
use crate::supabase::{auth::AuthClient, user::UserClient};

pub struct AuthService {
    client: AuthClient,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            client: AuthClient::new(),
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_up(
        &self,
        request: Request<sign_up::Request>,
    ) -> Result<Response<sign_up::Response>> {
        let sign_up::Request {
            email,
            password,
            profile,
        } = request.into_inner();

        match (!email.is_empty(), !password.is_empty(), profile) {
            (true, true, Some(profile)) => {
                // 1. create new user
                let user = self
                    .client
                    .sign_up(email, password)
                    .await
                    .map_err(|e| Status::unknown(e.to_string()))?;

                // 2. create user profile
                let res = UserClient::new()
                    .create_new_profile(&user.id, profile)
                    .await;

                match res {
                    Ok(_) => Ok(Response::new(sign_up::Response { user_id: user.id })),

                    Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

                    Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
                }
            }

            (false, _, _) => Err(Status::invalid_argument("email cannot be empty!")),
            (_, false, _) => Err(Status::invalid_argument("password cannot be empty!")),
            (_, _, None) => Err(Status::invalid_argument("profile cannot be e mpty!")),
        }
    }
}
