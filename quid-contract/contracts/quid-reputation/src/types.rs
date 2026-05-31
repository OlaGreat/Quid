use soroban_sdk::{contracttype, Address};

/// Storage keys used by the reputation contract.
#[contracttype]
pub enum DataKey {
    /// The contract administrator address.
    Admin,
    /// Per-subject reputation profile.
    Profile(Address),
}

/// On-chain reputation profile for a single subject address.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    /// The address this profile belongs to.
    pub subject: Address,
    /// Cumulative reputation score (starts at 0).
    pub score: i64,
    /// Total number of completed missions.
    pub missions_completed: u32,
    /// Total number of missions created (for creators).
    pub missions_created: u32,
}
