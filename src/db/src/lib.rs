use std::{cell::RefCell, fs};
use candid::{types::number::Nat, candid_method};

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(1));
}

/// Get the value of the counter. 
#[candid_method(query)]
#[ic_cdk_macros::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

#[candid_method(query)]
#[ic_cdk_macros::query]
fn get_test() -> String {
  return String::from("test");
}


/// Set the value of the counter.
#[candid_method(update)]
#[ic_cdk_macros::update]
fn set(n: Nat) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    COUNTER.with(|count| *count.borrow_mut() = n);
}

/// Increment the value of the counter.
#[candid_method(update)]
#[ic_cdk_macros::update]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}


// #[test]
// fn check_candid_interface() {
//   println!("dawdw");
//   use candid::utils::{service_compatible, CandidSource};
//   use std::path::Path;

//   candid::export_service!();
//   let new_interface = __export_service();

//   service_compatible(
//     CandidSource::Text(&new_interface),
//     CandidSource::File(Path::new("interface.did")),
//   ).unwrap();
// }

#[test]
pub fn generate_candid() -> () {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;
  
    candid::export_service!();
   
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("{:?}", dir);
    write(dir.join(format!("test.did")), __export_service()).expect("Write failed.");
}
