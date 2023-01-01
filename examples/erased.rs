//! In this example, we explore working with Erased Tweeners.
//! An "Erased" tweener will be extremely useful for most game engines,
//! where you'll want one component or field to store all your tweens of
//! a certain value and kind. For example, a Ui struct might want to have a
//! "transform" tween which tweens Vec2s over some frames. But sometimes you might
//! want that Tween to be `SineIn`, other times `CircOut`, etc etc.
//!
//! Because all the tweens in this library are ZSTs, you won't actually pay for an allocation,
//! but you will be using dynamic access. This is totally fine -- in practice, this reduces
//! performance by a small amount, but it takes ~10million tween operations to last 1ms, so any
//! dynamic access will not impact performance.

use tween::{FixedTweener, Linear, Tween, Tweener};

pub fn main() {
    // this is how we erase a tween
    let _tweener_of_erased: Tweener<i32, i32, Box<dyn Tween<i32>>> = Tweener::new(0, 0, 0, Box::new(Linear));

    // here's another way to erase a tween...
    let tweener_of_erased = Tweener::new(0, 0, 0, Box::new(Linear) as Box<dyn Tween<i32>>);

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

/// This is how you write a generic function which needs to access a variety of tweeners!
fn access_tweener<Value, Time, T>(tweener: &Tweener<Value, Time, T>)
where
    Value: tween::TweenValue,
    Time: tween::TweenTime,
    T: Tween<Value>,
{
    tweener.final_value();
}
