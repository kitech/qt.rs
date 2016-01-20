// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlcontext.h
// dst-file: /src/qml/qqmlcontext.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qurl::QUrl; // 771
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlContext_Class_Size() -> c_int;
  // proto:  void QQmlContext::QQmlContext(QQmlEngine * , bool );
  fn _ZN11QQmlContextC2EP10QQmlEngineb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QQmlContext::QQmlContext(QQmlContext * parent, QObject * objParent);
  fn _ZN11QQmlContextC2EPS_P7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QObject * QQmlContext::contextObject();
  fn _ZNK11QQmlContext13contextObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QQmlContext::metaObject();
  fn _ZNK11QQmlContext10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlContext::setContextProperty(const QString & , QObject * );
  fn _ZN11QQmlContext18setContextPropertyERK7QStringP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QQmlContext::QQmlContext(const QQmlContext & );
  fn _ZN11QQmlContextC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QQmlEngine * QQmlContext::engine();
  fn _ZNK11QQmlContext6engineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUrl QQmlContext::baseUrl();
  fn _ZNK11QQmlContext7baseUrlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QQmlContext::contextProperty(const QString & );
  fn _ZNK11QQmlContext15contextPropertyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlContext::setContextObject(QObject * );
  fn _ZN11QQmlContext16setContextObjectEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QQmlContext * QQmlContext::parentContext();
  fn _ZNK11QQmlContext13parentContextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUrl QQmlContext::resolvedUrl(const QUrl & );
  fn _ZN11QQmlContext11resolvedUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlContext::setBaseUrl(const QUrl & );
  fn _ZN11QQmlContext10setBaseUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QQmlContext::nameForObject(QObject * );
  fn _ZNK11QQmlContext13nameForObjectEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlContext::~QQmlContext();
  fn _ZN11QQmlContextD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlContext::isValid();
  fn _ZNK11QQmlContext7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlContext::QQmlContext(QQmlEngine * parent, QObject * objParent);
  fn _ZN11QQmlContextC2EP10QQmlEngineP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QQmlContext::setContextProperty(const QString & , const QVariant & );
  fn _ZN11QQmlContext18setContextPropertyERK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlContext)=1
