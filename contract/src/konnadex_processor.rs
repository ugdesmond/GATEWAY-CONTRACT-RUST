use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise};
use base64;
use near_sdk::collections::{UnorderedMap};
use serde::{Serialize, Deserialize};


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SweepContract {
    pub token_address: &'static str,
    pub owner :AccountId,
    pub recipient:AccountId
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Contract {
    pub owner: AccountId,
    pub gateway_charge: u128,
    pub gateway_amount_converter: u128,
    pub tokens: UnorderedMap<String, AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
      Self{
        gateway_charge:1,
        gateway_amount_converter:1000,
        owner: env::predecessor_account_id(),
        tokens: UnorderedMap::new(b"d"),
      }
    }
  }
 
  
  
  #[derive(Debug)]
  #[derive(Serialize, Deserialize)]
  pub struct TokenAdded {
      pub token_symbol: String,
      pub token_address: AccountId,
  }



#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PaymentSuccessful {
     payment_reference: String,
     token_event_address: &'static str,
     caller: AccountId,
     receiver_address: AccountId,
     amt: u128,
     fee_amount: u128,
     fee_address: AccountId,
     public_key: String,
     payment_type:String
}


const NEAR: &'static str = "near";




  #[near_bindgen]
impl Contract {


    #[init]
    #[private] 
    pub fn init(gateway_charge:u128,gateway_amount_converter:u128,owner:AccountId) -> Self {
      assert!(!env::state_exists(), "Contract is already initialized");
      Self {
        owner,
        gateway_charge,
        gateway_amount_converter,
        tokens: UnorderedMap::new(b"d"),
      }
    }
 
    pub fn add_token(&mut self, token_symbol: String, token_address: AccountId) {
        assert_eq!(self.owner.clone(),env::predecessor_account_id(), "Only the contract owner can add tokens");
        assert!(!self.tokens.get(&token_symbol).clone().is_some(), "Token already exist");
        self.tokens.insert(&token_symbol,&token_address);
        let token_event = TokenAdded {
            token_symbol,
            token_address,
        };
        let log_message = serde_json::to_string(&token_event).unwrap();
        env::log(format!("TokenAdded: {}", log_message).as_bytes());
     
    
    }
    pub fn get_total_balance(&self) -> U128 {
        let account_balance = env::account_balance();
        U128::from(account_balance)
    }
    pub fn get_token(&self, token_symbol: String) -> Option<AccountId> {
        self.tokens.get(&token_symbol).clone()
    }

    pub fn get_gateway_amount_converter(&self) -> u128 {
        self.gateway_amount_converter
    }
     pub fn set_gateway_amount_converter(&mut self, fee: u128) {
        assert_eq!(self.owner.clone(),env::predecessor_account_id(), "Only the contract owner can change gateway amount converter");
        self.gateway_amount_converter = fee;
    }
    pub fn get_gateway_charge(&self) -> u128 {
        self.gateway_charge
    }
    
   
    pub fn set_gateway_charge(&mut self, charge: u128) {
        assert_eq!(self.owner.clone(),env::predecessor_account_id(), "Only the contract owner can change gateway charge");
        self.gateway_charge = charge;
    }
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
    
   
    pub fn set_owner(&mut self, new_owner: AccountId) {
        assert_eq!(self.owner.clone(),env::predecessor_account_id(), "Only the contract owner can change the owwner");
        self.owner = new_owner;
    }
#[payable]
pub  fn native_token_payment(
     &mut self,
     reference: String,
     public_key:String,
     receiver_address: AccountId,
     amount: String,
     sender_should_pay_charge:bool,
     payment_type:String
 ) {
    let mut  amt : u128 = amount.parse().expect("Failed to parse number");
     assert!(amt  > 0, "Invoice amounts must be greater than zero");
     let  sender:AccountId= env::predecessor_account_id();
     let sender_balance :u128= env::attached_deposit();
     let fee_amount :u128 = (self.gateway_charge * amt) / self.gateway_amount_converter;
     if sender_should_pay_charge {
        assert!(sender_balance >= (amt+fee_amount), "Insuficient balance");
     }else{
        assert!(sender_balance >= amt, "Insuficient balance");
        amt-=fee_amount;
     }
     assert!(env::is_valid_account_id(receiver_address.as_bytes()), "Invalid address");
    
     let owner = self.owner.clone();
         Promise::new(owner).transfer(fee_amount);
         Promise::new(receiver_address.clone()).transfer(amt);
         

 let token_event = PaymentSuccessful {
     payment_reference: reference,
     token_event_address:NEAR,
     caller: sender,
     receiver_address,
     amt,
     fee_amount: fee_amount,
     fee_address:self.owner.clone(),
     public_key,
     payment_type
 };
 let log_message = serde_json::to_string(&token_event).unwrap();
 env::log(format!("PaymentSuccessful: {}", log_message).as_bytes());


}
   

#[payable]
pub fn sweep_native_token(&mut self, recipient: AccountId){
   assert_eq!(self.owner.clone(),env::predecessor_account_id(), "Only the contract owner can sweep tokens");
  let available_balance = env::account_balance();
  Promise::new(recipient.clone()).transfer(available_balance); 
  
  let sweep_event = SweepContract {
    token_address: NEAR,
    owner:self.owner.clone(),
    recipient:recipient.clone(),
};
let log_message = serde_json::to_string(&sweep_event).unwrap();
env::log(format!("SweepContract: {}", log_message).as_bytes());
}
    
}

