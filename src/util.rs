use std::fmt::Display;

pub fn complain<T, E>(message: &'static str) -> impl Fn(E) -> Result<T, E>
where E: Display
{
    move |e| { println!("{}: {}", message, e); Err(e) }
}
