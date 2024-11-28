//! Collects an iterator of views into a single view, wrapping each one with a <span>.
//! This avoids a nasty leptos 0.6 bug with fragments.

use leptos::*;

/// Collects an iterator of views into a single view, wrapping each one with a <span>.
/// This avoids a nasty leptos 0.6 bug with fragments.
pub fn collect_into_spans<I, T>(iter: I) -> View
where
    I: IntoIterator<Item = T>,
    T: IntoView,
{
    iter.into_iter()
        .map(|v| view!(<span>{v}</span>).into_view())
        .collect::<Vec<View>>()
        .into_view()
}
