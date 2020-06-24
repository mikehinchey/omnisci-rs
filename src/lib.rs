extern crate ordered_float; // Required by thrift
pub extern crate thrift;
extern crate try_from; // Required by thrift

pub mod common;
pub mod completion_hints;
pub mod extension_functions;
pub mod omnisci;
pub mod serialized_result_set;
// pub mod con;

pub mod client {
  use crate::omnisci::OmniSciSyncClient;
  use crate::omnisci::TOmniSciSyncClient;
  use crate::omnisci;

  use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol, TInputProtocol, TOutputProtocol};
  use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
  };

  pub fn create(
    remote_address: &str,
  ) -> Result<
    OmniSciSyncClient<
      TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>,
      TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>,
    >,
    thrift::Error,
  > {
    let mut c = TTcpChannel::new();
    c.open(remote_address)?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
    let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

    Ok(OmniSciSyncClient::new(i_prot, o_prot))
  }

  pub trait OmniSciConnection {
    fn run_query(&mut self, query: String) -> thrift::Result<crate::omnisci::TQueryResult>;
  }

  struct OmniSciBinaryConnection { // <Client> where Client: TOmniSciSyncClient {
    session: String,
    client: dyn TOmniSciSyncClient,
  }

  // impl OmniSciConnection for OmniSciBinaryConnection {
  //   fn run_query(&mut self, query: &str) -> thrift::Result<crate::omnisci::TQueryResult> {
  //     self.client.sql_execute(
  //       self.session.to_string(),
  //       query,
  //       false,
  //       "1".to_string(),
  //       10000,
  //       -1,
  //     )?
  //   }
  // }

  pub struct OmniSciSyncClient2 { //<IP, OP> where IP: TInputProtocol, OP: TOutputProtocol {
    session: String,
    client: OmniSciSyncClient<
      TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>,
      TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>,
    >,
  }
  
  impl OmniSciConnection for OmniSciSyncClient2 { // <IP, OP> {
    fn run_query(&mut self, query: String) -> thrift::Result<crate::omnisci::TQueryResult> {
      self.client.sql_execute(
        self.session.to_string(),
        query,
        false,
        "1".to_string(),
        10000,
        -1,
      )
    }
  }

  pub fn connect(
    remote_address: &str,
    user: &str,
    password: &str,
    db_name: &str,
  )
  -> Result<Box<dyn OmniSciConnection>, thrift::Error>
  {

    let mut c = TTcpChannel::new();
    c.open(remote_address)?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
    let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

    let mut client = OmniSciSyncClient::new(i_prot, o_prot);
    // let mut client = create(remote_address)?;
    let session = client.connect(String::from(user), String::from(password), String::from(db_name))?;

    // let client = Box::new(client);
    // let res = OmniSciBinaryConnection{session, client};
    Ok(Box::new(OmniSciSyncClient2{session, client}))
  }
}
