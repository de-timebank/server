pub use crate::proto::timebank::rating::rating_server::RatingServer;

use crate::proto::timebank::rating::{create, delete, get, rating_server::Rating, update};
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

                    Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

                    Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
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

                    Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

                    Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
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

            Err(e) => Err(Status::unknown(e.to_string())),
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

            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
        }
    }

    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let update::Request { rating_id, body } = request.into_inner();

        let res = self.client.update(rating_id, body).await;

        match res {
            Ok(values) => Ok(Response::new(update::Response {
                rating: values.into_iter().next(),
            })),

            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

            Err(ClientErrorKind::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
        }
    }
}
