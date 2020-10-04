mod lib;
// use futures::executor;
 fn main() {
   let client = lib::Client::new("https://elc-t.zqsd.io".to_string());
    let blockjson = client.get_block(String::from("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7"));
    println!("{:?}",blockjson);
    let blocktx = client.get_block_txs(String::from("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7"),None);
    println!("{:?}",blocktx);
}