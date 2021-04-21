mod default;

use
{
	core::
	{
		cmp::{Eq, Ord},
		fmt::Debug,
		hash::Hash,
		iter::Iterator,
	},
	std::{borrow::Cow, collections::HashSet},

	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature="serde_support", serde(content="value", tag="condition"))]
pub enum Match<'element, T> where
	T : Clone + Debug + Hash + Ord
{
	#[cfg_attr(feature="serde_support", serde(bound(deserialize = "T : Deserialize<'de>")))]

	/// # Summary
	///
	/// Always match.
	Any,

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	///
	/// # Example
	///
	/// ```rust
	/// use std::borrow::Cow::Borrowed;
	/// use clinvoice_adapter::data::query::Match;
	///
	/// let equal_to = Match::EqualTo(Borrowed(&5));
	///
	/// assert!(equal_to.matches(&5));
	/// assert!(!equal_to.matches(&4));
	/// assert!(equal_to.set_matches(&([5].iter().collect())));
	/// assert!(!equal_to.set_matches(&([1, 5].iter().collect())));
	/// ```
	EqualTo(Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v` is made up of elements which are contained in this set.
	/// * This set has one element, and `v` is equivalent.
	///
	/// # Example
	///
	/// ```rust
	/// use std::borrow::Cow::Borrowed;
	/// use clinvoice_adapter::data::query::Match;
	///
	/// let has_all = Match::HasAll(vec![Borrowed(&1), Borrowed(&5), Borrowed(&9)].into_iter().collect());
	///
	/// assert!(!has_all.matches(&1));
	/// assert!(!has_all.matches(&3));
	/// assert!(!has_all.set_matches(&([1, 5].iter().collect())));
	/// assert!(has_all.set_matches(&([1, 5, 9].iter().collect())));
	/// ```
	HasAll(HashSet<Cow<'element, T>>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has any value contained in this set.
	/// * `v` is contained within this set.
	///
	/// # Example
	///
	/// ```rust
	/// use std::borrow::Cow::Borrowed;
	/// use clinvoice_adapter::data::query::Match;
	///
	/// let has_any = Match::HasAny(vec![Borrowed(&1), Borrowed(&5), Borrowed(&7), Borrowed(&9)].into_iter().collect());
	///
	/// assert!(has_any.matches(&1));
	/// assert!(!has_any.matches(&4));
	/// assert!(has_any.set_matches(&([1, 10, 20].iter().collect())));
	/// ```
	HasAny(HashSet<Cow<'element, T>>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has no values contained in this set.
	/// * `v` is not contained within this set.
	///
	/// # Example
	///
	/// ```rust
	/// use std::borrow::Cow::Borrowed;
	/// use clinvoice_adapter::data::query::Match;
	///
	/// let has_none = Match::HasNone(vec![Borrowed(&1), Borrowed(&5), Borrowed(&7), Borrowed(&9)].into_iter().collect());
	///
	/// assert!(has_none.matches(&8));
	/// assert!(!has_none.matches(&9));
	/// assert!(has_none.set_matches(&([0, 2, 4, 6].iter().collect())));
	/// ```
	HasNone(HashSet<Cow<'element, T>>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * The value of `v` is greater than or equal to the first value.
	/// * The value of `v` is less than the first value.
	///
	/// # Example
	///
	/// ```rust
	/// use std::borrow::Cow::Borrowed;
	/// use clinvoice_adapter::data::query::Match;
	///
	/// let in_range = Match::InRange(Borrowed(&3), Borrowed(&5));
	///
	/// assert!(in_range.matches(&4));
	/// assert!(!in_range.matches(&5));
	/// assert!(in_range.set_matches(&([0, 1, 3].iter().collect())));
	/// ```
	InRange(Cow<'element, T>, Cow<'element, T>),
}

/// # Summary
///
/// Return whether or not some [`Match::InRange`] is in range.
fn is_in_range<T>(min: &T, max: &T, value: &T) -> bool where
	T : Ord,
{
	min <= value && value < max
}

impl<'element, T> Match<'element, T> where
	T : 'element + Clone + Debug + Hash + Ord
{
	/// # Summary
	///
	/// Determine whether or not a `value` matches.
	///
	/// # Parameters
	///
	/// * `value`, the value to check.
	///
	/// # Returns
	///
	/// * `true`, if the `value` matches the passed [`Match`].
	/// * `false`, if the `value` does not match.
	pub fn matches(&self, value: &T) -> bool
	{
		match self
		{
			Self::Any => true,
			Self::EqualTo(equal_value) => equal_value.as_ref() == value,
			Self::HasAll(required_values) => required_values.len() == 1 && required_values.contains(value),
			Self::HasAny(accepted_values) => accepted_values.contains(value),
			Self::HasNone(denied_values) => !denied_values.contains(value),
			Self::InRange(min, max) => is_in_range(min.as_ref(), max.as_ref(), value),
		}
	}

	/// # Summary
	///
	/// Determine whether or not the `values` match.
	///
	/// # Parameters
	///
	/// * `values`, the values to check.
	///
	/// # Returns
	///
	/// * `true`, if the `values` match the passed [`Match`].
	/// * `false`, if the `values` do not match.
	pub fn set_matches(&self, values: &HashSet<&T>) -> bool
	{
		match self
		{
			Self::Any => true,
			Self::EqualTo(equal_value) => values.len() == 1 && values.contains(equal_value.as_ref()),
			Self::HasAll(required_values) => required_values.iter().map(|v| v.as_ref()).collect::<HashSet<_>>().is_subset(values),
			Self::HasAny(accepted_values) => !accepted_values.iter().map(|v| v.as_ref()).collect::<HashSet<_>>().is_disjoint(values),
			Self::HasNone(denied_values) => denied_values.iter().map(|v| v.as_ref()).collect::<HashSet<_>>().is_disjoint(values),
			Self::InRange(min, max) => values.iter().any(|v| is_in_range(min.as_ref(), max.as_ref(), v)),
		}
	}
}
