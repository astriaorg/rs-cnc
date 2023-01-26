extern crate base64;

use bytes::Bytes;
use rand::Rng;

use rs_cnc::{CelestiaNodeClient, NamespacedDataResponse, PayForDataResponse};


#[tokio::test]
async fn test_data_roundtrip() {
    let base_url = String::from("http://localhost:26659");
    let client = CelestiaNodeClient::new(base_url).unwrap();

    // generate some random bytes for namespace_id
    let random_namespace_id = rand::thread_rng().gen::<[u8; 8]>();

    // create arbitrary vector of bytes
    let data = Bytes::from(&b"some random data"[..]);

    let res: Result<PayForDataResponse, reqwest::Error> = client.submit_pay_for_data(
        &random_namespace_id,
        &data,
        2_000,
        90_000).await;

    assert!(res.is_ok());

    // use height from previous response to call namespaced data endpoint
    if let Some(height) = res.unwrap().height {
        let res: Result<NamespacedDataResponse, reqwest::Error> = client.namespaced_data(random_namespace_id, height).await;
        assert!(res.is_ok());

        if let namespaced_data_response = res.unwrap() {
            // convert base64 encoded value from the response into a vector of bytes
            let res_data = namespaced_data_response.data.unwrap();
            let base64_data = &res_data[0];
            let bytes = base64::decode(base64_data).unwrap();

            assert_eq!(bytes, data);
            assert_eq!(namespaced_data_response.height.unwrap(), height);
        }
    }
}
