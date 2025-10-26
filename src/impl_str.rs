use {
    crate::{
        css::{
            err_el,
            ATTR_LABEL,
            CSS_CLASS_SMALL_INPUT,
            CSS_CLASS_TEXT,
        },
        FormElements,
        FormState,
        FormWith,
    },
    gloo::timers::callback::Timeout,
    rooting::{
        el,
        El,
    },
    std::{
        cell::RefCell,
        convert::Infallible,
        fmt::Display,
        str::FromStr,
    },
    wasm_bindgen::JsCast,
    web_sys::{
        HtmlInputElement,
        HtmlTextAreaElement,
    },
};

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

struct BigStringFormState {
    el: El,
}

impl FormState<BigString> for BigStringFormState {
    fn parse(&self) -> Result<BigString, ()> {
        return Ok(BigString(self.el.raw().dyn_ref::<HtmlTextAreaElement>().unwrap().value()));
    }
}

impl<C> FormWith<C> for BigString {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let textarea =
            el("div")
                .classes(&[CSS_CLASS_SMALL_INPUT, CSS_CLASS_TEXT])
                .attr(ATTR_LABEL, field)
                .attr("contenteditable", "plaintext-only")
                .text(from.map(|x| x.0.as_str()).unwrap_or(""));
        return (FormElements {
            error: None,
            elements: vec![textarea.clone()],
        }, Box::new(BigStringFormState { el: textarea }));
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
    >(label: &str, initial_value: &str) -> (FormElements, Box<dyn FormState<T>>) {
        let error_el = err_el();
        let input_el =
            el("div")
                .classes(&[CSS_CLASS_SMALL_INPUT, CSS_CLASS_TEXT])
                .attr(ATTR_LABEL, label)
                .attr("contenteditable", "plaintext-only")
                .attr("value", initial_value)
                .on("input", {
                    let error_el = error_el.clone();
                    let debounce = RefCell::new(None);
                    move |ev| {
                        error_el.ref_text("");
                        let text = ev.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value();
                        *debounce.borrow_mut() = Some(Timeout::new(300, {
                            let error_el = error_el.clone();
                            move || {
                                if text.len() >= 1 {
                                    match T::from_str(&text) {
                                        Err(e) => {
                                            error_el.ref_text(&e.to_string());
                                            return;
                                        },
                                        _ => { },
                                    }
                                }
                            }
                        }));
                    }
                });
        return (FormElements {
            error: Some(error_el.clone()),
            elements: vec![input_el.clone()],
        }, Box::new(FromStrFormState {
            el: input_el,
            error_el: error_el,
        }));
    }
}

impl<E: Display, T: FromStr<Err = E>> FormState<T> for FromStrFormState {
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
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, String>(field, from.as_ref().map(|x| x.as_str()).unwrap_or(""));
    }
}

impl<C> FormWith<C> for u8 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u16 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u32 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for u64 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i8 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i16 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i32 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for i64 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for f32 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}

impl<C> FormWith<C> for f64 {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        return FromStrFormState::new::<_, Self>(
            field,
            from.map(|x| x.to_string()).as_ref().map(|x| x.as_str()).unwrap_or(""),
        );
    }
}
