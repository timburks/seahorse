use crate::Context;
use std::error::Error;
use std::result::Result;

/// Command and application action type
///
/// Example
///
/// ```
/// use seahorse::{Action, Context};
///
/// let action: Action = |c: &Context| {
///     println!("{:?}", c.args);
///     return Ok(());
/// };
/// ```
pub type Action = fn(&Context) -> Result<(), Box<dyn Error>>;
