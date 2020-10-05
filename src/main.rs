mod lib;
// use futures::executor;
 fn main() {
   let client = lib::Client::new("https://elc-t.zqsd.io");
    // let blockjson = client.get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    // println!("{:?}",blockjson);
    // let get_block_status = client.get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    // println!("{:?}",get_block_status);
    let blocktx = client.get_block_txs("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",Some(0));
    println!("{:?}",blocktx.unwrap().iter().position(|tx| tx.txid == "bdbaa506c8903918b407fca86bd3498cd7794000b22cddeb1c87c2d9eb8fab62").unwrap());

}