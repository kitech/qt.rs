// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qstatemachine.h
// dst-file: /src/core/qstatemachine.rs
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
use super::qstate::QState; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
use super::qcoreevent::QEvent; // 773
use super::qabstractanimation::QAbstractAnimation; // 773
use super::qstring::QString; // 773
use super::qabstractstate::QAbstractState; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStateMachine_Class_Size() -> c_int;
  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
  fn _ZNK13QStateMachine17defaultAnimationsEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
  fn _ZN13QStateMachine16postDelayedEventEP6QEventi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
  fn _ZNK13QStateMachine13configurationEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QStateMachine::setRunning(bool running);
  fn _ZN13QStateMachine10setRunningEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStateMachine::addDefaultAnimation(QAbstractAnimation * animation);
  fn _ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::removeDefaultAnimation(QAbstractAnimation * animation);
  fn _ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::setAnimated(bool enabled);
  fn _ZN13QStateMachine11setAnimatedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStateMachine::QStateMachine(QObject * parent);
  fn dector_ZN13QStateMachineC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QStateMachineC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QStateMachine::errorString();
  fn _ZNK13QStateMachine11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QStateMachine::isRunning();
  fn _ZNK13QStateMachine9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
  fn _ZN13QStateMachine18cancelDelayedEventEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QStateMachine::~QStateMachine();
  fn _ZN13QStateMachineD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QStateMachine::metaObject();
  fn _ZNK13QStateMachine10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QStateMachine::runningChanged(bool running);
  fn _ZN13QStateMachine14runningChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStateMachine::addState(QAbstractState * state);
  fn _ZN13QStateMachine8addStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::clearError();
  fn _ZN13QStateMachine10clearErrorEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QStateMachine::removeState(QAbstractState * state);
  fn _ZN13QStateMachine11removeStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStateMachine::stop();
  fn _ZN13QStateMachine4stopEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QStateMachine::isAnimated();
  fn _ZNK13QStateMachine10isAnimatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QStateMachine::start();
  fn _ZN13QStateMachine5startEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
  fn _ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QStateMachine::QStateMachine(const QStateMachine & );
  fn dector_ZN13QStateMachineC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QStateMachineC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QStateMachine_SlotProxy_connect__ZN13QStateMachine14runningChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QStateMachine_SlotProxy_connect_box__ZN13QStateMachine14runningChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStateMachine)=1
#[derive(Default)]
pub struct QStateMachine {
  qbase: QState,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _started_1: QStateMachine_started_signal,
  pub _runningChanged_1: QStateMachine_runningChanged_signal,
  pub _stopped_1: QStateMachine_stopped_signal,
}

