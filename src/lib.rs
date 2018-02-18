//! # machinae
//!
//! `machinae` provides a generic state machine with a strong focus efficiency.
//! It expects you to use enums for the states by default, but you can also work with
//! trait objects by using the `Dyn*` types.
//!
//! ```
//! use machinae::{State, StateMachine, Trans};
//!
//! # #[allow(unused)]
//! struct Event {}
//!
//! enum HelloState {
//!     Hello,
//!     Bye,
//! }
//!
//! impl State<i32, (), Event> for HelloState {
//!     fn start(&mut self, number: i32) -> Result<Trans<Self>, ()> {
//!         match *self {
//!             HelloState::Hello => println!("Hello, {}", number),
//!             HelloState::Bye => println!("Bye, {}", number),
//!         }
//!
//!         Ok(Trans::None)
//!     }
//!
//!     fn update(&mut self, number: i32) -> Result<Trans<Self>, ()> {
//!         match *self {
//!             HelloState::Hello => {
//!                 if number == 5 {
//!                     Ok(Trans::Switch(HelloState::Bye))
//!                 } else {
//!                     Ok(Trans::None)
//!                 }
//!             }
//!             HelloState::Bye => {
//!                 if number == 10 {
//!                     Ok(Trans::Quit)
//!                 } else {
//!                     Ok(Trans::None)
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! let mut machine = StateMachine::new(HelloState::Hello);
//! machine.start(314).unwrap();
//! let mut counter = 1;
//! while machine.running() {
//!     machine.update(counter).unwrap();
//!     counter += 1;
//! }
//! ```
//!

#![warn(missing_docs)]

use std::marker::PhantomData;

#[cfg(test)]
mod tests;

/// Typedef for a state machine using boxed states (`DynState`).
pub type DynMachine<A, E, F> = StateMachine<A, E, F, Box<DynState<A, E, F>>>;

/// Typedef for the result type for `DynState`.
pub type DynResult<A, E, F> = Result<Trans<Box<DynState<A, E, F>>>, E>;

/// A dynamic version of `State` which allows transitions to boxed `DynState`s.
/// If you can use an enum instead, consider using `State` which is slightly more efficient.
///
/// If you're interested in what the methods mean and when they're called, take a look
/// at `State`.
pub trait DynState<A, E, F> {
    /// See `State::start`.
    fn start(&mut self, _args: A) -> DynResult<A, E, F> {
        Ok(Trans::None)
    }
    /// See `State::resume`.
    fn resume(&mut self, _args: A) {}
    /// See `State::pause`.
    fn pause(&mut self, _args: A) {}
    /// See `State::stop`.
    fn stop(&mut self, _args: A) {}
    /// See `State::update`.
    fn update(&mut self, _args: A) -> DynResult<A, E, F> {
        Ok(Trans::None)
    }
    /// See `State::fixed_update`.
    fn fixed_update(&mut self, _args: A) -> DynResult<A, E, F> {
        Ok(Trans::None)
    }
    /// See `State::event`.
    fn event(&mut self, _args: A, _event: F) -> DynResult<A, E, F> {
        Ok(Trans::None)
    }
}

/// A helper type used for the first type parameter of the
/// state machine in case the argument (`A`) is a mutable reference.
pub struct Ref<A: ?Sized> {
    marker: PhantomData<A>,
}

/// `State` trait with several callbacks which allow state transitions.
/// It's recommended that you use this with an enum; if you prefer trait objects,
/// you should use `DynState` instead.
pub trait State<A, E, F>: Sized {
    /// This method will be called before any other method of this state.
    /// Also see `StateMachine::start`.
    fn start(&mut self, _args: A) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
    /// Resumes the `State`. This will be called only if the state has been paused before
    /// (e.g. after a state has been pushed and popped again).
    fn resume(&mut self, _args: A) {}
    /// A state will be paused after it pushed a new state.
    /// Also, `pause` is always called before `stop`.
    fn pause(&mut self, _args: A) {}
    /// A state gets stopped one moment before it gets dropped,
    /// either because it got popped, it did a switch or the state machine has shut down.
    fn stop(&mut self, _args: A) {}
    /// State updates are performed by `StateMachine::update`.
    fn update(&mut self, _args: A) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
    /// Fixed state updates are performed by `StateMachine::fixed_update`.
    fn fixed_update(&mut self, _args: A) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
    /// This method allows supplying events to the state via `StateMachine::event`.
    fn event(&mut self, _args: A, _event: F) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
}

