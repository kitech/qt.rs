// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtWidgets/qkeyeventtransition.h
// dst-file: /src/widgets/qkeyeventtransition.rs
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
use super::super::core::qeventtransition::QEventTransition; // 771
use std::ops::Deref;
use super::super::core::qobject::QObject; // 771
use super::super::core::qstate::QState; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QKeyEventTransition_Class_Size() -> c_int;
  // proto:  void QKeyEventTransition::setKey(int key);
  fn _ZN19QKeyEventTransition6setKeyEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QKeyEventTransition::metaObject();
  fn _ZNK19QKeyEventTransition10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QKeyEventTransition::~QKeyEventTransition();
  fn _ZN19QKeyEventTransitionD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QKeyEventTransition::key();
  fn _ZNK19QKeyEventTransition3keyEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QKeyEventTransition::QKeyEventTransition(QState * sourceState);
  fn dector_ZN19QKeyEventTransitionC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QKeyEventTransitionC1EP6QState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QKeyEventTransition::QKeyEventTransition(const QKeyEventTransition & );
  fn dector_ZN19QKeyEventTransitionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QKeyEventTransitionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QKeyEventTransition)=1
#[derive(Default)]
pub struct QKeyEventTransition {
  qbase: QEventTransition,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QKeyEventTransition {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QKeyEventTransition {
    return QKeyEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QKeyEventTransition {
  type Target = QEventTransition;

  fn deref(&self) -> &QEventTransition {
    return & self.qbase;
  }
}
impl AsRef<QEventTransition> for QKeyEventTransition {
  fn as_ref(& self) -> & QEventTransition {
    return & self.qbase;
  }
}
  // proto:  void QKeyEventTransition::setKey(int key);
impl /*struct*/ QKeyEventTransition {
  pub fn setKey<RetType, T: QKeyEventTransition_setKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QKeyEventTransition_setKey<RetType> {
  fn setKey(self , rsthis: & QKeyEventTransition) -> RetType;
}

  // proto:  void QKeyEventTransition::setKey(int key);
impl<'a> /*trait*/ QKeyEventTransition_setKey<()> for (i32) {
  fn setKey(self , rsthis: & QKeyEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransition6setKeyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN19QKeyEventTransition6setKeyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QKeyEventTransition::metaObject();
impl /*struct*/ QKeyEventTransition {
  pub fn metaObject<RetType, T: QKeyEventTransition_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QKeyEventTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: & QKeyEventTransition) -> RetType;
}

  // proto:  const QMetaObject * QKeyEventTransition::metaObject();
impl<'a> /*trait*/ QKeyEventTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: & QKeyEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QKeyEventTransition10metaObjectEv()};
     unsafe {_ZNK19QKeyEventTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QKeyEventTransition::~QKeyEventTransition();
impl /*struct*/ QKeyEventTransition {
  pub fn Free<RetType, T: QKeyEventTransition_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QKeyEventTransition_Free<RetType> {
  fn Free(self , rsthis: & QKeyEventTransition) -> RetType;
}

  // proto:  void QKeyEventTransition::~QKeyEventTransition();
impl<'a> /*trait*/ QKeyEventTransition_Free<()> for () {
  fn Free(self , rsthis: & QKeyEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionD0Ev()};
     unsafe {_ZN19QKeyEventTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QKeyEventTransition::key();
impl /*struct*/ QKeyEventTransition {
  pub fn key<RetType, T: QKeyEventTransition_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QKeyEventTransition_key<RetType> {
  fn key(self , rsthis: & QKeyEventTransition) -> RetType;
}

  // proto:  int QKeyEventTransition::key();
impl<'a> /*trait*/ QKeyEventTransition_key<i32> for () {
  fn key(self , rsthis: & QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QKeyEventTransition3keyEv()};
    let mut ret = unsafe {_ZNK19QKeyEventTransition3keyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QKeyEventTransition::QKeyEventTransition(QState * sourceState);
impl /*struct*/ QKeyEventTransition {
  pub fn New<T: QKeyEventTransition_New>(value: T) -> QKeyEventTransition {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEventTransition_New {
  fn New(self) -> QKeyEventTransition;
}

  // proto:  void QKeyEventTransition::QKeyEventTransition(QState * sourceState);
impl<'a> /*trait*/ QKeyEventTransition_New for (&'a QState) {
  fn New(self) -> QKeyEventTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionC1EP6QState()};
    let ctysz: c_int = unsafe{QKeyEventTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QKeyEventTransitionC1EP6QState(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QKeyEventTransitionC1EP6QState(arg0)} as u64;
    let rsthis = QKeyEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeyEventTransition::QKeyEventTransition(const QKeyEventTransition & );
impl<'a> /*trait*/ QKeyEventTransition_New for (&'a QKeyEventTransition) {
  fn New(self) -> QKeyEventTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionC1ERKS_()};
    let ctysz: c_int = unsafe{QKeyEventTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QKeyEventTransitionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QKeyEventTransitionC1ERKS_(arg0)} as u64;
    let rsthis = QKeyEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

