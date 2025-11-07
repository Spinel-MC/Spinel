use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;
use syn::{
    Expr, Ident, Lit, LitStr, Token, Type, parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
};

lazy_static! {
    static ref CRATE_NAMESPACE: Mutex<Option<String>> = Mutex::new(None);
}

pub fn set_crate_namespace(namespace: String) {
    let mut ns = CRATE_NAMESPACE.lock().unwrap();
    if ns.is_some() {
    } else {
        *ns = Some(namespace);
    }
}

pub fn get_crate_namespace() -> String {
    if let Some(ns) = CRATE_NAMESPACE.lock().unwrap().as_deref() {
        return ns.to_string();
    }

    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown_crate".to_string());

    if pkg_name == "spinel" {
        "minecraft".to_string()
    } else {
        pkg_name
    }
}

pub struct Field {
    pub name: Ident,
    pub _colon_token: Token![:],
    pub ty: Ident,
    pub len_arg: Option<syn::LitInt>,
    pub generic_param: Option<Type>,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let colon_token: Token![:] = input.parse()?;
        let ty: Ident = input.parse()?;

        let mut len_arg = None;
        let mut generic_param = None;

        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            len_arg = Some(content.parse()?);
        } else if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            generic_param = Some(input.parse()?);
            input.parse::<Token![>]>()?;
        }

        Ok(Field {
            name,
            _colon_token: colon_token,
            ty,
            len_arg,
            generic_param,
        })
    }
}

pub struct FieldsAttr {
    pub _paren_token: syn::token::Paren,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for FieldsAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        Ok(FieldsAttr {
            _paren_token: paren_token,
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

pub struct AttrsParser {
    pub id: Option<syn::LitInt>,
    pub state: Option<syn::Expr>,
    pub events: Vec<syn::LitStr>,
    pub modules: Vec<syn::LitStr>,
    pub fields: Option<FieldsAttr>,
    pub priority: Option<syn::Expr>,
}

impl Parse for AttrsParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut id = None;
        let mut state = None;
        let mut events = Vec::new();
        let mut modules = Vec::new();
        let mut fields_attr_parsed = None;
        let mut priority = None;
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

            if key_str == "fields" {
                fields_attr_parsed = Some(input.parse()?);
            } else if key_str == "event" {
                if input.peek(syn::token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    let parsed: Punctuated<LitStr, Token![,]> =
                        content.parse_terminated(Parse::parse, Token![,])?;
                    events.extend(parsed);
                } else {
                    events.push(input.parse()?);
                }
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
            } else if key_str == "id" {
                let value: Lit = input.parse()?;
                id = Some(value.try_into_int("id")?);
            } else {
                let value: Expr = input.parse()?;
                match key_str.as_str() {
                    "state" => state = Some(value),
                    "priority" => priority = Some(value),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            key_ident,
                            "unsupported attribute key",
                        ));
                    }
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AttrsParser {
            id,
            state,
            events,
            modules,
            fields: fields_attr_parsed,
            priority,
        })
    }
}

pub struct EventAttrParser {
    pub event: syn::LitStr,
    pub with_client: bool,
    pub priority: Option<syn::Expr>,
    pub dependent: bool,
    pub modules: Vec<syn::LitStr>,
    pub r#async: bool,
}

impl Parse for EventAttrParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut event = None;
        let mut with_client = false;
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
            event: event.ok_or_else(|| input.error("missing required `event` attribute"))?,
            with_client,
            priority,
            dependent,
            modules,
            r#async,
        })
    }
}

pub trait LitExt {
    fn try_into_int(self, attr_name: &str) -> Result<syn::LitInt>;
}

impl LitExt for Lit {
    fn try_into_int(self, attr_name: &str) -> Result<syn::LitInt> {
        if let Lit::Int(lit_int) = self {
            Ok(lit_int)
        } else {
            Err(syn::Error::new_spanned(
                self,
                format!("expected integer literal for `{}`", attr_name),
            ))
        }
    }
}
