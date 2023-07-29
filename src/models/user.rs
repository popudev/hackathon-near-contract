#![allow(clippy::too_many_arguments)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

/// The `Roles` enum represents the various roles a user can have within the system.
#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Default, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Roles {
  /// The default role. Subscribers typically have access to consume content.
  #[default]
  Student,
  /// Instructors have the ability to create and manage content, such as courses.
  Instructor,
  /// Manager have the abilities belong to the system's partner
  Manager,
  /// Admins have administrative privileges, typically including the ability to manage users and system settings.
  Admin,
}

/// `UserId` is a type alias for `AccountId`, typically representing a unique identifier for a user in the system.
pub type UserId = AccountId;

/// This struct represents a user's metadata in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserMetadata {
  /// Unique identifier of the user.
  pub user_id: UserId,

  /// URL or identifier of the user's avatar image, if provided.
  pub avatar: Option<String>,

  /// User's full name, if provided.
  pub full_name: String,

  /// User's role within the system. Default is Student
  pub role: Roles,

  /// User's total credits.
  pub total_credit: u32,

  /// Unix timestamp (in seconds) when the user account was created.
  pub created_at: u64,

  /// Unix timestamp (in seconds) when the user account was last updated.
  pub updated_at: u64,
}

/// The `JsonUser` struct provides a comprehensive view of a user in the system.
/// It includes metadata and associated skills, certificates, and courses.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonUser {
  /// Unique identifier for the user, of type `UserId`.
  pub user_id: UserId,

  /// Detailed metadata about the user, of type `UserMetadata`.
  pub metadata: UserMetadata,
}

/// The `ImplUser` trait defines a set of behaviors associated with a user in the system.
pub trait ImplUser {
  /// Creates a new user with the provided nickname, first name, last name, and bio.
  /// The fields first_name, last_name, and bio are optional.
  fn create_student_user(&mut self, full_name: String);

  fn get_all_user_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser>;
}
