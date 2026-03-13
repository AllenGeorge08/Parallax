use crate::litesvm::harness::TestHarness;
use crate::oracle::OracleBehaviour;
use solana_keypair::Keypair;

#[derive(Debug)]
struct LendingPosition {
    collateral_units: u64,
    borrowed_units: u64,
}

impl LendingPosition {
    fn health_ratio_bps(&self, oracle_price_mantissa: i64) -> u64 {
        let safe_price = oracle_price_mantissa.max(0) as u128;
        let collateral_value = (self.collateral_units as u128) * safe_price;
        let borrowed = self.borrowed_units.max(1) as u128;

        ((collateral_value * 10_000) / borrowed) as u64
    }

    fn should_liquidate(&self, oracle_price_mantissa: i64, min_health_ratio_bps: u64) -> bool {
        self.health_ratio_bps(oracle_price_mantissa) < min_health_ratio_bps
    }
}

#[test]
pub fn test_liquidation_scenario_using_litesvm_oracle_price_crash() {
    let payer = Keypair::new();
    let mut harness = TestHarness::new(&payer);

    harness.deploy_program();
    harness.send_tx(&payer, 100);

    let seed = 11u64;
    let (oracle_pda, _bump) = OracleBehaviour::derive_oracle_pda(seed);
    let mut oracle = OracleBehaviour::new(&mut harness, oracle_pda);

    let from_price = 1_500_000i64;
    let to_price = 100_000i64;
    let price_exponent = -3;
    let confidence = 2;

    oracle.initialize_oracle(seed, from_price, price_exponent, confidence, true);

    let position = LendingPosition {
        collateral_units: 100,
        borrowed_units: 60_000_000,
    };
    let min_health_ratio_bps = 12_000;

    let before_slot = oracle.harness.get_current_slot();
    let (_, before_mantissa) = oracle.read_oracle();

    assert_eq!(before_mantissa, from_price);
    assert!(
        !position.should_liquidate(before_mantissa, min_health_ratio_bps),
        "Position must start healthy before crash"
    );

    let slot_jump = 8;
    oracle.price_crash(
        from_price,
        to_price,
        slot_jump,
        confidence,
        price_exponent,
        seed,
    );

    let after_slot = oracle.harness.get_current_slot();
    let (_, after_mantissa) = oracle.read_oracle();

    assert!(
        after_slot >= before_slot + slot_jump,
        "Expected slot warp to advance at least {slot_jump} slots"
    );
    assert_eq!(after_mantissa, to_price, "Expected crashed oracle price");
    assert!(
        position.should_liquidate(after_mantissa, min_health_ratio_bps),
        "Position should become liquidatable after crash"
    );
}