impl /*struct*/ QStateMachine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStateMachine {
    return QStateMachine{qbase: QState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStateMachine {
  type Target = QState;

  fn deref(&self) -> &QState {
    return & self.qbase;
  }
}
impl AsRef<QState> for QStateMachine {
  fn as_ref(& self) -> & QState {
    return & self.qbase;
  }
}
  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl /*struct*/ QStateMachine {
  pub fn defaultAnimations<RetType, T: QStateMachine_defaultAnimations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultAnimations(self);
    // return 1;
  }
}

pub trait QStateMachine_defaultAnimations<RetType> {
  fn defaultAnimations(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl<'a> /*trait*/ QStateMachine_defaultAnimations<()> for () {
  fn defaultAnimations(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine17defaultAnimationsEv()};
     unsafe {_ZNK13QStateMachine17defaultAnimationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl /*struct*/ QStateMachine {
  pub fn postDelayedEvent<RetType, T: QStateMachine_postDelayedEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.postDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_postDelayedEvent<RetType> {
  fn postDelayedEvent(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl<'a> /*trait*/ QStateMachine_postDelayedEvent<i32> for (&'a QEvent, i32) {
  fn postDelayedEvent(self , rsthis: & QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine16postDelayedEventEP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN13QStateMachine16postDelayedEventEP6QEventi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
impl /*struct*/ QStateMachine {
  pub fn configuration<RetType, T: QStateMachine_configuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.configuration(self);
    // return 1;
  }
}

pub trait QStateMachine_configuration<RetType> {
  fn configuration(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
impl<'a> /*trait*/ QStateMachine_configuration<()> for () {
  fn configuration(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine13configurationEv()};
     unsafe {_ZNK13QStateMachine13configurationEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::setRunning(bool running);
impl /*struct*/ QStateMachine {
  pub fn setRunning<RetType, T: QStateMachine_setRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_setRunning<RetType> {
  fn setRunning(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::setRunning(bool running);
impl<'a> /*trait*/ QStateMachine_setRunning<()> for (i8) {
  fn setRunning(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10setRunningEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QStateMachine10setRunningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::addDefaultAnimation(QAbstractAnimation * animation);
impl /*struct*/ QStateMachine {
  pub fn addDefaultAnimation<RetType, T: QStateMachine_addDefaultAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addDefaultAnimation(self);
    // return 1;
  }
}

pub trait QStateMachine_addDefaultAnimation<RetType> {
  fn addDefaultAnimation(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::addDefaultAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QStateMachine_addDefaultAnimation<()> for (&'a QAbstractAnimation) {
  fn addDefaultAnimation(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::removeDefaultAnimation(QAbstractAnimation * animation);
impl /*struct*/ QStateMachine {
  pub fn removeDefaultAnimation<RetType, T: QStateMachine_removeDefaultAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeDefaultAnimation(self);
    // return 1;
  }
}

pub trait QStateMachine_removeDefaultAnimation<RetType> {
  fn removeDefaultAnimation(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::removeDefaultAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QStateMachine_removeDefaultAnimation<()> for (&'a QAbstractAnimation) {
  fn removeDefaultAnimation(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::setAnimated(bool enabled);
impl /*struct*/ QStateMachine {
  pub fn setAnimated<RetType, T: QStateMachine_setAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_setAnimated<RetType> {
  fn setAnimated(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::setAnimated(bool enabled);
impl<'a> /*trait*/ QStateMachine_setAnimated<()> for (i8) {
  fn setAnimated(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11setAnimatedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QStateMachine11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::QStateMachine(QObject * parent);
impl /*struct*/ QStateMachine {
  pub fn New<T: QStateMachine_New>(value: T) -> QStateMachine {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStateMachine_New {
  fn New(self) -> QStateMachine;
}

  // proto:  void QStateMachine::QStateMachine(QObject * parent);
impl<'a> /*trait*/ QStateMachine_New for (&'a QObject) {
  fn New(self) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1EP7QObject()};
    let ctysz: c_int = unsafe{QStateMachine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QStateMachineC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QStateMachineC1EP7QObject(arg0)} as u64;
    let rsthis = QStateMachine{qbase: QState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QStateMachine::errorString();
impl /*struct*/ QStateMachine {
  pub fn errorString<RetType, T: QStateMachine_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QStateMachine_errorString<RetType> {
  fn errorString(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  QString QStateMachine::errorString();
impl<'a> /*trait*/ QStateMachine_errorString<QString> for () {
  fn errorString(self , rsthis: & QStateMachine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine11errorStringEv()};
    let mut ret = unsafe {_ZNK13QStateMachine11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStateMachine::isRunning();
impl /*struct*/ QStateMachine {
  pub fn isRunning<RetType, T: QStateMachine_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_isRunning<RetType> {
  fn isRunning(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::isRunning();
impl<'a> /*trait*/ QStateMachine_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine9isRunningEv()};
    let mut ret = unsafe {_ZNK13QStateMachine9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl /*struct*/ QStateMachine {
  pub fn cancelDelayedEvent<RetType, T: QStateMachine_cancelDelayedEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancelDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_cancelDelayedEvent<RetType> {
  fn cancelDelayedEvent(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl<'a> /*trait*/ QStateMachine_cancelDelayedEvent<i8> for (i32) {
  fn cancelDelayedEvent(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine18cancelDelayedEventEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN13QStateMachine18cancelDelayedEventEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStateMachine::~QStateMachine();
impl /*struct*/ QStateMachine {
  pub fn Free<RetType, T: QStateMachine_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStateMachine_Free<RetType> {
  fn Free(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::~QStateMachine();
impl<'a> /*trait*/ QStateMachine_Free<()> for () {
  fn Free(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineD0Ev()};
     unsafe {_ZN13QStateMachineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QStateMachine::metaObject();
impl /*struct*/ QStateMachine {
  pub fn metaObject<RetType, T: QStateMachine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStateMachine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  const QMetaObject * QStateMachine::metaObject();
impl<'a> /*trait*/ QStateMachine_metaObject<()> for () {
  fn metaObject(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10metaObjectEv()};
     unsafe {_ZNK13QStateMachine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::runningChanged(bool running);
impl /*struct*/ QStateMachine {
  pub fn runningChanged<RetType, T: QStateMachine_runningChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.runningChanged(self);
    // return 1;
  }
}

pub trait QStateMachine_runningChanged<RetType> {
  fn runningChanged(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::runningChanged(bool running);
impl<'a> /*trait*/ QStateMachine_runningChanged<()> for (i8) {
  fn runningChanged(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine14runningChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QStateMachine14runningChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::addState(QAbstractState * state);
impl /*struct*/ QStateMachine {
  pub fn addState<RetType, T: QStateMachine_addState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addState(self);
    // return 1;
  }
}

pub trait QStateMachine_addState<RetType> {
  fn addState(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::addState(QAbstractState * state);
impl<'a> /*trait*/ QStateMachine_addState<()> for (&'a QAbstractState) {
  fn addState(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine8addStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStateMachine8addStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::clearError();
impl /*struct*/ QStateMachine {
  pub fn clearError<RetType, T: QStateMachine_clearError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearError(self);
    // return 1;
  }
}

pub trait QStateMachine_clearError<RetType> {
  fn clearError(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::clearError();
impl<'a> /*trait*/ QStateMachine_clearError<()> for () {
  fn clearError(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10clearErrorEv()};
     unsafe {_ZN13QStateMachine10clearErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::removeState(QAbstractState * state);
impl /*struct*/ QStateMachine {
  pub fn removeState<RetType, T: QStateMachine_removeState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeState(self);
    // return 1;
  }
}

pub trait QStateMachine_removeState<RetType> {
  fn removeState(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::removeState(QAbstractState * state);
impl<'a> /*trait*/ QStateMachine_removeState<()> for (&'a QAbstractState) {
  fn removeState(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11removeStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStateMachine11removeStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::stop();
impl /*struct*/ QStateMachine {
  pub fn stop<RetType, T: QStateMachine_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QStateMachine_stop<RetType> {
  fn stop(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::stop();
impl<'a> /*trait*/ QStateMachine_stop<()> for () {
  fn stop(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine4stopEv()};
     unsafe {_ZN13QStateMachine4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStateMachine::isAnimated();
impl /*struct*/ QStateMachine {
  pub fn isAnimated<RetType, T: QStateMachine_isAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_isAnimated<RetType> {
  fn isAnimated(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::isAnimated();
impl<'a> /*trait*/ QStateMachine_isAnimated<i8> for () {
  fn isAnimated(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10isAnimatedEv()};
    let mut ret = unsafe {_ZNK13QStateMachine10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStateMachine::start();
impl /*struct*/ QStateMachine {
  pub fn start<RetType, T: QStateMachine_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QStateMachine_start<RetType> {
  fn start(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::start();
impl<'a> /*trait*/ QStateMachine_start<()> for () {
  fn start(self , rsthis: & QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine5startEv()};
     unsafe {_ZN13QStateMachine5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl /*struct*/ QStateMachine {
  pub fn eventFilter<RetType, T: QStateMachine_eventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.eventFilter(self);
    // return 1;
  }
}

pub trait QStateMachine_eventFilter<RetType> {
  fn eventFilter(self , rsthis: & QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl<'a> /*trait*/ QStateMachine_eventFilter<i8> for (&'a QObject, &'a QEvent) {
  fn eventFilter(self , rsthis: & QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStateMachine::QStateMachine(const QStateMachine & );
impl<'a> /*trait*/ QStateMachine_New for (&'a QStateMachine) {
  fn New(self) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1ERKS_()};
    let ctysz: c_int = unsafe{QStateMachine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QStateMachineC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QStateMachineC1ERKS_(arg0)} as u64;
    let rsthis = QStateMachine{qbase: QState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QStateMachine_started
pub struct QStateMachine_started_signal{poi:u64}
impl /* struct */ QStateMachine {
  pub fn started_1(&self) -> QStateMachine_started_signal {
     return QStateMachine_started_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStateMachine_started_signal {
  pub fn connect<T: QStateMachine_started_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStateMachine_started_signal_connect {
  fn connect(self, sigthis: QStateMachine_started_signal);
}

#[derive(Default)] // for QStateMachine_runningChanged
pub struct QStateMachine_runningChanged_signal{poi:u64}
impl /* struct */ QStateMachine {
  pub fn runningChanged_1(&self) -> QStateMachine_runningChanged_signal {
     return QStateMachine_runningChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStateMachine_runningChanged_signal {
  pub fn connect<T: QStateMachine_runningChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStateMachine_runningChanged_signal_connect {
  fn connect(self, sigthis: QStateMachine_runningChanged_signal);
}

#[derive(Default)] // for QStateMachine_stopped
pub struct QStateMachine_stopped_signal{poi:u64}
impl /* struct */ QStateMachine {
  pub fn stopped_1(&self) -> QStateMachine_stopped_signal {
     return QStateMachine_stopped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStateMachine_stopped_signal {
  pub fn connect<T: QStateMachine_stopped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStateMachine_stopped_signal_connect {
  fn connect(self, sigthis: QStateMachine_stopped_signal);
}

// runningChanged(_Bool)
extern fn QStateMachine_runningChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
}
extern fn QStateMachine_runningChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QStateMachine_runningChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QStateMachine_runningChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStateMachine_runningChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStateMachine_SlotProxy_connect__ZN13QStateMachine14runningChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStateMachine_runningChanged_signal_connect for Box<fn(i8)> {
  fn connect(self, sigthis: QStateMachine_runningChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStateMachine_runningChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStateMachine_SlotProxy_connect__ZN13QStateMachine14runningChangedEb(arg0, arg1, arg2)};
  }
}
// <= body block end

