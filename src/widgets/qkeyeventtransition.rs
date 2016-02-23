// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::super::core::qeventtransition::*; // 771
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qobject::*; // 771
use super::super::core::qstate::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QKeyEventTransition_Class_Size() -> c_int;
  // proto:  void QKeyEventTransition::setKey(int key);
  fn C_ZN19QKeyEventTransition6setKeyEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QKeyEventTransition::metaObject();
  fn C_ZNK19QKeyEventTransition10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QKeyEventTransition::~QKeyEventTransition();
  fn C_ZN19QKeyEventTransitionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QKeyEventTransition::key();
  fn C_ZNK19QKeyEventTransition3keyEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QKeyEventTransition::QKeyEventTransition(QState * sourceState);
  fn C_ZN19QKeyEventTransitionC2EP6QState(arg0: *mut c_void) -> u64;
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
     unsafe {C_ZN19QKeyEventTransition6setKeyEi(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QKeyEventTransition_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QKeyEventTransition) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QKeyEventTransition10metaObjectEv()};
    let mut ret = unsafe {C_ZNK19QKeyEventTransition10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QKeyEventTransition::~QKeyEventTransition();
impl /*struct*/ QKeyEventTransition {
  pub fn free<RetType, T: QKeyEventTransition_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QKeyEventTransition_free<RetType> {
  fn free(self , rsthis: & QKeyEventTransition) -> RetType;
}

  // proto:  void QKeyEventTransition::~QKeyEventTransition();
impl<'a> /*trait*/ QKeyEventTransition_free<()> for () {
  fn free(self , rsthis: & QKeyEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionD2Ev()};
     unsafe {C_ZN19QKeyEventTransitionD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK19QKeyEventTransition3keyEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QKeyEventTransition::QKeyEventTransition(QState * sourceState);
impl /*struct*/ QKeyEventTransition {
  pub fn new<T: QKeyEventTransition_new>(value: T) -> QKeyEventTransition {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEventTransition_new {
  fn new(self) -> QKeyEventTransition;
}

  // proto:  void QKeyEventTransition::QKeyEventTransition(QState * sourceState);
impl<'a> /*trait*/ QKeyEventTransition_new for (Option<&'a QState>) {
  fn new(self) -> QKeyEventTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionC2EP6QState()};
    let ctysz: c_int = unsafe{QKeyEventTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QKeyEventTransitionC2EP6QState(arg0)};
    let rsthis = QKeyEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

