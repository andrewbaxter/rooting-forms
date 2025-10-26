use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use syn::{
    self,
    parse_macro_input,
    Ident,
    DeriveInput,
    Attribute,
    punctuated::Punctuated,
    Field,
};

fn parse_title(attrs: &Vec<Attribute>) -> Result<String, &'static str> {
    match attrs.iter().find_map(|a| a.parse_meta().ok().and_then(|m| match m {
        syn::Meta::List(m) if m.path.to_token_stream().to_string() == "title" => {
            Some(m.nested)
        },
        _ => None,
    })) {
        Some(m) => {
            if m.len() != 1 {
                return Err("#[title()] needs exactly one literal string argument");
            }
            match m.first().unwrap() {
                syn::NestedMeta::Lit(l) => {
                    match l {
                        syn::Lit::Str(l) => {
                            return Ok(
                                litrs::StringLit::parse(l.to_token_stream().to_string()).unwrap().value().to_string(),
                            );
                        },
                        _ => return Err("#[title()] needs exactly one literal string argument"),
                    }
                },
                _ => {
                    return Err("#[title()] argument must be a string literal");
                },
            }
        },
        None => return Err("Missing #[title(\"Field name\")]"),
    };
}

fn build_fields_form<
    X,
>(
    form_ident: &Ident,
    value_type_ident: &Ident,
    value_construct_ident: &TokenStream,
    fields: &Punctuated<Field, X>,
) -> TokenStream {
    let mut form_fields = vec![];
    let mut form_construct_fields = vec![];
    let mut form_elements = vec![];
    let mut form_parse = vec![];
    let mut form_parse_assemble = vec![];
    for f in fields {
        let f_ident = f.ident.as_ref().unwrap();
        let f_ident_elements = format_ident!("{}_elements", f_ident);
        let f_name = parse_title(&f.attrs).expect(&format!("Error with attributes on field {}", f_ident));
        let f_type_ident = f.ty.to_token_stream();
        form_fields.push(quote!{
            #f_ident: Box < dyn rooting_forms:: FormState < #f_type_ident >>,
        });
        form_elements.push(quote!{
            let(
                #f_ident_elements,
                #f_ident
            ) =< #f_type_ident >:: new_form_with_(context, #f_name, from.map(| from |& from.#f_ident), depth);
            elements.extend(#f_ident_elements.error.into_iter());
            elements.push(rooting:: el("span").classes(&[rooting_forms::css::CSS_CLASS_LABEL]).text(#f_name));
            elements.extend(#f_ident_elements.elements);
        });
        form_construct_fields.push(quote!{
            #f_ident: #f_ident,
        });
        form_parse.push(quote!{
            let #f_ident = match self.#f_ident.parse() {
                Ok(v) => Some(v),
                Err(_) => {
                    errored = true;
                    None
                }
            };
        });
        form_parse_assemble.push(quote!{
            #f_ident: #f_ident.unwrap(),
        });
    }
    return quote!{
        let mut elements = Vec::new();
        #(#form_elements) * #[allow(non_camel_case_types)] struct #form_ident {
            #(#form_fields) *
        }
        impl rooting_forms:: FormState < #value_type_ident > for #form_ident {
            fn parse(&self) -> Result < #value_type_ident,
            () > {
                let mut errored = false;
                #(#form_parse) * 
                //. .
                if errored {
                    return Err(());
                }
                return Ok(#value_construct_ident {
                    #(#form_parse_assemble) *
                });
            }
        }
        (elements, Box:: new(#form_ident {
            #(#form_construct_fields) *
        }))
    };
}

