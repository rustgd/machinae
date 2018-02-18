# Implementing `State` for multiple types

This chapter describes how to work with machinae
if you want to use trait objects for your states.
This allows you to better organize the states using multiple
types, but it's not as efficient since it uses dynamic dispatch
and allocates on the heap.

You'll need to import the following machinae types,
plus your argument, error and event types:

```rust,ignore
use machinae::{DynMachine, DynResult, DynState, Trans};
use some_module::{Argument, Error, Event};
```

Next, define the structs you want to use as states:

```rust,ignore
struct State1;

struct State2(i32);
```

Because we're not using a single type here, we can't
use `State` directly, but rather implement `DynState`
for every struct. A boxed `DynState` automatically implements
`State`.

```rust
impl DynState<Argument, Error, Event> for State1 {
    fn start(&mut self, args: Argument) -> DynResult<i32, Error, Event> {
        Ok(Trans::None)
    }

    fn update(&mut self, args: Argument) -> DynResult<i32, Error, Event> {
        Ok(Trans::Switch(Box::new(State2(42))))
    }
}

impl DynState<i32, Error, Event> for State2 {
    fn start(&mut self, args: Argument) -> DynResult<i32, Error, Event> {
        Ok(Trans::Quit)
    }
}
```

And finally you can use the `DynMachine` typedef to
create a new state machine:

```rust,ignore
let mut machine = DynMachine::new(Box::new(State1));
```
