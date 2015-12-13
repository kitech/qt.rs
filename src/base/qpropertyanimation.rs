// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK18QPropertyAnimation12propertyNameEv() -> i32;
  fn _ZN18QPropertyAnimation15setTargetObjectEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN18QPropertyAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN18QPropertyAnimationD0Ev() -> i32;
  fn _ZNK18QPropertyAnimation12targetObjectEv() -> i32;
  fn _ZNK18QPropertyAnimation10metaObjectEv() -> i32;
  fn _ZN18QPropertyAnimationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN18QPropertyAnimationC1EP7QObjectRK10QByteArrayS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  fn _ZN18QPropertyAnimation15setPropertyNameERK10QByteArray(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPropertyAnimation)=1
pub struct QPropertyAnimation {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPropertyAnimation {
  pub fn propertyName<T: QPropertyAnimation_propertyName>(&mut self, value: T) -> i32 {
    value.propertyName(self);
    return 1;
  }
}

pub trait QPropertyAnimation_propertyName {
  fn propertyName(self, this: &mut QPropertyAnimation) -> i32;
}

// proto: QByteArray QPropertyAnimation::propertyName();
impl<'a> /*trait*/ QPropertyAnimation_propertyName for () {
  fn propertyName(self, this: &mut QPropertyAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation12propertyNameEv()};
    unsafe {_ZNK18QPropertyAnimation12propertyNameEv()};
    return 1;
  }
}

impl /*struct*/ QPropertyAnimation {
  pub fn setTargetObject<T: QPropertyAnimation_setTargetObject>(&mut self, value: T) -> i32 {
    value.setTargetObject(self);
    return 1;
  }
}

pub trait QPropertyAnimation_setTargetObject {
  fn setTargetObject(self, this: &mut QPropertyAnimation) -> i32;
}

// proto: void QPropertyAnimation::setTargetObject(QObject * target);
impl<'a> /*trait*/ QPropertyAnimation_setTargetObject for (&'a mut QObject) {
  fn setTargetObject(self, this: &mut QPropertyAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimation15setTargetObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QPropertyAnimation15setTargetObjectEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QPropertyAnimation {
  pub fn NewQPropertyAnimation<T: QPropertyAnimation_NewQPropertyAnimation>(value: T) -> QPropertyAnimation {
    let rsthis = value.NewQPropertyAnimation();
    return rsthis;
    // return 1;
  }
}

pub trait QPropertyAnimation_NewQPropertyAnimation {
  fn NewQPropertyAnimation(self) -> QPropertyAnimation;
}

// proto: void QPropertyAnimation::NewQPropertyAnimation(QObject * parent);
impl<'a> /*trait*/ QPropertyAnimation_NewQPropertyAnimation for (&'a mut QObject) {
  fn NewQPropertyAnimation(self) -> QPropertyAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QPropertyAnimationC1EP7QObject(qthis, arg0)};
    let rsthis = QPropertyAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPropertyAnimation {
  pub fn FreeQPropertyAnimation<T: QPropertyAnimation_FreeQPropertyAnimation>(&mut self, value: T) -> i32 {
    value.FreeQPropertyAnimation(self);
    return 1;
  }
}

pub trait QPropertyAnimation_FreeQPropertyAnimation {
  fn FreeQPropertyAnimation(self, this: &mut QPropertyAnimation) -> i32;
}

// proto: void QPropertyAnimation::FreeQPropertyAnimation();
impl<'a> /*trait*/ QPropertyAnimation_FreeQPropertyAnimation for () {
  fn FreeQPropertyAnimation(self, this: &mut QPropertyAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationD0Ev()};
    unsafe {_ZN18QPropertyAnimationD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPropertyAnimation {
  pub fn targetObject<T: QPropertyAnimation_targetObject>(&mut self, value: T) -> i32 {
    value.targetObject(self);
    return 1;
  }
}

pub trait QPropertyAnimation_targetObject {
  fn targetObject(self, this: &mut QPropertyAnimation) -> i32;
}

// proto: QObject * QPropertyAnimation::targetObject();
impl<'a> /*trait*/ QPropertyAnimation_targetObject for () {
  fn targetObject(self, this: &mut QPropertyAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation12targetObjectEv()};
    unsafe {_ZNK18QPropertyAnimation12targetObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPropertyAnimation {
  pub fn metaObject<T: QPropertyAnimation_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPropertyAnimation_metaObject {
  fn metaObject(self, this: &mut QPropertyAnimation) -> i32;
}

// proto: const QMetaObject * QPropertyAnimation::metaObject();
impl<'a> /*trait*/ QPropertyAnimation_metaObject for () {
  fn metaObject(self, this: &mut QPropertyAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation10metaObjectEv()};
    unsafe {_ZNK18QPropertyAnimation10metaObjectEv()};
    return 1;
  }
}

// proto: void QPropertyAnimation::NewQPropertyAnimation(const QPropertyAnimation & );
impl<'a> /*trait*/ QPropertyAnimation_NewQPropertyAnimation for (&'a  QPropertyAnimation) {
  fn NewQPropertyAnimation(self) -> QPropertyAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QPropertyAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QPropertyAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPropertyAnimation::NewQPropertyAnimation(QObject * target, const QByteArray & propertyName, QObject * parent);
impl<'a> /*trait*/ QPropertyAnimation_NewQPropertyAnimation for (&'a mut QObject, &'a  QByteArray, &'a mut QObject) {
  fn NewQPropertyAnimation(self) -> QPropertyAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC1EP7QObjectRK10QByteArrayS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN18QPropertyAnimationC1EP7QObjectRK10QByteArrayS1_(qthis, arg0, arg1, arg2)};
    let rsthis = QPropertyAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPropertyAnimation {
  pub fn setPropertyName<T: QPropertyAnimation_setPropertyName>(&mut self, value: T) -> i32 {
    value.setPropertyName(self);
    return 1;
  }
}

pub trait QPropertyAnimation_setPropertyName {
  fn setPropertyName(self, this: &mut QPropertyAnimation) -> i32;
}

// proto: void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
impl<'a> /*trait*/ QPropertyAnimation_setPropertyName for (&'a  QByteArray) {
  fn setPropertyName(self, this: &mut QPropertyAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray(arg0)};
    return 1;
  }
}

