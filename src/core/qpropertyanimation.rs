// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QByteArray QPropertyAnimation::propertyName();
  fn _ZNK18QPropertyAnimation12propertyNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPropertyAnimation::setTargetObject(QObject * target);
  fn _ZN18QPropertyAnimation15setTargetObjectEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * parent);
  fn _ZN18QPropertyAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPropertyAnimation::~QPropertyAnimation();
  fn _ZN18QPropertyAnimationD0Ev(qthis: *mut c_void);
  // proto:  QObject * QPropertyAnimation::targetObject();
  fn _ZNK18QPropertyAnimation12targetObjectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QPropertyAnimation::metaObject();
  fn _ZNK18QPropertyAnimation10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPropertyAnimation::QPropertyAnimation(const QPropertyAnimation & );
  fn _ZN18QPropertyAnimationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * target, const QByteArray & propertyName, QObject * parent);
  fn _ZN18QPropertyAnimationC1EP7QObjectRK10QByteArrayS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
  fn _ZN18QPropertyAnimation15setPropertyNameERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QPropertyAnimation)=1
pub struct QPropertyAnimation {
  pub qclsinst: *mut c_void,
}

  // proto:  QByteArray QPropertyAnimation::propertyName();
impl /*struct*/ QPropertyAnimation {
  pub fn propertyName<RetType, T: QPropertyAnimation_propertyName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.propertyName(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_propertyName<RetType> {
  fn propertyName(self , rsthis: &mut QPropertyAnimation) -> RetType;
}

  // proto:  QByteArray QPropertyAnimation::propertyName();
impl<'a> /*trait*/ QPropertyAnimation_propertyName<QByteArray> for () {
  fn propertyName(self , rsthis: &mut QPropertyAnimation) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation12propertyNameEv()};
    let mut ret = unsafe {_ZNK18QPropertyAnimation12propertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::setTargetObject(QObject * target);
impl /*struct*/ QPropertyAnimation {
  pub fn setTargetObject<RetType, T: QPropertyAnimation_setTargetObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTargetObject(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_setTargetObject<RetType> {
  fn setTargetObject(self , rsthis: &mut QPropertyAnimation) -> RetType;
}

  // proto:  void QPropertyAnimation::setTargetObject(QObject * target);
impl<'a> /*trait*/ QPropertyAnimation_setTargetObject<()> for (QObject) {
  fn setTargetObject(self , rsthis: &mut QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimation15setTargetObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QPropertyAnimation15setTargetObjectEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * parent);
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

  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * parent);
impl<'a> /*trait*/ QPropertyAnimation_NewQPropertyAnimation for (QObject) {
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

  // proto:  void QPropertyAnimation::~QPropertyAnimation();
impl /*struct*/ QPropertyAnimation {
  pub fn FreeQPropertyAnimation<RetType, T: QPropertyAnimation_FreeQPropertyAnimation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPropertyAnimation(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_FreeQPropertyAnimation<RetType> {
  fn FreeQPropertyAnimation(self , rsthis: &mut QPropertyAnimation) -> RetType;
}

  // proto:  void QPropertyAnimation::~QPropertyAnimation();
impl<'a> /*trait*/ QPropertyAnimation_FreeQPropertyAnimation<()> for () {
  fn FreeQPropertyAnimation(self , rsthis: &mut QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationD0Ev()};
     unsafe {_ZN18QPropertyAnimationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QPropertyAnimation::targetObject();
impl /*struct*/ QPropertyAnimation {
  pub fn targetObject<RetType, T: QPropertyAnimation_targetObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.targetObject(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_targetObject<RetType> {
  fn targetObject(self , rsthis: &mut QPropertyAnimation) -> RetType;
}

  // proto:  QObject * QPropertyAnimation::targetObject();
impl<'a> /*trait*/ QPropertyAnimation_targetObject<QObject> for () {
  fn targetObject(self , rsthis: &mut QPropertyAnimation) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation12targetObjectEv()};
    let mut ret = unsafe {_ZNK18QPropertyAnimation12targetObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPropertyAnimation::metaObject();
impl /*struct*/ QPropertyAnimation {
  pub fn metaObject<RetType, T: QPropertyAnimation_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPropertyAnimation) -> RetType;
}

  // proto:  const QMetaObject * QPropertyAnimation::metaObject();
impl<'a> /*trait*/ QPropertyAnimation_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation10metaObjectEv()};
     unsafe {_ZNK18QPropertyAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::QPropertyAnimation(const QPropertyAnimation & );
impl<'a> /*trait*/ QPropertyAnimation_NewQPropertyAnimation for (QPropertyAnimation) {
  fn NewQPropertyAnimation(self) -> QPropertyAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QPropertyAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QPropertyAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * target, const QByteArray & propertyName, QObject * parent);
impl<'a> /*trait*/ QPropertyAnimation_NewQPropertyAnimation for (QObject, QByteArray, QObject) {
  fn NewQPropertyAnimation(self) -> QPropertyAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC1EP7QObjectRK10QByteArrayS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN18QPropertyAnimationC1EP7QObjectRK10QByteArrayS1_(qthis, arg0, arg1, arg2)};
    let rsthis = QPropertyAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
impl /*struct*/ QPropertyAnimation {
  pub fn setPropertyName<RetType, T: QPropertyAnimation_setPropertyName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPropertyName(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_setPropertyName<RetType> {
  fn setPropertyName(self , rsthis: &mut QPropertyAnimation) -> RetType;
}

  // proto:  void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
impl<'a> /*trait*/ QPropertyAnimation_setPropertyName<()> for (QByteArray) {
  fn setPropertyName(self , rsthis: &mut QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

