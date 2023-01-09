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
    #[strum(serialize = "servicerequests_startservice")]
    StartService,
    #[strum(serialize = "servicerequests_completeservice")]
    CompleteService,
    #[strum(serialize = "servicerequests_getbyid")]
    GetById,
    #[strum(serialize = "servicerequests_getsummaryforuser")]
    GetSummaryForUser,
}

#[derive(AsRefStr, Debug)]
pub enum RatingRpc {
    #[strum(serialize = "ratings_createforprovider")]
    CreateForProvider,
    #[strum(serialize = "ratings_createforrequestor")]
    CreateForRequestor,
    #[allow(unused)]
    #[strum(serialize = "ratings_delete")]
    Delete,
}

#[derive(AsRefStr, Debug)]
pub enum UserRpc {
    #[strum(serialize = "users_getprofile")]
    GetProfile,
    #[strum(serialize = "users_createnewprofile")]
    HandleNewUser,
    #[strum(serialize = "users_checkifemailexist")]
    CheckIfEmailExist,
    #[strum(serialize = "users_getcreditbalance")]
    GetCreditBalance,
    #[strum(serialize = "users_gettransactionhistory")]
    GetTransactionHistory,
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
