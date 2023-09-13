// TODO: Remove with other dead code if required
#![allow(dead_code)]

const LOW_MASK: u64 = 0xFFFFFFFF;
const HIGH_MASK: u64 = 0x3FFFFFFF00000000;

#[repr(u64)]
enum FlagDiscriminators {
    EntityPair = 1 << (u64::BITS - 1),
    Deactivated = 1 << (u64::BITS - 2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub(crate) enum IdKind {
    Entity = 0,
    Pair = 1 << (u64::BITS - 1),
}

/// Internal implementation detail for a unified identifier for all entity/component/relationship
/// pair IDs. Has the same size as a `u64` integer, but the layout is split between a 32-bit low
/// segment, a 30-bit high segment, and 2 most significant bits reserved as type flags to denote
/// entity/pair discrimination and activation/deactivation bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct Identifier(u64);

impl Identifier {
    #[inline]
    #[must_use]
    pub(crate) const fn new(low: u32, high: u32, kind: IdKind) -> Self {
        // the high bits are masked to cut off the 2 most significant bits
        // as these are used for the type flags. This means that the high
        // portion is only 30 bits, but this still provides 2^30 or
        // 1,073,741,824 values/kinds/ids that can be stored in this segment.
        let masked_high = ((high as u64) << u32::BITS) & HIGH_MASK;

        Self(masked_high | (low as u64) | (kind as u64))
    }

    #[inline]
    pub(crate) const fn low(self) -> u32 {
        (self.0 & LOW_MASK) as u32
    }

    #[inline]
    pub(crate) const fn high(self) -> u32 {
        ((self.0 & HIGH_MASK) >> u32::BITS) as u32
    }

    #[inline]
    pub(crate) const fn to_bits(self) -> u64 {
        self.0
    }

    #[inline]
    pub(crate) const fn from_bits(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub(crate) const fn kind(self) -> IdKind {
        let mask = FlagDiscriminators::EntityPair as u64;
        let bit = self.0 & mask;

        // If the bit for the Entity/Pair flag is toggled, it has the
        // same representation as the mask.
        if bit == mask {
            IdKind::Pair
        } else {
            IdKind::Entity
        }
    }

    #[inline]
    pub(crate) const fn is_active(self) -> bool {
        let bit = self.0 & (FlagDiscriminators::Deactivated as u64);

        // If the bit is toggled, then the ID represents a deactivated identifier,
        // else if it is not toggled (the default), then it is active.
        bit == 0
    }

    #[inline]
    pub(crate) const fn activate(self) -> Self {
        const NEGATE_DEACTIVATE_MASK: u64 = !(1 << (u64::BITS - 2));

        Self(self.0 & NEGATE_DEACTIVATE_MASK)
    }

    #[inline]
    pub(crate) const fn deactivate(self) -> Self {
        Self(self.0 | (FlagDiscriminators::Deactivated as u64))
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
        assert_eq!(id.kind(), IdKind::Entity);
        // All IDs are active by default
        assert!(id.is_active());
    }

    #[test]
    fn from_bits() {
        // This high value should correspond to the max high() value
        // and also Entity + Deactivated flags.
        let high = 0x7FFFFFFF;
        let low = 12;
        let bits: u64 = high << u32::BITS | low;

        let id = Identifier::from_bits(bits);

        assert_eq!(id.to_bits(), 0x7FFFFFFF0000000C);
        assert_eq!(id.low(), low as u32);
        // MAX value will always return with the 2 MSB masked, so
        // equivalent to 0x3FFFFFFF, or u32::MAX >> 2
        assert_eq!(id.high(), 0x3FFFFFFF);
        assert_eq!(id.kind(), IdKind::Entity);
        assert!(!id.is_active());
    }

    #[test]
    fn id_deactivation() {
        let id = Identifier::new(12, 55, IdKind::Entity);

        let deactivated_id = id.deactivate();

        assert!(!deactivated_id.is_active());
        // The IDs should no longer match as their underlying bits are different
        assert_ne!(deactivated_id, id);

        let reactivated_id = deactivated_id.activate();

        assert!(reactivated_id.is_active());
        // The IDs should match again
        assert_eq!(reactivated_id, id);
    }
}
