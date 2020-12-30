use codec::{Decode, Encode};
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*};
// Custom types
pub type Identifier = Vec<u8>;
pub type AssetId = Identifier;
pub type InfoFieldName = Identifier;
pub type InfoFieldValue = Identifier;
// Note: these could also be passed as trait config parameters
pub const ASSET_ID_MAX_LENGTH: usize = 36;
pub const ASSET_INFO_NAME_MAX_LENGTH: usize = 20;
pub const ASSET_INFO_VALUE_MAX_LENGTH: usize = 40;
pub const ASSET_PROFILE_MAX_INFO: usize = 10;
pub const ASSET_INSURANCE_RECLAIM_MAX_INFO: usize = 10;
pub const ASSET_HEALTHCHECK_MAX_INFO: usize = 10;

// Contains profile info of a member as name-value pair e.g. country: UK,
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetProfileInfo {
	// Name of the profile info field e.g. class, breed, dob etc
	info_name: InfoFieldName,
	// Name of the profile info value field e.g. cow, redsindi, 2018AUG15 etc
	info_value: InfoFieldValue,
}

impl AssetProfileInfo {
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
pub enum AssetStatus {
	NewRegi,
	InFarm,
	ForSale,
	InTransfer,
	Expired,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetProfile<AccountId, Balance, Moment> {
	pub asset_id: AssetId,
	pub asset_info: Option<Vec<AssetProfileInfo>>,
	pub joined_date: Moment,
	pub asset_status: AssetStatus,
	pub asset_owners: Option<Vec<AccountId>>,
	pub asset_keepers: Option<Vec<AccountId>>,
	pub asset_insurance: Option<AssetInsurance<Balance, Moment>>,
	// pub health_rec: Option<Vec<AccountId>>,
	// pub audit_rec: Option<Vec<AccountId>>,
}

#[derive(Default)]
pub struct AssetProfileBuilder<AccountId, Balance, Moment>
where
	AccountId: Default,
	Balance: Default,
	Moment: Default,
{
	asset_id: AssetId,
	asset_info: Option<Vec<AssetProfileInfo>>,
	joined_date: Moment,
	asset_owners: Option<Vec<AccountId>>,
	asset_keepers: Option<Vec<AccountId>>,
	asset_insurance: Option<AssetInsurance<Balance, Moment>>,
}

impl<AccountId, Balance, Moment> AssetProfileBuilder<AccountId, Balance, Moment>
where
	AccountId: Default,
	Balance: Default,
	Moment: Default,
{
	pub fn update_asset_id(mut self, asset_id: AssetId) -> Self {
		self.asset_id = asset_id;
		self
	}

	pub fn update_asset_profile_info(mut self, asset_info: Option<Vec<AssetProfileInfo>>) -> Self {
		self.asset_info = asset_info;
		self
	}

	pub fn update_joined_date(mut self, joined_date: Moment) -> Self {
		self.joined_date = joined_date;
		self
	}

	pub fn update_owner(mut self, owner: AccountId) -> Self {
		self.asset_owners = Some(vec![owner]);
		self
	}

	pub fn update_keeper(mut self, owner: AccountId) -> Self {
		self.asset_keepers = Some(vec![owner]);
		self
	}

	pub fn build(self) -> AssetProfile<AccountId, Balance, Moment> {
		AssetProfile::<AccountId, Balance, Moment> {
			asset_id: self.asset_id,
			asset_info: self.asset_info,
			joined_date: self.joined_date,
			asset_status: AssetStatus::NewRegi,
			asset_owners: self.asset_owners,
			asset_keepers: self.asset_keepers,
			asset_insurance: self.asset_insurance,
		}
	}
}

// ======================== Insurance Related ==================================

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum AssetInsuranceStatus {
	NewApplication,
	PremiumQuoted,
	PremiumPaid,
	Active,
	ReClaimInProgress,
	ReClaimDone,
	Expired,
}
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetInsurance<Balance, Moment> {
	pub asset_id: AssetId,
	pub status: AssetInsuranceStatus,
	pub start_date: Moment,
	pub expiry_date: Moment,
	pub quoted_premium: Balance,
	pub premium_paid_date: Moment,
	pub coverage: Balance,
	// pub reclaim_info: Option<Vec<AccountId>>,
	// pub reclaim_vouch: Option<Vec<AccountId>>,
}
#[derive(Default)]
pub struct AssetInsuranceBuilder<Balance, Moment>
where
	Balance: Default,
	Moment: Default,
{
	pub asset_id: AssetId,
	pub start_date: Moment,
	pub expiry_date: Moment,
	pub quoted_premium: Balance,
	pub premium_paid_date: Moment,
	pub coverage: Balance,
}
impl<Balance, Moment> AssetInsuranceBuilder<Balance, Moment>
where
	Balance: Default,
	Moment: Default,
{
	pub fn update_asset_id(
		mut self,
		asset_id: AssetId
	) -> Self {
		self.asset_id = asset_id;
		self
	}

	pub fn build(self) -> AssetInsurance<Balance, Moment> {
		AssetInsurance::<Balance, Moment> {
			asset_id: self.asset_id,
			status: AssetInsuranceStatus::NewApplication,
			start_date: self.start_date,
			expiry_date: self.expiry_date,
			quoted_premium: self.quoted_premium,
			premium_paid_date: self.premium_paid_date,
			coverage: self.coverage,
		}
	}
}

// ======================== Insurance Re-Claim Related ==================================

// Contains insurance reclaim info for a asset as name-value pair e.g. reasone: dead due to disease,
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetInsuranceReClaimInfo {
	// Name of the reclaim info field e.g. reason, doe, symptoms etc
	info_name: InfoFieldName,
	// Name of the reclaim info value field e.g. dead due to illness, dead due to accident etc
	info_value: InfoFieldValue,
}
impl AssetInsuranceReClaimInfo {
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
pub enum AssetInsuranceReClaimStatus {
	NewReclaim,
	CommunityApproved,
	CommunityDisApproved,
	InsurerApproved,
	InsurerDisApproved,
	CoveragePaid,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetInsuranceReClaim<Moment> {
	pub asset_id: AssetId,
	pub applied_date: Moment,
	pub closed_date: Moment,
	pub status: AssetInsuranceReClaimStatus,
	pub owner_note: Option<Vec<AssetInsuranceReClaimInfo>>,
	pub community_note: Option<Vec<AssetInsuranceReClaimInfo>>,
	pub insurer_note: Option<Vec<AssetInsuranceReClaimInfo>>,
}

#[derive(Default)]
pub struct AssetInsuranceReClaimBuilder<Moment>
where
	Moment: Default,
{
	asset_id: AssetId,
	applied_date: Moment,
	closed_date: Moment,
	owner_note: Option<Vec<AssetInsuranceReClaimInfo>>,
}

impl<Moment> AssetInsuranceReClaimBuilder<Moment>
where
	Moment: Default,
{
	pub fn update_asset_id(
		mut self,
		asset_id: AssetId,
	) -> Self {
		self.asset_id = asset_id;
		self
	}

	pub fn update_owner_note(
		mut self,
		owner_note: Option<Vec<AssetInsuranceReClaimInfo>>,
	) -> Self {
		self.owner_note = owner_note;
		self
	}

	pub fn build(self) -> AssetInsuranceReClaim<Moment> {
		AssetInsuranceReClaim::<Moment> {
			asset_id: self.asset_id,
			status: AssetInsuranceReClaimStatus::NewReclaim,
			applied_date: self.applied_date,
			closed_date: self.closed_date,
			owner_note: self.owner_note,
			community_note: None,
			insurer_note: None,
		}
	}
}

// ======================== Health Check Record Related ==================================

// Contains Health check info for a asset as name-value pair e.g. illness: not drinking water,
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetHealthCheckRecInfo {
	// Name of the Health check info field e.g. illness
	info_name: InfoFieldName,
	// Name of the Health check info field e.g. not feeding well
	info_value: InfoFieldValue,
}
impl AssetHealthCheckRecInfo {
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
pub enum AssetHealthCheckRecStatus {
	NewReq,
	TreatmentInProgress,
	TreatmentDone,
	CommunityApproved,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AssetHealthCheckRec<Moment> {
	pub asset_id: AssetId,
	pub request_date: Moment,
	pub closed_date: Moment,
	pub status: AssetHealthCheckRecStatus,
	pub owner_note: Option<Vec<AssetHealthCheckRecInfo>>,
	pub health_officer_note: Option<Vec<AssetHealthCheckRecInfo>>,
	pub community_note: Option<Vec<AssetHealthCheckRecInfo>>,
}

#[derive(Default)]
pub struct AssetHealthCheckRecBuilder<Moment>
where
	Moment: Default,
{
	asset_id: AssetId,
	request_date: Moment,
	closed_date: Moment,
	owner_note: Option<Vec<AssetHealthCheckRecInfo>>,
}

impl<Moment> AssetHealthCheckRecBuilder<Moment>
where
	Moment: Default,
{
	pub fn update_asset_id(
		mut self,
		asset_id: AssetId,
	) -> Self {
		self.asset_id = asset_id;
		self
	}

	pub fn update_owner_note(
		mut self,
		owner_note: Option<Vec<AssetHealthCheckRecInfo>>,
	) -> Self {
		self.owner_note = owner_note;
		self
	}

	pub fn build(self) -> AssetHealthCheckRec<Moment> {
		AssetHealthCheckRec::<Moment> {
			asset_id: self.asset_id,
			status: AssetHealthCheckRecStatus::NewReq,
			request_date: self.request_date,
			closed_date: self.closed_date,
			owner_note: self.owner_note,
			community_note: None,
			health_officer_note: None,
		}
	}
}
