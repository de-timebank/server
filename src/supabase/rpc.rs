use std::convert::AsRef;
use strum_macros::AsRefStr;

pub trait RpcMethod {
    fn name(&self) -> &str;
}

#[derive(AsRefStr, Debug)]
pub enum ServiceRequestRpc {
    #[strum(serialize = "servicerequest_create")]
    Create,
    #[strum(serialize = "servicerequest_delete")]
    Delete,
    #[strum(serialize = "servicerequest_applyprovider")]
    ApplyProvider,
    #[strum(serialize = "servicerequest_selectprovider")]
    SelectProvider,
    #[strum(serialize = "servicerequest_completeservice")]
    CompleteService,
}

#[derive(AsRefStr, Debug)]
pub enum RatingRpc {
    #[strum(serialize = "rating_createforprovider")]
    CreateForProvider,
    #[strum(serialize = "rating_createforrequestor")]
    CreateForRequestor,
    #[strum(serialize = "rating_delete")]
    Delete,
}

#[allow(unused)]
#[derive(AsRefStr, Debug)]
pub enum UserRpc {
    #[strum(serialize = "user_getrating")]
    GetRating,
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
