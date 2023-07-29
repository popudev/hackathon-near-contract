use crate::models::contract::{SuperSchoolContract, SuperSchoolContractExt};
use crate::models::user::{ImplUser, JsonUser, Roles, UserMetadata};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
/// Implement function for user
impl ImplUser for SuperSchoolContract {
  /// Create a user
  fn create_student_user(&mut self, full_name: String) {
    // Check User has exist
    let user_id = env::signer_account_id();
    assert!(!self.user_metadata_by_id.contains_key(&user_id), "User has already exists");

    // Create the basic information of user
    let user_metadata = UserMetadata {
      user_id: user_id.clone(),
      role: Roles::Student,
      full_name,
      total_credit: 0,
      avatar: None,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    // Create a Json of user
    let json_user = JsonUser { user_id: user_id.clone(), metadata: user_metadata };

    // Storage json user in system contract
    self.user_metadata_by_id.insert(&user_id, &json_user);

    // Storage user_id in system contract
    self.student_user_ids.insert(&user_id);
  }

  fn get_all_user_metadata(&self, page: Option<u32>, page_size: Option<u32>) -> Vec<JsonUser> {
    let mut all_user = Vec::new();
    let page = page.unwrap_or(1) as usize;
    let page_size = page_size.unwrap_or(20) as usize;
    let start_index = (page - 1) * page_size;
    let end_index = start_index + page_size;

    for user_id in self.student_user_ids.iter().skip(start_index).take(end_index) {
      all_user.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }

    all_user
  }
}
