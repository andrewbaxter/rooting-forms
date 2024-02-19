use std::{
    convert::Infallible,
    fmt::Display,
    str::FromStr,
};
use rooting::{
    El,
    el,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::{
    css::{
        ATTR_LABEL,
        CSS_CLASS_ERROR,
        CSS_CLASS_SMALL_INPUT,
    },
    FormWith,
    FormElements,
    FormState,
};

/// A minimal string wrapper that creates a password form input.
pub struct Password(pub String);

impl FromStr for Password {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Password(s.to_string()));
    }
}

/// A minimal string wrapper that creates a textarea form input.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BigString(pub String);

impl FromStr for BigString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(BigString(s.to_string()));
    }
}

/// A helper form type for rust types that implement `FromStr`.
pub struct FromStrFormState {
    el: El,
    error_el: El,
}

impl FromStrFormState {
    pub fn new<
        E: Display,
        T: FromStr<Err = E>,
    >(label: &str, type_: &str, initial_value: &str) -> Box<dyn FormState<T>> {
        let error_el = el("span").classes(&[CSS_CLASS_ERROR]);
        return Box::new(FromStrFormState {
            el: el("input")
                .classes(&[CSS_CLASS_SMALL_INPUT])
                .attr(ATTR_LABEL, label)
                .attr("type", type_)
                .attr("value", initial_value)
                .on("change", {
                    let error_el = error_el.clone();
                    move |ev| {
                        let text = ev.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value();
                        if text.len() >= 1 {
                            match T::from_str(&text) {
                                Err(e) => {
                                    error_el.ref_text(&e.to_string());
                                    return;
                                },
                                _ => { },
                            }
                        }
                        error_el.ref_text("");
                    }
                }),
            error_el: error_el,
        });
    }
}

impl<E: Display, T: FromStr<Err = E>> FormState<T> for FromStrFormState {
    fn elements(&self) -> FormElements {
        return FormElements {
            error: Some(self.error_el.clone()),
            elements: vec![self.el.clone()],
        };
    }

    fn parse(&self) -> Result<T, ()> {
        match T::from_str(&self.el.raw().dyn_ref::<HtmlInputElement>().unwrap().value()) {
            Ok(v) => {
                self.error_el.ref_text("");
                return Ok(v);
            },
            Err(e) => {
                self.error_el.ref_text(&e.to_string());
                return Err(());
            },
        }
    }
}

impl<C> FormWith<C> for String {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, String>(
            field,
            "text",
            from.as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for Password {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Password>(
            field,
            "password",
            from.as_ref().map(|x| x.0.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for BigString {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, BigString>(
            field,
            "text",
            from.as_ref().map(|x| x.0.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u8 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u16 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u32 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u64 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i8 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i16 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i32 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i64 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for f32 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for f64 {
    fn new_form(_context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return FromStrFormState::new::<_, Self>(
            field,
            "text",
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}
