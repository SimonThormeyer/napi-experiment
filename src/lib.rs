#![deny(clippy::all)]

use core_crypto::ConnectionType;
use core_crypto::CoreCrypto;
use core_crypto::Database;
use napi_derive::napi;

#[napi]
pub async fn put_data(data: String) {
  let key = (*b"my suuuuuuuuuuuuuuper secret key").into();
  let db = Database::open(ConnectionType::Persistent("test-db"), &key)
    .await
    .unwrap();
  let cc = CoreCrypto::new(db);
  let ctx = cc.new_transaction().await.unwrap();
  ctx.set_data(data.into_bytes()).await.unwrap();
  ctx.finish().await.unwrap();
}

#[napi]
pub async fn get_data()-> String {
  let key = (*b"my suuuuuuuuuuuuuuper secret key").into();
  let db = Database::open(ConnectionType::Persistent("test-db"), &key)
    .await
    .unwrap();
  let cc = CoreCrypto::new(db);
  let ctx = cc.new_transaction().await.unwrap();
  let data = ctx.get_data().await.unwrap().unwrap();
  ctx.finish().await.unwrap();
  String::from_utf8(data).unwrap()
}
