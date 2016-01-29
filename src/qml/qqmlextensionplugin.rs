// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlextensionplugin.h
// dst-file: /src/qml/qqmlextensionplugin.rs
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
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qurl::QUrl; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlExtensionPlugin_Class_Size() -> c_int;
  // proto:  void QQmlExtensionPlugin::QQmlExtensionPlugin(QObject * parent);
  fn _ZN19QQmlExtensionPluginC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlExtensionPlugin::QQmlExtensionPlugin(const QQmlExtensionPlugin & );
  fn _ZN19QQmlExtensionPluginC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlExtensionPlugin::initializeEngine(QQmlEngine * engine, const char * uri);
  fn _ZN19QQmlExtensionPlugin16initializeEngineEP10QQmlEnginePKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QQmlExtensionPlugin::registerTypes(const char * uri);
  fn _ZN19QQmlExtensionPlugin13registerTypesEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  const QMetaObject * QQmlExtensionPlugin::metaObject();
  fn _ZNK19QQmlExtensionPlugin10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlExtensionPlugin::~QQmlExtensionPlugin();
  fn _ZN19QQmlExtensionPluginD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QUrl QQmlExtensionPlugin::baseUrl();
  fn _ZNK19QQmlExtensionPlugin7baseUrlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QQmlExtensionPlugin)=1
#[derive(Default)]
pub struct QQmlExtensionPlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlExtensionPlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlExtensionPlugin {
    return QQmlExtensionPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlExtensionPlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQmlExtensionPlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQmlExtensionPlugin::QQmlExtensionPlugin(QObject * parent);
impl /*struct*/ QQmlExtensionPlugin {
  pub fn new<T: QQmlExtensionPlugin_new>(value: T) -> QQmlExtensionPlugin {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlExtensionPlugin_new {
  fn new(self) -> QQmlExtensionPlugin;
}

  // proto:  void QQmlExtensionPlugin::QQmlExtensionPlugin(QObject * parent);
impl<'a> /*trait*/ QQmlExtensionPlugin_new for (&'a QObject) {
  fn new(self) -> QQmlExtensionPlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQmlExtensionPluginC2EP7QObject()};
    let ctysz: c_int = unsafe{QQmlExtensionPlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QQmlExtensionPluginC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlExtensionPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlExtensionPlugin::QQmlExtensionPlugin(const QQmlExtensionPlugin & );
impl<'a> /*trait*/ QQmlExtensionPlugin_new for (&'a QQmlExtensionPlugin) {
  fn new(self) -> QQmlExtensionPlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQmlExtensionPluginC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlExtensionPlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QQmlExtensionPluginC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlExtensionPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlExtensionPlugin::initializeEngine(QQmlEngine * engine, const char * uri);
impl /*struct*/ QQmlExtensionPlugin {
  pub fn initializeEngine<RetType, T: QQmlExtensionPlugin_initializeEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initializeEngine(self);
    // return 1;
  }
}

pub trait QQmlExtensionPlugin_initializeEngine<RetType> {
  fn initializeEngine(self , rsthis: & QQmlExtensionPlugin) -> RetType;
}

  // proto:  void QQmlExtensionPlugin::initializeEngine(QQmlEngine * engine, const char * uri);
impl<'a> /*trait*/ QQmlExtensionPlugin_initializeEngine<()> for (&'a QQmlEngine, &'a  String) {
  fn initializeEngine(self , rsthis: & QQmlExtensionPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQmlExtensionPlugin16initializeEngineEP10QQmlEnginePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN19QQmlExtensionPlugin16initializeEngineEP10QQmlEnginePKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QQmlExtensionPlugin::registerTypes(const char * uri);
impl /*struct*/ QQmlExtensionPlugin {
  pub fn registerTypes<RetType, T: QQmlExtensionPlugin_registerTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.registerTypes(self);
    // return 1;
  }
}

pub trait QQmlExtensionPlugin_registerTypes<RetType> {
  fn registerTypes(self , rsthis: & QQmlExtensionPlugin) -> RetType;
}

  // proto:  void QQmlExtensionPlugin::registerTypes(const char * uri);
impl<'a> /*trait*/ QQmlExtensionPlugin_registerTypes<()> for (&'a  String) {
  fn registerTypes(self , rsthis: & QQmlExtensionPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQmlExtensionPlugin13registerTypesEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN19QQmlExtensionPlugin13registerTypesEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlExtensionPlugin::metaObject();
impl /*struct*/ QQmlExtensionPlugin {
  pub fn metaObject<RetType, T: QQmlExtensionPlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlExtensionPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlExtensionPlugin) -> RetType;
}

  // proto:  const QMetaObject * QQmlExtensionPlugin::metaObject();
impl<'a> /*trait*/ QQmlExtensionPlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlExtensionPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QQmlExtensionPlugin10metaObjectEv()};
     unsafe {_ZNK19QQmlExtensionPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlExtensionPlugin::~QQmlExtensionPlugin();
impl /*struct*/ QQmlExtensionPlugin {
  pub fn free<RetType, T: QQmlExtensionPlugin_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlExtensionPlugin_free<RetType> {
  fn free(self , rsthis: & QQmlExtensionPlugin) -> RetType;
}

  // proto:  void QQmlExtensionPlugin::~QQmlExtensionPlugin();
impl<'a> /*trait*/ QQmlExtensionPlugin_free<()> for () {
  fn free(self , rsthis: & QQmlExtensionPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQmlExtensionPluginD2Ev()};
     unsafe {_ZN19QQmlExtensionPluginD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QUrl QQmlExtensionPlugin::baseUrl();
impl /*struct*/ QQmlExtensionPlugin {
  pub fn baseUrl<RetType, T: QQmlExtensionPlugin_baseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseUrl(self);
    // return 1;
  }
}

pub trait QQmlExtensionPlugin_baseUrl<RetType> {
  fn baseUrl(self , rsthis: & QQmlExtensionPlugin) -> RetType;
}

  // proto:  QUrl QQmlExtensionPlugin::baseUrl();
impl<'a> /*trait*/ QQmlExtensionPlugin_baseUrl<QUrl> for () {
  fn baseUrl(self , rsthis: & QQmlExtensionPlugin) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QQmlExtensionPlugin7baseUrlEv()};
    let mut ret = unsafe {_ZNK19QQmlExtensionPlugin7baseUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

