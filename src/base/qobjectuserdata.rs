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
  fn _ZN15QObjectUserDataD0Ev() -> i32;
}

// body block begin
// class sizeof(QObjectUserData)=8
pub struct QObjectUserData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QObjectUserData {
  pub fn FreeQObjectUserData<T: QObjectUserData_FreeQObjectUserData>(&mut self, value: T) -> i32 {
    value.FreeQObjectUserData(self);
    return 1;
  }
}

pub trait QObjectUserData_FreeQObjectUserData {
  fn FreeQObjectUserData(self, this: &mut QObjectUserData) -> i32;
}

// proto: void QObjectUserData::FreeQObjectUserData();
impl<'a> /*trait*/ QObjectUserData_FreeQObjectUserData for () {
  fn FreeQObjectUserData(self, this: &mut QObjectUserData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QObjectUserDataD0Ev()};
    unsafe {_ZN15QObjectUserDataD0Ev()};
    return 1;
  }
}

