// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qobject::QObject; // 773
use super::qcoreevent::QEvent; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
  fn _ZNK13QStateMachine17defaultAnimationsEv(qthis: *mut c_void);
  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
  fn _ZN13QStateMachine16postDelayedEventEP6QEventi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
  fn _ZNK13QStateMachine13configurationEv(qthis: *mut c_void);
  // proto:  void QStateMachine::setRunning(bool running);
  fn _ZN13QStateMachine10setRunningEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QStateMachine::setAnimated(bool enabled);
  fn _ZN13QStateMachine11setAnimatedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QStateMachine::QStateMachine(QObject * parent);
  fn _ZN13QStateMachineC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QStateMachine::errorString();
  fn _ZNK13QStateMachine11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QStateMachine::isRunning();
  fn _ZNK13QStateMachine9isRunningEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
  fn _ZN13QStateMachine18cancelDelayedEventEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QStateMachine::~QStateMachine();
  fn _ZN13QStateMachineD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QStateMachine::metaObject();
  fn _ZNK13QStateMachine10metaObjectEv(qthis: *mut c_void);
  // proto:  void QStateMachine::runningChanged(bool running);
  fn _ZN13QStateMachine14runningChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QStateMachine::clearError();
  fn _ZN13QStateMachine10clearErrorEv(qthis: *mut c_void);
  // proto:  void QStateMachine::stop();
  fn _ZN13QStateMachine4stopEv(qthis: *mut c_void);
  // proto:  bool QStateMachine::isAnimated();
  fn _ZNK13QStateMachine10isAnimatedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QStateMachine::start();
  fn _ZN13QStateMachine5startEv(qthis: *mut c_void);
  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
  fn _ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QStateMachine::QStateMachine(const QStateMachine & );
  fn _ZN13QStateMachineC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStateMachine)=1
pub struct QStateMachine {
  pub qclsinst: *mut c_void,
}

  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl /*struct*/ QStateMachine {
  pub fn defaultAnimations<RetType, T: QStateMachine_defaultAnimations<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultAnimations(self);
    // return 1;
  }
}

