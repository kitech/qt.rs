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
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
  fn _ZN19QStyleOptionToolBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(int version);
  fn _ZN19QStyleOptionToolBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
  fn _ZN19QStyleOptionToolBarC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionToolBar)=1
pub struct QStyleOptionToolBar {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl /*struct*/ QStyleOptionToolBar {
  pub fn NewQStyleOptionToolBar<T: QStyleOptionToolBar_NewQStyleOptionToolBar>(value: T) -> QStyleOptionToolBar {
    let rsthis = value.NewQStyleOptionToolBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBar_NewQStyleOptionToolBar {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar;
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl<'a> /*trait*/ QStyleOptionToolBar_NewQStyleOptionToolBar for (QStyleOptionToolBar) {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionToolBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(int version);
impl<'a> /*trait*/ QStyleOptionToolBar_NewQStyleOptionToolBar for (i32) {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionToolBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
impl<'a> /*trait*/ QStyleOptionToolBar_NewQStyleOptionToolBar for () {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1Ev()};
    unsafe {_ZN19QStyleOptionToolBarC1Ev(qthis)};
    let rsthis = QStyleOptionToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

