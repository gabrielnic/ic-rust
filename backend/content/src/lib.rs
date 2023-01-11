use candid::{CandidType, Deserialize, candid_method};
use std::collections::HashMap;
use std::{cell::RefCell};
use serde::{de::DeserializeOwned, Serialize};
use serde_cbor::Error;
use ic_cdk::export::{ Principal };
use ulid::Ulid;

use ic_cdk_macros::*;


pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>, Error> {
  let bytes = serde_cbor::to_vec(data);
  match bytes {
      Err(err) => Err(err),
      Ok(_bytes) => Ok(_bytes),
  }
}

pub fn deserialize<T: DeserializeOwned>(data: Vec<u8>) -> Result<T, Error> {
  let data = serde_cbor::from_slice(data.as_slice());
  match data {
      Err(err) => Err(err),
      Ok(_data) => Ok(_data),
  }
}


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

thread_local! {
  static STORE: RefCell<Store> = RefCell::new(Store::default());
}

#[derive(CandidType, Deserialize)] 
enum Entity {
  User(User),
  UserProfile(UserProfile),
  UserAddress(UserAddress),
}


#[update]
#[candid_method(update)]
fn insert(key: String, value: Entity) -> () {
  let id = Ulid::new().to_string();
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
