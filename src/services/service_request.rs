use reqwest::StatusCode;
use serde_json::json;
use tonic::{metadata::MetadataMap, Code as RpcCode, Request, Response, Status};

use crate::{
    proto::timebank::servicerequest::service_request_server::ServiceRequest,
    proto::timebank::servicerequest::{
        apply_provider, complete_service, create, delete, get, get_by_id, get_commitment,
        get_rating, select_provider, update, ServiceCommitmentData,
    },
    proto::timebank::{rating::RatingData, servicerequest::ServiceRequestData},
    services::{error_messages, util, Result},
    supabase::service_request::ServiceRequestClient,
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
                let res = self.client.create(requestor, request_data).await.unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values = res.json::<Vec<ServiceRequestData>>().await.unwrap();

                        Ok(Response::new(create::Response {
                            request: values.into_iter().next(),
                        }))
                    }

                    _ => {
                        let mut map = MetadataMap::new();
                        map.insert(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

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

        let res = self.client.update(request_id, body).await.unwrap();

        if !res.status().is_success() {
            let mut map = MetadataMap::new();
            map.insert(
                "error",
                res.text().await.unwrap_or_default().parse().unwrap(),
            );

            Err(Status::with_metadata(
                RpcCode::Unknown,
                error_messages::UNKNOWN,
                map,
            ))
        } else {
            let data = res
                .json::<Vec<ServiceRequestData>>()
                .await
                .map(|values| values.into_iter().next())
                .unwrap();

            Ok(Response::new(update::Response { request: data }))
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
            let res = self.client.delete(payload.request_id).await.unwrap();

            if !res.status().is_success() {
                let mut map = MetadataMap::new();
                map.insert(
                    "error",
                    res.text().await.unwrap_or_default().parse().unwrap(),
                );

                Err(Status::with_metadata(
                    RpcCode::Unknown,
                    error_messages::UNKNOWN,
                    map,
                ))
            } else {
                Ok(Response::new(delete::Response {}))
            }
        }
    }

    async fn get_rating(
        &self,
        request: Request<get_rating::Request>,
    ) -> Result<Response<get_rating::Response>> {
        todo!()
        // let payload = request.into_inner().payload;

        // match payload {
        //     Some(payload) => {
        //         let res = self
        //             .db_client
        //             .from("service_rating")
        //             .eq("request_id", payload.request_id)
        //             .execute()
        //             .await
        //             .unwrap();

        //         match res.status() {
        //             StatusCode::OK => {
        //                 let values: Vec<RatingData> = res.json().await.unwrap();

        //                 Ok(Response::new(get_rating::Response {
        //                     rating: values.into_iter().next(),
        //                 }))
        //             }

        //             StatusCode::BAD_REQUEST => {
        //                 Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD))
        //             }

        //             _ => Err(Status::unknown(error_messages::UNKNOWN)),
        //         }
        //     }

        //     _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        // }
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let get::Request { column, filter } = request.into_inner();

        match (column.is_empty(), filter.is_empty()) {
            (false, false) => {
                let res = self.client.get(column, filter).await.unwrap();

                if !res.status().is_success() {
                    let mut map = MetadataMap::new();
                    map.insert(
                        "error",
                        res.text().await.unwrap_or_default().parse().unwrap(),
                    );

                    Err(Status::with_metadata(
                        RpcCode::Unknown,
                        error_messages::UNKNOWN,
                        map,
                    ))
                } else {
                    let requests = res.json::<Vec<ServiceRequestData>>().await.unwrap();
                    Ok(Response::new(get::Response { requests }))
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<get_by_id::Request>,
    ) -> Result<Response<get_by_id::Response>> {
        let get_by_id::Request { request_id } = request.into_inner();

        if request_id.is_empty() {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            ))
        } else {
            let res = self.client.get("id", request_id).await.unwrap();

            if !res.status().is_success() {
                let mut map = MetadataMap::new();
                map.insert(
                    "error",
                    res.text().await.unwrap_or_default().parse().unwrap(),
                );

                Err(Status::with_metadata(
                    RpcCode::Unknown,
                    error_messages::UNKNOWN,
                    map,
                ))
            } else {
                let values = res.json::<Vec<ServiceRequestData>>().await.unwrap();
                Ok(Response::new(get_by_id::Response {
                    request: values.into_iter().next(),
                }))
            }
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

        match (request_id.is_empty(), user_id.is_empty()) {
            (false, false) => {
                let res = self
                    .client
                    .complete_service(request_id, user_id)
                    .await
                    .unwrap();

                if !res.status().is_success() {
                    let mut map = MetadataMap::new();
                    map.insert(
                        "error",
                        res.text().await.unwrap_or_default().parse().unwrap(),
                    );

                    Err(Status::with_metadata(
                        RpcCode::Unknown,
                        error_messages::UNKNOWN,
                        map,
                    ))
                } else {
                    Ok(Response::new(complete_service::Response {}))
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn apply_provider(
        &self,
        request: Request<apply_provider::Request>,
    ) -> Result<Response<apply_provider::Response>> {
        todo!()
    }

    // CONDITIONS :
    // 1. MUST only be called by the requestor of `request_id`
    async fn select_provider(
        &self,
        request: Request<select_provider::Request>,
    ) -> Result<Response<select_provider::Response>> {
        todo!()
    }

    async fn get_commitment(
        &self,
        request: Request<get_commitment::Request>,
    ) -> Result<Response<get_commitment::Response>> {
        todo!()
        // let payload = request.into_inner().payload;

        // match payload {
        //     Some(get_commitment::Payload { request_id }) => {
        //         let main_contract = MainContract::new(AdminAccount::new());

        //         let commitment = main_contract
        //             .get_commitment_of(&request_id)
        //             .await
        //             .map_err(|e| {
        //                 let mut s = Status::unknown(error_messages::UNKNOWN);
        //                 s.metadata_mut()
        //                     .append("error", e.to_string().parse().unwrap());
        //                 s
        //             })
        //             .unwrap();

        //         Ok(Response::new(get_commitment::Response {
        //             commitment: Some(commitment),
        //         }))
        //     }

        //     _ => Err(Status::new(
        //         tonic::Code::InvalidArgument,
        //         error_messages::INVALID_PAYLOAD,
        //     )),
        // }
    }
}
