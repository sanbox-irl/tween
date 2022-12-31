//! In this example, we explore working with erased tweeners.

use tween::{FixedTweener, Linear, Tween, Tweener};

#[cfg(feature = "std")]
pub fn main() {
    // we do some chaos to erase it...
    let tweener_of_erased: Tweener<i32, i32, Box<dyn Tween<i32>>> = Tweener::new(0, 0, 0, Box::new(Linear));
    access_tweener(&tweener_of_erased);

    // crucially, you can write `Send + Sync` or any other trait here to implement it
    let tweener_of_erased: Tweener<i32, i32, Box<dyn Tween<i32> + Send + Sync>> =
        Tweener::new(0, 0, 0, Box::new(Linear));
    std::thread::spawn(move || {
        access_tweener(&tweener_of_erased);
    });

    // here's a fixed version of it
    let fixed_tweener: FixedTweener<i32, i32, Box<dyn Tween<i32>>> = FixedTweener::new(0, 0, 0, Box::new(Linear), 0);

    // remember, `fixed_tweener` implements `Deref` to tweener, so you can generically use `Tweener` if
    // needed!
    access_tweener(&fixed_tweener);
}

#[cfg(not(feature = "std"))]
pub fn main() {}

/// This is how you write a generic function which needs to access a variety of tweeners!
#[cfg(feature = "std")]
fn access_tweener<Value, Time, T>(tweener: &Tweener<Value, Time, T>)
where
    Value: tween::TweenValue,
    Time: tween::TweenTime,
    T: Tween<Value>,
{
    tweener.final_value();
}
