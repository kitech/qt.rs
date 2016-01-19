// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworkreply.h
// dst-file: /src/network/qnetworkreply.rs
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
use super::super::core::qiodevice::QIODevice; // 771
use std::ops::Deref;
use super::super::core::qbytearray::QByteArray; // 771
use super::qnetworkrequest::QNetworkRequest; // 773
use super::qsslconfiguration::QSslConfiguration; // 773
use super::super::core::qurl::QUrl; // 771
use super::qnetworkaccessmanager::QNetworkAccessManager; // 773
use super::super::core::qobject::QObject; // 771
use super::qsslpresharedkeyauthenticator::QSslPreSharedKeyAuthenticator; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkReply_Class_Size() -> c_int;
  // proto:  void QNetworkReply::ignoreSslErrors();
  fn _ZN13QNetworkReply15ignoreSslErrorsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkReply::hasRawHeader(const QByteArray & headerName);
  fn _ZNK13QNetworkReply12hasRawHeaderERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QNetworkReply::setReadBufferSize(qint64 size);
  fn _ZN13QNetworkReply17setReadBufferSizeEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  QNetworkRequest QNetworkReply::request();
  fn _ZNK13QNetworkReply7requestEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkReply::abort();
  fn _ZN13QNetworkReply5abortEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkReply::isRunning();
  fn _ZNK13QNetworkReply9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QList<QByteArray> QNetworkReply::rawHeaderList();
  fn _ZNK13QNetworkReply13rawHeaderListEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QList<RawHeaderPair> & QNetworkReply::rawHeaderPairs();
  fn _ZNK13QNetworkReply14rawHeaderPairsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkReply::setSslConfiguration(const QSslConfiguration & configuration);
  fn _ZN13QNetworkReply19setSslConfigurationERK17QSslConfiguration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QUrl QNetworkReply::url();
  fn _ZNK13QNetworkReply3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QNetworkReply::isSequential();
  fn _ZNK13QNetworkReply12isSequentialEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QByteArray QNetworkReply::rawHeader(const QByteArray & headerName);
  fn _ZNK13QNetworkReply9rawHeaderERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QNetworkAccessManager * QNetworkReply::manager();
  fn _ZNK13QNetworkReply7managerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkReply::QNetworkReply(QObject * parent);
  fn _ZN13QNetworkReplyC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkReply::close();
  fn _ZN13QNetworkReply5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkReply::isFinished();
  fn _ZNK13QNetworkReply10isFinishedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QNetworkReply::readBufferSize();
  fn _ZNK13QNetworkReply14readBufferSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QSslConfiguration QNetworkReply::sslConfiguration();
  fn _ZNK13QNetworkReply16sslConfigurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QNetworkReply::metaObject();
  fn _ZNK13QNetworkReply10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkReply::~QNetworkReply();
  fn _ZN13QNetworkReplyD2Ev(qthis: u64 /* *mut c_void*/);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply16downloadProgressExx(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply8finishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply5errorENS_12NetworkErrorE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply15metaDataChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply14uploadProgressExx(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply9encryptedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkReply_SlotProxy_connect__ZN13QNetworkReply34preSharedKeyAuthenticationRequiredEP29QSslPreSharedKeyAuthenticator(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkReply)=1
#[derive(Default)]
pub struct QNetworkReply {
  qbase: QIODevice,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _preSharedKeyAuthenticationRequired: QNetworkReply_preSharedKeyAuthenticationRequired_signal,
  pub _metaDataChanged: QNetworkReply_metaDataChanged_signal,
  pub _encrypted: QNetworkReply_encrypted_signal,
  pub _sslErrors: QNetworkReply_sslErrors_signal,
  pub _uploadProgress: QNetworkReply_uploadProgress_signal,
  pub _downloadProgress: QNetworkReply_downloadProgress_signal,
  pub _finished: QNetworkReply_finished_signal,
  pub _error: QNetworkReply_error_signal,
}

impl /*struct*/ QNetworkReply {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkReply {
    return QNetworkReply{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNetworkReply {
  type Target = QIODevice;

  fn deref(&self) -> &QIODevice {
    return & self.qbase;
  }
}
impl AsRef<QIODevice> for QNetworkReply {
  fn as_ref(& self) -> & QIODevice {
    return & self.qbase;
  }
}
  // proto:  void QNetworkReply::ignoreSslErrors();
impl /*struct*/ QNetworkReply {
  pub fn ignoreSslErrors<RetType, T: QNetworkReply_ignoreSslErrors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignoreSslErrors(self);
    // return 1;
  }
}

pub trait QNetworkReply_ignoreSslErrors<RetType> {
  fn ignoreSslErrors(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  void QNetworkReply::ignoreSslErrors();
impl<'a> /*trait*/ QNetworkReply_ignoreSslErrors<()> for () {
  fn ignoreSslErrors(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReply15ignoreSslErrorsEv()};
     unsafe {_ZN13QNetworkReply15ignoreSslErrorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QNetworkReply::hasRawHeader(const QByteArray & headerName);
impl /*struct*/ QNetworkReply {
  pub fn hasRawHeader<RetType, T: QNetworkReply_hasRawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasRawHeader(self);
    // return 1;
  }
}

pub trait QNetworkReply_hasRawHeader<RetType> {
  fn hasRawHeader(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  bool QNetworkReply::hasRawHeader(const QByteArray & headerName);
impl<'a> /*trait*/ QNetworkReply_hasRawHeader<i8> for (&'a QByteArray) {
  fn hasRawHeader(self , rsthis: & QNetworkReply) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply12hasRawHeaderERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QNetworkReply12hasRawHeaderERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkReply::setReadBufferSize(qint64 size);
impl /*struct*/ QNetworkReply {
  pub fn setReadBufferSize<RetType, T: QNetworkReply_setReadBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadBufferSize(self);
    // return 1;
  }
}

pub trait QNetworkReply_setReadBufferSize<RetType> {
  fn setReadBufferSize(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  void QNetworkReply::setReadBufferSize(qint64 size);
impl<'a> /*trait*/ QNetworkReply_setReadBufferSize<()> for (i64) {
  fn setReadBufferSize(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReply17setReadBufferSizeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN13QNetworkReply17setReadBufferSizeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkRequest QNetworkReply::request();
impl /*struct*/ QNetworkReply {
  pub fn request<RetType, T: QNetworkReply_request<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.request(self);
    // return 1;
  }
}

pub trait QNetworkReply_request<RetType> {
  fn request(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  QNetworkRequest QNetworkReply::request();
impl<'a> /*trait*/ QNetworkReply_request<QNetworkRequest> for () {
  fn request(self , rsthis: & QNetworkReply) -> QNetworkRequest {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply7requestEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply7requestEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkRequest::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkReply::abort();
impl /*struct*/ QNetworkReply {
  pub fn abort<RetType, T: QNetworkReply_abort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.abort(self);
    // return 1;
  }
}

pub trait QNetworkReply_abort<RetType> {
  fn abort(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  void QNetworkReply::abort();
impl<'a> /*trait*/ QNetworkReply_abort<()> for () {
  fn abort(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReply5abortEv()};
     unsafe {_ZN13QNetworkReply5abortEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QNetworkReply::isRunning();
impl /*struct*/ QNetworkReply {
  pub fn isRunning<RetType, T: QNetworkReply_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QNetworkReply_isRunning<RetType> {
  fn isRunning(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  bool QNetworkReply::isRunning();
impl<'a> /*trait*/ QNetworkReply_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QNetworkReply) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply9isRunningEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QNetworkReply::rawHeaderList();
impl /*struct*/ QNetworkReply {
  pub fn rawHeaderList<RetType, T: QNetworkReply_rawHeaderList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeaderList(self);
    // return 1;
  }
}

pub trait QNetworkReply_rawHeaderList<RetType> {
  fn rawHeaderList(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  QList<QByteArray> QNetworkReply::rawHeaderList();
impl<'a> /*trait*/ QNetworkReply_rawHeaderList<()> for () {
  fn rawHeaderList(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply13rawHeaderListEv()};
     unsafe {_ZNK13QNetworkReply13rawHeaderListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QList<RawHeaderPair> & QNetworkReply::rawHeaderPairs();
impl /*struct*/ QNetworkReply {
  pub fn rawHeaderPairs<RetType, T: QNetworkReply_rawHeaderPairs<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeaderPairs(self);
    // return 1;
  }
}

pub trait QNetworkReply_rawHeaderPairs<RetType> {
  fn rawHeaderPairs(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  const QList<RawHeaderPair> & QNetworkReply::rawHeaderPairs();
impl<'a> /*trait*/ QNetworkReply_rawHeaderPairs<()> for () {
  fn rawHeaderPairs(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply14rawHeaderPairsEv()};
     unsafe {_ZNK13QNetworkReply14rawHeaderPairsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkReply::setSslConfiguration(const QSslConfiguration & configuration);
impl /*struct*/ QNetworkReply {
  pub fn setSslConfiguration<RetType, T: QNetworkReply_setSslConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSslConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkReply_setSslConfiguration<RetType> {
  fn setSslConfiguration(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  void QNetworkReply::setSslConfiguration(const QSslConfiguration & configuration);
impl<'a> /*trait*/ QNetworkReply_setSslConfiguration<()> for (&'a QSslConfiguration) {
  fn setSslConfiguration(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReply19setSslConfigurationERK17QSslConfiguration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QNetworkReply19setSslConfigurationERK17QSslConfiguration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QNetworkReply::url();
impl /*struct*/ QNetworkReply {
  pub fn url<RetType, T: QNetworkReply_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QNetworkReply_url<RetType> {
  fn url(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  QUrl QNetworkReply::url();
impl<'a> /*trait*/ QNetworkReply_url<QUrl> for () {
  fn url(self , rsthis: & QNetworkReply) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply3urlEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QNetworkReply::isSequential();
impl /*struct*/ QNetworkReply {
  pub fn isSequential<RetType, T: QNetworkReply_isSequential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSequential(self);
    // return 1;
  }
}

pub trait QNetworkReply_isSequential<RetType> {
  fn isSequential(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  bool QNetworkReply::isSequential();
impl<'a> /*trait*/ QNetworkReply_isSequential<i8> for () {
  fn isSequential(self , rsthis: & QNetworkReply) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply12isSequentialEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QNetworkReply::rawHeader(const QByteArray & headerName);
impl /*struct*/ QNetworkReply {
  pub fn rawHeader<RetType, T: QNetworkReply_rawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeader(self);
    // return 1;
  }
}

pub trait QNetworkReply_rawHeader<RetType> {
  fn rawHeader(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  QByteArray QNetworkReply::rawHeader(const QByteArray & headerName);
impl<'a> /*trait*/ QNetworkReply_rawHeader<QByteArray> for (&'a QByteArray) {
  fn rawHeader(self , rsthis: & QNetworkReply) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply9rawHeaderERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QNetworkReply9rawHeaderERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QNetworkAccessManager * QNetworkReply::manager();
impl /*struct*/ QNetworkReply {
  pub fn manager<RetType, T: QNetworkReply_manager<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.manager(self);
    // return 1;
  }
}

pub trait QNetworkReply_manager<RetType> {
  fn manager(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  QNetworkAccessManager * QNetworkReply::manager();
impl<'a> /*trait*/ QNetworkReply_manager<QNetworkAccessManager> for () {
  fn manager(self , rsthis: & QNetworkReply) -> QNetworkAccessManager {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply7managerEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply7managerEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkAccessManager::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkReply::QNetworkReply(QObject * parent);
impl /*struct*/ QNetworkReply {
  pub fn new<T: QNetworkReply_new>(value: T) -> QNetworkReply {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkReply_new {
  fn new(self) -> QNetworkReply;
}

  // proto:  void QNetworkReply::QNetworkReply(QObject * parent);
impl<'a> /*trait*/ QNetworkReply_new for (&'a QObject) {
  fn new(self) -> QNetworkReply {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReplyC2EP7QObject()};
    let ctysz: c_int = unsafe{QNetworkReply_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QNetworkReplyC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkReply{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkReply::close();
impl /*struct*/ QNetworkReply {
  pub fn close<RetType, T: QNetworkReply_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QNetworkReply_close<RetType> {
  fn close(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  void QNetworkReply::close();
impl<'a> /*trait*/ QNetworkReply_close<()> for () {
  fn close(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReply5closeEv()};
     unsafe {_ZN13QNetworkReply5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QNetworkReply::isFinished();
impl /*struct*/ QNetworkReply {
  pub fn isFinished<RetType, T: QNetworkReply_isFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinished(self);
    // return 1;
  }
}

pub trait QNetworkReply_isFinished<RetType> {
  fn isFinished(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  bool QNetworkReply::isFinished();
impl<'a> /*trait*/ QNetworkReply_isFinished<i8> for () {
  fn isFinished(self , rsthis: & QNetworkReply) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply10isFinishedEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QNetworkReply::readBufferSize();
impl /*struct*/ QNetworkReply {
  pub fn readBufferSize<RetType, T: QNetworkReply_readBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readBufferSize(self);
    // return 1;
  }
}

pub trait QNetworkReply_readBufferSize<RetType> {
  fn readBufferSize(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  qint64 QNetworkReply::readBufferSize();
impl<'a> /*trait*/ QNetworkReply_readBufferSize<i64> for () {
  fn readBufferSize(self , rsthis: & QNetworkReply) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply14readBufferSizeEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply14readBufferSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QSslConfiguration QNetworkReply::sslConfiguration();
impl /*struct*/ QNetworkReply {
  pub fn sslConfiguration<RetType, T: QNetworkReply_sslConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sslConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkReply_sslConfiguration<RetType> {
  fn sslConfiguration(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  QSslConfiguration QNetworkReply::sslConfiguration();
impl<'a> /*trait*/ QNetworkReply_sslConfiguration<QSslConfiguration> for () {
  fn sslConfiguration(self , rsthis: & QNetworkReply) -> QSslConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply16sslConfigurationEv()};
    let mut ret = unsafe {_ZNK13QNetworkReply16sslConfigurationEv(rsthis.qclsinst)};
    let mut ret1 = QSslConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QNetworkReply::metaObject();
impl /*struct*/ QNetworkReply {
  pub fn metaObject<RetType, T: QNetworkReply_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QNetworkReply_metaObject<RetType> {
  fn metaObject(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  const QMetaObject * QNetworkReply::metaObject();
impl<'a> /*trait*/ QNetworkReply_metaObject<()> for () {
  fn metaObject(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QNetworkReply10metaObjectEv()};
     unsafe {_ZNK13QNetworkReply10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkReply::~QNetworkReply();
impl /*struct*/ QNetworkReply {
  pub fn free<RetType, T: QNetworkReply_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkReply_free<RetType> {
  fn free(self , rsthis: & QNetworkReply) -> RetType;
}

  // proto:  void QNetworkReply::~QNetworkReply();
impl<'a> /*trait*/ QNetworkReply_free<()> for () {
  fn free(self , rsthis: & QNetworkReply) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QNetworkReplyD2Ev()};
     unsafe {_ZN13QNetworkReplyD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QNetworkReply_preSharedKeyAuthenticationRequired
pub struct QNetworkReply_preSharedKeyAuthenticationRequired_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn preSharedKeyAuthenticationRequired(&self) -> QNetworkReply_preSharedKeyAuthenticationRequired_signal {
     return QNetworkReply_preSharedKeyAuthenticationRequired_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_preSharedKeyAuthenticationRequired_signal {
  pub fn connect<T: QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect {
  fn connect(self, sigthis: QNetworkReply_preSharedKeyAuthenticationRequired_signal);
}

#[derive(Default)] // for QNetworkReply_metaDataChanged
pub struct QNetworkReply_metaDataChanged_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn metaDataChanged(&self) -> QNetworkReply_metaDataChanged_signal {
     return QNetworkReply_metaDataChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_metaDataChanged_signal {
  pub fn connect<T: QNetworkReply_metaDataChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_metaDataChanged_signal_connect {
  fn connect(self, sigthis: QNetworkReply_metaDataChanged_signal);
}

#[derive(Default)] // for QNetworkReply_encrypted
pub struct QNetworkReply_encrypted_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn encrypted(&self) -> QNetworkReply_encrypted_signal {
     return QNetworkReply_encrypted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_encrypted_signal {
  pub fn connect<T: QNetworkReply_encrypted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_encrypted_signal_connect {
  fn connect(self, sigthis: QNetworkReply_encrypted_signal);
}

#[derive(Default)] // for QNetworkReply_sslErrors
pub struct QNetworkReply_sslErrors_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn sslErrors(&self) -> QNetworkReply_sslErrors_signal {
     return QNetworkReply_sslErrors_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_sslErrors_signal {
  pub fn connect<T: QNetworkReply_sslErrors_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_sslErrors_signal_connect {
  fn connect(self, sigthis: QNetworkReply_sslErrors_signal);
}

#[derive(Default)] // for QNetworkReply_uploadProgress
pub struct QNetworkReply_uploadProgress_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn uploadProgress(&self) -> QNetworkReply_uploadProgress_signal {
     return QNetworkReply_uploadProgress_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_uploadProgress_signal {
  pub fn connect<T: QNetworkReply_uploadProgress_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_uploadProgress_signal_connect {
  fn connect(self, sigthis: QNetworkReply_uploadProgress_signal);
}

#[derive(Default)] // for QNetworkReply_downloadProgress
pub struct QNetworkReply_downloadProgress_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn downloadProgress(&self) -> QNetworkReply_downloadProgress_signal {
     return QNetworkReply_downloadProgress_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_downloadProgress_signal {
  pub fn connect<T: QNetworkReply_downloadProgress_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_downloadProgress_signal_connect {
  fn connect(self, sigthis: QNetworkReply_downloadProgress_signal);
}

#[derive(Default)] // for QNetworkReply_finished
pub struct QNetworkReply_finished_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn finished(&self) -> QNetworkReply_finished_signal {
     return QNetworkReply_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_finished_signal {
  pub fn connect<T: QNetworkReply_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_finished_signal_connect {
  fn connect(self, sigthis: QNetworkReply_finished_signal);
}

#[derive(Default)] // for QNetworkReply_error
pub struct QNetworkReply_error_signal{poi:u64}
impl /* struct */ QNetworkReply {
  pub fn error(&self) -> QNetworkReply_error_signal {
     return QNetworkReply_error_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkReply_error_signal {
  pub fn connect<T: QNetworkReply_error_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkReply_error_signal_connect {
  fn connect(self, sigthis: QNetworkReply_error_signal);
}

// downloadProgress(qint64, qint64)
extern fn QNetworkReply_downloadProgress_signal_connect_cb_0(rsfptr:fn(i64, i64), arg0: c_longlong, arg1: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i64;
  let rsarg1 = arg1 as i64;
  rsfptr(rsarg0,rsarg1);
}
extern fn QNetworkReply_downloadProgress_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i64, i64)>, arg0: c_longlong, arg1: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i64;
  let rsarg1 = arg1 as i64;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QNetworkReply_downloadProgress_signal_connect for fn(i64, i64) {
  fn connect(self, sigthis: QNetworkReply_downloadProgress_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_downloadProgress_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply16downloadProgressExx(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_downloadProgress_signal_connect for Box<Fn(i64, i64)> {
  fn connect(self, sigthis: QNetworkReply_downloadProgress_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_downloadProgress_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply16downloadProgressExx(arg0, arg1, arg2)};
  }
}
// finished()
extern fn QNetworkReply_finished_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkReply_finished_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkReply_finished_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkReply_finished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_finished_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply8finishedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_finished_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkReply_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_finished_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply8finishedEv(arg0, arg1, arg2)};
  }
}
// error(class QNetworkReply::NetworkError)
extern fn QNetworkReply_error_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QNetworkReply_error_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkReply_error_signal_connect for fn(i32) {
  fn connect(self, sigthis: QNetworkReply_error_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_error_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply5errorENS_12NetworkErrorE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_error_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QNetworkReply_error_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_error_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply5errorENS_12NetworkErrorE(arg0, arg1, arg2)};
  }
}
// metaDataChanged()
extern fn QNetworkReply_metaDataChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkReply_metaDataChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkReply_metaDataChanged_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkReply_metaDataChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_metaDataChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply15metaDataChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_metaDataChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkReply_metaDataChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_metaDataChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply15metaDataChangedEv(arg0, arg1, arg2)};
  }
}
// uploadProgress(qint64, qint64)
extern fn QNetworkReply_uploadProgress_signal_connect_cb_4(rsfptr:fn(i64, i64), arg0: c_longlong, arg1: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i64;
  let rsarg1 = arg1 as i64;
  rsfptr(rsarg0,rsarg1);
}
extern fn QNetworkReply_uploadProgress_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i64, i64)>, arg0: c_longlong, arg1: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i64;
  let rsarg1 = arg1 as i64;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QNetworkReply_uploadProgress_signal_connect for fn(i64, i64) {
  fn connect(self, sigthis: QNetworkReply_uploadProgress_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_uploadProgress_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply14uploadProgressExx(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_uploadProgress_signal_connect for Box<Fn(i64, i64)> {
  fn connect(self, sigthis: QNetworkReply_uploadProgress_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_uploadProgress_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply14uploadProgressExx(arg0, arg1, arg2)};
  }
}
// encrypted()
extern fn QNetworkReply_encrypted_signal_connect_cb_5(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkReply_encrypted_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkReply_encrypted_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkReply_encrypted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_encrypted_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply9encryptedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_encrypted_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkReply_encrypted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_encrypted_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply9encryptedEv(arg0, arg1, arg2)};
  }
}
// preSharedKeyAuthenticationRequired(class QSslPreSharedKeyAuthenticator *)
extern fn QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect_cb_6(rsfptr:fn(QSslPreSharedKeyAuthenticator), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QSslPreSharedKeyAuthenticator::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(QSslPreSharedKeyAuthenticator)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QSslPreSharedKeyAuthenticator::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect for fn(QSslPreSharedKeyAuthenticator) {
  fn connect(self, sigthis: QNetworkReply_preSharedKeyAuthenticationRequired_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply34preSharedKeyAuthenticationRequiredEP29QSslPreSharedKeyAuthenticator(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect for Box<Fn(QSslPreSharedKeyAuthenticator)> {
  fn connect(self, sigthis: QNetworkReply_preSharedKeyAuthenticationRequired_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkReply_preSharedKeyAuthenticationRequired_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkReply_SlotProxy_connect__ZN13QNetworkReply34preSharedKeyAuthenticationRequiredEP29QSslPreSharedKeyAuthenticator(arg0, arg1, arg2)};
  }
}
// <= body block end

