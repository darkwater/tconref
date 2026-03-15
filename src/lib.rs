#![feature(const_convert, const_default, const_trait_impl)]
#![warn(clippy::all, rust_2018_idioms)]

pub mod data;

mod app;
pub use app::TemplateApp;
