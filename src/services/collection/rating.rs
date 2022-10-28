use postgrest::Postgrest;
use reqwest::StatusCode;
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::timebank::rating::rating_server::Rating;
use crate::proto::timebank::rating::{create, delete, get_by, update, RatingData};
use crate::services::{error_messages, util, Result};

pub use crate::proto::timebank::rating::rating_server::RatingServer;

#[derive(Default)]
pub struct RatingService;

#[tonic::async_trait]
impl Rating for RatingService {
    async fn create_for_requestor(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        todo!()
    }

    async fn create_for_provider(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        todo!()
    }

    async fn get_by(
        &self,
        request: Request<get_by::Request>,
    ) -> Result<Response<get_by::Response>> {
        todo!()
    }

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        todo!()
    }

    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        todo!()
    }
}
