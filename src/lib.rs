pub mod application;
pub mod models;

use models::contract::{ContractStorageKey, SuperSchoolContract, SuperSchoolContractExt, SuperSchoolContractMetadata};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
impl SuperSchoolContract {
  #[init]
  pub fn init() -> Self {
    let owner_id = env::signer_account_id();
    Self::new(
      owner_id,
      SuperSchoolContractMetadata {
        spec: "Super School v1.0.0".to_string(),
        name: "SaiGon University".to_string(),
        symbol: "SGU".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: SuperSchoolContractMetadata) -> Self {
    Self {
      owner_id,
      metadata_contract: LazyOption::new(ContractStorageKey::ContractMetadata.try_to_vec().unwrap(), Some(&metadata)),
      student_user_ids: UnorderedSet::new(ContractStorageKey::StudentUserIds.try_to_vec().unwrap()),
      intructor_user_ids: UnorderedSet::new(ContractStorageKey::IntructorUserIds.try_to_vec().unwrap()),
      user_metadata_by_id: LookupMap::new(ContractStorageKey::UserMetadataById.try_to_vec().unwrap()),
    }
  }
}
