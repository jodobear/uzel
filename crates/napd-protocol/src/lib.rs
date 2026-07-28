#![forbid(unsafe_code)]
#![doc = "Validated private shell-to-daemon protocol bounds. No wire operations are implemented in Slice 01."]

pub const VERSION: u8 = 0;
pub const MAX_FRAME_BYTES: usize = 4_096;

#[cfg(test)]
mod tests {
    use super::{MAX_FRAME_BYTES, VERSION};

    #[test]
    fn gate_zero_bounds_remain_exact() {
        assert_eq!(VERSION, 0);
        assert_eq!(MAX_FRAME_BYTES, 4_096);
    }
}
