#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod tests;

use frame::prelude::*;
use frame::traits::fungible::Inspect;
use frame::traits::fungible::Mutate;
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {

	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 32],
		pub owner: T::AccountId,
		pub price: Option<BalanceOf<T>>,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 32], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32] },
		PriceSet { owner: T::AccountId, kitty_id: [u8; 32], new_price: Option<BalanceOf<T>> },
		Sold { buyer: T::AccountId, kitty_id: [u8; 32], price: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
		TransferToSelf,
		NoKitty,
		NotOwner,
		NotForSale,
		MaxPriceTooLow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}

		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: [u8; 32],
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_price(who, kitty_id, new_price)?;
			Ok(())
		}

		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: [u8; 32],
			max_price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_buy_kitty(who, kitty_id, max_price)?;
			Ok(())
		}
	}
}