pub trait QStateMachine_defaultAnimations<RetType> {
  fn defaultAnimations(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl<'a> /*trait*/ QStateMachine_defaultAnimations<()> for () {
  fn defaultAnimations(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine17defaultAnimationsEv()};
     unsafe {_ZNK13QStateMachine17defaultAnimationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl /*struct*/ QStateMachine {
  pub fn postDelayedEvent<RetType, T: QStateMachine_postDelayedEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.postDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_postDelayedEvent<RetType> {
  fn postDelayedEvent(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl<'a> /*trait*/ QStateMachine_postDelayedEvent<i32> for (QEvent, i32) {
  fn postDelayedEvent(self , rsthis: &mut QStateMachine) -> i32 {
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
  pub fn configuration<RetType, T: QStateMachine_configuration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.configuration(self);
    // return 1;
  }
}

pub trait QStateMachine_configuration<RetType> {
  fn configuration(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
impl<'a> /*trait*/ QStateMachine_configuration<()> for () {
  fn configuration(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine13configurationEv()};
     unsafe {_ZNK13QStateMachine13configurationEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::setRunning(bool running);
impl /*struct*/ QStateMachine {
  pub fn setRunning<RetType, T: QStateMachine_setRunning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_setRunning<RetType> {
  fn setRunning(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::setRunning(bool running);
impl<'a> /*trait*/ QStateMachine_setRunning<()> for (i8) {
  fn setRunning(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10setRunningEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QStateMachine10setRunningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::setAnimated(bool enabled);
impl /*struct*/ QStateMachine {
  pub fn setAnimated<RetType, T: QStateMachine_setAnimated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_setAnimated<RetType> {
  fn setAnimated(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::setAnimated(bool enabled);
impl<'a> /*trait*/ QStateMachine_setAnimated<()> for (i8) {
  fn setAnimated(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11setAnimatedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QStateMachine11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::QStateMachine(QObject * parent);
impl /*struct*/ QStateMachine {
  pub fn NewQStateMachine<T: QStateMachine_NewQStateMachine>(value: T) -> QStateMachine {
    let rsthis = value.NewQStateMachine();
    return rsthis;
    // return 1;
  }
}

pub trait QStateMachine_NewQStateMachine {
  fn NewQStateMachine(self) -> QStateMachine;
}

  // proto:  void QStateMachine::QStateMachine(QObject * parent);
impl<'a> /*trait*/ QStateMachine_NewQStateMachine for (QObject) {
  fn NewQStateMachine(self) -> QStateMachine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStateMachineC1EP7QObject(qthis, arg0)};
    let rsthis = QStateMachine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QStateMachine::errorString();
impl /*struct*/ QStateMachine {
  pub fn errorString<RetType, T: QStateMachine_errorString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QStateMachine_errorString<RetType> {
  fn errorString(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  QString QStateMachine::errorString();
impl<'a> /*trait*/ QStateMachine_errorString<QString> for () {
  fn errorString(self , rsthis: &mut QStateMachine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine11errorStringEv()};
    let mut ret = unsafe {_ZNK13QStateMachine11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStateMachine::isRunning();
impl /*struct*/ QStateMachine {
  pub fn isRunning<RetType, T: QStateMachine_isRunning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_isRunning<RetType> {
  fn isRunning(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::isRunning();
impl<'a> /*trait*/ QStateMachine_isRunning<i8> for () {
  fn isRunning(self , rsthis: &mut QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine9isRunningEv()};
    let mut ret = unsafe {_ZNK13QStateMachine9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl /*struct*/ QStateMachine {
  pub fn cancelDelayedEvent<RetType, T: QStateMachine_cancelDelayedEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cancelDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_cancelDelayedEvent<RetType> {
  fn cancelDelayedEvent(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl<'a> /*trait*/ QStateMachine_cancelDelayedEvent<i8> for (i32) {
  fn cancelDelayedEvent(self , rsthis: &mut QStateMachine) -> i8 {
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
  pub fn FreeQStateMachine<RetType, T: QStateMachine_FreeQStateMachine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStateMachine(self);
    // return 1;
  }
}

pub trait QStateMachine_FreeQStateMachine<RetType> {
  fn FreeQStateMachine(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::~QStateMachine();
impl<'a> /*trait*/ QStateMachine_FreeQStateMachine<()> for () {
  fn FreeQStateMachine(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineD0Ev()};
     unsafe {_ZN13QStateMachineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QStateMachine::metaObject();
impl /*struct*/ QStateMachine {
  pub fn metaObject<RetType, T: QStateMachine_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStateMachine_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  const QMetaObject * QStateMachine::metaObject();
impl<'a> /*trait*/ QStateMachine_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10metaObjectEv()};
     unsafe {_ZNK13QStateMachine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::runningChanged(bool running);
impl /*struct*/ QStateMachine {
  pub fn runningChanged<RetType, T: QStateMachine_runningChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.runningChanged(self);
    // return 1;
  }
}

pub trait QStateMachine_runningChanged<RetType> {
  fn runningChanged(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::runningChanged(bool running);
impl<'a> /*trait*/ QStateMachine_runningChanged<()> for (i8) {
  fn runningChanged(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine14runningChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QStateMachine14runningChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStateMachine::clearError();
impl /*struct*/ QStateMachine {
  pub fn clearError<RetType, T: QStateMachine_clearError<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearError(self);
    // return 1;
  }
}

pub trait QStateMachine_clearError<RetType> {
  fn clearError(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::clearError();
impl<'a> /*trait*/ QStateMachine_clearError<()> for () {
  fn clearError(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10clearErrorEv()};
     unsafe {_ZN13QStateMachine10clearErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStateMachine::stop();
impl /*struct*/ QStateMachine {
  pub fn stop<RetType, T: QStateMachine_stop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QStateMachine_stop<RetType> {
  fn stop(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::stop();
impl<'a> /*trait*/ QStateMachine_stop<()> for () {
  fn stop(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine4stopEv()};
     unsafe {_ZN13QStateMachine4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStateMachine::isAnimated();
impl /*struct*/ QStateMachine {
  pub fn isAnimated<RetType, T: QStateMachine_isAnimated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_isAnimated<RetType> {
  fn isAnimated(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::isAnimated();
impl<'a> /*trait*/ QStateMachine_isAnimated<i8> for () {
  fn isAnimated(self , rsthis: &mut QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10isAnimatedEv()};
    let mut ret = unsafe {_ZNK13QStateMachine10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStateMachine::start();
impl /*struct*/ QStateMachine {
  pub fn start<RetType, T: QStateMachine_start<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QStateMachine_start<RetType> {
  fn start(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  void QStateMachine::start();
impl<'a> /*trait*/ QStateMachine_start<()> for () {
  fn start(self , rsthis: &mut QStateMachine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine5startEv()};
     unsafe {_ZN13QStateMachine5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl /*struct*/ QStateMachine {
  pub fn eventFilter<RetType, T: QStateMachine_eventFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.eventFilter(self);
    // return 1;
  }
}

pub trait QStateMachine_eventFilter<RetType> {
  fn eventFilter(self , rsthis: &mut QStateMachine) -> RetType;
}

  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl<'a> /*trait*/ QStateMachine_eventFilter<i8> for (QObject, QEvent) {
  fn eventFilter(self , rsthis: &mut QStateMachine) -> i8 {
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
impl<'a> /*trait*/ QStateMachine_NewQStateMachine for (QStateMachine) {
  fn NewQStateMachine(self) -> QStateMachine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStateMachineC1ERKS_(qthis, arg0)};
    let rsthis = QStateMachine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

