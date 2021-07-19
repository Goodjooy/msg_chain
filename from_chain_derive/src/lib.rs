use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Generics};
use syn::{DeriveInput, Type};

#[proc_macro_derive(LoadFormMap)]
pub fn msg_chain_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_from_chains_macro(&ast)
}

fn impl_from_chains_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let generics = &ast.generics;
    let (head_g, where_c) = load_generics(generics);

    let data = &ast.data;
    let (data, is_named) = load_data(data);
    let data = data.unwrap();

    let c_d = data.clone();
    let create_data = c_d.iter().map(|f| {
        let name = &f.0;
        let ty = &f.1;

        let (t,b)=load_type(ty);
        quote! {
            let #name  :#t = #b::from_chain(map.get(stringify!(#name))?)?;
        }
    });

    let set = data.iter().map(|f| &f.0);

    let new = if is_named {
        quote! {
            Self{
                #(#set),*
            }
        }
    } else {
        quote! {
            Self
        }
    };

    let gen = quote! {
        impl #head_g LoadFormMap  for #name #head_g #where_c {
            fn load_from_map(map: &HashMap<String, ChainMeta>) -> Option<Self> {
                let __ty = map.get("type")?;
                let s = String::from_chain(__ty)?;
                if s != stringify!(#name) {
                    return None;
                }
                

                #(#create_data)*

                Some(
                    #new
                )
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
    let sub_where = if where_limit.len() > 0 {
        quote! {
            where
            #( #where_limit),*
        }
    } else {
        quote! {}
    };

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

fn load_data(data: &Data) -> (Option<Vec<(syn::Ident, Type)>>, bool) {
    if let Data::Struct(st) = data {
        let fields = &st.fields;
        match fields {
            syn::Fields::Named(ns) => {
                let fields = &ns.named;
                let res = fields
                    .into_iter()
                    .map(|f| (&f.ident, &f.ty))
                    .filter(|predicate| if let None = predicate.0 { false } else { true })
                    .map(|f| (f.0.clone().unwrap(), f.1.clone()))
                    .collect::<Vec<_>>();
                (Some(res), true)
            }
            syn::Fields::Unnamed(_) => (None, false),
            syn::Fields::Unit => (Some(vec![]), false),
        }
    } else {
        (None, false)
    }
}


fn load_type(ty:&Type)->(quote::__private::TokenStream,quote::__private::TokenStream){
    let ty_def=quote! {#ty};
    if let Type::Path(p) = ty {
        let path=&p.path;
        let seg=&path.segments;
        let mut paths=Vec::new();

        for seg in seg.into_iter(){
            let ident=&seg.ident;
            paths.push(ident);
        }

        let paths=paths.iter();
        let base=quote! {#(#paths)::*};
        (ty_def,base)
    }else{
        (
            quote! {#ty},
            quote! {#ty}
        )
    }
    
}