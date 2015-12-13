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
  // proto:  void QStyleOptionToolBox::NewQStyleOptionToolBox();
  fn _ZN19QStyleOptionToolBoxC1Ev(qthis: *mut c_void) ;
  // proto:  void QStyleOptionToolBox::NewQStyleOptionToolBox(const QStyleOptionToolBox & other);
  fn _ZN19QStyleOptionToolBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyleOptionToolBox::NewQStyleOptionToolBox(int version);
  fn _ZN19QStyleOptionToolBoxC1Ei(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QStyleOptionToolBox)=1
pub struct QStyleOptionToolBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionToolBox {
  pub fn NewQStyleOptionToolBox<T: QStyleOptionToolBox_NewQStyleOptionToolBox>(value: T) -> QStyleOptionToolBox {
    let rsthis = value.NewQStyleOptionToolBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBox_NewQStyleOptionToolBox {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox;
}

// proto: void QStyleOptionToolBox::NewQStyleOptionToolBox();
impl<'a> /*trait*/ QStyleOptionToolBox_NewQStyleOptionToolBox for () {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1Ev()};
    unsafe {_ZN19QStyleOptionToolBoxC1Ev(qthis)};
    let rsthis = QStyleOptionToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionToolBox::NewQStyleOptionToolBox(const QStyleOptionToolBox & other);
impl<'a> /*trait*/ QStyleOptionToolBox_NewQStyleOptionToolBox for (&'a  QStyleOptionToolBox) {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionToolBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionToolBox::NewQStyleOptionToolBox(int version);
impl<'a> /*trait*/ QStyleOptionToolBox_NewQStyleOptionToolBox for (i32) {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionToolBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

