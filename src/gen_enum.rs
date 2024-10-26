/// A macro for generating enums with string representation capabilities.
///
/// # Description
/// This macro creates an enum with specified variants and implements conversion
/// from a reference of the enum to a string slice (`&str`). It also provides
/// methods to get string representations of nested enum variants.
///
/// # Generated Methods
/// - `From<&T> for &str`: returns current variant name
/// - `nested_str()`: returns string representation one level down
/// - `deepest_str()`: recursively gets the deepest nested variant name
///
/// # Examples
///
/// 1. Simple variants without associated data:
/// ```rust
/// use anime_grubber::gen_enum;
///
/// gen_enum!(SimpleEnum, [First, Second, Third]);
///
/// let variant = SimpleEnum::First;
/// assert_eq!(<&str>::from(&variant), "First");
/// assert_eq!(variant.deepest_str(), "First");
/// ```
///
/// 2. Simple nested enum:
/// ```rust
/// use anime_grubber::gen_enum;
///
/// gen_enum!(Inner, [One, Two]);
/// gen_enum!(Outer, [Value(Inner)]);
///
/// let nested = Outer::Value(Inner::One);
/// assert_eq!(<&str>::from(&nested), "Value");  // Current variant
/// assert_eq!(nested.nested_str(), "One");      // One level down
/// assert_eq!(nested.deepest_str(), "One");     // Deepest level
/// ```
///
/// 3. Deep nested example (5 levels):
/// ```rust
/// use anime_grubber::gen_enum;
///
/// gen_enum!(Level5, [Five, Six]);
/// gen_enum!(Level4, [Fourth(Level5)]);
/// gen_enum!(Level3, [Third(Level4)]);
/// gen_enum!(Level2, [Second(Level3)]);
/// gen_enum!(Level1, [First(Level2)]);
///
/// let deep = Level1::First(
///     Level2::Second(
///         Level3::Third(
///             Level4::Fourth(
///                 Level5::Five
///             )
///         )
///     )
/// );
///
/// // Get current level variant name
/// assert_eq!(<&str>::from(&deep), "First");
///
/// // Get the deepest nested value
/// assert_eq!(deep.deepest_str(), "Five");
///
/// // Access intermediate levels through destructuring
/// let Level1::First(level2) = deep;
/// assert_eq!(<&str>::from(&level2), "Second");
///
/// let Level2::Second(level3) = level2;
/// assert_eq!(<&str>::from(&level3), "Third");
///
/// let Level3::Third(level4) = level3;
/// assert_eq!(<&str>::from(&level4), "Fourth");
///
/// let Level4::Fourth(level5) = level4;
/// assert_eq!(<&str>::from(&level5), "Five");
/// ```
#[macro_export]
macro_rules! gen_enum {
    ($name:tt, [ $($variant:ident),* $(,)? ]) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $(
                $variant,
            )*
        }
        impl From<&$name> for &str {
            fn from(value: &$name) -> Self {
                match value {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                }
            }
        }
        impl $name {
            pub fn deepest_str(&self) -> &str {
                <&str>::from(self)
            }
        }
    };
    ($name:tt, [ $($variant:ident($nested:ty)),* $(,)? ]) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $(
                $variant($nested),
            )*
        }
        impl From<&$name> for &str {
            fn from(value: &$name) -> Self {
                match value {
                    $(
                        $name::$variant(_) => stringify!($variant),
                    )*
                }
            }
        }
        impl $name {
            pub fn nested_str(&self) -> &str {
                match self {
                    $(
                        $name::$variant(inner) => <&str>::from(inner),
                    )*
                }
            }
            pub fn deepest_str(&self) -> &str {
                match self {
                    $(
                        $name::$variant(inner) => inner.deepest_str(),
                    )*
                }
            }
        }
    };
}
