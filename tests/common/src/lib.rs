#![deny(warnings)]

pub use leptos::prelude::*;

pub fn render_to_string<T>(view: T) -> String
where
    T: IntoView,
{
    let rendered = view.into_view().to_html();
    let comment_removed = remove_html_comments(rendered);
    let hk_removed = remove_hk(comment_removed);
    let weird_removed = remove_weird_stuff(hk_removed);
    decode_special_chars(weird_removed)
}

fn remove_noise(s: String, start_delim: &str, end_delim: &str) -> String {
    let Some((start, rest)) = s.split_once(start_delim) else {
        return s;
    };
    let mut output_str = start.to_owned();
    let (_, mut s) = rest.split_once(end_delim).unwrap();
    while let Some((start, rest)) = s.split_once(start_delim) {
        output_str.push_str(start);
        let (_, rest) = rest.split_once(end_delim).unwrap();
        s = rest;
    }
    output_str.push_str(s);
    output_str
}

fn remove_weird_stuff(s: String) -> String {
    let Some((before, after)) = s.split_once("<!>") else {
        return s;
    };

    let mut s = before.to_string();
    s.extend(after.split("<!>"));
    s
}

fn remove_html_comments(s: String) -> String {
    remove_noise(s, "<!--", "-->")
}

fn remove_hk(s: String) -> String {
    remove_noise(s, " data-hk=\"", "\"")
}

fn split_html_special_char(s: &str) -> Option<(&str, char, &str)> {
    let (before, rest) = s.split_once("&#x")?;
    let (code, after) = rest.split_once(';')?;
    let code = u32::from_str_radix(code, 16).ok()?;
    let ch = char::from_u32(code)?;

    Some((before, ch, after))
}

fn decode_special_chars(s: String) -> String {
    let Some((before, ch, mut s)) = split_html_special_char(&s) else {
        return s;
    };
    let mut output_str = before.to_owned();
    output_str.push(ch);
    while let Some((before, ch, rest)) = split_html_special_char(s) {
        output_str.push_str(before);
        output_str.push(ch);
        s = rest;
    }
    output_str.push_str(s);
    output_str
}

#[macro_export]
macro_rules! assert_eq_rendered {
    ($left:expr, $($right:tt)*) => {
        assert_eq!(render_to_string($left), $($right)*)
    };
}

#[macro_export]
macro_rules! assert_eq_string {
    ($left:expr, $($right:tt)*) => {
        assert_eq!($left.to_string(), $($right)*)
    };
}
