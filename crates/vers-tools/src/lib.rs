//! # `vengine_rs`
//!
//! ## Example
//!
//! ```
//! use vengine_rs::SystemInfo;
//!
//! let info = SystemInfo::new();
//! println!("{}", info.os_arch);
//! ```

mod system_info;

pub use system_info::SystemInfo;
pub use system_info::error::SystemError;
