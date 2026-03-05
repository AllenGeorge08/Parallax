use crate::litesvm::harness::TestHarness;
use solana_keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::sysvar::clock::Clock;

#[test]
pub fn test_harness_initializes() {
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    assert_eq!(
        payer.pubkey(),
        harness.payer.pubkey(),
        "Wrong key initializated"
    );

    harness.warp_to_slot(200);

    let clock: Clock = harness.svm.get_sysvar();
    assert_eq!(clock.slot, 200, "Slot warping isn't working");
}


