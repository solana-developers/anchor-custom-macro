// use anchor_lang::prelude::*;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

#[proc_macro_derive(Instructions)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(n) => n.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let instruction = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let fname = format_ident!("update_{}", name.clone().unwrap());

        quote! {
            pub fn #fname(ctx: Context<UpdateAdminAccount>, new_value: #ty) -> Result<()> {
                let admin_account = &mut ctx.accounts.admin_account;
                admin_account.#name = new_value;
                Ok(())
            }
        }
    });

    let expanded = quote! {
        impl #ident {
            #(#instruction)*
        }

        // #[derive(Accounts)]
        // pub struct UpdateAdminAccount<'info> {
        //     pub auth: Signer<'info>,
        //     #[account(
        //         mut,
        //         has_one = auth,
        //     )]
        //     pub admin_account: Account<'info, #ident>
        // }
    };
    expanded.into()
}
