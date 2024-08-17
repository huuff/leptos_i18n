use serde::de::DeserializeOwned;

use super::error::{Error, Result};
use crate::utils::key::Key;
use std::{borrow::Cow, collections::HashSet, path::PathBuf, rc::Rc};

#[derive(Debug)]
pub struct ConfigFile {
    pub default: Rc<Key>,
    pub locales: Vec<Rc<Key>>,
    pub name_spaces: Option<Vec<Rc<Key>>>,
    pub locales_dir: Cow<'static, str>,
}

impl ConfigFile {
    fn contain_duplicates(locales: &[Rc<Key>]) -> Option<HashSet<String>> {
        // monkey time

        let mut marked = HashSet::with_capacity(locales.len());

        let mut duplicates = None;

        for key in locales {
            if !marked.insert(key) {
                duplicates
                    .get_or_insert_with(HashSet::new)
                    .insert(key.name.clone());
            }
        }

        duplicates
    }

    pub fn new(manifest_dir_path: &mut PathBuf) -> Result<ConfigFile> {
        manifest_dir_path.push("Cargo.toml");

        #[allow(clippy::needless_borrows_for_generic_args)]
        // see https://github.com/rust-lang/rust-clippy/issues/12856
        let cfg_file_str =
            std::fs::read_to_string(&manifest_dir_path).map_err(Error::ManifestNotFound)?;

        manifest_dir_path.pop();

        let Some((before, i18n_cfg)) = cfg_file_str.split_once("[package.metadata.leptos-i18n]")
        else {
            return Err(Error::ConfigNotPresent);
        };

        // this is to have the correct line number in the reported error.
        let cfg_file_whitespaced = before
            .chars()
            .filter(|c| *c == '\n')
            .chain(i18n_cfg.chars())
            .collect::<String>();

        let mut cfg: ConfigFile =
            toml::de::from_str(&cfg_file_whitespaced).map_err(Error::ConfigFileDeser)?;

        if let Some(i) = cfg.locales.iter().position(|l| l == &cfg.default) {
            // put default as first locale
            cfg.locales.swap(0, i);
        } else {
            let len = cfg.locales.len();
            cfg.locales.push(Rc::clone(&cfg.default));
            cfg.locales.swap(0, len);
        }

        if let Some(duplicates) = Self::contain_duplicates(&cfg.locales) {
            Err(Error::DuplicateLocalesInConfig(duplicates))
        } else if let Some(duplicates) = cfg
            .name_spaces
            .as_deref()
            .and_then(Self::contain_duplicates)
        {
            Err(Error::DuplicateNamespacesInConfig(duplicates))
        } else {
            Ok(cfg)
        }
    }
}

/// -----------------------------------------
/// Deserialization
/// -----------------------------------------

struct CfgFileVisitor;

impl<'de> serde::Deserialize<'de> for ConfigFile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("ConfigFile", Field::FIELDS, CfgFileVisitor)
    }
}

enum Field {
    Default,
    Locales,
    Namespaces,
    LocalesDir,
    Unknown,
}

impl Field {
    const FIELDS: &'static [&'static str] = &["default", "locales", "namespaces", "locales-dir"];
}

struct FieldVisitor;

impl<'de> serde::Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> serde::de::Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "an identifier for the fields {:?}",
            Field::FIELDS
        )
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "default" => Ok(Field::Default),
            "locales" => Ok(Field::Locales),
            "namespaces" => Ok(Field::Namespaces),
            "locales-dir" => Ok(Field::LocalesDir),
            _ => Ok(Field::Unknown), // skip unknown fields
        }
    }
}

impl<'de> serde::de::Visitor<'de> for CfgFileVisitor {
    type Value = ConfigFile;

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        fn deser_field<'de, A, T>(
            option: &mut Option<T>,
            map: &mut A,
            field_name: &'static str,
        ) -> Result<(), A::Error>
        where
            A: serde::de::MapAccess<'de>,
            T: DeserializeOwned,
        {
            if option.replace(map.next_value()?).is_some() {
                Err(serde::de::Error::duplicate_field(field_name))
            } else {
                Ok(())
            }
        }
        let mut default = None;
        let mut locales = None;
        let mut name_spaces = None;
        let mut locales_dir = None;
        while let Some(field) = map.next_key::<Field>()? {
            match field {
                Field::Default => deser_field(&mut default, &mut map, "default")?,
                Field::Locales => deser_field(&mut locales, &mut map, "locales")?,
                Field::Namespaces => deser_field(&mut name_spaces, &mut map, "namespaces")?,
                Field::LocalesDir => deser_field(&mut locales_dir, &mut map, "locales-dir")?,
                Field::Unknown => continue,
            }
        }
        let Some(default) = default else {
            return Err(serde::de::Error::missing_field("default"));
        };

        let Some(locales) = locales else {
            return Err(serde::de::Error::missing_field("locales"));
        };

        let locales_dir = locales_dir
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed("./locales"));

        Ok(ConfigFile {
            default,
            locales,
            name_spaces,
            locales_dir,
        })
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a struct with fields \"default\" and \"locales\""
        )
    }
}
