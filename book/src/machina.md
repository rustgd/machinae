# machina

This is the documentation for [machina][ma], a generic state machine
intended to be primarily used in game development.

In addition to this book you can find a list of the items in
the [API documentation][ap].

[ma]: https://github.com/rustgd/machina
[ap]: https://docs.rs/machina

You'll mostly need the types `StateMachine`, `State` and `Trans`.
The state machine stores a stack of states and updates that stack
according to the `Trans`itions returned by the current state.

## Book structure

This book is split into five chapters, this being the introduction. After this chapter:

 * [Implementing State][im]
 * [State methods and transitions][me]
 * [Using machina outside of game development][us]
 * [Troubleshooting][tr]

[im]: ./01_state.html
[me]: ./02_methods.html
[us]: ./03_non_game_dev.html
[tr]: ./04_trouble.html

## Example code

```rust
extern crate machina;

use machina::*;

#[derive(Debug)]
struct Error;

enum Event {
    WindowQuit,
    KeyPress(char),
}

struct Game {
    // ..
}

enum GameState {
    Loading,
    MainMenu,
    InGame,
}

impl<'a> State<&'a mut Game, Error, Event> for GameState {
    fn start(&mut self, args: &mut Game) -> Result<Trans<Self>, Error> {
        match *self {
            GameState::Loading => {
                args.load("buttons");
                args.load("player");
                
                Trans::None
            }
            GameState::MainMenu => {}
            GameState::InGame => {
                if !args.login() {
                    Trans::None
                } else {
                    eprintln!("Login failed");
                                        
                    Trans::Pop
                }
            }
        }
    }

    // all methods have a default no-op implementation,
    // so you can also leave them out
    fn resume(&mut self, _: &mut Game) {}

    fn pause(&mut self, _: &mut Game) {}

    fn stop(&mut self, args: &mut Game) {
        match *self {
            GameState::Loading => {}
            GameState::MainMenu => {}
            GameState::InGame => args.logout(),
        }
    }

    fn update(&mut self, args: &mut Game) -> Result<Trans<Self>, Error> {
        match *self {
            GameState::Loading => {
                let progress = args.progress();
                args.draw_bar(progress);
                
                if progress == 1.0 {
                    Trans::Switch(GameState::MainMenu)
                } else {
                    Trans::None
                }
            }
            GameState::MainMenu => {
                if args.button("start_game") {
                    Trans::Push(GameState::InGame)
                } else {
                    Trans::None
                }
            }
            GameState::InGame => {
                args.draw("player");
                
                if args.is_alive("player") {
                    Trans::None
                } else {
                    // Don't let the user rage quit
                    Trans::Quit
                }
            },
        }
    }

    fn fixed_update(&mut self, args: &mut Game) -> Result<Trans<Self>, Error> {
        match *self {
            GameState::Loading => {}
            GameState::MainMenu => {}
            GameState::InGame => args.do_physics(),
        }
    }

    fn event(&mut self, args: &mut Game, event: Event) -> Result<Trans<Self>, Error> {
        match event {
            Event::KeyPress('w') => args.translate("player", 3.0, 0.0),
            Event::KeyPress('q') => Trans::Quit,
            Event::WindowQuit => Trans::Quit,
        }
    }
}

fn run() -> Result<(), Error> {
    use std::time::{Duration, Instant};

    let mut machine = StateMachineRef::new(GameState::Loading);

    let mut game = Game { /*..*/ };

    machine.start(&mut game)?;
    machine.fixed_update(&mut game)?;
    
    let mut last_fixed = Instant::now();
    
    while machine.running() {
        for event in window.poll_events() {
            machine.event(&mut game, event)?;
        }        
    
        if last_fixed.elapsed().subsec_nanos() > 4_000_000 {
            machine.fixed_update(&mut game)?;
        }
    
        machine.update(&mut game)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occurred: {:?}", e);
    }
}
```
