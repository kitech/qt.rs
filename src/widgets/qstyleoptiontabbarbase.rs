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
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
  fn _ZN22QStyleOptionTabBarBaseC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(int version);
  fn _ZN22QStyleOptionTabBarBaseC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
  fn _ZN22QStyleOptionTabBarBaseC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionTabBarBase)=1
pub struct QStyleOptionTabBarBase {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl /*struct*/ QStyleOptionTabBarBase {
  pub fn NewQStyleOptionTabBarBase<T: QStyleOptionTabBarBase_NewQStyleOptionTabBarBase>(value: T) -> QStyleOptionTabBarBase {
    let rsthis = value.NewQStyleOptionTabBarBase();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabBarBase_NewQStyleOptionTabBarBase {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase;
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl<'a> /*trait*/ QStyleOptionTabBarBase_NewQStyleOptionTabBarBase for () {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1Ev()};
    unsafe {_ZN22QStyleOptionTabBarBaseC1Ev(qthis)};
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(int version);
impl<'a> /*trait*/ QStyleOptionTabBarBase_NewQStyleOptionTabBarBase for (i32) {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionTabBarBaseC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
impl<'a> /*trait*/ QStyleOptionTabBarBase_NewQStyleOptionTabBarBase for (QStyleOptionTabBarBase) {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionTabBarBaseC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

