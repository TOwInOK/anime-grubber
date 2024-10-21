#[macro_export]
macro_rules! gen_enum {
    ($name:tt, [ $($variant:ident),* $(,)? ]) => {
        #[derive(Debug)]
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
    };
    ($name:tt, [ $($variant:ident($nested:ty)),* $(,)? ]) => {
        #[derive(Debug)]
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
    };
}
