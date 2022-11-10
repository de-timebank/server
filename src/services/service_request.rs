use serde_json::json;
use tonic::{metadata::MetadataMap, Code as RpcCode, Request, Response, Status};

use crate::{
    proto::timebank::servicerequest::{
        apply_provider, complete_service, create, delete, get, get_by_id, get_commitment,
        select_provider, service_request_server::ServiceRequest, update,
    },
    services::{error_messages, Result},
    supabase::{service_request::ServiceRequestClient, ClientErrorKind},
};

pub use crate::proto::timebank::servicerequest::service_request_server::ServiceRequestServer;

pub struct ServiceRequestService {
    client: ServiceRequestClient,
}

impl ServiceRequestService {
    pub fn new() -> Self {
        Self {
            client: ServiceRequestClient::new(),
        }
    }
}

#[tonic::async_trait]
impl ServiceRequest for ServiceRequestService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let payload = request.into_inner();

        match payload {
            create::Request {
                requestor,
                request_data: Some(request_data),
            } => {
                let res = self.client.create(requestor, request_data).await;

                match res {
                    Ok(value) => Ok(Response::new(create::Response {
                        request: Some(value),
                    })),

                    Err(ClientErrorKind::InternalError(e)) => {
                        Err(Status::internal(error_messages::UNKNOWN))
                    }

                    Err(ClientErrorKind::RequestError { code, body }) => {
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

            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    // updating a column with json-type value must also include all values
    // that are not being updated
    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let update::Request { request_id, body } = request.into_inner();

        let res = self.client.update(request_id, body).await;

        match res {
            Ok(values) => Ok(Response::new(update::Response {
                request: values.into_iter().next(),
            })),

            Err(e) => Err(Status::unknown(e.to_string())),
        }
    }

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        let payload = request.into_inner();

        if payload.request_id.is_empty() {
            Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD))
        } else {
            let res = self.client.delete(payload.request_id).await;

            match res {
                Ok(_) => Ok(Response::new(delete::Response {})),

                Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

                Err(ClientErrorKind::RequestError { code, body }) => {
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

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let get::Request { key, value } = request.into_inner();

        let res = self.client.get(key, value).await;

        match res {
            Ok(values) => Ok(Response::new(get::Response { requests: values })),

            Err(e) => Err(Status::unknown(e.to_string())),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<get_by_id::Request>,
    ) -> Result<Response<get_by_id::Response>> {
        let get_by_id::Request { request_id } = request.into_inner();

        let res = self.client.get("id", request_id).await;

        match res {
            Ok(values) => Ok(Response::new(get_by_id::Response {
                request: values.into_iter().next(),
            })),

            Err(e) => Err(Status::unknown(e.to_string())),
        }
    }

    async fn complete_service(
        &self,
        request: Request<complete_service::Request>,
    ) -> Result<Response<complete_service::Response>> {
        let complete_service::Request {
            request_id,
            user_id,
        } = request.into_inner();

        let res = self.client.complete_service(request_id, user_id).await;

        match res {
            Ok(_) => Ok(Response::new(complete_service::Response {})),

            Err(ClientErrorKind::InternalError(e)) => {
                Err(Status::internal(error_messages::UNKNOWN))
            }

            Err(ClientErrorKind::RequestError { code, body }) => {
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

    async fn apply_provider(
        &self,
        request: Request<apply_provider::Request>,
    ) -> Result<Response<apply_provider::Response>> {
        let apply_provider::Request {
            request_id,
            provider,
        } = request.into_inner();

        let res = self.client.apply_as_provider(request_id, provider).await;

        match res {
            Ok(_) => Ok(Response::new(apply_provider::Response {})),

            Err(ClientErrorKind::InternalError(e)) => Err(Status::internal(e.to_string())),

            Err(ClientErrorKind::RequestError { code, body }) => {
                // let mut map = MetadataMap::new();
                // map.insert("error", body.parse().unwrap());

                Err(Status::unknown(body))
            }
        }
    }

    // CONDITIONS :
    // 1. MUST only be called by the requestor of `request_id`
    async fn select_provider(
        &self,
        request: Request<select_provider::Request>,
    ) -> Result<Response<select_provider::Response>> {
        let select_provider::Request {
            request_id,
            provider,
            caller,
        } = request.into_inner();

        let res = self
            .client
            .select_provider(request_id, provider, caller)
            .await;

        match res {
            Ok(_) => Ok(Response::new(select_provider::Response {})),

            Err(ClientErrorKind::InternalError(e)) => {
                Err(Status::internal(error_messages::UNKNOWN))
            }

            Err(ClientErrorKind::RequestError { code, body }) => {
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

    #[allow(unused)]
    async fn get_commitment(
        &self,
        request: Request<get_commitment::Request>,
    ) -> Result<Response<get_commitment::Response>> {
        todo!()
    }
}
