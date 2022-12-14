use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::{
    proto::servicerequest::{
        apply_provider, complete_service, create, delete, get, get_available, get_by_id,
        get_summary_for_user, select_provider, service_request_server::ServiceRequest,
        start_service, update,
    },
    services::{error_messages, Result},
    starknet::{admin_account::AdminAccount, budi_core_contract::BudiCore},
    supabase::{service_request::ServiceRequestClient, ClientError},
};

pub use crate::proto::servicerequest::service_request_server::ServiceRequestServer;

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
                    Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
                    Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
                Ok(()) => Ok(Response::new(delete::Response {})),
                Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
                Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
            }
        }
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let get::Request { key, value } = request.into_inner();

        let res = self.client.get(key, value).await;

        match res {
            Ok(values) => Ok(Response::new(get::Response { requests: values })),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<get_by_id::Request>,
    ) -> Result<Response<get_by_id::Response>> {
        let get_by_id::Request { request_id } = request.into_inner();

        let res = self.client.get_by_id(request_id).await;

        match res {
            Ok(value) => Ok(Response::new(value)),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
            Ok(request) => {
                // // commit to blockchain here
                let admin = AdminAccount::new();
                let res = BudiCore::new(admin)
                    .commit_service_request(
                        request.id.as_str(),
                        request.requestor.as_str(),
                        request.provider(),
                        request.actual_payment,
                        request.completed_at(),
                    )
                    .await;

                match res {
                    Ok(tx) => info!(
                        "commitment submitted for request_id={} tx_hash={:#x}",
                        request.id, tx.transaction_hash
                    ),
                    Err(e) => warn!(
                        "error when submitting commitment for request_id={} error={e}",
                        request.id
                    ),
                }

                Ok(Response::new(complete_service::Response {}))
            }
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
            Ok(()) => Ok(Response::new(apply_provider::Response {})),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
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
            Ok(()) => Ok(Response::new(select_provider::Response {})),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn start_service(
        &self,
        request: Request<start_service::Request>,
    ) -> Result<Response<start_service::Response>> {
        let start_service::Request {
            user_id,
            request_id,
        } = request.into_inner();

        let res = self.client.start_service(&request_id, &user_id).await;

        match res {
            Ok(()) => Ok(Response::new(start_service::Response {})),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_available(
        &self,
        request: Request<get_available::Request>,
    ) -> Result<Response<get_available::Response>> {
        let get_available::Request { filter, range } = request.into_inner();

        let Some(filter) = filter else {
            return Err(Status::invalid_argument("missing filter data"))
        };

        let Some(range) = range else {
            return Err(Status::invalid_argument("missing page data"))
        };

        let res = self
            .client
            .get_available(
                &filter.by,
                &filter.value,
                range.from as usize,
                range.to as usize,
            )
            .await;

        match res {
            Ok(requests) => Ok(Response::new(get_available::Response { requests })),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_summary_for_user(
        &self,
        request: Request<get_summary_for_user::Request>,
    ) -> Result<Response<get_summary_for_user::Response>> {
        let get_summary_for_user::Request { user_id } = request.into_inner();

        let res = self.client.get_summary_for_user(&user_id).await;

        match res {
            Ok(value) => Ok(Response::new(value)),
            Err(ClientError::SupabaseError(e)) => Err(Status::unknown(e.to_string())),
            Err(ClientError::InternalError(e)) => Err(Status::internal(e.to_string())),
        }
    }
}
