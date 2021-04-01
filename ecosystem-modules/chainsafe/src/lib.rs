// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{pallet_prelude::*, transactional};
use frame_system::pallet_prelude::*;
use orml_traits::MultiCurrency;
use primitives::{Balance, CurrencyId};
use sp_runtime::SaturatedConversion;
use sp_std::vec::Vec;

pub use module::*;

#[frame_support::pallet]
pub mod module {
	use super::*;

	type ResourceId = chainbridge::ResourceId;

	#[pallet::config]
	pub trait Config: frame_system::Config + chainbridge::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: MultiCurrency<Self::AccountId, CurrencyId = CurrencyId, Balance = Balance>;

		#[pallet::constant]
		type NativeCurrencyId: Get<CurrencyId>;

		type RegistorOrigin: EnsureOrigin<Self::Origin>;

		/// Specifies the origin check provided by the bridge for calls that can
		/// only be called by the bridge pallet
		type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidDestChainId,
		ResourceIdAlreadyRegistered,
		ResourceIdNotRegistered,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		RegisteredResourceId(ResourceId, CurrencyId),
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	#[pallet::getter(fn resource_ids)]
	pub type ResourceIds<T: Config> = StorageMap<_, Twox64Concat, CurrencyId, ResourceId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn currency_ids)]
	pub type CurrencyIds<T: Config> = StorageMap<_, Twox64Concat, ResourceId, CurrencyId, OptionQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000_000)]
		#[transactional]
		pub fn register_resource_id(
			origin: OriginFor<T>,
			resource_id: ResourceId,
			currency_id: CurrencyId,
		) -> DispatchResultWithPostInfo {
			T::RegistorOrigin::ensure_origin(origin)?;
			ensure!(
				!ResourceIds::<T>::contains_key(currency_id) && !CurrencyIds::<T>::contains_key(resource_id),
				Error::<T>::ResourceIdAlreadyRegistered,
			);
			ResourceIds::<T>::insert(currency_id, resource_id);
			CurrencyIds::<T>::insert(resource_id, currency_id);
			Self::deposit_event(Event::RegisteredResourceId(resource_id, currency_id));
			Ok(().into())
		}

		#[pallet::weight(1_000_000)]
		#[transactional]
		pub fn remove_resource_id(origin: OriginFor<T>, resource_id: ResourceId) -> DispatchResultWithPostInfo {
			T::RegistorOrigin::ensure_origin(origin)?;
			if let Some(currency_id) = CurrencyIds::<T>::take(resource_id) {
				ResourceIds::<T>::remove(currency_id);
			}
			Ok(().into())
		}

		#[pallet::weight(1_000_000)]
		#[transactional]
		pub fn transfer_to_bridge(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			dest_chain_id: chainbridge::ChainId,
			recipient: Vec<u8>,
			amount: Balance,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_transfer_to_bridge(&who, currency_id, dest_chain_id, recipient, amount)?;
			Ok(().into())
		}

		#[pallet::weight(1_000_000)]
		#[transactional]
		pub fn transfer_native_to_bridge(
			origin: OriginFor<T>,
			dest_chain_id: chainbridge::ChainId,
			recipient: Vec<u8>,
			amount: Balance,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_transfer_to_bridge(&who, T::NativeCurrencyId::get(), dest_chain_id, recipient, amount)?;
			Ok(().into())
		}

		#[pallet::weight(1_000_000)]
		#[transactional]
		pub fn transfer_from_bridge(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: Balance,
			resource_id: ResourceId,
		) -> DispatchResultWithPostInfo {
			let bridge_account_id = T::BridgeOrigin::ensure_origin(origin)?;
			let currency_id = Self::currency_ids(resource_id).ok_or(Error::<T>::ResourceIdNotRegistered)?;
			T::Currency::transfer(currency_id, &bridge_account_id, &to, amount)?;
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn do_transfer_to_bridge(
		from: &T::AccountId,
		currency_id: CurrencyId,
		dest_chain_id: chainbridge::ChainId,
		recipient: Vec<u8>,
		amount: Balance,
	) -> DispatchResult {
		ensure!(
			chainbridge::Module::<T>::chain_whitelisted(dest_chain_id),
			Error::<T>::InvalidDestChainId
		);

		let bridge_account_id = chainbridge::Module::<T>::account_id();
		let resource_id = Self::resource_ids(currency_id).ok_or(Error::<T>::ResourceIdNotRegistered)?;
		T::Currency::transfer(currency_id, &from, &bridge_account_id, amount)?;
		chainbridge::Module::<T>::transfer_fungible(
			dest_chain_id,
			resource_id,
			recipient,
			sp_core::U256::from(amount.saturated_into::<u128>()),
		)
	}
}
