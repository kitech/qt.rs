// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qeventtransition.h
// dst-file: /src/core/qeventtransition.rs
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
use super::qstate::QState; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QEventTransition::QEventTransition(const QEventTransition & );
  fn _ZN16QEventTransitionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventTransition::~QEventTransition();
  fn _ZN16QEventTransitionD0Ev(qthis: *mut c_void);
  // proto:  void QEventTransition::setEventSource(QObject * object);
  fn _ZN16QEventTransition14setEventSourceEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventTransition::QEventTransition(QState * sourceState);
  fn _ZN16QEventTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QEventTransition::metaObject();
  fn _ZNK16QEventTransition10metaObjectEv(qthis: *mut c_void);
  // proto:  QObject * QEventTransition::eventSource();
  fn _ZNK16QEventTransition11eventSourceEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QEventTransition)=1
pub struct QEventTransition {
  pub qclsinst: *mut c_void,
}

  // proto:  void QEventTransition::QEventTransition(const QEventTransition & );
impl /*struct*/ QEventTransition {
  pub fn NewQEventTransition<T: QEventTransition_NewQEventTransition>(value: T) -> QEventTransition {
    let rsthis = value.NewQEventTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QEventTransition_NewQEventTransition {
  fn NewQEventTransition(self) -> QEventTransition;
}

  // proto:  void QEventTransition::QEventTransition(const QEventTransition & );
impl<'a> /*trait*/ QEventTransition_NewQEventTransition for (QEventTransition) {
  fn NewQEventTransition(self) -> QEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventTransition::~QEventTransition();
impl /*struct*/ QEventTransition {
  pub fn FreeQEventTransition<RetType, T: QEventTransition_FreeQEventTransition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQEventTransition(self);
    // return 1;
  }
}

pub trait QEventTransition_FreeQEventTransition<RetType> {
  fn FreeQEventTransition(self , rsthis: &mut QEventTransition) -> RetType;
}

  // proto:  void QEventTransition::~QEventTransition();
impl<'a> /*trait*/ QEventTransition_FreeQEventTransition<()> for () {
  fn FreeQEventTransition(self , rsthis: &mut QEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionD0Ev()};
     unsafe {_ZN16QEventTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventTransition::setEventSource(QObject * object);
impl /*struct*/ QEventTransition {
  pub fn setEventSource<RetType, T: QEventTransition_setEventSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEventSource(self);
    // return 1;
  }
}

pub trait QEventTransition_setEventSource<RetType> {
  fn setEventSource(self , rsthis: &mut QEventTransition) -> RetType;
}

  // proto:  void QEventTransition::setEventSource(QObject * object);
impl<'a> /*trait*/ QEventTransition_setEventSource<()> for (QObject) {
  fn setEventSource(self , rsthis: &mut QEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransition14setEventSourceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QEventTransition14setEventSourceEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEventTransition::QEventTransition(QState * sourceState);
impl<'a> /*trait*/ QEventTransition_NewQEventTransition for (QState) {
  fn NewQEventTransition(self) -> QEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QEventTransition::metaObject();
impl /*struct*/ QEventTransition {
  pub fn metaObject<RetType, T: QEventTransition_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QEventTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QEventTransition) -> RetType;
}

  // proto:  const QMetaObject * QEventTransition::metaObject();
impl<'a> /*trait*/ QEventTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QEventTransition10metaObjectEv()};
     unsafe {_ZNK16QEventTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QEventTransition::eventSource();
impl /*struct*/ QEventTransition {
  pub fn eventSource<RetType, T: QEventTransition_eventSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.eventSource(self);
    // return 1;
  }
}

pub trait QEventTransition_eventSource<RetType> {
  fn eventSource(self , rsthis: &mut QEventTransition) -> RetType;
}

  // proto:  QObject * QEventTransition::eventSource();
impl<'a> /*trait*/ QEventTransition_eventSource<QObject> for () {
  fn eventSource(self , rsthis: &mut QEventTransition) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QEventTransition11eventSourceEv()};
    let mut ret = unsafe {_ZNK16QEventTransition11eventSourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

