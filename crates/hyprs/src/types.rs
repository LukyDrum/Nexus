use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseIntError};

pub(crate) trait HyprsType: Display {
    const SELECTOR_PREFIX: &str = "";

    fn as_selector(&self) -> String
    where
        Self: Display,
    {
        format!("{}{self}", Self::SELECTOR_PREFIX)
    }
}

macro_rules! def_type {
    ($ident:ident, $backing:ty) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $ident($backing);

        impl $ident {
            pub fn new(inner: $backing) -> Self {
                Self(inner)
            }

            pub fn inner(&self) -> &$backing {
                &self.0
            }

            pub fn into_inner(self) -> $backing {
                self.0
            }
        }

        impl From<$backing> for $ident {
            fn from(value: $backing) -> Self {
                Self(value)
            }
        }

        impl Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inner())
            }
        }
    };
}

macro_rules! def_string_type {
    ($ident:ident) => {
        def_string_type!($ident, prefix = "");
    };
    ($ident:ident, prefix = $prefix:literal) => {
        def_type!($ident, String);

        impl<'a> From<&'a str> for $ident {
            fn from(value: &'a str) -> Self {
                Self(value.to_owned())
            }
        }

        impl HyprsType for $ident {
            const SELECTOR_PREFIX: &str = $prefix;
        }
    };
}

macro_rules! def_num_type {
    ($ident:ident) => {
        def_type!($ident, u32);

        impl<'a> TryFrom<&'a str> for $ident {
            type Error = ParseIntError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse().map(Self)
            }
        }

        impl TryFrom<String> for $ident {
            type Error = ParseIntError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.as_str().try_into()
            }
        }

        impl HyprsType for $ident {}
    };
}

pub struct ParseBool01;

macro_rules! def_bool_type {
    ($ident:ident) => {
        def_type!($ident, bool);

        impl<'a> TryFrom<&'a str> for $ident {
            type Error = ParseBool01;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    "0" => Ok(Self(false)),
                    "1" => Ok(Self(true)),
                    _ => Err(ParseBool01),
                }
            }
        }

        impl TryFrom<String> for $ident {
            type Error = ParseBool01;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.as_str().try_into()
            }
        }

        impl HyprsType for $ident {}
    };
}

/* Workspace related */
def_string_type!(WorkspaceName);
def_num_type!(WorkspaceId);

/* Monitor related */
def_string_type!(MonitorName);
def_num_type!(MonitorId);
def_string_type!(MonitorDescription);

/* Window related */
def_string_type!(WindowClass, prefix = "class:");
def_string_type!(WindowTitle, prefix = "title:");
def_string_type!(WindowAddres, prefix = "address:");

/* Keyboard related */
def_string_type!(KeyboardName);
def_string_type!(LayoutName);

/* ScreenCast */
def_string_type!(ScreenCastState);
def_string_type!(ScreenCastOwner);
def_string_type!(ScreenCastName);

/* Other */
def_string_type!(Namespace);
def_string_type!(SubMapName);

def_bool_type!(Fullscreen);
def_bool_type!(Floating);
def_bool_type!(ToggleStatus);
def_bool_type!(IgnoreGroupLock);
def_bool_type!(LockGroups);
def_bool_type!(Minimized);
def_bool_type!(PinState);

/* Special types */

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ActiveWindow;

impl Display for ActiveWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "activewindow")
    }
}
impl HyprsType for ActiveWindow {
    fn as_selector(&self) -> String {
        self.to_string()
    }
}
