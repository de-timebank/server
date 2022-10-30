// Service for handling user's account

use tonic::{metadata::MetadataMap, Code as RpcCode, Request, Response, Status};

use crate::proto::timebank::user::user_server::User;
use crate::proto::timebank::user::{get, get_by_id, get_rating, update};
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

            Err(ClientErrorKind::RequestError { body, .. }) => {
                let mut map = MetadataMap::new();
                map.insert("error", body.parse().unwrap());

                Err(Status::with_metadata(
                    RpcCode::Unknown,
                    error_messages::UNKNOWN,
                    map,
                ))
            }
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

            Err(ClientErrorKind::RequestError { body, .. }) => {
                let mut map = MetadataMap::new();
                map.insert("error", body.parse().unwrap());

                Err(Status::with_metadata(
                    RpcCode::Unknown,
                    error_messages::UNKNOWN,
                    map,
                ))
            }
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

            Err(ClientErrorKind::RequestError { body, .. }) => {
                let mut map = MetadataMap::new();
                map.insert("error", body.parse().unwrap());

                Err(Status::with_metadata(
                    RpcCode::Unknown,
                    error_messages::UNKNOWN,
                    map,
                ))
            }
        }
    }

    async fn get_rating(
        &self,
        request: Request<get_rating::Request>,
    ) -> Result<Response<get_rating::Response>> {
        todo!()
    }
}