This provides traits and a derive macro for generating rooting HTML forms from structs and enums.

**Example**

```rust
#[derive(rooting_forms::Form)]
struct Creds {
    #[title("Your username")]
    username: String,
    #[title("Your password")]
    password: rooting_forms::Password,
}

...
let ok_button = el("button").text("Ok");
let creds_form_state = Creds::new_form("", None);
ok_button.ref_on("click", move |_| {
    let Ok(creds) = creds_form_state.parse() else {
        return;
    };
    do_login(creds);
});
let modal =
    el(
        "div",
    ).extend(vec![el("div").classes(&["form_grid"]).extend(creds_form_state.elements().elements), ok_button]);
...
```

# Creating a form

Call `::new_form` on your form type. The first argument can be empty (it's used for aria hints for nested fields). The second argument is `Some` if you want to pre-populate the form from existing data, for instance if you're editing something.

`new_form` returns a state object.

1. Call `elements()` on the state to get the form elements and add them to the page.
2. Call `parse()` any number of times. It will either return a parsed object or an empty error. Any error messages will be automatically put in error elements near the relevant form fields. The error messages are cleared at the next `parse()`.

# Styling

`elements` above will be a list of (by CSS selector):

- `.form_label` - input labels
- `.form_input_small` - single column inputs like single line entry, checkboxes, dropdowns
- `.form_input_big` - multi column inputs like textareas
- `.form_input_option` - a special case, the checkbox for optional elements
- `.form_error` - an element containing validation error text. This is always visible, but the text may be empty
- `.subform` - for nested forms, namely within variants
- `.disable_hide` - for inactive form elements (ex: controls for a variant that's not selected)

I imagine you'll place these in a grid, with the labels in column 1, option checkboxes in column 2, small inputs in column 3, and big inputs/subforms spanning all columns.
