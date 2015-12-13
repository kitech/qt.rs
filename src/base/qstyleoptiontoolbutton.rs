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
  // proto: void QStyleOptionToolButton::NewQStyleOptionToolButton(int version);
  fn _ZN22QStyleOptionToolButtonC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QStyleOptionToolButton::NewQStyleOptionToolButton();
  fn _ZN22QStyleOptionToolButtonC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QStyleOptionToolButton::NewQStyleOptionToolButton(const QStyleOptionToolButton & other);
  fn _ZN22QStyleOptionToolButtonC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStyleOptionToolButton)=1
pub struct QStyleOptionToolButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionToolButton {
  pub fn NewQStyleOptionToolButton<T: QStyleOptionToolButton_NewQStyleOptionToolButton>(value: T) -> QStyleOptionToolButton {
    let rsthis = value.NewQStyleOptionToolButton();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolButton_NewQStyleOptionToolButton {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton;
}

// proto: void QStyleOptionToolButton::NewQStyleOptionToolButton(int version);
impl<'a> /*trait*/ QStyleOptionToolButton_NewQStyleOptionToolButton for (i32) {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionToolButtonC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionToolButton::NewQStyleOptionToolButton();
impl<'a> /*trait*/ QStyleOptionToolButton_NewQStyleOptionToolButton for () {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1Ev()};
    unsafe {_ZN22QStyleOptionToolButtonC1Ev(qthis)};
    let rsthis = QStyleOptionToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionToolButton::NewQStyleOptionToolButton(const QStyleOptionToolButton & other);
impl<'a> /*trait*/ QStyleOptionToolButton_NewQStyleOptionToolButton for (&'a  QStyleOptionToolButton) {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN22QStyleOptionToolButtonC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

