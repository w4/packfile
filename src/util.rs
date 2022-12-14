use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    ops::Deref,
    sync::Arc,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ArcOrCowStr {
    Arc(Arc<str>),
    Cow(Cow<'static, str>),
}

impl From<Arc<str>> for ArcOrCowStr {
    fn from(v: Arc<str>) -> Self {
        Self::Arc(v)
    }
}

impl From<Cow<'static, str>> for ArcOrCowStr {
    fn from(v: Cow<'static, str>) -> Self {
        Self::Cow(v)
    }
}

impl From<&'static str> for ArcOrCowStr {
    fn from(v: &'static str) -> Self {
        Self::Cow(Cow::Borrowed(v))
    }
}

impl From<String> for ArcOrCowStr {
    fn from(v: String) -> Self {
        Self::Cow(Cow::Owned(v))
    }
}

impl AsRef<str> for ArcOrCowStr {
    fn as_ref(&self) -> &str {
        match self {
            Self::Arc(v) => v.as_ref(),
            Self::Cow(v) => v.as_ref(),
        }
    }
}

impl Deref for ArcOrCowStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Display for ArcOrCowStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}
