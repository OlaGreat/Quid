use soroban_sdk::{contracttype, Address, String};

#[derive(Clone, Debug, Default, PartialEq, Eq, Copy)]
#[contracttype]
pub enum ProgramStatus {
    #[default]
    Draft,
    Active,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub id: u64,
    pub sponsor: Address,
    pub recipient: Address,
    pub reviewer: Option<Address>,
    pub token: Address,
    pub total_amount: i128,
    pub allocated_amount: i128,
    pub released_amount: i128,
    pub milestone_count: u64,
    pub metadata_cid: Option<String>,
    pub created_at: u64,
    pub status: ProgramStatus,
}
