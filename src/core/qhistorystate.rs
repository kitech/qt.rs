// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtCore/qhistorystate.h
// dst-file: /src/core/qhistorystate.rs
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
  // proto:  const QMetaObject * QHistoryState::metaObject();
  fn _ZNK13QHistoryState10metaObjectEv(qthis: *mut c_void);
  // proto:  void QHistoryState::QHistoryState(const QHistoryState & );
  fn _ZN13QHistoryStateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QHistoryState::QHistoryState(QState * parent);
  fn _ZN13QHistoryStateC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAbstractState * QHistoryState::defaultState();
  fn _ZNK13QHistoryState12defaultStateEv(qthis: *mut c_void);
  // proto:  void QHistoryState::~QHistoryState();
  fn _ZN13QHistoryStateD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QHistoryState)=1
pub struct QHistoryState {
  qbase: QAbstractState,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHistoryState {
  pub fn inheritFrom(qthis: *mut c_void) -> QHistoryState {
    return QHistoryState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QHistoryState {
  type Target = QAbstractState;

  fn deref(&self) -> &QAbstractState {
    return & self.qbase;
  }
}
impl AsRef<QAbstractState> for QHistoryState {
  fn as_ref(& self) -> & QAbstractState {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QHistoryState::metaObject();
impl /*struct*/ QHistoryState {
  pub fn metaObject<RetType, T: QHistoryState_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QHistoryState_metaObject<RetType> {
  fn metaObject(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  const QMetaObject * QHistoryState::metaObject();
impl<'a> /*trait*/ QHistoryState_metaObject<()> for () {
  fn metaObject(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QHistoryState10metaObjectEv()};
     unsafe {_ZNK13QHistoryState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHistoryState::QHistoryState(const QHistoryState & );
impl /*struct*/ QHistoryState {
  pub fn New<T: QHistoryState_New>(value: T) -> QHistoryState {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QHistoryState_New {
  fn New(self) -> QHistoryState;
}

  // proto:  void QHistoryState::QHistoryState(const QHistoryState & );
impl<'a> /*trait*/ QHistoryState_New for (&'a QHistoryState) {
  fn New(self) -> QHistoryState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QHistoryStateC1ERKS_(qthis, arg0)};
    let rsthis = QHistoryState{/**/qbase: QAbstractState::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHistoryState::QHistoryState(QState * parent);
impl<'a> /*trait*/ QHistoryState_New for (&'a QState) {
  fn New(self) -> QHistoryState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QHistoryStateC1EP6QState(qthis, arg0)};
    let rsthis = QHistoryState{/**/qbase: QAbstractState::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAbstractState * QHistoryState::defaultState();
impl /*struct*/ QHistoryState {
  pub fn defaultState<RetType, T: QHistoryState_defaultState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultState(self);
    // return 1;
  }
}

pub trait QHistoryState_defaultState<RetType> {
  fn defaultState(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  QAbstractState * QHistoryState::defaultState();
impl<'a> /*trait*/ QHistoryState_defaultState<()> for () {
  fn defaultState(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QHistoryState12defaultStateEv()};
     unsafe {_ZNK13QHistoryState12defaultStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHistoryState::~QHistoryState();
impl /*struct*/ QHistoryState {
  pub fn Free<RetType, T: QHistoryState_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QHistoryState_Free<RetType> {
  fn Free(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  void QHistoryState::~QHistoryState();
impl<'a> /*trait*/ QHistoryState_Free<()> for () {
  fn Free(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateD0Ev()};
     unsafe {_ZN13QHistoryStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

