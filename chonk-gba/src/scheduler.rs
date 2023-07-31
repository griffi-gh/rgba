use std::{collections::{BinaryHeap, binary_heap::PeekMut}, cmp::Ordering};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  _Todo
}

#[derive(Clone, Copy, Debug)]
struct FutureEvent {
  event: Event,
  timestamp: u64,
}

impl PartialEq for FutureEvent {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.timestamp == other.timestamp
  }
}

impl Eq for FutureEvent {}

impl PartialOrd for FutureEvent {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for FutureEvent {
  fn cmp(&self, other: &Self) -> Ordering {
    self.timestamp.cmp(&other.timestamp)
  }
}

pub struct Scheduler {
  /// Current timestamp in cycles
  timestamp: u64,
  events: BinaryHeap<FutureEvent>,
}

impl Scheduler {
  /// Create a new Scheduler
  #[inline]
  pub fn new() -> Self {
    Self {
      timestamp: 0,
      events: BinaryHeap::with_capacity(128),
    }
  }

  /// Advance the scheduler's timestamp by a specified number of cycles.
  #[inline]
  pub fn advance(&mut self, cycles: u64) {
    self.timestamp += cycles;
  }

  /// Get current scheduler timestamp
  #[inline]
  pub fn timestamp(&self) -> u64 {
    self.timestamp
  }

  /// Schedule an event for execution in `when` cycles from now
  #[inline]
  pub fn schedule_relative(&mut self, when: u64, event: Event) {
    self.events.push(FutureEvent {
      event,
      timestamp: self.timestamp + when
    })
  }
  
  /// Schedule an event to be executed on exactly `when`th cycle
  #[inline]
  pub fn schedule_absolute(&mut self, when: u64, event: Event) {
    self.events.push(FutureEvent {
      event,
      timestamp: when
    })
  }

  /// Retrieve the next pending event that is ready for execution, removing it from the scheduler.
  #[inline]
  pub fn poll(&mut self) -> Option<Event> {
    self.events.peek_mut().and_then(|event| {
      (self.timestamp >= event.timestamp).then(|| PeekMut::pop(event).event)
    })
  }

  /// Fast forward to the next event
  #[inline]
  pub fn fast_forward(&mut self) {
    if let Some(event) = self.events.peek() {
      self.timestamp = event.timestamp;
    }
  }
}
