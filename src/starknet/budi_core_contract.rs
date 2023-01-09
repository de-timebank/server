use super::admin_account::AdminAccount;

use starknet::{
    accounts::Call,
    core::{
        types::{AddTransactionResult, FieldElement},
        utils::starknet_keccak,
    },
    macros::selector,
};

use color_eyre::Result;

#[allow(unused)]
#[derive(Debug)]
pub struct CompletedServiceRequest {
    pub request_id: FieldElement,
    pub requestor: FieldElement,
    pub provider: FieldElement,
    pub amount: FieldElement,
    pub timestamp: FieldElement,
}

pub struct BudiCore {
    pub contract_address: FieldElement,
    account: AdminAccount,
}

impl BudiCore {
    pub fn new(account: AdminAccount) -> Self {
        let main_address = dotenv::var("BUDI_CORE_CONTRACT_ADDRESS")
            .expect("environment variable missing : BUDI_CORE_CONTRACT_ADDRESS");

        Self {
            contract_address: FieldElement::from_hex_be(&main_address).unwrap(),
            account,
        }
    }

    pub async fn commit_service_request(
        &self,
        request_id: impl AsRef<str>,
        requestor: impl AsRef<str>,
        provider: impl AsRef<str>,
        amount: f32,
        timestamp: impl AsRef<str>,
    ) -> Result<AddTransactionResult> {
        let amount = (amount * 1000000000000000000f32) as u128;
        let amount = FieldElement::from_dec_str(&amount.to_string())?;

        let res = self
            .account
            .execute(&[Call {
                to: self.contract_address,
                selector: selector!("commit_service_request"),
                calldata: vec![
                    starknet_keccak(request_id.as_ref().as_bytes()),
                    starknet_keccak(requestor.as_ref().as_bytes()),
                    starknet_keccak(provider.as_ref().as_bytes()),
                    amount,
                    // should convert into unix timestamp but im too lazy
                    starknet_keccak(timestamp.as_ref().as_bytes()),
                ],
            }])
            .send()
            .await
            .unwrap();
        Ok(res)
    }

    #[allow(unused)]
    pub async fn credit_balance_of(
        &self,
        user_id: impl AsRef<str>,
    ) -> Result<AddTransactionResult> {
        let res = self
            .account
            .execute(&[Call {
                to: self.contract_address,
                selector: selector!("credit_balance_of"),
                calldata: vec![starknet_keccak(user_id.as_ref().as_bytes())],
            }])
            .send()
            .await
            .unwrap();
        Ok(res)
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
