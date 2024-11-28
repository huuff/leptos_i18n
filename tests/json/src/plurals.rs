use crate::i18n::*;
use tests_common::*;

#[test]
fn cardinal_plural() {
    // count = 0
    let count = move || 0;
    let en = td!(Locale::en, cardinal_plural, count);
    assert_eq_rendered!(en, "<span>0</span><span> items</span>");
    let fr = td!(Locale::fr, cardinal_plural, count);
    assert_eq_rendered!(fr, "0");

    // count = 1
    let count = move || 1;
    let en = td!(Locale::en, cardinal_plural, count);
    assert_eq_rendered!(en, "one item");
    let fr = td!(Locale::fr, cardinal_plural, count);
    assert_eq_rendered!(fr, "1");

    // count = 2..
    for i in [2, 5, 10, 1000] {
        let count = move || i;
        let en = td!(Locale::en, cardinal_plural, count);
        assert_eq_rendered!(en, format!("<span>{}</span><span> items</span>", i));
        let fr = td!(Locale::fr, cardinal_plural, count);
        assert_eq_rendered!(fr, i.to_string());
    }
}

#[test]
fn ordinal_plural() {
    // count = 1
    let count = move || 1;
    let en = td!(Locale::en, ordinal_plural, count);
    assert_eq_rendered!(en, "<span>1</span><span>st place</span>");
    let fr = td!(Locale::fr, ordinal_plural, count);
    assert_eq_rendered!(fr, "<span>1</span><span>re place</span>");

    // count = 2
    let count = move || 2;
    let en = td!(Locale::en, ordinal_plural, count);
    assert_eq_rendered!(en, "<span>2</span><span>nd place</span>");
    let fr = td!(Locale::fr, ordinal_plural, count);
    assert_eq_rendered!(fr, "<span>2</span><span>e place</span>");

    // count = 3
    let count = move || 3;
    let en = td!(Locale::en, ordinal_plural, count);
    assert_eq_rendered!(en, "<span>3</span><span>rd place</span>");
    let fr = td!(Locale::fr, ordinal_plural, count);
    assert_eq_rendered!(fr, "<span>3</span><span>e place</span>");

    // count = 4
    let count = move || 4;
    let en = td!(Locale::en, ordinal_plural, count);
    assert_eq_rendered!(en, "<span>4</span><span>th place</span>");
    let fr = td!(Locale::fr, ordinal_plural, count);
    assert_eq_rendered!(fr, "<span>4</span><span>e place</span>");
}

#[test]
fn args_to_plural() {
    let count = move || 0;
    let en = td!(Locale::en, args_to_plural, count);
    assert_eq_rendered!(en, "<span>en </span><span>0</span>");
    let fr = td!(Locale::fr, args_to_plural, count);
    assert_eq_rendered!(fr, "fr singular");
}

#[test]
fn count_arg_to_plural() {
    let en = td!(Locale::en, count_arg_to_plural, arg = "en");
    assert_eq_rendered!(en, "<span>en</span><span> singular</span>");
    let fr = td!(Locale::fr, count_arg_to_plural, arg = "fr");
    assert_eq_rendered!(fr, "<span>fr</span><span> 2</span>");
}

#[test]
fn foreign_key_to_two_plurals() {
    let count = move || 0;
    let en = td!(Locale::en, foreign_key_to_two_plurals, count);
    assert_eq_rendered!(
        en,
        "<span><span>0</span><span> items</span></span><span> </span><span><span>en </span><span>0</span></span>"
    );
    let fr = td!(Locale::fr, foreign_key_to_two_plurals, count);
    assert_eq_rendered!(fr, "<span>0</span><span> </span><span>fr singular</span>");

    let count = move || 1;
    let en = td!(Locale::en, foreign_key_to_two_plurals, count);
    assert_eq_rendered!(
        en,
        "<span>one item</span><span> </span><span>en singular</span>"
    );
    let fr = td!(Locale::fr, foreign_key_to_two_plurals, count);
    assert_eq_rendered!(fr, "<span>1</span><span> </span><span>fr singular</span>");
}

#[test]
fn renamed_plurals_count() {
    let first_count = move || 0;
    let second_count = move || 1;
    let en = td!(Locale::en, renamed_plurals_count, first_count, second_count);
    assert_eq_rendered!(
        en,
        "<span><span>0</span><span> items</span></span><span> </span><span><span>1</span><span>st place</span></span>"
    );
    let fr = td!(Locale::fr, renamed_plurals_count, first_count, second_count);
    assert_eq_rendered!(
        fr,
        "<span>0</span><span> </span><span><span>1</span><span>re place</span></span>"
    );
}
