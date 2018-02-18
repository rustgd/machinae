extern crate machinae;

use machinae::*;

enum HelloState {
    Hello,
    Bye,
}

impl<'a> State<&'a mut i32, (), ()> for HelloState {
    fn start(&mut self, args: &mut i32) -> Result<Trans<Self>, ()> {
        match *self {
            HelloState::Hello => {
                *args *= *args;
                println!("Hey, {}", args);
            }
            HelloState::Bye => {
                println!("Bye, {}", args);
            }
        }

        Ok(Trans::None)
    }

    fn update(&mut self, args: &mut i32) -> Result<Trans<Self>, ()> {
        match *self {
            HelloState::Hello => {
                *args += 1;
                println!("Update: {}", args);

                Ok(Trans::Push(HelloState::Bye))
            }
            HelloState::Bye => Ok(Trans::Quit),
        }
    }
}

fn run() -> Result<(), ()> {
    // note that we use `StateMachineRef` here.
    let mut machine = StateMachineRef::new(HelloState::Hello);

    let mut context = 5;
    machine.start(&mut context)?;
    machine.update(&mut context)?;
    machine.update(&mut context)?;

    assert!(!machine.running());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occurred: {:?}", e);
    }
}
