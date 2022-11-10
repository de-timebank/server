#![allow(dead_code)]
use super::admin_account::AdminAccount;
use crate::proto::timebank::servicerequest::ServiceCommitmentData;
use starknet::{
    accounts::{single_owner::TransactionError, Call},
    core::{
        types::{AddTransactionResult, BlockId, FieldElement, InvokeFunctionTransactionRequest},
        utils::{get_selector_from_name, starknet_keccak},
    },
    macros::selector,
    providers::{Provider, SequencerGatewayProviderError},
    signers::Signer,
};

#[derive(Debug)]
pub struct CreateCommitmentRequest {
    pub request_id: String,
    pub requestor: FieldElement,
    pub provider: FieldElement,
    pub amount: FieldElement,
}

pub struct OperatorContract {
    pub contract_address: FieldElement,
    account: AdminAccount,
}

impl OperatorContract {
    pub fn new(account: AdminAccount) -> Self {
        let main_address = dotenv::var("MAIN_CONTRACT_ADDRESS")
            .expect("environment variable missing : MAIN_CONTRACT_ADDERSS");

        Self {
            contract_address: FieldElement::from_hex_be(&main_address).unwrap(),
            account,
        }
    }

    pub async fn mint_for_new_user(
        &self,
        recipient_address: &str,
    ) -> Result<AddTransactionResult, SequencerGatewayProviderError> {
        let res = self
            .account
            .execute(&[Call {
                to: self.contract_address,
                selector: selector!("mint_for_new_user"),
                calldata: vec![FieldElement::from_hex_be(recipient_address).unwrap()],
            }])
            .send()
            .await
            .unwrap();
        Ok(res)
    }

    pub async fn create_commitment(
        &self,
        new_commitment: &CreateCommitmentRequest,
    ) -> Result<AddTransactionResult, SequencerGatewayProviderError> {
        let res = self
            .account
            .execute(&[Call {
                to: self.contract_address,
                selector: selector!("create_commitment"),
                calldata: vec![
                    starknet_keccak(new_commitment.request_id.as_bytes()),
                    new_commitment.requestor,
                    new_commitment.provider,
                    new_commitment.amount,
                    FieldElement::from_dec_str("0").unwrap(),
                ],
            }])
            .send()
            .await
            .unwrap();
        Ok(res)
    }

    pub async fn complete_commitment(
        &self,
        request_id: &str,
    ) -> Result<AddTransactionResult, SequencerGatewayProviderError> {
        let res = self
            .account
            .execute(&[Call {
                to: self.contract_address,
                selector: selector!("complete_commitment"),
                calldata: vec![starknet_keccak(request_id.as_bytes())],
            }])
            .send()
            .await
            .unwrap();
        Ok(res)
    }

    pub async fn get_commitment_of(
        &self,
        request_id: &str,
    ) -> Result<ServiceCommitmentData, SequencerGatewayProviderError> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const REQUEST_ID: &'static str = "";

//     #[tokio::test]
//     async fn mint_for_new_user() {
//         let main_contract = OperatorContract::new(AdminAccount::new());
//         let res = main_contract
//             .mint_for_new_user(
//                 "0x7c4d7ff0b3eca8a7a95a69b46938ea06f2d5b321e132c71c39405279dad4d26".into(),
//             )
//             .await
//             .unwrap();

//         println!("mint_for_new_user : {:?}", res);
//     }

//     #[tokio::test]
//     async fn create_commitment() {
//         let main_contract = OperatorContract::new(AdminAccount::new());

//         let commitment = CreateCommitmentRequest {
//             request_id: REQUEST_ID.to_owned(),
//             requestor: "0x7c4d7ff0b3eca8a7a95a69b46938ea06f2d5b321e132c71c39405279dad4d26".into(),
//             provider: "0x4da0d1013e0fc835931f7c4d762371c01ecf44999928e782244c066baf8ef59".into(),
//             amount: 10,
//         };

//         let res = main_contract.create_commitment(commitment).await.unwrap();

//         println!("create_commitment : {:?}", res);
//     }

//     #[tokio::test]
//     async fn complete_commitment() {
//         let main_contract = OperatorContract::new(AdminAccount::new());
//         let res = main_contract.complete_commitment(REQUEST_ID).await.unwrap();
//         println!("complete_commitment : {:?}", res);
//     }

//     #[tokio::test]
//     async fn get_commitment() {
//         let main_contract = OperatorContract::new(AdminAccount::new());
//         let commitment = main_contract.get_commitment_of(REQUEST_ID).await.unwrap();
//         println!("get_commitment : {:?}", commitment);
//     }
// }
