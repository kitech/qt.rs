// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::qabstractstate::*; // 773
use std::ops::Deref;
use super::qstate::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFinalState_Class_Size() -> c_int;
  // proto:  void QFinalState::QFinalState(QState * parent);
  fn C_ZN11QFinalStateC2EP6QState(arg0: *mut c_void) -> u64;
  // proto:  void QFinalState::~QFinalState();
  fn C_ZN11QFinalStateD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QFinalState::metaObject();
  fn C_ZNK11QFinalState10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
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
  pub fn new<T: QFinalState_new>(value: T) -> QFinalState {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFinalState_new {
  fn new(self) -> QFinalState;
}

  // proto:  void QFinalState::QFinalState(QState * parent);
impl<'a> /*trait*/ QFinalState_new for (&'a QState) {
  fn new(self) -> QFinalState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC2EP6QState()};
    let ctysz: c_int = unsafe{QFinalState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QFinalStateC2EP6QState(arg0)};
    let rsthis = QFinalState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFinalState::~QFinalState();
impl /*struct*/ QFinalState {
  pub fn free<RetType, T: QFinalState_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFinalState_free<RetType> {
  fn free(self , rsthis: & QFinalState) -> RetType;
}

  // proto:  void QFinalState::~QFinalState();
impl<'a> /*trait*/ QFinalState_free<()> for () {
  fn free(self , rsthis: & QFinalState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateD2Ev()};
     unsafe {C_ZN11QFinalStateD2Ev(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QFinalState_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QFinalState) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFinalState10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QFinalState10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

