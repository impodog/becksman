pub(crate) use crate::assets;
pub(crate) use crate::panel::*;
pub(crate) use crate::panels::*;
pub(crate) use crate::repr::*;
pub(crate) use anyhow::Result;
pub(crate) use becks_network::*;
pub(crate) use iced::{widget, window, Element, Task};
pub(crate) use log::{debug, error, info, warn};
pub(crate) use std::sync::Arc;

use std::fmt::Debug;
use std::sync::Mutex;

/// A clonable single data handle where only the first acquisition returns an value
#[derive(Debug)]
pub struct Acquire<T>(Arc<Mutex<Option<T>>>)
where
    T: Debug;

impl<T> Clone for Acquire<T>
where
    T: Debug,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone();
    }
}

impl<T> Acquire<T>
where
    T: Debug,
{
    /// Creates a new acquire resource
    pub fn new(value: T) -> Self {
        Self(Arc::new(Mutex::new(Some(value))))
    }

    /// Acquires the data, returns [`Some`] if this is not previously acquired
    pub fn try_acquire(self) -> Option<T> {
        // Here try_lock is used, because if another thread is acquiring the lock, acquisition
        // will not succeed anyway
        self.0.try_lock().ok().and_then(|mut lock| lock.take())
    }

    /// Acquires the data, panics if the data has been acquired
    pub fn acquire(self) -> T {
        self.try_acquire().expect("the data has been acquired")
    }
}

pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
