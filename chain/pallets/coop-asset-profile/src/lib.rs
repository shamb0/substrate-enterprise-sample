//! # Demo - Demo - Co-operative organic livestock socitey

#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	ensure,
	sp_std::prelude::*,
	traits::{
		Currency, Get,
		EnsureOrigin, ExistenceRequirement::{AllowDeath},
	},
};
use frame_system::{self as system, ensure_signed};
use pallet_coop_member_profile::MemberRole;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;
use crate::types::*;

// type BalanceOf<T> = pallet_treasury::BalanceOf<T>;
pub type BalanceOf<T> =
	<<T as pallet_treasury::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait + timestamp::Trait + pallet_coop_member_profile::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as CoopMemberProfile {
		// Asset Profile Collections
		pub AssetProfiles get(fn assetprofile_by_id): map hasher(blake2_128_concat) AssetId => Option<AssetProfile<T::AccountId, BalanceOf<T>, T::Moment>>;
		// In-Q Asset Profile Requested for registration
		pub AssetRegistrationInQ get(fn asset_register_inq): map hasher(blake2_128_concat) AssetId => Option<AssetProfile<T::AccountId, BalanceOf<T>, T::Moment>>;
		// In-Q Asset Insurance Re-claim
		pub AssetInsuranceReclaimInQ get(fn asset_insurance_reclaim_inq): map hasher(blake2_128_concat) AssetId => Option<AssetInsuranceReClaim<T::Moment>>;
		// In-Q Asset Healthcheck record
		pub AssetHealthCheckRecInQ get(fn asset_healthcheck_rec_inq): map hasher(blake2_128_concat) AssetId => Option<AssetHealthCheckRec<T::Moment>>;
		// pub AssetHealthCheckRec get(fn asset_healthcheck_rec_inq): map hasher(blake2_128_concat) AssetId => Option<AssetInsuranceReClaim<T::Moment>>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Balance = BalanceOf<T>,
	{
		RequestAssetRegistration(AccountId, AssetId),
		ProcessRequestAssetRegistration(AccountId, AssetId, bool),
		RequestAssetInsurance(AccountId, AssetId),
		AssetInsurancePremiumQuoteUpdate(AccountId, AssetId, Balance),
		AssetInsurancePremiumDepositUpdate(AccountId, AssetId, Balance),
		AssetInsuranceApproved(AccountId, AssetId),
		AssetInsuranceReclaim(AccountId, AssetId),
		AssetInsuranceReclaimCommunityApproved(AccountId, AssetId),
		AssetInsuranceReclaimInsurerApproved(AccountId, AssetId),
		AssetHealthCheckRequest(AccountId, AssetId),
		AssetHealthCheckHealthOfficerUpdate(AccountId, AssetId),
		AssetHealthCheckCommunityUpdate(AccountId, AssetId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		AssetIdEmpty,
		AssetIdTooLong,
		AssetIdInvalid,
		AssetIdAlreadyExists,
		AssetProfileTooManyInfos,
		AssetProfileInvalidInfoName,
		AssetProfileInvalidInfoValue,
		AssetProfileInfoEmpty,
		AssetStatusUnexpected,
		AssetOwnerInvalid,
		AssetOwnerUnAssigned,
		AssetInsuranceRequestNotNew,
		AssetInsuranceStatusUnexpected,
		AssetInsuranceRequestNone,
		AssetInsuranceReClaimTooManyInfos,
		AssetInsuranceReClaimInvalidInfoName,
		AssetInsuranceReClaimInvalidInfoValue,
		AssetInsuranceReclaimAlreadyExists,
		AssetReClaimIdInvalid,
		AssetInsuranceReClaimStatusUnexpected,
		AssetHealthCheckRequestAlreadyExists,
		AssetHealthCheckTooManyInfos,
		AssetHealthCheckInvalidInfoName,
		AssetHealthCheckInvalidInfoValue,
		AssetHealthCheckRecIdInvalid,
		AssetHealthCheckRecStatusUnexpected,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn register_asset(
			origin,
			asset_id: AssetId,
			profile_info: Option<Vec<AssetProfileInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::AssetOwner,
			)?;

			// Validate format of asset ID
			Self::validate_asset_id(&asset_id)?;

			// Validate asset profile info
			Self::validate_asset_profile_info(&profile_info)?;

			// Check asset doesn't exist yet (2 DB read)
			Self::validate_is_new_asset(&asset_id)?;

			// Create new asset profile
			let asset_profile = Self::new_asset_profile()
				.update_asset_id(asset_id.clone())
				.update_owner(who.clone())
				.update_keeper(who.clone())
				.update_joined_date(<timestamp::Module<T>>::now())
				.update_asset_profile_info(profile_info)
				.build();

			// Storage writes
			// --------------
			// Add member profile (1 DB write)
			<AssetRegistrationInQ<T>>::insert(&asset_id, asset_profile);

			// Raise events
			Self::deposit_event(
				RawEvent::RequestAssetRegistration(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn process_new_asset_register_request(
			origin,
			asset_id: AssetId,
			do_approve: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::CommunityLeader,
			)?;

			AssetRegistrationInQ::<T>::try_mutate_exists(
				&asset_id,
				|maybe_asset_profile| -> DispatchResult {
					let mut asset_profile =
						maybe_asset_profile
						.take()
						.ok_or(Error::<T>::AssetIdInvalid)?;

					if do_approve {
						ensure!(
							asset_profile.asset_status == AssetStatus::NewRegi,
							Error::<T>::AssetStatusUnexpected,
						);
						asset_profile.asset_status = AssetStatus::InFarm;

						AssetProfiles::<T>::insert(
							&asset_id,
							asset_profile,
						);
					}
					*maybe_asset_profile = None;
					Ok(())
				}
			)?;

			// Raise events
			Self::deposit_event(
				RawEvent::ProcessRequestAssetRegistration(
					who,
					asset_id,
					do_approve
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn request_insurance(
			origin,
			asset_id: AssetId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Asset Owner
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::AssetOwner,
			)?;

			// check for validity of asset & Asset Ownership by the member
			Self::validate_asset_ownership(
				&asset_id,
				&who,
			)?;

			// check insurance request is new ?
			Self::validate_asset_insurance_is_none(
				&asset_id,
			)?;

			// create new AssetInsurance object
			let asset_insurance = Self::new_asset_insurance()
				.update_asset_id(asset_id.clone())
				.build();

			Self::try_mutate_assetprofiles(
				&asset_id,
				|asset_profile| -> DispatchResult {
				if asset_profile.asset_status == AssetStatus::InFarm {
						asset_profile.asset_insurance = Some(asset_insurance);
						return Ok(())
					} else {
						return Err(Error::<T>::AssetStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::RequestAssetInsurance(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn update_insurance_premium(
			origin,
			asset_id: AssetId,
			premium: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Insurance Agent
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::Insurer,
			)?;

			// check insurance request is active & new ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::NewApplication,
			)?;

			Self::try_mutate_assetprofiles(
				&asset_id,
				|asset_profile| -> DispatchResult {
					if asset_profile.asset_status == AssetStatus::InFarm {
						if let Some(mut asset_insurance) = asset_profile.asset_insurance.as_mut() {
							asset_insurance.quoted_premium = premium;
							asset_insurance.status = AssetInsuranceStatus::PremiumQuoted;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetInsurancePremiumQuoteUpdate(
					who,
					asset_id,
					premium,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn deposit_insurance_premium(
			origin,
			asset_id: AssetId,
			premium_deposit: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Insurance Agent
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::AssetOwner,
			)?;

			// check for validity of asset & Asset Ownership by the member
			Self::validate_asset_ownership(
				&asset_id,
				&who,
			)?;

			// check insurance request is active & new ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::PremiumQuoted,
			)?;

			Self::try_mutate_assetprofiles(
				&asset_id,
				|asset_profile| -> DispatchResult {
					if asset_profile.asset_status == AssetStatus::InFarm {
						if let Some(mut asset_insurance) = asset_profile.asset_insurance.as_mut() {
							let _ = T::Currency::transfer(
								&who,
								&pallet_coop_member_profile::Module::<T>::coop_account_id(
									<T as pallet_coop_member_profile::Trait>::CoopSocietyId::get()
								),
								premium_deposit,
								AllowDeath,
							);
							asset_insurance.status = AssetInsuranceStatus::PremiumPaid;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetInsurancePremiumDepositUpdate(
					who,
					asset_id,
					premium_deposit,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn approve_insurance(
			origin,
			asset_id: AssetId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// check for valid member role, Insurance Agent
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::Insurer,
			)?;
			// check insurance request is active & new ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::PremiumPaid,
			)?;
			// Update asset profile in DB
			Self::try_mutate_assetprofiles(
				&asset_id,
				|asset_profile| -> DispatchResult {
					if asset_profile.asset_status == AssetStatus::InFarm {
						if let Some(mut asset_insurance) = asset_profile.asset_insurance.as_mut() {
							asset_insurance.start_date = <timestamp::Module<T>>::now();
							asset_insurance.expiry_date = <timestamp::Module<T>>::now();
							asset_insurance.status = AssetInsuranceStatus::Active;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetInsuranceApproved(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn request_insurance_reclaim(
			origin,
			asset_id: AssetId,
			reclaim_info: Option<Vec<AssetInsuranceReClaimInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Asset Owner
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::AssetOwner,
			)?;

			// check for validity of asset & Asset Ownership by the member
			Self::validate_asset_ownership(
				&asset_id,
				&who,
			)?;

			// check for valide asset status
			Self::validate_asset_status(
				&asset_id,
				AssetStatus::InFarm,
			)?;

			// check insurance is active ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::Active,
			)?;

			// Validate insurance reclaim info
			Self::validate_asset_insurance_reclaim_info(
				&reclaim_info
			)?;

			// check In-Q for insurance reclaim to avoid duplicates
			Self::validate_is_new_insurance_reclaim(
				&asset_id,
			)?;

			// create new AssetInsurance object
			let asset_insurance_reclaim = Self::new_asset_insurance_reclaim()
				.update_asset_id(asset_id.clone())
				.update_owner_note(reclaim_info)
				.build();

			// Storage writes
			// --------------
			// Add reclaim to In-Q (1 DB write)
			<AssetInsuranceReclaimInQ<T>>::insert(&asset_id, asset_insurance_reclaim);

			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetInsuranceReclaim(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn community_approve_insurance_reclaim(
			origin,
			asset_id: AssetId,
			approve: bool,
			community_reclaim_note: Option<Vec<AssetInsuranceReClaimInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Asset Owner
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::CommunityLeader,
			)?;

			// check for valide asset status
			Self::validate_asset_status(
				&asset_id,
				AssetStatus::InFarm,
			)?;

			// check insurance is active ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::Active,
			)?;

			// Validate insurance reclaim info
			Self::validate_asset_insurance_reclaim_info(
				&community_reclaim_note,
			)?;

			Self::try_mutate_assetinsurancereclaiminq(
				&asset_id,
				|reclaim_info| -> DispatchResult {
					if reclaim_info.status == AssetInsuranceReClaimStatus::NewReclaim {
						reclaim_info.community_note = community_reclaim_note;
						if approve {
							reclaim_info.status =
								AssetInsuranceReClaimStatus::CommunityApproved;
						} else {
							reclaim_info.status =
								AssetInsuranceReClaimStatus::CommunityDisApproved;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetInsuranceReClaimStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetInsuranceReclaimCommunityApproved(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn insurer_approve_insurance_reclaim(
			origin,
			asset_id: AssetId,
			approve: bool,
			insurer_reclaim_note: Option<Vec<AssetInsuranceReClaimInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Insurer
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::Insurer,
			)?;

			// check for valide asset status
			Self::validate_asset_status(
				&asset_id,
				AssetStatus::InFarm,
			)?;

			// check insurance is Active ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::Active,
			)?;

			// Validate insurance reclaim info
			Self::validate_asset_insurance_reclaim_info(
				&insurer_reclaim_note,
			)?;
			// Update the Insurance reclaim object in-Q
			Self::try_mutate_assetinsurancereclaiminq(
				&asset_id,
				|reclaim_info| -> DispatchResult {
					if reclaim_info.status == AssetInsuranceReClaimStatus::CommunityApproved {
						reclaim_info.insurer_note = insurer_reclaim_note;
						if approve {
							reclaim_info.status =
								AssetInsuranceReClaimStatus::InsurerApproved;
						} else {
							reclaim_info.status =
								AssetInsuranceReClaimStatus::InsurerDisApproved;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetInsuranceReClaimStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetInsuranceReclaimInsurerApproved(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn request_healthcheck(
			origin,
			asset_id: AssetId,
			request_info: Option<Vec<AssetHealthCheckRecInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Asset Owner
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::AssetOwner,
			)?;

			// check for validity of asset & Asset Ownership by the member
			Self::validate_asset_ownership(
				&asset_id,
				&who,
			)?;

			// check for valide asset status
			Self::validate_asset_status(
				&asset_id,
				AssetStatus::InFarm,
			)?;

			// check insurance is active ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::Active,
			)?;

			// Validate health-check request info
			Self::validate_asset_healthcheck_info(
				&request_info,
			)?;

			// check In-Q for insurance reclaim to avoid duplicates
			Self::validate_is_new_healthcheck_request(
				&asset_id,
			)?;

			// create new healthcheck object
			let asset_healthcheck_request = Self::new_asset_healthcheck_record()
				.update_asset_id(asset_id.clone())
				.update_owner_note(request_info)
				.build();

			// Storage writes
			// --------------
			// Add healthcheck request to In-Q (1 DB write)
			<AssetHealthCheckRecInQ<T>>::insert(&asset_id, asset_healthcheck_request);

			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetHealthCheckRequest(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn healthofficer_remark(
			origin,
			asset_id: AssetId,
			treatment_done: bool,
			healthofficer_note: Option<Vec<AssetHealthCheckRecInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Insurer
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::HealthOfficer,
			)?;

			// check for valide asset status
			Self::validate_asset_status(
				&asset_id,
				AssetStatus::InFarm,
			)?;

			// check insurance is Active ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::Active,
			)?;

			// Validate health officer note
			Self::validate_asset_healthcheck_info(
				&healthofficer_note,
			)?;

			Self::try_mutate_assethealthcheckrecinq(
				&asset_id,
				|healthcheck_rec| -> DispatchResult {
					if healthcheck_rec.status == AssetHealthCheckRecStatus::NewReq ||
						healthcheck_rec.status == AssetHealthCheckRecStatus::TreatmentInProgress
					{
						if let Some(healthofficer_remark) = &mut healthcheck_rec.health_officer_note {
							healthofficer_remark.append(
								&mut healthofficer_note.unwrap_or_default()
							);
						}
						if treatment_done {
							healthcheck_rec.status =
								AssetHealthCheckRecStatus::TreatmentDone;
						} else {
							healthcheck_rec.status =
								AssetHealthCheckRecStatus::TreatmentInProgress;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetHealthCheckRecStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetHealthCheckHealthOfficerUpdate(
					who,
					asset_id,
				)
			);
			Ok(())
		}

		#[weight = 10_000]
		pub fn community_remark(
			origin,
			asset_id: AssetId,
			approved: bool,
			community_remark: Option<Vec<AssetHealthCheckRecInfo>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for valid member role, Insurer
			pallet_coop_member_profile::Module::<T>::is_valid_member_role(
				&who,
				MemberRole::CommunityLeader,
			)?;

			// check for valide asset status
			Self::validate_asset_status(
				&asset_id,
				AssetStatus::InFarm,
			)?;

			// check insurance is Active ?
			Self::validate_asset_insurance_status(
				&asset_id,
				AssetInsuranceStatus::Active,
			)?;

			// Validate health officer note
			Self::validate_asset_healthcheck_info(
				&community_remark,
			)?;

			Self::try_mutate_assethealthcheckrecinq(
				&asset_id,
				|healthcheck_rec| -> DispatchResult {
					if healthcheck_rec.status == AssetHealthCheckRecStatus::TreatmentDone {
						if let Some(community_note) = &mut healthcheck_rec.community_note {
							community_note.append(
								&mut community_remark.unwrap_or_default()
							);
						}
						if approved {
							healthcheck_rec.status =
								AssetHealthCheckRecStatus::CommunityApproved;
						}
						return Ok(())
					} else {
						return Err(Error::<T>::AssetHealthCheckRecStatusUnexpected.into())
					}
				}
			)?;
			// Trigget the event
			Self::deposit_event(
				RawEvent::AssetHealthCheckCommunityUpdate(
					who,
					asset_id,
				)
			);
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	// Helper methods
	fn new_asset_profile()
		-> AssetProfileBuilder<T::AccountId, BalanceOf<T>, T::Moment> {
		AssetProfileBuilder::<T::AccountId, BalanceOf<T>, T::Moment>::default()
	}

	fn new_asset_insurance()
		-> AssetInsuranceBuilder<BalanceOf<T>, T::Moment> {
		AssetInsuranceBuilder::<BalanceOf<T>, T::Moment>::default()
	}

	fn new_asset_insurance_reclaim()
		-> AssetInsuranceReClaimBuilder<T::Moment> {
		AssetInsuranceReClaimBuilder::<T::Moment>::default()
	}

	fn new_asset_healthcheck_record()
		-> AssetHealthCheckRecBuilder<T::Moment> {
		AssetHealthCheckRecBuilder::<T::Moment>::default()
	}

	fn validate_asset_id(id: &[u8]) -> DispatchResult {
		// Basic Society member ID validation
		ensure!(!id.is_empty(), Error::<T>::AssetIdEmpty);
		ensure!(id.len() <= ASSET_ID_MAX_LENGTH, Error::<T>::AssetIdTooLong);
		Ok(())
	}

	fn validate_asset_profile_info(
		profile_info: &Option<Vec<AssetProfileInfo>>,
	) -> DispatchResult {
		if let Some(profile_info) = profile_info {
			ensure!(
				profile_info.len() <= ASSET_PROFILE_MAX_INFO,
				Error::<T>::AssetProfileTooManyInfos,
			);
			for profile in profile_info {
				ensure!(
					profile.info_name().len() <= ASSET_INFO_NAME_MAX_LENGTH,
					Error::<T>::AssetProfileInvalidInfoName,
				);
				ensure!(
					profile.info_value().len() <= ASSET_INFO_VALUE_MAX_LENGTH,
					Error::<T>::AssetProfileInvalidInfoValue,
				);
			}
		}
		Ok(())
	}

	fn validate_is_new_asset(id: &[u8]) -> DispatchResult {
		// Asset existence check
		ensure!(
			!<AssetRegistrationInQ<T>>::contains_key(id),
			Error::<T>::AssetIdAlreadyExists,
		);
		ensure!(
			!<AssetProfiles<T>>::contains_key(id),
			Error::<T>::AssetIdAlreadyExists,
		);
		Ok(())
	}

	fn validate_asset_ownership(
		asset_id: &AssetId,
		acc: &T::AccountId,
	) -> DispatchResult {
		// Asset ownership check
		let asset_profile = Self::assetprofile_by_id(&asset_id)
			.ok_or(Error::<T>::AssetIdInvalid)?;
		let asset_owners = asset_profile
			.asset_owners
			.ok_or(Error::<T>::AssetOwnerUnAssigned)?;
		ensure!(
			asset_owners.iter().last().unwrap() == acc,
			Error::<T>::AssetOwnerInvalid,
		);
		Ok(())
	}

	fn validate_asset_status(
		asset_id: &AssetId,
		exp_status: AssetStatus,
	) -> DispatchResult {
		// check asset status
		let asset_profile =
			Self::assetprofile_by_id(&asset_id)
			.ok_or(Error::<T>::AssetIdInvalid)?;
		ensure!(
			asset_profile.asset_status == exp_status,
			Error::<T>::AssetStatusUnexpected,
		);
		Ok(())
	}

	fn validate_asset_insurance_is_none(
		asset_id: &AssetId,
	) -> DispatchResult {
		// Check asset insurance status in asset profile
		let asset_profile =
			Self::assetprofile_by_id(&asset_id)
			.ok_or(Error::<T>::AssetIdInvalid)?;
		ensure!(
			asset_profile.asset_insurance.is_none(),
			Error::<T>::AssetInsuranceRequestNotNew,
		);
		Ok(())
	}

	fn validate_asset_insurance_status(
		asset_id: &AssetId,
		exp_status: AssetInsuranceStatus,
	) -> DispatchResult {
		// Check asset insurance status
		let asset_profile = Self::assetprofile_by_id(&asset_id)
			.ok_or(Error::<T>::AssetIdInvalid)?;

		ensure!(
			!asset_profile.asset_insurance.is_none(),
			Error::<T>::AssetInsuranceRequestNone,
		);

		if let Some(asset_insurance) = &asset_profile.asset_insurance {
			ensure!(
				asset_insurance.status == exp_status,
				Error::<T>::AssetInsuranceStatusUnexpected,
			);
		}
		Ok(())
	}

	fn try_mutate_assetprofiles(
		asset_id: &AssetId,
		f: impl FnOnce(&mut AssetProfile<T::AccountId, BalanceOf<T>, T::Moment>) -> DispatchResult
	) -> DispatchResult {
		AssetProfiles::<T>::try_mutate(
			&asset_id,
			|maybe_asset_profile| -> DispatchResult {
				let mut asset_profile =
					maybe_asset_profile
					.as_mut()
					.ok_or(Error::<T>::AssetIdInvalid)?;

				f(&mut asset_profile)?;
				Ok(())
			}
		)?;
		Ok(())
	}

	fn validate_is_new_insurance_reclaim(id: &[u8]) -> DispatchResult {
		// Asset existence check
		ensure!(
			!<AssetInsuranceReclaimInQ<T>>::contains_key(id),
			Error::<T>::AssetInsuranceReclaimAlreadyExists,
		);
		Ok(())
	}

	fn validate_asset_insurance_reclaim_info(
		reclaim_info: &Option<Vec<AssetInsuranceReClaimInfo>>,
	) -> DispatchResult {
		if let Some(reclaim_info) = reclaim_info {
			ensure!(
				reclaim_info.len() <= ASSET_INSURANCE_RECLAIM_MAX_INFO,
				Error::<T>::AssetInsuranceReClaimTooManyInfos,
			);
			for reclaim_note in reclaim_info {
				ensure!(
					reclaim_note.info_name().len() <= ASSET_INFO_NAME_MAX_LENGTH,
					Error::<T>::AssetInsuranceReClaimInvalidInfoName,
				);
				ensure!(
					reclaim_note.info_value().len() <= ASSET_INFO_VALUE_MAX_LENGTH,
					Error::<T>::AssetInsuranceReClaimInvalidInfoValue,
				);
			}
		}
		Ok(())
	}

	fn try_mutate_assetinsurancereclaiminq(
		asset_id: &AssetId,
		f: impl FnOnce(&mut AssetInsuranceReClaim<T::Moment>) -> DispatchResult
	) -> DispatchResult {
		AssetInsuranceReclaimInQ::<T>::try_mutate(
			&asset_id,
			|maybe_reclaim_info| -> DispatchResult {
				let mut reclaim_info =
					maybe_reclaim_info
					.as_mut()
					.ok_or(Error::<T>::AssetReClaimIdInvalid)?;

				f(&mut reclaim_info)?;
				Ok(())
			}
		)?;
		Ok(())
	}

	fn validate_is_new_healthcheck_request(id: &[u8]) -> DispatchResult {
		// Asset existence check
		ensure!(
			!<AssetHealthCheckRecInQ<T>>::contains_key(id),
			Error::<T>::AssetHealthCheckRequestAlreadyExists,
		);
		Ok(())
	}

	fn validate_asset_healthcheck_info(
		healthcheck_info: &Option<Vec<AssetHealthCheckRecInfo>>,
	) -> DispatchResult {
		if let Some(healthcheck_info) = healthcheck_info {
			ensure!(
				healthcheck_info.len() <= ASSET_HEALTHCHECK_MAX_INFO,
				Error::<T>::AssetHealthCheckTooManyInfos,
			);
			for healthcheck_note in healthcheck_info {
				ensure!(
					healthcheck_note.info_name().len() <= ASSET_INFO_NAME_MAX_LENGTH,
					Error::<T>::AssetHealthCheckInvalidInfoName,
				);
				ensure!(
					healthcheck_note.info_value().len() <= ASSET_INFO_VALUE_MAX_LENGTH,
					Error::<T>::AssetHealthCheckInvalidInfoValue,
				);
			}
		}
		Ok(())
	}

	fn try_mutate_assethealthcheckrecinq(
		asset_id: &AssetId,
		f: impl FnOnce(&mut AssetHealthCheckRec<T::Moment>) -> DispatchResult
	) -> DispatchResult {
		AssetHealthCheckRecInQ::<T>::try_mutate(
			&asset_id,
			|maybe_healthcheck_rec| -> DispatchResult {
				let mut healthcheck_rec =
					maybe_healthcheck_rec
					.as_mut()
					.ok_or(Error::<T>::AssetHealthCheckRecIdInvalid)?;

				f(&mut healthcheck_rec)?;
				Ok(())
			}
		)?;
		Ok(())
	}
}
