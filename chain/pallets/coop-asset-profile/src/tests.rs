// Unit Tests for pallet-coop-asset-profile

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, dispatch};
use pallet_coop_member_profile::MemberProfileInfo;

const TEST_NULL_STR: &str = "";
const TEST_LONG_STR: &str = "abcdefghijklmnopqrstuvwxyz
	| ABCDEFGHIJKLMNOPQRSTUVWXYZ
	| 0123456789";
pub const TEST_COMMUNITY_HEAD: &str = "Alice";
pub const TEST_PROFILE1_NAME: &str = "Shin Chan";
const TEST_PROFILE1_SOCIETY_MEMBER_ID: &str = "INFZ2468TN0123580";
// const TEST_PROFILE1_MEMBER_ROLE: Vec<MemberRole> = vec![
// 	MemberRole::CommunityLeader,
// ];
const TEST_PROFILE1_ASSET_ID1: &str = "INFZ2468TN0123580-AS20-0001";
pub const TEST_PROFILE2_NAME: &str = "Nene";
const TEST_PROFILE2_SOCIETY_MEMBER_ID: &str = "INFZ2468TN0123582";
// const TEST_PROFILE2_MEMBER_ROLE: Vec<MemberRole> = vec![
// 	MemberRole::AssetOwner,
// 	MemberRole::AssetKeeper,
// ];
const TEST_PROFILE2_ASSET_ID1: &str = "INFZ2468TN0123582-AS20-0001";
pub const TEST_PROFILE3_NAME: &str = "Kazama";
const TEST_PROFILE3_SOCIETY_MEMBER_ID: &str = "INFZ2468TN0123583";
// const TEST_PROFILE3_MEMBER_ROLE: Vec<MemberRole> = vec![
// 	MemberRole::HealthOfficer,
// ];
pub const TEST_PROFILE4_NAME: &str = "Masao";
const TEST_PROFILE4_SOCIETY_MEMBER_ID: &str = "INFZ2468TN0123584";
// const TEST_PROFILE4_MEMBER_ROLE: Vec<MemberRole> = vec![
// 	MemberRole::Insurer,
// ];
pub const TEST_PROFILE5_NAME: &str = "BoChan";
const TEST_PROFILE5_SOCIETY_MEMBER_ID: &str = "INFZ2468TN0123585";
// const TEST_PROFILE5_MEMBER_ROLE: Vec<MemberRole> = vec![
// 	MemberRole::AssetKeeper,
// ];

#[test]
fn asset_registration_and_approval_works() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let community_leader_id = TEST_PROFILE1_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let owner1_id = TEST_PROFILE2_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let asset1_id = TEST_PROFILE2_ASSET_ID1.as_bytes().to_owned();
		let community_leader_acc = account_key(TEST_PROFILE1_NAME);
		let asset1_owner_acc = account_key(TEST_PROFILE2_NAME);
		let now = 42;
		Timestamp::set_timestamp(now);
		// Register & update role of community leader
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				community_leader_acc.clone(),
				community_leader_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				community_leader_acc.clone(),
				Some(
					vec![
						MemberRole::CommunityLeader,
					],
				),
			),
		);
		// Register & update role of asset owner
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				owner1_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				Some(
					vec![
						MemberRole::AssetOwner,
					],
				),
			),
		);
		// asset owner request for asset registration
		assert_ok!(
			ModAssetProfile::register_asset(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				Some(
					vec![
						AssetProfileInfo::new(b"class", b"cow"),
						AssetProfileInfo::new(b"type", b"sindhi"),
						AssetProfileInfo::new(b"dob", b"2018AUG15"),
						AssetProfileInfo::new(b"prime vaccination", b"Done"),
					],
				),
			),
		);
		// check status on storage
		assert_eq!(
			ModAssetProfile::asset_register_inq(&asset1_id),
			Some(
				AssetProfile {
					asset_id: asset1_id.clone(),
					asset_info: Some(
						vec![
							AssetProfileInfo::new(b"class", b"cow"),
							AssetProfileInfo::new(b"type", b"sindhi"),
							AssetProfileInfo::new(b"dob", b"2018AUG15"),
							AssetProfileInfo::new(b"prime vaccination", b"Done"),
						],
					),
					joined_date: now,
					asset_status: AssetStatus::NewRegi,
					asset_owners: Some(vec![asset1_owner_acc.clone(),],),
					asset_keepers: Some(vec![asset1_owner_acc.clone(),],),
					asset_insurance: None,
				},
			),
		);
		// community leader approve for asset registration
		assert_ok!(
			ModAssetProfile::process_new_asset_register_request(
				Origin::signed(community_leader_acc),
				asset1_id.clone(),
				true,
			),
		);

		// check status on storage
		assert!(
			ModAssetProfile::asset_register_inq(
				&asset1_id
			).is_none(),
		);

		// check status on storage
		assert_eq!(
			ModAssetProfile::assetprofile_by_id(&asset1_id),
			Some(
				AssetProfile {
					asset_id: asset1_id.clone(),
					asset_info: Some(
						vec![
							AssetProfileInfo::new(b"class", b"cow"),
							AssetProfileInfo::new(b"type", b"sindhi"),
							AssetProfileInfo::new(b"dob", b"2018AUG15"),
							AssetProfileInfo::new(b"prime vaccination", b"Done"),
						],
					),
					joined_date: now,
					asset_status: AssetStatus::InFarm,
					asset_owners: Some(
						vec![
								asset1_owner_acc.clone(),
						],
					),
					asset_keepers: Some(
						vec![
								asset1_owner_acc.clone(),
						],
					),
					asset_insurance: None,
				},
			),
		);
		// Events check
		// Number of events expected is 10
		assert_eq!(
			System::events().len(),
			10
		);
		// Request for asset registration event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::RequestAssetRegistration(
						asset1_owner_acc,
						asset1_id.clone(),
					),
				),
			),
		);
		// process asset registration event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::ProcessRequestAssetRegistration(
						community_leader_acc,
						asset1_id.clone(),
						true,
					),
				),
			),
		);
	});
}

