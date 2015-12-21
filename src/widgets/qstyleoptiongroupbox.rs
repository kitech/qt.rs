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
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
  fn _ZN20QStyleOptionGroupBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
  fn _ZN20QStyleOptionGroupBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
  fn _ZN20QStyleOptionGroupBoxC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionGroupBox)=1
pub struct QStyleOptionGroupBox {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
impl /*struct*/ QStyleOptionGroupBox {
  pub fn NewQStyleOptionGroupBox<T: QStyleOptionGroupBox_NewQStyleOptionGroupBox>(value: T) -> QStyleOptionGroupBox {
    let rsthis = value.NewQStyleOptionGroupBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGroupBox_NewQStyleOptionGroupBox {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox;
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
impl<'a> /*trait*/ QStyleOptionGroupBox_NewQStyleOptionGroupBox for (i32) {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionGroupBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
impl<'a> /*trait*/ QStyleOptionGroupBox_NewQStyleOptionGroupBox for (QStyleOptionGroupBox) {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionGroupBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
impl<'a> /*trait*/ QStyleOptionGroupBox_NewQStyleOptionGroupBox for () {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1Ev()};
    unsafe {_ZN20QStyleOptionGroupBoxC1Ev(qthis)};
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

