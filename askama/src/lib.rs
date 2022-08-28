//! Askama implements a type-safe compiler for Jinja-like templates.
//! It lets you write templates in a Jinja-like syntax,
//! which are linked to a `struct` defining the template context.
//! This is done using a custom derive implementation (implemented
//! in [`askama_derive`](https://crates.io/crates/askama_derive)).
//!
//! For feature highlights and a quick start, please review the
//! [README](https://github.com/djc/askama/blob/main/README.md).
//!
//! The primary documentation for this crate now lives in
//! [the book](https://djc.github.io/askama/).
//!
//! # Creating Askama templates
//!
//! An Askama template is a `struct` definition which provides the template
//! context combined with a UTF-8 encoded text file (or inline source, see
//! below). Askama can be used to generate any kind of text-based format.
//! The template file's extension may be used to provide content type hints.
//!
//! A template consists of **text contents**, which are passed through as-is,
//! **expressions**, which get replaced with content while being rendered, and
//! **tags**, which control the template's logic.
//! The template syntax is very similar to [Jinja](http://jinja.pocoo.org/),
//! as well as Jinja-derivatives like [Twig](http://twig.sensiolabs.org/) or
//! [Tera](https://github.com/Keats/tera).
//!
//! ## The `template()` attribute
//!
//! Askama works by generating one or more trait implementations for any
//! `struct` type decorated with the `#[derive(Template)]` attribute. The
//! code generation process takes some options that can be specified through
//! the `template()` attribute. The following sub-attributes are currently
//! recognized:
//!
//! * `path` (as `path = "foo.html"`): sets the path to the template file. The
//!   path is interpreted as relative to the configured template directories
//!   (by default, this is a `templates` directory next to your `Cargo.toml`).
//!   The file name extension is used to infer an escape mode (see below). In
//!   web framework integrations, the path's extension may also be used to
//!   infer the content type of the resulting response.
//!   Cannot be used together with `source`.
//! * `source` (as `source = "{{ foo }}"`): directly sets the template source.
//!   This can be useful for test cases or short templates. The generated path
//!   is undefined, which generally makes it impossible to refer to this
//!   template from other templates. If `source` is specified, `ext` must also
//!   be specified (see below). Cannot be used together with `path`.
//! * `ext` (as `ext = "txt"`): lets you specify the content type as a file
//!   extension. This is used to infer an escape mode (see below), and some
//!   web framework integrations use it to determine the content type.
//!   Cannot be used together with `path`.
//! * `print` (as `print = "code"`): enable debugging by printing nothing
//!   (`none`), the parsed syntax tree (`ast`), the generated code (`code`)
//!   or `all` for both. The requested data will be printed to stdout at
//!   compile time.
//! * `escape` (as `escape = "none"`): override the template's extension used for
//!   the purpose of determining the escaper for this template. See the section
//!   on configuring custom escapers for more information.
//! * `syntax` (as `syntax = "foo"`): set the syntax name for a parser defined
//!   in the configuration file. The default syntax , "default",  is the one
//!   provided by Askama.

#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unreachable_pub)]

mod error;
pub mod filters;
pub mod helpers;

pub use askama_derive::Template;
pub use askama_escape::{Html, MarkupDisplay, Text};

#[doc(hidden)]
pub use {async_stream::try_stream, bytestring::ByteString, futures_util::stream::LocalBoxStream};

/// Result of calling `Template::render`
/// currently the implementation by the derive macro uses [async_stream](https://crates.io/crates/async-stream)
/// in the future when async generators become stable the api will use compiler generated gnerator
///
/// ## 'static lifetime and caveats
/// hence the `Template::render` method takes ownership it requires `'static` lifetime.
/// if you are using generic **types** you should add the 'static bounds specially for `Stream`s and `Future`s
///
/// ## Why `ByteString`
///
/// The type `ByteString` is backed by [`Bytes`](https://docs.rs/bytes/latest/bytes/struct.Bytes.html)
/// Which can store readonly view to both `&'static` and `String` for less allocations
pub type RenderResult = LocalBoxStream<'static, Result<ByteString>>;

#[doc(hidden)]
pub use crate as shared;
pub use crate::error::{Error, Result};

/// Main `Template` trait; implementations are generally derived
///
/// If you need an object-safe template, use [`DynTemplate`].
pub trait Template {
    /// Renders the template as a stream
    ///
    /// ## Why `self by value`
    /// hence the implementation uses `async move {}` which requires ownership and `'static` lifetime
    fn render(self) -> RenderResult;

    /// The template's extension, if provided
    const EXTENSION: Option<&'static str>;

    /// Provides a conservative estimate of the expanded length of the rendered template
    const SIZE_HINT: usize;

    /// The MIME type (Content-Type) of the data that gets rendered by this Template
    const MIME_TYPE: &'static str;
}

/// Object-safe wrapper trait around [`Template`] implementers
///
/// This trades reduced performance (mostly due to writing into `dyn Write`) for object safety.
pub trait DynTemplate {
    fn dyn_render(self) -> RenderResult;

    /// Helper function to inspect the template's extension
    fn extension(&self) -> Option<&'static str>;

    /// Provides a conservative estimate of the expanded length of the rendered template
    fn size_hint(&self) -> usize;

    /// The MIME type (Content-Type) of the data that gets rendered by this Template
    fn mime_type(&self) -> &'static str;
}

impl<T: Template> DynTemplate for T {
    fn dyn_render(self) -> RenderResult {
        Template::render(self)
    }

    fn extension(&self) -> Option<&'static str> {
        Self::EXTENSION
    }

    fn size_hint(&self) -> usize {
        Self::SIZE_HINT
    }

    fn mime_type(&self) -> &'static str {
        Self::MIME_TYPE
    }
}
