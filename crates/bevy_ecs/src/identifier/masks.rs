use super::IdKind;

const LOW_MASK: u64 = 0xFFFFFFFF;
const HIGH_MASK: u32 = 0x7FFFFFFF;

#[repr(u32)]
enum FlagDiscriminators {
    EntityPair = 1 << (u32::BITS - 1),
}

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

    /// Extract the value component from a high segment of an [`super::Identifier`].
    #[inline]
    pub(crate) const fn extract_value_from_high(value: u32) -> u32 {
        value & HIGH_MASK
    }

    /// Extract the ID kind component from a high segment of an [`super::Identifier`].
    #[inline]
    pub(crate) const fn extract_kind_from_high(value: u32) -> IdKind {
        let mask = FlagDiscriminators::EntityPair as u32;
        let bit = value & mask;

        if bit == mask {
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
        let high: u32 = 0x7FFF_FFFF;

        assert_eq!(IdentifierMask::extract_kind_from_high(high), IdKind::Entity);
    }

    #[test]
    fn extract_high_value() {
        let high: u32 = 0xFFFF_FFFF;

        // Excludes the MSB as that is a flag bit.
        assert_eq!(IdentifierMask::extract_value_from_high(high), 0x7FFF_FFFF);
    }
}
