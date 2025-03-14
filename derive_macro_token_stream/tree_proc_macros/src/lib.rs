extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Attribute, Meta, NestedMeta, Lit};
use quote::quote;
use proc_macro2::Ident;
use poseidon_rs::{Fr, Poseidon};
use merkle_tree_storage::MerkleTreeStorage;



#[proc_macro_derive(MerkleTree, attributes(tree, tree_arg))]
pub fn derive_merkle_tree(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // let tree_attr = input.attrs.iter().find(|attr| attr.path.is_ident("tree"));
    // let mut _depth = 0;
    // let mut _storage_name = Ident::new("TreeStorage", proc_macro2::Span::call_site());
    // if let Some(attr) = tree_attr {
    //     _depth = attr.parse_args::<().unwrap() as u32;
    //     _storage_name = attr.parse_args::<Ident>().unwrap();
    // }

    let attrs = &input.attrs;
    let mut _depth = None;
    let mut _storage = None;

    for attr in attrs {
        // Check if the attribute is `tree`
        if attr.path.is_ident("tree") {
            // Parse the attribute as a Meta item
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                // Iterate over the nested meta items (e.g., depth and storage)
                for nested in meta_list.nested {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                        if name_value.path.is_ident("depth") {
                            // Parse the `depth` value
                            if let Lit::Int(lit_int) = &name_value.lit {
                                _depth = Some(lit_int.base10_parse::<u32>().unwrap());
                            }
                        } else if name_value.path.is_ident("storage") {
                            // Parse the `storage` value
                            if let Lit::Str(lit_str) = &name_value.lit {
                                _storage = Some(lit_str.value());
                            }
                        }
                    }
                }
            }
        }
    }
    let _depth = _depth.expect("depth must be specified");
    let _storage = _storage.expect("storage must be specified");

    let tree_arg_fields = if let Data::Struct(data_struct) = &input.data {
        data_struct.fields.iter()
            .filter_map(|field| {
                if field.attrs.iter().any(|attr| attr.path.is_ident("tree_arg")) {
                    Some(field.ident.as_ref().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    

    // let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match input.data {
        Data::Struct(ref data) => {
            // let leaf_hash_method = generate_leaf_hash_method(&struct_name);
            // let tree_storage_method = generate_tree_storage_method(&struct_name);
            // let read_on_chain_root_method = generate_read_on_chain_root_method(&struct_name);
    // let expanded = quote! {
    //     impl #impl_generics #struct_name #ty_generics #where_clause {
            let contract_address = quote! {
                let CONTRACT_ADDRESS: [u8;32] = [0;32];
            };
            let depth = quote! {let DEPTH: u32 = #_depth;};
            // fn tree_args(&self) -> Vec<String> {
            //     vec![
            //         #(self.#tree_arg_fields.to_string()), *
            //     ]
            // }
            quote! {
                impl MerkleTree for #struct_name {
                    type Storage = MerkleTreeStorage;
                    const CONTRACT_ADDRESS: [u8;32] = #contract_address;
                    const DEPTH: u32 = #depth;

                    fn to_leaf_hash(&self) -> Fr {
                        // #leaf_hash_method
                        let data = vec![#(#tree_arg_fields.to_string())*].join("/");
                        Fr::from_str(&data).unwrap()
                    }

                    fn tree_storage(&self) -> MerkleTreeStorage {
                        // #tree_storage_method
                        MerkleTreeStorage::new(self.#depth)
                    }

                    async fn read_on_chain_root(&self) -> Fr {
                        // #read_on_chain_root_method
                        Fr::from_str("0").unwrap()
                    }
                }
            }
        }
        _ => {
            panic!("MerkleTree can only be derived for structs");
        }
    };

    TokenStream::from(expanded)
}

// fn generate_leaf_hash_method(struct_name: &Ident) -> proc_macro2::TokenStream {
//     let field_name = struct_name.to_string();
//     quote! {
//         let mut field_value = self.#field_name;
//         field_value.to_bytes_be();
//         #Fr::from_str(field_value)
//     }
// }

// fn generate_tree_storage_method(struct_name: &Ident) -> proc_macro2::TokenStream {
//     let field_name = struct_name.to_string();
//     quote! {
//         // MerkleTreeStorage::new()
//     }
// }

// fn generate_read_on_chain_root_method(struct_name: &Ident) -> proc_macro2::TokenStream {
//     let field_name = struct_name.to_string();
//     quote! {
//         let mut field_value = self.#field_name;
//         field_value.to_bytes_be();
//         #Fr::from_str(field_value)
//     }
// }
