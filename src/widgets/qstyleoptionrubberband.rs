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
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
  fn _ZN22QStyleOptionRubberBandC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
  fn _ZN22QStyleOptionRubberBandC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
  fn _ZN22QStyleOptionRubberBandC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionRubberBand)=1
pub struct QStyleOptionRubberBand {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
impl /*struct*/ QStyleOptionRubberBand {
  pub fn NewQStyleOptionRubberBand<T: QStyleOptionRubberBand_NewQStyleOptionRubberBand>(value: T) -> QStyleOptionRubberBand {
    let rsthis = value.NewQStyleOptionRubberBand();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionRubberBand_NewQStyleOptionRubberBand {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand;
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
impl<'a> /*trait*/ QStyleOptionRubberBand_NewQStyleOptionRubberBand for (i32) {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionRubberBandC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
impl<'a> /*trait*/ QStyleOptionRubberBand_NewQStyleOptionRubberBand for () {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1Ev()};
    unsafe {_ZN22QStyleOptionRubberBandC1Ev(qthis)};
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
impl<'a> /*trait*/ QStyleOptionRubberBand_NewQStyleOptionRubberBand for (QStyleOptionRubberBand) {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionRubberBandC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

