use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(rooting_forms::Form)]
pub struct Alpha {
    #[title("A")]
    pub a: i32,
}

#[derive(rooting_forms::Form)]
pub enum Beta {
    #[title("A")]
    A,
    #[title("B")]
    B(i32),
    #[title("C")]
    C {
        #[title("Something")]
        nix: i32,
    },
}

#[derive(rooting_forms::Form)]
pub struct Gamma {
    #[title("G")]
    pub g: Rc<RefCell<i32>>,
}

#[derive(rooting_forms::Form)]
pub enum Delta {
    #[title("D")]
    G(Rc<RefCell<i32>>),
}
