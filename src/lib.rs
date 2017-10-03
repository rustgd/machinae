use std::marker::PhantomData;

pub trait State<A, E, F>: Sized {
    fn start(&mut self, _args: A) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
    fn resume(&mut self, _args: A) {}
    fn pause(&mut self, _args: A) {}
    fn stop(&mut self, _args: A) {}
    fn update(&mut self, _args: A) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
    fn fixed_update(&mut self, _args: A) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
    fn event(&mut self, _args: A, _event: F) -> Result<Trans<Self>, E> {
        Ok(Trans::None)
    }
}

pub struct StateMachine<A, E, F, S> {
    marker: PhantomData<(A, E, F)>,
    running: bool,
    stack: Vec<S>,
}

impl<A, E, F, S> StateMachine<A, E, F, S> {
    pub fn new(initial: S) -> Self {
        StateMachine {
            marker: PhantomData,
            running: false,
            stack: vec![initial],
        }
    }

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

impl<A, E, F, S> StateMachine<A, E, F, S>
where
    A: Clone,
    S: State<A, E, F>,
{
    pub fn start(&mut self, args: A) -> Result<(), E> {
        if !self.running {
            let trans = self.last().start(args.clone())?;
            self.handle(args, trans)?;
            self.running = true;

            Ok(())
        } else {
            panic!("Running already")
        }
    }

    pub fn update(&mut self, args: A) -> Result<(), E> {
        self.assert_running();
        let trans = self.last().update(args.clone())?;

        self.handle(args, trans)
    }

    fn handle(&mut self, args: A, trans: Trans<S>) -> Result<(), E> {
        match trans {
            Trans::Push(mut s) => {
                self.last().pause(args.clone());

                self.handle(args.clone(), s.start(args.clone())?)?;
                s.resume(args);
                self.stack.push(s);

                Ok(())
            }
            Trans::Switch(mut s) => {
                {
                    let old = self.last();
                    old.pause(args.clone());
                    old.stop(args.clone());
                }

                self.handle(args.clone(), s.start(args.clone())?)?;
                s.resume(args);
                *self.last() = s;

                Ok(())
            }
            Trans::Pop => {
                let mut old = self.stack.pop().unwrap();
                old.pause(args.clone());
                old.stop(args.clone());
                match self.stack.last_mut() {
                    Some(s) => {
                        s.resume(args);
                    }
                    None => self.running = false,
                }

                Ok(())
            }
            Trans::None => Ok(()),
            Trans::Quit => {
                self.running = false;

                Ok(())
            }
        }
    }
}

impl<A, E, F, S> Default for StateMachine<A, E, F, S>
where
    S: Default,
{
    fn default() -> Self {
        StateMachine::new(Default::default())
    }
}

pub enum Trans<S> {
    Push(S),
    Switch(S),
    Pop,
    None,
    Quit,
}
