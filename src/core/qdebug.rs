// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qdebug.h
// dst-file: /src/core/qdebug.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
// use super::qdebug::QDebug; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNoDebug_Class_Size() -> c_int;
  fn QDebugStateSaver_Class_Size() -> c_int;
  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
  fn dector_ZN16QDebugStateSaverC1ER6QDebug(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QDebugStateSaverC1ER6QDebug(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebugStateSaver::QDebugStateSaver(const QDebugStateSaver & );
  fn dector_ZN16QDebugStateSaverC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QDebugStateSaverC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebugStateSaver::~QDebugStateSaver();
  fn _ZN16QDebugStateSaverD0Ev(qthis: *mut c_void);
  fn QDebug_Class_Size() -> c_int;
  // proto:  QDebug & QDebug::resetFormat();
  fn _ZN6QDebug11resetFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::~QDebug();
  fn _ZN6QDebugD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNoDebug)=1
pub struct QNoDebug {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDebugStateSaver)=1
pub struct QDebugStateSaver {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDebug)=8
pub struct QDebug {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QNoDebug {
  pub fn inheritFrom(qthis: *mut c_void) -> QNoDebug {
    return QNoDebug{qclsinst: qthis};
  }
}
impl /*struct*/ QDebugStateSaver {
  pub fn inheritFrom(qthis: *mut c_void) -> QDebugStateSaver {
    return QDebugStateSaver{qclsinst: qthis};
  }
}
  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
impl /*struct*/ QDebugStateSaver {
  pub fn New<T: QDebugStateSaver_New>(value: T) -> QDebugStateSaver {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDebugStateSaver_New {
  fn New(self) -> QDebugStateSaver;
}

  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
impl<'a> /*trait*/ QDebugStateSaver_New for (&'a QDebug) {
  fn New(self) -> QDebugStateSaver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ER6QDebug()};
    let ctysz: c_int = unsafe{QDebugStateSaver_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QDebugStateSaverC1ER6QDebug(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QDebugStateSaverC1ER6QDebug(arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebugStateSaver::QDebugStateSaver(const QDebugStateSaver & );
impl<'a> /*trait*/ QDebugStateSaver_New for (&'a QDebugStateSaver) {
  fn New(self) -> QDebugStateSaver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ERKS_()};
    let ctysz: c_int = unsafe{QDebugStateSaver_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QDebugStateSaverC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QDebugStateSaverC1ERKS_(arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebugStateSaver::~QDebugStateSaver();
impl /*struct*/ QDebugStateSaver {
  pub fn Free<RetType, T: QDebugStateSaver_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDebugStateSaver_Free<RetType> {
  fn Free(self , rsthis: & QDebugStateSaver) -> RetType;
}

  // proto:  void QDebugStateSaver::~QDebugStateSaver();
impl<'a> /*trait*/ QDebugStateSaver_Free<()> for () {
  fn Free(self , rsthis: & QDebugStateSaver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverD0Ev()};
     unsafe {_ZN16QDebugStateSaverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn inheritFrom(qthis: *mut c_void) -> QDebug {
    return QDebug{qclsinst: qthis};
  }
}
  // proto:  QDebug & QDebug::resetFormat();
impl /*struct*/ QDebug {
  pub fn resetFormat<RetType, T: QDebug_resetFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetFormat(self);
    // return 1;
  }
}

pub trait QDebug_resetFormat<RetType> {
  fn resetFormat(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::resetFormat();
impl<'a> /*trait*/ QDebug_resetFormat<QDebug> for () {
  fn resetFormat(self , rsthis: & QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug11resetFormatEv()};
    let mut ret = unsafe {_ZN6QDebug11resetFormatEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDebug::~QDebug();
impl /*struct*/ QDebug {
  pub fn Free<RetType, T: QDebug_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDebug_Free<RetType> {
  fn Free(self , rsthis: & QDebug) -> RetType;
}

  // proto:  void QDebug::~QDebug();
impl<'a> /*trait*/ QDebug_Free<()> for () {
  fn Free(self , rsthis: & QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugD0Ev()};
     unsafe {_ZN6QDebugD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

