use crate::litesvm::harness::TestHarness;
use crate::oracle::OracleBehaviour;
use solana_keypair::Keypair;

#[test]
pub fn test_oracle_behaviour_initialize() {
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    harness.send_tx(&payer, 100);

    let (oracle_pda, _bump) = OracleBehaviour::derive_oracle_pda(1);

    let oracle_behaviour_account = OracleBehaviour::new(&mut harness, oracle_pda);

    println!(
        "Oracle Behaviour Account Initialized.. Oracle PDA Key: {:#?} ",
        oracle_pda
    );
}

#[test]
pub fn test_initialize_oracle() {
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    harness.deploy_program();
    harness.send_tx(&payer, 100);

    let (oracle_pda, _bump) = OracleBehaviour::derive_oracle_pda(1);

    let mut oracle_behaviour_account = OracleBehaviour::new(&mut harness, oracle_pda);

    oracle_behaviour_account.initialize_oracle(1, 1500000, -3, 2, true);
}

#[test]
pub fn test_price_crash() {
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    harness.deploy_program();
    harness.send_tx(&payer, 100);

    let (oracle_pda, _bump) = OracleBehaviour::derive_oracle_pda(1);

    let mut oracle_behaviour_account = OracleBehaviour::new(&mut harness, oracle_pda);

    oracle_behaviour_account.initialize_oracle(1, 1500000, -3, 2, true);

    let (initial_exponent, initial_mantissa) = oracle_behaviour_account.read_oracle();
    println!("Initial Exponent: {:?}", initial_exponent);
    println!("Initial Mantissa Before Crash: {:?}", initial_mantissa);
    oracle_behaviour_account.price_crash(1500000, 100000, 2, 2, -3,1);

    let (final_exponent, final_mantissa) = oracle_behaviour_account.read_oracle();
    println!("Final Exponent: {:?}", final_exponent);
    println!("Final Mantissa After Crash: {:?}", final_mantissa);

    assert_eq!(final_mantissa, 100000, "Price Crash Didn't work");
}