impl<A, E, F> State<A, E, F> for Box<DynState<A, E, F>> {
    fn start(&mut self, args: A) -> Result<Trans<Self>, E> {
        self.as_mut().start(args)
    }

    fn resume(&mut self, args: A) {
        self.as_mut().resume(args);
    }

    fn pause(&mut self, args: A) {
        self.as_mut().pause(args);
    }

    fn stop(&mut self, args: A) {
        self.as_mut().stop(args);
    }

    fn update(&mut self, args: A) -> Result<Trans<Self>, E> {
        self.as_mut().update(args)
    }

    fn fixed_update(&mut self, args: A) -> Result<Trans<Self>, E> {
        self.as_mut().fixed_update(args)
    }

    fn event(&mut self, args: A, event: F) -> Result<Trans<Self>, E> {
        self.as_mut().event(args, event)
    }
}

/// A simple, generic state machine.
/// The argument can be
///
/// 1) an owned value which implements `Clone`
/// 2) an immutable reference
/// 3) a mutable reference
///
/// **In case 3 you need to specify the type of the
/// state machine by either writing `StateMachine<Ref<str>, _, _, _>`
/// or by using `StateMachineRef`.**
pub struct StateMachine<A, E, F, S> {
    marker: PhantomData<(A, E, F)>,
    running: bool,
    stack: Vec<S>,
}

impl<A, E, F, S> StateMachine<A, E, F, S> {
    /// Creates a new state machine with a give `initial` state.
    pub fn new(initial: S) -> Self {
        StateMachine {
            marker: PhantomData,
            running: false,
            stack: vec![initial],
        }
    }

    /// Checks if the state machine is running.
    pub fn running(&self) -> bool {
        self.running
    }

    fn assert_running(&self) {
        assert!(self.running, "State machine not running");
    }

    fn last(&mut self) -> &mut S {
        self.stack.last_mut().unwrap()
    }
}

