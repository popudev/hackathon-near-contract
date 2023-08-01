use near_sdk::{
  collections::{LazyOption, LookupMap, UnorderedSet},
  json_types::Base64VecU8,
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, CryptoHash, PanicOnDefault,
};

use crate::borsh::{self, BorshDeserialize, BorshSerialize};

use super::{
  major::{MajorId, MajorMetadata},
  subject::{SubjectId, SubjectMetadata},
  user::{UserId, UserMetadata},
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SuperSchoolContractMetadata {
  pub spec: String,
  pub name: String,
  pub symbol: String,
  pub icon: Option<String>,
  pub base_uri: Option<String>,
  pub reference: Option<String>,
  pub reference_hash: Option<Base64VecU8>,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct SuperSchoolContract {
  pub owner_id: AccountId,
  pub metadata_contract: LazyOption<SuperSchoolContractMetadata>,

  pub user_ids: UnorderedSet<UserId>,
  pub user_metadata_by_id: LookupMap<UserId, UserMetadata>,
  pub user_metadata_by_username: LookupMap<String, UserMetadata>,

  pub major_ids: UnorderedSet<MajorId>,
  pub major_metadata_by_id: LookupMap<MajorId, MajorMetadata>,
  pub subjects_per_major: LookupMap<MajorId, UnorderedSet<SubjectId>>,
  pub students_per_major: LookupMap<MajorId, UnorderedSet<UserId>>,
  pub instructor_per_major: LookupMap<MajorId, UnorderedSet<UserId>>,

  pub subject_ids: UnorderedSet<SubjectId>,
  pub subject_metadata_by_id: LookupMap<SubjectId, SubjectMetadata>,
}

#[derive(BorshSerialize)]
pub enum ContractStorageKey {
  ContractMetadata,
  UserIds,
  UserMetadataById,
  UserMetadataByUsername,
  MajorIds,
  MajorMetadataById,
  SubjectsPerMajor,
  SubjectsPerMajorInter { instructor_id_hash: CryptoHash },
  StudentsPerMajor,
  InstructorPerMajor,

  SubjectIds,
  SubjectMetadataById,
}
