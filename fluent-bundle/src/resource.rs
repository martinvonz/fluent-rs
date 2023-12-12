use fluent_syntax::ast;
use fluent_syntax::parser::{parse_runtime, ParserError};
use yoke::Yoke;

type Resource<'s> = ast::Resource<&'s str>;

type InnerFluentResource = Yoke<Resource<'static>, String>;

/// A resource containing a list of localization messages.
///
/// [`FluentResource`] wraps an [`Abstract Syntax Tree`](../fluent_syntax/ast/index.html) produced by the
/// [`parser`](../fluent_syntax/parser/index.html) and provides an access to a list
/// of its entries.
///
/// A good mental model for a resource is a single FTL file, but in the future
/// there's nothing preventing a resource from being stored in a data base,
/// pre-parsed format or in some other structured form.
///
/// # Example
///
/// ```
/// use fluent_bundle::FluentResource;
///
/// let source = r#"
///
/// hello-world = Hello World!
///
/// "#;
///
/// let resource = FluentResource::try_new(source.to_string())
///     .expect("Errors encountered while parsing a resource.");
///
/// assert_eq!(resource.entries().count(), 1);
/// ```
///
/// # Ownership
///
/// A resource owns the source string and the AST contains references
/// to the slices of the source.
#[derive(Debug)]
pub struct FluentResource(InnerFluentResource);

impl FluentResource {
    /// A fallible constructor of a new [`FluentResource`].
    ///
    /// It takes an encoded `Fluent Translation List` string, parses
    /// it and stores both, the input string and the AST view of it,
    /// for runtime use.
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    ///
    /// let source = r#"
    ///
    /// hello-world = Hello, { $user }!
    ///
    /// "#;
    ///
    /// let resource = FluentResource::try_new(source.to_string());
    ///
    /// assert!(resource.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// The method will return the resource irrelevant of parse errors
    /// encountered during parsing of the source, but in case of errors,
    /// the `Err` variant will contain both the structure and a vector
    /// of errors.
    pub fn try_new(source: String) -> Result<Self, (Self, Vec<ParserError>)> {
        let mut errors = None;

        let res =
            InnerFluentResource::attach_to_cart(source, |source| match parse_runtime(source) {
                Ok(ast) => ast,
                Err((ast, err)) => {
                    errors = Some(err);
                    ast
                }
            });

        match errors {
            None => Ok(Self(res)),
            Some(err) => Err((Self(res), err)),
        }
    }

    /// Returns a reference to the source string that was used
    /// to construct the [`FluentResource`].
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    ///
    /// let source = "hello-world = Hello, { $user }!";
    ///
    /// let resource = FluentResource::try_new(source.to_string())
    ///     .expect("Failed to parse FTL.");
    ///
    /// assert_eq!(
    ///     resource.source(),
    ///     "hello-world = Hello, { $user }!"
    /// );
    /// ```
    pub fn source(&self) -> &str {
        self.0.backing_cart()
    }

    /// Returns an iterator over [`entries`](fluent_syntax::ast::Entry) of the [`FluentResource`].
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    /// use fluent_syntax::ast;
    ///
    /// let source = r#"
    ///
    /// hello-world = Hello, { $user }!
    ///
    /// "#;
    ///
    /// let resource = FluentResource::try_new(source.to_string())
    ///     .expect("Failed to parse FTL.");
    ///
    /// assert_eq!(
    ///     resource.entries().count(),
    ///     1
    /// );
    /// assert!(matches!(resource.entries().next(), Some(ast::Entry::Message(_))));
    /// ```
    pub fn entries(&self) -> impl Iterator<Item = &ast::Entry<&str>> {
        Yoke::get(&self.0).body.iter()
    }

    /// Returns an [`Entry`](fluent_syntax::ast::Entry) at the
    /// given index out of the [`FluentResource`].
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    /// use fluent_syntax::ast;
    ///
    /// let source = r#"
    ///
    /// hello-world = Hello, { $user }!
    ///
    /// "#;
    ///
    /// let resource = FluentResource::try_new(source.to_string())
    ///     .expect("Failed to parse FTL.");
    ///
    /// assert!(matches!(resource.get_entry(0), Some(ast::Entry::Message(_))));
    /// ```
    pub fn get_entry(&self, idx: usize) -> Option<&ast::Entry<&str>> {
        Yoke::get(&self.0).body.get(idx)
    }
}
