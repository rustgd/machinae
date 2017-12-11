# Taking a reference as parameter

If you're taking a reference as parameter,
you can mostly follow [chapter 1.1][c1] or [chapter 1.2][c2],
however with one little exception:

[c1]: ./01_state/01_enum.html
[c2]: ./01_state/02_dyn.html

Your implementation for `State` needs to work with every possible
lifetime `'a`. So it has to look like that:

```rust,ignore
impl<'a> State<&'a Argument, Error, Event> for MyState {}
```

It is very important that `'a` is not bound on anything.
If your state has a lifetime `'b`, you have to declare
two lifetimes for the `State` implementation, otherwise
a given `MyState<'a>` would only implement `State<'a>`
(but it has to implement `State` for every possible
lifetime which Rust expresses with `for<'a> State<'a>`).
