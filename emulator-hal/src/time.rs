//! Traits and implementations for coordinating time between emulated components

use core::fmt::Debug;
use core::ops::{Add, Mul};
use core::time::Duration;

/// Represents a monotonic instant in time
pub trait Instant: Add<Self::Duration, Output = Self> + Eq + Ord + Debug + Copy {
    /// The start of the epoch according to this time representation
    const START: Self;

    /// Represents a duration that can be added to an instant of this type
    type Duration: Mul<u32, Output = Self::Duration> + Debug;

    /// Returns the duration of one period of the given frequency is hertz
    fn hertz_to_duration(hertz: u64) -> Self::Duration;
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

impl Instant for Duration {
    const START: Self = Duration::from_nanos(0);

    type Duration = Duration;

    fn hertz_to_duration(hertz: u64) -> Self::Duration {
        Duration::from_nanos(1_000_000_000 / hertz)
    }
}

#[cfg(feature = "fugit")]
impl<const NOM: u32, const DENOM: u32> Instant for fugit::Instant<u32, NOM, DENOM>
where
    Self: Add<fugit::Duration<u32, NOM, DENOM>, Output = Self>,
{
    const START: Self = fugit::Instant::<u32, NOM, DENOM>::from_ticks(0);

    type Duration = fugit::Duration<u32, NOM, DENOM>;

    fn hertz_to_duration(hertz: u64) -> Self::Duration {
        fugit::Duration::<u32, NOM, DENOM>::from_ticks(DENOM / hertz as u32)
    }
}

#[cfg(feature = "fugit")]
impl<const NOM: u32, const DENOM: u32> Instant for fugit::Instant<u64, NOM, DENOM>
where
    Self: Add<fugit::Duration<u64, NOM, DENOM>, Output = Self>,
{
    const START: Self = fugit::Instant::<u64, NOM, DENOM>::from_ticks(0);

    type Duration = fugit::Duration<u64, NOM, DENOM>;

    fn hertz_to_duration(hertz: u64) -> Self::Duration {
        fugit::Duration::<u64, NOM, DENOM>::from_ticks(DENOM as u64 / hertz)
    }
}

#[cfg(feature = "femtos")]
impl Instant for femtos::Instant {
    const START: Self = femtos::Instant::START;

    type Duration = femtos::Duration;

    fn hertz_to_duration(hertz: u64) -> Self::Duration {
        femtos::Duration::from_femtos(1_000_000_000_000_000 / hertz as femtos::Femtos)
    }
}

#[cfg(test)]
mod test {}