#[test]
fn asset_insurance_works() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let community_leader_id = TEST_PROFILE1_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let owner1_id = TEST_PROFILE2_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let asset1_id = TEST_PROFILE2_ASSET_ID1.as_bytes().to_owned();
		let community_leader_acc = account_key(TEST_PROFILE1_NAME);
		let asset1_owner_acc = account_key(TEST_PROFILE2_NAME);
		let insurer_acc = account_key(TEST_PROFILE3_NAME);
		let insurer_id = TEST_PROFILE3_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let now = 42;
		Timestamp::set_timestamp(now);
		// Register & update role of community leader
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				community_leader_acc.clone(),
				community_leader_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				community_leader_acc.clone(),
				Some(
					vec![
						MemberRole::CommunityLeader,
					],
				),
			),
		);
		// Register & update role of asset owner
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				owner1_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				Some(
					vec![
						MemberRole::AssetOwner,
					],
				),
			),
		);
		// Register & update role of insurer
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				insurer_acc.clone(),
				insurer_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"JP009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				insurer_acc.clone(),
				Some(
					vec![
						MemberRole::Insurer,
					],
				),
			),
		);
		// asset owner request for asset registration
		assert_ok!(
			ModAssetProfile::register_asset(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				Some(
					vec![
						AssetProfileInfo::new(b"class", b"cow"),
						AssetProfileInfo::new(b"type", b"sindhi"),
						AssetProfileInfo::new(b"dob", b"2018AUG15"),
						AssetProfileInfo::new(b"prime vaccination", b"Done"),
					],
				),
			),
		);
		// community leader approve for asset registration
		assert_ok!(
			ModAssetProfile::process_new_asset_register_request(
				Origin::signed(community_leader_acc),
				asset1_id.clone(),
				true,
			),
		);
		// request for asset insurance by asset owner
		assert_ok!(
			ModAssetProfile::request_insurance(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
			),
		);
		// update asset insurance premium by insurer
		assert_ok!(
			ModAssetProfile::update_insurance_premium(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
				5,
			),
		);
		// deposit asset insurance premium by asset owner
		assert_ok!(
			ModAssetProfile::deposit_insurance_premium(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				5,
			),
		);
		// update asset insurance
		assert_ok!(
			ModAssetProfile::approve_insurance(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
			),
		);
		// Events check
		// Number of events expected is 10
		assert_eq!(
			System::events().len(),
			18,
		);
		// Request for asset insurance event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::RequestAssetInsurance(
						asset1_owner_acc,
						asset1_id.clone(),
					),
				),
			),
		);
		// update insurance premium event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetInsurancePremiumQuoteUpdate(
						insurer_acc,
						asset1_id.clone(),
						5,
					),
				),
			),
		);
		// deposit insurance premium event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetInsurancePremiumDepositUpdate(
						asset1_owner_acc,
						asset1_id.clone(),
						5,
					),
				),
			),
		);
		// asset insurance approved event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetInsuranceApproved(
						insurer_acc,
						asset1_id.clone(),
					),
				),
			),
		);
	});
}

