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
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
  fn _ZN21QStyleOptionFocusRectC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
  fn _ZN21QStyleOptionFocusRectC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
  fn _ZN21QStyleOptionFocusRectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionFocusRect)=1
pub struct QStyleOptionFocusRect {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
impl /*struct*/ QStyleOptionFocusRect {
  pub fn NewQStyleOptionFocusRect<T: QStyleOptionFocusRect_NewQStyleOptionFocusRect>(value: T) -> QStyleOptionFocusRect {
    let rsthis = value.NewQStyleOptionFocusRect();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFocusRect_NewQStyleOptionFocusRect {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect;
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
impl<'a> /*trait*/ QStyleOptionFocusRect_NewQStyleOptionFocusRect for (i32) {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QStyleOptionFocusRectC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
impl<'a> /*trait*/ QStyleOptionFocusRect_NewQStyleOptionFocusRect for () {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1Ev()};
    unsafe {_ZN21QStyleOptionFocusRectC1Ev(qthis)};
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
impl<'a> /*trait*/ QStyleOptionFocusRect_NewQStyleOptionFocusRect for (QStyleOptionFocusRect) {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QStyleOptionFocusRectC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

