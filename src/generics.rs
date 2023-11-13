struct Point<T> {
    x: T,
    y: T,
}

struct MultiPoint<T, U> {
    x: T,
    y: U,
}

// If we want to add methods to a generic type, we must specifiy `<T>` after the `impl` keyword
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This is because we can implement methods for concrete types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn run() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = MultiPoint { x: 5, y: 4.0 };
}
