// This integration test expects OmniSciDB running with Backend TCP port running here:
const OMNISCI_ADDRESS: &str = "127.0.0.1:6274";
const OMNISCI_USER: &str = "admin";
const OMNISCI_PASSWD: &str = "HyperInteractive";
const OMNISCI_DBNAME: &str = "omnisci";

use chrono;
use omnisci;

#[test]
fn test_insert_and_query() -> Result<(), thrift::Error> {
  let table_name = format!(
    "omnisci_rs_test_{}",
    chrono::Utc::now().format("%Y%m%d_%H%M%S")
  );

  let mut client = omnisci::client::connect(OMNISCI_ADDRESS, OMNISCI_USER, OMNISCI_PASSWD, OMNISCI_DBNAME)?;

  client.run_query(format!(
    "CREATE TABLE {} (date_ text, trans text, symbol text, qty int, price float, vol float);",
    table_name
  ))?;

  client.run_query(format!(
    "INSERT INTO {} VALUES ('2020-10-31','BUY','RHAT',100,35.14,1.1);",
    table_name
  ))?;

  client.run_query(format!(
    "INSERT INTO {} VALUES ('2020-10-31','BUY','GOOG',100,12.14,1.1);",
    table_name
  ))?;

  let results = client.run_query(format!(
    "SELECT symbol, qty FROM {} WHERE symbol = 'GOOG'",
    table_name
  ))?;

  assert!(results.row_set.unwrap().rows.unwrap().len() == 1);

  client.run_query(format!("DROP TABLE {};", table_name))?;

  Ok(())
}