macro_rules! def_machine {
    ( $param:ty, $clone:ident ) => {
    /// Starts the state machine, calling `.start` on the initial state.
    ///
    /// ## Panics
    ///
    /// Panics if the state machine is running already.
    pub fn start(&mut self, args: $param) -> Result<(), E> {
        if !self.running {
            let trans = self.last().start($clone!(args))?;
            self.running = true;
            self.handle($clone!(args), trans)?;

            Ok(())
        } else {
            panic!("Running already")
        }
    }

    /// Updates the current state by calling `State::update` with `args`.
    ///
    /// ## Panics
    ///
    /// Panics if the state machine is not running.
    pub fn update(&mut self, args: $param) -> Result<(), E> {
        self.assert_running();
        let trans = self.last().update($clone!(args))?;

        self.handle($clone!(args), trans)
    }

    /// Performs a fixed update on the current state
    /// by calling `State::fixed_update` with `args`.
    ///
    /// ## Panics
    ///
    /// Panics if the state machine is not running.
    pub fn fixed_update(&mut self, args: $param) -> Result<(), E> {
        self.assert_running();
        let trans = self.last().fixed_update($clone!(args))?;

        self.handle($clone!(args), trans)
    }

    /// Sends an `event` to the current state.
    ///
    /// ## Panics
    ///
    /// Panics if the state machine is not running.
    pub fn event(&mut self, args: $param, event: F) -> Result<(), E> {
        self.assert_running();
        let trans = self.last().event($clone!(args), event)?;

        self.handle($clone!(args), trans)
    }

    /// Stops the state machine.
    /// This removes all states from the state machine and calls `stop` on them.
    ///
    /// It is highly recommended that you call this method
    /// in case you want to stop the state machine, otherwise the states can't
    /// do anything on a shut down. If `running()` is false already, don't call this method.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use machinae::{State, StateMachine};
    /// # struct A; impl State<(), (), ()> for A {}
    /// # let mut machine = StateMachine::new(A);
    /// # fn shut_down_state_machine() -> bool { true }
    /// while machine.running() {
    ///     if shut_down_state_machine() {
    ///         // If you end the machine yourself you should call
    ///         machine.stop(());
    ///         break;
    ///     }
    /// }
    /// // If the loop exited because `machine.running()` was `false`,
    /// // the machine is already stopped.
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if the state machine is not running.
    pub fn stop(&mut self, args: $param) {
        self.assert_running();

        if let Some(s) = self.stack.last_mut() {
            s.pause($clone!(args));
        }
        while let Some(mut s) = self.stack.pop() {
            s.stop($clone!(args));
        }
        self.running = false;
    }

    fn handle(&mut self, args: $param, trans: Trans<S>) -> Result<(), E> {
        match trans {
            Trans::Push(mut s) => {
                self.last().pause($clone!(args));

                let trans = s.start($clone!(args))?;
                self.handle($clone!(args), trans)?;
                s.resume($clone!(args));
                self.stack.push(s);

                Ok(())
            }
            Trans::Switch(mut s) => {
                {
                    let old = self.last();
                    old.pause($clone!(args));
                    old.stop($clone!(args));
                }

                let trans = s.start($clone!(args))?;
                self.handle($clone!(args), trans)?;
                s.resume($clone!(args));
                *self.last() = s;

                Ok(())
            }
            Trans::Pop => {
                let mut old = self.stack.pop().unwrap();
                old.pause($clone!(args));
                old.stop($clone!(args));

                // We need tp create a boolean here because the borrow checker
                // can't validate this otherwise.
                let resumed = if let Some(s) = self.stack.last_mut() {
                    s.resume($clone!(args));

                    true
                } else {
                    false
                };

                if !resumed {
                    self.stop($clone!(args));
                }

                Ok(())
            }
            Trans::None => Ok(()),
            Trans::Quit => {
                self.stop($clone!(args));

                Ok(())
            }
        }
    }
    };
}

macro_rules! pass_on {
    ($a:ident) => {$a};
}

macro_rules! clone {
    ($a:ident) => {{Clone::clone(& $a)}};
}

impl<A, E, F, S> StateMachine<Ref<A>, E, F, S>
where
    A: ?Sized,
    S: for<'a> State<&'a mut A, E, F>,
{
    def_machine!(&mut A, pass_on);
}

impl<A, E, F, S> StateMachine<A, E, F, S>
where
    A: Clone,
    S: State<A, E, F>,
{
    def_machine!(A, clone);
}

impl<A, E, F, S> Default for StateMachine<A, E, F, S>
where
    S: Default,
{
    fn default() -> Self {
        StateMachine::new(Default::default())
    }
}

/// A state machine accepting a mutable reference as argument.
/// **You need to use this in case you're passing a mutable argument to
/// the state machine, otherwise the compiler will complain.**
pub type StateMachineRef<A, E, F, S> = StateMachine<Ref<A>, E, F, S>;

/// A potential transition to another state.
pub enum Trans<S> {
    /// Pushes another state onto the state machine.
    /// This causes the current state to be paused and the pushed
    /// state to become the current one.
    Push(S),
    /// Removes the current state and pushes a new state to the state machine.
    /// The current state will be paused and stopped.
    Switch(S),
    /// Removes the current state after pausing and stopping it.
    /// If there's a state "below" this one, it'll be resumed and become the
    /// current state. Otherwise, the state machine will shut down.
    Pop,
    /// Don't do any transition, continue with the current state.
    None,
    /// Shuts down the state machine.
    Quit,
}
