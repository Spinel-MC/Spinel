use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;
use syn::{
    Ident, Lit, LitStr, Token, parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
};

lazy_static! {
    static ref CRATE_NAMESPACE: Mutex<Option<String>> = Mutex::new(None);
}

pub fn set_crate_namespace(namespace: String) {
    let mut ns = CRATE_NAMESPACE.lock().unwrap();
    if ns.is_none() {
        *ns = Some(namespace);
    }
}

pub fn get_crate_namespace() -> String {
    if let Some(ns) = CRATE_NAMESPACE.lock().unwrap().as_deref() {
        return ns.to_string();
    }
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown_crate".to_string());

    if pkg_name.starts_with("spinel") {
        "minecraft".to_string()
    } else {
        pkg_name
    }
}

pub struct AttrsParser {
    pub events: Vec<syn::LitStr>,
    pub modules: Vec<syn::LitStr>,
    pub priority: Option<syn::Expr>,
    pub id: Option<syn::Lit>,
    pub state: Option<syn::Expr>,
    pub recipient: Option<syn::Expr>,
    pub generate_fields: bool,
    pub generate_constructor: bool,
}

impl Parse for AttrsParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut events = Vec::new();
        let mut modules = Vec::new();
        let mut priority = None;
        let mut id = None;
        let mut state = None;
        let mut recipient = None;
        let mut generate_fields = false;
        let mut generate_constructor = false;
        let mut seen_keys = HashSet::new();

        while !input.is_empty() {
            let key_ident: Ident = input.parse()?;
            let key_str = key_ident.to_string();

            if !seen_keys.insert(key_str.clone()) {
                return Err(syn::Error::new_spanned(
                    key_ident,
                    format!("duplicate attribute key: `{}`", key_str),
                ));
            }

            input.parse::<Token![:]>()?;

            match key_str.as_str() {
                "event" => {
                    if input.peek(syn::token::Paren) {
                        let content;
                        parenthesized!(content in input);
                        let parsed: Punctuated<LitStr, Token![,]> =
                            content.parse_terminated(Parse::parse, Token![,])?;
                        events.extend(parsed);
                    } else {
                        events.push(input.parse()?);
                    }
                }
                "module" => {
                    if input.peek(syn::token::Paren) {
                        let content;
                        parenthesized!(content in input);
                        let parsed: Punctuated<LitStr, Token![,]> =
                            content.parse_terminated(Parse::parse, Token![,])?;
                        modules.extend(parsed);
                    } else {
                        modules.push(input.parse()?);
                    }
                }
                "priority" => priority = Some(input.parse()?),
                "id" => id = Some(input.parse()?),
                "state" => state = Some(input.parse()?),
                "recipient" => recipient = Some(input.parse()?),
                "generate_fields" => {
                    let val: Lit = input.parse()?;
                    if let Lit::Bool(b) = val {
                        generate_fields = b.value;
                    }
                }
                "generate_constructor" => {
                    let val: Lit = input.parse()?;
                    if let Lit::Bool(b) = val {
                        generate_constructor = b.value;
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key_ident,
                        "unsupported attribute key",
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AttrsParser {
            events,
            modules,
            priority,
            id,
            state,
            recipient,
            generate_fields,
            generate_constructor,
        })
    }
}

pub struct EventAttrParser {
    pub event: Option<syn::LitStr>,
    pub with_client: bool,
    pub with_server: bool,
    pub priority: Option<syn::Expr>,
    pub dependent: bool,
    pub modules: Vec<syn::LitStr>,
    pub r#async: bool,
}

impl Parse for EventAttrParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut event = None;
        let mut with_client = false;
        let mut with_server = false;
        let mut priority = None;
        let mut dependent = false;
        let mut modules = Vec::new();
        let mut r#async = false;
        let mut seen_keys = HashSet::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            if !seen_keys.insert(key_str.clone()) {
                return Err(syn::Error::new_spanned(
                    &key,
                    format!("duplicate attribute key: `{}`", key_str),
                ));
            }

            input.parse::<Token![:]>()?;

            if key_str == "event" {
                event = Some(input.parse()?);
            } else if key_str == "module" {
                if input.peek(syn::token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    let parsed: Punctuated<LitStr, Token![,]> =
                        content.parse_terminated(Parse::parse, Token![,])?;
                    modules.extend(parsed);
                } else {
                    modules.push(input.parse()?);
                }
            } else if key_str == "priority" {
                priority = Some(input.parse()?);
            } else if key_str == "dependent" {
                let value: Lit = input.parse()?;
                if let Lit::Bool(b) = value {
                    dependent = b.value;
                } else {
                    return Err(syn::Error::new_spanned(
                        value,
                        "expected boolean for `dependent`",
                    ));
                }
            } else {
                let value: Lit = input.parse()?;
                match key_str.as_str() {
                    "with_client" => {
                        if let Lit::Bool(b) = value {
                            with_client = b.value;
                        } else {
                            return Err(syn::Error::new_spanned(
                                value,
                                "expected boolean for `with_client`",
                            ));
                        }
                    }
                    "with_server" => {
                        if let Lit::Bool(b) = value {
                            with_server = b.value;
                        } else {
                            return Err(syn::Error::new_spanned(
                                value,
                                "expected boolean for `with_server`",
                            ));
                        }
                    }
                    "is_async" => {
                        if let Lit::Bool(b) = value {
                            r#async = b.value;
                        } else {
                            return Err(syn::Error::new_spanned(
                                value,
                                "expected boolean for `is_async`",
                            ));
                        }
                    }
                    _ => return Err(syn::Error::new_spanned(key, "unexpected attribute key")),
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(EventAttrParser {
            event,
            with_client,
            with_server,
            priority,
            dependent,
            modules,
            r#async,
        })
    }
}

pub struct ModuleListParser {
    pub modules: Punctuated<LitStr, Token![,]>,
}

impl Parse for ModuleListParser {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            modules: Punctuated::parse_terminated(input)?,
        })
    }
}
