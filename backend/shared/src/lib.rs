use serde::{de::DeserializeOwned, Serialize};
use serde_cbor::Error;

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

pub fn get_entry_length<T: Serialize>(entry: &T) -> usize {
  serialize::<T>(&entry).map_or(0, |v| v.len())
}