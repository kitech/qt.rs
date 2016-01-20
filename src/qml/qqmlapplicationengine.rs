// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlapplicationengine.h
// dst-file: /src/qml/qqmlapplicationengine.rs
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
use super::qqmlengine::QQmlEngine; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qurl::QUrl; // 771
use super::super::core::qbytearray::QByteArray; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlApplicationEngine_Class_Size() -> c_int;
  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QString & filePath, QObject * parent);
  fn _ZN21QQmlApplicationEngineC2ERK7QStringP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QQmlApplicationEngine::~QQmlApplicationEngine();
  fn _ZN21QQmlApplicationEngineD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlApplicationEngine::load(const QString & filePath);
  fn _ZN21QQmlApplicationEngine4loadERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(QObject * parent);
  fn _ZN21QQmlApplicationEngineC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QUrl & url, QObject * parent);
  fn _ZN21QQmlApplicationEngineC2ERK4QUrlP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QQmlApplicationEngine::load(const QUrl & url);
  fn _ZN21QQmlApplicationEngine4loadERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlApplicationEngine::loadData(const QByteArray & data, const QUrl & url);
  fn _ZN21QQmlApplicationEngine8loadDataERK10QByteArrayRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QMetaObject * QQmlApplicationEngine::metaObject();
  fn _ZNK21QQmlApplicationEngine10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QQmlApplicationEngine & );
  fn _ZN21QQmlApplicationEngineC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QObject *> QQmlApplicationEngine::rootObjects();
  fn _ZN21QQmlApplicationEngine11rootObjectsEv(qthis: u64 /* *mut c_void*/);
  fn QQmlApplicationEngine_SlotProxy_connect__ZN21QQmlApplicationEngine13objectCreatedEP7QObjectRK4QUrl(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlApplicationEngine)=1
#[derive(Default)]
pub struct QQmlApplicationEngine {
  qbase: QQmlEngine,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _objectCreated: QQmlApplicationEngine_objectCreated_signal,
}

