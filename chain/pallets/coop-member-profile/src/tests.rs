// Unit Tests for pallet-coop-member-profile

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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
pub const TEST_PROFILE2_NAME: &str = "Nene";
const TEST_PROFILE2_SOCIETY_MEMBER_ID: &str = "INFZ2468TN0123582";
// const TEST_PROFILE2_MEMBER_ROLE: Vec<MemberRole> = vec![
// 	MemberRole::AssetOwner,
// 	MemberRole::AssetKeeper,
// ];
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
fn register_member_without_profile_info() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let society_membership_id =
			TEST_PROFILE1_SOCIETY_MEMBER_ID
			.as_bytes()
			.to_owned();
		let member_acc = account_key(TEST_PROFILE1_NAME);
		let now = 42;
		Timestamp::set_timestamp(now);

		assert_eq!(
			Balances::free_balance(
				&ModMemberProfile::coop_account_id(
					<Test as Trait>::CoopSocietyId::get(),
				)
			),
			0,
		);

		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				member_acc.clone(),
				society_membership_id.clone(),
				None,
				2,
			)
		);

		assert_eq!(
			<MemberId<Test>>::get(member_acc),
			Some(
				society_membership_id.clone(),
			)
		);

		assert_eq!(
			ModMemberProfile::memberprofile_by_id(&society_membership_id),
			Some(
				MemberProfile {
					society_membership_id: society_membership_id.clone(),
					member_acc: member_acc,
					joined_date: now,
					prof_info: None,
					member_status: None,
					role: None,
					karma: 0u32,
					deposit: 2,
				}
			)
		);

		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_member_profile(
					RawEvent::MemberProfileRegistered(
						sender,
						society_membership_id.clone(),
						member_acc
					)
				)
			)
		);

		assert_eq!(
			Balances::free_balance(
				&ModMemberProfile::coop_account_id(
					<Test as Trait>::CoopSocietyId::get(),
				)
			),
			2,
		);
	});
}

#[test]
fn register_member_with_profile_info() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let society_membership_id =
			TEST_PROFILE1_SOCIETY_MEMBER_ID
			.as_bytes()
			.to_owned();
		let member_acc = account_key(TEST_PROFILE1_NAME);
		let now = 42;
		Timestamp::set_timestamp(now);

		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				member_acc.clone(),
				society_membership_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					]
				),
				2,
			),
		);

		assert_eq!(
			<MemberId<Test>>::get(member_acc),
			Some(
				society_membership_id.clone(),
			)
		);

		assert_eq!(
			ModMemberProfile::memberprofile_by_id(&society_membership_id),
			Some(
				MemberProfile {
					society_membership_id: society_membership_id.clone(),
					member_acc: member_acc,
					joined_date: now,
					prof_info: Some(
						vec![
							MemberProfileInfo::new(b"country", b"UK"),
							MemberProfileInfo::new(b"kyc", b"UK007"),
						],
					),
					member_status: None,
					role: None,
					karma: 0u32,
					deposit: 2,
				},
			),
		);

		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_member_profile(
					RawEvent::MemberProfileRegistered(
						sender,
						society_membership_id.clone(),
						member_acc,
					)
				)
			)
		);
	});
}

#[test]
fn register_member_with_invalid_membership_id() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let society_membership_id = TEST_NULL_STR
			.as_bytes()
			.to_owned();
		let member_acc = account_key(TEST_PROFILE1_NAME);
		let now = 42;
		Timestamp::set_timestamp(now);

		assert_noop!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				member_acc.clone(),
				society_membership_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
				2,
			),
			Error::<Test>::SocietyMemberIdEmpty,
		);

		let society_membership_id = TEST_LONG_STR
			.as_bytes()
			.to_owned();
		assert_noop!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				member_acc.clone(),
				society_membership_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
				2,
			),
			Error::<Test>::SocietyMemberIdTooLong
		);
	});
}

