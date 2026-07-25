use crate::oracle::pyth_v2::PythV2Oracle;
use crate::protocol::ProtocolUnderTest;
use litesvm::LiteSVM;
use solana_sdk::clock::Clock;
use solana_signer::Signer;

pub struct ScenarioRunner<P: ProtocolUnderTest>{
    pub oracle: PythV2Oracle,
    pub protocol: P
}

impl<P: ProtocolUnderTest> ScenarioRunner<P>{
    pub fn new(oracle: PythV2Oracle, protocol: P) -> Self{
        Self {oracle,protocol}
    }

    pub fn price_crash(&self,svm: &mut LiteSVM,from_price: i64,to_price: i64,steps: u64) {
     let delta = from_price - to_price;

     for i in 1..=steps{
        let mut clock: Clock= svm.get_sysvar();
        clock.slot += 1;
        clock.unix_timestamp += 1;
        svm.set_sysvar(&clock);

       
        let step_price = if i == steps {
                to_price
        } else {
                from_price - (delta * i as i64 / steps as i64)
        };

        self.oracle.set_price(
                svm,
                step_price,
                2_000,
                -8,
                clock.unix_timestamp,
                clock.slot,
        );

        self.protocol.invoke(svm, self.oracle.price_update_keypair.pubkey());
        println!("step {}/{} | slot: {} | price: {}", i, steps, clock.slot, step_price);
     }


    }

}


//Trait Protocol UnderTest -> Invoke (Yep).

//Struct Scenario runner (Oracle,Protocol) ... impl ScenarioRunner(It has a )