use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::tokens::Preservation;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);

		BlakeTwo256::hash_of(&unique_payload).into()
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone(), price: None };

		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u32 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == from, Error::<T>::NotOwner);

		kitty.owner = to.clone();
		kitty.price = None;
		let mut to_owned = KittiesOwned::<T>::get(to.clone());
		to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyOwned)?;

		let mut from_owned = KittiesOwned::<T>::get(from.clone());
		if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
			from_owned.swap_remove(ind);
		} else {
			return Err(Error::<T>::NoKitty.into());
		}

		Kitties::<T>::insert(kitty_id, kitty);
		KittiesOwned::<T>::insert(&to, to_owned);
		KittiesOwned::<T>::insert(&from, from_owned);

		Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id });
		Ok(())
	}

	pub fn do_set_price(
		caller: T::AccountId,
		kitty_id: [u8; 32],
		new_price: Option<BalanceOf<T>>,
	) -> DispatchResult {
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == caller, Error::<T>::NotOwner);
		kitty.price = new_price;
		Kitties::<T>::insert(kitty_id, kitty);

		Self::deposit_event(Event::<T>::PriceSet { owner: caller, kitty_id, new_price });
		Ok(())
	}

	pub fn do_buy_kitty(
		buyer: T::AccountId,
		kitty_id: [u8; 32],
		price: BalanceOf<T>,
	) -> DispatchResult {
		let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		let real_price = kitty.price.ok_or(Error::<T>::NotForSale)?;
		ensure!(price >= real_price, Error::<T>::MaxPriceTooLow);

		T::NativeBalance::transfer(&buyer, &kitty.owner, real_price, Preservation::Preserve)?;
		Self::do_transfer(kitty.owner, buyer.clone(), kitty_id)?;

		Self::deposit_event(Event::<T>::Sold { buyer, kitty_id, price: real_price });
		Ok(())
	}
}
