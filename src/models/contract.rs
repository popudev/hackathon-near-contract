use near_sdk::{
  collections::{LazyOption, LookupMap, UnorderedSet},
  json_types::Base64VecU8,
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, CryptoHash, PanicOnDefault,
};

use crate::borsh::{self, BorshDeserialize, BorshSerialize};

use super::user::{JsonUser, UserId};

/// The `ELearningContractMetadata` struct represents metadata for an e-learning contract.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SuperSchoolContractMetadata {
  /// Specification associated with the e-learning contract.
  pub spec: String,

  /// Name of the e-learning contract.
  pub name: String,

  /// Symbol associated with the e-learning contract.
  pub symbol: String,

  /// Optional icon for the e-learning contract.
  pub icon: Option<String>,

  /// Optional base URI for the e-learning contract.
  pub base_uri: Option<String>,

  /// Optional reference string for the e-learning contract.
  pub reference: Option<String>,

  /// Optional hash of the reference, encoded in base64.
  pub reference_hash: Option<Base64VecU8>,
}

/// The `ELearningContract` struct represents an e-learning contract in the system.
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct SuperSchoolContract {
  /// Account ID of the owner of the contract.
  pub owner_id: AccountId,

  /// Metadata associated with the e-learning contract.
  pub metadata_contract: LazyOption<SuperSchoolContractMetadata>,

  /// Storage all user_id of subscriber users -> For count all of users in the system
  pub student_user_ids: UnorderedSet<UserId>,

  /// Storage all user_id of instructor users. -> For count all of instructors in the system
  pub intructor_user_ids: UnorderedSet<UserId>,

  /// Map of `JsonUser` metadata by user ID.
  pub user_metadata_by_id: LookupMap<UserId, JsonUser>,
}

/// The `ContractStorageKey` enum represents keys for different persistent collections in the contract storage.
#[derive(BorshSerialize)]
pub enum ContractStorageKey {
  ContractMetadata,
  StudentUserIds,
  IntructorUserIds,
  MentorUsers,
  UserMetadataById,
  CoursesPerUser,
  CourseMetadataById,
  CertificatesPerUser,
  CertificateMetadataById,
  CertificatePerUserInner { account_id_hash: CryptoHash },
  CoursesPerInstructor,
  CoursesPerInstructorInner { instructor_id_hash: CryptoHash },
  SkillMetadataPerSkillId,
  SkillMetadataPerSkillIdInner { skill_id_hash: CryptoHash },
}
