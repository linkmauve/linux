pub(crate) mod bp;
pub(crate) mod cp;
pub(crate) mod pe;
pub(crate) mod pi;
pub(crate) mod vi;

#[macro_export]
/// Generates an enum which can be converted to and from a Bounded of the right size.
macro_rules! impl_bounded {
    (#[repr($ty:ty)] #[bits($bits:expr)] $(#[$meta:meta])* enum $id:ident {$($(#[$meta2:meta])* $name:ident = $value:expr,)*}) => {
        $(#[$meta])*
        #[derive(Debug)]
        #[repr($ty)]
        pub enum $id {
            $(
                $(#[$meta2])*
                $name = $value,
            )*
        }

        impl From<kernel::num::Bounded<$ty, $bits>> for $id {
            fn from(num: kernel::num::Bounded<$ty, $bits>) -> $id {
                match num.get() {
                    $(
                        $value => $id::$name,
                    )*
                    _ => unreachable!(concat!(stringify!($id), " can only have up to ", $bits, " bits!")),
                }
            }
        }

        impl From<$id> for kernel::num::Bounded<$ty, $bits> {
            fn from(mode: $id) -> kernel::num::Bounded<$ty, $bits> {
                match mode {
                    $(
                        $id::$name => kernel::num::Bounded::<$ty, $bits>::new::<$value>(),
                    )*
                }
            }
        }
    };
}
