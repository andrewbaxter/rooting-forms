use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Rc,
};
use rooting::{
    el,
    Container,
    ContainerEntry,
    El,
};
use crate::{
    FormWith,
    FormState,
    ATTR_LABEL,
    CSS_CLASS_VEC,
    CSS_CLASS_VEC_ADD,
    CSS_CLASS_VEC_DELETE,
    CSS_CLASS_VEC_ITEM,
    CSS_CLASS_VEC_ITEM_HEADER,
    CSS_CLASS_VEC_MOVE_DOWN,
    CSS_CLASS_VEC_MOVE_UP,
};

struct Item_<T> {
    root: El,
    title: El,
    state: Box<dyn FormState<T>>,
}

struct Item<C, T>(Rc<Item_<T>>, PhantomData<C>);

impl<C, T> Clone for Item<C, T> {
    fn clone(&self) -> Self {
        return Self(self.0.clone(), Default::default());
    }
}

impl<C, T: FormWith<C>> ContainerEntry for Item<C, T> {
    fn el(&self) -> &El {
        return &self.0.root;
    }
}

struct VecFormState<C, T: FormWith<C>> {
    items: Rc<RefCell<Container<Item<C, T>>>>,
    elements: Vec<El>,
}

impl<C, T: FormWith<C>> FormState<Vec<T>> for VecFormState<C, T> {
    fn elements(&self) -> crate::FormElements {
        return crate::FormElements {
            error: None,
            elements: self.elements.clone(),
        };
    }

    fn parse(&self) -> Result<Vec<T>, ()> {
        let mut out = vec![];
        let mut ok = true;
        for i in &*self.items.borrow() {
            let Ok(v) = i.0.state.parse() else {
                ok = false;
                continue;
            };
            out.push(v);
        }
        if !ok {
            return Err(());
        }
        return Ok(out);
    }
}

fn build_item<
    C: 'static,
    T: FormWith<C> + 'static,
>(context: &C, field: &str, items: &Rc<RefCell<rooting::Container<Item<C, T>>>>, from: Option<&T>) {
    let item = Item(Rc::new(Item_ {
        root: el("div").classes(&[CSS_CLASS_VEC_ITEM]),
        title: el("span"),
        state: T::new_form(context, "Item", from),
    }), Default::default());

    fn renumber<C, T: FormWith<C>>(items: &mut rooting::Container<Item<C, T>>) {
        for (i, e) in items.iter().enumerate() {
            e.0.title.ref_text(&format!("Item {}", i + 1));
        }
    }

    fn index<C, T: FormWith<C>>(items: &mut rooting::Container<Item<C, T>>, item: &Rc<Item_<T>>) -> usize {
        return items.iter().enumerate().find_map(|(i, x)| if Rc::ptr_eq(&x.0, item) {
            return Some(i);
        } else {
            return None;
        }).unwrap();
    }

    let move_up =
        el("button")
            .classes(&[CSS_CLASS_VEC_MOVE_UP])
            .attr(ATTR_LABEL, &format!("{} - Move item up", field))
            .on("click", {
                let item = Rc::downgrade(&item.0);
                let items = items.clone();
                move |_| {
                    let Some(item) = item.upgrade() else {
                        return;
                    };
                    let mut items = items.as_ref().borrow_mut();
                    let i = index(&mut *items, &item);
                    if i == 0 {
                        return;
                    }
                    let item = items.remove(i);
                    items.insert(i - 1, item);
                    renumber(&mut *items);
                }
            });
    let move_down =
        el("button")
            .classes(&[CSS_CLASS_VEC_MOVE_DOWN])
            .attr(ATTR_LABEL, &format!("{} - Move item down", field))
            .on("click", {
                let item = Rc::downgrade(&item.0);
                let items = items.clone();
                move |_| {
                    let Some(item) = item.upgrade() else {
                        return;
                    };
                    let mut items = items.as_ref().borrow_mut();
                    let i = index(&mut *items, &item);
                    if i + 1 >= items.len() {
                        return;
                    }
                    let item = items.remove(i);
                    items.insert(i + 1, item);
                    renumber(&mut *items);
                }
            });
    let delete =
        el("button")
            .classes(&[CSS_CLASS_VEC_DELETE])
            .attr(ATTR_LABEL, &format!("{} - Delete item", field))
            .on("click", {
                let item = Rc::downgrade(&item.0);
                let items = items.clone();
                move |_| {
                    let Some(item) = item.upgrade() else {
                        return;
                    };
                    let mut items = items.as_ref().borrow_mut();
                    let i = index(&mut *items, &item);
                    items.remove(i);
                    renumber(&mut *items);
                }
            });
    let elements = item.0.state.elements();
    item
        .0
        .root
        .ref_push(
            el("div")
                .classes(&[CSS_CLASS_VEC_ITEM_HEADER])
                .extend(vec![item.0.title.clone(), move_up, move_down, delete]),
        );
    if let Some(error) = elements.error {
        item.0.root.ref_push(error);
    }
    item.0.root.ref_extend(elements.elements);
    items.as_ref().borrow_mut().push(item);
}

impl<C: 'static + Clone, T: FormWith<C> + 'static> FormWith<C> for Vec<T> {
    fn new_form(context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        let items = Rc::new(RefCell::new(Container::new(el("div").classes(&[CSS_CLASS_VEC]))));
        if let Some(from) = from {
            for v in from {
                build_item(context, field, &items, Some(v));
            }
        }
        let add =
            el("button")
                .classes(&[CSS_CLASS_VEC_ADD])
                .attr(ATTR_LABEL, &format!("{} - Add new item", field))
                .on("click", {
                    let field = field.to_string();
                    let items = items.clone();
                    let context = context.clone();
                    move |_| {
                        build_item(&context, &field, &items, None);
                    }
                });
        let elements = vec![items.as_ref().borrow().el().clone(), add];
        return Box::new(VecFormState {
            elements: elements,
            items: items,
        });
    }
}
