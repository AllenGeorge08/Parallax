use solana_sdk::clock::Clock;

use crate::litesvm::harness::TestHarness;

const SLOTS_PER_EPOCH: u64 = 432_000;

pub struct TimeController<'a> {
    pub harness: &'a mut TestHarness<'a>,
}

impl<'a> TimeController<'a> {
    pub fn new(harness: &'a mut TestHarness<'a>) -> Self {
        Self { harness }
    }

    pub fn current_slot(&self) -> u64 {
        let clock: Clock = self.harness.svm.get_sysvar();
        clock.slot
    }

    pub fn current_epoch(&self) -> u64 {
        self.current_slot() / SLOTS_PER_EPOCH
    }

    pub fn advance_to_slot(&mut self, slot: u64) {
        self.harness.warp_to_slot(slot)
    }

    pub fn advance_to_epoch(&mut self, epoch: u64) {
        let target_slot = epoch * SLOTS_PER_EPOCH;

        self.harness.warp_to_slot(target_slot)
    }
}
