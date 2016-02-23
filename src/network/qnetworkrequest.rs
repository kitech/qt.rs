// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qnetworkrequest.h
// dst-file: /src/network/qnetworkrequest.rs
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
use super::super::core::qobject::QObject; // 771
use super::super::core::qurl::QUrl; // 771
use super::qsslconfiguration::QSslConfiguration; // 773
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkRequest_Class_Size() -> c_int;
  // proto:  void QNetworkRequest::setOriginatingObject(QObject * object);
  fn _ZN15QNetworkRequest20setOriginatingObjectEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QUrl QNetworkRequest::url();
  fn _ZNK15QNetworkRequest3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkRequest::setSslConfiguration(const QSslConfiguration & configuration);
  fn _ZN15QNetworkRequest19setSslConfigurationERK17QSslConfiguration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QNetworkRequest::rawHeader(const QByteArray & headerName);
  fn _ZNK15QNetworkRequest9rawHeaderERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkRequest::QNetworkRequest(const QNetworkRequest & other);
  fn _ZN15QNetworkRequestC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QByteArray> QNetworkRequest::rawHeaderList();
  fn _ZNK15QNetworkRequest13rawHeaderListEv(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QNetworkRequest::originatingObject();
  fn _ZNK15QNetworkRequest17originatingObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkRequest::swap(QNetworkRequest & other);
  fn _ZN15QNetworkRequest4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkRequest::~QNetworkRequest();
  fn _ZN15QNetworkRequestD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkRequest::hasRawHeader(const QByteArray & headerName);
  fn _ZNK15QNetworkRequest12hasRawHeaderERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QSslConfiguration QNetworkRequest::sslConfiguration();
  fn _ZNK15QNetworkRequest16sslConfigurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkRequest::setUrl(const QUrl & url);
  fn _ZN15QNetworkRequest6setUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkRequest::QNetworkRequest(const QUrl & url);
  fn _ZN15QNetworkRequestC2ERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkRequest::setRawHeader(const QByteArray & headerName, const QByteArray & value);
  fn _ZN15QNetworkRequest12setRawHeaderERK10QByteArrayS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkRequest)=1
