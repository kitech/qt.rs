// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtCore/qabstracttransition.h
// dst-file: /src/core/qabstracttransition.rs
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractTransition_Class_Size() -> c_int;
  // proto:  QAbstractState * QAbstractTransition::targetState();
  fn _ZNK19QAbstractTransition11targetStateEv(qthis: *mut c_void);
  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
  fn _ZNK19QAbstractTransition12targetStatesEv(qthis: *mut c_void);
  // proto:  QState * QAbstractTransition::sourceState();
  fn _ZNK19QAbstractTransition11sourceStateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractTransition::~QAbstractTransition();
  fn _ZN19QAbstractTransitionD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
  fn dector_ZN19QAbstractTransitionC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
  fn _ZNK19QAbstractTransition10animationsEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QAbstractTransition::metaObject();
  fn _ZNK19QAbstractTransition10metaObjectEv(qthis: *mut c_void);
  // proto:  QStateMachine * QAbstractTransition::machine();
  fn _ZNK19QAbstractTransition7machineEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractTransition)=1
pub struct QAbstractTransition {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractTransition {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractTransition {
    return QAbstractTransition{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAbstractTransition {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractTransition {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QAbstractState * QAbstractTransition::targetState();
impl /*struct*/ QAbstractTransition {
  pub fn targetState<RetType, T: QAbstractTransition_targetState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.targetState(self);
    // return 1;
  }
}

pub trait QAbstractTransition_targetState<RetType> {
  fn targetState(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  QAbstractState * QAbstractTransition::targetState();
impl<'a> /*trait*/ QAbstractTransition_targetState<()> for () {
  fn targetState(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition11targetStateEv()};
     unsafe {_ZNK19QAbstractTransition11targetStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
impl /*struct*/ QAbstractTransition {
  pub fn targetStates<RetType, T: QAbstractTransition_targetStates<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.targetStates(self);
    // return 1;
  }
}

pub trait QAbstractTransition_targetStates<RetType> {
  fn targetStates(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
impl<'a> /*trait*/ QAbstractTransition_targetStates<()> for () {
  fn targetStates(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition12targetStatesEv()};
     unsafe {_ZNK19QAbstractTransition12targetStatesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QState * QAbstractTransition::sourceState();
impl /*struct*/ QAbstractTransition {
  pub fn sourceState<RetType, T: QAbstractTransition_sourceState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sourceState(self);
    // return 1;
  }
}

pub trait QAbstractTransition_sourceState<RetType> {
  fn sourceState(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  QState * QAbstractTransition::sourceState();
impl<'a> /*trait*/ QAbstractTransition_sourceState<QState> for () {
  fn sourceState(self , rsthis: & QAbstractTransition) -> QState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition11sourceStateEv()};
    let mut ret = unsafe {_ZNK19QAbstractTransition11sourceStateEv(rsthis.qclsinst)};
    let mut ret1 = QState::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTransition::~QAbstractTransition();
impl /*struct*/ QAbstractTransition {
  pub fn Free<RetType, T: QAbstractTransition_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractTransition_Free<RetType> {
  fn Free(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  void QAbstractTransition::~QAbstractTransition();
impl<'a> /*trait*/ QAbstractTransition_Free<()> for () {
  fn Free(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionD0Ev()};
     unsafe {_ZN19QAbstractTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
impl /*struct*/ QAbstractTransition {
  pub fn New<T: QAbstractTransition_New>(value: T) -> QAbstractTransition {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTransition_New {
  fn New(self) -> QAbstractTransition;
}

  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
impl<'a> /*trait*/ QAbstractTransition_New for (&'a QState) {
  fn New(self) -> QAbstractTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionC1EP6QState()};
    let ctysz: c_int = unsafe{QAbstractTransition_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractTransitionC1EP6QState(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QAbstractTransitionC1EP6QState(arg0)};
    let rsthis = QAbstractTransition{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
impl /*struct*/ QAbstractTransition {
  pub fn animations<RetType, T: QAbstractTransition_animations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.animations(self);
    // return 1;
  }
}

pub trait QAbstractTransition_animations<RetType> {
  fn animations(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
impl<'a> /*trait*/ QAbstractTransition_animations<()> for () {
  fn animations(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition10animationsEv()};
     unsafe {_ZNK19QAbstractTransition10animationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractTransition::metaObject();
impl /*struct*/ QAbstractTransition {
  pub fn metaObject<RetType, T: QAbstractTransition_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  const QMetaObject * QAbstractTransition::metaObject();
impl<'a> /*trait*/ QAbstractTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition10metaObjectEv()};
     unsafe {_ZNK19QAbstractTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStateMachine * QAbstractTransition::machine();
impl /*struct*/ QAbstractTransition {
  pub fn machine<RetType, T: QAbstractTransition_machine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.machine(self);
    // return 1;
  }
}

pub trait QAbstractTransition_machine<RetType> {
  fn machine(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  QStateMachine * QAbstractTransition::machine();
impl<'a> /*trait*/ QAbstractTransition_machine<QStateMachine> for () {
  fn machine(self , rsthis: & QAbstractTransition) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition7machineEv()};
    let mut ret = unsafe {_ZNK19QAbstractTransition7machineEv(rsthis.qclsinst)};
    let mut ret1 = QStateMachine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

