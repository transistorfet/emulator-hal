//! Traits and implementations for coordinating time between emulated components

use core::ops::Add;

/// Represents a monotonic instant in time
pub trait Instant: Add<Self::Duration> {
    /// Represents a duration that can be added to an instant of this type
    type Duration;
}

/*
// TODO this would be used if Instant was an associated type on the other traits instead of a generic

/// Represents the types common to a bus abstraction
pub trait InstantType {
    /// A measure of time at which a bus transaction can occur
    type Instant: Instant;
}

impl<T: Instant> InstantType for T {
    type Instant = T;
}
*/

#[cfg(feature = "std")]
impl Instant for std::time::Instant {
    type Duration = std::time::Duration;
}

#[cfg(feature = "fugit")]
impl<T, const NOM: u32, const DENOM: u32> Instant for fugit::Instant<T, NOM, DENOM>
where
    Self: Add<fugit::Duration<T, NOM, DENOM>>,
{
    type Duration = fugit::Duration<T, NOM, DENOM>;
}

#[cfg(feature = "femtos")]
impl Instant for femtos::Instant {
    type Duration = femtos::Duration;
}

#[cfg(test)]
mod test {}