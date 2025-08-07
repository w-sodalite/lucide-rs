use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::BTreeMap;

fn main() {
    let version = "0.536.0";
    let bytes = include_bytes!("../data/info.json");
    let icons = serde_json::from_slice::<BTreeMap<String, Icon>>(bytes).unwrap();
    let items = icons
        .into_iter()
        .flat_map(|(name, icon)| {
            let unicode = &icon.unicode;
            let unicode = match unicode.strip_prefix("&#").and_then(|v| v.strip_suffix(";")) {
                Some(unicode) => unicode.parse::<u32>(),
                None => unicode.parse::<u32>(),
            };
            match &unicode {
                Ok(unicode) => Some((format_camel_case(&name), name, *unicode)),
                Err(e) => panic!("{}", e),
            }
        })
        .collect::<Vec<_>>();

    // enum field tokens
    let fields = items.iter().map(|(name, raw, unicode)| {
        let preview = format!(
            "![](https://unpkg.com/lucide-static@{}/icons/{}.svg)",
            version, raw
        );
        let name = format_ident!("{}", name);
        quote! {
            #[doc = #preview]
            #name = #unicode
        }
    });

    // enum variant tokens
    let variants = items.iter().map(|(name, _, _)| {
        let name = format_ident!("{}", name);
        quote! {
            Lucide::#name
        }
    });

    // generate code
    let code = quote! {

        #[repr(u32)]
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Lucide {
            #(
                #fields
            ),*
        }

        impl Lucide {
            ///
            /// Lucide all icons
            ///
            pub const ALL: &'static [Lucide] = &[
                #(
                    #variants
                ),*
            ];

            ///
            /// Lucide font for iced
            ///
            #[cfg(feature = "iced")]
            pub const FONT: iced::Font = iced::Font::with_name("lucide");

            ///
            /// Lucide version
            ///
            pub const fn version() -> &'static str {
                #version
            }

            ///
            /// Lucide font binary data
            ///
            #[cfg(feature = "data")]
            pub const fn font_data() -> &'static [u8] {
                include_bytes!("../data/lucide.ttf")
            }
        }

        impl From<Lucide> for char {
            fn from(value: Lucide) -> Self {
                unsafe {
                    // SAFETY: the value is Unicode
                    char::from_u32_unchecked(value as u32)
                }
            }
        }

        impl std::fmt::Display for Lucide {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                char::from(*self).fmt(f)
            }
        }

        #[cfg(feature = "iced")]
        impl<'a> iced::widget::text::IntoFragment<'a> for Lucide {
            fn into_fragment(self) -> iced::widget::text::Fragment<'a> {
                char::from(self).into_fragment()
            }
        }

        #[cfg(feature = "iced")]
        impl<'a> iced::widget::text::IntoFragment<'a> for &'a Lucide {
            fn into_fragment(self) -> iced::widget::text::Fragment<'a> {
                char::from(*self).into_fragment()
            }
        }
    };

    // generate stub.rs
    std::fs::write("src/stub.rs", code.to_string()).unwrap();
}

fn format_camel_case(input: &str) -> String {
    input
        .split('-')
        .filter_map(|part| {
            if part.is_empty() {
                None
            } else {
                let mut chars = part.chars();
                chars
                    .next()
                    .map(|first_char| first_char.to_ascii_uppercase().to_string() + chars.as_str())
            }
        })
        .collect::<String>()
}

#[derive(Deserialize)]
struct Icon {
    unicode: String,
}
