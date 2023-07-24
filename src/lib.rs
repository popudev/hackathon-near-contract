pub mod application;
pub mod models;

use models::contract::{ContractStorageKey, ELearningContract, ELearningContractExt, ELearningContractMetadata};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
impl ELearningContract {
  #[init]
  pub fn init() -> Self {
    let owner_id = env::signer_account_id();
    Self::new(
      owner_id,
      ELearningContractMetadata {
        spec: "elearning-1.0.0".to_string(),
        name: "vbiacademy".to_string(),
        symbol: "VBIAcademy".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: ELearningContractMetadata) -> Self {
    Self {
      owner_id,
      metadata_contract: LazyOption::new(ContractStorageKey::ContractMetadata.try_to_vec().unwrap(), Some(&metadata)),
      subscriber_users: UnorderedSet::new(ContractStorageKey::SubscriberUsers.try_to_vec().unwrap()),
      intructor_users: UnorderedSet::new(ContractStorageKey::IntructorUsers.try_to_vec().unwrap()),
      mentor_users: UnorderedMap::new(ContractStorageKey::MentorUsers.try_to_vec().unwrap()),
      user_metadata_by_id: LookupMap::new(ContractStorageKey::UserMetadataById.try_to_vec().unwrap()),
      courses_per_user: LookupMap::new(ContractStorageKey::CoursesPerUser.try_to_vec().unwrap()),
      courses_per_instructor: LookupMap::new(ContractStorageKey::CoursesPerInstructor.try_to_vec().unwrap()),
      course_metadata_by_id: LookupMap::new(ContractStorageKey::CourseMetadataById.try_to_vec().unwrap()),
      certificate_per_user: LookupMap::new(ContractStorageKey::CertificatesPerUser.try_to_vec().unwrap()),
      certificate_metadata_by_id: LookupMap::new(ContractStorageKey::CertificateMetadataById.try_to_vec().unwrap()),
      skill_metadata_by_skill_id: LookupMap::new(ContractStorageKey::SkillMetadataPerSkillId.try_to_vec().unwrap()),
    }
  }
}
