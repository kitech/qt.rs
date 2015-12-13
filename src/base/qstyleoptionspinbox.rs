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
  // proto: void QStyleOptionSpinBox::NewQStyleOptionSpinBox();
  fn _ZN19QStyleOptionSpinBoxC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QStyleOptionSpinBox::NewQStyleOptionSpinBox(const QStyleOptionSpinBox & other);
  fn _ZN19QStyleOptionSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStyleOptionSpinBox::NewQStyleOptionSpinBox(int version);
  fn _ZN19QStyleOptionSpinBoxC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QStyleOptionSpinBox)=1
pub struct QStyleOptionSpinBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionSpinBox {
  pub fn NewQStyleOptionSpinBox<T: QStyleOptionSpinBox_NewQStyleOptionSpinBox>(value: T) -> QStyleOptionSpinBox {
    let rsthis = value.NewQStyleOptionSpinBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSpinBox_NewQStyleOptionSpinBox {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox;
}

// proto: void QStyleOptionSpinBox::NewQStyleOptionSpinBox();
impl<'a> /*trait*/ QStyleOptionSpinBox_NewQStyleOptionSpinBox for () {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1Ev()};
    unsafe {_ZN19QStyleOptionSpinBoxC1Ev(qthis)};
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionSpinBox::NewQStyleOptionSpinBox(const QStyleOptionSpinBox & other);
impl<'a> /*trait*/ QStyleOptionSpinBox_NewQStyleOptionSpinBox for (&'a  QStyleOptionSpinBox) {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QStyleOptionSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionSpinBox::NewQStyleOptionSpinBox(int version);
impl<'a> /*trait*/ QStyleOptionSpinBox_NewQStyleOptionSpinBox for (i32) {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionSpinBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

