use {
    rooting::{
        el,
        El,
    },
};

/// Used for the text label before form fields.
pub const CSS_CLASS_LABEL: &'static str = "rf_label";

/// Used for single-column inputs (text entry, checkbox). Exclusive with other
/// `rf_input_` classes.
pub const CSS_CLASS_SMALL_INPUT: &'static str = "rf_input_small";

/// Used for two-column inputs like text area. Exclusive with other `rf_input_`
/// classes.
pub const CSS_CLASS_BIG_INPUT: &'static str = "rf_input_big";

/// Used by the checkbox for options. Exclusive with other `rf_input_` classes.
pub const CSS_CLASS_OPTION_ENABLE: &'static str = "rf_input_option";

/// Used for validation errors, appears before the input (also before the
/// associated label, if there is one).
pub const CSS_CLASS_ERROR: &'static str = "rf_error";

/// Used for nested struct/enum fields, namely within variants or options.
pub const CSS_CLASS_SUBFORM: &'static str = "rf_subform";
pub const CSS_CLASS_BUTTON_ICON: &'static str = "rf_button_icon";

/// Text input (div plus contenteditable, get contents with text_contents)
pub const CSS_CLASS_TEXT: &'static str = "rf_input_text";

/// Used to hide disabled variants - hidden to keep user input in case they
/// re-enable later.
pub const CSS_CLASS_HIDDEN: &'static str = "disable_hide";
pub const CSS_CLASS_VEC: &'static str = "rf_vec";
pub const CSS_CLASS_VEC_ITEMS: &'static str = "rf_vec_items";
pub const CSS_CLASS_VEC_ITEM_HEADER: &'static str = "rf_vec_item_header";
pub const CSS_CLASS_BUTTON_ICON_DELETE: &'static str = "rf_button_delete";
pub const CSS_CLASS_BUTTON_ICON_ADD: &'static str = "rf_button_add";
pub const CSS_CLASS_BUTTON_ICON_MOVE_DOWN: &'static str = "rf_button_move_up";
pub const CSS_CLASS_BUTTON_ICON_MOVE_UP: &'static str = "rf_button_move_down";

/// This should be used on all inputs, since `<label>` isn't used.
pub const ATTR_LABEL: &'static str = "aria-label";

pub fn css_class_depth(depth: usize) -> String {
    return format!("rf_depth_m7_{}", 1 + depth % 7);
}

pub fn err_el() -> El {
    return el("span").classes(&[CSS_CLASS_ERROR]);
}
