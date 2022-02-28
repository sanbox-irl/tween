pub struct Tweener<T> {
    tween: T,
}

impl<T: Tween> Tweener<T> {
    pub fn new(tween: T) -> Self {
        Self { tween }
    }

    pub fn run(&mut self, new_time: f32, value: f32) -> f32 {
        self.tween.update(new_time, value)
    }

    pub fn pipe<OuterTween: Tween>(
        self,
        outer_tween: OuterTween,
    ) -> Tweener<PipedTween<Self, OuterTween>> {
        Tweener::new(PipedTween {
            base: self,
            outer_tween,
        })
    }
}

pub trait Tween {
    fn update(&mut self, new_time: f32, value: f32) -> f32;
}

#[derive(Debug, Default)]
pub struct Linear {
    start_value: f32,
    delta: f32,
    duration: f32,
}

impl Linear {
    pub fn new(start: f32, end: f32, duration: f32) -> Self {
        Self {
            start_value: start,
            delta: end - start,
            duration,
        }
    }
}

impl Tween for Linear {
    fn update(&mut self, new_time: f32, _value: f32) -> f32 {
        let percent_time = new_time / self.duration;
        self.delta * percent_time + self.start_value
    }
}

impl<T> Tween for T
where
    T: FnMut(f32, f32) -> f32,
{
    fn update(&mut self, new_time: f32, current_value: f32) -> f32 {
        self(new_time, current_value)
    }
}

impl<T: Tween> Tween for Tweener<T> {
    fn update(&mut self, new_time: f32, value: f32) -> f32 {
        self.run(new_time, value)
    }
}

pub struct PipedTween<BaseTween, OuterTween> {
    base: BaseTween,
    outer_tween: OuterTween,
}

impl<T0, T1> Tween for PipedTween<T0, T1>
where
    T0: Tween,
    T1: Tween,
{
    fn update(&mut self, new_time: f32, value: f32) -> f32 {
        let value = self.base.update(new_time, value);

        self.outer_tween.update(new_time, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn linear() {
        let mut value = 0.0;
        let mut tweener = Tweener::new(Linear::new(value, 100.0, 10.0));

        for val in 1..=10 {
            value = tweener.run(val as f32, value);
            assert_ulps_eq!(value, (val * 10) as f32);
        }
    }

    #[test]
    fn closure() {
        let mut value = 0.0;
        let mut tweener = Tweener::new(|new_time, new_value| new_time + new_value);

        value = tweener.run(1.0, value);
        assert_ulps_eq!(value, 1.0);

        value = tweener.run(2.0, value);
        assert_ulps_eq!(value, 3.0);

        value = tweener.run(3.0, value);
        assert_ulps_eq!(value, 6.0);
    }

    #[test]
    fn multiple() {
        let mut value = 0.0;
        let mut doubled_linear_tween =
            Tweener::new(Linear::new(0.0, 100.0, 10.0)).pipe(|_, v| v * 2.0);

        value = doubled_linear_tween.run(1.0, value);
        assert_ulps_eq!(value, 20.0);

        // let mut doubling_tweener = Tweener::new(|_, v| v * 2.0);
    }
}
