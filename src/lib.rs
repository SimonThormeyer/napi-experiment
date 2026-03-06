#![deny(clippy::all)]

use core_crypto::ConnectionType;
use core_crypto::CoreCrypto;
use core_crypto::Database;
use core_crypto::DatabaseKey;
use napi_derive::napi;

#[napi]
pub async fn plus_100(input: u32) -> u32 {
  let key = DatabaseKey::generate();
  let db = Database::open(ConnectionType::Persistent("test-db"),&key).await.unwrap();
  let cc = CoreCrypto::new(db);
  let ctx = cc.new_transaction().await.unwrap();
  ctx.set_data(b"hello napi".into()).await.unwrap();
  ctx.finish().await.unwrap();
  input + 100
}
