// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qopenglvertexarrayobject.h
// dst-file: /src/gui/qopenglvertexarrayobject.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
  fn _ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
  fn _ZNK24QOpenGLVertexArrayObject8objectIdEv(qthis: *mut c_void);
  // proto:  void QOpenGLVertexArrayObject::release();
  fn _ZN24QOpenGLVertexArrayObject7releaseEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
  fn _ZNK24QOpenGLVertexArrayObject10metaObjectEv(qthis: *mut c_void);
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
  fn _ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLVertexArrayObject::bind();
  fn _ZN24QOpenGLVertexArrayObject4bindEv(qthis: *mut c_void);
  // proto:  bool QOpenGLVertexArrayObject::isCreated();
  fn _ZNK24QOpenGLVertexArrayObject9isCreatedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLVertexArrayObject::destroy();
  fn _ZN24QOpenGLVertexArrayObject7destroyEv(qthis: *mut c_void);
  // proto:  bool QOpenGLVertexArrayObject::create();
  fn _ZN24QOpenGLVertexArrayObject6createEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
  fn _ZN24QOpenGLVertexArrayObjectD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLVertexArrayObject)=1
pub struct QOpenGLVertexArrayObject {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLVertexArrayObject {
    return QOpenGLVertexArrayObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLVertexArrayObject {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return &self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLVertexArrayObject {
  fn as_ref(&self) -> &QObject {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn NewQOpenGLVertexArrayObject<T: QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject>(value: T) -> QOpenGLVertexArrayObject {
    let rsthis = value.NewQOpenGLVertexArrayObject();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject;
}

  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
impl<'a> /*trait*/ QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject for (QOpenGLVertexArrayObject) {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLVertexArrayObject{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn objectId<RetType, T: QOpenGLVertexArrayObject_objectId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_objectId<RetType> {
  fn objectId(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_objectId<()> for () {
  fn objectId(self , rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject8objectIdEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject8objectIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::release();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn release<RetType, T: QOpenGLVertexArrayObject_release<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_release<RetType> {
  fn release(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::release();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_release<()> for () {
  fn release(self , rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7releaseEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn metaObject<RetType, T: QOpenGLVertexArrayObject_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject10metaObjectEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
impl<'a> /*trait*/ QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject for (QObject) {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLVertexArrayObject{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::bind();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn bind<RetType, T: QOpenGLVertexArrayObject_bind<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_bind<RetType> {
  fn bind(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::bind();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_bind<()> for () {
  fn bind(self , rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject4bindEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVertexArrayObject::isCreated();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn isCreated<RetType, T: QOpenGLVertexArrayObject_isCreated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_isCreated<RetType> {
  fn isCreated(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  bool QOpenGLVertexArrayObject::isCreated();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_isCreated<i8> for () {
  fn isCreated(self , rsthis: &mut QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject9isCreatedEv()};
    let mut ret = unsafe {_ZNK24QOpenGLVertexArrayObject9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::destroy();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn destroy<RetType, T: QOpenGLVertexArrayObject_destroy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_destroy<RetType> {
  fn destroy(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::destroy();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_destroy<()> for () {
  fn destroy(self , rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7destroyEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVertexArrayObject::create();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn create<RetType, T: QOpenGLVertexArrayObject_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_create<RetType> {
  fn create(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  bool QOpenGLVertexArrayObject::create();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_create<i8> for () {
  fn create(self , rsthis: &mut QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject6createEv()};
    let mut ret = unsafe {_ZN24QOpenGLVertexArrayObject6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn FreeQOpenGLVertexArrayObject<RetType, T: QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLVertexArrayObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject<RetType> {
  fn FreeQOpenGLVertexArrayObject(self , rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject<()> for () {
  fn FreeQOpenGLVertexArrayObject(self , rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectD0Ev()};
     unsafe {_ZN24QOpenGLVertexArrayObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