fn main() {}

  
  


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils::{accounts, VMContextBuilder}, testing_env, MockedBlockchain};
    use std::convert::TryInto;
    //NOTE that by default the contract is already initialized with 100Near token.



#[test]
fn test_contract() {
    // Define a test context
    let context = VMContextBuilder::new()
        .current_account_id(accounts(0))
        .predecessor_account_id(accounts(1))
        .build();

    // Initialize the contract
    testing_env!(context.clone());
    let mut contract = Contract::init(1, 1000,accounts(1));

    // Assert the initial values
    assert_eq!(contract.get_gateway_amount_converter(), 1000);
    assert_eq!(contract.get_gateway_charge(), 1);
    assert_eq!(contract.get_owner(), accounts(1));

    // Update the fee amount converter
    testing_env!(context.clone());
    contract.set_gateway_amount_converter(2000);
    assert_eq!(contract.get_gateway_amount_converter(), 2000);

    // Update the salary charge
    testing_env!(context.clone());
    contract.set_gateway_charge(2);
    assert_eq!(contract.get_gateway_charge(), 2);

    // Update the owner
    testing_env!(context.clone());
    contract.set_owner(accounts(2));
    assert_eq!(contract.get_owner(), accounts(2));
}

    #[test]
    fn test_add_token() {
        let mut context = VMContextBuilder::new();
        context.signer_account_id(accounts(0));
        testing_env!(context.build());
        
        let mut contract = Contract::default();
        
        let token_symbol = "BUSD".to_string();
        let token_address = accounts(1);
        contract.add_token(token_symbol.clone(), token_address.clone());
        
        assert_eq!(contract.get_token(token_symbol), Some(token_address));
    }

 

    #[test]
    fn test_native_token_payment() {
        let mut context = VMContextBuilder::new();
        context.signer_account_id(accounts(0));
         //after deploying contract ..it get 100 near by default.
        context.attached_deposit(2000000000000000000000000);
        testing_env!(context.build());
        
        let mut contract = Contract::default();
        contract.add_token("USDT".to_string(), accounts(1));
        
        let reference = "YHURE748".to_string();
        let public_key = "YHURE748ewwee3".to_string();
        let payment_type="SDK".to_string();
        let receiver_address = accounts(2);
        let amount = "1000000000000000000000000";
        let sender_should_pay_charge = false;
        assert_eq!(contract.get_total_balance(), U128::from(102000000000000000000000000));
        contract.native_token_payment(reference.clone(),public_key.clone(), receiver_address.clone(), amount.to_string(), sender_should_pay_charge,payment_type);

       assert_eq!(contract.get_total_balance(), U128::from(101000000000000000000000000));
    }



  #[test]
  fn test_sweep() {
    // Set up the mock environment
    let mut context = VMContextBuilder::new();
    context.signer_account_id(accounts(0));
    context.attached_deposit(1000000000000000000000000); // 100 NEAR
    testing_env!(context.build());
    
    // Instantiate the contract
    let mut contract = Contract::default();
    // Call the sweep function
    let recipient = accounts(1);
    contract.sweep_native_token(recipient.clone());
    
    // Assert that the transfer was made and the event was logged
    assert_eq!(env::account_balance(), 0, "Sweep failed: Available balance was not transferred");

}

}