use chrono::NaiveDateTime;
use mongodb::{options::ClientOptions, Client};
use mysql_async::prelude::Query;
use mysql_common::prelude::FromRow;
use mysql_common::FromRowError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct DataSheetRecord {
  id: u64,
  record_id: Option<String>,
  dst_id: Option<String>,
  // #[mysql(json)]
  data: Option<serde_json::Value>,
  revision_history: Option<String>,
  revision: Option<u64>,
  // // #[mysql(json)]
  field_updated_info: Option<serde_json::Value>,
  is_deleted: u8,
  created_by: Option<i64>,
  updated_by: Option<i64>,
  // created_at: Option<Vec<u8>>,
  // updated_at: Option<Vec<u8>>,
  created_at: Option<NaiveDateTime>,
  updated_at: Option<NaiveDateTime>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // 获取 MySQL 连接
  let mysql_url = "mysql://vika:kQ6zYMdvqJe9bRUvUdof@127.0.0.1:33306/vikadata";
  let mysql_pool = mysql_async::Pool::new(mysql_url);

  // 获取 MongoDB 连接字符串
  let mongodb_url = "mongodb://vika:vika@localhost:27017/baseapp?authSource=admin";

  // 连接到 MongoDB
  let client_options = ClientOptions::parse(&mongodb_url)
    .await
    .expect("Failed to parse client options");
  let client = Client::with_options(client_options).expect("Failed to create client");
  let mongodb_db = client.database("baseapp");
  // mongodb_db.collection::<DataSheetRecord>("vika_datasheet_record").drop(None).await?;
  let collection_name = "vika_datasheet_record";

  // Delete the collection
  // match mongodb_db
  //   .collection::<DataSheetRecord>(collection_name)
  //   .drop(None)
  //   .await
  // {
  //   Ok(_) => println!("Collection '{}' deleted successfully", collection_name),
  //   Err(e) => eprintln!("Error deleting collection: {}", e),
  // }

  // 分页参数
  let page_size = 10000;
  let mut last_id = 0;

  /// 1466343107857416195
  for _ in 0..10 {
    let mut conn = mysql_pool.get_conn().await?;
    // let sql = format!("SELECT * FROM vika_datasheet_record where id > {} and dst_id='dstrmVd9p6ZPMYXbXc' order by id LIMIT {}", last_id, page_size);
    // let param = params! {"dst_id" => "dstrmVd9p6ZPMYXbXc"};
    let sql = format!("SELECT * FROM vika_datasheet_record where id > {} and created_at != '0000-00-00 00:00:00' and updated_at != '0000-00-00 00:00:00' order by id LIMIT {}", last_id, page_size);
    // let param = Params::Named(HashMap::new());

    println!("conn get, sql:{}", sql);
    let rows: Vec<DataSheetRecord> = sql
      // .with(param)
      .fetch(&mut conn)
      .await?;
    // 从 MySQL 获取数据
    last_id = rows.last().map(|row| row.id).unwrap_or(0);
    // 如果没有更多的行，退出循环
    if rows.is_empty() {
      break;
    }
    // break;
    println!("{}", rows.len());

    let result = mongodb_db
      .collection::<DataSheetRecord>(collection_name)
      .insert_many(rows, None)
      .await;
    if result.is_err() {
      println!("insert_many error: {:?}", result);
    }
  }
  Ok(())
}
