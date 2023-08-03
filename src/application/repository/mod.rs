#![allow(unused)]

use near_sdk::{env, AccountId, CryptoHash};
use unidecode::unidecode;

use crate::models::{subject::SubjectId, user::UserId};

pub mod internal_major;
pub mod internal_score;
pub mod internal_subject;
pub mod internal_user;

pub(crate) fn hash_string(string: &String) -> CryptoHash {
  let mut hash = CryptoHash::default();
  hash.copy_from_slice(&env::sha256(string.as_bytes()));
  hash
}

pub(crate) fn convert_to_score_id(subject_id: &SubjectId, student_id: &UserId) -> String {
  let cert = "score ".to_ascii_lowercase();
  let student_convert = student_id.to_string().to_ascii_lowercase();
  let lowercased = subject_id.to_string().to_ascii_lowercase();
  let result = cert + &lowercased + " " + &student_convert;
  result.replace(' ', "_")
}
