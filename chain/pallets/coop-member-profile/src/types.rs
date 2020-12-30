use codec::{Decode, Encode};
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*};

// Custom types
/// An index of a society. Just a `u32`.
pub type CoopSocietyIndex = u32;
pub type Identifier = Vec<u8>;
pub type SocietyMemberShipId = Identifier;
pub type ProfileInfoFieldName = Identifier;
pub type ProfileInfoFieldValue = Identifier;
// Note: these could also be passed as trait config parameters
pub const SOCIETY_MEMBERSHIP_ID_MAX_LENGTH: usize = 36;
pub const MEMBER_PROFILE_INFO_NAME_MAX_LENGTH: usize = 10;
pub const MEMBER_PROFILE_INFO_VALUE_MAX_LENGTH: usize = 20;
pub const MEMBER_PROFILE_MAX_INFO: usize = 5;

// Contains profile info of a member as name-value pair e.g. country: UK,
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct MemberProfileInfo {
    // Name of the profile info field e.g. kyc, country etc
    info_name: ProfileInfoFieldName,
    // Name of the profile info value field e.g. IN002468009VF, UK, INDIA etc
    info_value: ProfileInfoFieldValue,
}

impl MemberProfileInfo {
    pub fn new(info_name: &[u8], info_value: &[u8]) -> Self {
        Self {
            info_name: info_name.to_vec(),
            info_value: info_value.to_vec(),
        }
    }

    pub fn info_name(&self) -> &[u8] {
        self.info_name.as_ref()
    }

    pub fn info_value(&self) -> &[u8] {
        self.info_value.as_ref()
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MemberStatus {
    Active,
    Suspended,
    Terminated
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MemberRole {
    CommunityHead,
    CommunityLeader,
    AssetOwner,
    AssetKeeper,
    Insurer,
    HealthOfficer,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct MemberProfile<AccountId, Balance, Moment> {
    pub member_acc: AccountId,
    pub society_membership_id: SocietyMemberShipId,
    pub prof_info: Option<Vec<MemberProfileInfo>>,
    pub joined_date: Moment,
    pub member_status: Option<MemberStatus>,
    pub role: Option<Vec<MemberRole>>,
	pub karma: u32,
	/// The deposit amount for membership.
	pub deposit: Balance,
}

#[derive(Default)]
pub struct MemberProfileBuilder<AccountId, Balance, Moment>
where
	AccountId: Default,
	Moment: Default,
{
	member_acc: AccountId,
	society_membership_id: SocietyMemberShipId,
	prof_info: Option<Vec<MemberProfileInfo>>,
	joined_date: Moment,
	/// The deposit amount for membership.
	pub deposit: Balance,
}

impl<AccountId, Balance, Moment> MemberProfileBuilder<AccountId, Balance, Moment>
where
	AccountId: Default,
	Balance: Default,
	Moment: Default,
{
	pub fn update_member_acc(
		mut self,
		member_acc: AccountId
	) -> Self {
		self.member_acc = member_acc;
		self
	}

	pub fn update_society_membership_id(
		mut self,
		society_membership_id: SocietyMemberShipId
	) -> Self {
		self.society_membership_id = society_membership_id;
		self
	}

	pub fn update_member_profile_info(
		mut self,
		prof_info: Option<Vec<MemberProfileInfo>>
	) -> Self {
		self.prof_info = prof_info;
		self
	}

	pub fn update_joined_date(
		mut self,
		joined_date: Moment
	) -> Self {
		self.joined_date = joined_date;
		self
	}

	pub fn update_deposit(
		mut self,
		deposit: Balance
	) -> Self {
		self.deposit = deposit;
		self
	}

    pub fn build(self) -> MemberProfile<AccountId, Balance, Moment> {
        MemberProfile::<AccountId, Balance, Moment> {
			member_acc: self.member_acc,
			society_membership_id: self.society_membership_id,
			prof_info: self.prof_info,
			joined_date: self.joined_date,
			member_status: None,
			role: None,
			karma: 0,
			deposit: self.deposit,
        }
    }
}
