use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{env, near_bindgen, AccountId};
use std::fmt;
use near_sdk::serde::{Serialize, Deserialize};

use near_sdk::PanicOnDefault;
extern crate rand;
use crate::rand::Rng;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;

use rand_seeder::{Seeder};

near_sdk::setup_alloc!();

pub type TokenId = String;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct BulletinBoard {

    // pub owner_id: AccountId,
    
    pub post_per_owner: UnorderedMap<AccountId, UnorderedSet<TokenId>>,

    pub post_by_id: UnorderedMap<TokenId, BulletinPost>,

}


#[derive(BorshSerialize,Copy, Clone)]
pub enum StorageKey {
    PostPerOwner,
    PostById
}

#[near_bindgen]
impl BulletinBoard {
    #[init]
    pub fn new() -> Self {
         let this = Self {
            // owner_id,
            post_per_owner: UnorderedMap::new(StorageKey::PostPerOwner.try_to_vec().unwrap()),
            post_by_id: UnorderedMap::new(StorageKey::PostById.try_to_vec().unwrap())
            
        };
        this
    }
}

pub trait BulletinBoards {
    fn new_bulletinpost(&mut self, img_url: String, location: String, description: String, contact: String) -> BulletinPost;
    fn get_bulletins(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<BulletinPost>;
    fn get_bulletin(&self, id: String)-> Option<BulletinPost>;
    fn add_post_by_owner( &mut self, account_id: &AccountId, post_id: &TokenId); 
}

#[near_bindgen]
impl BulletinBoards for BulletinBoard { 
    fn new_bulletinpost(&mut self, img_url: String, location: String, description: String, contact: String) -> BulletinPost{
        // env::log(b"before everything...");
        // let mut rng = rand::thread_rng();

        // const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        //                         abcdefghijklmnopqrstuvwxyz\
        //                         0123456789)(*&^%$#@!~";
        // const PRIMARYKEY_LEN: usize = 30;

        // let primary_key: String = (0..PRIMARYKEY_LEN)
        //     .map(|_| {
        //         let idx = rng.gen_range(0..CHARSET.len());
        //         CHARSET[idx] as char
        //     })
        //     .collect();

        let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64(10);
        // let primary_key: Vec<u64> = (0..100).map(|_| gen.next_u64()).collect(); 
        let primary_key = gen.next_u64();

        // println!(".................");
        // println!("{:?}", primary_key);

        // let primary_key:String = "IJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvw".to_string();
        let new_post:BulletinPost = BulletinPost {
            id: primary_key.to_string(),
            sender: env::signer_account_id(),
            img_url: img_url,
            location: location,
            description: description,
            contact: contact
        };

        // println!(".................");
        // println!("{:?}", new_post);
        // env::log(b"new_post.id");
        let new_post_id = &new_post.id;
        // self.post_per_owner.insert(&AccountId, &new_post_id);
        self.add_post_by_owner(&env::current_account_id(), &new_post_id);
        self.post_by_id.insert(&new_post.id, &new_post);
        // self.get_bulletins( Some(U128(0)), Some(50));
        return new_post
    }

    fn get_bulletins(&self, from_index: Option<U128>, limit: Option<u64>) ->Vec<BulletinPost>{
        // return self.post_per_owner.get(&id); //cloned();
        // return self.post_per_owner;
        let start = u128::from(from_index.unwrap_or(U128(0)));
        let result =  self.post_per_owner.keys()
        .skip(start as usize)
        .take(limit.unwrap_or(50) as usize)
        .map(|post_id| self.get_bulletin(post_id.clone()).unwrap())
        .collect();
        return result;

    }

    fn get_bulletin(&self, id: String) -> Option<BulletinPost> {
        return self.post_by_id.get(&id);
    }
        
    fn add_post_by_owner(
        &mut self,
        account_id: &AccountId,
        post_id: &TokenId
    ) {
        let mut post_set = self.post_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::PostPerOwner
                .try_to_vec()
                .unwrap(),
            )
        });
        post_set.insert(post_id);
        self.post_per_owner.insert(account_id, &post_set);
    }


}
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BulletinPost {
    pub id: String,
    pub sender: AccountId,
    pub img_url: String,
    pub location: String,
    pub description: String,
    pub contact: String

}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn create_post() {
        let context = get_context(vec![]);
        testing_env!(context);
        let mut contract = BulletinBoard::new();
        let result = contract.new_bulletinpost("http://wwww.u.com".to_string(), "malaysia".to_string(), "missing...".to_string(), "+60128193931".to_string());
        let contact = contract.get_bulletin(result.id);
        // println!("{:?}", contact.as_ref().unwrap().contact);
        assert_eq!("+60128193931".to_string(), contact.unwrap().contact);
        // assert_eq!(vec!(result), contract.get_bulletins( Some(U128(0)), Some(50)));

    }
    #[test]
    fn posts() {
        let context = get_context(vec![]);
        testing_env!(context);
        let contract = BulletinBoard::new();
        let result = contract.get_bulletins( Some(U128(0)), Some(50));
    }

}