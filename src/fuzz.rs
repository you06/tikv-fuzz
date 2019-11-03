use tikv_client::*;

use std::result::Result;

pub struct Fuzz {
  client: TransactionClient,
}

impl Fuzz {
  pub async fn new(pds: Vec<&str>) -> Fuzz {
    println!("before init");
    if let Result::Ok(client) = TransactionClient::new(Config::new(pds)).await {
      Fuzz{
        client: client,
      }
    } else {
      panic!("connect to PD failed.");
    }
  }

  pub async fn read_fuzz(&self, count: u32) {
    println!("start with count {}", count);
    self.write_key("114514", "810・893").await;
  }

  async fn write_key(&self, key: &str, val: &str) {
    let mut txn = self.client.begin().await.unwrap();
    txn.set(key.to_owned(), val.to_owned()).await.unwrap();
    txn.commit().await.unwrap();
  }
}
