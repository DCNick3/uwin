use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{bracketed, parenthesized, token, Error, LitByteStr, LitInt, LitStr, Token};

#[derive(Debug)]
enum CpuFlag {
    Carry,
    Zero,
    Sign,
    Overflow,
}

struct TestSnippet {
    name: Ident,
    _colon: Token![:],
    _paren_token: token::Paren,
    asm: TokenStream,
    _bracket_token: token::Bracket,
    flags: Vec<CpuFlag>,
}

struct Arg {
    data: Vec<u32>,
    name: Ident,
}

struct TestFunction {
    name: Ident,
    _colon: Token![:],
    _bracket_token: token::Bracket,
    args: Punctuated<Arg, Token![,]>,
    _paren_token: token::Paren,
    asm: TokenStream,
}

struct TestElfFunction {
    name: Ident,
    _colon: Token![:],
    _bracket_token: token::Bracket,
    args: Punctuated<Arg, Token![,]>,
    _paren_token: token::Paren,
    path: LitStr,
}

struct TestPeFunction {
    name: Ident,
    _colon: Token![:],
    _bracket_token: token::Bracket,
    args: Punctuated<Arg, Token![,]>,
    _paren_token: token::Paren,
    path: LitStr,
}

impl Parse for CpuFlag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id: Ident = input.parse()?;

        match id.to_string().as_str() {
            "CF" => Ok(CpuFlag::Carry),
            "ZF" => Ok(CpuFlag::Zero),
            "SF" => Ok(CpuFlag::Sign),
            "OF" => Ok(CpuFlag::Overflow),
            _ => Err(Error::new(id.span(), "Unknown CPU flag")),
        }
    }
}

fn parse_flags(input: ParseStream) -> syn::Result<Vec<CpuFlag>> {
    let mut res = Vec::new();
    while !input.is_empty() {
        res.push(input.parse()?);
    }
    Ok(res)
}

impl Parse for TestSnippet {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let asm;
        let flags;
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _paren_token: parenthesized!(asm in input),
            asm: asm.parse()?,
            _bracket_token: bracketed!(flags in input),
            flags: flags.call(parse_flags)?,
        })
    }
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let (data, name) = if input.peek(token::Paren) {
            let ints;
            parenthesized!(ints in input);
            let ints: Punctuated<u32, Token![,]> = ints.parse_terminated(|s| {
                let neg = s.peek(Token![-]);
                if neg {
                    let _: Token![-] = s.parse()?;
                }
                let span = s.span();
                let lit: LitInt = s.parse()?;
                let mut val = lit.base10_parse::<u64>()?;
                if neg {
                    if val > 2147483648 {
                        return Err(Error::new(span, "Negative value too small to fit into u32"));
                    }
                    val = !val + 1
                }
                Ok(val as u32)
            })?;
            let ints: Vec<u32> = ints.into_iter().collect();

            let ints_str: Vec<String> =
                ints.iter().cloned().map(|v| format!("{:08x}", v)).collect();
            (ints, ints_str.join("_"))
        } else {
            let literal: LitByteStr = input.parse()?;
            let byte = literal.value();
            let mut res = vec![byte.len() as u32];
            res.extend(byte.iter().map(|&x| x as u32));

            // if we have a trailing zero byte - trim it off (to allow nice names for c-like strings)
            let bytes = if byte.last() == Some(&0u8) {
                &byte[..byte.len() - 1]
            } else {
                byte.as_slice()
            };

            //println!("{:?}", bytes);

            let name = if bytes.iter().all(|t| (0x20..=0x7e).contains(t))
                && syn::parse_str::<Ident>(
                    format!("on_{}", std::str::from_utf8(bytes).unwrap()).as_str(),
                )
                .is_ok()
            {
                format!("on_str_{}", std::str::from_utf8(bytes).unwrap())
            } else {
                hex::encode(byte)
            };

            (res, name)
        };

        Ok(Self {
            data,
            name: Ident::new(format!("on_{name}").as_str(), span),
        })
    }
}

impl Parse for TestFunction {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args;
        let asm;
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _bracket_token: bracketed!(args in input),
            args: args.parse_terminated(Arg::parse)?,
            _paren_token: parenthesized!(asm in input),
            asm: asm.parse()?,
        })
    }
}

impl Parse for TestElfFunction {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args;
        let path;
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _bracket_token: bracketed!(args in input),
            args: args.parse_terminated(Arg::parse)?,
            _paren_token: parenthesized!(path in input),
            path: path.parse()?,
        })
    }
}

impl Parse for TestPeFunction {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args;
        let path;
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            _bracket_token: bracketed!(args in input),
            args: args.parse_terminated(Arg::parse)?,
            _paren_token: parenthesized!(path in input),
            path: path.parse()?,
        })
    }
}

impl ToTokens for CpuFlag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = Ident::new(&format!("{:?}", self), Span::call_site());
        tokens.append_all(quote! {
            rusty_x86::types::Flag::#id
        })
    }
}

