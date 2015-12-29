// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qfinalstate.h
// dst-file: /src/core/qfinalstate.rs
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
use super::qabstractstate::QAbstractState; // 773
use std::ops::Deref;
use super::qstate::QState; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFinalState_Class_Size() -> c_int;
  // proto:  void QFinalState::QFinalState(QState * parent);
  fn dector_ZN11QFinalStateC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFinalStateC1EP6QState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFinalState::QFinalState(const QFinalState & );
  fn dector_ZN11QFinalStateC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFinalStateC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFinalState::~QFinalState();
  fn _ZN11QFinalStateD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QFinalState::metaObject();
  fn _ZNK11QFinalState10metaObjectEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QFinalState)=1
#[derive(Default)]
pub struct QFinalState {
  qbase: QAbstractState,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFinalState {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFinalState {
    return QFinalState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFinalState {
  type Target = QAbstractState;

  fn deref(&self) -> &QAbstractState {
    return & self.qbase;
  }
}
impl AsRef<QAbstractState> for QFinalState {
  fn as_ref(& self) -> & QAbstractState {
    return & self.qbase;
  }
}
  // proto:  void QFinalState::QFinalState(QState * parent);
impl /*struct*/ QFinalState {
  pub fn New<T: QFinalState_New>(value: T) -> QFinalState {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFinalState_New {
  fn New(self) -> QFinalState;
}

  // proto:  void QFinalState::QFinalState(QState * parent);
impl<'a> /*trait*/ QFinalState_New for (&'a QState) {
  fn New(self) -> QFinalState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC1EP6QState()};
    let ctysz: c_int = unsafe{QFinalState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFinalStateC1EP6QState(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QFinalStateC1EP6QState(arg0)} as u64;
    let rsthis = QFinalState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFinalState::QFinalState(const QFinalState & );
impl<'a> /*trait*/ QFinalState_New for (&'a QFinalState) {
  fn New(self) -> QFinalState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC1ERKS_()};
    let ctysz: c_int = unsafe{QFinalState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFinalStateC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QFinalStateC1ERKS_(arg0)} as u64;
    let rsthis = QFinalState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFinalState::~QFinalState();
impl /*struct*/ QFinalState {
  pub fn Free<RetType, T: QFinalState_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFinalState_Free<RetType> {
  fn Free(self , rsthis: & QFinalState) -> RetType;
}

  // proto:  void QFinalState::~QFinalState();
impl<'a> /*trait*/ QFinalState_Free<()> for () {
  fn Free(self , rsthis: & QFinalState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateD0Ev()};
     unsafe {_ZN11QFinalStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFinalState::metaObject();
impl /*struct*/ QFinalState {
  pub fn metaObject<RetType, T: QFinalState_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFinalState_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFinalState) -> RetType;
}

  // proto:  const QMetaObject * QFinalState::metaObject();
impl<'a> /*trait*/ QFinalState_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFinalState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFinalState10metaObjectEv()};
     unsafe {_ZNK11QFinalState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

