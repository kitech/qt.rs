// #[warn(non_snake_case)]
// #[warn(non_camel_case_types)]
// #[warn(unused_mut)]
// #[warn(unused_attributes)]
// #[warn(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(unused_imports)]

#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
#[link(name = "QtInline")]
extern {}  // 这行还是需要的

pub mod core;
pub mod gui;
pub mod widgets;

