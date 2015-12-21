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
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
  fn _ZN20QStyleOptionSizeGripC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
  fn _ZN20QStyleOptionSizeGripC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
  fn _ZN20QStyleOptionSizeGripC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionSizeGrip)=1
pub struct QStyleOptionSizeGrip {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
impl /*struct*/ QStyleOptionSizeGrip {
  pub fn NewQStyleOptionSizeGrip<T: QStyleOptionSizeGrip_NewQStyleOptionSizeGrip>(value: T) -> QStyleOptionSizeGrip {
    let rsthis = value.NewQStyleOptionSizeGrip();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSizeGrip_NewQStyleOptionSizeGrip {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip;
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for (i32) {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionSizeGripC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for () {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1Ev()};
    unsafe {_ZN20QStyleOptionSizeGripC1Ev(qthis)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for (QStyleOptionSizeGrip) {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionSizeGripC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

