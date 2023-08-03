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
  score::{ScoreId, ScoreMetadata},
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
  pub subjects_per_user: LookupMap<UserId, UnorderedSet<SubjectId>>,
  pub scores_per_user: LookupMap<UserId, UnorderedSet<ScoreId>>,

  pub major_ids: UnorderedSet<MajorId>,
  pub major_metadata_by_id: LookupMap<MajorId, MajorMetadata>,
  pub students_per_major: LookupMap<MajorId, UnorderedSet<UserId>>,
  pub subjects_per_major: LookupMap<MajorId, UnorderedSet<SubjectId>>,

  pub subject_ids: UnorderedSet<SubjectId>,
  pub subject_metadata_by_id: LookupMap<SubjectId, SubjectMetadata>,
  pub students_per_subject: LookupMap<SubjectId, UnorderedSet<UserId>>,

  pub score_ids: UnorderedSet<ScoreId>,
  pub scores_metadata_by_id: LookupMap<ScoreId, ScoreMetadata>,
}

#[derive(BorshSerialize)]
pub enum ContractStorageKey {
  ContractMetadata,
  UserIds,
  UserMetadataById,
  UserMetadataByUsername,
  SubjectsPerUser,
  SubjectsPerUserInter { subject_student_id_hash: CryptoHash },

  MajorIds,
  MajorMetadataById,
  StudentsPerMajor,
  StudentsPerMajorInter { major_student_id_hash: CryptoHash },
  InstructorPerMajor,

  SubjectIds,
  SubjectMetadataById,
  SubjectsPerMajor,
  SubjectsPerMajorInter { subject_major_id_hash: CryptoHash },
  StudentsPerSubject,
  StudentsPerSubjectInter { student_subject_id_hash: CryptoHash },

  ScoreIds,
  ScoreMetadataById,
  ScoresPerUser,
  ScoresPerUserInter { score_student_id_hash: CryptoHash },
}
