// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qiconengineplugin.h
// dst-file: /src/gui/qiconengineplugin.rs
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
use super::super::core::qstring::*; // 771
use super::qiconengine::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QIconEnginePlugin_Class_Size() -> c_int;
  // proto:  const QMetaObject * QIconEnginePlugin::metaObject();
  fn C_ZNK17QIconEnginePlugin10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QIconEnginePlugin::QIconEnginePlugin(QObject * parent);
  fn C_ZN17QIconEnginePluginC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QIconEngine * QIconEnginePlugin::create(const QString & filename);
  fn C_ZN17QIconEnginePlugin6createERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QIconEnginePlugin::~QIconEnginePlugin();
  fn C_ZN17QIconEnginePluginD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QIconEnginePlugin)=1
#[derive(Default)]
pub struct QIconEnginePlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIconEnginePlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIconEnginePlugin {
    return QIconEnginePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QIconEnginePlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QIconEnginePlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QIconEnginePlugin::metaObject();
impl /*struct*/ QIconEnginePlugin {
  pub fn metaObject<RetType, T: QIconEnginePlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIconEnginePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QIconEnginePlugin) -> RetType;
}

  // proto:  const QMetaObject * QIconEnginePlugin::metaObject();
impl<'a> /*trait*/ QIconEnginePlugin_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QIconEnginePlugin) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QIconEnginePlugin10metaObjectEv()};
    let mut ret = unsafe {C_ZNK17QIconEnginePlugin10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIconEnginePlugin::QIconEnginePlugin(QObject * parent);
impl /*struct*/ QIconEnginePlugin {
  pub fn new<T: QIconEnginePlugin_new>(value: T) -> QIconEnginePlugin {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIconEnginePlugin_new {
  fn new(self) -> QIconEnginePlugin;
}

  // proto:  void QIconEnginePlugin::QIconEnginePlugin(QObject * parent);
impl<'a> /*trait*/ QIconEnginePlugin_new for (Option<&'a QObject>) {
  fn new(self) -> QIconEnginePlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePluginC2EP7QObject()};
    let ctysz: c_int = unsafe{QIconEnginePlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QIconEnginePluginC2EP7QObject(arg0)};
    let rsthis = QIconEnginePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QIconEngine * QIconEnginePlugin::create(const QString & filename);
impl /*struct*/ QIconEnginePlugin {
  pub fn create<RetType, T: QIconEnginePlugin_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QIconEnginePlugin_create<RetType> {
  fn create(self , rsthis: & QIconEnginePlugin) -> RetType;
}

  // proto:  QIconEngine * QIconEnginePlugin::create(const QString & filename);
impl<'a> /*trait*/ QIconEnginePlugin_create<QIconEngine> for (Option<&'a QString>) {
  fn create(self , rsthis: & QIconEnginePlugin) -> QIconEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePlugin6createERK7QString()};
    let arg0 = (if self.is_none() {QString::new(()).qclsinst} else {self.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN17QIconEnginePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QIconEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIconEnginePlugin::~QIconEnginePlugin();
impl /*struct*/ QIconEnginePlugin {
  pub fn free<RetType, T: QIconEnginePlugin_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIconEnginePlugin_free<RetType> {
  fn free(self , rsthis: & QIconEnginePlugin) -> RetType;
}

  // proto:  void QIconEnginePlugin::~QIconEnginePlugin();
impl<'a> /*trait*/ QIconEnginePlugin_free<()> for () {
  fn free(self , rsthis: & QIconEnginePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePluginD2Ev()};
     unsafe {C_ZN17QIconEnginePluginD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

