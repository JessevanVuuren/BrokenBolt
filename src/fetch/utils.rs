use core::fmt;

pub enum BalanceType {
    Rebased,
    Base,
}

impl fmt::Display for BalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BalanceType::Base => write!(f, "base"),
            BalanceType::Rebased => write!(f, "rebased"),
        }
    }
}
