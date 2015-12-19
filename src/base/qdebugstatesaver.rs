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
  // proto:  void QDebugStateSaver::NewQDebugStateSaver(QDebug & dbg);
  fn _ZN16QDebugStateSaverC1ER6QDebug(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDebugStateSaver::NewQDebugStateSaver(const QDebugStateSaver & );
  fn _ZN16QDebugStateSaverC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDebugStateSaver::FreeQDebugStateSaver();
  fn _ZN16QDebugStateSaverD0Ev(qthis: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDebugStateSaverC1ERKS_(qthis, arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QDebugStateSaver::FreeQDebugStateSaver();
impl /*struct*/ QDebugStateSaver {
  pub fn FreeQDebugStateSaver<RetType, T: QDebugStateSaver_FreeQDebugStateSaver<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQDebugStateSaver(self);
    // return 1;
  }
}

pub trait QDebugStateSaver_FreeQDebugStateSaver<RetType> {
  fn FreeQDebugStateSaver(self , rsthis: &mut QDebugStateSaver) -> RetType;
}

// proto:  void QDebugStateSaver::FreeQDebugStateSaver();
impl<'a> /*trait*/ QDebugStateSaver_FreeQDebugStateSaver<()> for () {
  fn FreeQDebugStateSaver(self , rsthis: &mut QDebugStateSaver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverD0Ev()};
     unsafe {_ZN16QDebugStateSaverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

