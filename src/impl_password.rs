use {
    rooting::{
        El,
        el,
    },
    wasm_bindgen::JsCast,
    web_sys::HtmlInputElement,
    crate::{
        css::{
            ATTR_LABEL,
            CSS_CLASS_SMALL_INPUT,
        },
        FormWith,
        FormElements,
        FormState,
    },
};

/// A minimal string wrapper that creates a password form input.
pub struct Password(pub String);

/// A helper form type for rust types that implement `FromStr`.
pub struct PasswordFormState {
    pub el: El,
}

impl FormState<Password> for PasswordFormState {
    fn parse(&self) -> Result<Password, ()> {
        return Ok(Password(self.el.raw().dyn_ref::<HtmlInputElement>().unwrap().value()));
    }
}

impl<C> FormWith<C> for Password {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let input_el =
            el("input")
                .classes(&[CSS_CLASS_SMALL_INPUT])
                .attr(ATTR_LABEL, field)
                .attr("type", "password")
                .attr("value", match from {
                    Some(x) => &x.0,
                    None => "",
                });
        return (FormElements {
            error: None,
            elements: vec![input_el.clone()],
        }, Box::new(PasswordFormState { el: input_el }));
    }
}
