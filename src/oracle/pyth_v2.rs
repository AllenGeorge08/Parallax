use litesvm::LiteSVM;
use solana_account::Account;
use solana_keypair::Keypair;
use solana_pubkey::{pubkey, Pubkey};
use solana_signer::Signer;
use solana_sdk::clock::Clock;
use borsh::{BorshSerialize,BorshDeserialize,to_vec};
use crate::tests::utils::fetch_live_pyth_price;

const PRICE_UPDATE_V2_DISCRIMINATOR: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct PriceFeedMessage{
    pub feed_id: [u8;32],
    pub price: i64,
    pub conf: u64,
    pub exponent: i32,
    pub publish_time: i64,
    pub prev_publish_time: i64,
    pub ema_price: i64,
    pub ema_conf: u64 
}

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct PriceUpdateV2Manual{
    pub write_authority:    [u8; 32],  // just a Pubkey as bytes
    pub verification_level: u8,        // 0 = Partial, 1 = Full — use 1
    pub num_signatures:     u8,        // only present if Partial
    pub price_message:      PriceFeedMessage,
    pub posted_slot:        u64,
}

pub const PYTH_RECEIVER_PROGRAM_ID: Pubkey = pubkey!("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");

pub struct PythV2Oracle {
    pub price_update_keypair: Keypair,
    pub feed_id: [u8; 32],
}

impl PythV2Oracle {
    pub fn new(feed_id: [u8; 32]) -> Self {
        Self {
            price_update_keypair: Keypair::new(),
            feed_id,
        }
    }

    pub fn set_price(
        &self,
        svm: &mut LiteSVM,
        price: i64,
        conf: u64,
        exponent: i32,
        publish_time: i64,
        posted_slot: u64,
    ) {
        let price_update = PriceUpdateV2Manual {
            write_authority: self.price_update_keypair.pubkey().to_bytes(),
            verification_level: 1,
            num_signatures: 0,
            price_message: PriceFeedMessage {
                feed_id: self.feed_id,
                price,
                conf,
                exponent,
                publish_time,
                prev_publish_time: publish_time - 1,
                ema_price: price,
                ema_conf: conf,
            },
            posted_slot,
        };

        let mut data = PRICE_UPDATE_V2_DISCRIMINATOR.to_vec();
        data.extend(to_vec(&price_update).unwrap());

        svm.set_account(
            self.price_update_keypair.pubkey(),
            Account {
                lamports: 1_000_000_000,  //error?
                data,
                owner: PYTH_RECEIVER_PROGRAM_ID,
                executable: false,
                rent_epoch: u64::MAX,
            },
        ).unwrap();
    }

    pub fn get_price(&self,svm: &LiteSVM) -> PriceUpdateV2Manual{
        let account = svm.get_account(&self.price_update_keypair.pubkey()).expect("Price Updated Account not found, call the set_price() function first");

        // Skipping the initial 8 bytes
        let data = &account.data[8..];
        PriceUpdateV2Manual::try_from_slice(data).expect("Failed to deserialize PriceUpdateV2Manual")
    }

    pub fn price_crash(
        &self,
        svm: &mut LiteSVM,
        from_price: i64,
        to_price: i64,
        conf: u64,
        exponent: i32,
        slot_jump: u64,
    ){
        let steps = slot_jump;
        let price_delta = from_price - to_price;

        for i in 1..=steps{

            let step_price = if i == steps{
                to_price
            } else{
                from_price - (price_delta*i as i64/steps as i64)
            };

            let sysvaraccount: Clock  = svm.get_sysvar();
            let current_slot = sysvaraccount.slot;
            let current_timestamp = sysvaraccount.unix_timestamp;
            println!("Slot before warping: {current_slot} and timestamp before warping: {:?}",current_timestamp);
            
           
            let mut clock: Clock = svm.get_sysvar();
            clock.slot += 1;
            clock.unix_timestamp += 1; //1s per test
            svm.set_sysvar(&clock);

            self.set_price(svm, step_price, conf, exponent, clock.unix_timestamp, clock.slot);            
        }
    }

    pub async fn get_live_pyth_price(&self,feed_id: &str) -> i64 {
        let price = fetch_live_pyth_price(feed_id).await;
        price
    }

}

