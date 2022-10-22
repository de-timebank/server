// TODO:
// include db_client's response error in the rpc response metadata

use postgrest::Postgrest;
use reqwest::StatusCode;
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::{
    proto::timebank::servicerequest::service_request_server::ServiceRequest,
    proto::timebank::servicerequest::{
        complete_service, create, delete, get, get_by_id, get_commitment, get_rating, select_bid,
        update, TServiceCommitment,
    },
    proto::timebank::{servicerating::TServiceRating, servicerequest::TServiceRequest},
    services::{error_messages, util, Result},
    starknet::{
        admin_account::AdminAccount,
        main_contract::{CreateCommitmentRequest, MainContract},
    },
};

pub use crate::proto::timebank::servicerequest::service_request_server::ServiceRequestServer;

pub struct ServiceRequestService {
    db_client: Postgrest,
}

impl ServiceRequestService {
    pub fn new() -> Self {
        Self {
            db_client: util::miscellaneous::create_postgrest_client(),
        }
    }
}

#[tonic::async_trait]
impl ServiceRequest for ServiceRequestService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(create::Payload {
                request_data,
                requestor,
            }) => {
                let res = self
                    .db_client
                    .rpc(
                        "service_request_create",
                        json!({
                            "_requestor": requestor,
                            "_request": request_data
                        })
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TServiceRequest> = res.json().await.unwrap();

                        Ok(Response::new(create::Response {
                            request: values.into_iter().next(),
                        }))
                    }

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::internal(error_messages::INVALID_PAYLOAD)),
        }
    }

    // updating a column with json-type value must also include all values
    // that are not being updated
    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) if !payload.request_id.is_empty() => {
                let update::Payload { update, request_id } = payload;

                let res = self
                    .db_client
                    .from("service_request")
                    .eq("id", request_id)
                    .update(update)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TServiceRequest> = res.json().await.unwrap();

                        Ok(Response::new(update::Response {
                            request: values.into_iter().next(),
                        }))
                    }

                    StatusCode::BAD_REQUEST => {
                        Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD))
                    }

                    _ => Err(Status::unknown(error_messages::UNKNOWN)),
                }
            }
            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let res = self
                    .db_client
                    .rpc(
                        "service_request_delete",
                        json!(
                            {
                                "_request_id": payload.request_id
                            }
                        )
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::NO_CONTENT | StatusCode::OK => {
                        Ok(Response::new(delete::Response {}))
                    }

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn select_bid(
        &self,
        request: Request<select_bid::Request>,
    ) -> Result<Response<select_bid::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let res = self
                    .db_client
                    .rpc(
                        "service_request_select_bid",
                        json!(
                            {
                                "_request_id": payload.request_id,
                                "_bid_id": payload.bid_id
                            }
                        )
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                // let response = self
                //     .get(tonic::Request::new(get::Request {
                //         payload: Some(get::Payload {
                //             column: "id".into(),
                //             filter: payload.request_id,
                //         }),
                //     }))
                //     .await
                //     .unwrap()
                //     .into_inner();

                // let main_contract = MainContract::new();
                // let admin_account = AdminAccount::new();
                // main_contract.create_commitment(admin_account, CreateCommitmentRequest {
                //     request_id: payload.request_id,
                //     requestor:
                // });

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TServiceRequest> = res
                            .json()
                            .await
                            .expect("UNABLE TO PARSE JSON AS `Vec<TServiceRequest>`");

                        Ok(Response::new(select_bid::Response {
                            request: values.into_iter().next(),
                        }))
                    }

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn get_rating(
        &self,
        request: Request<get_rating::Request>,
    ) -> Result<Response<get_rating::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let res = self
                    .db_client
                    .from("service_rating")
                    .eq("request_id", payload.request_id)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TServiceRating> = res.json().await.unwrap();

                        Ok(Response::new(get_rating::Response {
                            rating: values.into_iter().next(),
                        }))
                    }

                    StatusCode::BAD_REQUEST => {
                        Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD))
                    }

                    _ => Err(Status::unknown(error_messages::UNKNOWN)),
                }
            }

            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(get::Payload { column, filter }) => {
                let res = self
                    .db_client
                    .from("service_request")
                    .eq(column, filter)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let requests: Vec<TServiceRequest> = res.json().await.unwrap();

                        Ok(Response::new(get::Response { requests }))
                    }

                    _ => Err(Status::unknown(error_messages::UNKNOWN)),
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn complete_service(
        &self,
        request: Request<complete_service::Request>,
    ) -> Result<Response<complete_service::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(complete_service::Payload {
                request_id,
                user_id,
            }) => {
                let res = self
                    .db_client
                    .rpc(
                        "service_request_complete_service",
                        json!({
                            "_user_id": user_id,
                            "_request_id": request_id
                        })
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::NO_CONTENT => Ok(Response::new(complete_service::Response {})),

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn get_commitment(
        &self,
        request: Request<get_commitment::Request>,
    ) -> Result<Response<get_commitment::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(get_commitment::Payload { request_id }) => {
                let main_contract = MainContract::new(AdminAccount::new());

                let commitment = main_contract
                    .get_commitment_of(&request_id)
                    .await
                    .map_err(|e| {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut()
                            .append("error", e.to_string().parse().unwrap());
                        s
                    })
                    .unwrap();

                Ok(Response::new(get_commitment::Response {
                    commitment: Some(commitment),
                }))
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
        todo!()
    }
}
