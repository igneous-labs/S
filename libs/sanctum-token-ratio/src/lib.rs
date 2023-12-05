mod amts_after_fee;
mod err;
mod u64_bps_fee_ceil;
mod u64_fee_ceil;
mod u64_fee_floor;
mod u64_ratio_floor;

pub use amts_after_fee::*;
pub use err::*;
pub use u64_bps_fee_ceil::*;
pub use u64_fee_ceil::*;
pub use u64_fee_floor::*;
pub use u64_ratio_floor::*;

pub const BPS_DENOMINATOR: u16 = 10_000;
