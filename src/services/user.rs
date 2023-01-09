use tonic::{Request, Response, Status};

pub use crate::proto::user::user_server::UserServer;
use crate::proto::user::{
    get, get_by_id, get_credit_balance, get_profile, get_rating, get_transaction_history, update,
    user_server::User,
};
use crate::services::Result;
use crate::supabase::user::UserClient;
use crate::supabase::ClientError;

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
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_profile(
        &self,
        request: Request<get_profile::Request>,
    ) -> Result<Response<get_profile::Response>> {
        let get_profile::Request { user_id } = request.into_inner();

        if user_id.is_empty() {
            return Err(Status::invalid_argument("user id cannot be empty"));
        }

        let res = self.client.get_profile(&user_id).await;

        match res {
            Ok(value) => Ok(Response::new(get_profile::Response { user: Some(value) })),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    #[allow(unused)]
    async fn get_rating(
        &self,
        request: Request<get_rating::Request>,
    ) -> Result<Response<get_rating::Response>> {
        todo!()
    }

    async fn get_credit_balance(
        &self,
        request: Request<get_credit_balance::Request>,
    ) -> Result<Response<get_credit_balance::Response>> {
        let get_credit_balance::Request { user_id } = request.into_inner();

        let res = self.client.get_credit_balance(&user_id).await;

        match res {
            Ok(value) => Ok(Response::new(value)),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_transaction_history(
        &self,
        request: Request<get_transaction_history::Request>,
    ) -> Result<Response<get_transaction_history::Response>> {
        let get_transaction_history::Request { user_id } = request.into_inner();

        let res = self.client.get_transaction_history(&user_id).await;

        match res {
            Ok(data) => Ok(Response::new(get_transaction_history::Response { data })),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }
}
