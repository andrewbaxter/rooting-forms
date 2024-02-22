use std::marker::PhantomData;
use rooting::{
    El,
    el,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::{
    css::{
        ATTR_LABEL,
        CSS_CLASS_HIDDEN,
        CSS_CLASS_OPTION_ENABLE,
    },
    FormElements,
    FormState,
    FormWith,
};

struct OptionFormState<C, T> {
    checked_el: El,
    subform: Box<dyn FormState<T>>,
    _pd: PhantomData<C>,
}

impl<C, T: FormWith<C>> FormState<Option<T>> for OptionFormState<C, T> {
    fn parse(&self) -> Result<Option<T>, ()> {
        let checked = self.checked_el.raw().dyn_ref::<HtmlInputElement>().unwrap().checked();
        if checked {
            return Ok(Some(self.subform.parse()?));
        } else {
            return Ok(None);
        }
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Option<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let (subform_elements, subform) = T::new_form_with_(context, field, from.and_then(|x| x.as_ref()), depth);
        let mut additional = vec![];
        additional.extend(subform_elements.error.iter().cloned());
        additional.extend(subform_elements.elements);
        let check = el("input")
            //. .
            .classes(&[CSS_CLASS_OPTION_ENABLE])
            .attr(ATTR_LABEL, &format!("{} - Enabled", field))
            .attr("type", "checkbox")
            .on("click", {
                let additional = additional.clone();
                move |ev| {
                    let checked = ev.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().checked();
                    for e in &additional {
                        e.ref_modify_classes(&[(CSS_CLASS_HIDDEN, checked)]);
                    }
                }
            });
        if from.map(|x| x.is_some()).unwrap_or(false) {
            check.ref_attr("checked", "checked");
        } else {
            for e in &additional {
                e.ref_modify_classes(&[(CSS_CLASS_HIDDEN, true)]);
            }
        }
        let mut elements = vec![check.clone()];
        elements.extend(additional);
        return (FormElements {
            error: None,
            elements: elements,
        }, Box::new(OptionFormState {
            checked_el: check,
            subform: subform,
            _pd: Default::default(),
        }));
    }
}