fn derive1(body: DeriveInput) -> TokenStream {
    let t_ident = &body.ident;
    match body.data {
        syn::Data::Struct(s) => {
            match s.fields {
                syn::Fields::Named(fields) => {
                    let form_build =
                        build_fields_form(
                            &format_ident!("FormStateImpl"),
                            &t_ident,
                            &t_ident.to_token_stream(),
                            &fields.named,
                        );
                    return quote!{
                        impl < C: 'static + Clone > rooting_forms:: FormWith < C > for #t_ident {
                            fn new_form_with_(
                                context: & C,
                                field: & str,
                                from: Option <& Self >,
                                depth: usize
                            ) ->(rooting_forms::FormElements, Box < dyn rooting_forms:: FormState < Self >>) {
                                #[allow(unused_imports)]
                                use rooting_forms::FormState;
                                #[allow(unused_imports)]
                                use std::str::FromStr;
                                #[allow(unused_imports)]
                                use wasm_bindgen::JsCast;
                                let(elements, state) = {
                                    #form_build
                                };
                                (rooting_forms::FormElements {
                                    error: None,
                                    elements: elements,
                                }, state)
                            }
                        }
                    };
                },
                syn::Fields::Unnamed(_) => panic!("Tuple structs aren't supported"),
                syn::Fields::Unit => panic!("Unit structs aren't supported"),
            }
        },
        syn::Data::Enum(e) => {
            let mut build_variants = vec![];
            let mut variant_to_int = vec![];
            for (i, v) in e.variants.iter().enumerate() {
                let v_ident = &v.ident;
                let v_name =
                    parse_title(&v.attrs).expect(&format!("Error with attributes on {}::{}", t_ident, v_ident));
                let v_value = format!("{}", i);
                let build_option = |ignore: TokenStream| -> TokenStream {
                    let default;
                    if i == 0 {
                        default = quote!(true);
                    } else {
                        default = quote!(false);
                    }
                    return quote!{
                        let option = rooting:: el("option").text(#v_name).attr("value", #v_value);
                        if from.map(| from | match from {
                            #t_ident:: #v_ident #ignore => true,
                            _ => false
                        }).unwrap_or(#default) {
                            option.ref_attr("selected", "selected");
                        }
                        select.ref_push(option);
                    };
                };
                match &v.fields {
                    syn::Fields::Named(fields) => {
                        variant_to_int.push(quote!{
                            #t_ident:: #v_ident {
                                ..
                            }
                            => #i,
                        });
                        let build_option = build_option(quote!({
                            ..
                        }));
                        let mut subform_from_fields_def = vec![];
                        let mut subform_from_fields_match = vec![];
                        let mut subform_from_fields_copy = vec![];
                        for f in &fields.named {
                            let f_ident = f.ident.as_ref().unwrap();
                            let f_type = f.ty.to_token_stream();
                            subform_from_fields_def.push(quote!{
                                #f_ident: #f_type,
                            });
                            subform_from_fields_match.push(quote!(#f_ident));
                            subform_from_fields_copy.push(quote!(#f_ident: #f_ident.clone()));
                        }
                        let subform_build =
                            build_fields_form(
                                &format_ident!("{}_{}_FormState", t_ident, v.ident),
                                &t_ident,
                                &quote!(#t_ident:: #v_ident),
                                &fields.named,
                            );
                        build_variants.push(quote!{
                            {
                                #build_option 
                                //. .
                                variant_parse.push(rooting_forms:: impl_macroutil:: LazySubform:: new({
                                    struct FromTemp {
                                        #(#subform_from_fields_def) *
                                    }
                                    let from = from.and_then(| from | match from {
                                        #t_ident:: #v_ident {
                                            #(#subform_from_fields_match),
                                            *
                                        }
                                        => Some(FromTemp {
                                            #(#subform_from_fields_copy),
                                            *
                                        }),
                                        _ => None,
                                    });
                                    let context = context.clone();
                                    let depth = depth + 1;
                                    move || {
                                        let context = &context;
                                        let from = from.as_ref();
                                        let(elements, state) = {
                                            #subform_build
                                        };
                                        (rooting_forms::FormElements {
                                            error: None,
                                            elements: elements,
                                        }, state)
                                    }
                                }));
                            }
                        });
                    },
                    syn::Fields::Unnamed(fields) => {
                        if fields.unnamed.len() != 1 {
                            panic!(
                                "Only single field tuple enum variants are supported currently ({}::{})",
                                t_ident,
                                v_ident
                            );
                        }
                        variant_to_int.push(quote!{
                            #t_ident:: #v_ident(_) => #i,
                        });
                        let build_option = build_option(quote!((_)));
                        let f = fields.unnamed.first().unwrap();
                        let f_type_ident = f.ty.to_token_stream();
                        build_variants.push(quote!{
                            {
                                #build_option 
                                //. .
                                variant_parse.push(rooting_forms:: impl_macroutil:: LazySubform:: new({
                                    let from = from.and_then(| from | match from {
                                        #t_ident:: #v_ident(from) => Some(from.clone()),
                                        _ => None,
                                    });
                                    let context = context.clone();
                                    move || {
                                        let context = &context;
                                        let(
                                            elements,
                                            state
                                        ) =< #f_type_ident >:: new_form_with_(
                                            context,
                                            #v_name,
                                            from.as_ref(),
                                            depth + 1
                                        );
                                        return(
                                            elements,
                                            Box:: new(
                                                rooting_forms:: impl_macroutil:: VariantWrapFormState::< C,
                                                _,
                                                _ >:: new(state, | x | #t_ident:: #v_ident(x))
                                            )
                                        );
                                    }
                                }));
                            }
                        });
                    },
                    syn::Fields::Unit => {
                        variant_to_int.push(quote!{
                            #t_ident:: #v_ident => #i,
                        });
                        let build_option = build_option(quote!());
                        build_variants.push(quote!{
                            {
                                #build_option 
                                //. .
                                variant_parse.push(rooting_forms:: impl_macroutil:: LazySubform:: new({
                                    || {
                                        (
                                            rooting_forms::FormElements {
                                                error: None,
                                                elements: vec![],
                                            },
                                            Box:: new(
                                                rooting_forms:: impl_macroutil:: VariantUnitFormState::< C,
                                                _ >:: new(|| #t_ident:: #v_ident,)
                                            )
                                        )
                                    }
                                }));
                            }
                        });
                    },
                }
            }
            return quote!{
                impl < C: 'static + Clone > rooting_forms:: FormWith < C > for #t_ident {
                    fn new_form_with_(
                        context: & C,
                        field: & str,
                        from: Option <& Self >,
                        depth: usize
                    ) ->(rooting_forms::FormElements, Box < dyn rooting_forms:: FormState < Self >>) {
                        #[allow(unused_imports)]
                        use rooting_forms::FormState;
                        #[allow(unused_imports)]
                        use std::str::FromStr;
                        #[allow(unused_imports)]
                        use wasm_bindgen::JsCast;
                        struct FormStateImpl < C: 'static + Clone,
                        T: rooting_forms:: FormWith < C >> {
                            variant_parse: Vec < rooting_forms:: impl_macroutil:: LazySubform < C,
                            T >>,
                            current_variant: std:: rc:: Rc < std:: cell:: Cell < usize >>
                        }
                        impl < C: 'static + Clone,
                        T: rooting_forms:: FormWith < C >> rooting_forms:: FormState < T > for FormStateImpl < C,
                        T > {
                            fn parse(&self) -> Result<T, ()> {
                                return self.variant_parse[self.current_variant.get()].parse();
                            }
                        }
                        let variant = std:: rc:: Rc:: new(std:: cell:: Cell:: new(match from {
                            Some(v) => match v {
                                #(#variant_to_int) *
                            },
                            None => 0,
                        }));
                        let mut elements = vec![];
                        let select =
                            rooting::el("select")
                                .classes(&[rooting_forms::css::CSS_CLASS_SMALL_INPUT])
                                .attr(rooting_forms::css::ATTR_LABEL, field);
                        elements.push(select.clone());
                        let mut variant_parse: Vec < rooting_forms:: impl_macroutil:: LazySubform < C,
                        #t_ident >> 
                        //. .
                        = vec ![];
                        #(#build_variants) * 
                        //. .
                        let subform = rooting:: el(
                            "div"
                        ).classes(
                            &[rooting_forms::css::CSS_CLASS_SUBFORM, &rooting_forms::css::css_class_depth(depth)]
                        ).extend(variant_parse[variant.get()].elements());
                        select.ref_on("change", {
                            let variant_parse = variant_parse.clone();
                            let variant = variant.clone();
                            let subform = subform.clone();
                            move |event| {
                                let index =
                                    usize::from_str(
                                        &event
                                            .target()
                                            .unwrap()
                                            .dyn_into::<rooting_forms::republish::HtmlSelectElement>()
                                            .unwrap()
                                            .value(),
                                    ).unwrap();
                                variant.set(index);
                                subform.ref_clear().ref_extend(variant_parse[variant.get()].elements());
                            }
                        });
                        return (rooting_forms::FormElements {
                            error: None,
                            elements: vec![select, subform],
                        }, Box::new(FormStateImpl {
                            variant_parse: variant_parse,
                            current_variant: variant,
                        }));
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union types unsupported"),
    };
}

#[proc_macro_derive(Form, attributes(title))]
pub fn derive(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(body as syn::DeriveInput);
    return derive1(ast).into();
}
