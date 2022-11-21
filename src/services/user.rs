use tonic::{Request, Response, Status};

pub use crate::proto::timebank::user::user_server::UserServer;
use crate::proto::timebank::user::{
    get, get_by_id, get_credit_balance, get_rating, update, user_server::User,
};
use crate::services::{error_messages, Result};
use crate::supabase::user::UserClient;
use crate::supabase::ClientErrorKind;

pub struct UserService {
    client: UserClient,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            client: UserClient::new(),
        }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let get::Request { key, value } = request.into_inner();

        let res = self.client.get(key, value).await;

        match res {
            Ok(values) => Ok(Response::new(get::Response { users: values })),

            Err(ClientErrorKind::InternalError(_)) => {
                Err(Status::internal(error_messages::UNKNOWN))
            }

            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<get_by_id::Request>,
    ) -> Result<Response<get_by_id::Response>> {
        let get_by_id::Request { user_id } = request.into_inner();

        let res = self.client.get("user_id", user_id).await;

        match res {
            Ok(values) => Ok(Response::new(get_by_id::Response {
                user: values.into_iter().next(),
            })),

            Err(ClientErrorKind::InternalError(_)) => {
                Err(Status::internal(error_messages::UNKNOWN))
            }

            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
        }
    }

    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let update::Request { user_id, body } = request.into_inner();

        let res = self.client.update(user_id, body).await;

        match res {
            Ok(value) => Ok(Response::new(update::Response { user: Some(value) })),

            Err(ClientErrorKind::InternalError(_)) => {
                Err(Status::internal(error_messages::UNKNOWN))
            }

            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
        }
    }

    #[allow(unused)]
    async fn get_rating(
        &self,
        request: Request<get_rating::Request>,
    ) -> Result<Response<get_rating::Response>> {
        todo!()
    }

    #[allow(unused)]
    async fn get_credit_balance(
        &self,
        request: Request<get_credit_balance::Request>,
    ) -> Result<Response<get_credit_balance::Response>> {
        todo!()
    }
}
