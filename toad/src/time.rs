use crate::todo::String;

/// A duration in milliseconds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Milliseconds<T>(pub T);

/// [`Milliseconds`] with a `u64` inner
pub type Millis = Milliseconds<u64>;

impl core::ops::Add for Millis {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Millis(self.0 + rhs.0)
  }
}

impl core::ops::Sub for Millis {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Millis(self.0.saturating_sub(rhs.0))
  }
}

impl core::ops::Mul<u64> for Millis {
  type Output = Self;

  fn mul(self, rhs: u64) -> Self {
    Millis(self.0 * rhs)
  }
}

/// An error returned when the system clock cannot be read
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockError;

/// A point in time, represented as milliseconds since an arbitrary epoch
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Instant(pub u64);

impl Instant {
  /// Create a new Instant from a millisecond timestamp
  pub fn new(millis: u64) -> Self {
    Self(millis)
  }

  /// Duration elapsed since the epoch, in milliseconds
  pub fn duration_since_epoch(self) -> Millis {
    Millis(self.0)
  }
}

impl core::ops::Add<Millis> for Instant {
  type Output = Self;

  fn add(self, rhs: Millis) -> Self {
    Instant(self.0 + rhs.0)
  }
}

impl core::ops::Sub for Instant {
  type Output = Millis;

  fn sub(self, rhs: Self) -> Millis {
    Millis(self.0.saturating_sub(rhs.0))
  }
}

/// A clock that can return the current time as milliseconds since an arbitrary epoch
pub trait Clock: core::fmt::Debug {
  /// Get the current time
  fn try_now(&self) -> Result<Instant, ClockError>;
}

/// Timeout configuration allowing for "never time out" as an option
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Timeout {
  /// Timeout after some number of milliseconds has elapsed
  Millis(u64),
  /// Never time out
  Never,
}

/// Data associated with a timestamp
pub struct Stamped<T>(pub T, pub Instant);

impl<T: core::fmt::Debug> core::fmt::Debug for Stamped<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    use core::fmt::Write;

    let mut instant = String::<100>::default();
    write!(
      instant,
      "<{}ms since epoch>",
      self.1.duration_since_epoch().0
    )?;

    f.debug_tuple("Stamped")
      .field(&self.0)
      .field(&instant)
      .finish()
  }
}

impl<T: PartialEq> PartialEq for Stamped<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl<T: Eq> Eq for Stamped<T> {}

impl<T: PartialOrd> PartialOrd for Stamped<T> {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    use core::cmp::Ordering;

    match self.0.partial_cmp(&other.0) {
      | Some(Ordering::Equal) => Some(self.1.cmp(&other.1)),
      | ne => ne,
    }
  }
}

impl<T: Ord> Ord for Stamped<T> {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    use core::cmp::Ordering;

    match self.0.cmp(&other.0) {
      | Ordering::Equal => self.1.cmp(&other.1),
      | ne => ne,
    }
  }
}

impl<T: Default> Default for Stamped<T> {
  fn default() -> Self {
    Self(T::default(), Instant::new(0))
  }
}

impl<T: Clone> Clone for Stamped<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1)
  }
}

impl<T: Copy> Copy for Stamped<T> {}

impl<T> Stamped<T> {
  /// Create a new stamped value using the current time from `clock`
  pub fn new<C: Clock>(clock: &C, t: T) -> Result<Self, ClockError> {
    clock.try_now().map(|now| Self(t, now))
  }

  /// TODO
  pub fn as_ref(&self) -> Stamped<&T> {
    Stamped(&self.0, self.1)
  }

  /// TODO
  pub fn as_mut(&mut self) -> Stamped<&mut T> {
    Stamped(&mut self.0, self.1)
  }

  /// TODO
  pub fn data(&self) -> &T {
    &self.0
  }

  /// TODO
  pub fn time(&self) -> Instant {
    self.1
  }

  /// TODO
  pub fn discard_timestamp(self) -> T {
    self.0
  }

  /// TODO
  pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Stamped<R> {
    Stamped(f(self.0), self.1)
  }

  /// TODO
  pub fn find_latest(winner: Option<Stamped<T>>, cur: Stamped<T>) -> Option<Stamped<T>> {
    Some(
      winner
        .filter(|winner| winner.time() > cur.time())
        .unwrap_or(cur),
    )
  }
}
