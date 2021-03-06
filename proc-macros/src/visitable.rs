use crate::helpers::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{Data, DeriveInput};

pub fn visitable_impl(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let path = quote!(rtk::visitor);

    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(rtk::widget::Widget)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data) => {
            let child_fields = find_tagged_fields(&data.fields, "visit_child");
            let iter_fields = find_tagged_fields(&data.fields, "visit_iter");

            let mut expanded: Vec<_> = child_fields
                .iter()
                .map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            let visitor = self.#field.accept(visitor, &ctx);
                            if visitor.finished() { return visitor }
                        },
                    )
                })
                .chain(iter_fields.iter().map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            let mut visitor = visitor;
                            for obj in &mut self.#field {
                                visitor = obj.accept(visitor, &ctx);
                                if visitor.finished() { return visitor }
                            }
                        },
                    )
                }))
                .collect();

            expanded.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

            let stmts = expanded.into_iter().map(|(_, s)| s);

            Ok(quote! {
                impl #impl_generics #path::Visitable for #name #ty_generics #where_clause {
                    #[inline]
                    fn accept<V: #path::Visitor>(&mut self, visitor: V, prev_ctx: &V::Context) -> V {
                        if visitor.finished() { return visitor }
                        if let Some(ctx) = visitor.new_context(self, prev_ctx) {
                            let visitor = visitor.visit_before(self, &ctx);
                            #(#stmts)*
                            visitor.visit_after(self, &ctx)
                        } else {
                            visitor
                        }
                    }
                }
            })
        }
        Data::Enum(data) => match_patterns_for_enum(&data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Visitable for #name #ty_generics #where_clause {
                    #[inline]
                    fn accept<V: #path::Visitor>(&mut self, visitor: V, ctx: &V::Context) -> V {
                        match self {
                            #(#patterns => #path::Visitable::accept(a, visitor, ctx),)*
                        }
                    }
                }
            }
        }),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    expanded.unwrap_or_else(|err| err.to_error("Visitable").to_compile_error())
}
