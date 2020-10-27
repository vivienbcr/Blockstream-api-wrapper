use esplora_api::blocking::{ApiClient, ClientOptions, HeadersOptions};
use reqwest;
use reqwest::header;
static ENDPOINT_URL: &str = "https://blockstream.info/testnet/api/";

fn default_client() -> ApiClient {
    return ApiClient::new(ENDPOINT_URL, None).unwrap();
}
#[test]
fn blocking_client() {
    let client = esplora_api::async_impl::ApiClient::new(ENDPOINT_URL, None);
    assert!(client.is_ok());
}
#[test]
fn blocking_client_custom_header() {
    let options = ClientOptions {
        headers: Some(HeadersOptions {
            authorization: Some("secret".to_string()),
        }),
    };
    let client = ApiClient::new(ENDPOINT_URL, Some(options));
    assert!(client.is_ok());
}
#[test]
fn blocking_client_custom_reqwest_builder() {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("secret"),
    );
    let reqwest_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let client = esplora_api::async_impl::ApiClient::new_from_config(ENDPOINT_URL, reqwest_client);
    assert!(client.is_ok());
}
#[test]
fn blocking_get_block() {
    let client = default_client();
    let response =
        client.get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    assert!(response.is_ok());
}
#[test]
fn blocking_get_block_status() {
    let client = default_client();
    let response =
        client.get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    assert!(response.is_ok());
}
#[test]
fn blocking_get_block_txs_with_and_without_index() {
    let client = default_client();
    let first_txs_index = client.get_block_txs(
        "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
        None,
    );
    let second_txs_index = client.get_block_txs(
        "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
        Some(25),
    );
    assert!(first_txs_index.is_ok());
    assert!(second_txs_index.is_ok());
}
#[test]
fn blocking_get_block_txids() {
    let client = default_client();
    let txids_list =
        client.get_block_txids("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    assert!(txids_list.is_ok());
}
#[test]
fn blocking_get_block_txid_at_index() {
    let client = default_client();
    let txid = client.get_block_txid_at_index(
        "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
        2,
    );
    assert!(txid.is_ok());
}

#[test]
fn blocking_get_block_raw_format() {
    let client = default_client();
    let response = client
        .get_block_raw_format("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
    assert!(response.is_ok());
}
#[test]
fn blocking_get_block_height() {
    let client = default_client();
    let block_hash = client.get_block_height(424242).unwrap();
    let block = client.get_block(&block_hash);
    assert!(block.is_ok());
}
#[test]
// Return 10 blocks from start_height
fn blocking_get_blocks() {
    let client = default_client();
    let blocks = client.get_blocks(1234);
    assert!(blocks.is_ok());
}
#[test]
// Function need return last block height
fn blocking_get_blocks_tip_height() {
    let client = default_client();
    let height = client.get_blocks_tip_height();

    assert!(height.is_ok());
}
#[test]
// Verify function return hash
fn blocking_get_blocks_tip_hash() {
    let client = default_client();
    let hash = client.get_blocks_tip_hash();

    assert!(hash.is_ok());
}
#[test]
// Check tx version
fn blocking_get_tx() {
    let client = default_client();
    let tx = client.get_tx("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24");
    assert!(tx.is_ok());
}
#[test]
// Tx status is confirmed
fn blocking_get_tx_status() {
    let client = default_client();
    let tx_status =
        client.get_tx_status("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24");
    assert!(tx_status.is_ok());
}
#[test]
// Tx raw
fn blocking_get_tx_raw() {
    let client = default_client();
    let tx_raw =
        client.get_tx_raw("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24");
    assert!(tx_raw.is_ok());
}
#[test]
// Tx hex
fn blocking_get_tx_hex() {
    let client = default_client();
    let tx_hex =
        client.get_tx_hex("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24");
    assert!(tx_hex.is_ok());
}
#[test]
fn blocking_get_tx_merkleblock_proof() {
    let client = default_client();
    let tx_hex = client.get_tx_merkleblock_proof(
        "c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24",
    );
    assert!(tx_hex.is_ok());
}
#[test]
fn blocking_get_tx_merkle_proof() {
    let client = default_client();
    let merkle_proof = client
        .get_tx_merkle_proof("6814c0b3915a8de663851b9887e0cce7d0d6c6b3f7c28b97ba8a643b72e1b7c3");
    assert!(merkle_proof.is_ok());
}
#[test]
fn blocking_get_tx_outspend() {
    let client = default_client();
    let outspend = client.get_tx_outspend(
        "fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316",
        Some(1),
    );
    assert!(outspend.is_ok());
}
#[test]
fn blocking_get_tx_outspends() {
    let client = default_client();
    let outpends =
        client.get_tx_outspends("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316");
    assert!(outpends.is_ok());
}
#[test]
fn post_tx() {
    let client = default_client();
    let resp = client.post_tx("010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2003220d1c04d6d37c5f0877fffb9a4b3500000d2f6e6f64655374726174756d2f00000000030000000000000000266a24aa21a9ed61dc942663feda48033d1026d2fa8acf0f098870202c541bffa7771e8dc51e159b0e2801000000001976a914dfdf4d53296fac595dc33d8ac7216ba516b8dcc588ac8ffd0200000000001976a914bfcc245931cbad63d09f62df43bcab989991014e88ac0120000000000000000000000000000000000000000000000000000000000000000000000000");
    assert!(resp.is_ok())
}
#[test]
fn blocking_get_address() {
    let client = default_client();
    let address_1 = client.get_address("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5");
    let address_2 = client.get_address("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk");
    assert!(address_1.is_ok());
    assert!(address_2.is_ok());
}
#[test]
fn blocking_get_script_hash() {
    let client = default_client();
    let address =
        client.get_script_hash("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c");
    assert!(address.is_ok())
}
#[test]
fn blocking_get_address_txs() {
    let client = default_client();
    let tx_list = client.get_address_txs("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5");
    assert!(tx_list.is_ok())
}
#[test]
fn blocking_get_script_hash_txs() {
    let client = default_client();
    let tx_list = client
        .get_script_hash_txs("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c");
    assert!(tx_list.is_ok())
}
#[test]
fn blocking_get_address_txs_chain() {
    let client = default_client();
    let tx_list = client.get_address_txs_chain(
        "n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",
        Some("d0075b62f8b3e464472b8edecf56083ca3e9e8424f5f332ed2f9045d7fcccddc"),
    );
    let tx_list_from_index = client.get_address_txs_chain(
        "n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",
        Some(&tx_list.unwrap()[1].txid),
    );
    assert!(tx_list_from_index.is_ok())
}
#[test]
fn blocking_get_script_hash_txs_chain() {
    let client = default_client();
    let tx_list = client.get_script_hash_txs_chain(
        "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
        None,
    );
    assert!(tx_list.is_ok())
}
#[test]
fn blocking_get_address_txs_mempool() {
    let client = default_client();
    let tx_list = client.get_address_txs_mempool("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5");
    assert!(tx_list.is_ok())
}
#[test]
fn blocking_get_script_hash_txs_mempool() {
    let client = default_client();
    let tx_list = client.get_script_hash_txs_mempool(
        "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
    );
    assert!(tx_list.is_ok())
}
#[test]
fn blocking_get_address_utxo() {
    let client = default_client();
    let utxo = client.get_address_utxo("2NDcM3CGUTwqFL7y8BSBJTYJ9kToeXawkUF");
    assert!(utxo.is_ok())
}
#[test]
fn blocking_get_script_hash_utxo() {
    let client = default_client();
    let utxo = client
        .get_script_hash_utxo("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c");
    assert!(utxo.is_ok())
}
#[test]
fn blocking_get_address_prefix() {
    let client = default_client();
    let addresses = client.get_address_prefix("2NDcM");
    assert!(addresses.is_ok())
}
// fee_estimate(get_mempool_recent(get_mempool_txids(get_mempool
#[test]
fn blocking_get_mempool() {
    let client = default_client();
    let mempool = client.get_mempool();
    assert!(mempool.is_ok())
}
#[test]
fn blocking_get_mempool_txids() {
    let client = default_client();
    let mempool_txids = client.get_mempool_txids();
    assert!(mempool_txids.is_ok())
}
#[test]
fn blocking_get_mempool_recent() {
    let client = default_client();
    let mempool_txids = client.get_mempool_recent();
    assert!(mempool_txids.is_ok())
}
#[test]
fn fee_estimate() {
    let client = default_client();
    let fee = client.fee_estimate();
    assert!(fee.is_ok())
}
