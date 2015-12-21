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
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
  fn _ZN20QStyleOptionTitleBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
  fn _ZN20QStyleOptionTitleBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
  fn _ZN20QStyleOptionTitleBarC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionTitleBar)=1
pub struct QStyleOptionTitleBar {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
impl /*struct*/ QStyleOptionTitleBar {
  pub fn NewQStyleOptionTitleBar<T: QStyleOptionTitleBar_NewQStyleOptionTitleBar>(value: T) -> QStyleOptionTitleBar {
    let rsthis = value.NewQStyleOptionTitleBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTitleBar_NewQStyleOptionTitleBar {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar;
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
impl<'a> /*trait*/ QStyleOptionTitleBar_NewQStyleOptionTitleBar for (i32) {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionTitleBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
impl<'a> /*trait*/ QStyleOptionTitleBar_NewQStyleOptionTitleBar for (QStyleOptionTitleBar) {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionTitleBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
impl<'a> /*trait*/ QStyleOptionTitleBar_NewQStyleOptionTitleBar for () {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1Ev()};
    unsafe {_ZN20QStyleOptionTitleBarC1Ev(qthis)};
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

