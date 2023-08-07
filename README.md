# Leptos i18n

This crate is made to simplify internalisation in a Leptos application, that load locales at **_compile time_** and provide compile time check for keys and selected locale.

## How to use

### Configuration files

There are files that need to exist, the first one is the `i18n.json` file that describe the default locale and supported locales, it need to be at the root of the project and look like this:

```json
{
  "default": "en",
  "locales": ["en", "fr"]
}
```

The other ones are the files containing the translation, they are key-value pairs and need to be situated in the `/locales` directory at root of the project, they should be named `{locale}.json`, one per locale defined in the `i18n.json` file.
They look like this:

```
/locales/en.json

{
    "hello_world": "Hello World!"
}

/locales/fr.json

{
    "hello_world": "Bonjour le monde!"
}

```

All locales files need to have exactly the same keys.

### Loading the locales

you can then use the `load_locales!()` macro in a module of the project, this will load _at compile time_ the locales, and create a struct that describe your locales:

```rust
struct Locale {
    pub hello_world: &'static str
}
```

Two other helper types are created, one enum representing the locales:

```rust
enum LocalesVariants {
    en,
    fr
}
```

and an empty struct named `Locales` that serves as a link beetween the two, it is this one that is the most important, most functions of the crate need this type, not the one containing the locales nor the enum.

### The `t!()` macro

A typical `i18n.rs` module would look like this:

```rust
leptos_i18n::load_locales!();

#[macro_export]
macro_rules! t {
    ($cx: ident) => {
        ::leptos_i18n::t!($cx, $crate::i18n::Locales)
    };
    ($cx: ident, $key: ident) => {
        move || t!($cx).$key
    };
}
```

First line is the macro that load and parse the locales and then create the types.

the crate export a macro named `t!()` that help with extracting the local from the context, but it needs the `Locales` type,
so to avoid retyping it every time we can redefine the macro to already contain the path to the `Locales` type.

The first macro version return the entire locale struct, and you can access every key, the second one is when you just want to put the string in the html:

```rust
view! { cx,
    <p>{t!(cx, hello_world)}</p>
}
```

by wrapping it in a function it allows it to be reactive and if the selected locale change it will display the correct one.

### Context Provider

To make all of that work, it needs to have the `I18nContext` available, for that wrap your application in the `I18nContextProvider`:

```rust
use leptos_i18n::I18nContextProvider;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    leptos_meta::provide_meta_context(cx);

    view! { cx,
        <I18nContextProvider locales=Locales>
            {/* ... */}
        </I18nContextProvider>
    }
}
```

You must provide you `Locales` type to the context provider so it can infer the needed related types, this type being an empty struct it can be created for 0 cost.

In the server side, when a client make a request it include in the request headers a weighted list of accepted languages,
this crate parse this header and try to match those languages against the defined locales to find the one that suits the client the best.

### Setting the locale

You can use the `set_locale` function to change the locale:

```rust
let set_locale = leptos_i18n::set_locale::<Locales>(cx);
let on_click = move |_| set_locale(LocaleEnum::fr);

view! { cx,
    <button on:click=on_click>
        {t!(cx, set_locale_to_french)}
    </button>
}

```

The `t!()` macro suscribe to locale change so every translation will switch to the new locale.

When a new locale is set, a cookie is set on the client side to remember the prefered locale. If you are using Chromium on localhost it may not work, as it blocks cookie set on the client side, try with another browser like Firefox.

If examples works better for you, you can look at the different examples available on the Github.

## Features

You must enable the `hydrate` feature when building the client, and when building the server you must enable either the `actix` or `axum` feature.

## What's to come ?

The main focus now is to be able to interpolate values in the translation, so you could have

```json
{
  "bananas": "Henry as {{ banana_count }} bananas"
}
```

and being able to do something like this:

```rust
let count = ...;

view! { cx,
    <p>{t!(cx, hello_world, banana_count = count)}</p>
}
```
