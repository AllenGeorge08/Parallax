use crate::oracle::pyth_v2::{PYTH_RECEIVER_PROGRAM_ID,PythV2Oracle};
use crate::helpers::{feed_id_from_hex,fetch_live_pyth_price};
use crate::litesvm::harness::TestHarness;
use crate::oracle::OracleBehaviour;
use solana_keypair::Keypair;
use solana_sdk::clock::Clock;
use crate::oracle::pyth_v2::PriceUpdateV2Manual;

const SOL_USD_FEED_ID: &str = 
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

#[tokio::test]
async fn test_liquidation_pyth_v2_price_crash() {
    let payer = Keypair::new();
    let mut harness = TestHarness::new(&payer);
    harness.deploy_program();
    harness.send_tx(&payer, 100);

    let live_price = fetch_live_pyth_price(SOL_USD_FEED_ID).await;
    println!("Live SOL/USD price: {live_price}");

    let feed_id = feed_id_from_hex(SOL_USD_FEED_ID);
    
    let oracle = PythV2Oracle::new(feed_id);

    let clock:Clock  = harness.svm.get_sysvar();
    oracle.set_price(&mut harness.svm, live_price, 2_000, -3, clock.unix_timestamp, clock.slot);
    let before_crashing_price: PriceUpdateV2Manual = oracle.get_price(&harness.svm);
    println!("Price before crash: {}",before_crashing_price.price_message.price);

    oracle.price_crash(&mut harness.svm, 1_500_000, 100_000, 2_000, -3, 8);
    
    let after_crashing_price: PriceUpdateV2Manual = oracle.get_price(&harness.svm);
    println!("Price after crash: {}",after_crashing_price.price_message.price);
    assert_eq!(after_crashing_price.price_message.price,100_000);
    
    let clock: Clock = harness.svm.get_sysvar();
    println!("Final slot: {}", clock.slot);
    assert_eq!(after_crashing_price.posted_slot, clock.slot);

}
