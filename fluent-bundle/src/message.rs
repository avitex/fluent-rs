use fluent_syntax::ast;

#[derive(Debug, PartialEq)]
pub struct FluentAttribute<'m> {
    pub id: &'m str,
    pub value: &'m ast::Pattern<&'m str>,
}

impl<'m> From<&'m ast::Attribute<&'m str>> for FluentAttribute<'m> {
    fn from(attr: &'m ast::Attribute<&'m str>) -> Self {
        FluentAttribute {
            id: attr.id.name,
            value: &attr.value,
        }
    }
}

/// [`FluentMessage`] is a basic translation unit of the Fluent system.
///
/// A message is composed of a value and, optionally a list of attributes.
///
/// # Example
///
/// ```
/// use fluent_bundle::{FluentResource, FluentBundle};
///
/// let source = r#"
///
/// hello-world = Hello, ${ user }
///
/// "#;
///
/// let resource = FluentResource::try_new(source.to_string())
///     .expect("Failed to parse the resource.");
///
/// let mut bundle = FluentBundle::default();
/// bundle.add_resource(resource)
///     .expect("Failed to add a resource.");
///
/// let msg = bundle.get_message("hello-world")
///     .expect("Failed to retrieve a message.");
///
/// assert!(msg.value.is_some());
/// ```
///
/// That value can be then passed to
/// [`FluentBundle::format_pattern`](crate::FluentBundle::format_pattern) to be formatted
/// within the context of a given [`FluentBundle`](crate::FluentBundle) instance.
///
/// # Compound Message
///
/// A message may contain just a simple `id` and `value`, but it can also be what is
/// called a "compound" message.
///
/// In such case, the message contains a list of key-value attributes that represent
/// different translation values associated with a single translation unit.
///
/// This is useful for scenarios where a [`FluentMessage`] is associated with a
/// complex User Interface widget which has multiple attributes that need to be translated.
/// ```text
/// confirm-modal = Are you sure?
///     .confirm = Yes
///     .cancel = No
///     .tooltip = Closing the window will lose all unsaved data.
/// ```
#[derive(Debug, PartialEq)]
pub struct FluentMessage<'m> {
    pub value: Option<&'m ast::Pattern<&'m str>>,
    pub attributes: Vec<FluentAttribute<'m>>,
}

impl<'m> FluentMessage<'m> {
    pub fn get_attribute(&self, key: &str) -> Option<&FluentAttribute> {
        self.attributes.iter().find(|attr| attr.id == key)
    }
}

impl<'m> From<&'m ast::Message<&'m str>> for FluentMessage<'m> {
    fn from(msg: &'m ast::Message<&'m str>) -> Self {
        FluentMessage {
            value: msg.value.as_ref(),
            attributes: msg.attributes.iter().map(Into::into).collect(),
        }
    }
}
