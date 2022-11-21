use postgrest::Postgrest;
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::auth::auth_server::Auth;
pub use crate::proto::auth::auth_server::AuthServer;
use crate::proto::auth::sign_up;
use crate::services::Result;
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
        // 1. create new user
        // let sign_up::Request {
        //     email,
        //     password,
        //     profile,
        // } = request.into_inner();

        // match (!email.is_empty(), !password.is_empty(), profile) {
        //     (true, true, Some(profile)) => {
        //         // 2. create user profile
        //         let user = self
        //             .client
        //             .sign_up(email, password)
        //             .await
        //             .map_err(|e| Status::unknown(e.to_string()))?;

        //         UserClient::new()
        //             .create_profile(&user.id, profile)
        //             .await
        //             .map_err(|e| Status::unknown(e.to_string()))?;

        //         // mint 10 points for user

        //         Ok(Response::new(sign_up::Response { user_id: user.id }))
        //     }

        //     (false, _, _) => Err(Status::invalid_argument("email cannot be empty!")),
        //     (_, false, _) => Err(Status::invalid_argument("password cannot be empty!")),
        //     (_, _, None) => Err(Status::invalid_argument("profile cannot be e mpty!")),
        // }

        let res = Postgrest::new("https://quepskrrpovzwydvfezs.supabase.co/rest/v1")
        .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InF1ZXBza3JycG92end5ZHZmZXpzIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTY2NzEyNTU3OSwiZXhwIjoxOTgyNzAxNTc5fQ.VwMvERKSiR_7fS_H3ROWBrTCnYkWrBtkPDRj8s8Ma_E")
        .rpc("servicerequest_applyprovider", json!({
            "_request_id": "714c4cc1-e72b-40bf-a212-01ad49d56e49",
            "_provider": "f53809c5-68e6-480c-902e-a5bc3821a003",
        }).to_string())
            // .from("service_request")
            // .eq("asd", "asdad")
            .execute()
            .await
            .unwrap();

        println!("{}", res.text().await.unwrap());

        Ok(Response::new(sign_up::Response {
            ..Default::default()
        }))
    }
}
