// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qnetworkaccessmanager.h
// dst-file: /src/network/qnetworkaccessmanager.rs
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
use super::qnetworkcookiejar::QNetworkCookieJar; // 773
use super::qnetworkrequest::QNetworkRequest; // 773
use super::qhttpmultipart::QHttpMultiPart; // 773
use super::qnetworkreply::QNetworkReply; // 773
use super::super::core::qiodevice::QIODevice; // 771
use super::qnetworkconfiguration::QNetworkConfiguration; // 773
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qstring::QString; // 771
use super::qsslconfiguration::QSslConfiguration; // 773
use super::qnetworkproxy::QNetworkProxyFactory; // 773
use super::qabstractnetworkcache::QAbstractNetworkCache; // 773
use super::qnetworkproxy::QNetworkProxy; // 773
use super::qauthenticator::QAuthenticator; // 773
use super::qsslpresharedkeyauthenticator::QSslPreSharedKeyAuthenticator; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkAccessManager_Class_Size() -> c_int;
  // proto:  void QNetworkAccessManager::setCookieJar(QNetworkCookieJar * cookieJar);
  fn _ZN21QNetworkAccessManager12setCookieJarEP17QNetworkCookieJar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, QHttpMultiPart * multiPart);
  fn _ZN21QNetworkAccessManager3putERK15QNetworkRequestP14QHttpMultiPart(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QNetworkCookieJar * QNetworkAccessManager::cookieJar();
  fn _ZNK21QNetworkAccessManager9cookieJarEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QNetworkAccessManager::metaObject();
  fn _ZNK21QNetworkAccessManager10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QNetworkAccessManager::supportedSchemes();
  fn _ZNK21QNetworkAccessManager16supportedSchemesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractNetworkCache * QNetworkAccessManager::cache();
  fn _ZNK21QNetworkAccessManager5cacheEv(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, QIODevice * data);
  fn _ZN21QNetworkAccessManager3putERK15QNetworkRequestP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkAccessManager::QNetworkAccessManager(QObject * parent);
  fn _ZN21QNetworkAccessManagerC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkReply * QNetworkAccessManager::get(const QNetworkRequest & request);
  fn _ZN21QNetworkAccessManager3getERK15QNetworkRequest(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QNetworkReply * QNetworkAccessManager::deleteResource(const QNetworkRequest & request);
  fn _ZN21QNetworkAccessManager14deleteResourceERK15QNetworkRequest(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QNetworkReply * QNetworkAccessManager::head(const QNetworkRequest & request);
  fn _ZN21QNetworkAccessManager4headERK15QNetworkRequest(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QNetworkConfiguration QNetworkAccessManager::configuration();
  fn _ZNK21QNetworkAccessManager13configurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkAccessManager::setConfiguration(const QNetworkConfiguration & config);
  fn _ZN21QNetworkAccessManager16setConfigurationERK21QNetworkConfiguration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, QIODevice * data);
  fn _ZN21QNetworkAccessManager4postERK15QNetworkRequestP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, const QByteArray & data);
  fn _ZN21QNetworkAccessManager4postERK15QNetworkRequestRK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QNetworkReply * QNetworkAccessManager::sendCustomRequest(const QNetworkRequest & request, const QByteArray & verb, QIODevice * data);
  fn _ZN21QNetworkAccessManager17sendCustomRequestERK15QNetworkRequestRK10QByteArrayP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkAccessManager::connectToHostEncrypted(const QString & hostName, quint16 port, const QSslConfiguration & sslConfiguration);
  fn _ZN21QNetworkAccessManager22connectToHostEncryptedERK7QStringtRK17QSslConfiguration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_ushort, arg2: *mut c_void);
  // proto:  void QNetworkAccessManager::clearAccessCache();
  fn _ZN21QNetworkAccessManager16clearAccessCacheEv(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkConfiguration QNetworkAccessManager::activeConfiguration();
  fn _ZNK21QNetworkAccessManager19activeConfigurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkAccessManager::setProxyFactory(QNetworkProxyFactory * factory);
  fn _ZN21QNetworkAccessManager15setProxyFactoryEP20QNetworkProxyFactory(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkAccessManager::~QNetworkAccessManager();
  fn _ZN21QNetworkAccessManagerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, const QByteArray & data);
  fn _ZN21QNetworkAccessManager3putERK15QNetworkRequestRK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkAccessManager::setCache(QAbstractNetworkCache * cache);
  fn _ZN21QNetworkAccessManager8setCacheEP21QAbstractNetworkCache(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkAccessManager::connectToHost(const QString & hostName, quint16 port);
  fn _ZN21QNetworkAccessManager13connectToHostERK7QStringt(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_ushort);
  // proto:  void QNetworkAccessManager::setProxy(const QNetworkProxy & proxy);
  fn _ZN21QNetworkAccessManager8setProxyERK13QNetworkProxy(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkProxy QNetworkAccessManager::proxy();
  fn _ZNK21QNetworkAccessManager5proxyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, QHttpMultiPart * multiPart);
  fn _ZN21QNetworkAccessManager4postERK15QNetworkRequestP14QHttpMultiPart(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QNetworkProxyFactory * QNetworkAccessManager::proxyFactory();
  fn _ZNK21QNetworkAccessManager12proxyFactoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager23networkSessionConnectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager27proxyAuthenticationRequiredERK13QNetworkProxyP14QAuthenticator(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager34preSharedKeyAuthenticationRequiredEP13QNetworkReplyP29QSslPreSharedKeyAuthenticator(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager8finishedEP13QNetworkReply(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager9encryptedEP13QNetworkReply(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager22authenticationRequiredEP13QNetworkReplyP14QAuthenticator(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkAccessManager)=1
#[derive(Default)]
pub struct QNetworkAccessManager {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _encrypted: QNetworkAccessManager_encrypted_signal,
  pub _sslErrors: QNetworkAccessManager_sslErrors_signal,
  pub _networkAccessibleChanged: QNetworkAccessManager_networkAccessibleChanged_signal,
  pub _preSharedKeyAuthenticationRequired: QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal,
  pub _finished: QNetworkAccessManager_finished_signal,
  pub _proxyAuthenticationRequired: QNetworkAccessManager_proxyAuthenticationRequired_signal,
  pub _authenticationRequired: QNetworkAccessManager_authenticationRequired_signal,
  pub _networkSessionConnected: QNetworkAccessManager_networkSessionConnected_signal,
}

impl /*struct*/ QNetworkAccessManager {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkAccessManager {
    return QNetworkAccessManager{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNetworkAccessManager {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QNetworkAccessManager {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QNetworkAccessManager::setCookieJar(QNetworkCookieJar * cookieJar);
impl /*struct*/ QNetworkAccessManager {
  pub fn setCookieJar<RetType, T: QNetworkAccessManager_setCookieJar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCookieJar(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_setCookieJar<RetType> {
  fn setCookieJar(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::setCookieJar(QNetworkCookieJar * cookieJar);
impl<'a> /*trait*/ QNetworkAccessManager_setCookieJar<()> for (&'a QNetworkCookieJar) {
  fn setCookieJar(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager12setCookieJarEP17QNetworkCookieJar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkAccessManager12setCookieJarEP17QNetworkCookieJar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, QHttpMultiPart * multiPart);
impl /*struct*/ QNetworkAccessManager {
  pub fn put<RetType, T: QNetworkAccessManager_put<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.put(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_put<RetType> {
  fn put(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, QHttpMultiPart * multiPart);
impl<'a> /*trait*/ QNetworkAccessManager_put<QNetworkReply> for (&'a QNetworkRequest, &'a QHttpMultiPart) {
  fn put(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager3putERK15QNetworkRequestP14QHttpMultiPart()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager3putERK15QNetworkRequestP14QHttpMultiPart(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkCookieJar * QNetworkAccessManager::cookieJar();
impl /*struct*/ QNetworkAccessManager {
  pub fn cookieJar<RetType, T: QNetworkAccessManager_cookieJar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cookieJar(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_cookieJar<RetType> {
  fn cookieJar(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkCookieJar * QNetworkAccessManager::cookieJar();
impl<'a> /*trait*/ QNetworkAccessManager_cookieJar<QNetworkCookieJar> for () {
  fn cookieJar(self , rsthis: & QNetworkAccessManager) -> QNetworkCookieJar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager9cookieJarEv()};
    let mut ret = unsafe {_ZNK21QNetworkAccessManager9cookieJarEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkCookieJar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QNetworkAccessManager::metaObject();
impl /*struct*/ QNetworkAccessManager {
  pub fn metaObject<RetType, T: QNetworkAccessManager_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_metaObject<RetType> {
  fn metaObject(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  const QMetaObject * QNetworkAccessManager::metaObject();
impl<'a> /*trait*/ QNetworkAccessManager_metaObject<()> for () {
  fn metaObject(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager10metaObjectEv()};
     unsafe {_ZNK21QNetworkAccessManager10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QNetworkAccessManager::supportedSchemes();
impl /*struct*/ QNetworkAccessManager {
  pub fn supportedSchemes<RetType, T: QNetworkAccessManager_supportedSchemes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportedSchemes(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_supportedSchemes<RetType> {
  fn supportedSchemes(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QStringList QNetworkAccessManager::supportedSchemes();
impl<'a> /*trait*/ QNetworkAccessManager_supportedSchemes<()> for () {
  fn supportedSchemes(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager16supportedSchemesEv()};
     unsafe {_ZNK21QNetworkAccessManager16supportedSchemesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractNetworkCache * QNetworkAccessManager::cache();
impl /*struct*/ QNetworkAccessManager {
  pub fn cache<RetType, T: QNetworkAccessManager_cache<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cache(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_cache<RetType> {
  fn cache(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QAbstractNetworkCache * QNetworkAccessManager::cache();
impl<'a> /*trait*/ QNetworkAccessManager_cache<()> for () {
  fn cache(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager5cacheEv()};
     unsafe {_ZNK21QNetworkAccessManager5cacheEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, QIODevice * data);
impl<'a> /*trait*/ QNetworkAccessManager_put<QNetworkReply> for (&'a QNetworkRequest, &'a QIODevice) {
  fn put(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager3putERK15QNetworkRequestP9QIODevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager3putERK15QNetworkRequestP9QIODevice(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::QNetworkAccessManager(QObject * parent);
impl /*struct*/ QNetworkAccessManager {
  pub fn new<T: QNetworkAccessManager_new>(value: T) -> QNetworkAccessManager {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkAccessManager_new {
  fn new(self) -> QNetworkAccessManager;
}

  // proto:  void QNetworkAccessManager::QNetworkAccessManager(QObject * parent);
impl<'a> /*trait*/ QNetworkAccessManager_new for (&'a QObject) {
  fn new(self) -> QNetworkAccessManager {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManagerC2EP7QObject()};
    let ctysz: c_int = unsafe{QNetworkAccessManager_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QNetworkAccessManagerC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkAccessManager{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::get(const QNetworkRequest & request);
impl /*struct*/ QNetworkAccessManager {
  pub fn get<RetType, T: QNetworkAccessManager_get<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.get(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_get<RetType> {
  fn get(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkReply * QNetworkAccessManager::get(const QNetworkRequest & request);
impl<'a> /*trait*/ QNetworkAccessManager_get<QNetworkReply> for (&'a QNetworkRequest) {
  fn get(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager3getERK15QNetworkRequest()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager3getERK15QNetworkRequest(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::deleteResource(const QNetworkRequest & request);
impl /*struct*/ QNetworkAccessManager {
  pub fn deleteResource<RetType, T: QNetworkAccessManager_deleteResource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deleteResource(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_deleteResource<RetType> {
  fn deleteResource(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkReply * QNetworkAccessManager::deleteResource(const QNetworkRequest & request);
impl<'a> /*trait*/ QNetworkAccessManager_deleteResource<QNetworkReply> for (&'a QNetworkRequest) {
  fn deleteResource(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager14deleteResourceERK15QNetworkRequest()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager14deleteResourceERK15QNetworkRequest(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::head(const QNetworkRequest & request);
impl /*struct*/ QNetworkAccessManager {
  pub fn head<RetType, T: QNetworkAccessManager_head<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.head(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_head<RetType> {
  fn head(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkReply * QNetworkAccessManager::head(const QNetworkRequest & request);
impl<'a> /*trait*/ QNetworkAccessManager_head<QNetworkReply> for (&'a QNetworkRequest) {
  fn head(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager4headERK15QNetworkRequest()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager4headERK15QNetworkRequest(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkConfiguration QNetworkAccessManager::configuration();
impl /*struct*/ QNetworkAccessManager {
  pub fn configuration<RetType, T: QNetworkAccessManager_configuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.configuration(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_configuration<RetType> {
  fn configuration(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkConfiguration QNetworkAccessManager::configuration();
impl<'a> /*trait*/ QNetworkAccessManager_configuration<QNetworkConfiguration> for () {
  fn configuration(self , rsthis: & QNetworkAccessManager) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager13configurationEv()};
    let mut ret = unsafe {_ZNK21QNetworkAccessManager13configurationEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::setConfiguration(const QNetworkConfiguration & config);
impl /*struct*/ QNetworkAccessManager {
  pub fn setConfiguration<RetType, T: QNetworkAccessManager_setConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_setConfiguration<RetType> {
  fn setConfiguration(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::setConfiguration(const QNetworkConfiguration & config);
impl<'a> /*trait*/ QNetworkAccessManager_setConfiguration<()> for (&'a QNetworkConfiguration) {
  fn setConfiguration(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager16setConfigurationERK21QNetworkConfiguration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkAccessManager16setConfigurationERK21QNetworkConfiguration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, QIODevice * data);
impl /*struct*/ QNetworkAccessManager {
  pub fn post<RetType, T: QNetworkAccessManager_post<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.post(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_post<RetType> {
  fn post(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, QIODevice * data);
impl<'a> /*trait*/ QNetworkAccessManager_post<QNetworkReply> for (&'a QNetworkRequest, &'a QIODevice) {
  fn post(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager4postERK15QNetworkRequestP9QIODevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager4postERK15QNetworkRequestP9QIODevice(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, const QByteArray & data);
impl<'a> /*trait*/ QNetworkAccessManager_post<QNetworkReply> for (&'a QNetworkRequest, &'a QByteArray) {
  fn post(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager4postERK15QNetworkRequestRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager4postERK15QNetworkRequestRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::sendCustomRequest(const QNetworkRequest & request, const QByteArray & verb, QIODevice * data);
impl /*struct*/ QNetworkAccessManager {
  pub fn sendCustomRequest<RetType, T: QNetworkAccessManager_sendCustomRequest<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sendCustomRequest(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_sendCustomRequest<RetType> {
  fn sendCustomRequest(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkReply * QNetworkAccessManager::sendCustomRequest(const QNetworkRequest & request, const QByteArray & verb, QIODevice * data);
impl<'a> /*trait*/ QNetworkAccessManager_sendCustomRequest<QNetworkReply> for (&'a QNetworkRequest, &'a QByteArray, &'a QIODevice) {
  fn sendCustomRequest(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager17sendCustomRequestERK15QNetworkRequestRK10QByteArrayP9QIODevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager17sendCustomRequestERK15QNetworkRequestRK10QByteArrayP9QIODevice(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::connectToHostEncrypted(const QString & hostName, quint16 port, const QSslConfiguration & sslConfiguration);
impl /*struct*/ QNetworkAccessManager {
  pub fn connectToHostEncrypted<RetType, T: QNetworkAccessManager_connectToHostEncrypted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.connectToHostEncrypted(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_connectToHostEncrypted<RetType> {
  fn connectToHostEncrypted(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::connectToHostEncrypted(const QString & hostName, quint16 port, const QSslConfiguration & sslConfiguration);
impl<'a> /*trait*/ QNetworkAccessManager_connectToHostEncrypted<()> for (&'a QString, u16, &'a QSslConfiguration) {
  fn connectToHostEncrypted(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager22connectToHostEncryptedERK7QStringtRK17QSslConfiguration()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ushort;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkAccessManager22connectToHostEncryptedERK7QStringtRK17QSslConfiguration(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::clearAccessCache();
impl /*struct*/ QNetworkAccessManager {
  pub fn clearAccessCache<RetType, T: QNetworkAccessManager_clearAccessCache<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearAccessCache(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_clearAccessCache<RetType> {
  fn clearAccessCache(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::clearAccessCache();
impl<'a> /*trait*/ QNetworkAccessManager_clearAccessCache<()> for () {
  fn clearAccessCache(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager16clearAccessCacheEv()};
     unsafe {_ZN21QNetworkAccessManager16clearAccessCacheEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkConfiguration QNetworkAccessManager::activeConfiguration();
impl /*struct*/ QNetworkAccessManager {
  pub fn activeConfiguration<RetType, T: QNetworkAccessManager_activeConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_activeConfiguration<RetType> {
  fn activeConfiguration(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkConfiguration QNetworkAccessManager::activeConfiguration();
impl<'a> /*trait*/ QNetworkAccessManager_activeConfiguration<QNetworkConfiguration> for () {
  fn activeConfiguration(self , rsthis: & QNetworkAccessManager) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager19activeConfigurationEv()};
    let mut ret = unsafe {_ZNK21QNetworkAccessManager19activeConfigurationEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::setProxyFactory(QNetworkProxyFactory * factory);
impl /*struct*/ QNetworkAccessManager {
  pub fn setProxyFactory<RetType, T: QNetworkAccessManager_setProxyFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProxyFactory(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_setProxyFactory<RetType> {
  fn setProxyFactory(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::setProxyFactory(QNetworkProxyFactory * factory);
impl<'a> /*trait*/ QNetworkAccessManager_setProxyFactory<()> for (&'a QNetworkProxyFactory) {
  fn setProxyFactory(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager15setProxyFactoryEP20QNetworkProxyFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkAccessManager15setProxyFactoryEP20QNetworkProxyFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::~QNetworkAccessManager();
impl /*struct*/ QNetworkAccessManager {
  pub fn free<RetType, T: QNetworkAccessManager_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_free<RetType> {
  fn free(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::~QNetworkAccessManager();
impl<'a> /*trait*/ QNetworkAccessManager_free<()> for () {
  fn free(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManagerD2Ev()};
     unsafe {_ZN21QNetworkAccessManagerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::put(const QNetworkRequest & request, const QByteArray & data);
impl<'a> /*trait*/ QNetworkAccessManager_put<QNetworkReply> for (&'a QNetworkRequest, &'a QByteArray) {
  fn put(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager3putERK15QNetworkRequestRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager3putERK15QNetworkRequestRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::setCache(QAbstractNetworkCache * cache);
impl /*struct*/ QNetworkAccessManager {
  pub fn setCache<RetType, T: QNetworkAccessManager_setCache<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCache(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_setCache<RetType> {
  fn setCache(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::setCache(QAbstractNetworkCache * cache);
impl<'a> /*trait*/ QNetworkAccessManager_setCache<()> for (&'a QAbstractNetworkCache) {
  fn setCache(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager8setCacheEP21QAbstractNetworkCache()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkAccessManager8setCacheEP21QAbstractNetworkCache(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::connectToHost(const QString & hostName, quint16 port);
impl /*struct*/ QNetworkAccessManager {
  pub fn connectToHost<RetType, T: QNetworkAccessManager_connectToHost<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.connectToHost(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_connectToHost<RetType> {
  fn connectToHost(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::connectToHost(const QString & hostName, quint16 port);
impl<'a> /*trait*/ QNetworkAccessManager_connectToHost<()> for (&'a QString, u16) {
  fn connectToHost(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager13connectToHostERK7QStringt()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ushort;
     unsafe {_ZN21QNetworkAccessManager13connectToHostERK7QStringt(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QNetworkAccessManager::setProxy(const QNetworkProxy & proxy);
impl /*struct*/ QNetworkAccessManager {
  pub fn setProxy<RetType, T: QNetworkAccessManager_setProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProxy(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_setProxy<RetType> {
  fn setProxy(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  void QNetworkAccessManager::setProxy(const QNetworkProxy & proxy);
impl<'a> /*trait*/ QNetworkAccessManager_setProxy<()> for (&'a QNetworkProxy) {
  fn setProxy(self , rsthis: & QNetworkAccessManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager8setProxyERK13QNetworkProxy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkAccessManager8setProxyERK13QNetworkProxy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkProxy QNetworkAccessManager::proxy();
impl /*struct*/ QNetworkAccessManager {
  pub fn proxy<RetType, T: QNetworkAccessManager_proxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.proxy(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_proxy<RetType> {
  fn proxy(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkProxy QNetworkAccessManager::proxy();
impl<'a> /*trait*/ QNetworkAccessManager_proxy<QNetworkProxy> for () {
  fn proxy(self , rsthis: & QNetworkAccessManager) -> QNetworkProxy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager5proxyEv()};
    let mut ret = unsafe {_ZNK21QNetworkAccessManager5proxyEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkProxy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkReply * QNetworkAccessManager::post(const QNetworkRequest & request, QHttpMultiPart * multiPart);
impl<'a> /*trait*/ QNetworkAccessManager_post<QNetworkReply> for (&'a QNetworkRequest, &'a QHttpMultiPart) {
  fn post(self , rsthis: & QNetworkAccessManager) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkAccessManager4postERK15QNetworkRequestP14QHttpMultiPart()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QNetworkAccessManager4postERK15QNetworkRequestP14QHttpMultiPart(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QNetworkReply::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkProxyFactory * QNetworkAccessManager::proxyFactory();
impl /*struct*/ QNetworkAccessManager {
  pub fn proxyFactory<RetType, T: QNetworkAccessManager_proxyFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.proxyFactory(self);
    // return 1;
  }
}

pub trait QNetworkAccessManager_proxyFactory<RetType> {
  fn proxyFactory(self , rsthis: & QNetworkAccessManager) -> RetType;
}

  // proto:  QNetworkProxyFactory * QNetworkAccessManager::proxyFactory();
impl<'a> /*trait*/ QNetworkAccessManager_proxyFactory<QNetworkProxyFactory> for () {
  fn proxyFactory(self , rsthis: & QNetworkAccessManager) -> QNetworkProxyFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkAccessManager12proxyFactoryEv()};
    let mut ret = unsafe {_ZNK21QNetworkAccessManager12proxyFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkProxyFactory::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QNetworkAccessManager_encrypted
pub struct QNetworkAccessManager_encrypted_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn encrypted(&self) -> QNetworkAccessManager_encrypted_signal {
     return QNetworkAccessManager_encrypted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_encrypted_signal {
  pub fn connect<T: QNetworkAccessManager_encrypted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_encrypted_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_encrypted_signal);
}

#[derive(Default)] // for QNetworkAccessManager_sslErrors
pub struct QNetworkAccessManager_sslErrors_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn sslErrors(&self) -> QNetworkAccessManager_sslErrors_signal {
     return QNetworkAccessManager_sslErrors_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_sslErrors_signal {
  pub fn connect<T: QNetworkAccessManager_sslErrors_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_sslErrors_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_sslErrors_signal);
}

#[derive(Default)] // for QNetworkAccessManager_networkAccessibleChanged
pub struct QNetworkAccessManager_networkAccessibleChanged_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn networkAccessibleChanged(&self) -> QNetworkAccessManager_networkAccessibleChanged_signal {
     return QNetworkAccessManager_networkAccessibleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_networkAccessibleChanged_signal {
  pub fn connect<T: QNetworkAccessManager_networkAccessibleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_networkAccessibleChanged_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_networkAccessibleChanged_signal);
}

#[derive(Default)] // for QNetworkAccessManager_preSharedKeyAuthenticationRequired
pub struct QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn preSharedKeyAuthenticationRequired(&self) -> QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal {
     return QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal {
  pub fn connect<T: QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal);
}

#[derive(Default)] // for QNetworkAccessManager_finished
pub struct QNetworkAccessManager_finished_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn finished(&self) -> QNetworkAccessManager_finished_signal {
     return QNetworkAccessManager_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_finished_signal {
  pub fn connect<T: QNetworkAccessManager_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_finished_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_finished_signal);
}

#[derive(Default)] // for QNetworkAccessManager_proxyAuthenticationRequired
pub struct QNetworkAccessManager_proxyAuthenticationRequired_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn proxyAuthenticationRequired(&self) -> QNetworkAccessManager_proxyAuthenticationRequired_signal {
     return QNetworkAccessManager_proxyAuthenticationRequired_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_proxyAuthenticationRequired_signal {
  pub fn connect<T: QNetworkAccessManager_proxyAuthenticationRequired_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_proxyAuthenticationRequired_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_proxyAuthenticationRequired_signal);
}

#[derive(Default)] // for QNetworkAccessManager_authenticationRequired
pub struct QNetworkAccessManager_authenticationRequired_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn authenticationRequired(&self) -> QNetworkAccessManager_authenticationRequired_signal {
     return QNetworkAccessManager_authenticationRequired_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_authenticationRequired_signal {
  pub fn connect<T: QNetworkAccessManager_authenticationRequired_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_authenticationRequired_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_authenticationRequired_signal);
}

#[derive(Default)] // for QNetworkAccessManager_networkSessionConnected
pub struct QNetworkAccessManager_networkSessionConnected_signal{poi:u64}
impl /* struct */ QNetworkAccessManager {
  pub fn networkSessionConnected(&self) -> QNetworkAccessManager_networkSessionConnected_signal {
     return QNetworkAccessManager_networkSessionConnected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkAccessManager_networkSessionConnected_signal {
  pub fn connect<T: QNetworkAccessManager_networkSessionConnected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkAccessManager_networkSessionConnected_signal_connect {
  fn connect(self, sigthis: QNetworkAccessManager_networkSessionConnected_signal);
}

// networkSessionConnected()
extern fn QNetworkAccessManager_networkSessionConnected_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkAccessManager_networkSessionConnected_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkAccessManager_networkSessionConnected_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkAccessManager_networkSessionConnected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_networkSessionConnected_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager23networkSessionConnectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkAccessManager_networkSessionConnected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkAccessManager_networkSessionConnected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_networkSessionConnected_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager23networkSessionConnectedEv(arg0, arg1, arg2)};
  }
}
// proxyAuthenticationRequired(const class QNetworkProxy &, class QAuthenticator *)
extern fn QNetworkAccessManager_proxyAuthenticationRequired_signal_connect_cb_1(rsfptr:fn(QNetworkProxy, QAuthenticator), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkProxy::inheritFrom(arg0 as u64);
  let rsarg1 = QAuthenticator::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QNetworkAccessManager_proxyAuthenticationRequired_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QNetworkProxy, QAuthenticator)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkProxy::inheritFrom(arg0 as u64);
  let rsarg1 = QAuthenticator::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QNetworkAccessManager_proxyAuthenticationRequired_signal_connect for fn(QNetworkProxy, QAuthenticator) {
  fn connect(self, sigthis: QNetworkAccessManager_proxyAuthenticationRequired_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_proxyAuthenticationRequired_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager27proxyAuthenticationRequiredERK13QNetworkProxyP14QAuthenticator(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkAccessManager_proxyAuthenticationRequired_signal_connect for Box<Fn(QNetworkProxy, QAuthenticator)> {
  fn connect(self, sigthis: QNetworkAccessManager_proxyAuthenticationRequired_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_proxyAuthenticationRequired_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager27proxyAuthenticationRequiredERK13QNetworkProxyP14QAuthenticator(arg0, arg1, arg2)};
  }
}
// preSharedKeyAuthenticationRequired(class QNetworkReply *, class QSslPreSharedKeyAuthenticator *)
extern fn QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect_cb_2(rsfptr:fn(QNetworkReply, QSslPreSharedKeyAuthenticator), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  let rsarg1 = QSslPreSharedKeyAuthenticator::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QNetworkReply, QSslPreSharedKeyAuthenticator)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  let rsarg1 = QSslPreSharedKeyAuthenticator::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect for fn(QNetworkReply, QSslPreSharedKeyAuthenticator) {
  fn connect(self, sigthis: QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager34preSharedKeyAuthenticationRequiredEP13QNetworkReplyP29QSslPreSharedKeyAuthenticator(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect for Box<Fn(QNetworkReply, QSslPreSharedKeyAuthenticator)> {
  fn connect(self, sigthis: QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_preSharedKeyAuthenticationRequired_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager34preSharedKeyAuthenticationRequiredEP13QNetworkReplyP29QSslPreSharedKeyAuthenticator(arg0, arg1, arg2)};
  }
}
// finished(class QNetworkReply *)
extern fn QNetworkAccessManager_finished_signal_connect_cb_3(rsfptr:fn(QNetworkReply), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QNetworkAccessManager_finished_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QNetworkReply)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkAccessManager_finished_signal_connect for fn(QNetworkReply) {
  fn connect(self, sigthis: QNetworkAccessManager_finished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_finished_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager8finishedEP13QNetworkReply(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkAccessManager_finished_signal_connect for Box<Fn(QNetworkReply)> {
  fn connect(self, sigthis: QNetworkAccessManager_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_finished_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager8finishedEP13QNetworkReply(arg0, arg1, arg2)};
  }
}
// encrypted(class QNetworkReply *)
extern fn QNetworkAccessManager_encrypted_signal_connect_cb_4(rsfptr:fn(QNetworkReply), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QNetworkAccessManager_encrypted_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QNetworkReply)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkAccessManager_encrypted_signal_connect for fn(QNetworkReply) {
  fn connect(self, sigthis: QNetworkAccessManager_encrypted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_encrypted_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager9encryptedEP13QNetworkReply(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkAccessManager_encrypted_signal_connect for Box<Fn(QNetworkReply)> {
  fn connect(self, sigthis: QNetworkAccessManager_encrypted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_encrypted_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager9encryptedEP13QNetworkReply(arg0, arg1, arg2)};
  }
}
// authenticationRequired(class QNetworkReply *, class QAuthenticator *)
extern fn QNetworkAccessManager_authenticationRequired_signal_connect_cb_5(rsfptr:fn(QNetworkReply, QAuthenticator), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  let rsarg1 = QAuthenticator::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QNetworkAccessManager_authenticationRequired_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QNetworkReply, QAuthenticator)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkReply::inheritFrom(arg0 as u64);
  let rsarg1 = QAuthenticator::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QNetworkAccessManager_authenticationRequired_signal_connect for fn(QNetworkReply, QAuthenticator) {
  fn connect(self, sigthis: QNetworkAccessManager_authenticationRequired_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_authenticationRequired_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager22authenticationRequiredEP13QNetworkReplyP14QAuthenticator(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkAccessManager_authenticationRequired_signal_connect for Box<Fn(QNetworkReply, QAuthenticator)> {
  fn connect(self, sigthis: QNetworkAccessManager_authenticationRequired_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkAccessManager_authenticationRequired_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkAccessManager_SlotProxy_connect__ZN21QNetworkAccessManager22authenticationRequiredEP13QNetworkReplyP14QAuthenticator(arg0, arg1, arg2)};
  }
}
// <= body block end

