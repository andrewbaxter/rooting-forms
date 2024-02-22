/// Used for the text label before form fields.
pub const CSS_CLASS_LABEL: &'static str = "form_label";

/// Used for single-column inputs (text entry, checkbox). Exclusive with other
/// `form_input_` classes.
pub const CSS_CLASS_SMALL_INPUT: &'static str = "form_input_small";

/// Used for two-column inputs like text area. Exclusive with other `form_input_`
/// classes.
pub const CSS_CLASS_BIG_INPUT: &'static str = "form_input_big";

/// Used by the checkbox for options. Exclusive with other `form_input_` classes.
pub const CSS_CLASS_OPTION_ENABLE: &'static str = "form_input_option";

/// Used for validation errors, appears before the input (also before the
/// associated label, if there is one).
pub const CSS_CLASS_ERROR: &'static str = "form_error";

/// Used for nested struct/enum fields, namely within variants or options.
pub const CSS_CLASS_SUBFORM: &'static str = "form_subform";
pub const CSS_CLASS_BUTTON_ICON: &'static str = "form_button_icon";

/// Used to hide disabled variants - hidden to keep user input in case they
/// re-enable later.
pub const CSS_CLASS_HIDDEN: &'static str = "disable_hide";
pub const CSS_CLASS_VEC: &'static str = "form_vec";
pub const CSS_CLASS_VEC_ITEMS: &'static str = "form_vec_items";
pub const CSS_CLASS_VEC_ITEM_HEADER: &'static str = "form_vec_item_header";
pub const CSS_CLASS_BUTTON_ICON_DELETE: &'static str = "form_button_delete";
pub const CSS_CLASS_BUTTON_ICON_ADD: &'static str = "form_button_add";
pub const CSS_CLASS_BUTTON_ICON_MOVE_DOWN: &'static str = "form_button_move_up";
pub const CSS_CLASS_BUTTON_ICON_MOVE_UP: &'static str = "form_button_move_down";

/// This should be used on all inputs, since `<label>` isn't used.
pub const ATTR_LABEL: &'static str = "aria-label";

pub fn css_class_depth(depth: usize) -> String {
    return format!("form_depth_m7_{}", 1 + depth % 7);
}
