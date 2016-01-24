// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qpropertyanimation.h
// dst-file: /src/core/qpropertyanimation.rs
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
use super::qvariantanimation::QVariantAnimation; // 773
use std::ops::Deref;
use super::qbytearray::QByteArray; // 773
use super::qobject::QObject; // 773
use super::qobjectdefs::QMetaObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPropertyAnimation_Class_Size() -> c_int;
  // proto:  QByteArray QPropertyAnimation::propertyName();
  fn C_ZNK18QPropertyAnimation12propertyNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPropertyAnimation::setTargetObject(QObject * target);
  fn C_ZN18QPropertyAnimation15setTargetObjectEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * parent);
  fn C_ZN18QPropertyAnimationC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QPropertyAnimation::~QPropertyAnimation();
  fn C_ZN18QPropertyAnimationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QPropertyAnimation::targetObject();
  fn C_ZNK18QPropertyAnimation12targetObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QPropertyAnimation::metaObject();
  fn C_ZNK18QPropertyAnimation10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * target, const QByteArray & propertyName, QObject * parent);
  fn C_ZN18QPropertyAnimationC2EP7QObjectRK10QByteArrayS1_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> u64;
  // proto:  void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
  fn C_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPropertyAnimation)=1
#[derive(Default)]
pub struct QPropertyAnimation {
  qbase: QVariantAnimation,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPropertyAnimation {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPropertyAnimation {
    return QPropertyAnimation{qbase: QVariantAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPropertyAnimation {
  type Target = QVariantAnimation;

  fn deref(&self) -> &QVariantAnimation {
    return & self.qbase;
  }
}
impl AsRef<QVariantAnimation> for QPropertyAnimation {
  fn as_ref(& self) -> & QVariantAnimation {
    return & self.qbase;
  }
}
  // proto:  QByteArray QPropertyAnimation::propertyName();
impl /*struct*/ QPropertyAnimation {
  pub fn propertyName<RetType, T: QPropertyAnimation_propertyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyName(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_propertyName<RetType> {
  fn propertyName(self , rsthis: & QPropertyAnimation) -> RetType;
}

  // proto:  QByteArray QPropertyAnimation::propertyName();
impl<'a> /*trait*/ QPropertyAnimation_propertyName<QByteArray> for () {
  fn propertyName(self , rsthis: & QPropertyAnimation) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation12propertyNameEv()};
    let mut ret = unsafe {C_ZNK18QPropertyAnimation12propertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::setTargetObject(QObject * target);
impl /*struct*/ QPropertyAnimation {
  pub fn setTargetObject<RetType, T: QPropertyAnimation_setTargetObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTargetObject(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_setTargetObject<RetType> {
  fn setTargetObject(self , rsthis: & QPropertyAnimation) -> RetType;
}

  // proto:  void QPropertyAnimation::setTargetObject(QObject * target);
impl<'a> /*trait*/ QPropertyAnimation_setTargetObject<()> for (&'a QObject) {
  fn setTargetObject(self , rsthis: & QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimation15setTargetObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QPropertyAnimation15setTargetObjectEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * parent);
impl /*struct*/ QPropertyAnimation {
  pub fn new<T: QPropertyAnimation_new>(value: T) -> QPropertyAnimation {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPropertyAnimation_new {
  fn new(self) -> QPropertyAnimation;
}

  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * parent);
impl<'a> /*trait*/ QPropertyAnimation_new for (&'a QObject) {
  fn new(self) -> QPropertyAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC2EP7QObject()};
    let ctysz: c_int = unsafe{QPropertyAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QPropertyAnimationC2EP7QObject(arg0)};
    let rsthis = QPropertyAnimation{qbase: QVariantAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::~QPropertyAnimation();
impl /*struct*/ QPropertyAnimation {
  pub fn free<RetType, T: QPropertyAnimation_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_free<RetType> {
  fn free(self , rsthis: & QPropertyAnimation) -> RetType;
}

  // proto:  void QPropertyAnimation::~QPropertyAnimation();
impl<'a> /*trait*/ QPropertyAnimation_free<()> for () {
  fn free(self , rsthis: & QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationD2Ev()};
     unsafe {C_ZN18QPropertyAnimationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QPropertyAnimation::targetObject();
impl /*struct*/ QPropertyAnimation {
  pub fn targetObject<RetType, T: QPropertyAnimation_targetObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.targetObject(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_targetObject<RetType> {
  fn targetObject(self , rsthis: & QPropertyAnimation) -> RetType;
}

  // proto:  QObject * QPropertyAnimation::targetObject();
impl<'a> /*trait*/ QPropertyAnimation_targetObject<QObject> for () {
  fn targetObject(self , rsthis: & QPropertyAnimation) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation12targetObjectEv()};
    let mut ret = unsafe {C_ZNK18QPropertyAnimation12targetObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPropertyAnimation::metaObject();
impl /*struct*/ QPropertyAnimation {
  pub fn metaObject<RetType, T: QPropertyAnimation_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPropertyAnimation) -> RetType;
}

  // proto:  const QMetaObject * QPropertyAnimation::metaObject();
impl<'a> /*trait*/ QPropertyAnimation_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPropertyAnimation) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPropertyAnimation10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QPropertyAnimation10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::QPropertyAnimation(QObject * target, const QByteArray & propertyName, QObject * parent);
impl<'a> /*trait*/ QPropertyAnimation_new for (&'a QObject, &'a QByteArray, &'a QObject) {
  fn new(self) -> QPropertyAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimationC2EP7QObjectRK10QByteArrayS1_()};
    let ctysz: c_int = unsafe{QPropertyAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QPropertyAnimationC2EP7QObjectRK10QByteArrayS1_(arg0, arg1, arg2)};
    let rsthis = QPropertyAnimation{qbase: QVariantAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
impl /*struct*/ QPropertyAnimation {
  pub fn setPropertyName<RetType, T: QPropertyAnimation_setPropertyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPropertyName(self);
    // return 1;
  }
}

pub trait QPropertyAnimation_setPropertyName<RetType> {
  fn setPropertyName(self , rsthis: & QPropertyAnimation) -> RetType;
}

  // proto:  void QPropertyAnimation::setPropertyName(const QByteArray & propertyName);
impl<'a> /*trait*/ QPropertyAnimation_setPropertyName<()> for (&'a QByteArray) {
  fn setPropertyName(self , rsthis: & QPropertyAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

