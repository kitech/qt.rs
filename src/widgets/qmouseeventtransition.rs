// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::super::core::qeventtransition::QEventTransition; // 771
use std::ops::Deref;
use super::super::core::qstate::QState; // 771
use super::super::core::qobject::QObject; // 771
use super::super::gui::qpainterpath::QPainterPath; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  const QMetaObject * QMouseEventTransition::metaObject();
  fn _ZNK21QMouseEventTransition10metaObjectEv(qthis: *mut c_void);
  // proto:  void QMouseEventTransition::QMouseEventTransition(QState * sourceState);
  fn _ZN21QMouseEventTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMouseEventTransition::QMouseEventTransition(const QMouseEventTransition & );
  fn _ZN21QMouseEventTransitionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
  fn _ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMouseEventTransition::~QMouseEventTransition();
  fn _ZN21QMouseEventTransitionD0Ev(qthis: *mut c_void);
  // proto:  QPainterPath QMouseEventTransition::hitTestPath();
  fn _ZNK21QMouseEventTransition11hitTestPathEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QMouseEventTransition)=1
pub struct QMouseEventTransition {
  qbase: QEventTransition,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMouseEventTransition {
  pub fn inheritFrom(qthis: *mut c_void) -> QMouseEventTransition {
    return QMouseEventTransition{qbase: QEventTransition::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMouseEventTransition {
  type Target = QEventTransition;

  fn deref(&self) -> &QEventTransition {
    return &self.qbase;
  }
}
impl AsRef<QEventTransition> for QMouseEventTransition {
  fn as_ref(&self) -> &QEventTransition {
    return &self.qbase;
  }
}
  // proto:  const QMetaObject * QMouseEventTransition::metaObject();
impl /*struct*/ QMouseEventTransition {
  pub fn metaObject<RetType, T: QMouseEventTransition_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QMouseEventTransition) -> RetType;
}

  // proto:  const QMetaObject * QMouseEventTransition::metaObject();
impl<'a> /*trait*/ QMouseEventTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QMouseEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QMouseEventTransition10metaObjectEv()};
     unsafe {_ZNK21QMouseEventTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::QMouseEventTransition(QState * sourceState);
impl /*struct*/ QMouseEventTransition {
  pub fn NewQMouseEventTransition<T: QMouseEventTransition_NewQMouseEventTransition>(value: T) -> QMouseEventTransition {
    let rsthis = value.NewQMouseEventTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEventTransition_NewQMouseEventTransition {
  fn NewQMouseEventTransition(self) -> QMouseEventTransition;
}

  // proto:  void QMouseEventTransition::QMouseEventTransition(QState * sourceState);
impl<'a> /*trait*/ QMouseEventTransition_NewQMouseEventTransition for (QState) {
  fn NewQMouseEventTransition(self) -> QMouseEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QMouseEventTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QMouseEventTransition{/**/qbase: QEventTransition::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::QMouseEventTransition(const QMouseEventTransition & );
impl<'a> /*trait*/ QMouseEventTransition_NewQMouseEventTransition for (QMouseEventTransition) {
  fn NewQMouseEventTransition(self) -> QMouseEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QMouseEventTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QMouseEventTransition{/**/qbase: QEventTransition::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
impl /*struct*/ QMouseEventTransition {
  pub fn setHitTestPath<RetType, T: QMouseEventTransition_setHitTestPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHitTestPath(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_setHitTestPath<RetType> {
  fn setHitTestPath(self , rsthis: &mut QMouseEventTransition) -> RetType;
}

  // proto:  void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
impl<'a> /*trait*/ QMouseEventTransition_setHitTestPath<()> for (QPainterPath) {
  fn setHitTestPath(self , rsthis: &mut QMouseEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMouseEventTransition::~QMouseEventTransition();
impl /*struct*/ QMouseEventTransition {
  pub fn FreeQMouseEventTransition<RetType, T: QMouseEventTransition_FreeQMouseEventTransition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMouseEventTransition(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_FreeQMouseEventTransition<RetType> {
  fn FreeQMouseEventTransition(self , rsthis: &mut QMouseEventTransition) -> RetType;
}

  // proto:  void QMouseEventTransition::~QMouseEventTransition();
impl<'a> /*trait*/ QMouseEventTransition_FreeQMouseEventTransition<()> for () {
  fn FreeQMouseEventTransition(self , rsthis: &mut QMouseEventTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionD0Ev()};
     unsafe {_ZN21QMouseEventTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QMouseEventTransition::hitTestPath();
impl /*struct*/ QMouseEventTransition {
  pub fn hitTestPath<RetType, T: QMouseEventTransition_hitTestPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hitTestPath(self);
    // return 1;
  }
}

pub trait QMouseEventTransition_hitTestPath<RetType> {
  fn hitTestPath(self , rsthis: &mut QMouseEventTransition) -> RetType;
}

  // proto:  QPainterPath QMouseEventTransition::hitTestPath();
impl<'a> /*trait*/ QMouseEventTransition_hitTestPath<QPainterPath> for () {
  fn hitTestPath(self , rsthis: &mut QMouseEventTransition) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QMouseEventTransition11hitTestPathEv()};
    let mut ret = unsafe {_ZNK21QMouseEventTransition11hitTestPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

