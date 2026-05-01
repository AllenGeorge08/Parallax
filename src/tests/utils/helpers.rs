pub fn feed_id_from_hex(hex: &str) -> [u8; 32] {
    let hex = hex.trim_start_matches("0x");
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&hex[i*2..i*2+2], 16).unwrap();
    }
    bytes
}


// src/helpers.rs
pub async fn fetch_live_pyth_price(feed_id: &str) -> i64 {
    let url = format!(
        "https://hermes.pyth.network/v2/updates/price/latest?ids[]={feed_id}"
    );

    let body: serde_json::Value = reqwest::get(&url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    body["parsed"][0]["price"]["price"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap()
}