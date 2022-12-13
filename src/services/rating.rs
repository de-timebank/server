pub use crate::proto::rating::rating_server::RatingServer;

use crate::proto::rating::{
    create, delete, get, get_by_id, get_for_request, rating_server::Rating, update,
};
use crate::services::{error_messages, Result};
use crate::supabase::{rating::RatingClient, ClientErrorKind};

use tonic::{Request, Response, Status};

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
                    Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
                    Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
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
                    Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
                    Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
                }
            }

            None => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn get_for_request(
        &self,
        request: Request<get_for_request::Request>,
    ) -> Result<Response<get_for_request::Response>> {
        let get_for_request::Request { request_id } = request.into_inner();

        let res = self.client.get_for_request(&request_id).await;

        match res {
            Ok(values) => Ok(Response::new(get_for_request::Response { ratings: values })),
            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        let delete::Request {
            request_id,
            rating_for,
        } = request.into_inner();

        let res = self.client.delete(request_id, rating_for).await;

        match res {
            Ok(_) => Ok(Response::new(delete::Response {})),
            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let update::Request {
            request_id,
            rating_for,
            body,
        } = request.into_inner();

        let res = self.client.update(request_id, rating_for, body).await;

        match res {
            Ok(values) => Ok(Response::new(update::Response {
                rating: values.into_iter().next(),
            })),
            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<get_by_id::Request>,
    ) -> Result<Response<get_by_id::Response>> {
        let get_by_id::Request {
            request_id,
            rating_for,
        } = request.into_inner();

        let res = self.client.get_by_id(&request_id, &rating_for).await;

        match res {
            Ok(values) => Ok(Response::new(get_by_id::Response {
                rating: values.into_iter().next(),
            })),
            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let get::Request { key, value } = request.into_inner();

        let res = self.client.get(&key, &value).await;

        match res {
            Ok(ratings) => Ok(Response::new(get::Response { ratings })),
            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }
}
