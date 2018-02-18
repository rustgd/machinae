# Implementing `State` for an enum

This is probably the most straightforward way of creating
states. First, you create an enum with all your states:

```rust,ignore
pub enum MyState {
    State1,
    State2,
    State3,
    StateWithData(String),
}
```

Next, you need to import your argument, error and event types:

```rust,ignore
use some_module::{Argument, Event, Error};
```

And then you can finally implement `State`:

```rust,ignore
use machinae::{State, Trans};

impl State<Argument, Error, Event> for MyState {}
```

Then, you'll have to match the enum in every method:

```rust,ignore
fn start(&mut self, arg: Argument) -> Result<Trans<Self>, Error> {
    match *self {
        State1 => Trans::None,
        State2 => Trans::None,
        State3 => Trans::None,
        StateWithData(_) => Trans::None,
    }
}
```
