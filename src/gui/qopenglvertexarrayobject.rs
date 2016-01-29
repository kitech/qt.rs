// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLVertexArrayObject_Class_Size() -> c_int;
  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
  fn C_ZNK24QOpenGLVertexArrayObject8objectIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLVertexArrayObject::release();
  fn C_ZN24QOpenGLVertexArrayObject7releaseEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
  fn C_ZNK24QOpenGLVertexArrayObject10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
  fn C_ZN24QOpenGLVertexArrayObjectC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QOpenGLVertexArrayObject::bind();
  fn C_ZN24QOpenGLVertexArrayObject4bindEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLVertexArrayObject::isCreated();
  fn C_ZNK24QOpenGLVertexArrayObject9isCreatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLVertexArrayObject::destroy();
  fn C_ZN24QOpenGLVertexArrayObject7destroyEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLVertexArrayObject::create();
  fn C_ZN24QOpenGLVertexArrayObject6createEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
  fn C_ZN24QOpenGLVertexArrayObjectD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLVertexArrayObject)=1
#[derive(Default)]
pub struct QOpenGLVertexArrayObject {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVertexArrayObject {
    return QOpenGLVertexArrayObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLVertexArrayObject {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLVertexArrayObject {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn objectId<RetType, T: QOpenGLVertexArrayObject_objectId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_objectId<RetType> {
  fn objectId(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_objectId<u32> for () {
  fn objectId(self , rsthis: & QOpenGLVertexArrayObject) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject8objectIdEv()};
    let mut ret = unsafe {C_ZNK24QOpenGLVertexArrayObject8objectIdEv(rsthis.qclsinst)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::release();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn release<RetType, T: QOpenGLVertexArrayObject_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_release<RetType> {
  fn release(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::release();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_release<()> for () {
  fn release(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7releaseEv()};
     unsafe {C_ZN24QOpenGLVertexArrayObject7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn metaObject<RetType, T: QOpenGLVertexArrayObject_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOpenGLVertexArrayObject) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject10metaObjectEv()};
    let mut ret = unsafe {C_ZNK24QOpenGLVertexArrayObject10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn new<T: QOpenGLVertexArrayObject_new>(value: T) -> QOpenGLVertexArrayObject {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_new {
  fn new(self) -> QOpenGLVertexArrayObject;
}

  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
impl<'a> /*trait*/ QOpenGLVertexArrayObject_new for (&'a QObject) {
  fn new(self) -> QOpenGLVertexArrayObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC2EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLVertexArrayObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN24QOpenGLVertexArrayObjectC2EP7QObject(arg0)};
    let rsthis = QOpenGLVertexArrayObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::bind();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn bind<RetType, T: QOpenGLVertexArrayObject_bind<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_bind<RetType> {
  fn bind(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::bind();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_bind<()> for () {
  fn bind(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject4bindEv()};
     unsafe {C_ZN24QOpenGLVertexArrayObject4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVertexArrayObject::isCreated();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn isCreated<RetType, T: QOpenGLVertexArrayObject_isCreated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_isCreated<RetType> {
  fn isCreated(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  bool QOpenGLVertexArrayObject::isCreated();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_isCreated<i8> for () {
  fn isCreated(self , rsthis: & QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject9isCreatedEv()};
    let mut ret = unsafe {C_ZNK24QOpenGLVertexArrayObject9isCreatedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::destroy();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn destroy<RetType, T: QOpenGLVertexArrayObject_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_destroy<RetType> {
  fn destroy(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::destroy();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_destroy<()> for () {
  fn destroy(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7destroyEv()};
     unsafe {C_ZN24QOpenGLVertexArrayObject7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVertexArrayObject::create();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn create<RetType, T: QOpenGLVertexArrayObject_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_create<RetType> {
  fn create(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  bool QOpenGLVertexArrayObject::create();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_create<i8> for () {
  fn create(self , rsthis: & QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject6createEv()};
    let mut ret = unsafe {C_ZN24QOpenGLVertexArrayObject6createEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn free<RetType, T: QOpenGLVertexArrayObject_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_free<RetType> {
  fn free(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_free<()> for () {
  fn free(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectD2Ev()};
     unsafe {C_ZN24QOpenGLVertexArrayObjectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

