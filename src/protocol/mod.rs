use litesvm::LiteSVM;
use solana_pubkey::Pubkey;

pub  trait ProtocolUnderTest{
    fn invoke(&self,svm: &mut LiteSVM, price_update_key: Pubkey);
}