
cargo test --test generate -q

mkdir -p .dfx/local/canisters/$1

cp backend/candid/$1.did .dfx/local/canisters/$1

cargo build --target wasm32-unknown-unknown --release -p $1 --locked 

cp target/wasm32-unknown-unknown/release/$1.wasm .dfx/local/canisters/$1

ic-wasm .dfx/local/canisters/$1/$1.wasm -o .dfx/local/canisters/$1/$1.wasm shrink

gzip -c .dfx/local/canisters/$1/$1.wasm > .dfx/local/canisters/$1/$1.wasm.gz 