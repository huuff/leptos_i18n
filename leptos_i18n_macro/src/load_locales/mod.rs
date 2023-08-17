use std::{collections::HashMap, ops::Not, path::Path};

pub mod cfg_file;
pub mod error;
pub mod interpolate;
pub mod key;
pub mod locale;
pub mod parsed_value;
pub mod plural;

use cfg_file::ConfigFile;
use error::Result;
use interpolate::{create_empty_type, Interpolation};
use key::Key;
use locale::{Locale, LocaleValue};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use self::locale::{BuildersKeys, BuildersKeysInner, LocalesOrNamespaces, Namespace};

pub fn load_locales(cfg_file_path: Option<impl AsRef<Path>>) -> Result<TokenStream> {
    let cfg_file = ConfigFile::new(cfg_file_path)?;

    let locales = LocalesOrNamespaces::new(&cfg_file)?;

    let keys = Locale::check_locales(&locales)?;

    let locale_type = create_locale_type(&locales, &keys);
    let locale_variants = create_locales_enum(&cfg_file);
    let locales = create_locales_type(&cfg_file);

    Ok(quote! {
        use ::leptos as __leptos__;

        #locales

        #locale_variants

        #locale_type
    })
}

fn create_locales_enum(cfg_file: &ConfigFile) -> TokenStream {
    let ConfigFile {
        default, locales, ..
    } = cfg_file;

    let as_str_match_arms = locales
        .iter()
        .map(|key| (&key.ident, &key.name))
        .map(|(variant, locale)| quote!(LocaleEnum::#variant => #locale))
        .collect::<Vec<_>>();

    let from_str_match_arms = locales
        .iter()
        .map(|key| (&key.ident, &key.name))
        .map(|(variant, locale)| quote!(#locale => Some(LocaleEnum::#variant)))
        .collect::<Vec<_>>();

    quote! {
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
        #[allow(non_camel_case_types)]
        pub enum LocaleEnum {
            #(#locales,)*
        }

        impl Default for LocaleEnum {
            fn default() -> Self {
                LocaleEnum::#default
            }
        }

        impl ::leptos_i18n::LocaleVariant for LocaleEnum {
            fn as_str(&self) -> &'static str {
                match *self {
                    #(#as_str_match_arms,)*
                }
            }
            fn from_str(s: &str) -> Option<Self> {
                match s {
                    #(#from_str_match_arms,)*
                    _ => None
                }
            }
        }
    }
}

fn create_locales_type(_cfg_file: &ConfigFile) -> TokenStream {
    quote! {
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
        pub struct Locales;

        impl ::leptos_i18n::Locales for Locales {
            type Variants = LocaleEnum;
            type LocaleKeys = I18nKeys;
        }
    }
}

fn create_locale_type_inner(
    type_ident: &syn::Ident,
    locales: &[Locale],
    BuildersKeysInner(keys): &BuildersKeysInner,
) -> TokenStream {
    let string_keys = keys
        .iter()
        .filter(|(_, value)| matches!(value, LocaleValue::String))
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();

    let string_fields = string_keys
        .iter()
        .copied()
        .map(|key| quote!(pub #key: &'static str))
        .collect::<Vec<_>>();

    let builders = keys
        .iter()
        .filter_map(|(key, value)| match value {
            LocaleValue::String => None,
            LocaleValue::Builder(keys) => Some((*key, Interpolation::new(key, keys, locales))),
        })
        .collect::<Vec<_>>();

    let builder_fields = builders.iter().map(|(key, inter)| {
        let inter_ident = &inter.default_generic_ident;
        quote!(pub #key: _builders::#inter_ident)
    });

    let init_builder_fields: Vec<TokenStream> = builders
        .iter()
        .map(|(key, inter)| {
            let ident = &inter.ident;
            quote!(#key: _builders::#ident::new(_variant))
        })
        .collect();

    let from_variant_match_arms = locales.iter().map(|locale| {
        let filled_string_fields = locale.keys.iter().filter_map(|(key, value)| {
            let str_value = value.is_string()?;
            Some(quote!(#key: #str_value))
        });

        let ident = &locale.name.ident;
        quote! {
            LocaleEnum::#ident => #type_ident {
                #(#filled_string_fields,)*
                #(#init_builder_fields,)*
            }
        }
    });

    let builder_impls = builders.iter().map(|(_, inter)| &inter.imp);

    let builder_module = builders.is_empty().not().then(move || {
        let empty_type = create_empty_type();
        quote! {
            #[doc(hidden)]
            pub mod _builders {
                use super::{LocaleEnum, __leptos__};

                #empty_type

                #(
                    #builder_impls
                )*
            }
        }
    });

    quote! {
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
        #[allow(non_snake_case)]
        pub struct #type_ident {
            #(#string_fields,)*
            #(#builder_fields,)*
        }

        impl ::leptos_i18n::LocaleKeys for #type_ident {
            type Locales = Locales;
            fn from_variant(_variant: LocaleEnum) -> Self {
                match _variant {
                    #(
                        #from_variant_match_arms,
                    )*
                }
            }
        }

        #builder_module
    }
}

fn create_namespaces_types(
    i18n_keys_ident: &syn::Ident,
    namespaces: &[Namespace],
    keys: &HashMap<&Key, BuildersKeysInner>,
) -> TokenStream {
    let namespaces_ts = namespaces.iter().map(|namespace| {
        let namespace_ident = &namespace.key.ident;
        let namespace_module_ident = format_ident!("__{}_mod", namespace_ident);
        let builders_keys = keys.get(&namespace.key).unwrap();
        let type_impl =
            create_locale_type_inner(namespace_ident, &namespace.locales, builders_keys);
        quote! {
            pub mod #namespace_module_ident {
                use super::{LocaleEnum, __leptos__, Locales};

                #type_impl
            }
        }
    });

    let namespaces_fields = namespaces.iter().map(|namespace| {
        let key = &namespace.key;
        let namespace_module_ident = format_ident!("__{}_mod", &key.ident);
        quote!(pub #key: __namespaces::#namespace_module_ident::#key)
    });

    let namespaces_fields_new = namespaces.iter().map(|namespace| {
        let key = &namespace.key;
        quote!(#key: ::leptos_i18n::LocaleKeys::from_variant(_variant))
    });

    quote! {
        pub mod __namespaces {
            use super::{LocaleEnum, __leptos__, Locales};

            #(
                #namespaces_ts
            )*

        }

        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
        #[allow(non_snake_case)]
        pub struct #i18n_keys_ident {
            #(#namespaces_fields,)*
        }

        impl ::leptos_i18n::LocaleKeys for #i18n_keys_ident {
            type Locales = Locales;
            fn from_variant(_variant: LocaleEnum) -> Self {
                Self {
                    #(
                        #namespaces_fields_new,
                    )*
                }
            }
        }
    }
}

fn create_locale_type(locales: &LocalesOrNamespaces, keys: &BuildersKeys) -> TokenStream {
    let i18n_keys_ident = format_ident!("I18nKeys");
    match (locales, keys) {
        (LocalesOrNamespaces::NameSpaces(namespaces), BuildersKeys::NameSpaces(keys)) => {
            create_namespaces_types(&i18n_keys_ident, namespaces, keys)
        }
        (LocalesOrNamespaces::Locales(locales), BuildersKeys::Locales(keys)) => {
            create_locale_type_inner(&i18n_keys_ident, locales, keys)
        }
        _ => unreachable!(),
    }
}