#[derive(Default)]
pub struct QNetworkRequest {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkRequest {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkRequest {
    return QNetworkRequest{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QNetworkRequest::setOriginatingObject(QObject * object);
impl /*struct*/ QNetworkRequest {
  pub fn setOriginatingObject<RetType, T: QNetworkRequest_setOriginatingObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOriginatingObject(self);
    // return 1;
  }
}

pub trait QNetworkRequest_setOriginatingObject<RetType> {
  fn setOriginatingObject(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  void QNetworkRequest::setOriginatingObject(QObject * object);
impl<'a> /*trait*/ QNetworkRequest_setOriginatingObject<()> for (&'a QObject) {
  fn setOriginatingObject(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequest20setOriginatingObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QNetworkRequest20setOriginatingObjectEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QNetworkRequest::url();
impl /*struct*/ QNetworkRequest {
  pub fn url<RetType, T: QNetworkRequest_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QNetworkRequest_url<RetType> {
  fn url(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  QUrl QNetworkRequest::url();
impl<'a> /*trait*/ QNetworkRequest_url<QUrl> for () {
  fn url(self , rsthis: & QNetworkRequest) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkRequest3urlEv()};
    let mut ret = unsafe {_ZNK15QNetworkRequest3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkRequest::setSslConfiguration(const QSslConfiguration & configuration);
impl /*struct*/ QNetworkRequest {
  pub fn setSslConfiguration<RetType, T: QNetworkRequest_setSslConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSslConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkRequest_setSslConfiguration<RetType> {
  fn setSslConfiguration(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  void QNetworkRequest::setSslConfiguration(const QSslConfiguration & configuration);
impl<'a> /*trait*/ QNetworkRequest_setSslConfiguration<()> for (&'a QSslConfiguration) {
  fn setSslConfiguration(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequest19setSslConfigurationERK17QSslConfiguration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QNetworkRequest19setSslConfigurationERK17QSslConfiguration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QNetworkRequest::rawHeader(const QByteArray & headerName);
impl /*struct*/ QNetworkRequest {
  pub fn rawHeader<RetType, T: QNetworkRequest_rawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeader(self);
    // return 1;
  }
}

pub trait QNetworkRequest_rawHeader<RetType> {
  fn rawHeader(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  QByteArray QNetworkRequest::rawHeader(const QByteArray & headerName);
impl<'a> /*trait*/ QNetworkRequest_rawHeader<QByteArray> for (&'a QByteArray) {
  fn rawHeader(self , rsthis: & QNetworkRequest) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkRequest9rawHeaderERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QNetworkRequest9rawHeaderERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkRequest::QNetworkRequest(const QNetworkRequest & other);
impl /*struct*/ QNetworkRequest {
  pub fn new<T: QNetworkRequest_new>(value: T) -> QNetworkRequest {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkRequest_new {
  fn new(self) -> QNetworkRequest;
}

  // proto:  void QNetworkRequest::QNetworkRequest(const QNetworkRequest & other);
impl<'a> /*trait*/ QNetworkRequest_new for (&'a QNetworkRequest) {
  fn new(self) -> QNetworkRequest {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequestC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkRequest_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QNetworkRequestC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkRequest{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QNetworkRequest::rawHeaderList();
impl /*struct*/ QNetworkRequest {
  pub fn rawHeaderList<RetType, T: QNetworkRequest_rawHeaderList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeaderList(self);
    // return 1;
  }
}

pub trait QNetworkRequest_rawHeaderList<RetType> {
  fn rawHeaderList(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  QList<QByteArray> QNetworkRequest::rawHeaderList();
impl<'a> /*trait*/ QNetworkRequest_rawHeaderList<()> for () {
  fn rawHeaderList(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkRequest13rawHeaderListEv()};
     unsafe {_ZNK15QNetworkRequest13rawHeaderListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QNetworkRequest::originatingObject();
impl /*struct*/ QNetworkRequest {
  pub fn originatingObject<RetType, T: QNetworkRequest_originatingObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.originatingObject(self);
    // return 1;
  }
}

pub trait QNetworkRequest_originatingObject<RetType> {
  fn originatingObject(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  QObject * QNetworkRequest::originatingObject();
impl<'a> /*trait*/ QNetworkRequest_originatingObject<QObject> for () {
  fn originatingObject(self , rsthis: & QNetworkRequest) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkRequest17originatingObjectEv()};
    let mut ret = unsafe {_ZNK15QNetworkRequest17originatingObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkRequest::swap(QNetworkRequest & other);
impl /*struct*/ QNetworkRequest {
  pub fn swap<RetType, T: QNetworkRequest_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkRequest_swap<RetType> {
  fn swap(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  void QNetworkRequest::swap(QNetworkRequest & other);
impl<'a> /*trait*/ QNetworkRequest_swap<()> for (&'a QNetworkRequest) {
  fn swap(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequest4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QNetworkRequest4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkRequest::~QNetworkRequest();
impl /*struct*/ QNetworkRequest {
  pub fn free<RetType, T: QNetworkRequest_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkRequest_free<RetType> {
  fn free(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  void QNetworkRequest::~QNetworkRequest();
impl<'a> /*trait*/ QNetworkRequest_free<()> for () {
  fn free(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequestD2Ev()};
     unsafe {_ZN15QNetworkRequestD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QNetworkRequest::hasRawHeader(const QByteArray & headerName);
impl /*struct*/ QNetworkRequest {
  pub fn hasRawHeader<RetType, T: QNetworkRequest_hasRawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasRawHeader(self);
    // return 1;
  }
}

pub trait QNetworkRequest_hasRawHeader<RetType> {
  fn hasRawHeader(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  bool QNetworkRequest::hasRawHeader(const QByteArray & headerName);
impl<'a> /*trait*/ QNetworkRequest_hasRawHeader<i8> for (&'a QByteArray) {
  fn hasRawHeader(self , rsthis: & QNetworkRequest) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkRequest12hasRawHeaderERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QNetworkRequest12hasRawHeaderERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSslConfiguration QNetworkRequest::sslConfiguration();
impl /*struct*/ QNetworkRequest {
  pub fn sslConfiguration<RetType, T: QNetworkRequest_sslConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sslConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkRequest_sslConfiguration<RetType> {
  fn sslConfiguration(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  QSslConfiguration QNetworkRequest::sslConfiguration();
impl<'a> /*trait*/ QNetworkRequest_sslConfiguration<QSslConfiguration> for () {
  fn sslConfiguration(self , rsthis: & QNetworkRequest) -> QSslConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkRequest16sslConfigurationEv()};
    let mut ret = unsafe {_ZNK15QNetworkRequest16sslConfigurationEv(rsthis.qclsinst)};
    let mut ret1 = QSslConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkRequest::setUrl(const QUrl & url);
impl /*struct*/ QNetworkRequest {
  pub fn setUrl<RetType, T: QNetworkRequest_setUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUrl(self);
    // return 1;
  }
}

pub trait QNetworkRequest_setUrl<RetType> {
  fn setUrl(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  void QNetworkRequest::setUrl(const QUrl & url);
impl<'a> /*trait*/ QNetworkRequest_setUrl<()> for (&'a QUrl) {
  fn setUrl(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequest6setUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QNetworkRequest6setUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkRequest::QNetworkRequest(const QUrl & url);
impl<'a> /*trait*/ QNetworkRequest_new for (&'a QUrl) {
  fn new(self) -> QNetworkRequest {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequestC2ERK4QUrl()};
    let ctysz: c_int = unsafe{QNetworkRequest_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QNetworkRequestC2ERK4QUrl(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkRequest{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkRequest::setRawHeader(const QByteArray & headerName, const QByteArray & value);
impl /*struct*/ QNetworkRequest {
  pub fn setRawHeader<RetType, T: QNetworkRequest_setRawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawHeader(self);
    // return 1;
  }
}

pub trait QNetworkRequest_setRawHeader<RetType> {
  fn setRawHeader(self , rsthis: & QNetworkRequest) -> RetType;
}

  // proto:  void QNetworkRequest::setRawHeader(const QByteArray & headerName, const QByteArray & value);
impl<'a> /*trait*/ QNetworkRequest_setRawHeader<()> for (&'a QByteArray, &'a QByteArray) {
  fn setRawHeader(self , rsthis: & QNetworkRequest) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkRequest12setRawHeaderERK10QByteArrayS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QNetworkRequest12setRawHeaderERK10QByteArrayS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

