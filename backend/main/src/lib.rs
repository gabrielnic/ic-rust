use candid::{CandidType, Deserialize, candid_method};
use ic_cdk::{caller};
use ic_stable_structures::memory_manager::{VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl};
use uuid::Uuid;
use std::collections::HashMap;
use std::{cell::RefCell};
use serde::{Serialize};
use ic_cdk::export::{ Principal };
use shared::{
  serialize, api_error, ApiErrorType, Result,
  canister_helper::{ Canister, CanisterSettings, CanisterID, InstallCodeMode, },
  consts::{ DEFAULT_CYCLES }
};

thread_local! {
  static OWNER: RefCell<String> = RefCell::new(String::from(""));
  static STORE: RefCell<Store> = RefCell::new(Store::default());
}

pub fn init() {
  let controller = caller();
  OWNER.with(|data| data.replace(caller().to_text()));
}

use ic_cdk_macros::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;


const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Deserialize)]

enum Tag {
  user,
  user_profile,
  user_address,
}


#[derive(CandidType, Clone, Serialize, Deserialize)]
struct User {
  principal: Principal,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
struct UserProfile {
    uid: String,
    age: u8,
    name: String,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
struct UserAddress {
  uid: String,
  first_line: String,
}

enum Return {
  user(HashMap<String, User>),
  user_profile(HashMap<String, UserProfile>),
  user_address(HashMap<String, UserAddress>)
}


fn get_tag(tag: Tag) -> Return {
  match tag {
    Tag::user => {
       Return::user(Store::_user())
    },
    Tag::user_profile => {
      Return::user_profile(Store::_user_profile())
    },
    Tag::user_address => {
      Return::user_address(Store::_user_address())
    },
 }
}


#[derive(CandidType, Clone, Deserialize)]
struct Store {
  pub user: HashMap<String, User>,
  pub user_profile: HashMap<String, UserProfile>,
  pub user_address: HashMap<String, UserAddress>,
}

impl Store {
  fn default() -> Store {
    Store { 
      user: HashMap::new(),
      user_profile: HashMap::new(),
      user_address: HashMap::new() 
    }
  }
  fn _user() -> HashMap<String, User> {
    STORE.with(|data| {
      data.borrow().clone().user
    })
  }
  fn _user_profile() -> HashMap<String, UserProfile> {
    STORE.with(|data| {
      data.borrow().clone().user_profile
    })
  }
  fn _user_address() -> HashMap<String, UserAddress> {
    STORE.with(|data| {
      data.borrow().clone().user_address
    })
  }
}

// async fn _spawn_empty_canister(caller: Principal) -> Result<Principal, ApiError> {
//   let inputs = Some(vec![format!("caller - {}", &caller.to_string())]);

//   let canister_settings = CanisterSettings {
//       controllers: Some(vec![caller, id()]),
//       compute_allocation: None,
//       memory_allocation: None,
//       freezing_threshold: None,
//   };

//   let new_canister = Canister::create(Some(canister_settings), 25_000_000_000_000).await;
//   match new_canister {
//       Err(err) => Err(api_error(
//           ApiErrorType::BadRequest,
//           "CANISTER_NOT_CREATED",
//           err.1.as_str(),
//           inputs,
//       )),
//       Ok(_canister) => {
//           let new_canister_principal = CanisterID::from(_canister);
//           let canister_data = CanisterDetails {
//               principal: new_canister_principal,
//               wasm_version: WasmVersion::None,
//               canister_type: CanisterType::Empty,
//               is_available: true,
//           };

//           let existing_canisters = DATA
//               .with(|v| v.borrow().clone())
//               .canisters
//               .get(&caller)
//               .cloned();
//           let updated_canisters = match existing_canisters {
//               None => vec![canister_data.clone()],
//               Some(_existing_canisters) => {
//                   let mut new_existing_canisters = _existing_canisters.clone();
//                   new_existing_canisters.push(canister_data.clone());
//                   new_existing_canisters
//               }
//           };

//           DATA.with(|v| v.borrow_mut().canisters.insert(caller, updated_canisters));
//           Ok(new_canister_principal)
//       }
//   }
// }

#[query]
#[candid_method(query)]
async fn spawn_new_bucket() -> Result {

  ic_cdk::println!("{:?}", std::env::var("HOME").unwrap());
  let bucket_wasm = include_bytes!("/home/gabriel/projects/ic-rust/target/wasm32-unknown-unknown/release/player.wasm");
  ic_cdk::println!("{:?}", bucket_wasm);
  let canister_settings = CanisterSettings {
    controllers: Some(vec![caller()]),
    compute_allocation: None,
    memory_allocation: None,
    freezing_threshold: None,
   };

  let new_canister = Canister::create(Some(canister_settings), DEFAULT_CYCLES).await;

  match new_canister {
    Err(err) => api_error(
        ApiErrorType::BadRequest,
        String::from(err.1.as_str()),
    ),
    Ok(_canister) => {
        let install_canister = Canister::from(_canister)
        .install_code(
          InstallCodeMode::Install,
          bucket_wasm.to_vec(),
          (caller(),),
          ).await;
          Ok(())
    }
}
}

// async fn _install_foundation_canister(
//   caller: Principal,
//   name: String,
//   canister_principal: Principal,
// ) -> Result<Principal, ApiError> {
//   let inputs = Some(vec![
//       format!("caller - {}", &caller.to_string()),
//       format!("name - {}", &name.to_string()),
//       format!("canister_principal - {}", &canister_principal.to_string()),
//   ]);

//   if name == "".to_string() {
//       return Err(api_error(
//           ApiErrorType::Unauthorized,
//           "INVALID_NAME",
//           "Please specify a valid name",
//           "Master canister",
//           "install_foundation_canister",
//           inputs,
//       ));
//   }
//   let data = DATA.with(|v| v.borrow().clone());
//   let existing_wasm = data
//       .wasms
//       .iter()
//       .find(|w| w.wasm_type == CanisterType::Foundation);

//   match existing_wasm {
//       None => Err(api_error(
//           ApiErrorType::BadRequest,
//           "NO_WASM_SPECIFIED",
//           "There is no foundation WASM uploaded",
//           "Master canister",
//           "install_foundation_canister",
//           inputs,
//       )),
//       Some(wasm_details) => {
//           if wasm_details.bytes.len() == 0 {
//               Err(api_error(
//                   ApiErrorType::Unauthorized,
//                   "NO_BYTES",
//                   "Wasm has no bytes",
//                   "Master canister",
//                   "install_foundation_canister",
//                   inputs,
//               ))
//           } else {
//               let existing_canisters = data.canisters.get(&caller);

//               match existing_canisters {
//                   None => Err(api_error(
//                       ApiErrorType::NotFound,
//                       "NO_CANISTER_FOUND",
//                       "There are no canisters for this user",
//                       "Master canister",
//                       "install_foundation_canister",
//                       inputs,
//                   )),
//                   Some(_foundations) => {
//                       if _foundations
//                           .iter()
//                           .any(|c| c.principal == canister_principal)
//                       {
//                           let install_canister = Canister::from(canister_principal)
//                               .install_code(
//                                   InstallCodeMode::Install,
//                                   wasm_details.bytes.clone(),
//                                   (name, caller, id()),
//                               )
//                               .await;

//                           match install_canister {
//                               Err(err) => Err(api_error(
//                                   ApiErrorType::NotFound,
//                                   "CANISTER_INSTALL_FAILED",
//                                   err.1.as_str(),
//                                   "Master canister",
//                                   "install_foundation_canister",
//                                   inputs,
//                               )),
//                               Ok(_) => {
//                                   let updated_foundation = CanisterDetails {
//                                       principal: canister_principal,
//                                       wasm_version: wasm_details.wasm_version.clone(),
//                                       is_available: true,
//                                       canister_type: CanisterType::Foundation,
//                                   };

//                                   let replace =
//                                       Self::replace_foundation(caller, updated_foundation);
//                                   match replace {
//                                       Err(err) => Err(err),
//                                       Ok(_) => Ok(canister_principal),
//                                   }
//                               }
//                           }
//                       } else {
//                           Err(api_error(
//                               ApiErrorType::Unauthorized,
//                               "NOT_OWNER_OF_CANISTER",
//                               "Not the owner of this canister",
//                               "Master canister",
//                               "install_foundation_canister",
//                               inputs,
//                           ))
//                       }
//                   }
//               }
//           }
//       }
//   }
// }



#[derive(CandidType, Deserialize)] 
enum Entity {
  User(User),
  UserProfile(UserProfile),
  UserAddress(UserAddress),
}

#[update]
#[candid_method(update)]
fn insert(key: String, value: Entity) -> () {
  let id = Uuid::new_v4().to_string();
  STORE.with(|p| {
    match value {
      Entity::User(_user) => {
        p.borrow_mut().user.insert(id, _user);
      }
      Entity::UserProfile(_user_profile) => {
        p.borrow_mut().user_profile.insert(id, _user_profile);
      }
      Entity::UserAddress(_user_address) => {
        p.borrow_mut().user_address.insert(id, _user_address);
      }
    }
  })
}

#[query]
#[candid_method(query)]
fn list(t: Tag) -> Result<Vec<u8>, bool> {
  // get the storage hashmap based on the tag 
  let storage = get_tag(t);
  match storage {
    Return::user(h) => {
      Ok(serialize(&h).unwrap_or(vec![]))
    },
    Return::user_address(h) => {
      Ok(serialize(&h).unwrap_or(vec![]))
    },
    Return::user_profile(h) => {
      Ok(serialize(&h).unwrap_or(vec![]))
    }
  }
}

#[test]
pub fn generate_candid() -> () {
  use std::env;
  use std::fs::write;
  use std::path::PathBuf;

  candid::export_service!();
 
  let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let dir = dir.parent().unwrap().join("candid");
  write(dir.join(format!("main.did")), __export_service()).expect("Write failed.");
}