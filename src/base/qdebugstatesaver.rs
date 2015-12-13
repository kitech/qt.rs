// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdebug::QDebug;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN16QDebugStateSaverC1ER6QDebug(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN16QDebugStateSaverC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN16QDebugStateSaverD0Ev() -> i32;
}

// body block begin
// class sizeof(QDebugStateSaver)=1
pub struct QDebugStateSaver {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDebugStateSaver {
  pub fn NewQDebugStateSaver<T: QDebugStateSaver_NewQDebugStateSaver>(value: T) -> QDebugStateSaver {
    let rsthis = value.NewQDebugStateSaver();
    return rsthis;
    // return 1;
  }
}

pub trait QDebugStateSaver_NewQDebugStateSaver {
  fn NewQDebugStateSaver(self) -> QDebugStateSaver;
}

// proto: void QDebugStateSaver::NewQDebugStateSaver(QDebug & dbg);
impl<'a> /*trait*/ QDebugStateSaver_NewQDebugStateSaver for (&'a mut QDebug) {
  fn NewQDebugStateSaver(self) -> QDebugStateSaver {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ER6QDebug()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDebugStateSaverC1ER6QDebug(qthis, arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QDebugStateSaver::NewQDebugStateSaver(const QDebugStateSaver & );
impl<'a> /*trait*/ QDebugStateSaver_NewQDebugStateSaver for (&'a  QDebugStateSaver) {
  fn NewQDebugStateSaver(self) -> QDebugStateSaver {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QDebugStateSaverC1ERKS_(qthis, arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDebugStateSaver {
  pub fn FreeQDebugStateSaver<T: QDebugStateSaver_FreeQDebugStateSaver>(&mut self, value: T) -> i32 {
    value.FreeQDebugStateSaver(self);
    return 1;
  }
}

pub trait QDebugStateSaver_FreeQDebugStateSaver {
  fn FreeQDebugStateSaver(self, this: &mut QDebugStateSaver) -> i32;
}

// proto: void QDebugStateSaver::FreeQDebugStateSaver();
impl<'a> /*trait*/ QDebugStateSaver_FreeQDebugStateSaver for () {
  fn FreeQDebugStateSaver(self, this: &mut QDebugStateSaver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverD0Ev()};
    unsafe {_ZN16QDebugStateSaverD0Ev()};
    return 1;
  }
}

