










#![cfg(test)]

use crate as pallet_kitties;
use crate::*;
use frame::deps::frame_support::runtime;
use frame::deps::sp_io;
use frame::runtime::prelude::*;
use frame::testing_prelude::*;
use frame::traits::fungible::*;

type Balance = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;





const ALICE: u64 = 1;
const BOB: u64 = 2;
const DEFAULT_KITTY: Kitty<TestRuntime> = Kitty { dna: [0u8; 32], owner: 0, price: None };

#[runtime]
mod runtime {
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeTask,
		RuntimeHoldReason,
		RuntimeFreezeReason
	)]
	#[runtime::runtime]
	
	
	
	
	pub struct TestRuntime;

	
	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<TestRuntime>;

	
	#[runtime::pallet_index(1)]
	pub type PalletBalances = pallet_balances::Pallet<TestRuntime>;

	
	#[runtime::pallet_index(2)]
	pub type PalletKitties = pallet_kitties::Pallet<TestRuntime>;
}



#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}



#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}



impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type NativeBalance = PalletBalances;
}





pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn starting_template_is_sane() {
	new_test_ext().execute_with(|| {
		let event = Event::<TestRuntime>::Created { owner: ALICE };
		let _runtime_event: RuntimeEvent = event.into();
		let _call = Call::<TestRuntime>::create_kitty {};
		let result = PalletKitties::create_kitty(RuntimeOrigin::signed(BOB));
		assert_ok!(result);
	});
}

#[test]
fn system_and_balances_work() {
	
	new_test_ext().execute_with(|| {
		
		assert_ok!(PalletBalances::mint_into(&ALICE, 100));
		assert_ok!(PalletBalances::mint_into(&BOB, 100));
	});
}

#[test]
fn create_kitty_checks_signed() {
	new_test_ext().execute_with(|| {
		
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		
		assert_noop!(PalletKitties::create_kitty(RuntimeOrigin::none()), DispatchError::BadOrigin);
	})
}

#[test]
fn create_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		
		System::assert_last_event(Event::<TestRuntime>::Created { owner: 1 }.into());
	})
}

#[test]
fn count_for_kitties_created_correctly() {
	new_test_ext().execute_with(|| {
		
		assert_eq!(CountForKitties::<TestRuntime>::get(), 0);
		
		CountForKitties::<TestRuntime>::set(1337u32);
		
		CountForKitties::<TestRuntime>::put(1337u32);
	})
}

#[test]
fn mint_increments_count_for_kitty() {
	new_test_ext().execute_with(|| {
		
		assert_eq!(CountForKitties::<TestRuntime>::get(), 0);
		
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		
		assert_eq!(CountForKitties::<TestRuntime>::get(), 1);
	})
}

#[test]
fn mint_errors_when_overflow() {
	new_test_ext().execute_with(|| {
		
		CountForKitties::<TestRuntime>::set(u32::MAX);
		
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyKitties
		);
	})
}

#[test]
fn kitties_map_created_correctly() {
	new_test_ext().execute_with(|| {
		let zero_key = [0u8; 32];
		assert!(!Kitties::<TestRuntime>::contains_key(zero_key));
		Kitties::<TestRuntime>::insert(zero_key, DEFAULT_KITTY);
		assert!(Kitties::<TestRuntime>::contains_key(zero_key));
	})
}

#[test]
fn create_kitty_adds_to_map() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);
	})
}

#[test]
fn cannot_mint_duplicate_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(ALICE, [0u8; 32]));
		assert_noop!(PalletKitties::mint(BOB, [0u8; 32]), Error::<TestRuntime>::DuplicateKitty);
	})
}

#[test]
fn kitty_struct_has_expected_traits() {
	new_test_ext().execute_with(|| {
		let kitty = DEFAULT_KITTY;
		let bytes = kitty.encode();
		let _decoded_kitty = Kitty::<TestRuntime>::decode(&mut &bytes[..]).unwrap();
		assert!(Kitty::<TestRuntime>::max_encoded_len() > 0);
		let _info = Kitty::<TestRuntime>::type_info();
	})
}

#[test]
fn mint_stores_owner_in_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(1337, [42u8; 32]));
		let kitty = Kitties::<TestRuntime>::get([42u8; 32]).unwrap();
		assert_eq!(kitty.owner, 1337);
		assert_eq!(kitty.dna, [42u8; 32]);
	})
}

#[test]
fn create_kitty_makes_unique_kitties() {
	new_test_ext().execute_with(|| {
		
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(BOB)));
		
		assert_eq!(CountForKitties::<TestRuntime>::get(), 2);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 2);
	})
}

#[test]
fn kitties_owned_created_correctly() {
	new_test_ext().execute_with(|| {
		
		assert_eq!(KittiesOwned::<TestRuntime>::get(1).len(), 0);
		
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		
		assert_eq!(KittiesOwned::<TestRuntime>::get(1).len(), 2);
	});
}

#[test]
fn cannot_own_too_many_kitties() {
	new_test_ext().execute_with(|| {
		
		for _ in 0..100 {
			assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		}
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyOwned
		);
	});
}

#[test]
fn transfer_emits_event() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		System::assert_last_event(
			Event::<TestRuntime>::Transferred { from: ALICE, to: BOB, kitty_id }.into(),
		);
	});
}

#[test]
fn transfer_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		let kitty_id = kitty.dna;
		assert_eq!(kitty.owner, ALICE);
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![kitty_id]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![]);
		
		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(ALICE), ALICE, kitty_id),
			Error::<TestRuntime>::TransferToSelf
		);
		
		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, [0u8; 32]),
			Error::<TestRuntime>::NoKitty
		);
		
		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(BOB), ALICE, kitty_id),
			Error::<TestRuntime>::NotOwner
		);
		
		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![kitty_id]);
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(kitty.owner, BOB);
	});
}

#[test]
fn native_balance_associated_type_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(<<TestRuntime as Config>::NativeBalance as Mutate<_>>::mint_into(&ALICE, 1337));
		assert_eq!(
			<<TestRuntime as Config>::NativeBalance as Inspect<_>>::total_balance(&ALICE),
			1337
		);
	});
}
