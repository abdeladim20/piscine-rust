pub fn add_curry<T: std::ops::Add <Output = T> + Copy>(x: T) -> impl Fn(T) -> T {
    move |y| y + x
}

pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T{
    move |x| f(f(x))
}
