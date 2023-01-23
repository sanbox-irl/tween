//! In this example, we explore working with Delayed Tweens using a negative time interval.
//! This is relatively simple, but an extension of the API which users can trivially wrap.

use tween::{CurrentTimeState, SineInOut, Tweener};

pub fn main() {
    let (start, end) = (0, 100);
    let duration = 15.0;
    // notice that this is NEGATIVE
    let current_time = -15.0;

    let mut tweener = Tweener::new_at(start, end, duration, SineInOut, current_time);
    let mut position = 0;

    const DT: f32 = 1.0 / 60.0;

    // and then in your main loop...
    loop {
        let new_position = tweener.move_by(DT);
        match tweener.current_time_state() {
            CurrentTimeState::Waiting => {
                // do nothing -- this is how we wait around!
            }
            CurrentTimeState::Valid => {
                // assign the poistion out
                position = new_position;
            }
            CurrentTimeState::Finished => {
                break;
            }
        }
    }

    assert!(tweener.is_finished());
    assert_eq!(tweener.current_time_state(), CurrentTimeState::Finished);

    assert_eq!(position, 100, "we've moved to the end of the tween");
}
