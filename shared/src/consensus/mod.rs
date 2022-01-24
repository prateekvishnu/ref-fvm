use num_derive::FromPrimitive;

use super::{Address, ChainEpoch};

/// Result of checking two headers for a consensus fault.
#[derive(Clone, Debug)]
pub struct ConsensusFault {
    /// Address of the miner at fault (always an ID address).
    pub target: Address,
    /// Epoch of the fault, which is the higher epoch of the two blocks causing it.
    pub epoch: ChainEpoch,
    /// Type of fault.
    pub fault_type: ConsensusFaultType,
}

/// Consensus fault types in VM.
#[derive(FromPrimitive, Clone, Copy, Debug)]
#[repr(u8)]
pub enum ConsensusFaultType {
    DoubleForkMining = 1,
    ParentGrinding = 2,
    TimeOffsetMining = 3,
}

impl TryFrom<u8> for ConsensusFaultType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::DoubleForkMining),
            2 => Ok(Self::ParentGrinding),
            3 => Ok(Self::TimeOffsetMining),
            _ => Err(()),
        }
    }
}
