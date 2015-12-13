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
  // proto: void QStyleOptionSizeGrip::NewQStyleOptionSizeGrip(int version);
  fn _ZN20QStyleOptionSizeGripC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QStyleOptionSizeGrip::NewQStyleOptionSizeGrip();
  fn _ZN20QStyleOptionSizeGripC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QStyleOptionSizeGrip::NewQStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
  fn _ZN20QStyleOptionSizeGripC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStyleOptionSizeGrip)=1
pub struct QStyleOptionSizeGrip {
  pub qclsinst: *mut c_void,
}

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

// proto: void QStyleOptionSizeGrip::NewQStyleOptionSizeGrip(int version);
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

// proto: void QStyleOptionSizeGrip::NewQStyleOptionSizeGrip();
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

// proto: void QStyleOptionSizeGrip::NewQStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for (&'a  QStyleOptionSizeGrip) {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QStyleOptionSizeGripC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

