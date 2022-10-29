use starknet::{
    accounts::{Account, AttachedAccountCall, Call, SingleOwnerAccount},
    core::types::FieldElement,
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

use super::provider::StarkNetProvider;

pub struct AdminAccount {
    account: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>,
}

impl AdminAccount {
    pub fn new() -> Self {
        let private_key = dotenv::var("ADMIN_PRIVATE_KEY")
            .expect("environment variable missing : ADMIN_PRIVATE_KEY");

        let account_address = dotenv::var("ADMIN_ACCOUNT_ADDRESS")
            .expect("environment variable missing : ADMIN_ACCOUNT_ADDRESS");

        let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
            FieldElement::from_hex_be(&private_key).unwrap(),
        ));

        let account = SingleOwnerAccount::new(
            StarkNetProvider::new().provider,
            signer,
            FieldElement::from_hex_be(&account_address).unwrap(),
            starknet::core::chain_id::TESTNET,
        );

        Self { account }
    }

    pub fn execute(
        &self,
        calls: &[Call],
    ) -> AttachedAccountCall<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>> {
        self.account.execute(calls)
    }

    pub fn provider(&self) -> &SequencerGatewayProvider {
        self.account.provider()
    }

    pub fn into_inner(self) -> SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> {
        self.account
    }
}
