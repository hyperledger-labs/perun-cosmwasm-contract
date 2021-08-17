#![deny(warnings)]
#![allow(clippy::ptr_arg)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(dead_code)]
#![deny(rustdoc::bare_urls)]
#![deny(unused_imports)]
#![doc(html_logo_url = "https://perun.network/images/Asset%2010.svg")]
#![doc(html_favicon_url = "https://perun.network/favicon-32x32.png")]
#![doc(issue_tracker_base_url = "https://github.com/perun-network/perun-cosmwasm/issues")]

pub mod contract;
pub mod crypto;
pub mod error;
pub mod msg;
pub mod storage;
pub mod types;
