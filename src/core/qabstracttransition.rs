// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
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
use super::qabstractstate::QAbstractState; // 773
use super::qabstractanimation::QAbstractAnimation; // 773
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
  fn _ZNK19QAbstractTransition11targetStateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
  fn _ZNK19QAbstractTransition12targetStatesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QState * QAbstractTransition::sourceState();
  fn _ZNK19QAbstractTransition11sourceStateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractTransition::~QAbstractTransition();
  fn _ZN19QAbstractTransitionD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
  fn dector_ZN19QAbstractTransitionC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractTransitionC1EP6QState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractTransition::setTargetState(QAbstractState * target);
  fn _ZN19QAbstractTransition14setTargetStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractTransition::addAnimation(QAbstractAnimation * animation);
  fn _ZN19QAbstractTransition12addAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
  fn _ZNK19QAbstractTransition10animationsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractTransition::removeAnimation(QAbstractAnimation * animation);
  fn _ZN19QAbstractTransition15removeAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractTransition::metaObject();
  fn _ZNK19QAbstractTransition10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractTransition::QAbstractTransition(const QAbstractTransition & );
  fn dector_ZN19QAbstractTransitionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractTransitionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStateMachine * QAbstractTransition::machine();
  fn _ZNK19QAbstractTransition7machineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractTransition)=1
#[derive(Default)]
pub struct QAbstractTransition {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _targetStateChanged: QAbstractTransition_targetStateChanged_signal,
  pub _targetStatesChanged: QAbstractTransition_targetStatesChanged_signal,
  pub _triggered: QAbstractTransition_triggered_signal,
}

impl /*struct*/ QAbstractTransition {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractTransition {
    return QAbstractTransition{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret1 = QState::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTransition::~QAbstractTransition();
impl /*struct*/ QAbstractTransition {
  pub fn free<RetType, T: QAbstractTransition_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractTransition_free<RetType> {
  fn free(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  void QAbstractTransition::~QAbstractTransition();
impl<'a> /*trait*/ QAbstractTransition_free<()> for () {
  fn free(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionD0Ev()};
     unsafe {_ZN19QAbstractTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
impl /*struct*/ QAbstractTransition {
  pub fn new<T: QAbstractTransition_new>(value: T) -> QAbstractTransition {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTransition_new {
  fn new(self) -> QAbstractTransition;
}

  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
impl<'a> /*trait*/ QAbstractTransition_new for (&'a QState) {
  fn new(self) -> QAbstractTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionC1EP6QState()};
    let ctysz: c_int = unsafe{QAbstractTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractTransitionC1EP6QState(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QAbstractTransitionC1EP6QState(arg0)} as u64;
    let rsthis = QAbstractTransition{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractTransition::setTargetState(QAbstractState * target);
impl /*struct*/ QAbstractTransition {
  pub fn setTargetState<RetType, T: QAbstractTransition_setTargetState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTargetState(self);
    // return 1;
  }
}

pub trait QAbstractTransition_setTargetState<RetType> {
  fn setTargetState(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  void QAbstractTransition::setTargetState(QAbstractState * target);
impl<'a> /*trait*/ QAbstractTransition_setTargetState<()> for (&'a QAbstractState) {
  fn setTargetState(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransition14setTargetStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractTransition14setTargetStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractTransition::addAnimation(QAbstractAnimation * animation);
impl /*struct*/ QAbstractTransition {
  pub fn addAnimation<RetType, T: QAbstractTransition_addAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAnimation(self);
    // return 1;
  }
}

pub trait QAbstractTransition_addAnimation<RetType> {
  fn addAnimation(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  void QAbstractTransition::addAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QAbstractTransition_addAnimation<()> for (&'a QAbstractAnimation) {
  fn addAnimation(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransition12addAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractTransition12addAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
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

  // proto:  void QAbstractTransition::removeAnimation(QAbstractAnimation * animation);
impl /*struct*/ QAbstractTransition {
  pub fn removeAnimation<RetType, T: QAbstractTransition_removeAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAnimation(self);
    // return 1;
  }
}

pub trait QAbstractTransition_removeAnimation<RetType> {
  fn removeAnimation(self , rsthis: & QAbstractTransition) -> RetType;
}

  // proto:  void QAbstractTransition::removeAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QAbstractTransition_removeAnimation<()> for (&'a QAbstractAnimation) {
  fn removeAnimation(self , rsthis: & QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransition15removeAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractTransition15removeAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
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

  // proto:  void QAbstractTransition::QAbstractTransition(const QAbstractTransition & );
impl<'a> /*trait*/ QAbstractTransition_new for (&'a QAbstractTransition) {
  fn new(self) -> QAbstractTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractTransitionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QAbstractTransitionC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractTransition{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
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
    let mut ret1 = QStateMachine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QAbstractTransition_targetStateChanged
pub struct QAbstractTransition_targetStateChanged_signal{poi:u64}
impl /* struct */ QAbstractTransition {
  pub fn targetStateChanged(&self) -> QAbstractTransition_targetStateChanged_signal {
     return QAbstractTransition_targetStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTransition_targetStateChanged_signal {
  pub fn connect<T: QAbstractTransition_targetStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTransition_targetStateChanged_signal_connect {
  fn connect(self, sigthis: QAbstractTransition_targetStateChanged_signal);
}

#[derive(Default)] // for QAbstractTransition_targetStatesChanged
pub struct QAbstractTransition_targetStatesChanged_signal{poi:u64}
impl /* struct */ QAbstractTransition {
  pub fn targetStatesChanged(&self) -> QAbstractTransition_targetStatesChanged_signal {
     return QAbstractTransition_targetStatesChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTransition_targetStatesChanged_signal {
  pub fn connect<T: QAbstractTransition_targetStatesChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTransition_targetStatesChanged_signal_connect {
  fn connect(self, sigthis: QAbstractTransition_targetStatesChanged_signal);
}

#[derive(Default)] // for QAbstractTransition_triggered
pub struct QAbstractTransition_triggered_signal{poi:u64}
impl /* struct */ QAbstractTransition {
  pub fn triggered(&self) -> QAbstractTransition_triggered_signal {
     return QAbstractTransition_triggered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTransition_triggered_signal {
  pub fn connect<T: QAbstractTransition_triggered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTransition_triggered_signal_connect {
  fn connect(self, sigthis: QAbstractTransition_triggered_signal);
}

// <= body block end

