use super::kinds::IdKind;

const LOW_MASK: u64 = 0xFFFFFFFF;
const HIGH_MASK: u32 = 0x7FFFFFFF;

/// Abstraction over masks needed to extract values/components of an [`super::Identifier`].
pub(crate) struct IdentifierMask;

impl IdentifierMask {
    /// Returns the low component from a `u64` value
    #[inline]
    pub(crate) const fn get_low(value: u64) -> u32 {
        (value & LOW_MASK) as u32
    }

    /// Returns the high component from a `u64` value
    #[inline]
    pub(crate) const fn get_high(value: u64) -> u32 {
        ((value & !LOW_MASK) >> u32::BITS) as u32
    }

    /// Pack a low and high `u32` values into a single `u64` value.
    #[inline]
    pub(crate) const fn pack_into_u64(low: u32, high: u32) -> u64 {
        ((high as u64) << u32::BITS) | (low as u64)
    }

    /// Pack the [`IdKind`] bits into a high segment.
    #[inline]
    pub(crate) const fn pack_kind_into_high(value: u32, kind: IdKind) -> u32 {
        value | ((kind as u32) << 24)
    }

    /// Extract the value component from a high segment of an [`super::Identifier`].
    #[inline]
    pub(crate) const fn extract_value_from_high(value: u32) -> u32 {
        value & HIGH_MASK
    }

    /// Extract the ID kind component from a high segment of an [`super::Identifier`].
    #[inline]
    pub(crate) const fn extract_kind_from_high(value: u32) -> IdKind {
        // The negated HIGH_MASK will extract just the bit we need for kind.
        let kind_mask = !HIGH_MASK;
        let bit = value & kind_mask;

        if bit == kind_mask {
            IdKind::Pair
        } else {
            IdKind::Entity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_u64_parts() {
        let value: u64 = 0x7FFF_FFFF_0000_000C;

        assert_eq!(IdentifierMask::get_low(value), 0x0000_000C);
        assert_eq!(IdentifierMask::get_high(value), 0x7FFF_FFFF);
    }

    #[test]
    fn extract_kind() {
        let high: u32 = 0xFFFF_FFFF;

        assert_eq!(IdentifierMask::extract_kind_from_high(high), IdKind::Pair);
    }

    #[test]
    fn extract_high_value() {
        let high: u32 = 0xFFFF_FFFF;

        // Excludes the MSB as that is a flag bit.
        assert_eq!(IdentifierMask::extract_value_from_high(high), 0x7FFF_FFFF);
    }

    #[test]
    fn pack_kind_bits() {
        let high: u32 = 0x7FFF_FFFF;

        assert_eq!(
            IdentifierMask::pack_kind_into_high(high, IdKind::Pair),
            0xFFFF_FFFF
        );

        let high: u32 = 0x00FF_FFFF;

        assert_eq!(
            IdentifierMask::pack_kind_into_high(high, IdKind::Entity),
            0x00FF_FFFF
        );
    }

    #[test]
    fn pack_into_u64() {
        let high: u32 = 0x7FFF_FFFF;
        let low: u32 = 0x0000_00CC;

        assert_eq!(
            IdentifierMask::pack_into_u64(low, high),
            0x7FFF_FFFF_0000_00CC
        );
    }
}
