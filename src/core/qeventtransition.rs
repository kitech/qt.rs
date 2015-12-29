// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
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
use super::qabstracttransition::QAbstractTransition; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
use super::qstate::QState; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QEventTransition_Class_Size() -> c_int;
  // proto:  void QEventTransition::QEventTransition(const QEventTransition & );
  fn dector_ZN16QEventTransitionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QEventTransitionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QEventTransition::~QEventTransition();
  fn _ZN16QEventTransitionD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QEventTransition::setEventSource(QObject * object);
  fn _ZN16QEventTransition14setEventSourceEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QEventTransition::QEventTransition(QState * sourceState);
  fn dector_ZN16QEventTransitionC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QEventTransitionC1EP6QState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QEventTransition::metaObject();
  fn _ZNK16QEventTransition10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QEventTransition::eventSource();
  fn _ZNK16QEventTransition11eventSourceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QEventTransition)=1
#[derive(Default)]
pub struct QEventTransition {
  qbase: QAbstractTransition,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QEventTransition {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QEventTransition {
    return QEventTransition{qbase: QAbstractTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QEventTransition {
  type Target = QAbstractTransition;

  fn deref(&self) -> &QAbstractTransition {
    return & self.qbase;
  }
}
impl AsRef<QAbstractTransition> for QEventTransition {
  fn as_ref(& self) -> & QAbstractTransition {
    return & self.qbase;
  }
}
  // proto:  void QEventTransition::QEventTransition(const QEventTransition & );
impl /*struct*/ QEventTransition {
  pub fn New<T: QEventTransition_New>(value: T) -> QEventTransition {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QEventTransition_New {
  fn New(self) -> QEventTransition;
}

  // proto:  void QEventTransition::QEventTransition(const QEventTransition & );
impl<'a> /*trait*/ QEventTransition_New for (&'a QEventTransition) {
  fn New(self) -> QEventTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionC1ERKS_()};
    let ctysz: c_int = unsafe{QEventTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QEventTransitionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QEventTransitionC1ERKS_(arg0)} as u64;
    let rsthis = QEventTransition{qbase: QAbstractTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventTransition::~QEventTransition();
impl /*struct*/ QEventTransition {
  pub fn Free<RetType, T: QEventTransition_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QEventTransition_Free<RetType> {
  fn Free(self , rsthis: & QEventTransition) -> RetType;
}

  // proto:  void QEventTransition::~QEventTransition();
impl<'a> /*trait*/ QEventTransition_Free<()> for () {
  fn Free(self , rsthis: & QEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionD0Ev()};
     unsafe {_ZN16QEventTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventTransition::setEventSource(QObject * object);
impl /*struct*/ QEventTransition {
  pub fn setEventSource<RetType, T: QEventTransition_setEventSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEventSource(self);
    // return 1;
  }
}

pub trait QEventTransition_setEventSource<RetType> {
  fn setEventSource(self , rsthis: & QEventTransition) -> RetType;
}

  // proto:  void QEventTransition::setEventSource(QObject * object);
impl<'a> /*trait*/ QEventTransition_setEventSource<()> for (&'a QObject) {
  fn setEventSource(self , rsthis: & QEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransition14setEventSourceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QEventTransition14setEventSourceEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEventTransition::QEventTransition(QState * sourceState);
impl<'a> /*trait*/ QEventTransition_New for (&'a QState) {
  fn New(self) -> QEventTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionC1EP6QState()};
    let ctysz: c_int = unsafe{QEventTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QEventTransitionC1EP6QState(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QEventTransitionC1EP6QState(arg0)} as u64;
    let rsthis = QEventTransition{qbase: QAbstractTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QEventTransition::metaObject();
impl /*struct*/ QEventTransition {
  pub fn metaObject<RetType, T: QEventTransition_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QEventTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: & QEventTransition) -> RetType;
}

  // proto:  const QMetaObject * QEventTransition::metaObject();
impl<'a> /*trait*/ QEventTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: & QEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QEventTransition10metaObjectEv()};
     unsafe {_ZNK16QEventTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QEventTransition::eventSource();
impl /*struct*/ QEventTransition {
  pub fn eventSource<RetType, T: QEventTransition_eventSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.eventSource(self);
    // return 1;
  }
}

pub trait QEventTransition_eventSource<RetType> {
  fn eventSource(self , rsthis: & QEventTransition) -> RetType;
}

  // proto:  QObject * QEventTransition::eventSource();
impl<'a> /*trait*/ QEventTransition_eventSource<QObject> for () {
  fn eventSource(self , rsthis: & QEventTransition) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QEventTransition11eventSourceEv()};
    let mut ret = unsafe {_ZNK16QEventTransition11eventSourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

