use tikv_client::*;
use futures::executor::block_on;

trait Connect {
  fn New(&self, pds: Vec<&str>) {
    async {
      self.client = await connect;
    }
  }
}

pub async fn connect(pds: Vec<&str>) {
  TransactionClient::new(Config::new(pds))
    .await.unwrap()
  // let client = TransactionClient::new(config).await.unwrap();
  // let mut txn = client.begin().await.unwrap();
  
  // let key = "114514".to_owned();
  // let val = "810・893".to_owned();
  // txn.set(key, val).await.unwrap();
  // // Finish the transaction...
  // txn.commit().await.unwrap();

  // let mut txn = client.begin().await.unwrap();
  // let key = "114514".to_owned();
  // let result: Option<Value> = txn.get(key).await.unwrap();
  // txn.commit().await.unwrap();

  // println!("{:?}", result);
}