#[derive(Default)]
pub struct QQmlContext {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlContext {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlContext {
    return QQmlContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlContext {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQmlContext {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQmlContext::QQmlContext(QQmlEngine * , bool );
impl /*struct*/ QQmlContext {
  pub fn new<T: QQmlContext_new>(value: T) -> QQmlContext {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlContext_new {
  fn new(self) -> QQmlContext;
}

  // proto:  void QQmlContext::QQmlContext(QQmlEngine * , bool );
impl<'a> /*trait*/ QQmlContext_new for (&'a QQmlEngine, i8) {
  fn new(self) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContextC2EP10QQmlEngineb()};
    let ctysz: c_int = unsafe{QQmlContext_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    unsafe {_ZN11QQmlContextC2EP10QQmlEngineb(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlContext::QQmlContext(QQmlContext * parent, QObject * objParent);
impl<'a> /*trait*/ QQmlContext_new for (&'a QQmlContext, &'a QObject) {
  fn new(self) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContextC2EPS_P7QObject()};
    let ctysz: c_int = unsafe{QQmlContext_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QQmlContextC2EPS_P7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QQmlContext::contextObject();
impl /*struct*/ QQmlContext {
  pub fn contextObject<RetType, T: QQmlContext_contextObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contextObject(self);
    // return 1;
  }
}

pub trait QQmlContext_contextObject<RetType> {
  fn contextObject(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QObject * QQmlContext::contextObject();
impl<'a> /*trait*/ QQmlContext_contextObject<QObject> for () {
  fn contextObject(self , rsthis: & QQmlContext) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext13contextObjectEv()};
    let mut ret = unsafe {_ZNK11QQmlContext13contextObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlContext::metaObject();
impl /*struct*/ QQmlContext {
  pub fn metaObject<RetType, T: QQmlContext_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlContext_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  const QMetaObject * QQmlContext::metaObject();
impl<'a> /*trait*/ QQmlContext_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext10metaObjectEv()};
     unsafe {_ZNK11QQmlContext10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlContext::setContextProperty(const QString & , QObject * );
impl /*struct*/ QQmlContext {
  pub fn setContextProperty<RetType, T: QQmlContext_setContextProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContextProperty(self);
    // return 1;
  }
}

pub trait QQmlContext_setContextProperty<RetType> {
  fn setContextProperty(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  void QQmlContext::setContextProperty(const QString & , QObject * );
impl<'a> /*trait*/ QQmlContext_setContextProperty<()> for (&'a QString, &'a QObject) {
  fn setContextProperty(self , rsthis: & QQmlContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContext18setContextPropertyERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QQmlContext18setContextPropertyERK7QStringP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QQmlContext::QQmlContext(const QQmlContext & );
impl<'a> /*trait*/ QQmlContext_new for (&'a QQmlContext) {
  fn new(self) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContextC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlContext_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QQmlContextC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QQmlEngine * QQmlContext::engine();
impl /*struct*/ QQmlContext {
  pub fn engine<RetType, T: QQmlContext_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QQmlContext_engine<RetType> {
  fn engine(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QQmlEngine * QQmlContext::engine();
impl<'a> /*trait*/ QQmlContext_engine<QQmlEngine> for () {
  fn engine(self , rsthis: & QQmlContext) -> QQmlEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext6engineEv()};
    let mut ret = unsafe {_ZNK11QQmlContext6engineEv(rsthis.qclsinst)};
    let mut ret1 = QQmlEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QQmlContext::baseUrl();
impl /*struct*/ QQmlContext {
  pub fn baseUrl<RetType, T: QQmlContext_baseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseUrl(self);
    // return 1;
  }
}

pub trait QQmlContext_baseUrl<RetType> {
  fn baseUrl(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QUrl QQmlContext::baseUrl();
impl<'a> /*trait*/ QQmlContext_baseUrl<QUrl> for () {
  fn baseUrl(self , rsthis: & QQmlContext) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext7baseUrlEv()};
    let mut ret = unsafe {_ZNK11QQmlContext7baseUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QQmlContext::contextProperty(const QString & );
impl /*struct*/ QQmlContext {
  pub fn contextProperty<RetType, T: QQmlContext_contextProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contextProperty(self);
    // return 1;
  }
}

pub trait QQmlContext_contextProperty<RetType> {
  fn contextProperty(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QVariant QQmlContext::contextProperty(const QString & );
impl<'a> /*trait*/ QQmlContext_contextProperty<QVariant> for (&'a QString) {
  fn contextProperty(self , rsthis: & QQmlContext) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext15contextPropertyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QQmlContext15contextPropertyERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlContext::setContextObject(QObject * );
impl /*struct*/ QQmlContext {
  pub fn setContextObject<RetType, T: QQmlContext_setContextObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContextObject(self);
    // return 1;
  }
}

pub trait QQmlContext_setContextObject<RetType> {
  fn setContextObject(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  void QQmlContext::setContextObject(QObject * );
impl<'a> /*trait*/ QQmlContext_setContextObject<()> for (&'a QObject) {
  fn setContextObject(self , rsthis: & QQmlContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContext16setContextObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QQmlContext16setContextObjectEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QQmlContext * QQmlContext::parentContext();
impl /*struct*/ QQmlContext {
  pub fn parentContext<RetType, T: QQmlContext_parentContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentContext(self);
    // return 1;
  }
}

pub trait QQmlContext_parentContext<RetType> {
  fn parentContext(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QQmlContext * QQmlContext::parentContext();
impl<'a> /*trait*/ QQmlContext_parentContext<QQmlContext> for () {
  fn parentContext(self , rsthis: & QQmlContext) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext13parentContextEv()};
    let mut ret = unsafe {_ZNK11QQmlContext13parentContextEv(rsthis.qclsinst)};
    let mut ret1 = QQmlContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QQmlContext::resolvedUrl(const QUrl & );
impl /*struct*/ QQmlContext {
  pub fn resolvedUrl<RetType, T: QQmlContext_resolvedUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolvedUrl(self);
    // return 1;
  }
}

pub trait QQmlContext_resolvedUrl<RetType> {
  fn resolvedUrl(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QUrl QQmlContext::resolvedUrl(const QUrl & );
impl<'a> /*trait*/ QQmlContext_resolvedUrl<QUrl> for (&'a QUrl) {
  fn resolvedUrl(self , rsthis: & QQmlContext) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContext11resolvedUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QQmlContext11resolvedUrlERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlContext::setBaseUrl(const QUrl & );
impl /*struct*/ QQmlContext {
  pub fn setBaseUrl<RetType, T: QQmlContext_setBaseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaseUrl(self);
    // return 1;
  }
}

pub trait QQmlContext_setBaseUrl<RetType> {
  fn setBaseUrl(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  void QQmlContext::setBaseUrl(const QUrl & );
impl<'a> /*trait*/ QQmlContext_setBaseUrl<()> for (&'a QUrl) {
  fn setBaseUrl(self , rsthis: & QQmlContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContext10setBaseUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QQmlContext10setBaseUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QQmlContext::nameForObject(QObject * );
impl /*struct*/ QQmlContext {
  pub fn nameForObject<RetType, T: QQmlContext_nameForObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameForObject(self);
    // return 1;
  }
}

pub trait QQmlContext_nameForObject<RetType> {
  fn nameForObject(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  QString QQmlContext::nameForObject(QObject * );
impl<'a> /*trait*/ QQmlContext_nameForObject<QString> for (&'a QObject) {
  fn nameForObject(self , rsthis: & QQmlContext) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext13nameForObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QQmlContext13nameForObjectEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlContext::~QQmlContext();
impl /*struct*/ QQmlContext {
  pub fn free<RetType, T: QQmlContext_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlContext_free<RetType> {
  fn free(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  void QQmlContext::~QQmlContext();
impl<'a> /*trait*/ QQmlContext_free<()> for () {
  fn free(self , rsthis: & QQmlContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContextD2Ev()};
     unsafe {_ZN11QQmlContextD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlContext::isValid();
impl /*struct*/ QQmlContext {
  pub fn isValid<RetType, T: QQmlContext_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QQmlContext_isValid<RetType> {
  fn isValid(self , rsthis: & QQmlContext) -> RetType;
}

  // proto:  bool QQmlContext::isValid();
impl<'a> /*trait*/ QQmlContext_isValid<i8> for () {
  fn isValid(self , rsthis: & QQmlContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQmlContext7isValidEv()};
    let mut ret = unsafe {_ZNK11QQmlContext7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlContext::QQmlContext(QQmlEngine * parent, QObject * objParent);
impl<'a> /*trait*/ QQmlContext_new for (&'a QQmlEngine, &'a QObject) {
  fn new(self) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContextC2EP10QQmlEngineP7QObject()};
    let ctysz: c_int = unsafe{QQmlContext_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QQmlContextC2EP10QQmlEngineP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlContext::setContextProperty(const QString & , const QVariant & );
impl<'a> /*trait*/ QQmlContext_setContextProperty<()> for (&'a QString, &'a QVariant) {
  fn setContextProperty(self , rsthis: & QQmlContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQmlContext18setContextPropertyERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QQmlContext18setContextPropertyERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

