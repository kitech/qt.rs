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
  // proto: void QStyleOptionTabWidgetFrame::NewQStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
  fn _ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStyleOptionTabWidgetFrame::NewQStyleOptionTabWidgetFrame(int version);
  fn _ZN26QStyleOptionTabWidgetFrameC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QStyleOptionTabWidgetFrame::NewQStyleOptionTabWidgetFrame();
  fn _ZN26QStyleOptionTabWidgetFrameC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QStyleOptionTabWidgetFrame)=1
pub struct QStyleOptionTabWidgetFrame {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn NewQStyleOptionTabWidgetFrame<T: QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.NewQStyleOptionTabWidgetFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame;
}

// proto: void QStyleOptionTabWidgetFrame::NewQStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for (&'a  QStyleOptionTabWidgetFrame) {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionTabWidgetFrame::NewQStyleOptionTabWidgetFrame(int version);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for (i32) {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionTabWidgetFrame::NewQStyleOptionTabWidgetFrame();
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for () {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ev()};
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ev(qthis)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

