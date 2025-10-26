use {
    crate::{
        css::{
            css_class_depth,
            ATTR_LABEL,
            CSS_CLASS_BUTTON_ICON,
            CSS_CLASS_BUTTON_ICON_ADD,
            CSS_CLASS_BUTTON_ICON_DELETE,
            CSS_CLASS_BUTTON_ICON_MOVE_DOWN,
            CSS_CLASS_BUTTON_ICON_MOVE_UP,
            CSS_CLASS_SUBFORM,
            CSS_CLASS_VEC,
            CSS_CLASS_VEC_ITEMS,
            CSS_CLASS_VEC_ITEM_HEADER,
        },
        FormElements,
        FormState,
        FormWith,
    },
    rooting::{
        el,
        Container,
        ContainerEntry,
        El,
    },
    std::{
        cell::{
            Cell,
            RefCell,
        },
        marker::PhantomData,
        rc::Rc,
    },
};

struct Item_<T> {
    root: El,
    add_index: Rc<Cell<usize>>,
    add: El,
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
}

impl<C, T: FormWith<C>> FormState<Vec<T>> for VecFormState<C, T> {
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

fn build_add<
    C: 'static + Clone,
    T: 'static + FormWith<C>,
>(
    context: &C,
    field: &str,
    items: &Rc<RefCell<Container<Item<C, T>>>>,
    index: Rc<Cell<usize>>,
    depth: usize,
) -> El {
    return el("button").classes(&[CSS_CLASS_BUTTON_ICON, CSS_CLASS_BUTTON_ICON_ADD]).on("click", {
        let field = field.to_string();
        let items = items.clone();
        let context = context.clone();
        move |_| {
            build_item::<C, T>(&context, &field, &items, None, index.get(), depth);
        }
    });
}

fn build_item<
    C: 'static + Clone,
    T: 'static + FormWith<C>,
>(
    context: &C,
    field: &str,
    items: &Rc<RefCell<rooting::Container<Item<C, T>>>>,
    from: Option<&T>,
    initial_index: usize,
    depth: usize,
) {
    let (item_elements, item_state) = T::new_form_with_(context, "Item", from, depth + 1);
    let subform = el("div").classes(&[CSS_CLASS_SUBFORM, &css_class_depth(depth)]);
    let add_index = Rc::new(Cell::new(initial_index));
    let add = build_add(context, field, items, add_index.clone(), depth);
    let item = Item(Rc::new(Item_ {
        root: el("div").push(add.clone()).push(subform.clone()),
        add_index: add_index,
        add: add,
        title: el("span"),
        state: item_state,
    }), Default::default());

    fn renumber<C, T: FormWith<C>>(field: &str, items: &mut rooting::Container<Item<C, T>>) {
        for (i, e) in items.iter().enumerate() {
            let add_help = format!("{} - Add new item", field);
            e.0.add.ref_attr(ATTR_LABEL, &add_help).ref_attr("title", &add_help);
            e.0.add_index.set(i);
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
            .classes(&[CSS_CLASS_BUTTON_ICON, CSS_CLASS_BUTTON_ICON_MOVE_UP])
            .attr(ATTR_LABEL, &format!("{} - Move item up", field))
            .on("click", {
                let field = field.to_string();
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
                    renumber(&field, &mut *items);
                }
            });
    let move_down =
        el("button")
            .classes(&[CSS_CLASS_BUTTON_ICON, CSS_CLASS_BUTTON_ICON_MOVE_DOWN])
            .attr(ATTR_LABEL, &format!("{} - Move item down", field))
            .on("click", {
                let field = field.to_string();
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
                    renumber(&field, &mut *items);
                }
            });
    let delete =
        el("button")
            .classes(&[CSS_CLASS_BUTTON_ICON, CSS_CLASS_BUTTON_ICON_DELETE])
            .attr(ATTR_LABEL, &format!("{} - Delete item", field))
            .on("click", {
                let field = field.to_string();
                let item = Rc::downgrade(&item.0);
                let items = items.clone();
                move |_| {
                    let Some(item) = item.upgrade() else {
                        return;
                    };
                    let mut items = items.as_ref().borrow_mut();
                    let i = index(&mut *items, &item);
                    items.remove(i);
                    renumber(&field, &mut *items);
                }
            });
    subform.ref_push(
        el("div")
            .classes(&[CSS_CLASS_VEC_ITEM_HEADER])
            .extend(vec![item.0.title.clone(), move_up, move_down, delete]),
    );
    if let Some(error) = item_elements.error {
        subform.ref_push(error);
    }
    subform.ref_extend(item_elements.elements);
    items.as_ref().borrow_mut().insert(initial_index, item);
    renumber(field, &mut *items.borrow_mut());
}

impl<C: 'static + Clone, T: FormWith<C> + 'static> FormWith<C> for Vec<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let items = Rc::new(RefCell::new(Container::new(el("div").classes(&[CSS_CLASS_VEC_ITEMS]))));
        if let Some(from) = from {
            for (i, v) in from.iter().enumerate() {
                build_item(context, field, &items, Some(v), i, depth);
            }
        }
        let elements =
            vec![
                items.as_ref().borrow().el().clone(),
                build_add(context, field, &items, Rc::new(Cell::new(items.borrow().len())), depth)
            ];
        return (crate::FormElements {
            error: None,
            elements: vec![el("div").classes(&[CSS_CLASS_VEC]).extend(elements)],
        }, Box::new(VecFormState { items: items }));
    }
}
