extern crate crux;
#[macro_use]
extern crate log;

use std::marker::PhantomData;
use crux::{Store, State, Middleware};

pub trait Log {
    fn log(&self) -> String;
}

pub struct Logger<T> {
    phantom: PhantomData<T>
}

impl <T> Logger<T> {
    pub fn new() -> Self {
        Logger {
            phantom: PhantomData,
        }
    }
}

impl <T> Middleware<T> for Logger<T> where
    T: State + Log + Clone,
    T::Action: std::fmt::Debug + Copy {
    fn dispatch(&mut self, store: &Store<T>, next: &mut FnMut(T::Action), action: T::Action) {
        debug!("previous state: {}", store.state().log());
        next(action);
        debug!("action: {:?}", action);
        debug!("next state: {}", store.state().log());
    }
}
