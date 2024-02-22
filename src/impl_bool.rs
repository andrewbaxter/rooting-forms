use rooting::{
    El,
    el,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::{
    css::{
        ATTR_LABEL,
        CSS_CLASS_SMALL_INPUT,
    },
    FormElements,
    FormState,
    FormWith,
};

struct BoolFormState {
    input: El,
}

impl FormState<bool> for BoolFormState {
    fn parse(&self) -> Result<bool, ()> {
        return Ok(self.input.raw().dyn_ref::<HtmlInputElement>().unwrap().checked());
    }
}

impl<C> FormWith<C> for bool {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let e = el("input").classes(&[CSS_CLASS_SMALL_INPUT]).attr(ATTR_LABEL, field).attr("type", "checkbox");
        if from.cloned().unwrap_or(false) {
            e.ref_attr("checked", "checked");
        }
        return (FormElements {
            error: None,
            elements: vec![e.clone()],
        }, Box::new(BoolFormState { input: e }));
    }
}
