use std::convert::AsRef;
use strum_macros::AsRefStr;

pub trait RpcMethod {
    fn name(&self) -> &str;
}

#[derive(AsRefStr, Debug)]
pub enum ServiceRequestRpc {
    #[strum(serialize = "servicerequests_create")]
    Create,
    #[strum(serialize = "servicerequests_delete")]
    Delete,
    #[strum(serialize = "servicerequests_applyasprovider")]
    ApplyProvider,
    #[strum(serialize = "servicerequests_selectprovider")]
    SelectProvider,
    #[strum(serialize = "servicerequests_completeservice")]
    CompleteService,
    #[strum(serialize = "servicerequests_getbyid")]
    GetById,
}

#[derive(AsRefStr, Debug)]
pub enum RatingRpc {
    #[strum(serialize = "ratings_createforprovider")]
    CreateForProvider,
    #[strum(serialize = "ratings_createforrequestor")]
    CreateForRequestor,
    #[strum(serialize = "ratings_delete")]
    Delete,
}

#[allow(unused)]
#[derive(AsRefStr, Debug)]
pub enum UserRpc {
    #[strum(serialize = "users_getbyid")]
    GetById,
    #[strum(serialize = "users_createnewprofile")]
    HandleNewUser,
    #[strum(serialize = "users_checkifemailexist")]
    CheckIfEmailExist,
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
rpc_method!(UserRpc);
