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
  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
  fn _ZN15QStyleOptionTabC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTab::QStyleOptionTab(int version);
  fn _ZN15QStyleOptionTabC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTab::QStyleOptionTab();
  fn _ZN15QStyleOptionTabC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionTab)=1
pub struct QStyleOptionTab {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl /*struct*/ QStyleOptionTab {
  pub fn NewQStyleOptionTab<T: QStyleOptionTab_NewQStyleOptionTab>(value: T) -> QStyleOptionTab {
    let rsthis = value.NewQStyleOptionTab();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTab_NewQStyleOptionTab {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab;
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl<'a> /*trait*/ QStyleOptionTab_NewQStyleOptionTab for (QStyleOptionTab) {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QStyleOptionTabC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTab{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(int version);
impl<'a> /*trait*/ QStyleOptionTab_NewQStyleOptionTab for (i32) {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QStyleOptionTabC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTab{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab();
impl<'a> /*trait*/ QStyleOptionTab_NewQStyleOptionTab for () {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1Ev()};
    unsafe {_ZN15QStyleOptionTabC1Ev(qthis)};
    let rsthis = QStyleOptionTab{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

