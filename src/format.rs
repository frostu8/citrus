// NOTE: allow this so clippy doesn't complain about text_format!
// by the way this is a terrible implementation of a storage system, if I was
// any less tired I would make a PR.
#![allow(clippy::from_over_into)]

use yew::text_format;

/// A wrapper for the RON format.
pub struct Ron<T>(pub T);

text_format!(Ron based on ron);
