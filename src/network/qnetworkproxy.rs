// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworkproxy.h
// dst-file: /src/network/qnetworkproxy.rs
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
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qvariant::QVariant; // 771
// use super::qnetworkproxy::QNetworkProxyQuery; // 773
use super::super::core::qurl::QUrl; // 771
use super::qnetworkconfiguration::QNetworkConfiguration; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkProxy_Class_Size() -> c_int;
  // proto: static QNetworkProxy QNetworkProxy::applicationProxy();
  fn _ZN13QNetworkProxy16applicationProxyEv() -> *mut c_void;
  // proto:  void QNetworkProxy::~QNetworkProxy();
  fn _ZN13QNetworkProxyD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QNetworkProxy::hostName();
  fn _ZNK13QNetworkProxy8hostNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkProxy::QNetworkProxy();
  fn _ZN13QNetworkProxyC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QByteArray> QNetworkProxy::rawHeaderList();
  fn _ZNK13QNetworkProxy13rawHeaderListEv(qthis: u64 /* *mut c_void*/);
  // proto: static void QNetworkProxy::setApplicationProxy(const QNetworkProxy & proxy);
  fn _ZN13QNetworkProxy19setApplicationProxyERKS_(arg0: *mut c_void);
  // proto:  void QNetworkProxy::setHostName(const QString & hostName);
  fn _ZN13QNetworkProxy11setHostNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QNetworkProxy::user();
  fn _ZNK13QNetworkProxy4userEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QNetworkProxy::password();
  fn _ZNK13QNetworkProxy8passwordEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QNetworkProxy::rawHeader(const QByteArray & headerName);
  fn _ZNK13QNetworkProxy9rawHeaderERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QNetworkProxy::isTransparentProxy();
  fn _ZNK13QNetworkProxy18isTransparentProxyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QNetworkProxy::setPassword(const QString & password);
  fn _ZN13QNetworkProxy11setPasswordERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxy::QNetworkProxy(const QNetworkProxy & other);
  fn _ZN13QNetworkProxyC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QNetworkProxy::hasRawHeader(const QByteArray & headerName);
  fn _ZNK13QNetworkProxy12hasRawHeaderERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QNetworkProxy::swap(QNetworkProxy & other);
  fn _ZN13QNetworkProxy4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  quint16 QNetworkProxy::port();
  fn _ZNK13QNetworkProxy4portEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  void QNetworkProxy::setUser(const QString & userName);
  fn _ZN13QNetworkProxy7setUserERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxy::setPort(quint16 port);
  fn _ZN13QNetworkProxy7setPortEt(qthis: u64 /* *mut c_void*/, arg0: c_ushort);
  // proto:  void QNetworkProxy::setRawHeader(const QByteArray & headerName, const QByteArray & value);
  fn _ZN13QNetworkProxy12setRawHeaderERK10QByteArrayS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QNetworkProxy::isCachingProxy();
  fn _ZNK13QNetworkProxy14isCachingProxyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QNetworkProxyFactory_Class_Size() -> c_int;
  // proto: static void QNetworkProxyFactory::setUseSystemConfiguration(bool enable);
  fn _ZN20QNetworkProxyFactory25setUseSystemConfigurationEb(arg0: c_char);
  // proto:  void QNetworkProxyFactory::QNetworkProxyFactory();
  fn _ZN20QNetworkProxyFactoryC2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static void QNetworkProxyFactory::setApplicationProxyFactory(QNetworkProxyFactory * factory);
  fn _ZN20QNetworkProxyFactory26setApplicationProxyFactoryEPS_(arg0: *mut c_void);
  // proto: static QList<QNetworkProxy> QNetworkProxyFactory::systemProxyForQuery(const QNetworkProxyQuery & query);
  fn _ZN20QNetworkProxyFactory19systemProxyForQueryERK18QNetworkProxyQuery(arg0: *mut c_void);
  // proto:  QList<QNetworkProxy> QNetworkProxyFactory::queryProxy(const QNetworkProxyQuery & query);
  fn _ZN20QNetworkProxyFactory10queryProxyERK18QNetworkProxyQuery(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxyFactory::~QNetworkProxyFactory();
  fn _ZN20QNetworkProxyFactoryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static QList<QNetworkProxy> QNetworkProxyFactory::proxyForQuery(const QNetworkProxyQuery & query);
  fn _ZN20QNetworkProxyFactory13proxyForQueryERK18QNetworkProxyQuery(arg0: *mut c_void);
  fn QNetworkProxyQuery_Class_Size() -> c_int;
  // proto:  QUrl QNetworkProxyQuery::url();
  fn _ZNK18QNetworkProxyQuery3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QNetworkProxyQuery::localPort();
  fn _ZNK18QNetworkProxyQuery9localPortEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QNetworkProxyQuery::setNetworkConfiguration(const QNetworkConfiguration & networkConfiguration);
  fn _ZN18QNetworkProxyQuery23setNetworkConfigurationERK21QNetworkConfiguration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxyQuery::setProtocolTag(const QString & protocolTag);
  fn _ZN18QNetworkProxyQuery14setProtocolTagERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxyQuery::setUrl(const QUrl & url);
  fn _ZN18QNetworkProxyQuery6setUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QNetworkProxyQuery::peerPort();
  fn _ZNK18QNetworkProxyQuery8peerPortEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QNetworkProxyQuery::setLocalPort(int port);
  fn _ZN18QNetworkProxyQuery12setLocalPortEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QNetworkProxyQuery::~QNetworkProxyQuery();
  fn _ZN18QNetworkProxyQueryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkProxyQuery::QNetworkProxyQuery(const QNetworkProxyQuery & other);
  fn _ZN18QNetworkProxyQueryC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxyQuery::setPeerHostName(const QString & hostname);
  fn _ZN18QNetworkProxyQuery15setPeerHostNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxyQuery::swap(QNetworkProxyQuery & other);
  fn _ZN18QNetworkProxyQuery4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkProxyQuery::setPeerPort(int port);
  fn _ZN18QNetworkProxyQuery11setPeerPortEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QNetworkConfiguration QNetworkProxyQuery::networkConfiguration();
  fn _ZNK18QNetworkProxyQuery20networkConfigurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QNetworkProxyQuery::peerHostName();
  fn _ZNK18QNetworkProxyQuery12peerHostNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QNetworkProxyQuery::protocolTag();
  fn _ZNK18QNetworkProxyQuery11protocolTagEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkProxyQuery::QNetworkProxyQuery();
  fn _ZN18QNetworkProxyQueryC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkProxy)=1
