// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qmouseeventtransition.h
// dst-file: /src/widgets/qmouseeventtransition.rs
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
use super::super::core::qstate::*; // 771
use super::super::core::qobject::*; // 771
use super::super::gui::qpainterpath::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMouseEventTransition_Class_Size() -> c_int;
  // proto:  const QMetaObject * QMouseEventTransition::metaObject();
  fn C_ZNK21QMouseEventTransition10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMouseEventTransition::QMouseEventTransition(QState * sourceState);
  fn C_ZN21QMouseEventTransitionC2EP6QState(arg0: *mut c_void) -> u64;
  // proto:  void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
  fn C_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMouseEventTransition::~QMouseEventTransition();
  fn C_ZN21QMouseEventTransitionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QMouseEventTransition::hitTestPath();
  fn C_ZNK21QMouseEventTransition11hitTestPathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QMouseEventTransition)=1
#[derive(Default)]
pub struct QMouseEventTransition {
  qbase: QEventTransition,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMouseEventTransition {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMouseEventTransition {
    return QMouseEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMouseEventTransition {
  type Target = QEventTransition;

  fn deref(&self) -> &QEventTransition {
    return & self.qbase;
  }
}
impl AsRef<QEventTransition> for QMouseEventTransition {
  fn as_ref(& self) -> & QEventTransition {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QMouseEventTransition::metaObject();
impl /*struct*/ QMouseEventTransition {
  pub fn metaObject<RetType, T: QMouseEventTransition_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMouseEventTransition) -> RetType;
}

  // proto:  const QMetaObject * QMouseEventTransition::metaObject();
impl<'a> /*trait*/ QMouseEventTransition_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QMouseEventTransition) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QMouseEventTransition10metaObjectEv()};
    let mut ret = unsafe {C_ZNK21QMouseEventTransition10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::QMouseEventTransition(QState * sourceState);
impl /*struct*/ QMouseEventTransition {
  pub fn new<T: QMouseEventTransition_new>(value: T) -> QMouseEventTransition {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEventTransition_new {
  fn new(self) -> QMouseEventTransition;
}

  // proto:  void QMouseEventTransition::QMouseEventTransition(QState * sourceState);
impl<'a> /*trait*/ QMouseEventTransition_new for (Option<&'a QState>) {
  fn new(self) -> QMouseEventTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionC2EP6QState()};
    let ctysz: c_int = unsafe{QMouseEventTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QMouseEventTransitionC2EP6QState(arg0)};
    let rsthis = QMouseEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
impl /*struct*/ QMouseEventTransition {
  pub fn setHitTestPath<RetType, T: QMouseEventTransition_setHitTestPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHitTestPath(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_setHitTestPath<RetType> {
  fn setHitTestPath(self , rsthis: & QMouseEventTransition) -> RetType;
}

  // proto:  void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
impl<'a> /*trait*/ QMouseEventTransition_setHitTestPath<()> for (&'a QPainterPath) {
  fn setHitTestPath(self , rsthis: & QMouseEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::~QMouseEventTransition();
impl /*struct*/ QMouseEventTransition {
  pub fn free<RetType, T: QMouseEventTransition_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_free<RetType> {
  fn free(self , rsthis: & QMouseEventTransition) -> RetType;
}

  // proto:  void QMouseEventTransition::~QMouseEventTransition();
impl<'a> /*trait*/ QMouseEventTransition_free<()> for () {
  fn free(self , rsthis: & QMouseEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionD2Ev()};
     unsafe {C_ZN21QMouseEventTransitionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QMouseEventTransition::hitTestPath();
impl /*struct*/ QMouseEventTransition {
  pub fn hitTestPath<RetType, T: QMouseEventTransition_hitTestPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hitTestPath(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_hitTestPath<RetType> {
  fn hitTestPath(self , rsthis: & QMouseEventTransition) -> RetType;
}

  // proto:  QPainterPath QMouseEventTransition::hitTestPath();
impl<'a> /*trait*/ QMouseEventTransition_hitTestPath<QPainterPath> for () {
  fn hitTestPath(self , rsthis: & QMouseEventTransition) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QMouseEventTransition11hitTestPathEv()};
    let mut ret = unsafe {C_ZNK21QMouseEventTransition11hitTestPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