#[test]
fn asset_insurance_reclaim_works() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let community_leader_id = TEST_PROFILE1_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let owner1_id = TEST_PROFILE2_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let asset1_id = TEST_PROFILE2_ASSET_ID1.as_bytes().to_owned();
		let community_leader_acc = account_key(TEST_PROFILE1_NAME);
		let asset1_owner_acc = account_key(TEST_PROFILE2_NAME);
		let insurer_acc = account_key(TEST_PROFILE3_NAME);
		let insurer_id = TEST_PROFILE3_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let now = 42;
		Timestamp::set_timestamp(now);
		// Register & update role of community leader
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				community_leader_acc.clone(),
				community_leader_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				community_leader_acc.clone(),
				Some(
					vec![
						MemberRole::CommunityLeader,
					],
				),
			),
		);
		// Register & update role of asset owner
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				owner1_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				Some(
					vec![
						MemberRole::AssetOwner,
					],
				),
			),
		);
		// Register & update role of insurer
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				insurer_acc.clone(),
				insurer_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"JP009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				insurer_acc.clone(),
				Some(
					vec![
						MemberRole::Insurer,
					],
				),
			),
		);
		// asset owner request for asset registration
		assert_ok!(
			ModAssetProfile::register_asset(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				Some(
					vec![
						AssetProfileInfo::new(b"class", b"cow"),
						AssetProfileInfo::new(b"type", b"sindhi"),
						AssetProfileInfo::new(b"dob", b"2018AUG15"),
						AssetProfileInfo::new(b"prime vaccination", b"Done"),
					],
				),
			),
		);
		// community leader approve for asset registration
		assert_ok!(
			ModAssetProfile::process_new_asset_register_request(
				Origin::signed(community_leader_acc),
				asset1_id.clone(),
				true,
			),
		);
		// request for asset insurance by asset owner
		assert_ok!(
			ModAssetProfile::request_insurance(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
			),
		);
		// update asset insurance premium by insurer
		assert_ok!(
			ModAssetProfile::update_insurance_premium(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
				5,
			),
		);
		// deposit asset insurance premium by asset owner
		assert_ok!(
			ModAssetProfile::deposit_insurance_premium(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				5,
			),
		);
		// update asset insurance
		assert_ok!(
			ModAssetProfile::approve_insurance(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
			),
		);
		// request asset insurance reclaim
		assert_ok!(
			ModAssetProfile::request_insurance_reclaim(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				Some(
					vec![
						AssetInsuranceReClaimInfo::new(b"expired", b"due to illness bovid19"),
						AssetInsuranceReClaimInfo::new(b"illness symptoms", b"tired & week"),
						AssetInsuranceReClaimInfo::new(b"doe", b"2019JAN29"),
					],
				),
			),
		);
		// community approve asset insurance reclaim request
		assert_ok!(
			ModAssetProfile::community_approve_insurance_reclaim(
				Origin::signed(community_leader_acc),
				asset1_id.clone(),
				true,
				Some(
					vec![
						AssetInsuranceReClaimInfo::new(b"expired", b"illness mass spread"),
					],
				),
			)
		);
		// insurer approve asset insurance reclaim request
		assert_ok!(
			ModAssetProfile::insurer_approve_insurance_reclaim(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
				true,
				Some(
					vec![
						AssetInsuranceReClaimInfo::new(b"expired", b"illness mass spread"),
					],
				),
			)
		);
		// Events check
		// Number of events expected is 21
		assert_eq!(
			System::events().len(),
			21,
		);
		// Request for asset insurance reclaim event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetInsuranceReclaim(
						asset1_owner_acc,
						asset1_id.clone(),
					),
				),
			),
		);
		// asset insurance reclaim community approve event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetInsuranceReclaimCommunityApproved(
						community_leader_acc,
						asset1_id.clone(),
					),
				),
			),
		);
		// asset insurance reclaim insurer approve event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetInsuranceReclaimInsurerApproved(
						insurer_acc,
						asset1_id.clone(),
					),
				),
			),
		);
	});
}

