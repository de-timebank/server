pub use crate::proto::timebank::rating::rating_server::RatingServer;

use crate::proto::timebank::rating::{create, delete, get, rating_server::Rating, update};
use crate::services::{error_messages, Result};
use crate::supabase::{rating::RatingClient, ClientErrorKind};

use tonic::{metadata::MetadataMap, Code as RpcCode, Request, Response, Status};

pub struct RatingService {
    client: RatingClient,
}

impl RatingService {
    pub fn new() -> Self {
        Self {
            client: RatingClient::new(),
        }
    }
}

#[tonic::async_trait]
impl Rating for RatingService {
    // should retrieve user's auth token
    async fn create_for_requestor(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let create::Request { rating } = request.into_inner();

        match rating {
            Some(data) => {
                let res = self.client.create_for_requestor(data).await;

                match res {
                    Ok(value) => Ok(Response::new(create::Response {
                        rating: Some(value),
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

            None => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn create_for_provider(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let create::Request { rating } = request.into_inner();

        match rating {
            Some(data) => {
                let res = self.client.create_for_provider(data).await;

                match res {
                    Ok(value) => Ok(Response::new(create::Response {
                        rating: Some(value),
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

            None => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let get::Request { key, value } = request.into_inner();

        let res = self.client.get(key, value).await;

        match res {
            Ok(values) => Ok(Response::new(get::Response { ratings: values })),

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

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        let delete::Request { rating_id } = request.into_inner();

        let res = self.client.delete(rating_id).await;

        match res {
            Ok(_) => Ok(Response::new(delete::Response {})),

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
        let update::Request { rating_id, body } = request.into_inner();

        let res = self.client.update(rating_id, body).await;

        match res {
            Ok(value) => Ok(Response::new(update::Response {
                rating: Some(value),
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
}
