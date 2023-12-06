#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "without_tag_import")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "without_tag_import")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "package-2022-05", not(feature = "without_tag_import")))]
pub use package_2022_05::*;
#[cfg(feature = "package-2021-06-30-preview")]
pub mod package_2021_06_30_preview;
#[cfg(all(feature = "package-2021-06-30-preview", not(feature = "without_tag_import")))]
pub use package_2021_06_30_preview::*;
#[cfg(feature = "package-2020-12")]
pub mod package_2020_12;
#[cfg(all(feature = "package-2020-12", not(feature = "without_tag_import")))]
pub use package_2020_12::*;
