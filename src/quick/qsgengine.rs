// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgengine.h
// dst-file: /src/quick/qsgengine.rs
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
use super::super::gui::qopenglcontext::QOpenGLContext; // 771
use super::super::core::qsize::QSize; // 771
use super::qsgabstractrenderer::QSGAbstractRenderer; // 773
use super::super::gui::qimage::QImage; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGEngine_Class_Size() -> c_int;
  // proto:  const QMetaObject * QSGEngine::metaObject();
  fn _ZNK9QSGEngine10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGEngine::initialize(QOpenGLContext * context);
  fn _ZN9QSGEngine10initializeEP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSGAbstractRenderer * QSGEngine::createRenderer();
  fn _ZNK9QSGEngine14createRendererEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGEngine::~QSGEngine();
  fn _ZN9QSGEngineD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGEngine::QSGEngine(QObject * parent);
  fn _ZN9QSGEngineC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGEngine::invalidate();
  fn _ZN9QSGEngine10invalidateEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSGEngine)=1
#[derive(Default)]
pub struct QSGEngine {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGEngine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGEngine {
    return QSGEngine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGEngine {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSGEngine {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QSGEngine::metaObject();
impl /*struct*/ QSGEngine {
  pub fn metaObject<RetType, T: QSGEngine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSGEngine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSGEngine) -> RetType;
}

  // proto:  const QMetaObject * QSGEngine::metaObject();
impl<'a> /*trait*/ QSGEngine_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSGEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSGEngine10metaObjectEv()};
     unsafe {_ZNK9QSGEngine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGEngine::initialize(QOpenGLContext * context);
impl /*struct*/ QSGEngine {
  pub fn initialize<RetType, T: QSGEngine_initialize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initialize(self);
    // return 1;
  }
}

pub trait QSGEngine_initialize<RetType> {
  fn initialize(self , rsthis: & QSGEngine) -> RetType;
}

  // proto:  void QSGEngine::initialize(QOpenGLContext * context);
impl<'a> /*trait*/ QSGEngine_initialize<()> for (&'a QOpenGLContext) {
  fn initialize(self , rsthis: & QSGEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSGEngine10initializeEP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSGEngine10initializeEP14QOpenGLContext(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSGAbstractRenderer * QSGEngine::createRenderer();
impl /*struct*/ QSGEngine {
  pub fn createRenderer<RetType, T: QSGEngine_createRenderer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createRenderer(self);
    // return 1;
  }
}

pub trait QSGEngine_createRenderer<RetType> {
  fn createRenderer(self , rsthis: & QSGEngine) -> RetType;
}

  // proto:  QSGAbstractRenderer * QSGEngine::createRenderer();
impl<'a> /*trait*/ QSGEngine_createRenderer<QSGAbstractRenderer> for () {
  fn createRenderer(self , rsthis: & QSGEngine) -> QSGAbstractRenderer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSGEngine14createRendererEv()};
    let mut ret = unsafe {_ZNK9QSGEngine14createRendererEv(rsthis.qclsinst)};
    let mut ret1 = QSGAbstractRenderer::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGEngine::~QSGEngine();
impl /*struct*/ QSGEngine {
  pub fn free<RetType, T: QSGEngine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGEngine_free<RetType> {
  fn free(self , rsthis: & QSGEngine) -> RetType;
}

  // proto:  void QSGEngine::~QSGEngine();
impl<'a> /*trait*/ QSGEngine_free<()> for () {
  fn free(self , rsthis: & QSGEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSGEngineD2Ev()};
     unsafe {_ZN9QSGEngineD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGEngine::QSGEngine(QObject * parent);
impl /*struct*/ QSGEngine {
  pub fn new<T: QSGEngine_new>(value: T) -> QSGEngine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGEngine_new {
  fn new(self) -> QSGEngine;
}

  // proto:  void QSGEngine::QSGEngine(QObject * parent);
impl<'a> /*trait*/ QSGEngine_new for (&'a QObject) {
  fn new(self) -> QSGEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSGEngineC2EP7QObject()};
    let ctysz: c_int = unsafe{QSGEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSGEngineC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGEngine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSGEngine::invalidate();
impl /*struct*/ QSGEngine {
  pub fn invalidate<RetType, T: QSGEngine_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QSGEngine_invalidate<RetType> {
  fn invalidate(self , rsthis: & QSGEngine) -> RetType;
}

  // proto:  void QSGEngine::invalidate();
impl<'a> /*trait*/ QSGEngine_invalidate<()> for () {
  fn invalidate(self , rsthis: & QSGEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSGEngine10invalidateEv()};
     unsafe {_ZN9QSGEngine10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