#[test]
fn asset_healthcheck_works() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let community_leader_id = TEST_PROFILE1_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let owner1_id = TEST_PROFILE2_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let asset1_id = TEST_PROFILE2_ASSET_ID1.as_bytes().to_owned();
		let community_leader_acc = account_key(TEST_PROFILE1_NAME);
		let asset1_owner_acc = account_key(TEST_PROFILE2_NAME);
		let insurer_acc = account_key(TEST_PROFILE3_NAME);
		let insurer_id = TEST_PROFILE3_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let healthofficer_acc = account_key(TEST_PROFILE4_NAME);
		let healthofficer_id = TEST_PROFILE4_SOCIETY_MEMBER_ID.as_bytes().to_owned();
		let now = 42;
		Timestamp::set_timestamp(now);
		// Register & update role of community leader
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				community_leader_acc.clone(),
				community_leader_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				community_leader_acc.clone(),
				Some(
					vec![
						MemberRole::CommunityLeader,
					],
				),
			),
		);
		// Register & update role of asset owner
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				owner1_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				asset1_owner_acc.clone(),
				Some(
					vec![
						MemberRole::AssetOwner,
					],
				),
			),
		);
		// Register & update role of insurer
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				insurer_acc.clone(),
				insurer_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"JP009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				insurer_acc.clone(),
				Some(
					vec![
						MemberRole::Insurer,
					],
				),
			),
		);
		// Register & update role of health officer
		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				healthofficer_acc.clone(),
				healthofficer_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"IN009"),
					],
				),
				2,
			),
		);
		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				healthofficer_acc.clone(),
				Some(
					vec![
						MemberRole::HealthOfficer,
					],
				),
			),
		);
		// asset owner request for asset registration
		assert_ok!(
			ModAssetProfile::register_asset(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				Some(
					vec![
						AssetProfileInfo::new(b"class", b"cow"),
						AssetProfileInfo::new(b"type", b"sindhi"),
						AssetProfileInfo::new(b"dob", b"2018AUG15"),
						AssetProfileInfo::new(b"prime vaccination", b"Done"),
					],
				),
			),
		);
		// community leader approve for asset registration
		assert_ok!(
			ModAssetProfile::process_new_asset_register_request(
				Origin::signed(community_leader_acc),
				asset1_id.clone(),
				true,
			),
		);
		// request for asset insurance by asset owner
		assert_ok!(
			ModAssetProfile::request_insurance(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
			),
		);
		// update asset insurance premium by insurer
		assert_ok!(
			ModAssetProfile::update_insurance_premium(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
				5,
			),
		);
		// deposit asset insurance premium by asset owner
		assert_ok!(
			ModAssetProfile::deposit_insurance_premium(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				5,
			),
		);
		// update asset insurance
		assert_ok!(
			ModAssetProfile::approve_insurance(
				Origin::signed(insurer_acc),
				asset1_id.clone(),
			),
		);
		// request asset health check
		assert_ok!(
			ModAssetProfile::request_healthcheck(
				Origin::signed(asset1_owner_acc),
				asset1_id.clone(),
				Some(
					vec![
						AssetHealthCheckRecInfo::new(b"illness", b"Not having enough food & water"),
					],
				),
			),
		);
		// HealthOfficer Remark
		assert_ok!(
			ModAssetProfile::healthofficer_remark(
				Origin::signed(healthofficer_acc),
				asset1_id.clone(),
				false,
				Some(
					vec![
						AssetHealthCheckRecInfo::new(b"Cause", b"May be due to seasonal virus"),
						AssetHealthCheckRecInfo::new(b"prescription", b"Tab1 | Tab2 | Tab3"),
					],
				),
			),
		);
		// HealthOfficer followup
		assert_ok!(
			ModAssetProfile::healthofficer_remark(
				Origin::signed(healthofficer_acc),
				asset1_id.clone(),
				true,
				Some(
					vec![
						AssetHealthCheckRecInfo::new(b"Followup", b"Health Improved no more medication"),
					],
				),
			),
		);
		// Community remark
		assert_ok!(
			ModAssetProfile::community_remark(
				Origin::signed(community_leader_acc),
				asset1_id.clone(),
				true,
				Some(
					vec![
						AssetHealthCheckRecInfo::new(b"Followup", b"Looks healthy & active"),
					],
				),
			),
		);
		// Events check
		// Number of events expected is 21
		assert_eq!(
			System::events().len(),
			25,
		);
		// healthcheck remark by Asset Owner
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetHealthCheckRequest(
						asset1_owner_acc,
						asset1_id.clone(),
					),
				),
			),
		);
		// healthcheck remark by HealthOfficer
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetHealthCheckHealthOfficerUpdate(
						healthofficer_acc,
						asset1_id.clone(),
					),
				),
			),
		);
		// healthcheck remark by Community leader
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_asset_profile(
					RawEvent::AssetHealthCheckCommunityUpdate(
						community_leader_acc,
						asset1_id.clone(),
					),
				),
			),
		);
	});
}
