use super::*;

#[test]
fn start_and_stop() {
    struct MyState;

    impl State<i32, (), ()> for MyState {
        fn start(&mut self, x: i32) -> Result<Trans<Self>, ()> {
            if x == 5 {
                Ok(Trans::Quit)
            } else {
                Ok(Trans::None)
            }
        }
    }

    let mut sm = StateMachine::new(MyState);
    sm.start(3).unwrap();
    assert!(sm.running());
    let mut sm = StateMachine::new(MyState);
    sm.start(5).unwrap();
    assert!(!sm.running());
}
