use proc_macro2::TokenStream;
use syn::Expr;
use syn_rsx::{parse, Node, NodeType};

#[proc_macro]
pub fn view(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            if let Some(node) = nodes.get(0).and_then(node_to_tokens) {
                node
            } else {
                quote::quote! {}
            }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}

fn node_to_tokens(node: &Node) -> Option<TokenStream> {
    match node.node_type {
        NodeType::Element => {
            let tag = node.name_as_string();
            let span = node.name_span().unwrap();

            if tag.is_some() {
                element_node(node)
            } else {
                Some(quote::quote_spanned! {
                    span => compile_error!("blocks in tag name position are not supported")
                })
            }
        }
        NodeType::Text | NodeType::Block => {
            let value = node.value.as_ref().unwrap();

            Some(quote::quote! {
                #value
            })
        }
        NodeType::Fragment => {
            let children = node.children.iter().filter_map(node_to_tokens);
            Some(quote::quote! {
                vec![
                    #(#children,)*
                ]
            })
        }
        NodeType::Comment => None,
        NodeType::Doctype => None,
        NodeType::Attribute => None,
    }
}

fn element_node(node: &Node) -> Option<TokenStream> {
    let listeners = listeners_from_node(node, "on:");

    let classes = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let attr_name = attr.name_as_string().unwrap();
            if attr_name.starts_with("class:") {
                let class_name = attr_name.replace("class:", "");
                let value = attr.value.as_ref();
                let span = attr.name_span().unwrap();
                Some(
                    value
                        .map({
                            let class_name = class_name.clone();
                            move |value| {
                                quote::quote_spanned! {
                                    span => leptos2::Attribute::Class(#class_name.to_string(), #value)
                                }
                            }
                        })
                        .unwrap_or_else(|| {
                            quote::quote_spanned! {
                                span => leptos2::Attribute::Class(#class_name.to_string(), true)
                            }
                        }),
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let attributes = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let attr_name = attr.name_as_string().unwrap();
            if !attr_name.starts_with("class:")
                && !attr_name.starts_with("on:")
                && !attr_name.starts_with("window:")
                && !attr_name.starts_with("document:")
                && !attr_name.starts_with("foreign:")
                && attr_name != "key"
            {
                let real_attr_name = if attr_name.starts_with("prop:") {
                    attr_name.replace("prop:", "")
                } else {
                    attr_name.to_string()
                };
                let value = attr.value.as_ref();
                let span = attr.name_span().unwrap();
                value.map(|value| {
                    if attr_name.starts_with("prop:") {
                        quote::quote_spanned! {
                            span => leptos2::property(#real_attr_name, #value)
                        }
                    } else {
                        quote::quote_spanned! {
                            span => leptos2::attribute(#real_attr_name, #value)
                        }
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let attributes = quote::quote! {
        vec![
             #(#attributes,)*
             #(#classes,)*
        ]
    };

    let tag = node.name_as_string().unwrap();
    let span = node.name_span().unwrap();

    let children = node
        .children
        .iter()
        .filter_map(node_to_tokens)
        .collect::<Vec<_>>();

    if tag == "Host" {
        let window_listeners = listeners_from_node(node, "window:");
        let document_listeners = listeners_from_node(node, "document:");
        let foreign_listeners = foreign_listeners_from_node(node);

        Some(quote::quote_spanned! {
            span => leptos2::Host {
                tag: Self::tag(),
                host_attrs: #attributes,
                state_attrs: self.state_to_attributes(),
                listeners: #listeners,
                window_listeners: #window_listeners,
                document_listeners: #document_listeners,
                foreign_listeners: #foreign_listeners,
                children: vec![]
            }
            #(.child(#children))*
        })
    } else {
        // can refer to components by their Rust type and automatically insert the Custom Element tag
        let tag = if tag.chars().next().unwrap().is_ascii_uppercase() && tag != "Host" {
            let element_name: TokenStream = tag.parse().unwrap();
            quote::quote! { #element_name::tag() }
        } else {
            quote::quote! { #tag.to_string() }
        };

        let key = if let Some(key) = node
            .attributes
            .iter()
            .find(|attr| attr.name_as_string() == Some("key".to_string()))
        {
            let span = key.name_span().unwrap();
            let value = key.value.as_ref().unwrap();
            quote::quote_spanned! {
                span => Some(#value.to_string())
            }
        } else {
            quote::quote! { None }
        };

        Some(quote::quote_spanned! {
            span => leptos2::Node::Element(leptos2::Element {
                tag: #tag.to_string(),
                key: #key,
                attrs: #attributes,
                listeners: #listeners,
                children: vec![]
            }
            #(.child(#children))*)
        })
    }
}

fn listeners_from_node(node: &Node, starting_phrase: &str) -> proc_macro2::TokenStream {
    let listeners = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let attr_name = attr.name_as_string().unwrap();
            if attr_name.starts_with(starting_phrase) {
                let event_name = attr_name.replace(starting_phrase, "");
                let handler = attr.value.as_ref();
                let span = attr.name_span().unwrap();
                handler.map(|handler| {
                    quote::quote_spanned! {
                        span => (#event_name, std::rc::Rc::new(#handler) as std::rc::Rc<dyn Fn(_) -> _>)
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if listeners.is_empty() {
        quote::quote! { Vec::new() }
    } else {
        quote::quote! {
            leptos2::vdom::build_listeners(leptos2::vdom::Listeners::from([
                #(#listeners,)*
            ]))
        }
    }
}

fn foreign_listeners_from_node(node: &Node) -> proc_macro2::TokenStream {
    let listeners = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let span = attr.name_span().unwrap();
            let attr_name = attr.name_as_string().unwrap();
            if attr_name.starts_with("foreign:") {
                let event_name = attr_name.replace("foreign:", "");
                let (selector, handler) = match attr.value.as_ref().unwrap() {
                    Expr::Tuple(tuple) => {
                        let selector = tuple.elems.first();
                        let map_fn = tuple.elems.last();
                        (selector.unwrap(), map_fn.unwrap())
                    }
                    _ => {
                        panic!("expected a tuple with selector and handler")
                    }
                };
                Some(quote::quote_spanned! {
                    span => (
                        #event_name.to_string(),
                        #selector.to_string(),
                        std::rc::Rc::new(#handler) as std::rc::Rc<dyn Fn(_) -> _>
                    )
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if listeners.is_empty() {
        quote::quote! { Vec::new() }
    } else {
        quote::quote! {
            leptos2::vdom::build_foreign_listeners(vec![
                #(#listeners,)*
            ])
        }
    }
}

// Derive WebComponent trait for any struct
#[proc_macro_derive(WebComponent, attributes(prop))]
pub fn web_component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_wc(&ast)
}

fn impl_wc(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let tag = format!("l-{}", slugify(&name.to_string()));

    let attrs = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
            .named
            .iter()
            .filter(|field| matches!(field.vis, syn::Visibility::Public(_)))
            .collect()
    } else {
        vec![]
    };

    let observed_attributes = attrs
        .iter()
        .filter(|field| {
            !field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| slugify(&field.ident.as_ref().unwrap().to_string()));

    let observed_properties = attrs
        .iter()
        .filter(|field| {
            field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| slugify(&field.ident.as_ref().unwrap().to_string()));

    let set_attribute = attrs
        .iter()
        .filter(|field| {
            !field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| {
            let attr_name = slugify(&field.ident.as_ref().unwrap().to_string());
            let ident = &field.ident;
            let ty = &field.ty;

            quote::quote! {
                #attr_name => {
                    if let Some(#ident) = new_value.and_then(|value| value.parse::<#ty>().ok()) {
                        self.#ident = #ident;
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let set_attribute = if set_attribute.is_empty() {
        quote::quote! { {} }
    } else {
        quote::quote! {
            match attr_name.as_str() {
                #(#set_attribute,)*
                _ => {}
            }
        }
    };

    let set_property = attrs
        .iter()
        .filter(|field| {
            field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| {
            let attr_name = slugify(&field.ident.as_ref().unwrap().to_string());
            let ident = &field.ident;
            let ty = &field.ty;

            quote::quote! {
                #attr_name => {
                    match serde_wasm_bindgen::from_value::<#ty>(new_value) {
                        Ok(val) => {
                            self.#ident = val;
                        },
                        Err(e) => leptos2::warn(&format!("[set_property] deserialization error {}", e.to_string()))
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let set_property = if set_property.is_empty() {
        quote::quote! { {} }
    } else {
        quote::quote! {
            match prop_name.as_str() {
                #(#set_property,)*
                _ => {}
            }
        }
    };

    let state_to_attributes = attrs.iter().map(|field| {
        let attr_name = slugify(&field.ident.as_ref().unwrap().to_string());
        let ident = &field.ident.as_ref().unwrap();

        let prop = field.attrs.iter().any(|attr| {
            attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
        });

        if prop {
            quote::quote! {
                leptos2::property(
                    #attr_name,
                    Box::new(self.#ident.clone())
                )
            }
        } else {
            quote::quote! {
                leptos2::attribute(
                    #attr_name,
                    self.#ident.to_string()
                )
            }
        }
    });

    let gen = quote::quote! {
        impl WebComponent for #name {
            fn tag() -> &'static str {
                #tag
            }

            fn attributes() -> &'static [&'static str] {
               &[#(#observed_attributes,)*]
            }

            fn properties() -> &'static [&'static str] {
               &[#(#observed_properties,)*]
            }

            fn set_attribute(&mut self, attr_name: String, new_value: Option<String>) {
                #set_attribute
            }

            fn set_property(&mut self, prop_name: String, new_value: wasm_bindgen::JsValue) {
                #set_property
            }

            fn state_to_attributes(&self) -> Vec<Attribute> {
                vec![#(#state_to_attributes,)*]
            }
        }
    };
    gen.into()
}

const UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn slugify(unslugged: &str) -> String {
    let mut splits = unslugged.split_inclusive(UPPERCASE).enumerate().peekable();

    let mut buffer = String::new();

    while let Some((i, part)) = splits.next() {
        let is_first = i == 0;
        let len = part.len();

        if is_first {
            buffer.push_str(&part.to_ascii_lowercase())
        } else {
            let is_last = splits.peek().is_none();
            for (j, ch) in part.chars().enumerate() {
                if j == len - 1 && (!is_last || ch.is_ascii_uppercase()) {
                    buffer.push('-');
                    buffer.push(ch.to_ascii_lowercase());
                } else {
                    buffer.push(ch.to_ascii_lowercase());
                }
            }
        }
    }
    buffer.replace('_', "-")
}