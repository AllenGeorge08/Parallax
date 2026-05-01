use crate::{harness, litesvm::harness::TestHarness};
use solana_keypair::Keypair;
use solana_sdk::clock::Clock;
use solana_signer::Signer;

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

#[test]
pub fn test_send_tx() {
    let payer = Keypair::new();
    let receiver = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    harness.send_tx(&receiver, 20);

    assert_eq!(
        harness.svm.get_balance(&receiver.pubkey()),
        Some(20_000_000_000)
    );
}

#[test]
pub fn test_deploy() {
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    harness.deploy_program();
}

#[test]
pub fn test_get_account() {
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    harness.send_tx(&payer, 5);
    let account = harness.get_account(payer.pubkey());
    println!(
        "Account for pubkey: {:?} : \n {:#?}",
        payer.pubkey(),
        account
    );
}

#[test]
pub fn create_mint_and_ata() {
    let payer = Keypair::new();
    let mut harness = TestHarness::new(&payer);
    harness.send_tx(&payer, 100);

    let account = harness.get_account(payer.pubkey());
    println!(
        "Account for pubkey: {:?} : \n {:#?} with lamports: {:?}",
        payer.pubkey(),
        account,
        account.lamports
    );

    let user_ata = Keypair::new();

    let mint = harness.create_mint(&payer);

    let mint_retrieved = harness.get_mint(&mint).unwrap_or_default();
    println!("Mint Created: {}", mint_retrieved);

    assert_eq!(mint, mint_retrieved);

    let ata = harness.create_ata(&payer, &user_ata, &mint);
    println!(
        "Ata created succesfully for user Account : {:?} : \n ATA:  {:?}",
        ata,
        user_ata.to_base58_string()
    );
}
