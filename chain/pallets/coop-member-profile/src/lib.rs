//! # Demo - Demo - Co-operative organic livestock socitey

#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	ensure,
	sp_std::prelude::*,
	traits::{
		Currency, Get,
		EnsureOrigin, ExistenceRequirement::{AllowDeath}},
};

use sp_runtime::{
	traits::{AccountIdConversion},
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;
pub use crate::types::*;

// type BalanceOf<T> = pallet_treasury::BalanceOf<T>;
pub type BalanceOf<T> =
	<<T as pallet_treasury::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// pub trait Config: frame_system::Config + timestamp::Trait + pallet_treasury::Config {
pub trait Trait: system::Trait + pallet_treasury::Trait + timestamp::Trait {
	/// Minimum deposit value for a membership.
	type MemberDepositValueMinimum: Get<BalanceOf<Self>>;

	/// Cooperative Society ID
	type CoopSocietyId: Get<u32>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type CreateRoleOrigin: EnsureOrigin<Self::Origin>;
}

decl_storage! {
	trait Store for Module<T: Trait> as CoopMemberProfile {
		// Member Profile
		pub MemberProfiles get(fn memberprofile_by_id): map hasher(blake2_128_concat) SocietyMemberShipId => Option<MemberProfile<T::AccountId, BalanceOf<T>, T::Moment>>;
		// Member Account to membership id
		pub MemberId get(fn member_id_by_acc): map hasher(blake2_128_concat) T::AccountId => Option<SocietyMemberShipId>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		<T as frame_system::Trait>::AccountId,
	{
		MemberProfileRegistered(AccountId, SocietyMemberShipId, AccountId),
		MemberProfileInfoUpdated(AccountId, AccountId),
		MemberProfileRoleUpdated(AccountId, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		SocietyMemberIdEmpty,
		SocietyMemberIdTooLong,
		MemberIdAlreadyExists,
		MemberAccountAlreadyExists,
		UnknownMemberAccount,
		UnknownMemberProfile,
		MemberProfileTooManyInfos,
		MemberProfileInvalidInfoName,
		MemberProfileInvalidInfoValue,
		MemberProfileInfoEmpty,
		MemberRoleInvalid,
		MemberDepositValueInvalid,
		MemberAccountBalanceLow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn register_member(
			origin,
			member_acc: T::AccountId,
			society_membership_id: SocietyMemberShipId,
			profile_info: Option<Vec<MemberProfileInfo>>,
			deposit_value: BalanceOf<T>,
		) -> DispatchResult {
			T::CreateRoleOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			// Validate format of membership ID
			Self::validate_membership_id(&society_membership_id)?;

			// Validate member profile info
			Self::validate_member_profile_info(&profile_info)?;

			// Check member doesn't exist yet (2 DB read)
			Self::validate_new_member(
				&society_membership_id,
				&member_acc,
			)?;

			ensure!(
				deposit_value >= T::MemberDepositValueMinimum::get(),
				Error::<T>::MemberDepositValueInvalid,
			);

			let member_acc_balance = T::Currency::free_balance(&member_acc);

			ensure!(
				member_acc_balance >= T::MemberDepositValueMinimum::get(),
				Error::<T>::MemberAccountBalanceLow,
			);

			let _ = T::Currency::transfer(
				&member_acc,
				&Self::coop_account_id(T::CoopSocietyId::get()),
				deposit_value,
				AllowDeath,
			);

			// Create new member profile
			let member_profile = Self::new_member_profile()
				.update_society_membership_id(society_membership_id.clone())
				.update_member_acc(member_acc.clone())
				.update_joined_date(<timestamp::Module<T>>::now())
				.update_member_profile_info(profile_info)
				.update_deposit(deposit_value)
				.build();

			// Storage writes
			// --------------
			// Add member profile (2 DB write)
			<MemberId<T>>::insert(&member_acc, society_membership_id.clone());
			<MemberProfiles<T>>::insert(&society_membership_id, member_profile);

			// Raise events
			Self::deposit_event(
				RawEvent::MemberProfileRegistered(
					who,
					society_membership_id,
					member_acc,
				)
			);

			Ok(())
		}

		#[weight = 10_000]
		pub fn update_member_profile_info(
			origin,
			member_acc: T::AccountId,
			profile_info: Option<Vec<MemberProfileInfo>>,
		){
			T::CreateRoleOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			// Check if profile info is not empty
			profile_info
				.clone()
				.ok_or(Error::<T>::MemberProfileInfoEmpty)?;

			// Validate member profile info
			Self::validate_member_profile_info(&profile_info)?;

			// Update asset profile in DB
			Self::try_mutate_memberprofiles(
				&member_acc,
				|member_profile| -> DispatchResult {
					member_profile.prof_info = profile_info;
					return Ok(())
				}
			)?;
			Self::deposit_event(
				Event::<T>::MemberProfileInfoUpdated(who, member_acc)
			);
		}

		#[weight = 10_000]
		pub fn update_member_role(
			origin,
			member_acc: T::AccountId,
			profile_role: Option<Vec<MemberRole>>,
		){
			T::CreateRoleOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			let default_member_status: MemberStatus =
				if !profile_role.is_none() {
					MemberStatus::Active
				} else {
					MemberStatus::Suspended
				};

			Self::try_mutate_memberprofiles(
				&member_acc,
				|member_profile| -> DispatchResult {
					member_profile.member_status = Some(default_member_status);
					member_profile.role = profile_role;
					return Ok(())
				}
			)?;

			Self::deposit_event(
				Event::<T>::MemberProfileRoleUpdated(who, member_acc),
			);
		}
	}
}

impl<T: Trait> Module<T> {
	// Helper methods
	fn new_member_profile() -> MemberProfileBuilder<T::AccountId, BalanceOf<T>, T::Moment> {
		MemberProfileBuilder::<T::AccountId, BalanceOf<T>, T::Moment>::default()
	}

	/// The account ID of a cooperative society account
	pub fn coop_account_id(id: CoopSocietyIndex) -> T::AccountId {
		// only use two byte prefix to support 16 byte account id (used by test)
		// "modl" ++ "py/trsry" ++ "coop-soci-" is 14 bytes,
		// and two bytes remaining for Coop Society index
		T::ModuleId::get().into_sub_account(("coop-soci-", id))
	}

	fn validate_membership_id(id: &[u8]) -> Result<(), Error<T>> {
		// Basic Society member ID validation
		ensure!(
			!id.is_empty(),
			Error::<T>::SocietyMemberIdEmpty,
		);
		ensure!(
			id.len() <= SOCIETY_MEMBERSHIP_ID_MAX_LENGTH,
			Error::<T>::SocietyMemberIdTooLong,
		);
		Ok(())
	}

	fn validate_new_member(
		id: &[u8],
		acc: &T::AccountId
	) -> Result<(), Error<T>> {
		// Member existence check
		ensure!(
			!<MemberProfiles<T>>::contains_key(id),
			Error::<T>::MemberIdAlreadyExists,
		);
		// Member account existence check
		ensure!(
			!<MemberId<T>>::contains_key(acc),
			Error::<T>::MemberAccountAlreadyExists,
		);
		Ok(())
	}

	fn validate_member_profile_info(
		profile_info: &Option<Vec<MemberProfileInfo>>
	) -> Result<(), Error<T>> {
		if let Some(profile_info) = profile_info {
			ensure!(
				profile_info.len() <= MEMBER_PROFILE_MAX_INFO,
				Error::<T>::MemberProfileTooManyInfos,
			);
			for profile in profile_info {
				ensure!(
					profile.info_name().len() <=
						MEMBER_PROFILE_INFO_NAME_MAX_LENGTH,
					Error::<T>::MemberProfileInvalidInfoName
				);
				ensure!(
					profile.info_value().len() <=
						MEMBER_PROFILE_INFO_VALUE_MAX_LENGTH,
					Error::<T>::MemberProfileInvalidInfoValue
				);
			}
		}
		Ok(())
	}

	pub fn is_valid_member_role(
		member_acc: &T::AccountId,
		role: MemberRole,
	) -> DispatchResult {
		// Check for give role on the member
		Self::try_mutate_memberprofiles(
			member_acc,
			|member_profile| -> DispatchResult {
				if let Some(member_role) = &member_profile.role {
					if !member_role.iter().any(|ele| ele == &role){
						Err(Error::<T>::MemberRoleInvalid)?
					}
					Ok(())
				} else {
					Err(Error::<T>::MemberRoleInvalid)?
				}
			}
		)?;
		Ok(())
	}

	fn try_mutate_memberprofiles(
		member_acc: &T::AccountId,
		f: impl FnOnce(&mut MemberProfile<T::AccountId, BalanceOf<T>, T::Moment>) -> DispatchResult
	) -> DispatchResult {
		<MemberId<T>>::try_mutate_exists(
			&member_acc,
			|maybe_membership_id| -> DispatchResult {
				let membership_id = maybe_membership_id
					.as_mut()
					.ok_or(Error::<T>::UnknownMemberAccount)?;
				MemberProfiles::<T>::try_mutate(
					membership_id,
					|maybe_member_profile| -> DispatchResult {
						let mut member_profile =
							maybe_member_profile
							.as_mut()
							.ok_or(Error::<T>::UnknownMemberProfile)?;
						f(&mut member_profile)?;
						Ok(())
					}
				)?;
				Ok(())
			}
		)?;
		Ok(())
	}
}
