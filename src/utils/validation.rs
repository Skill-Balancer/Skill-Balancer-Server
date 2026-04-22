use num::Zero;
use std::fmt::Display;

pub fn validate<T: Clone, C, E>(param: Option<T>, check: C, err_message: E) -> Result<(), String>
where
    C: Fn(T) -> bool,
    E: Fn(T) -> String,
{
    if let Some(p) = param
        && !check(p.clone())
    {
        return Err(err_message(p));
    }
    Ok(())
}

pub fn positive<T: Clone + PartialOrd + Zero + Display>(
    name: &str,
    value: Option<T>,
) -> Result<(), String> {
    validate(
        value,
        |v| v > T::zero(),
        |v| format!("{} was {}, but needs to be greater than 0", name, v),
    )
}

pub fn range<T: Clone + PartialOrd + Zero + Display>(
    name: &str,
    value: Option<T>,
    min: T,
    max: T,
) -> Result<(), String> {
    validate(
        value,
        |v| v > min && v < max,
        |v| {
            format!(
                "{} was {}, but needs to be between {} and {}",
                name, v, min, max
            )
        },
    )
}
