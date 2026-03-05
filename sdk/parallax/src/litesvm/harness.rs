use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_sdk::signature::{Signer, read_keypair_file};
use solana_sdk::{system_instruction, transaction::Transaction};

pub struct TestHarness {
    pub svm: LiteSVM,
    pub payer: Keypair,
}

impl TestHarness {
    pub fn new(payer: Keypair) -> Self {
        Self {
            svm: LiteSVM::new(),
            payer,
        }
    }

    pub fn deploy_program(&mut self) {
        let program_keypair =
            read_keypair_file("../../../../oracle/target/deploy/oracle-keypair.json")
                .expect("Failed to get keypair");
        let program_id: Pubkey = program_keypair.pubkey();
        let program_bytes = include_bytes!("../../../../oracle/target/deploy/oracle.so");
        self.svm.add_program(program_id, program_bytes);
        println!("Oracle Program deployed succesfully");
    }

    pub fn send_tx(&mut self, to: Keypair, lamports: u64) {
        let payer = &self.payer;
        let payer_pubkey = payer.pubkey();
        let receiver_pubkey = to.pubkey();

        let transfer_ix = system_instruction::transfer(&payer_pubkey, &receiver_pubkey, lamports);

        let tx = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer_pubkey),
            &[&payer],
            self.svm.latest_blockhash(),
        );

        let result = self.svm.send_transaction(tx).unwrap();
        println!("Tx finalized: {:?}", result);
    }

    pub fn warp_to_slot(&mut self, slot: u64) {
        self.svm.warp_to_slot(slot);
        println!("Warped to slot: {:?}", slot);
    }

    pub fn get_account(&mut self, pubkey: Pubkey) -> Pubkey{
        self.svm.get_account(&pubkey).unwrap()
    }
}