impl /*struct*/ QQmlApplicationEngine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlApplicationEngine {
    return QQmlApplicationEngine{qbase: QQmlEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlApplicationEngine {
  type Target = QQmlEngine;

  fn deref(&self) -> &QQmlEngine {
    return & self.qbase;
  }
}
impl AsRef<QQmlEngine> for QQmlApplicationEngine {
  fn as_ref(& self) -> & QQmlEngine {
    return & self.qbase;
  }
}
  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QString & filePath, QObject * parent);
impl /*struct*/ QQmlApplicationEngine {
  pub fn new<T: QQmlApplicationEngine_new>(value: T) -> QQmlApplicationEngine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlApplicationEngine_new {
  fn new(self) -> QQmlApplicationEngine;
}

  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QString & filePath, QObject * parent);
impl<'a> /*trait*/ QQmlApplicationEngine_new for (&'a QString, &'a QObject) {
  fn new(self) -> QQmlApplicationEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngineC2ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QQmlApplicationEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN21QQmlApplicationEngineC2ERK7QStringP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlApplicationEngine{qbase: QQmlEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::~QQmlApplicationEngine();
impl /*struct*/ QQmlApplicationEngine {
  pub fn free<RetType, T: QQmlApplicationEngine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlApplicationEngine_free<RetType> {
  fn free(self , rsthis: & QQmlApplicationEngine) -> RetType;
}

  // proto:  void QQmlApplicationEngine::~QQmlApplicationEngine();
impl<'a> /*trait*/ QQmlApplicationEngine_free<()> for () {
  fn free(self , rsthis: & QQmlApplicationEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngineD2Ev()};
     unsafe {_ZN21QQmlApplicationEngineD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::load(const QString & filePath);
impl /*struct*/ QQmlApplicationEngine {
  pub fn load<RetType, T: QQmlApplicationEngine_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QQmlApplicationEngine_load<RetType> {
  fn load(self , rsthis: & QQmlApplicationEngine) -> RetType;
}

  // proto:  void QQmlApplicationEngine::load(const QString & filePath);
impl<'a> /*trait*/ QQmlApplicationEngine_load<()> for (&'a QString) {
  fn load(self , rsthis: & QQmlApplicationEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngine4loadERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QQmlApplicationEngine4loadERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(QObject * parent);
impl<'a> /*trait*/ QQmlApplicationEngine_new for (&'a QObject) {
  fn new(self) -> QQmlApplicationEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngineC2EP7QObject()};
    let ctysz: c_int = unsafe{QQmlApplicationEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QQmlApplicationEngineC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlApplicationEngine{qbase: QQmlEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QUrl & url, QObject * parent);
impl<'a> /*trait*/ QQmlApplicationEngine_new for (&'a QUrl, &'a QObject) {
  fn new(self) -> QQmlApplicationEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngineC2ERK4QUrlP7QObject()};
    let ctysz: c_int = unsafe{QQmlApplicationEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN21QQmlApplicationEngineC2ERK4QUrlP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlApplicationEngine{qbase: QQmlEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::load(const QUrl & url);
impl<'a> /*trait*/ QQmlApplicationEngine_load<()> for (&'a QUrl) {
  fn load(self , rsthis: & QQmlApplicationEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngine4loadERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QQmlApplicationEngine4loadERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::loadData(const QByteArray & data, const QUrl & url);
impl /*struct*/ QQmlApplicationEngine {
  pub fn loadData<RetType, T: QQmlApplicationEngine_loadData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadData(self);
    // return 1;
  }
}

pub trait QQmlApplicationEngine_loadData<RetType> {
  fn loadData(self , rsthis: & QQmlApplicationEngine) -> RetType;
}

  // proto:  void QQmlApplicationEngine::loadData(const QByteArray & data, const QUrl & url);
impl<'a> /*trait*/ QQmlApplicationEngine_loadData<()> for (&'a QByteArray, &'a QUrl) {
  fn loadData(self , rsthis: & QQmlApplicationEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngine8loadDataERK10QByteArrayRK4QUrl()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN21QQmlApplicationEngine8loadDataERK10QByteArrayRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlApplicationEngine::metaObject();
impl /*struct*/ QQmlApplicationEngine {
  pub fn metaObject<RetType, T: QQmlApplicationEngine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlApplicationEngine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlApplicationEngine) -> RetType;
}

  // proto:  const QMetaObject * QQmlApplicationEngine::metaObject();
impl<'a> /*trait*/ QQmlApplicationEngine_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlApplicationEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QQmlApplicationEngine10metaObjectEv()};
     unsafe {_ZNK21QQmlApplicationEngine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlApplicationEngine::QQmlApplicationEngine(const QQmlApplicationEngine & );
impl<'a> /*trait*/ QQmlApplicationEngine_new for (&'a QQmlApplicationEngine) {
  fn new(self) -> QQmlApplicationEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngineC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlApplicationEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QQmlApplicationEngineC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlApplicationEngine{qbase: QQmlEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QObject *> QQmlApplicationEngine::rootObjects();
impl /*struct*/ QQmlApplicationEngine {
  pub fn rootObjects<RetType, T: QQmlApplicationEngine_rootObjects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootObjects(self);
    // return 1;
  }
}

pub trait QQmlApplicationEngine_rootObjects<RetType> {
  fn rootObjects(self , rsthis: & QQmlApplicationEngine) -> RetType;
}

  // proto:  QList<QObject *> QQmlApplicationEngine::rootObjects();
impl<'a> /*trait*/ QQmlApplicationEngine_rootObjects<()> for () {
  fn rootObjects(self , rsthis: & QQmlApplicationEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlApplicationEngine11rootObjectsEv()};
     unsafe {_ZN21QQmlApplicationEngine11rootObjectsEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QQmlApplicationEngine_objectCreated
pub struct QQmlApplicationEngine_objectCreated_signal{poi:u64}
impl /* struct */ QQmlApplicationEngine {
  pub fn objectCreated(&self) -> QQmlApplicationEngine_objectCreated_signal {
     return QQmlApplicationEngine_objectCreated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlApplicationEngine_objectCreated_signal {
  pub fn connect<T: QQmlApplicationEngine_objectCreated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlApplicationEngine_objectCreated_signal_connect {
  fn connect(self, sigthis: QQmlApplicationEngine_objectCreated_signal);
}

// objectCreated(class QObject *, const class QUrl &)
extern fn QQmlApplicationEngine_objectCreated_signal_connect_cb_0(rsfptr:fn(QObject, QUrl), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  let rsarg1 = QUrl::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QQmlApplicationEngine_objectCreated_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QObject, QUrl)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  let rsarg1 = QUrl::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QQmlApplicationEngine_objectCreated_signal_connect for fn(QObject, QUrl) {
  fn connect(self, sigthis: QQmlApplicationEngine_objectCreated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlApplicationEngine_objectCreated_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQmlApplicationEngine_SlotProxy_connect__ZN21QQmlApplicationEngine13objectCreatedEP7QObjectRK4QUrl(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQmlApplicationEngine_objectCreated_signal_connect for Box<Fn(QObject, QUrl)> {
  fn connect(self, sigthis: QQmlApplicationEngine_objectCreated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlApplicationEngine_objectCreated_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQmlApplicationEngine_SlotProxy_connect__ZN21QQmlApplicationEngine13objectCreatedEP7QObjectRK4QUrl(arg0, arg1, arg2)};
  }
}
// <= body block end

