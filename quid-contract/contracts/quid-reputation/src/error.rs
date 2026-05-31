use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReputationError {
    /// No admin has been bootstrapped yet.
    AdminNotSet = 1,
    /// Caller is not the contract admin.
    NotAdmin = 2,
    /// No profile exists for the given subject address.
    ProfileNotFound = 3,
}
