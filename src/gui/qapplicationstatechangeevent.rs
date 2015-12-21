// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
}

// body block begin
// class sizeof(QApplicationStateChangeEvent)=24
pub struct QApplicationStateChangeEvent {
  pub qclsinst: *mut c_void,
}

