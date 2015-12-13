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
  // proto:  void QStyleOptionMenuItem::NewQStyleOptionMenuItem(const QStyleOptionMenuItem & other);
  fn _ZN20QStyleOptionMenuItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyleOptionMenuItem::NewQStyleOptionMenuItem();
  fn _ZN20QStyleOptionMenuItemC1Ev(qthis: *mut c_void) ;
  // proto:  void QStyleOptionMenuItem::NewQStyleOptionMenuItem(int version);
  fn _ZN20QStyleOptionMenuItemC1Ei(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QStyleOptionMenuItem)=1
pub struct QStyleOptionMenuItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionMenuItem {
  pub fn NewQStyleOptionMenuItem<T: QStyleOptionMenuItem_NewQStyleOptionMenuItem>(value: T) -> QStyleOptionMenuItem {
    let rsthis = value.NewQStyleOptionMenuItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionMenuItem_NewQStyleOptionMenuItem {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem;
}

// proto: void QStyleOptionMenuItem::NewQStyleOptionMenuItem(const QStyleOptionMenuItem & other);
impl<'a> /*trait*/ QStyleOptionMenuItem_NewQStyleOptionMenuItem for (&'a  QStyleOptionMenuItem) {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionMenuItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionMenuItem::NewQStyleOptionMenuItem();
impl<'a> /*trait*/ QStyleOptionMenuItem_NewQStyleOptionMenuItem for () {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1Ev()};
    unsafe {_ZN20QStyleOptionMenuItemC1Ev(qthis)};
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionMenuItem::NewQStyleOptionMenuItem(int version);
impl<'a> /*trait*/ QStyleOptionMenuItem_NewQStyleOptionMenuItem for (i32) {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionMenuItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

