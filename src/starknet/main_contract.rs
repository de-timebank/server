#![allow(dead_code)]
use super::admin_account::AdminAccount;
use crate::proto::timebank::servicerequest::ServiceCommitmentData;
use starknet::{
    accounts::Call,
    core::{
        types::{AddTransactionResult, BlockId, FieldElement, InvokeFunctionTransactionRequest},
        utils::starknet_keccak,
    },
    macros::selector,
    providers::{Provider, SequencerGatewayProviderError},
};

#[derive(Debug)]
pub struct CreateCommitmentRequest {
    pub request_id: String,
    pub requestor: String,
    pub provider: String,
    pub amount: u32,
    // pub message: String,
    // pub requestor_signature: Signature,
    // pub provider_signature: Signature,
}

pub struct MainContract {
    pub address: FieldElement,
    account: AdminAccount,
}

impl MainContract {
    pub fn new(account: AdminAccount) -> Self {
        let main_address = dotenv::var("MAIN_CONTRACT_ADDRESS")
            .expect("environment variable missing : MAIN_CONTRACT_ADDERSS");

        Self {
            address: FieldElement::from_hex_be(&main_address).unwrap(),
            account,
        }
    }

    pub async fn get_commitment_of(
        &self,
        request_id: &str,
    ) -> Result<ServiceCommitmentData, SequencerGatewayProviderError> {
        let hash = starknet_keccak(request_id.as_bytes());
        let res = self
            .account
            .provider()
            .call_contract(
                InvokeFunctionTransactionRequest {
                    contract_address: self.address.clone(),
                    entry_point_selector: selector!("get_commitment_of"),
                    calldata: vec![hash],
                    max_fee: FieldElement::ZERO,
                    signature: vec![],
                },
                BlockId::Latest,
            )
            .await
            .unwrap();

        let commitment = res.result;

        Ok(ServiceCommitmentData {
            requestor: commitment[0].to_string(),
            provider: commitment[1].to_string(),
            amount: commitment[2].to_string().parse().unwrap(),
            is_completed: if commitment[3].to_string() == "1" {
                true
            } else {
                false
            },
        })
    }

    pub async fn mint_for_new_user(
        &self,
        recipient_address: &str,
    ) -> Result<AddTransactionResult, SequencerGatewayProviderError> {
        let calldata = vec![FieldElement::from_hex_be(recipient_address).unwrap()];

        let result = self
            .account
            .execute(&[Call {
                to: self.address,
                selector: selector!("mint_for_new_user"),
                calldata,
            }])
            .send()
            .await
            .unwrap();

        Ok(result)
    }

    pub async fn create_commitment(
        &self,
        commitment: CreateCommitmentRequest,
    ) -> Result<AddTransactionResult, SequencerGatewayProviderError> {
        let calldata: Vec<FieldElement> = vec![
            starknet_keccak(commitment.request_id.as_bytes()),
            FieldElement::from_hex_be(&commitment.requestor).unwrap(),
            FieldElement::from_hex_be(&commitment.provider).unwrap(),
            FieldElement::from(commitment.amount),
        ];

        let result = self
            .account
            .execute(&[Call {
                to: self.address,
                selector: selector!("create_commitment"),
                calldata,
            }])
            .send()
            .await
            .unwrap();

        Ok(result)
    }

    pub async fn complete_commitment(
        &self,
        request_id: &str,
    ) -> Result<AddTransactionResult, SequencerGatewayProviderError> {
        let calldata: Vec<FieldElement> = vec![starknet_keccak(request_id.as_bytes())];

        let result = self
            .account
            .execute(&[Call {
                to: self.address,
                selector: selector!("complete_commitment"),
                calldata,
            }])
            .send()
            .await
            .unwrap();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_ID: &'static str = "";

    #[tokio::test]
    async fn mint_for_new_user() {
        let main_contract = MainContract::new(AdminAccount::new());
        let res = main_contract
            .mint_for_new_user(
                "0x7c4d7ff0b3eca8a7a95a69b46938ea06f2d5b321e132c71c39405279dad4d26".into(),
            )
            .await
            .unwrap();

        println!("mint_for_new_user : {:?}", res);
    }

    #[tokio::test]
    async fn create_commitment() {
        let main_contract = MainContract::new(AdminAccount::new());

        let commitment = CreateCommitmentRequest {
            request_id: REQUEST_ID.to_owned(),
            requestor: "0x7c4d7ff0b3eca8a7a95a69b46938ea06f2d5b321e132c71c39405279dad4d26".into(),
            provider: "0x4da0d1013e0fc835931f7c4d762371c01ecf44999928e782244c066baf8ef59".into(),
            amount: 10,
        };

        let res = main_contract.create_commitment(commitment).await.unwrap();

        println!("create_commitment : {:?}", res);
    }

    #[tokio::test]
    async fn complete_commitment() {
        let main_contract = MainContract::new(AdminAccount::new());
        let res = main_contract.complete_commitment(REQUEST_ID).await.unwrap();
        println!("complete_commitment : {:?}", res);
    }

    #[tokio::test]
    async fn get_commitment() {
        let main_contract = MainContract::new(AdminAccount::new());
        let commitment = main_contract.get_commitment_of(REQUEST_ID).await.unwrap();
        println!("get_commitment : {:?}", commitment);
    }
}
