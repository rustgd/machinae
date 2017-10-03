extern crate machina;

use machina::*;

enum HelloState {
    Hello,
    Bye,
}

impl State<String, (), ()> for HelloState {
    fn start(&mut self, args: String) -> Result<Trans<Self>, ()> {
        match *self {
            HelloState::Hello => {
                println!("Hey, {}", args);
            }
            HelloState::Bye => {
                println!("Bye, {}", args);
            }
        }

        Ok(Trans::None)
    }

    fn update(&mut self, args: String) -> Result<Trans<Self>, ()> {
        match *self {
            HelloState::Hello => {
                println!("Update: {}", args);

                Ok(Trans::Push(HelloState::Bye))
            }
            HelloState::Bye => Ok(Trans::Quit),
        }
    }
}

fn run() -> Result<(), ()> {
    let mut machine = StateMachine::new(HelloState::Hello);

    machine.start("you!".to_owned())?;
    machine.update("Whatever".to_owned())?;
    machine.update("Irrelevant".to_owned())?;

    assert!(!machine.running());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occured: {:?}", e);
    }
}
