use std::{collections::BinaryHeap, cmp::Ordering};

#[derive(Clone, Copy, Debug)]
pub enum EventType {
  Todo
}

#[derive(Clone, Copy, Debug)]
pub struct Event {
  event: EventType,
  /// Cycle on which event should occur
  timestamp: usize,
}

impl PartialEq for Event {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.timestamp == other.timestamp
  }
}

impl Eq for Event {}

impl PartialOrd for Event {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.timestamp.cmp(&other.timestamp))
  }
}

impl Ord for Event {
  fn cmp(&self, other: &Self) -> Ordering {
    self.timestamp.cmp(&other.timestamp)
  }
}

pub struct Scheduler {
  /// Current timestamp in cycles
  timestamp: usize,
  events: BinaryHeap<Event>,
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

  /// Update scheduler timestamp by specified amount of cycles
  #[inline]
  pub fn update(&mut self, cycles: usize) {
    self.timestamp += cycles;
  }

  /// Get current scheduler timestamp
  #[inline]
  pub fn timestamp(&self) -> usize {
    self.timestamp
  }

  /// Schedule an event for execution in `when` cycles from now
  #[inline]
  pub fn schedule(&mut self, when: usize, event: EventType) {
    self.events.push(Event {
      event,
      timestamp: self.timestamp + when
    })
  }

  /// Retrieve the next pending event that is ready for execution, removing it from the scheduler.
  #[inline]
  pub fn poll_event(&mut self) -> Option<EventType> {
    let Some(event) = self.events.peek() else {
      return None
    };
    if self.timestamp >= event.timestamp {
      return None
    }
    // SAFETY: events.peek() above guarantees that event exists
    // (`unwrap_unchecked` should panic on debug)
    unsafe { Some(self.events.pop().unwrap_unchecked().event) }
  }

  /// Fast forward to the next event
  #[inline]
  pub fn fast_forward(&mut self) {
    let Some(event) = self.events.peek() else { return };
    self.timestamp = event.timestamp;
  }
}
