use litesvm::LiteSVM;
use litesvm_token::spl_token::state::Mint;
use solana_account::Account;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_sdk::clock::Clock;
// use solana_sdk::signature::{Signer, read_keypair_file};
use crate::errors::Errors;
use litesvm_token::{
    spl_token::ID as TOKEN_PROGRAM_ID, CreateAssociatedTokenAccount, CreateMint, MintTo,
};
use solana_keypair::read_keypair_file;
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;
use std::fmt::{self};
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct TestHarness<'a> {
    pub svm: LiteSVM,
    pub payer: &'a Keypair,
    pub mint: Vec<Option<Pubkey>>,
}

impl<'a> fmt::Display for TestHarness<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " ")
    }
}

impl<'a> TestHarness<'a> {
    pub fn new(payer: &'a Keypair) -> Self {
        Self {
            svm: LiteSVM::new(),
            payer,
            mint: Vec::new(),
        }
    }

    pub fn return_instance(&mut self) -> Self{
        Self { svm: self.svm.clone(), payer: self.payer, mint: self.mint.clone()}
    }

    pub fn deploy_program(&mut self) {
        let keypair_bytes = include_bytes!("../../artifacts/oracle-keypair.json");
        let program_bytes = include_bytes!("../../artifacts/oracle.so");

        let secret_key: Vec<u8> =
            serde_json::from_slice(keypair_bytes).expect("Failed to parse oracle keypair JSON");
        let program_keypair =
            Keypair::from_bytes(&secret_key).expect("Failed to create keypair from bytes");

        let program_id = program_keypair.pubkey();
        self.svm.add_program(program_id, program_bytes);
        println!("Oracle program deployed successfully...");
    }

    pub fn deploy_program_from(
        &mut self,
        keypair_path: impl AsRef<Path>,
        program_path: impl AsRef<Path>,
    ) {
        let program_keypair = read_keypair_file(keypair_path)
            .expect("Failed to read oracle program keypair. Check the provided path.");
        let program_id = program_keypair.pubkey();
        let program_bytes = fs::read(program_path)
            .expect("Failed to read oracle program binary. Build it first with `anchor build`.");
        self.svm.add_program(program_id, &program_bytes);
        println!("Oracle program deployed succesfully...");
    }

    pub fn send_tx(&mut self, to: &Keypair, lamports: u64) {
        let payer = &self.payer;
        let payer_pubkey = payer.pubkey();
        let receiver_pubkey = to.pubkey();

        self.svm
            .airdrop(&payer_pubkey, (lamports + 1) * 1_000_000_000)
            .map_err(|err| println!("Failed Airdrop: {:?}", err));

        let transfer_ix = transfer(&payer_pubkey, &receiver_pubkey, lamports * 1_000_000_000);

        let tx: Transaction = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer_pubkey),
            &[&payer],
            self.svm.latest_blockhash(),
        );

        let result = self.svm.send_transaction(tx).unwrap();
        println!("Tx finalized: {:?}", result.pretty_logs());
    }

    pub fn warp_to_slot(&mut self, slot: u64) {
        self.svm.warp_to_slot(slot);
    }

    pub fn get_account(&mut self, pubkey: Pubkey) -> Account {
        self.svm.get_account(&pubkey).unwrap()
    }

    pub fn get_current_slot(&mut self) -> u64 {
        let clock: Clock = self.svm.get_sysvar();
        let slot = clock.slot;
        slot
    }

    pub fn get_current_epoch(&mut self) -> u64 {
        let clock: Clock = self.svm.get_sysvar();
        let epoch = clock.epoch;
        epoch
    }

    pub fn create_mint(&mut self, payer: &'a Keypair) -> Pubkey {
        let mint = CreateMint::new(&mut self.svm, payer)
            .authority(&payer.pubkey())
            .decimals(6)
            .send()
            .expect("Error creating mint");
        self.mint.push(Some(mint));
        mint
    }

    pub fn get_mint(&mut self, mint: &Pubkey) -> Result<Pubkey, Errors> {
        if self.mint.contains(&Some(*mint)) {
            Ok(*mint)
        } else {
            println!("Mint doesn't exist, create one using create_mint");
            Err(Errors::MintNotFound)
        }
    }

    pub fn create_ata(&mut self, payer: &Keypair, ata_owner: &Keypair, mint: &Pubkey) -> Pubkey {
        let mint_ata = self.get_mint(mint).unwrap_or_default();
        let ata = CreateAssociatedTokenAccount::new(&mut self.svm, &payer, &mint_ata)
            .owner(&ata_owner.pubkey())
            .send()
            .unwrap_or_default();
        ata
    }

    pub fn mint_to(&mut self, to: Pubkey, amount: u64, mint: &Pubkey) {
        let mint_account = self.get_mint(mint).unwrap_or_default();
        MintTo::new(&mut self.svm, &self.payer, &mint_account, &to, amount);
        println!("Succesfully minted to : {:?}", to);
    }

    pub fn send_instruction(&mut self,ix: Instruction,signers: &[&Keypair]){
        let payer_pubkey = self.payer.pubkey();
        let message = Message::new(&[ix],Some(&payer_pubkey));
        let blockhash = self.svm.latest_blockhash();
        let tx = Transaction::new(signers,message,blockhash);
        self.svm.send_transaction(tx).unwrap_or_default();
    }
}
