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
let (creds_form, creds_form_state) = Creds::new_form("", None);
ok_button.ref_on("click", move |_| {
    let Ok(creds) = creds_form_state.parse() else {
        return;
    };
    do_login(creds);
});
let modal =
    el(
        "div",
    ).extend(vec![el("div").classes(&["form_grid"]).extend(creds_form.elements), ok_button]);
...
```

# Creating a form

Call `::new_form` on your form type. The first argument can be empty (it's used for aria hints for nested fields but not for top level structs). The second argument is `Some` if you want to pre-populate the form from existing data, for instance if you're editing something.

`new_form` returns a the form `rooting::El` elements and a state object.

Call `parse()` on the state object any number of times. It will either return the parsed object or an empty error. Any error messages will be automatically put in error elements near the relevant form fields. The error messages are cleared at the next `parse()`.
