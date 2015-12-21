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
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
  fn _ZN23QStyleOptionProgressBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(int version);
  fn _ZN23QStyleOptionProgressBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
  fn _ZN23QStyleOptionProgressBarC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionProgressBar)=1
pub struct QStyleOptionProgressBar {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl /*struct*/ QStyleOptionProgressBar {
  pub fn NewQStyleOptionProgressBar<T: QStyleOptionProgressBar_NewQStyleOptionProgressBar>(value: T) -> QStyleOptionProgressBar {
    let rsthis = value.NewQStyleOptionProgressBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionProgressBar_NewQStyleOptionProgressBar {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar;
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl<'a> /*trait*/ QStyleOptionProgressBar_NewQStyleOptionProgressBar for (QStyleOptionProgressBar) {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QStyleOptionProgressBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(int version);
impl<'a> /*trait*/ QStyleOptionProgressBar_NewQStyleOptionProgressBar for (i32) {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN23QStyleOptionProgressBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
impl<'a> /*trait*/ QStyleOptionProgressBar_NewQStyleOptionProgressBar for () {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1Ev()};
    unsafe {_ZN23QStyleOptionProgressBarC1Ev(qthis)};
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

