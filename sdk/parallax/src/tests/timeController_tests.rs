use crate::litesvm::harness::TestHarness;
use crate::timecontroller::{TimeController, controller};
use solana_keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::sysvar::clock::Clock;

#[test]
pub fn test_timecontroller_initializes(){
    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    let controller = TimeController::new(&mut harness);

    let current_slot = controller.current_slot();
    println!("Controller Initialized succesfully..");
    println!("Current slot: {:?}",current_slot);
}


#[test]
pub fn test_advance_to_slot(){

    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    let mut controller = TimeController::new(&mut harness);

    let current_slot = controller.current_slot();

    println!("Current slot is: {:?}",current_slot);

    controller.advance_to_slot(450000000000000);

    let current_slot = controller.current_slot();

    println!("Current slot after warping is : {:?}",current_slot);
    println!("Advance To Slot working succesfully...");
}

#[test]
pub fn test_advance_to_epoch(){

    let payer = Keypair::new();

    let mut harness = TestHarness::new(&payer);

    let mut controller = TimeController::new(&mut harness);

    let current_epoch = controller.current_epoch();

    println!("Current slot is: {:?}",current_epoch);

    controller.advance_to_epoch(450000);

    let current_epoch = controller.current_epoch();
    let current_slot = controller.current_slot();

    println!("Current epoch after warping is : {:?}",current_epoch);
    println!("Advance To epoch working succesfully...");

    println!("Current slot after warping epoch is: {:?}",current_slot);
}

