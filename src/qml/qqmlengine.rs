// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlengine.h
// dst-file: /src/qml/qqmlengine.rs
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
use super::qjsengine::QJSEngine; // 773
use std::ops::Deref;
use super::qqmlnetworkaccessmanagerfactory::QQmlNetworkAccessManagerFactory; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qobject::QObject; // 771
// use super::qqmlengine::QQmlImageProviderBase; // 773
use super::qqmlabstracturlinterceptor::QQmlAbstractUrlInterceptor; // 773
use super::super::core::qstringlist::QStringList; // 771
use super::qqmlcontext::QQmlContext; // 773
use super::qqmlincubator::QQmlIncubationController; // 773
use super::super::network::qnetworkaccessmanager::QNetworkAccessManager; // 771
use super::super::core::qurl::QUrl; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlEngine_Class_Size() -> c_int;
  // proto:  void QQmlEngine::setNetworkAccessManagerFactory(QQmlNetworkAccessManagerFactory * );
  fn _ZN10QQmlEngine30setNetworkAccessManagerFactoryEP31QQmlNetworkAccessManagerFactory(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QQmlEngine::outputWarningsToStandardError();
  fn _ZNK10QQmlEngine29outputWarningsToStandardErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlEngine::setOfflineStoragePath(const QString & dir);
  fn _ZN10QQmlEngine21setOfflineStoragePathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlEngine::trimComponentCache();
  fn _ZN10QQmlEngine18trimComponentCacheEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlEngine::addNamedBundle(const QString & name, const QString & fileName);
  fn _ZN10QQmlEngine14addNamedBundleERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QQmlEngine::addImageProvider(const QString & id, QQmlImageProviderBase * );
  fn _ZN10QQmlEngine16addImageProviderERK7QStringP21QQmlImageProviderBase(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QQmlEngine::setUrlInterceptor(QQmlAbstractUrlInterceptor * urlInterceptor);
  fn _ZN10QQmlEngine17setUrlInterceptorEP26QQmlAbstractUrlInterceptor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlEngine::setImportPathList(const QStringList & paths);
  fn _ZN10QQmlEngine17setImportPathListERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlEngine::addPluginPath(const QString & dir);
  fn _ZN10QQmlEngine13addPluginPathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlEngine::setPluginPathList(const QStringList & paths);
  fn _ZN10QQmlEngine17setPluginPathListERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static void QQmlEngine::setContextForObject(QObject * , QQmlContext * );
  fn _ZN10QQmlEngine19setContextForObjectEP7QObjectP11QQmlContext(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QQmlImageProviderBase * QQmlEngine::imageProvider(const QString & id);
  fn _ZNK10QQmlEngine13imageProviderERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlEngine::QQmlEngine(const QQmlEngine & );
  fn _ZN10QQmlEngineC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlEngine::~QQmlEngine();
  fn _ZN10QQmlEngineD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlEngine::QQmlEngine(QObject * p);
  fn _ZN10QQmlEngineC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QQmlContext * QQmlEngine::rootContext();
  fn _ZNK10QQmlEngine11rootContextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlEngine::addImportPath(const QString & dir);
  fn _ZN10QQmlEngine13addImportPathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlEngine::setIncubationController(QQmlIncubationController * );
  fn _ZN10QQmlEngine23setIncubationControllerEP24QQmlIncubationController(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QQmlEngine::importPathList();
  fn _ZNK10QQmlEngine14importPathListEv(qthis: u64 /* *mut c_void*/);
  // proto:  QQmlAbstractUrlInterceptor * QQmlEngine::urlInterceptor();
  fn _ZNK10QQmlEngine14urlInterceptorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QQmlEngine::offlineStoragePath();
  fn _ZNK10QQmlEngine18offlineStoragePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlEngine::removeImageProvider(const QString & id);
  fn _ZN10QQmlEngine19removeImageProviderERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkAccessManager * QQmlEngine::networkAccessManager();
  fn _ZNK10QQmlEngine20networkAccessManagerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlEngine::setBaseUrl(const QUrl & );
  fn _ZN10QQmlEngine10setBaseUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QUrl QQmlEngine::baseUrl();
  fn _ZNK10QQmlEngine7baseUrlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QQmlIncubationController * QQmlEngine::incubationController();
  fn _ZNK10QQmlEngine20incubationControllerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QQmlContext * QQmlEngine::contextForObject(const QObject * );
  fn _ZN10QQmlEngine16contextForObjectEPK7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto:  QQmlNetworkAccessManagerFactory * QQmlEngine::networkAccessManagerFactory();
  fn _ZNK10QQmlEngine27networkAccessManagerFactoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringList QQmlEngine::pluginPathList();
  fn _ZNK10QQmlEngine14pluginPathListEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QQmlEngine::metaObject();
  fn _ZNK10QQmlEngine10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlEngine::setOutputWarningsToStandardError(bool );
  fn _ZN10QQmlEngine32setOutputWarningsToStandardErrorEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QQmlEngine::clearComponentCache();
  fn _ZN10QQmlEngine19clearComponentCacheEv(qthis: u64 /* *mut c_void*/);
  fn QQmlImageProviderBase_Class_Size() -> c_int;
  // proto:  void QQmlImageProviderBase::QQmlImageProviderBase();
  fn _ZN21QQmlImageProviderBaseC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlImageProviderBase::~QQmlImageProviderBase();
  fn _ZN21QQmlImageProviderBaseD2Ev(qthis: u64 /* *mut c_void*/);
  fn QQmlEngine_SlotProxy_connect__ZN10QQmlEngine4quitEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlEngine)=1
#[derive(Default)]
pub struct QQmlEngine {
  qbase: QJSEngine,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _quit: QQmlEngine_quit_signal,
  pub _warnings: QQmlEngine_warnings_signal,
}

// class sizeof(QQmlImageProviderBase)=8
#[derive(Default)]
pub struct QQmlImageProviderBase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlEngine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlEngine {
    return QQmlEngine{qbase: QJSEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlEngine {
  type Target = QJSEngine;

  fn deref(&self) -> &QJSEngine {
    return & self.qbase;
  }
}
impl AsRef<QJSEngine> for QQmlEngine {
  fn as_ref(& self) -> & QJSEngine {
    return & self.qbase;
  }
}
  // proto:  void QQmlEngine::setNetworkAccessManagerFactory(QQmlNetworkAccessManagerFactory * );
impl /*struct*/ QQmlEngine {
  pub fn setNetworkAccessManagerFactory<RetType, T: QQmlEngine_setNetworkAccessManagerFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNetworkAccessManagerFactory(self);
    // return 1;
  }
}

pub trait QQmlEngine_setNetworkAccessManagerFactory<RetType> {
  fn setNetworkAccessManagerFactory(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setNetworkAccessManagerFactory(QQmlNetworkAccessManagerFactory * );
impl<'a> /*trait*/ QQmlEngine_setNetworkAccessManagerFactory<()> for (&'a QQmlNetworkAccessManagerFactory) {
  fn setNetworkAccessManagerFactory(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine30setNetworkAccessManagerFactoryEP31QQmlNetworkAccessManagerFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine30setNetworkAccessManagerFactoryEP31QQmlNetworkAccessManagerFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQmlEngine::outputWarningsToStandardError();
impl /*struct*/ QQmlEngine {
  pub fn outputWarningsToStandardError<RetType, T: QQmlEngine_outputWarningsToStandardError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.outputWarningsToStandardError(self);
    // return 1;
  }
}

pub trait QQmlEngine_outputWarningsToStandardError<RetType> {
  fn outputWarningsToStandardError(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  bool QQmlEngine::outputWarningsToStandardError();
impl<'a> /*trait*/ QQmlEngine_outputWarningsToStandardError<i8> for () {
  fn outputWarningsToStandardError(self , rsthis: & QQmlEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine29outputWarningsToStandardErrorEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine29outputWarningsToStandardErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlEngine::setOfflineStoragePath(const QString & dir);
impl /*struct*/ QQmlEngine {
  pub fn setOfflineStoragePath<RetType, T: QQmlEngine_setOfflineStoragePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOfflineStoragePath(self);
    // return 1;
  }
}

pub trait QQmlEngine_setOfflineStoragePath<RetType> {
  fn setOfflineStoragePath(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setOfflineStoragePath(const QString & dir);
impl<'a> /*trait*/ QQmlEngine_setOfflineStoragePath<()> for (&'a QString) {
  fn setOfflineStoragePath(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine21setOfflineStoragePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine21setOfflineStoragePathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::trimComponentCache();
impl /*struct*/ QQmlEngine {
  pub fn trimComponentCache<RetType, T: QQmlEngine_trimComponentCache<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.trimComponentCache(self);
    // return 1;
  }
}

pub trait QQmlEngine_trimComponentCache<RetType> {
  fn trimComponentCache(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::trimComponentCache();
impl<'a> /*trait*/ QQmlEngine_trimComponentCache<()> for () {
  fn trimComponentCache(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine18trimComponentCacheEv()};
     unsafe {_ZN10QQmlEngine18trimComponentCacheEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlEngine::addNamedBundle(const QString & name, const QString & fileName);
impl /*struct*/ QQmlEngine {
  pub fn addNamedBundle<RetType, T: QQmlEngine_addNamedBundle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addNamedBundle(self);
    // return 1;
  }
}

pub trait QQmlEngine_addNamedBundle<RetType> {
  fn addNamedBundle(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  bool QQmlEngine::addNamedBundle(const QString & name, const QString & fileName);
impl<'a> /*trait*/ QQmlEngine_addNamedBundle<i8> for (&'a QString, &'a QString) {
  fn addNamedBundle(self , rsthis: & QQmlEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine14addNamedBundleERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QQmlEngine14addNamedBundleERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlEngine::addImageProvider(const QString & id, QQmlImageProviderBase * );
impl /*struct*/ QQmlEngine {
  pub fn addImageProvider<RetType, T: QQmlEngine_addImageProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addImageProvider(self);
    // return 1;
  }
}

pub trait QQmlEngine_addImageProvider<RetType> {
  fn addImageProvider(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::addImageProvider(const QString & id, QQmlImageProviderBase * );
impl<'a> /*trait*/ QQmlEngine_addImageProvider<()> for (&'a QString, &'a QQmlImageProviderBase) {
  fn addImageProvider(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine16addImageProviderERK7QStringP21QQmlImageProviderBase()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine16addImageProviderERK7QStringP21QQmlImageProviderBase(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::setUrlInterceptor(QQmlAbstractUrlInterceptor * urlInterceptor);
impl /*struct*/ QQmlEngine {
  pub fn setUrlInterceptor<RetType, T: QQmlEngine_setUrlInterceptor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUrlInterceptor(self);
    // return 1;
  }
}

pub trait QQmlEngine_setUrlInterceptor<RetType> {
  fn setUrlInterceptor(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setUrlInterceptor(QQmlAbstractUrlInterceptor * urlInterceptor);
impl<'a> /*trait*/ QQmlEngine_setUrlInterceptor<()> for (&'a QQmlAbstractUrlInterceptor) {
  fn setUrlInterceptor(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine17setUrlInterceptorEP26QQmlAbstractUrlInterceptor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine17setUrlInterceptorEP26QQmlAbstractUrlInterceptor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::setImportPathList(const QStringList & paths);
impl /*struct*/ QQmlEngine {
  pub fn setImportPathList<RetType, T: QQmlEngine_setImportPathList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setImportPathList(self);
    // return 1;
  }
}

pub trait QQmlEngine_setImportPathList<RetType> {
  fn setImportPathList(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setImportPathList(const QStringList & paths);
impl<'a> /*trait*/ QQmlEngine_setImportPathList<()> for (&'a QStringList) {
  fn setImportPathList(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine17setImportPathListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine17setImportPathListERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::addPluginPath(const QString & dir);
impl /*struct*/ QQmlEngine {
  pub fn addPluginPath<RetType, T: QQmlEngine_addPluginPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPluginPath(self);
    // return 1;
  }
}

pub trait QQmlEngine_addPluginPath<RetType> {
  fn addPluginPath(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::addPluginPath(const QString & dir);
impl<'a> /*trait*/ QQmlEngine_addPluginPath<()> for (&'a QString) {
  fn addPluginPath(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine13addPluginPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine13addPluginPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::setPluginPathList(const QStringList & paths);
impl /*struct*/ QQmlEngine {
  pub fn setPluginPathList<RetType, T: QQmlEngine_setPluginPathList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPluginPathList(self);
    // return 1;
  }
}

pub trait QQmlEngine_setPluginPathList<RetType> {
  fn setPluginPathList(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setPluginPathList(const QStringList & paths);
impl<'a> /*trait*/ QQmlEngine_setPluginPathList<()> for (&'a QStringList) {
  fn setPluginPathList(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine17setPluginPathListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine17setPluginPathListERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QQmlEngine::setContextForObject(QObject * , QQmlContext * );
impl /*struct*/ QQmlEngine {
  pub fn setContextForObject_s<RetType, T: QQmlEngine_setContextForObject_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setContextForObject_s();
    // return 1;
  }
}

pub trait QQmlEngine_setContextForObject_s<RetType> {
  fn setContextForObject_s(self ) -> RetType;
}

  // proto: static void QQmlEngine::setContextForObject(QObject * , QQmlContext * );
impl<'a> /*trait*/ QQmlEngine_setContextForObject_s<()> for (&'a QObject, &'a QQmlContext) {
  fn setContextForObject_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine19setContextForObjectEP7QObjectP11QQmlContext()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine19setContextForObjectEP7QObjectP11QQmlContext(arg0, arg1)};
    // return 1;
  }
}

  // proto:  QQmlImageProviderBase * QQmlEngine::imageProvider(const QString & id);
impl /*struct*/ QQmlEngine {
  pub fn imageProvider<RetType, T: QQmlEngine_imageProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.imageProvider(self);
    // return 1;
  }
}

pub trait QQmlEngine_imageProvider<RetType> {
  fn imageProvider(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QQmlImageProviderBase * QQmlEngine::imageProvider(const QString & id);
impl<'a> /*trait*/ QQmlEngine_imageProvider<QQmlImageProviderBase> for (&'a QString) {
  fn imageProvider(self , rsthis: & QQmlEngine) -> QQmlImageProviderBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine13imageProviderERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQmlEngine13imageProviderERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QQmlImageProviderBase::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlEngine::QQmlEngine(const QQmlEngine & );
impl /*struct*/ QQmlEngine {
  pub fn new<T: QQmlEngine_new>(value: T) -> QQmlEngine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlEngine_new {
  fn new(self) -> QQmlEngine;
}

  // proto:  void QQmlEngine::QQmlEngine(const QQmlEngine & );
impl<'a> /*trait*/ QQmlEngine_new for (&'a QQmlEngine) {
  fn new(self) -> QQmlEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngineC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QQmlEngineC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlEngine{qbase: QJSEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlEngine::~QQmlEngine();
impl /*struct*/ QQmlEngine {
  pub fn free<RetType, T: QQmlEngine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlEngine_free<RetType> {
  fn free(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::~QQmlEngine();
impl<'a> /*trait*/ QQmlEngine_free<()> for () {
  fn free(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngineD2Ev()};
     unsafe {_ZN10QQmlEngineD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::QQmlEngine(QObject * p);
impl<'a> /*trait*/ QQmlEngine_new for (&'a QObject) {
  fn new(self) -> QQmlEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngineC2EP7QObject()};
    let ctysz: c_int = unsafe{QQmlEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QQmlEngineC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlEngine{qbase: QJSEngine::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QQmlContext * QQmlEngine::rootContext();
impl /*struct*/ QQmlEngine {
  pub fn rootContext<RetType, T: QQmlEngine_rootContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootContext(self);
    // return 1;
  }
}

pub trait QQmlEngine_rootContext<RetType> {
  fn rootContext(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QQmlContext * QQmlEngine::rootContext();
impl<'a> /*trait*/ QQmlEngine_rootContext<QQmlContext> for () {
  fn rootContext(self , rsthis: & QQmlEngine) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine11rootContextEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine11rootContextEv(rsthis.qclsinst)};
    let mut ret1 = QQmlContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlEngine::addImportPath(const QString & dir);
impl /*struct*/ QQmlEngine {
  pub fn addImportPath<RetType, T: QQmlEngine_addImportPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addImportPath(self);
    // return 1;
  }
}

pub trait QQmlEngine_addImportPath<RetType> {
  fn addImportPath(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::addImportPath(const QString & dir);
impl<'a> /*trait*/ QQmlEngine_addImportPath<()> for (&'a QString) {
  fn addImportPath(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine13addImportPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine13addImportPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::setIncubationController(QQmlIncubationController * );
impl /*struct*/ QQmlEngine {
  pub fn setIncubationController<RetType, T: QQmlEngine_setIncubationController<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIncubationController(self);
    // return 1;
  }
}

pub trait QQmlEngine_setIncubationController<RetType> {
  fn setIncubationController(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setIncubationController(QQmlIncubationController * );
impl<'a> /*trait*/ QQmlEngine_setIncubationController<()> for (&'a QQmlIncubationController) {
  fn setIncubationController(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine23setIncubationControllerEP24QQmlIncubationController()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine23setIncubationControllerEP24QQmlIncubationController(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QQmlEngine::importPathList();
impl /*struct*/ QQmlEngine {
  pub fn importPathList<RetType, T: QQmlEngine_importPathList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.importPathList(self);
    // return 1;
  }
}

pub trait QQmlEngine_importPathList<RetType> {
  fn importPathList(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QStringList QQmlEngine::importPathList();
impl<'a> /*trait*/ QQmlEngine_importPathList<()> for () {
  fn importPathList(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine14importPathListEv()};
     unsafe {_ZNK10QQmlEngine14importPathListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QQmlAbstractUrlInterceptor * QQmlEngine::urlInterceptor();
impl /*struct*/ QQmlEngine {
  pub fn urlInterceptor<RetType, T: QQmlEngine_urlInterceptor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.urlInterceptor(self);
    // return 1;
  }
}

pub trait QQmlEngine_urlInterceptor<RetType> {
  fn urlInterceptor(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QQmlAbstractUrlInterceptor * QQmlEngine::urlInterceptor();
impl<'a> /*trait*/ QQmlEngine_urlInterceptor<QQmlAbstractUrlInterceptor> for () {
  fn urlInterceptor(self , rsthis: & QQmlEngine) -> QQmlAbstractUrlInterceptor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine14urlInterceptorEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine14urlInterceptorEv(rsthis.qclsinst)};
    let mut ret1 = QQmlAbstractUrlInterceptor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QQmlEngine::offlineStoragePath();
impl /*struct*/ QQmlEngine {
  pub fn offlineStoragePath<RetType, T: QQmlEngine_offlineStoragePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offlineStoragePath(self);
    // return 1;
  }
}

pub trait QQmlEngine_offlineStoragePath<RetType> {
  fn offlineStoragePath(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QString QQmlEngine::offlineStoragePath();
impl<'a> /*trait*/ QQmlEngine_offlineStoragePath<QString> for () {
  fn offlineStoragePath(self , rsthis: & QQmlEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine18offlineStoragePathEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine18offlineStoragePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlEngine::removeImageProvider(const QString & id);
impl /*struct*/ QQmlEngine {
  pub fn removeImageProvider<RetType, T: QQmlEngine_removeImageProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeImageProvider(self);
    // return 1;
  }
}

pub trait QQmlEngine_removeImageProvider<RetType> {
  fn removeImageProvider(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::removeImageProvider(const QString & id);
impl<'a> /*trait*/ QQmlEngine_removeImageProvider<()> for (&'a QString) {
  fn removeImageProvider(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine19removeImageProviderERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine19removeImageProviderERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkAccessManager * QQmlEngine::networkAccessManager();
impl /*struct*/ QQmlEngine {
  pub fn networkAccessManager<RetType, T: QQmlEngine_networkAccessManager<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.networkAccessManager(self);
    // return 1;
  }
}

pub trait QQmlEngine_networkAccessManager<RetType> {
  fn networkAccessManager(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QNetworkAccessManager * QQmlEngine::networkAccessManager();
impl<'a> /*trait*/ QQmlEngine_networkAccessManager<QNetworkAccessManager> for () {
  fn networkAccessManager(self , rsthis: & QQmlEngine) -> QNetworkAccessManager {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine20networkAccessManagerEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine20networkAccessManagerEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkAccessManager::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlEngine::setBaseUrl(const QUrl & );
impl /*struct*/ QQmlEngine {
  pub fn setBaseUrl<RetType, T: QQmlEngine_setBaseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaseUrl(self);
    // return 1;
  }
}

pub trait QQmlEngine_setBaseUrl<RetType> {
  fn setBaseUrl(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setBaseUrl(const QUrl & );
impl<'a> /*trait*/ QQmlEngine_setBaseUrl<()> for (&'a QUrl) {
  fn setBaseUrl(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine10setBaseUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQmlEngine10setBaseUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QQmlEngine::baseUrl();
impl /*struct*/ QQmlEngine {
  pub fn baseUrl<RetType, T: QQmlEngine_baseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseUrl(self);
    // return 1;
  }
}

pub trait QQmlEngine_baseUrl<RetType> {
  fn baseUrl(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QUrl QQmlEngine::baseUrl();
impl<'a> /*trait*/ QQmlEngine_baseUrl<QUrl> for () {
  fn baseUrl(self , rsthis: & QQmlEngine) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine7baseUrlEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine7baseUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQmlIncubationController * QQmlEngine::incubationController();
impl /*struct*/ QQmlEngine {
  pub fn incubationController<RetType, T: QQmlEngine_incubationController<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.incubationController(self);
    // return 1;
  }
}

pub trait QQmlEngine_incubationController<RetType> {
  fn incubationController(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QQmlIncubationController * QQmlEngine::incubationController();
impl<'a> /*trait*/ QQmlEngine_incubationController<QQmlIncubationController> for () {
  fn incubationController(self , rsthis: & QQmlEngine) -> QQmlIncubationController {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine20incubationControllerEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine20incubationControllerEv(rsthis.qclsinst)};
    let mut ret1 = QQmlIncubationController::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QQmlContext * QQmlEngine::contextForObject(const QObject * );
impl /*struct*/ QQmlEngine {
  pub fn contextForObject_s<RetType, T: QQmlEngine_contextForObject_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.contextForObject_s();
    // return 1;
  }
}

pub trait QQmlEngine_contextForObject_s<RetType> {
  fn contextForObject_s(self ) -> RetType;
}

  // proto: static QQmlContext * QQmlEngine::contextForObject(const QObject * );
impl<'a> /*trait*/ QQmlEngine_contextForObject_s<QQmlContext> for (&'a QObject) {
  fn contextForObject_s(self ) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine16contextForObjectEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QQmlEngine16contextForObjectEPK7QObject(arg0)};
    let mut ret1 = QQmlContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQmlNetworkAccessManagerFactory * QQmlEngine::networkAccessManagerFactory();
impl /*struct*/ QQmlEngine {
  pub fn networkAccessManagerFactory<RetType, T: QQmlEngine_networkAccessManagerFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.networkAccessManagerFactory(self);
    // return 1;
  }
}

pub trait QQmlEngine_networkAccessManagerFactory<RetType> {
  fn networkAccessManagerFactory(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QQmlNetworkAccessManagerFactory * QQmlEngine::networkAccessManagerFactory();
impl<'a> /*trait*/ QQmlEngine_networkAccessManagerFactory<QQmlNetworkAccessManagerFactory> for () {
  fn networkAccessManagerFactory(self , rsthis: & QQmlEngine) -> QQmlNetworkAccessManagerFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine27networkAccessManagerFactoryEv()};
    let mut ret = unsafe {_ZNK10QQmlEngine27networkAccessManagerFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QQmlNetworkAccessManagerFactory::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QQmlEngine::pluginPathList();
impl /*struct*/ QQmlEngine {
  pub fn pluginPathList<RetType, T: QQmlEngine_pluginPathList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pluginPathList(self);
    // return 1;
  }
}

pub trait QQmlEngine_pluginPathList<RetType> {
  fn pluginPathList(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  QStringList QQmlEngine::pluginPathList();
impl<'a> /*trait*/ QQmlEngine_pluginPathList<()> for () {
  fn pluginPathList(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine14pluginPathListEv()};
     unsafe {_ZNK10QQmlEngine14pluginPathListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlEngine::metaObject();
impl /*struct*/ QQmlEngine {
  pub fn metaObject<RetType, T: QQmlEngine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlEngine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  const QMetaObject * QQmlEngine::metaObject();
impl<'a> /*trait*/ QQmlEngine_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQmlEngine10metaObjectEv()};
     unsafe {_ZNK10QQmlEngine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::setOutputWarningsToStandardError(bool );
impl /*struct*/ QQmlEngine {
  pub fn setOutputWarningsToStandardError<RetType, T: QQmlEngine_setOutputWarningsToStandardError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOutputWarningsToStandardError(self);
    // return 1;
  }
}

pub trait QQmlEngine_setOutputWarningsToStandardError<RetType> {
  fn setOutputWarningsToStandardError(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::setOutputWarningsToStandardError(bool );
impl<'a> /*trait*/ QQmlEngine_setOutputWarningsToStandardError<()> for (i8) {
  fn setOutputWarningsToStandardError(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine32setOutputWarningsToStandardErrorEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQmlEngine32setOutputWarningsToStandardErrorEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlEngine::clearComponentCache();
impl /*struct*/ QQmlEngine {
  pub fn clearComponentCache<RetType, T: QQmlEngine_clearComponentCache<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearComponentCache(self);
    // return 1;
  }
}

pub trait QQmlEngine_clearComponentCache<RetType> {
  fn clearComponentCache(self , rsthis: & QQmlEngine) -> RetType;
}

  // proto:  void QQmlEngine::clearComponentCache();
impl<'a> /*trait*/ QQmlEngine_clearComponentCache<()> for () {
  fn clearComponentCache(self , rsthis: & QQmlEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQmlEngine19clearComponentCacheEv()};
     unsafe {_ZN10QQmlEngine19clearComponentCacheEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QQmlImageProviderBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlImageProviderBase {
    return QQmlImageProviderBase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlImageProviderBase::QQmlImageProviderBase();
impl /*struct*/ QQmlImageProviderBase {
  pub fn new<T: QQmlImageProviderBase_new>(value: T) -> QQmlImageProviderBase {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlImageProviderBase_new {
  fn new(self) -> QQmlImageProviderBase;
}

  // proto:  void QQmlImageProviderBase::QQmlImageProviderBase();
impl<'a> /*trait*/ QQmlImageProviderBase_new for () {
  fn new(self) -> QQmlImageProviderBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlImageProviderBaseC2Ev()};
    let ctysz: c_int = unsafe{QQmlImageProviderBase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN21QQmlImageProviderBaseC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlImageProviderBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlImageProviderBase::~QQmlImageProviderBase();
impl /*struct*/ QQmlImageProviderBase {
  pub fn free<RetType, T: QQmlImageProviderBase_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlImageProviderBase_free<RetType> {
  fn free(self , rsthis: & QQmlImageProviderBase) -> RetType;
}

  // proto:  void QQmlImageProviderBase::~QQmlImageProviderBase();
impl<'a> /*trait*/ QQmlImageProviderBase_free<()> for () {
  fn free(self , rsthis: & QQmlImageProviderBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QQmlImageProviderBaseD2Ev()};
     unsafe {_ZN21QQmlImageProviderBaseD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QQmlEngine_quit
pub struct QQmlEngine_quit_signal{poi:u64}
impl /* struct */ QQmlEngine {
  pub fn quit(&self) -> QQmlEngine_quit_signal {
     return QQmlEngine_quit_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlEngine_quit_signal {
  pub fn connect<T: QQmlEngine_quit_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlEngine_quit_signal_connect {
  fn connect(self, sigthis: QQmlEngine_quit_signal);
}

#[derive(Default)] // for QQmlEngine_warnings
pub struct QQmlEngine_warnings_signal{poi:u64}
impl /* struct */ QQmlEngine {
  pub fn warnings(&self) -> QQmlEngine_warnings_signal {
     return QQmlEngine_warnings_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlEngine_warnings_signal {
  pub fn connect<T: QQmlEngine_warnings_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlEngine_warnings_signal_connect {
  fn connect(self, sigthis: QQmlEngine_warnings_signal);
}

// quit()
extern fn QQmlEngine_quit_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQmlEngine_quit_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQmlEngine_quit_signal_connect for fn() {
  fn connect(self, sigthis: QQmlEngine_quit_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlEngine_quit_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQmlEngine_SlotProxy_connect__ZN10QQmlEngine4quitEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQmlEngine_quit_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQmlEngine_quit_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlEngine_quit_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQmlEngine_SlotProxy_connect__ZN10QQmlEngine4quitEv(arg0, arg1, arg2)};
  }
}
// <= body block end