#[derive(Default)]
pub struct QNetworkProxy {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QNetworkProxyFactory)=8
#[derive(Default)]
pub struct QNetworkProxyFactory {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QNetworkProxyQuery)=1
#[derive(Default)]
pub struct QNetworkProxyQuery {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkProxy {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkProxy {
    return QNetworkProxy{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QNetworkProxy QNetworkProxy::applicationProxy();
impl /*struct*/ QNetworkProxy {
  pub fn applicationProxy_s<RetType, T: QNetworkProxy_applicationProxy_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationProxy_s();
    // return 1;
  }
}

pub trait QNetworkProxy_applicationProxy_s<RetType> {
  fn applicationProxy_s(self ) -> RetType;
}

  // proto: static QNetworkProxy QNetworkProxy::applicationProxy();
impl<'a> /*trait*/ QNetworkProxy_applicationProxy_s<QNetworkProxy> for () {
  fn applicationProxy_s(self ) -> QNetworkProxy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy16applicationProxyEv()};
    let mut ret = unsafe {_ZN13QNetworkProxy16applicationProxyEv()};
    let mut ret1 = QNetworkProxy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkProxy::~QNetworkProxy();
impl /*struct*/ QNetworkProxy {
  pub fn free<RetType, T: QNetworkProxy_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkProxy_free<RetType> {
  fn free(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::~QNetworkProxy();
impl<'a> /*trait*/ QNetworkProxy_free<()> for () {
  fn free(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxyD2Ev()};
     unsafe {_ZN13QNetworkProxyD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QNetworkProxy::hostName();
impl /*struct*/ QNetworkProxy {
  pub fn hostName<RetType, T: QNetworkProxy_hostName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hostName(self);
    // return 1;
  }
}

pub trait QNetworkProxy_hostName<RetType> {
  fn hostName(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  QString QNetworkProxy::hostName();
impl<'a> /*trait*/ QNetworkProxy_hostName<QString> for () {
  fn hostName(self , rsthis: & QNetworkProxy) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy8hostNameEv()};
    let mut ret = unsafe {_ZNK13QNetworkProxy8hostNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkProxy::QNetworkProxy();
impl /*struct*/ QNetworkProxy {
  pub fn new<T: QNetworkProxy_new>(value: T) -> QNetworkProxy {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkProxy_new {
  fn new(self) -> QNetworkProxy;
}

  // proto:  void QNetworkProxy::QNetworkProxy();
impl<'a> /*trait*/ QNetworkProxy_new for () {
  fn new(self) -> QNetworkProxy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxyC2Ev()};
    let ctysz: c_int = unsafe{QNetworkProxy_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN13QNetworkProxyC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkProxy{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QNetworkProxy::rawHeaderList();
impl /*struct*/ QNetworkProxy {
  pub fn rawHeaderList<RetType, T: QNetworkProxy_rawHeaderList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeaderList(self);
    // return 1;
  }
}

pub trait QNetworkProxy_rawHeaderList<RetType> {
  fn rawHeaderList(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  QList<QByteArray> QNetworkProxy::rawHeaderList();
impl<'a> /*trait*/ QNetworkProxy_rawHeaderList<()> for () {
  fn rawHeaderList(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy13rawHeaderListEv()};
     unsafe {_ZNK13QNetworkProxy13rawHeaderListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QNetworkProxy::setApplicationProxy(const QNetworkProxy & proxy);
impl /*struct*/ QNetworkProxy {
  pub fn setApplicationProxy_s<RetType, T: QNetworkProxy_setApplicationProxy_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationProxy_s();
    // return 1;
  }
}

pub trait QNetworkProxy_setApplicationProxy_s<RetType> {
  fn setApplicationProxy_s(self ) -> RetType;
}

  // proto: static void QNetworkProxy::setApplicationProxy(const QNetworkProxy & proxy);
impl<'a> /*trait*/ QNetworkProxy_setApplicationProxy_s<()> for (&'a QNetworkProxy) {
  fn setApplicationProxy_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy19setApplicationProxyERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkProxy19setApplicationProxyERKS_(arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxy::setHostName(const QString & hostName);
impl /*struct*/ QNetworkProxy {
  pub fn setHostName<RetType, T: QNetworkProxy_setHostName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHostName(self);
    // return 1;
  }
}

pub trait QNetworkProxy_setHostName<RetType> {
  fn setHostName(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::setHostName(const QString & hostName);
impl<'a> /*trait*/ QNetworkProxy_setHostName<()> for (&'a QString) {
  fn setHostName(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy11setHostNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkProxy11setHostNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QNetworkProxy::user();
impl /*struct*/ QNetworkProxy {
  pub fn user<RetType, T: QNetworkProxy_user<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.user(self);
    // return 1;
  }
}

pub trait QNetworkProxy_user<RetType> {
  fn user(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  QString QNetworkProxy::user();
impl<'a> /*trait*/ QNetworkProxy_user<QString> for () {
  fn user(self , rsthis: & QNetworkProxy) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy4userEv()};
    let mut ret = unsafe {_ZNK13QNetworkProxy4userEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QNetworkProxy::password();
impl /*struct*/ QNetworkProxy {
  pub fn password<RetType, T: QNetworkProxy_password<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.password(self);
    // return 1;
  }
}

pub trait QNetworkProxy_password<RetType> {
  fn password(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  QString QNetworkProxy::password();
impl<'a> /*trait*/ QNetworkProxy_password<QString> for () {
  fn password(self , rsthis: & QNetworkProxy) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy8passwordEv()};
    let mut ret = unsafe {_ZNK13QNetworkProxy8passwordEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QNetworkProxy::rawHeader(const QByteArray & headerName);
impl /*struct*/ QNetworkProxy {
  pub fn rawHeader<RetType, T: QNetworkProxy_rawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeader(self);
    // return 1;
  }
}

pub trait QNetworkProxy_rawHeader<RetType> {
  fn rawHeader(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  QByteArray QNetworkProxy::rawHeader(const QByteArray & headerName);
impl<'a> /*trait*/ QNetworkProxy_rawHeader<QByteArray> for (&'a QByteArray) {
  fn rawHeader(self , rsthis: & QNetworkProxy) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy9rawHeaderERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QNetworkProxy9rawHeaderERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QNetworkProxy::isTransparentProxy();
impl /*struct*/ QNetworkProxy {
  pub fn isTransparentProxy<RetType, T: QNetworkProxy_isTransparentProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTransparentProxy(self);
    // return 1;
  }
}

pub trait QNetworkProxy_isTransparentProxy<RetType> {
  fn isTransparentProxy(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  bool QNetworkProxy::isTransparentProxy();
impl<'a> /*trait*/ QNetworkProxy_isTransparentProxy<i8> for () {
  fn isTransparentProxy(self , rsthis: & QNetworkProxy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy18isTransparentProxyEv()};
    let mut ret = unsafe {_ZNK13QNetworkProxy18isTransparentProxyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkProxy::setPassword(const QString & password);
impl /*struct*/ QNetworkProxy {
  pub fn setPassword<RetType, T: QNetworkProxy_setPassword<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPassword(self);
    // return 1;
  }
}

pub trait QNetworkProxy_setPassword<RetType> {
  fn setPassword(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::setPassword(const QString & password);
impl<'a> /*trait*/ QNetworkProxy_setPassword<()> for (&'a QString) {
  fn setPassword(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy11setPasswordERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkProxy11setPasswordERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxy::QNetworkProxy(const QNetworkProxy & other);
impl<'a> /*trait*/ QNetworkProxy_new for (&'a QNetworkProxy) {
  fn new(self) -> QNetworkProxy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxyC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkProxy_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QNetworkProxyC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkProxy{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QNetworkProxy::hasRawHeader(const QByteArray & headerName);
impl /*struct*/ QNetworkProxy {
  pub fn hasRawHeader<RetType, T: QNetworkProxy_hasRawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasRawHeader(self);
    // return 1;
  }
}

pub trait QNetworkProxy_hasRawHeader<RetType> {
  fn hasRawHeader(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  bool QNetworkProxy::hasRawHeader(const QByteArray & headerName);
impl<'a> /*trait*/ QNetworkProxy_hasRawHeader<i8> for (&'a QByteArray) {
  fn hasRawHeader(self , rsthis: & QNetworkProxy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy12hasRawHeaderERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QNetworkProxy12hasRawHeaderERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkProxy::swap(QNetworkProxy & other);
impl /*struct*/ QNetworkProxy {
  pub fn swap<RetType, T: QNetworkProxy_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkProxy_swap<RetType> {
  fn swap(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::swap(QNetworkProxy & other);
impl<'a> /*trait*/ QNetworkProxy_swap<()> for (&'a QNetworkProxy) {
  fn swap(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkProxy4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  quint16 QNetworkProxy::port();
impl /*struct*/ QNetworkProxy {
  pub fn port<RetType, T: QNetworkProxy_port<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.port(self);
    // return 1;
  }
}

pub trait QNetworkProxy_port<RetType> {
  fn port(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  quint16 QNetworkProxy::port();
impl<'a> /*trait*/ QNetworkProxy_port<u16> for () {
  fn port(self , rsthis: & QNetworkProxy) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy4portEv()};
    let mut ret = unsafe {_ZNK13QNetworkProxy4portEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  void QNetworkProxy::setUser(const QString & userName);
impl /*struct*/ QNetworkProxy {
  pub fn setUser<RetType, T: QNetworkProxy_setUser<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUser(self);
    // return 1;
  }
}

pub trait QNetworkProxy_setUser<RetType> {
  fn setUser(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::setUser(const QString & userName);
impl<'a> /*trait*/ QNetworkProxy_setUser<()> for (&'a QString) {
  fn setUser(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy7setUserERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkProxy7setUserERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxy::setPort(quint16 port);
impl /*struct*/ QNetworkProxy {
  pub fn setPort<RetType, T: QNetworkProxy_setPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPort(self);
    // return 1;
  }
}

pub trait QNetworkProxy_setPort<RetType> {
  fn setPort(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::setPort(quint16 port);
impl<'a> /*trait*/ QNetworkProxy_setPort<()> for (u16) {
  fn setPort(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy7setPortEt()};
    let arg0 = self  as c_ushort;
     unsafe {_ZN13QNetworkProxy7setPortEt(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxy::setRawHeader(const QByteArray & headerName, const QByteArray & value);
impl /*struct*/ QNetworkProxy {
  pub fn setRawHeader<RetType, T: QNetworkProxy_setRawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawHeader(self);
    // return 1;
  }
}

pub trait QNetworkProxy_setRawHeader<RetType> {
  fn setRawHeader(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  void QNetworkProxy::setRawHeader(const QByteArray & headerName, const QByteArray & value);
impl<'a> /*trait*/ QNetworkProxy_setRawHeader<()> for (&'a QByteArray, &'a QByteArray) {
  fn setRawHeader(self , rsthis: & QNetworkProxy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkProxy12setRawHeaderERK10QByteArrayS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkProxy12setRawHeaderERK10QByteArrayS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QNetworkProxy::isCachingProxy();
impl /*struct*/ QNetworkProxy {
  pub fn isCachingProxy<RetType, T: QNetworkProxy_isCachingProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCachingProxy(self);
    // return 1;
  }
}

pub trait QNetworkProxy_isCachingProxy<RetType> {
  fn isCachingProxy(self , rsthis: & QNetworkProxy) -> RetType;
}

  // proto:  bool QNetworkProxy::isCachingProxy();
impl<'a> /*trait*/ QNetworkProxy_isCachingProxy<i8> for () {
  fn isCachingProxy(self , rsthis: & QNetworkProxy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkProxy14isCachingProxyEv()};
    let mut ret = unsafe {_ZNK13QNetworkProxy14isCachingProxyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QNetworkProxyFactory {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkProxyFactory {
    return QNetworkProxyFactory{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static void QNetworkProxyFactory::setUseSystemConfiguration(bool enable);
impl /*struct*/ QNetworkProxyFactory {
  pub fn setUseSystemConfiguration_s<RetType, T: QNetworkProxyFactory_setUseSystemConfiguration_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setUseSystemConfiguration_s();
    // return 1;
  }
}

pub trait QNetworkProxyFactory_setUseSystemConfiguration_s<RetType> {
  fn setUseSystemConfiguration_s(self ) -> RetType;
}

  // proto: static void QNetworkProxyFactory::setUseSystemConfiguration(bool enable);
impl<'a> /*trait*/ QNetworkProxyFactory_setUseSystemConfiguration_s<()> for (i8) {
  fn setUseSystemConfiguration_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactory25setUseSystemConfigurationEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN20QNetworkProxyFactory25setUseSystemConfigurationEb(arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyFactory::QNetworkProxyFactory();
impl /*struct*/ QNetworkProxyFactory {
  pub fn new<T: QNetworkProxyFactory_new>(value: T) -> QNetworkProxyFactory {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkProxyFactory_new {
  fn new(self) -> QNetworkProxyFactory;
}

  // proto:  void QNetworkProxyFactory::QNetworkProxyFactory();
impl<'a> /*trait*/ QNetworkProxyFactory_new for () {
  fn new(self) -> QNetworkProxyFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactoryC2Ev()};
    let ctysz: c_int = unsafe{QNetworkProxyFactory_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN20QNetworkProxyFactoryC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkProxyFactory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QNetworkProxyFactory::setApplicationProxyFactory(QNetworkProxyFactory * factory);
impl /*struct*/ QNetworkProxyFactory {
  pub fn setApplicationProxyFactory_s<RetType, T: QNetworkProxyFactory_setApplicationProxyFactory_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationProxyFactory_s();
    // return 1;
  }
}

pub trait QNetworkProxyFactory_setApplicationProxyFactory_s<RetType> {
  fn setApplicationProxyFactory_s(self ) -> RetType;
}

  // proto: static void QNetworkProxyFactory::setApplicationProxyFactory(QNetworkProxyFactory * factory);
impl<'a> /*trait*/ QNetworkProxyFactory_setApplicationProxyFactory_s<()> for (&'a QNetworkProxyFactory) {
  fn setApplicationProxyFactory_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactory26setApplicationProxyFactoryEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkProxyFactory26setApplicationProxyFactoryEPS_(arg0)};
    // return 1;
  }
}

  // proto: static QList<QNetworkProxy> QNetworkProxyFactory::systemProxyForQuery(const QNetworkProxyQuery & query);
impl /*struct*/ QNetworkProxyFactory {
  pub fn systemProxyForQuery_s<RetType, T: QNetworkProxyFactory_systemProxyForQuery_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemProxyForQuery_s();
    // return 1;
  }
}

pub trait QNetworkProxyFactory_systemProxyForQuery_s<RetType> {
  fn systemProxyForQuery_s(self ) -> RetType;
}

  // proto: static QList<QNetworkProxy> QNetworkProxyFactory::systemProxyForQuery(const QNetworkProxyQuery & query);
impl<'a> /*trait*/ QNetworkProxyFactory_systemProxyForQuery_s<()> for (&'a QNetworkProxyQuery) {
  fn systemProxyForQuery_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactory19systemProxyForQueryERK18QNetworkProxyQuery()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkProxyFactory19systemProxyForQueryERK18QNetworkProxyQuery(arg0)};
    // return 1;
  }
}

  // proto:  QList<QNetworkProxy> QNetworkProxyFactory::queryProxy(const QNetworkProxyQuery & query);
impl /*struct*/ QNetworkProxyFactory {
  pub fn queryProxy<RetType, T: QNetworkProxyFactory_queryProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.queryProxy(self);
    // return 1;
  }
}

pub trait QNetworkProxyFactory_queryProxy<RetType> {
  fn queryProxy(self , rsthis: & QNetworkProxyFactory) -> RetType;
}

  // proto:  QList<QNetworkProxy> QNetworkProxyFactory::queryProxy(const QNetworkProxyQuery & query);
impl<'a> /*trait*/ QNetworkProxyFactory_queryProxy<()> for (&'a QNetworkProxyQuery) {
  fn queryProxy(self , rsthis: & QNetworkProxyFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactory10queryProxyERK18QNetworkProxyQuery()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkProxyFactory10queryProxyERK18QNetworkProxyQuery(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyFactory::~QNetworkProxyFactory();
impl /*struct*/ QNetworkProxyFactory {
  pub fn free<RetType, T: QNetworkProxyFactory_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkProxyFactory_free<RetType> {
  fn free(self , rsthis: & QNetworkProxyFactory) -> RetType;
}

  // proto:  void QNetworkProxyFactory::~QNetworkProxyFactory();
impl<'a> /*trait*/ QNetworkProxyFactory_free<()> for () {
  fn free(self , rsthis: & QNetworkProxyFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactoryD2Ev()};
     unsafe {_ZN20QNetworkProxyFactoryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QList<QNetworkProxy> QNetworkProxyFactory::proxyForQuery(const QNetworkProxyQuery & query);
impl /*struct*/ QNetworkProxyFactory {
  pub fn proxyForQuery_s<RetType, T: QNetworkProxyFactory_proxyForQuery_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.proxyForQuery_s();
    // return 1;
  }
}

pub trait QNetworkProxyFactory_proxyForQuery_s<RetType> {
  fn proxyForQuery_s(self ) -> RetType;
}

  // proto: static QList<QNetworkProxy> QNetworkProxyFactory::proxyForQuery(const QNetworkProxyQuery & query);
impl<'a> /*trait*/ QNetworkProxyFactory_proxyForQuery_s<()> for (&'a QNetworkProxyQuery) {
  fn proxyForQuery_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkProxyFactory13proxyForQueryERK18QNetworkProxyQuery()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkProxyFactory13proxyForQueryERK18QNetworkProxyQuery(arg0)};
    // return 1;
  }
}

impl /*struct*/ QNetworkProxyQuery {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkProxyQuery {
    return QNetworkProxyQuery{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QUrl QNetworkProxyQuery::url();
impl /*struct*/ QNetworkProxyQuery {
  pub fn url<RetType, T: QNetworkProxyQuery_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_url<RetType> {
  fn url(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  QUrl QNetworkProxyQuery::url();
impl<'a> /*trait*/ QNetworkProxyQuery_url<QUrl> for () {
  fn url(self , rsthis: & QNetworkProxyQuery) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QNetworkProxyQuery3urlEv()};
    let mut ret = unsafe {_ZNK18QNetworkProxyQuery3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QNetworkProxyQuery::localPort();
impl /*struct*/ QNetworkProxyQuery {
  pub fn localPort<RetType, T: QNetworkProxyQuery_localPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPort(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_localPort<RetType> {
  fn localPort(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  int QNetworkProxyQuery::localPort();
impl<'a> /*trait*/ QNetworkProxyQuery_localPort<i32> for () {
  fn localPort(self , rsthis: & QNetworkProxyQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QNetworkProxyQuery9localPortEv()};
    let mut ret = unsafe {_ZNK18QNetworkProxyQuery9localPortEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::setNetworkConfiguration(const QNetworkConfiguration & networkConfiguration);
impl /*struct*/ QNetworkProxyQuery {
  pub fn setNetworkConfiguration<RetType, T: QNetworkProxyQuery_setNetworkConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNetworkConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_setNetworkConfiguration<RetType> {
  fn setNetworkConfiguration(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::setNetworkConfiguration(const QNetworkConfiguration & networkConfiguration);
impl<'a> /*trait*/ QNetworkProxyQuery_setNetworkConfiguration<()> for (&'a QNetworkConfiguration) {
  fn setNetworkConfiguration(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery23setNetworkConfigurationERK21QNetworkConfiguration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QNetworkProxyQuery23setNetworkConfigurationERK21QNetworkConfiguration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::setProtocolTag(const QString & protocolTag);
impl /*struct*/ QNetworkProxyQuery {
  pub fn setProtocolTag<RetType, T: QNetworkProxyQuery_setProtocolTag<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProtocolTag(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_setProtocolTag<RetType> {
  fn setProtocolTag(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::setProtocolTag(const QString & protocolTag);
impl<'a> /*trait*/ QNetworkProxyQuery_setProtocolTag<()> for (&'a QString) {
  fn setProtocolTag(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery14setProtocolTagERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QNetworkProxyQuery14setProtocolTagERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::setUrl(const QUrl & url);
impl /*struct*/ QNetworkProxyQuery {
  pub fn setUrl<RetType, T: QNetworkProxyQuery_setUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUrl(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_setUrl<RetType> {
  fn setUrl(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::setUrl(const QUrl & url);
impl<'a> /*trait*/ QNetworkProxyQuery_setUrl<()> for (&'a QUrl) {
  fn setUrl(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery6setUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QNetworkProxyQuery6setUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QNetworkProxyQuery::peerPort();
impl /*struct*/ QNetworkProxyQuery {
  pub fn peerPort<RetType, T: QNetworkProxyQuery_peerPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerPort(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_peerPort<RetType> {
  fn peerPort(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  int QNetworkProxyQuery::peerPort();
impl<'a> /*trait*/ QNetworkProxyQuery_peerPort<i32> for () {
  fn peerPort(self , rsthis: & QNetworkProxyQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QNetworkProxyQuery8peerPortEv()};
    let mut ret = unsafe {_ZNK18QNetworkProxyQuery8peerPortEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::setLocalPort(int port);
impl /*struct*/ QNetworkProxyQuery {
  pub fn setLocalPort<RetType, T: QNetworkProxyQuery_setLocalPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocalPort(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_setLocalPort<RetType> {
  fn setLocalPort(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::setLocalPort(int port);
impl<'a> /*trait*/ QNetworkProxyQuery_setLocalPort<()> for (i32) {
  fn setLocalPort(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery12setLocalPortEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QNetworkProxyQuery12setLocalPortEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::~QNetworkProxyQuery();
impl /*struct*/ QNetworkProxyQuery {
  pub fn free<RetType, T: QNetworkProxyQuery_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_free<RetType> {
  fn free(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::~QNetworkProxyQuery();
impl<'a> /*trait*/ QNetworkProxyQuery_free<()> for () {
  fn free(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQueryD2Ev()};
     unsafe {_ZN18QNetworkProxyQueryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::QNetworkProxyQuery(const QNetworkProxyQuery & other);
impl /*struct*/ QNetworkProxyQuery {
  pub fn new<T: QNetworkProxyQuery_new>(value: T) -> QNetworkProxyQuery {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkProxyQuery_new {
  fn new(self) -> QNetworkProxyQuery;
}

  // proto:  void QNetworkProxyQuery::QNetworkProxyQuery(const QNetworkProxyQuery & other);
impl<'a> /*trait*/ QNetworkProxyQuery_new for (&'a QNetworkProxyQuery) {
  fn new(self) -> QNetworkProxyQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQueryC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkProxyQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QNetworkProxyQueryC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkProxyQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::setPeerHostName(const QString & hostname);
impl /*struct*/ QNetworkProxyQuery {
  pub fn setPeerHostName<RetType, T: QNetworkProxyQuery_setPeerHostName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPeerHostName(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_setPeerHostName<RetType> {
  fn setPeerHostName(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::setPeerHostName(const QString & hostname);
impl<'a> /*trait*/ QNetworkProxyQuery_setPeerHostName<()> for (&'a QString) {
  fn setPeerHostName(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery15setPeerHostNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QNetworkProxyQuery15setPeerHostNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::swap(QNetworkProxyQuery & other);
impl /*struct*/ QNetworkProxyQuery {
  pub fn swap<RetType, T: QNetworkProxyQuery_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_swap<RetType> {
  fn swap(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::swap(QNetworkProxyQuery & other);
impl<'a> /*trait*/ QNetworkProxyQuery_swap<()> for (&'a QNetworkProxyQuery) {
  fn swap(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QNetworkProxyQuery4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::setPeerPort(int port);
impl /*struct*/ QNetworkProxyQuery {
  pub fn setPeerPort<RetType, T: QNetworkProxyQuery_setPeerPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPeerPort(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_setPeerPort<RetType> {
  fn setPeerPort(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  void QNetworkProxyQuery::setPeerPort(int port);
impl<'a> /*trait*/ QNetworkProxyQuery_setPeerPort<()> for (i32) {
  fn setPeerPort(self , rsthis: & QNetworkProxyQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQuery11setPeerPortEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QNetworkProxyQuery11setPeerPortEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkConfiguration QNetworkProxyQuery::networkConfiguration();
impl /*struct*/ QNetworkProxyQuery {
  pub fn networkConfiguration<RetType, T: QNetworkProxyQuery_networkConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.networkConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_networkConfiguration<RetType> {
  fn networkConfiguration(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  QNetworkConfiguration QNetworkProxyQuery::networkConfiguration();
impl<'a> /*trait*/ QNetworkProxyQuery_networkConfiguration<QNetworkConfiguration> for () {
  fn networkConfiguration(self , rsthis: & QNetworkProxyQuery) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QNetworkProxyQuery20networkConfigurationEv()};
    let mut ret = unsafe {_ZNK18QNetworkProxyQuery20networkConfigurationEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QNetworkProxyQuery::peerHostName();
impl /*struct*/ QNetworkProxyQuery {
  pub fn peerHostName<RetType, T: QNetworkProxyQuery_peerHostName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerHostName(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_peerHostName<RetType> {
  fn peerHostName(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  QString QNetworkProxyQuery::peerHostName();
impl<'a> /*trait*/ QNetworkProxyQuery_peerHostName<QString> for () {
  fn peerHostName(self , rsthis: & QNetworkProxyQuery) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QNetworkProxyQuery12peerHostNameEv()};
    let mut ret = unsafe {_ZNK18QNetworkProxyQuery12peerHostNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QNetworkProxyQuery::protocolTag();
impl /*struct*/ QNetworkProxyQuery {
  pub fn protocolTag<RetType, T: QNetworkProxyQuery_protocolTag<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.protocolTag(self);
    // return 1;
  }
}

pub trait QNetworkProxyQuery_protocolTag<RetType> {
  fn protocolTag(self , rsthis: & QNetworkProxyQuery) -> RetType;
}

  // proto:  QString QNetworkProxyQuery::protocolTag();
impl<'a> /*trait*/ QNetworkProxyQuery_protocolTag<QString> for () {
  fn protocolTag(self , rsthis: & QNetworkProxyQuery) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QNetworkProxyQuery11protocolTagEv()};
    let mut ret = unsafe {_ZNK18QNetworkProxyQuery11protocolTagEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkProxyQuery::QNetworkProxyQuery();
impl<'a> /*trait*/ QNetworkProxyQuery_new for () {
  fn new(self) -> QNetworkProxyQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QNetworkProxyQueryC2Ev()};
    let ctysz: c_int = unsafe{QNetworkProxyQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN18QNetworkProxyQueryC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkProxyQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

