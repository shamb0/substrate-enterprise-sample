// Creating mock runtime here
use crate as coop_asset_profile;
use crate::tests::*;
use crate::{Module, Trait};
use core::marker::PhantomData;
use frame_support::{
    impl_outer_event, impl_outer_origin, parameter_types,
    traits::{Contains, ContainsLengthBound, EnsureOrigin},
    weights::Weight,
};
use frame_system as system;
use frame_system::RawOrigin;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    ModuleId, Perbill, Percent, Permill,
};
use std::cell::RefCell;

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

impl_outer_event! {
    pub enum TestEvent for Test {
        system<T>,
        pallet_balances<T>,
        pallet_treasury<T>,
        pallet_coop_member_profile<T>,
        coop_asset_profile<T>,
    }
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl frame_system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = TestEvent;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type AvailableBlockRatio = AvailableBlockRatio;
    type MaximumBlockLength = MaximumBlockLength;
    type Version = ();
    type PalletInfo = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}
impl timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Trait for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = TestEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

thread_local! {
    static TEN_TO_FOURTEEN: RefCell<Vec<sr25519::Public>> = RefCell::new(
        vec![
                sp_core::sr25519::Pair::generate().0.public(),
                sp_core::sr25519::Pair::generate().0.public(),
                sp_core::sr25519::Pair::generate().0.public(),
                sp_core::sr25519::Pair::generate().0.public(),
                sp_core::sr25519::Pair::generate().0.public(),
            ]
        );
}
pub struct TenToFourteen;
impl Contains<sr25519::Public> for TenToFourteen {
    fn sorted_members() -> Vec<sr25519::Public> {
        TEN_TO_FOURTEEN.with(|v| v.borrow().clone())
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn add(new: &sr25519::Public) {
        TEN_TO_FOURTEEN.with(|v| {
            let mut members = v.borrow_mut();
            members.push(*new);
            members.sort();
        })
    }
}
impl ContainsLengthBound for TenToFourteen {
    fn max_len() -> usize {
        TEN_TO_FOURTEEN.with(|v| v.borrow().len())
    }
    fn min_len() -> usize {
        0
    }
}
parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: u64 = 1;
    pub const SpendPeriod: u64 = 2;
    pub const Burn: Permill = Permill::from_percent(50);
    pub const TipCountdown: u64 = 1;
    pub const TipFindersFee: Percent = Percent::from_percent(20);
    pub const TipReportDepositBase: u64 = 1;
    pub const DataDepositPerByte: u64 = 1;
    pub const BountyDepositBase: u64 = 80;
    pub const BountyDepositPayoutDelay: u64 = 3;
    pub const TreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
    pub const BountyUpdatePeriod: u32 = 20;
    pub const MaximumReasonLength: u32 = 16384;
    pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
    pub const BountyValueMinimum: u64 = 1;
}
impl pallet_treasury::Trait for Test {
    type ModuleId = TreasuryModuleId;
    type Currency = pallet_balances::Module<Test>;
    type ApproveOrigin = frame_system::EnsureRoot<sr25519::Public>;
    type RejectOrigin = frame_system::EnsureRoot<sr25519::Public>;
    type Tippers = TenToFourteen;
    type TipCountdown = TipCountdown;
    type TipFindersFee = TipFindersFee;
    type TipReportDepositBase = TipReportDepositBase;
    type DataDepositPerByte = DataDepositPerByte;
    type Event = TestEvent;
    type OnSlash = ();
    type ProposalBond = ProposalBond;
    type ProposalBondMinimum = ProposalBondMinimum;
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type BountyDepositBase = BountyDepositBase;
    type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
    type BountyUpdatePeriod = BountyUpdatePeriod;
    type BountyCuratorDeposit = BountyCuratorDeposit;
    type BountyValueMinimum = BountyValueMinimum;
    type MaximumReasonLength = MaximumReasonLength;
    type BurnDestination = (); // Just gets burned.
    type WeightInfo = ();
}
parameter_types! {
    pub const MemberDepositValueMinimum: u64 = 1;
    pub const CoopSocietyId: u32 = 369;
}
impl pallet_coop_member_profile::Trait for Test {
    type MemberDepositValueMinimum = MemberDepositValueMinimum;
    type CoopSocietyId = CoopSocietyId;
    type Event = TestEvent;
    type CreateRoleOrigin = MockOrigin<Test>;
}
impl Trait for Test {
    type Event = TestEvent;
}
pub type System = system::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
// pub type Treasury = pallet_treasury::Module<Test>;
pub type ModMemberProfile = pallet_coop_member_profile::Module<Test>;
pub type ModAssetProfile = Module<Test>;

pub struct MockOrigin<T>(PhantomData<T>);

impl<T: Trait> EnsureOrigin<T::Origin> for MockOrigin<T> {
    type Success = T::AccountId;
    fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
        o.into().and_then(|o| match o {
            RawOrigin::Signed(ref who) => Ok(who.clone()),
            r => Err(T::Origin::from(r)),
        })
    }
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        // Total issuance will be 200 with treasury account initialized at ED.
        balances: vec![
            (account_key(TEST_COMMUNITY_HEAD), 100),
            (account_key(TEST_PROFILE1_NAME), 50),
            (account_key(TEST_PROFILE2_NAME), 50),
            (account_key(TEST_PROFILE3_NAME), 50),
            (account_key(TEST_PROFILE4_NAME), 50),
            (account_key(TEST_PROFILE5_NAME), 50),
        ],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext = sp_io::TestExternalities::from(storage);
    // Events are not emitted on block 0 -> advance to block 1.
    // Any dispatchable calls made during genesis block will have no events emitted.
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn account_key(s: &str) -> sr25519::Public {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}
