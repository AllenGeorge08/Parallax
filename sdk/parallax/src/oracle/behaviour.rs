use crate::litesvm::harness::TestHarness;
use anchor_lang::AccountDeserialize;
use anchor_lang::{InstructionData, ToAccountMetas};
use oracle::accounts::Initialize;
use oracle::accounts::SetPrice;
use oracle::instruction::Initialize as Initialize_Ix;
use oracle::instruction::SetPrice as SetPrice_Ix;
use oracle::state::Oracle;
use solana_instruction::Instruction;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::system_program;
use solana_transaction::Transaction;
// use solana_sdk::pubkey::Pubkey;

pub struct OracleBehaviour<'a> {
    pub harness: &'a mut TestHarness<'a>,
    pub oracle_account: Pubkey,
}

impl<'a> OracleBehaviour<'a> {
    pub fn new(harness: &'a mut TestHarness<'a>, oracle_account: Pubkey) -> Self {
        Self {
            harness,
            oracle_account,
        }
    }

    pub fn initialize_oracle(
        &mut self,
        seed: u64,
        price_mantissa: i64,
        price_exponent: i32,
        confidence: u64,
        is_valid: bool,
    ) {
        let initialize_ix = Instruction {
            program_id: oracle::ID,
            accounts: Initialize {
                authority: self.harness.payer.pubkey(),
                oracle: self.oracle_account,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: Initialize_Ix {
                seed: seed,
                price_exponent: price_exponent,
                price_mantissa: price_mantissa,
                confidence: confidence,
                is_valid: is_valid,
            }
            .data(),
        };

        let message = Message::new(&[initialize_ix], Some(&self.harness.payer.pubkey()));
        let recent_blockhash = self.harness.svm.latest_blockhash();

        let transaction = Transaction::new(&[&self.harness.payer], message, recent_blockhash);

        let final_tx = self.harness.svm.send_transaction(transaction).unwrap();

        println!("Initialize tx succesful");
        println!("Initialize tx signature: {:?}", final_tx.signature);
        println!(
            "Initialize tx compute units: {:?} ",
            final_tx.compute_units_consumed
        );
    }

    pub fn set_price(&mut self, price_mantissa: i64, price_exponent: i32, confidence: u64) {
        let set_price_ix = Instruction {
            program_id: oracle::ID,
            accounts: SetPrice {
                authority: self.harness.payer.pubkey(),
                oracle: self.oracle_account,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: SetPrice_Ix {
                price_exponent: price_exponent,
                price_mantissa: price_mantissa,
                confidence: confidence,
            }
            .data(),
        };

        let message = Message::new(&[set_price_ix], Some(&self.harness.payer.pubkey()));
        let recent_blockhash = self.harness.svm.latest_blockhash();

        let transaction = Transaction::new(&[&self.harness.payer], message, recent_blockhash);

        let final_tx = self.harness.svm.send_transaction(transaction).unwrap();

        println!("Initialize tx succesful");
        println!("Initialize tx signature: {:?}", final_tx.signature);
        println!(
            "Initialize tx compute units: {:?} ",
            final_tx.compute_units_consumed
        );
    }

    pub fn derive_oracle_pda(seed: u64) -> (Pubkey, u8) {
        let program_id = oracle::ID;

        let (pda, bump) =
            Pubkey::find_program_address(&[b"oracle", seed.to_le_bytes().as_ref()], &program_id);

        println!("The pda is: {:?}", pda);
        println!("The bump is: {:?}", bump);

        (pda, bump)
    }

    pub fn price_crash(
        &mut self,
        from: i64,
        to: i64,
        slots: u64,
        confidence: u64,
        price_exponent: i32,
    ) {
        self.set_price(from, price_exponent, confidence);
        let current_slot = self.harness.get_current_slot();

        self.harness.warp_to_slot(current_slot + slots);
        self.set_price(to, price_exponent, confidence);
    }


    pub fn read_oracle(&mut self) -> (i32,i64){
        let account = self.harness.svm.get_account(&self.oracle_account).unwrap();

        let oracle: Oracle = Oracle::try_deserialize(&mut account.data.as_slice()).unwrap();

        (oracle.price_exponent,oracle.price_mantissa)
    }

}
