use std::vec;

use proc_macro::TokenStream;

use quote::quote;
use syn::Data;
use syn::{DeriveInput, Generics};

#[proc_macro_derive(MessageChain)]
pub fn msg_chain_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_msg_chains_macro(&ast)
}

fn impl_msg_chains_macro(ast: &DeriveInput) -> TokenStream {
    //type get
    let chain_type = &ast.ident;

    //type used gerace
    let generics = &ast.generics;
    let (impl_g, where_c) = load_generics(generics);

    //load inside items
    let items = &ast.data;
    let datas = load_data(items).unwrap();
    let datas = datas.iter();
    let match_data = datas.clone().map(|f| {
        quote! {
            stringify!(#f)=>Some(self.#f.into_chain())
        }
    });

    let match_data=if match_data.len()>0{
        quote! {
            match key {
                #( #match_data ),*
                ,
                _=>None
            }
        }
    }else{
        quote! {
            None
        }
    };

    let gen = quote! {

        impl #impl_g MessageChain for #chain_type #impl_g #where_c {
            fn get_type(&self) -> &'static str{
                stringify!(#chain_type)
            }
            fn get(&self, key: &str) -> Option<ChainMeta> {
                #match_data
            }
            fn all_keys(&self) -> Vec<&'static str> {
                vec![
                    # ( stringify!(#datas)),*
                ]
            }
        }
    };
    gen.into()
}

fn load_generics(g: &Generics) -> (quote::__private::TokenStream, quote::__private::TokenStream) {
    //can set to where
    let type_params = g.type_params();
    let lifetimes = g.lifetimes();
    //can not set to where
    let const_params = g
        .const_params()
        .into_iter()
        .map(|f| quote! {#f})
        .collect::<Vec<_>>();

    let where_clause = g.where_clause.clone();
    let mut used = Vec::new();
    let mut limits = Vec::new();
    // load life time
    for lifetime in lifetimes.into_iter() {
        let life = &lifetime.lifetime;
        let has_limit = &lifetime.bounds.len() > &0;
        used.push(quote! {#life});

        let t = if has_limit {
            quote! {
                #lifetime
            }
        } else {
            quote! {}
        };
        limits.push(t);
    }
    //load type params
    for type_param in type_params.into_iter() {
        let base = &type_param.ident;
        let has_limits = type_param.bounds.len() > 0;
        used.push(quote! {#base});
        let t = if has_limits {
            quote! {
                #type_param
            }
        } else {
            quote! {}
        };
        limits.push(t);
    }

    let mut where_limit = Vec::new();
    // genreate where
    if let Some(where_clause) = where_clause {
        for wh in where_clause.predicates {
            where_limit.push(quote! {
                #wh
            })
        }
    }

    for limit in limits {
        where_limit.push(limit);
    }
    let where_limit = where_limit.iter();
    let sub_where = if where_limit.len()>0{ quote! {
        where
        #( #where_limit),*
    }}else { quote! {}};

    let g_useds = used.iter();
    // use gereric
    let g = quote! {
        <
            # ( #g_useds ),*
            #( #const_params),*
        >
    };

    (g, sub_where)
}

fn load_data(data: &Data) -> Option<Vec<syn::Ident>> {
    if let Data::Struct(st) = data {
        let fields = &st.fields;
        match fields {
            syn::Fields::Named(ns) => {
                let fields = &ns.named;
                let res = fields
                    .into_iter()
                    .map(|f| &f.ident)
                    .filter(|predicate| if let None = predicate { false } else { true })
                    .map(|f| f.clone().unwrap())
                    .collect::<Vec<_>>();
                Some(res)
            }
            syn::Fields::Unnamed(_) => None,
            syn::Fields::Unit => Some(vec![]),
        }
    } else {
        None
    }
}
