use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Type};

type Str = &'static str;
type FieldFindResult<T> = Result<T, FieldFindError>;

pub enum FieldFindError {
    Duplicate(Span, Str),
    DuplicateAttr(Span, Str),
    NotFound(Span, Str),
    Empty(Span),
    Unsupported(Span, Str),
}

impl FieldFindError {
    pub fn to_error(self, t_name: &str) -> Error {
        match self {
            FieldFindError::Duplicate(span, name) => Error::new(
                span,
                format!("found multiple `{}` typed fields while deriving `{}`", name, t_name),
            ),
            FieldFindError::DuplicateAttr(span, name) => Error::new(
                span,
                format!("found multiple `{}` attributes while deriving `{}`", name, t_name),
            ),
            FieldFindError::NotFound(span, name) => Error::new(
                span,
                format!("field with `{}` type not found while deriving `{}`", name, t_name),
            ),
            FieldFindError::Empty(span) => Error::new(span, format!("can't derive `{}` on empty type", t_name)),
            FieldFindError::Unsupported(span, name) => {
                Error::new(span, format!("can't derive `{}` for `{}`", t_name, name))
            }
        }
    }
}

pub fn find_named_field(fields: &FieldsNamed, name: Str, tag: Str) -> FieldFindResult<TokenStream> {
    if fields.named.is_empty() {
        return Err(FieldFindError::Empty(fields.brace_token.span));
    }

    let mut field_found = None;

    for field in &fields.named {
        for attr in &field.attrs {
            if let Some(attr_name) = attr.path.segments.iter().last() {
                if attr_name.ident == tag {
                    if field_found.is_some() {
                        return Err(FieldFindError::DuplicateAttr(attr_name.ident.span(), tag));
                    }
                    field_found = field.ident.as_ref();
                }
            }
        }
    }

    if field_found.is_none() {
        for field in &fields.named {
            if let Type::Path(path) = &field.ty {
                if let Some(ty_name) = path.path.segments.iter().last() {
                    if ty_name.ident == name {
                        if field_found.is_some() {
                            return Err(FieldFindError::Duplicate(field.ident.as_ref().unwrap().span(), name));
                        }
                        field_found = field.ident.as_ref();
                    }
                }
            }
        }
    }

    field_found
        .map(ToTokens::to_token_stream)
        .ok_or(FieldFindError::NotFound(fields.brace_token.span, name))
}

pub fn find_unnamed_field(fields: &FieldsUnnamed, name: Str, tag: Str) -> FieldFindResult<TokenStream> {
    if fields.unnamed.is_empty() {
        return Err(FieldFindError::Empty(fields.paren_token.span));
    }

    let mut field_found = None;

    for (i, field) in fields.unnamed.iter().enumerate() {
        for attr in &field.attrs {
            if let Some(attr_name) = attr.path.segments.iter().last() {
                if attr_name.ident == tag {
                    if field_found.is_some() {
                        return Err(FieldFindError::DuplicateAttr(attr_name.ident.span(), tag));
                    }
                    field_found = Some(i);
                }
            }
        }
    }

    if field_found.is_none() {
        for (i, field) in fields.unnamed.iter().enumerate() {
            if let Type::Path(path) = &field.ty {
                if let Some(ty_name) = path.path.segments.iter().last() {
                    if ty_name.ident == name {
                        if field_found.is_some() {
                            return Err(FieldFindError::Duplicate(ty_name.ident.span(), name));
                        }
                        field_found = Some(i);
                    }
                }
            }
        }
    }

    field_found
        .map(|i| Index::from(i).to_token_stream())
        .ok_or(FieldFindError::NotFound(fields.paren_token.span, name))
}

pub fn find_field_struct(data: &DataStruct, s_name: &Ident, ty_name: Str, tag: Str) -> FieldFindResult<TokenStream> {
    match &data.fields {
        Fields::Named(fields) => find_named_field(fields, ty_name, tag),
        Fields::Unnamed(fields) => find_unnamed_field(fields, ty_name, tag),
        Fields::Unit => Err(FieldFindError::Empty(s_name.span())),
    }
}

pub fn find_field_enum(data: &DataEnum, e_name: &Ident) -> FieldFindResult<Vec<TokenStream>> {
    data.variants
        .iter()
        .map(|variant| {
            match &variant.fields {
                Fields::Named(fields) => fields
                    .named
                    .first()
                    .map(|first| first.ident.to_token_stream())
                    .ok_or_else(|| FieldFindError::Empty(fields.brace_token.span)),
                Fields::Unnamed(fields) => {
                    if fields.unnamed.is_empty() {
                        Err(FieldFindError::Empty(fields.paren_token.span))
                    } else {
                        Ok(Index::from(0).to_token_stream())
                    }
                }
                Fields::Unit => Err(FieldFindError::Empty(variant.ident.span())),
            }
            .map(|field| {
                let vname = &variant.ident;
                quote! { #e_name::#vname { #field: a, .. } }
            })
        })
        .collect()
}
