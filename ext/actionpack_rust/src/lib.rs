use magnus::{function, method, prelude::*, Error, Ruby};

#[magnus::wrap(class = "Euclid::Point", free_immediately, size)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn x(&self) -> isize {
        self.x
    }

    fn y(&self) -> isize {
        self.y
    }
}

fn distance(a: &Point, b: &Point) -> f64 {
    (((b.x - a.x).pow(2) + (b.y - a.y).pow(2)) as f64).sqrt()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Euclid")?;
    let class = module.define_class("Point", ruby.class_object())?;
    class.define_singleton_method("new", function!(Point::new, 2))?;
    class.define_method("x", method!(Point::x, 0))?;
    class.define_method("y", method!(Point::y, 0))?;
    module.define_module_function("distance", function!(distance, 2))?;
    Ok(())
}

