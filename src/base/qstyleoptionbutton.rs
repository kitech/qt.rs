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
  // proto:  void QStyleOptionButton::NewQStyleOptionButton(int version);
  fn _ZN18QStyleOptionButtonC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStyleOptionButton::NewQStyleOptionButton();
  fn _ZN18QStyleOptionButtonC1Ev(qthis: *mut c_void) ;
  // proto:  void QStyleOptionButton::NewQStyleOptionButton(const QStyleOptionButton & other);
  fn _ZN18QStyleOptionButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStyleOptionButton)=1
pub struct QStyleOptionButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionButton {
  pub fn NewQStyleOptionButton<T: QStyleOptionButton_NewQStyleOptionButton>(value: T) -> QStyleOptionButton {
    let rsthis = value.NewQStyleOptionButton();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionButton_NewQStyleOptionButton {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton;
}

// proto: void QStyleOptionButton::NewQStyleOptionButton(int version);
impl<'a> /*trait*/ QStyleOptionButton_NewQStyleOptionButton for (i32) {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionButtonC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionButton::NewQStyleOptionButton();
impl<'a> /*trait*/ QStyleOptionButton_NewQStyleOptionButton for () {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1Ev()};
    unsafe {_ZN18QStyleOptionButtonC1Ev(qthis)};
    let rsthis = QStyleOptionButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionButton::NewQStyleOptionButton(const QStyleOptionButton & other);
impl<'a> /*trait*/ QStyleOptionButton_NewQStyleOptionButton for (&'a  QStyleOptionButton) {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionButtonC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

