//! A module for the unified [`Identifier`] ID struct, for use as a representation
//! of multiple types of IDs in a single, packed type. Allows for describing an [`crate::entity::Entity`],
//! or other IDs that can be packed and expressed within a `u64` sized type.
//! [`Identifier`]s cannot be created directly, only able to be converted from other
//! compatible IDs.
use self::{kinds::IdKind, masks::IdentifierMask};

pub mod error;
pub(crate) mod kinds;
pub(crate) mod masks;

/// A unified identifier for all entity/component/relationship pair IDs.
/// Has the same size as a `u64` integer, but the layout is split between a 32-bit low
/// segment, a 30-bit high segment, and the significant bit reserved as type flags to denote
/// entity/pair discrimination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    lo: u32,
    hi: u32,
}

impl Identifier {
    /// Construct a new [`Identifier`]. The `high` parameter is masked with the
    /// `kind` so to pack the high value and bit flags into the same field.
    #[inline]
    #[must_use]
    pub(crate) const fn new(low: u32, high: u32, kind: IdKind) -> Self {
        // the high bits are masked to cut off the most significant bit
        // as these are used for the type flags. This means that the high
        // portion is only 31 bits, but this still provides 2^31
        // values/kinds/ids that can be stored in this segment.
        let masked_value = IdentifierMask::extract_value_from_high(high);

        Self {
            lo: low,
            hi: IdentifierMask::pack_kind_into_high(masked_value, kind),
        }
    }

    /// Returns the value of the low segment of the [`Identifier`].
    #[inline]
    pub(crate) const fn low(self) -> u32 {
        self.lo
    }

    /// Returns the value of the high segment of the [`Identifier`]. This
    /// does not apply any masking.
    #[inline]
    pub(crate) const fn high(self) -> u32 {
        self.hi
    }

    /// Convert the [`Identifier`] into a `u64`.
    #[inline]
    pub(crate) const fn to_bits(self) -> u64 {
        IdentifierMask::pack_into_u64(self.lo, self.hi)
    }

    /// Convert a `u64` into an [`Identifier`].
    #[inline]
    pub(crate) const fn from_bits(value: u64) -> Self {
        Self {
            lo: IdentifierMask::get_low(value),
            hi: IdentifierMask::get_high(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_construction() {
        let id = Identifier::new(12, 55, IdKind::Entity);

        assert_eq!(id.low(), 12);
        assert_eq!(id.high(), 55);
        assert_eq!(
            IdentifierMask::extract_kind_from_high(id.high()),
            IdKind::Entity
        );
    }

    #[test]
    fn from_bits() {
        // This high value should correspond to the max high() value
        // and also Entity flag.
        let high = 0x7FFFFFFF;
        let low = 0xC;
        let bits: u64 = high << u32::BITS | low;

        let id = Identifier::from_bits(bits);

        assert_eq!(id.to_bits(), 0x7FFFFFFF0000000C);
        assert_eq!(id.low(), low as u32);
        assert_eq!(id.high(), 0x7FFFFFFF);
        assert_eq!(
            IdentifierMask::extract_kind_from_high(id.high()),
            IdKind::Entity
        );
    }
}
