mod default;

use super::Location;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature="serde_support", serde(tag="type"))]
pub enum OuterLocation<'m>
{
	Any,
	Some(Box<Location<'m>>),
	None,
}
