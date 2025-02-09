pub(crate) use crate::assets;
pub(crate) use crate::panel::*;
pub(crate) use crate::panels::*;
pub(crate) use anyhow::Result;
pub(crate) use becks_network::*;
pub(crate) use iced::{widget, window, Element, Task};
pub(crate) use log::{debug, error, info, warn};

use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// A clonable single data handle where only the first acquisition returns an value
#[derive(Debug)]
pub struct Acquire<T>(pub Arc<Mutex<Option<T>>>)
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
        self.0.lock().unwrap().take()
    }

    /// Acquires the data, panics if the data has been acquired
    pub fn acquire(self) -> T {
        self.try_acquire().expect("the data has been acquired")
    }
}
