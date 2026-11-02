use std::borrow::Cow;

/// Filtering for committed, single-line user edits. It is independent of numeric
/// masks: there is no rounding, range clamp, length cap or separator insertion.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputFilter {
    #[default]
    None,
    AsciiDigits,
}
impl InputFilter {
    pub(crate) fn apply<'a>(self, text: &'a str) -> Cow<'a, str> {
        match self {
            Self::None => Cow::Borrowed(text),
            Self::AsciiDigits if text.bytes().all(|byte| byte.is_ascii_digit()) => {
                Cow::Borrowed(text)
            }
            Self::AsciiDigits => Cow::Owned(text.chars().filter(char::is_ascii_digit).collect()),
        }
    }
}
