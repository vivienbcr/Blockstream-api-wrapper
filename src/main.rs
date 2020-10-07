extern crate reqwest;

mod lib;
pub use serde;
use serde::Deserialize;
// use futures::executor;
 fn main() {
   let client = lib::ApiClient::new("https://elc-t.zqsd.io",Some(lib::ClientOptions{headers:Some(lib::HeadersOptions{authorization:Some("okfo".to_string())})})).unwrap();
    // let blockjson = client.get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    // println!("{:?}",blockjson);
    // let get_block_status = client.get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    // println!("{:?}",get_block_status);
    // let blocktx = client.get_block_txs("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",Some(0));
    // println!("{:?}",blocktx.unwrap().iter().position(|tx| tx.txid == "bdbaa506c8903918b407fca86bd3498cd7794000b22cddeb1c87c2d9eb8fab62").unwrap());
    // let blocktx = client.get_block_txid_at_index("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",2);
    // println!("{:?}",blocktx.unwrap());
    // let blocktx = client.get_block_raw_format("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    // println!("{:?}",blocktx.unwrap());
    // let blocktx = client.get_blocks_tip_height();
    // let outpends = client.get_tx_outspends("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316").unwrap();
    // let ooo = client.get_address_txs_chain("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",Some("bf1454b31bc132622d989a1497d8e7344007b8376443a95b2733fd04afd9f8a3")).unwrap();
    let mem = client.get_mempool_recent().unwrap();
    println!("{:?}",mem);
}
