// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qstate.h
// dst-file: /src/core/qstate.rs
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
use super::qobject::QObject; // 773
use super::qvariant::QVariant; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QAbstractState * QState::errorState();
  fn _ZNK6QState10errorStateEv(qthis: *mut c_void);
  // proto:  QAbstractState * QState::initialState();
  fn _ZNK6QState12initialStateEv(qthis: *mut c_void);
  // proto:  void QState::~QState();
  fn _ZN6QStateD0Ev(qthis: *mut c_void);
  // proto:  void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
  fn _ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void);
  // proto:  void QState::QState(const QState & );
  fn _ZN6QStateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QState::QState(QState * parent);
  fn _ZN6QStateC1EPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QState::metaObject();
  fn _ZNK6QState10metaObjectEv(qthis: *mut c_void);
  // proto:  QList<QAbstractTransition *> QState::transitions();
  fn _ZNK6QState11transitionsEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QState)=1
pub struct QState {
  qbase: QAbstractState,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QState {
  pub fn inheritFrom(qthis: *mut c_void) -> QState {
    return QState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QState {
  type Target = QAbstractState;

  fn deref(&self) -> &QAbstractState {
    return &self.qbase;
  }
}
impl AsRef<QAbstractState> for QState {
  fn as_ref(&self) -> &QAbstractState {
    return &self.qbase;
  }
}
  // proto:  QAbstractState * QState::errorState();
impl /*struct*/ QState {
  pub fn errorState<RetType, T: QState_errorState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.errorState(self);
    // return 1;
  }
}

pub trait QState_errorState<RetType> {
  fn errorState(self , rsthis: &mut QState) -> RetType;
}

  // proto:  QAbstractState * QState::errorState();
impl<'a> /*trait*/ QState_errorState<()> for () {
  fn errorState(self , rsthis: &mut QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState10errorStateEv()};
     unsafe {_ZNK6QState10errorStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractState * QState::initialState();
impl /*struct*/ QState {
  pub fn initialState<RetType, T: QState_initialState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.initialState(self);
    // return 1;
  }
}

pub trait QState_initialState<RetType> {
  fn initialState(self , rsthis: &mut QState) -> RetType;
}

  // proto:  QAbstractState * QState::initialState();
impl<'a> /*trait*/ QState_initialState<()> for () {
  fn initialState(self , rsthis: &mut QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState12initialStateEv()};
     unsafe {_ZNK6QState12initialStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QState::~QState();
impl /*struct*/ QState {
  pub fn FreeQState<RetType, T: QState_FreeQState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQState(self);
    // return 1;
  }
}

pub trait QState_FreeQState<RetType> {
  fn FreeQState(self , rsthis: &mut QState) -> RetType;
}

  // proto:  void QState::~QState();
impl<'a> /*trait*/ QState_FreeQState<()> for () {
  fn FreeQState(self , rsthis: &mut QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateD0Ev()};
     unsafe {_ZN6QStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
impl /*struct*/ QState {
  pub fn assignProperty<RetType, T: QState_assignProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.assignProperty(self);
    // return 1;
  }
}

pub trait QState_assignProperty<RetType> {
  fn assignProperty(self , rsthis: &mut QState) -> RetType;
}

  // proto:  void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
impl<'a> /*trait*/ QState_assignProperty<()> for (QObject, &'a  String, QVariant) {
  fn assignProperty(self , rsthis: &mut QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QState::QState(const QState & );
impl /*struct*/ QState {
  pub fn NewQState<T: QState_NewQState>(value: T) -> QState {
    let rsthis = value.NewQState();
    return rsthis;
    // return 1;
  }
}

pub trait QState_NewQState {
  fn NewQState(self) -> QState;
}

  // proto:  void QState::QState(const QState & );
impl<'a> /*trait*/ QState_NewQState for (QState) {
  fn NewQState(self) -> QState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QStateC1ERKS_(qthis, arg0)};
    let rsthis = QState{/**/qbase: QAbstractState::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QState::metaObject();
impl /*struct*/ QState {
  pub fn metaObject<RetType, T: QState_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QState_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QState) -> RetType;
}

  // proto:  const QMetaObject * QState::metaObject();
impl<'a> /*trait*/ QState_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState10metaObjectEv()};
     unsafe {_ZNK6QState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QAbstractTransition *> QState::transitions();
impl /*struct*/ QState {
  pub fn transitions<RetType, T: QState_transitions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transitions(self);
    // return 1;
  }
}

pub trait QState_transitions<RetType> {
  fn transitions(self , rsthis: &mut QState) -> RetType;
}

  // proto:  QList<QAbstractTransition *> QState::transitions();
impl<'a> /*trait*/ QState_transitions<()> for () {
  fn transitions(self , rsthis: &mut QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState11transitionsEv()};
     unsafe {_ZNK6QState11transitionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

