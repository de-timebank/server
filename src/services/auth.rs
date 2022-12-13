use tonic::{Request, Response, Status};

use crate::proto::auth::auth_server::Auth;
pub use crate::proto::auth::auth_server::AuthServer;
use crate::proto::auth::sign_up;
use crate::services::Result;
use crate::supabase::ClientError;
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
                let user_client = UserClient::new();

                {
                    let res = user_client.check_if_email_exist(&email).await;

                    match res {
                        Ok(value) if value => {
                            return Err(Status::already_exists(
                                "user with that email already exists",
                            ))
                        }

                        Err(e) => return Err(Status::internal(e.to_string())),
                        _ => {}
                    }
                };

                // 1. create new user
                let user = self
                    .client
                    .sign_up(email, password)
                    .await
                    .map_err(|e| Status::unknown(e.to_string()))?;

                // 2. create user profile
                let res = user_client.create_new_profile(&user.id, profile).await;

                match res {
                    Ok(_) => Ok(Response::new(sign_up::Response { user_id: user.id })),
                    Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
                    Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
                }
            }

            (false, _, _) => Err(Status::invalid_argument("email cannot be empty!")),
            (_, false, _) => Err(Status::invalid_argument("password cannot be empty!")),
            (_, _, None) => Err(Status::invalid_argument("profile cannot be empty!")),
        }
    }
}
