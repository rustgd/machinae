# Implementing `State`

This chapter explains how you implement the states.
For the state lifecycle see the [second chapter][se].

[se]: ./02_methods.html

`machina` is designed to be as lightweight as possible.
To achieve that, the `State`s in the state machine are strongly
typed (in lieu of using trait objects). That means that your
state is likely to be an enum. However, for some scenarios you
need many different states; that can get quite confusing if
you have to `match` your enum in every method. Because of that,
there's a second way to work with this crate: you just create multiple
structs and implement `DynState` for each struct. Every `Box<DynState>`
has an implementation for `State` automatically.

* [Use an enum for your `State`][en]
* [Check out `DynState` instead][dy]

[en]: ./01_state/01_enum.html
[dy]: ./01_state/02_dyn.html

## Working with references

In case your argument to the states is a reference, there are
some things you need pay attention to. [Chapter 1.3][ch] describes
how to do that.

[ch]: ./01_state/03_ref.html