#[test]
fn register_member_and_add_profile_info() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let society_membership_id =
			TEST_PROFILE1_SOCIETY_MEMBER_ID
			.as_bytes()
			.to_owned();
		let member_acc = account_key(TEST_PROFILE1_NAME);
		let now = 42;
		Timestamp::set_timestamp(now);

		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				member_acc.clone(),
				society_membership_id.clone(),
				None,
				2,
			)
		);

		assert_ok!(
			ModMemberProfile::update_member_profile_info(
				Origin::signed(sender),
				member_acc.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					],
				),
			)
		);

		assert_eq!(
			ModMemberProfile::memberprofile_by_id(&society_membership_id),
			Some(
				MemberProfile {
					society_membership_id: society_membership_id.clone(),
					member_acc: member_acc,
					joined_date: now,
					prof_info: Some(
						vec![
							MemberProfileInfo::new(b"country", b"UK"),
							MemberProfileInfo::new(b"kyc", b"UK007"),
						]
					),
					member_status: None,
					role: None,
					karma: 0u32,
					deposit: 2,
				}
			)
		);

		assert_noop!(
			ModMemberProfile::update_member_profile_info(
				Origin::signed(sender),
				member_acc.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
						MemberProfileInfo::new(b"email", b"ukbond@kmail.com"),
						MemberProfileInfo::new(b"mob", b"8907804500"),
						MemberProfileInfo::new(b"yob", b"1960"),
						MemberProfileInfo::new(b"married", b"y"),
					]
				),
			),
			Error::<Test>::MemberProfileTooManyInfos,
		);

		assert_ok!(
			ModMemberProfile::update_member_profile_info(
				Origin::signed(sender),
				member_acc.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
						MemberProfileInfo::new(b"email", b"ukbond@kmail.com"),
						MemberProfileInfo::new(b"mob", b"8907804500"),
						MemberProfileInfo::new(b"yob", b"1960"),
					]
				),
			),
		);

		assert_eq!(
			ModMemberProfile::memberprofile_by_id(&society_membership_id),
			Some(
				MemberProfile {
					society_membership_id: society_membership_id.clone(),
					member_acc: member_acc,
					joined_date: now,
					prof_info: Some(
						vec![
							MemberProfileInfo::new(b"country", b"UK"),
							MemberProfileInfo::new(b"kyc", b"UK007"),
							MemberProfileInfo::new(b"email", b"ukbond@kmail.com"),
							MemberProfileInfo::new(b"mob", b"8907804500"),
							MemberProfileInfo::new(b"yob", b"1960"),
						]
					),
					member_status: None,
					role: None,
					karma: 0u32,
					deposit: 2,
				}
			)
		);

		// Number of events expected is 6
		assert_eq!(System::events().len(), 6);
		// Member registerd event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_member_profile(
					RawEvent::MemberProfileRegistered(
						sender,
						society_membership_id.clone(),
						member_acc
					)
				)
			)
		);
		// Member profile update event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_member_profile(
					RawEvent::MemberProfileInfoUpdated(
						sender,
						member_acc
					)
				)
			)
		);
	});
}

#[test]
fn register_member_add_role() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_COMMUNITY_HEAD);
		let society_membership_id =
			TEST_PROFILE1_SOCIETY_MEMBER_ID
			.as_bytes()
			.to_owned();
		let member_acc = account_key(TEST_PROFILE1_NAME);
		let now = 42;
		Timestamp::set_timestamp(now);

		assert_ok!(
			ModMemberProfile::register_member(
				Origin::signed(sender),
				member_acc.clone(),
				society_membership_id.clone(),
				Some(
					vec![
						MemberProfileInfo::new(b"country", b"UK"),
						MemberProfileInfo::new(b"kyc", b"UK007"),
					]
				),
				2,
			)
		);

		assert_ok!(
			ModMemberProfile::update_member_role(
				Origin::signed(sender),
				member_acc.clone(),
				Some(
					vec![
						MemberRole::CommunityLeader,
					]
				),
			)
		);

		assert_eq!(
			ModMemberProfile::memberprofile_by_id(&society_membership_id),
			Some(
				MemberProfile {
					society_membership_id: society_membership_id.clone(),
					member_acc: member_acc,
					joined_date: now,
					prof_info: Some(
						vec![
							MemberProfileInfo::new(b"country", b"UK"),
							MemberProfileInfo::new(b"kyc", b"UK007"),
						]
					),
					member_status: Some(
						MemberStatus::Active
					),
					role: Some(
						vec![
							MemberRole::CommunityLeader,
						]
					),
					karma: 0u32,
					deposit: 2,
				}
			)
		);

		let is_valid_role =
			ModMemberProfile::is_valid_member_role(
				&member_acc,
				MemberRole::CommunityHead,
			);

		// TODO :: commented due to compiler error
		// Hve to recheck
		// assert_eq!(
		//     is_valid_role.unwrap_err(),
		// 	Error::<Test>::MemberRoleInvalid
		// );

		println!(
			"is_valid_role - rval[{:#?}]",
			is_valid_role.unwrap_err(),
		);

		assert_ok!(
			ModMemberProfile::is_valid_member_role(
				&member_acc,
				MemberRole::CommunityLeader,
			)
		);

		// Number of events expected is 5
		assert_eq!(System::events().len(), 5);
		// Member registerd event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_member_profile(
					RawEvent::MemberProfileRegistered(
						sender,
						society_membership_id.clone(),
						member_acc
					)
				)
			)
		);
		// Member profile update event
		assert!(
			System::events().iter().any(
				|er|
				er.event == TestEvent::coop_member_profile(
					RawEvent::MemberProfileRoleUpdated(
						sender,
						member_acc
					)
				)
			)
		);
	});
}
