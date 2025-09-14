macro_rules! auto_import {
    ($mod_name:ident) => {
        #[cfg_attr(docsrs, cfg(feature = "v0_1"))]
        #[cfg_attr(not(docsrs), cfg(all(feature = "v0_1", not(feature = "v0_2"))))]
        pub use crate::v0_1::$mod_name::*;
        #[cfg(feature = "v0_2")]
        pub use crate::v0_2::$mod_name::*;
    };
}

macro_rules! auto_mod {
    ($mod_name:ident) => {
        pub mod $mod_name {
            auto_import!($mod_name);
        }
    };
}

auto_mod!(cta);
auto_mod!(cvt);
auto_mod!(displayid);
auto_mod!(dmt);
auto_mod!(edid);
auto_mod!(gtf);
auto_mod!(info);
