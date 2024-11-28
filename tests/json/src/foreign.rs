use crate::i18n::*;
use tests_common::*;

#[test]
fn foreign_key_to_string() {
    let en = td!(Locale::en, foreign_key_to_string);
    assert_eq_rendered!(en, "before Click to increment the counter after");
    let fr = td!(Locale::fr, foreign_key_to_string);
    assert_eq_rendered!(fr, "before Cliquez pour incrémenter le compteur after");
}

#[test]
fn foreign_key_to_interpolation() {
    for count in -5..5 {
        let en = td!(Locale::en, foreign_key_to_interpolation, count);
        assert_eq_rendered!(
            en,
            format!(
                "<span>before You clicked </span><span>{}</span><span> times after</span>",
                count
            )
        );
        let fr = td!(Locale::fr, foreign_key_to_interpolation, count);
        assert_eq_rendered!(
            fr,
            format!(
                "<span>before Vous avez cliqué </span><span>{}</span><span> fois after</span>",
                count
            )
        );
    }

    let count = "whatever impl into view";
    let en = td!(Locale::en, foreign_key_to_interpolation, count);
    assert_eq_rendered!(
        en,
        format!(
            "<span>before You clicked </span><span>{}</span><span> times after</span>",
            count
        )
    );
    let fr = td!(Locale::fr, foreign_key_to_interpolation, count);
    assert_eq_rendered!(
        fr,
        format!(
            "<span>before Vous avez cliqué </span><span>{}</span><span> fois after</span>",
            count
        )
    );

    let count = view! { <p>"even a view!"</p> };
    let en = td!(
        Locale::en,
        foreign_key_to_interpolation,
        count = count.clone()
    );
    assert_eq_rendered!(
        en,
        "<span>before You clicked </span><span><p>even a view!</p></span><span> times after</span>"
    );
    let fr = td!(Locale::fr, foreign_key_to_interpolation, count);
    assert_eq_rendered!(fr, "<span>before Vous avez cliqué </span><span><p>even a view!</p></span><span> fois after</span>");
}

#[test]
fn foreign_key_to_subkey() {
    let en = td!(Locale::en, foreign_key_to_subkey);
    assert_eq_rendered!(en, "before subkey_1 after");
    let fr = td!(Locale::fr, foreign_key_to_subkey);
    assert_eq_rendered!(fr, "before subkey_1 after");
}

#[test]
fn foreign_key_to_explicit_default() {
    let en = td!(Locale::en, foreign_key_to_explicit_default);
    assert_eq_rendered!(en, "no explicit default in default locale");
    let fr = td!(Locale::fr, foreign_key_to_explicit_default);
    assert_eq_rendered!(fr, "before this string is declared in locale en after");
}

#[test]
fn populated_foreign_key() {
    let en = td!(Locale::en, populated_foreign_key);
    assert_eq_rendered!(en, "before You clicked 45 times after");
    let fr = td!(Locale::fr, populated_foreign_key);
    assert_eq_rendered!(fr, "before Vous avez cliqué 32 fois after");
}

#[test]
fn populated_foreign_key_with_arg() {
    let en = td!(Locale::en, populated_foreign_key_with_arg, new_count = 12);
    assert_eq_rendered!(
        en,
        "<span>before You clicked </span><span>12</span><span> times after</span>"
    );
    let fr = td!(Locale::fr, populated_foreign_key_with_arg, new_count = 67);
    assert_eq_rendered!(
        fr,
        "<span>before Vous avez cliqué </span><span>67</span><span> fois after</span>"
    );
}

#[test]
fn populated_foreign_key_with_foreign_key_arg() {
    let en = td!(Locale::en, populated_foreign_key_with_foreign_key_arg);
    assert_eq_rendered!(en, "before You clicked subkey_1 times after");
    let fr = td!(Locale::fr, populated_foreign_key_with_foreign_key_arg);
    assert_eq_rendered!(fr, "before Vous avez cliqué subkey_1 fois after");
}
