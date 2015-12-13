// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstate::QState;
use super::qpainterpath::QPainterPath;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QMouseEventTransition::metaObject();
  fn _ZNK21QMouseEventTransition10metaObjectEv() -> i32;
  // proto: void QMouseEventTransition::NewQMouseEventTransition(QState * sourceState);
  fn _ZN21QMouseEventTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QMouseEventTransition::NewQMouseEventTransition(const QMouseEventTransition & );
  fn _ZN21QMouseEventTransitionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
  fn _ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QMouseEventTransition::FreeQMouseEventTransition();
  fn _ZN21QMouseEventTransitionD0Ev() -> i32;
  // proto: QPainterPath QMouseEventTransition::hitTestPath();
  fn _ZNK21QMouseEventTransition11hitTestPathEv() -> i32;
}

// body block begin
// class sizeof(QMouseEventTransition)=1
pub struct QMouseEventTransition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMouseEventTransition {
  pub fn metaObject<T: QMouseEventTransition_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMouseEventTransition_metaObject {
  fn metaObject(self, this: &mut QMouseEventTransition) -> i32;
}

// proto: const QMetaObject * QMouseEventTransition::metaObject();
impl<'a> /*trait*/ QMouseEventTransition_metaObject for () {
  fn metaObject(self, this: &mut QMouseEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QMouseEventTransition10metaObjectEv()};
    unsafe {_ZNK21QMouseEventTransition10metaObjectEv()};
    return 1;
  }
}

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

// proto: void QMouseEventTransition::NewQMouseEventTransition(QState * sourceState);
impl<'a> /*trait*/ QMouseEventTransition_NewQMouseEventTransition for (&'a mut QState) {
  fn NewQMouseEventTransition(self) -> QMouseEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QMouseEventTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QMouseEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMouseEventTransition::NewQMouseEventTransition(const QMouseEventTransition & );
impl<'a> /*trait*/ QMouseEventTransition_NewQMouseEventTransition for (&'a  QMouseEventTransition) {
  fn NewQMouseEventTransition(self) -> QMouseEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QMouseEventTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QMouseEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMouseEventTransition {
  pub fn setHitTestPath<T: QMouseEventTransition_setHitTestPath>(&mut self, value: T) -> i32 {
    value.setHitTestPath(self);
    return 1;
  }
}

pub trait QMouseEventTransition_setHitTestPath {
  fn setHitTestPath(self, this: &mut QMouseEventTransition) -> i32;
}

// proto: void QMouseEventTransition::setHitTestPath(const QPainterPath & path);
impl<'a> /*trait*/ QMouseEventTransition_setHitTestPath for (&'a  QPainterPath) {
  fn setHitTestPath(self, this: &mut QMouseEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QMouseEventTransition {
  pub fn FreeQMouseEventTransition<T: QMouseEventTransition_FreeQMouseEventTransition>(&mut self, value: T) -> i32 {
    value.FreeQMouseEventTransition(self);
    return 1;
  }
}

pub trait QMouseEventTransition_FreeQMouseEventTransition {
  fn FreeQMouseEventTransition(self, this: &mut QMouseEventTransition) -> i32;
}

// proto: void QMouseEventTransition::FreeQMouseEventTransition();
impl<'a> /*trait*/ QMouseEventTransition_FreeQMouseEventTransition for () {
  fn FreeQMouseEventTransition(self, this: &mut QMouseEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QMouseEventTransitionD0Ev()};
    unsafe {_ZN21QMouseEventTransitionD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMouseEventTransition {
  pub fn hitTestPath<T: QMouseEventTransition_hitTestPath>(&mut self, value: T) -> i32 {
    value.hitTestPath(self);
    return 1;
  }
}

pub trait QMouseEventTransition_hitTestPath {
  fn hitTestPath(self, this: &mut QMouseEventTransition) -> i32;
}

// proto: QPainterPath QMouseEventTransition::hitTestPath();
impl<'a> /*trait*/ QMouseEventTransition_hitTestPath for () {
  fn hitTestPath(self, this: &mut QMouseEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QMouseEventTransition11hitTestPathEv()};
    unsafe {_ZNK21QMouseEventTransition11hitTestPathEv()};
    return 1;
  }
}