impl ToTokens for TestSnippet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let code = &self.asm;
        let flags = &self.flags;

        tokens.append_all(quote! {
            mod #name {
                use crate::common::MEM_ADDR;

                fn get_code() -> Vec<u8> {
                    rusty_x86::assemble_x86!(
                        #code
                    )
                }

                #[test_log::test]
                fn llvm() {
                    log::info!("Running {}::llvm", stringify!(#name));
                    let code = get_code();
                    crate::common::test_code(
                        crate::common::CodeToTest::Snippet(code.as_slice()),
                        vec![#(#flags),*],
                        crate::common::Backend::Llvm
                    );
                }

                #[test_log::test]
                fn interp() {
                    log::info!("Running {}::interp", stringify!(#name));
                    let code = get_code();
                    crate::common::test_code(
                        crate::common::CodeToTest::Snippet(code.as_slice()),
                        vec![#(#flags),*],
                        crate::common::Backend::Interp
                    );
                }
            }
        });
    }
}

impl ToTokens for TestFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let asm = &self.asm;

        let args: Vec<TokenStream> = self
            .args
            .iter()
            .map(|args| {
                let name = &args.name;
                let args = &args.data;

                let mut tokens = TokenStream::new();

                for backend_str in ["Llvm", "Interp"] {
                    let name = Ident::new(
                        &format!("{}_{}", backend_str.to_ascii_lowercase(), name),
                        Span::call_site(),
                    );
                    let backend = TokenStream::from_str(backend_str).unwrap();
                    tokens.extend(quote! {
                        #[test_log::test]
                        fn #name () {
                            let args: &[u32] = &[#(#args),*];
                            log::info!("Running {} on {:?} with {}", stringify!(#name), args, #backend_str);

                            let code = get_code();

                            crate::common::test_code(
                                crate::common::CodeToTest::Function(code.as_slice(), args),
                                vec![],
                                crate::common::Backend::#backend
                            );
                        }
                    });
                }

                tokens
            })
            .collect();

        tokens.append_all(quote! {
            mod #name {

                fn get_code() -> Vec<u8> {
                    rusty_x86::assemble_x86!(
                        #asm
                    )
                }

                #(#args)*
            }
        })
    }
}

impl ToTokens for TestElfFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let path = &self.path;

        let args: Vec<TokenStream> = self
            .args
            .iter()
            .map(|args| {
                let name = &args.name;
                let args = &args.data;

                let mut tokens = TokenStream::new();

                for backend_str in ["Llvm", "Interp"] {
                    let name = Ident::new(
                        &format!("{}_{}", backend_str.to_ascii_lowercase(), name),
                        Span::call_site(),
                    );
                    let backend = TokenStream::from_str(backend_str).unwrap();
                    tokens.extend(quote! {
                        #[test_log::test]
                        fn #name () {
                            let args: &[u32] = &[#(#args),*];
                            log::info!("Running {} on {:?} with {}", stringify!(#name), args, #backend_str);

                            let code = get_elf();

                            crate::common::test_code(
                                crate::common::CodeToTest::ElfFunction(code, args),
                                vec![],
                                crate::common::Backend::#backend
                            );
                        }
                    });
                }

                tokens
            })
            .collect();

        tokens.append_all(quote! {
            mod #name {

                fn get_elf() -> &'static [u8] {
                    include_bytes!(#path)
                }

                #(#args)*
            }
        })
    }
}

impl ToTokens for TestPeFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let path = &self.path;

        let args: Vec<TokenStream> = self
            .args
            .iter()
            .map(|args| {
                let name = &args.name;
                let args = &args.data;

                let mut tokens = TokenStream::new();

                for backend_str in ["Llvm", "Interp"] {
                    let name = Ident::new(
                        &format!("{}_{}", backend_str.to_ascii_lowercase(), name),
                        Span::call_site(),
                    );
                    let backend = TokenStream::from_str(backend_str).unwrap();
                    tokens.extend(quote! {
                        #[test_log::test]
                        fn #name () {
                            let args: &[u32] = &[#(#args),*];
                            log::info!("Running {} on {:?} with {}", stringify!(#name), args, #backend_str);

                            let code = get_pe();

                            crate::common::test_code(
                                crate::common::CodeToTest::PeFunction(code, args),
                                vec![],
                                crate::common::Backend::#backend
                            );
                        }
                    });
                }

                tokens
            })
            .collect();

        tokens.append_all(quote! {
            mod #name {

                fn get_pe() -> &'static [u8] {
                    include_bytes!(#path)
                }

                #(#args)*
            }
        })
    }
}

pub fn test_snippets_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let parser = Punctuated::<TestSnippet, Token![,]>::parse_terminated;
    let input = parser.parse(input.into())?;

    let snippets = input.into_iter();

    Ok(quote! {
        #(#snippets)*
    })
}

pub fn test_functions_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let parser = Punctuated::<TestFunction, Token![,]>::parse_terminated;
    let input = parser.parse(input.into())?;

    let functions = input.into_iter();

    Ok(quote! {
        #(#functions)*
    })
}

pub fn test_elf_functions_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let parser = Punctuated::<TestElfFunction, Token![,]>::parse_terminated;
    let input = parser.parse(input.into())?;

    let functions = input.into_iter();

    Ok(quote! {
        #(#functions)*
    })
}

pub fn test_pe_functions_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let parser = Punctuated::<TestPeFunction, Token![,]>::parse_terminated;
    let input = parser.parse(input.into())?;

    let functions = input.into_iter();

    Ok(quote! {
        #(#functions)*
    })
}
