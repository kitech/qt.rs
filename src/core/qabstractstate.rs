// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtCore/qabstractstate.h
// dst-file: /src/core/qabstractstate.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstate::QState; // 773
use super::qstatemachine::QStateMachine; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QAbstractState::~QAbstractState();
  fn _ZN14QAbstractStateD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractState::QAbstractState(QState * parent);
  fn _ZN14QAbstractStateC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractState::metaObject();
  fn _ZNK14QAbstractState10metaObjectEv(qthis: *mut c_void);
  // proto:  QState * QAbstractState::parentState();
  fn _ZNK14QAbstractState11parentStateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStateMachine * QAbstractState::machine();
  fn _ZNK14QAbstractState7machineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractState::activeChanged(bool active);
  fn _ZN14QAbstractState13activeChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QAbstractState::active();
  fn _ZNK14QAbstractState6activeEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractState)=1
pub struct QAbstractState {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractState {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractState {
    return QAbstractState{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAbstractState {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractState {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAbstractState::~QAbstractState();
impl /*struct*/ QAbstractState {
  pub fn Free<RetType, T: QAbstractState_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractState_Free<RetType> {
  fn Free(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  void QAbstractState::~QAbstractState();
impl<'a> /*trait*/ QAbstractState_Free<()> for () {
  fn Free(self , rsthis: & QAbstractState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAbstractStateD0Ev()};
     unsafe {_ZN14QAbstractStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractState::QAbstractState(QState * parent);
impl /*struct*/ QAbstractState {
  pub fn New<T: QAbstractState_New>(value: T) -> QAbstractState {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractState_New {
  fn New(self) -> QAbstractState;
}

  // proto:  void QAbstractState::QAbstractState(QState * parent);
impl<'a> /*trait*/ QAbstractState_New for (&'a QState) {
  fn New(self) -> QAbstractState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAbstractStateC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QAbstractStateC1EP6QState(qthis, arg0)};
    let rsthis = QAbstractState{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractState::metaObject();
impl /*struct*/ QAbstractState {
  pub fn metaObject<RetType, T: QAbstractState_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractState_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  const QMetaObject * QAbstractState::metaObject();
impl<'a> /*trait*/ QAbstractState_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState10metaObjectEv()};
     unsafe {_ZNK14QAbstractState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QState * QAbstractState::parentState();
impl /*struct*/ QAbstractState {
  pub fn parentState<RetType, T: QAbstractState_parentState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentState(self);
    // return 1;
  }
}

pub trait QAbstractState_parentState<RetType> {
  fn parentState(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  QState * QAbstractState::parentState();
impl<'a> /*trait*/ QAbstractState_parentState<QState> for () {
  fn parentState(self , rsthis: & QAbstractState) -> QState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState11parentStateEv()};
    let mut ret = unsafe {_ZNK14QAbstractState11parentStateEv(rsthis.qclsinst)};
    let mut ret1 = QState::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QStateMachine * QAbstractState::machine();
impl /*struct*/ QAbstractState {
  pub fn machine<RetType, T: QAbstractState_machine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.machine(self);
    // return 1;
  }
}

pub trait QAbstractState_machine<RetType> {
  fn machine(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  QStateMachine * QAbstractState::machine();
impl<'a> /*trait*/ QAbstractState_machine<QStateMachine> for () {
  fn machine(self , rsthis: & QAbstractState) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState7machineEv()};
    let mut ret = unsafe {_ZNK14QAbstractState7machineEv(rsthis.qclsinst)};
    let mut ret1 = QStateMachine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractState::activeChanged(bool active);
impl /*struct*/ QAbstractState {
  pub fn activeChanged<RetType, T: QAbstractState_activeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeChanged(self);
    // return 1;
  }
}

pub trait QAbstractState_activeChanged<RetType> {
  fn activeChanged(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  void QAbstractState::activeChanged(bool active);
impl<'a> /*trait*/ QAbstractState_activeChanged<()> for (i8) {
  fn activeChanged(self , rsthis: & QAbstractState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAbstractState13activeChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QAbstractState13activeChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractState::active();
impl /*struct*/ QAbstractState {
  pub fn active<RetType, T: QAbstractState_active<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.active(self);
    // return 1;
  }
}

pub trait QAbstractState_active<RetType> {
  fn active(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  bool QAbstractState::active();
impl<'a> /*trait*/ QAbstractState_active<i8> for () {
  fn active(self , rsthis: & QAbstractState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState6activeEv()};
    let mut ret = unsafe {_ZNK14QAbstractState6activeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

