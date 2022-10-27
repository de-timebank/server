use std::convert::AsRef;
use strum_macros::AsRefStr;

pub trait RpcMethod {
    fn name(&self) -> &str;
}

#[derive(AsRefStr, Debug)]
pub enum ServiceRequestRpc {
    #[strum(serialize = "service_request_create")]
    Create,
    #[strum(serialize = "service_request_delete")]
    Delete,
    #[strum(serialize = "service_request_select_bid")]
    SelectBid,
    #[strum(serialize = "service_request_complete_service")]
    CompleteService,
}

#[derive(AsRefStr, Debug)]
pub enum RatingRpc {
    #[strum(serialize = "rating_create")]
    Create,
    #[strum(serialize = "rating_delete")]
    Delete,
}

#[derive(AsRefStr, Debug)]
pub enum BidRpc {
    #[strum(serialize = "bid_create")]
    Create,
    #[strum(serialize = "bid_delete")]
    Delete,
}

macro_rules! rpc_method {
    ($rpc_enum:ty) => {
        impl RpcMethod for $rpc_enum {
            fn name(&self) -> &str {
                self.as_ref()
            }
        }
    };
}

rpc_method!(ServiceRequestRpc);
rpc_method!(RatingRpc);
rpc_method!(BidRpc);
