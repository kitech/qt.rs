// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qsslsocket.h
// dst-file: /src/network/qsslsocket.rs
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
use super::qtcpsocket::QTcpSocket; // 773
use std::ops::Deref;
use super::qsslcertificate::QSslCertificate; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qvariant::QVariant; // 771
use super::qsslcipher::QSslCipher; // 773
use super::qsslconfiguration::QSslConfiguration; // 773
use super::qsslkey::QSslKey; // 773
use super::super::core::qobject::QObject; // 771
use super::qsslerror::QSslError; // 773
use super::qsslpresharedkeyauthenticator::QSslPreSharedKeyAuthenticator; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslSocket_Class_Size() -> c_int;
  // proto: static QList<QSslCipher> QSslSocket::defaultCiphers();
  fn _ZN10QSslSocket14defaultCiphersEv();
  // proto:  QList<QSslCipher> QSslSocket::ciphers();
  fn _ZNK10QSslSocket7ciphersEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslSocket::ignoreSslErrors();
  fn _ZN10QSslSocket15ignoreSslErrorsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslSocket::close();
  fn _ZN10QSslSocket5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSslCertificate QSslSocket::peerCertificate();
  fn _ZNK10QSslSocket15peerCertificateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslSocket::QSslSocket(const QSslSocket & );
  fn _ZN10QSslSocketC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslSocket::startClientEncryption();
  fn _ZN10QSslSocket21startClientEncryptionEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSslSocket::isEncrypted();
  fn _ZNK10QSslSocket11isEncryptedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QSslSocket::bytesAvailable();
  fn _ZNK10QSslSocket14bytesAvailableEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  qint64 QSslSocket::encryptedBytesToWrite();
  fn _ZNK10QSslSocket21encryptedBytesToWriteEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QSslCipher QSslSocket::sessionCipher();
  fn _ZNK10QSslSocket13sessionCipherEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslSocket::setReadBufferSize(qint64 size);
  fn _ZN10QSslSocket17setReadBufferSizeEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  QString QSslSocket::peerVerifyName();
  fn _ZNK10QSslSocket14peerVerifyNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QSslSocket::peerVerifyDepth();
  fn _ZNK10QSslSocket15peerVerifyDepthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QList<QSslCertificate> QSslSocket::systemCaCertificates();
  fn _ZN10QSslSocket20systemCaCertificatesEv();
  // proto:  QSslCertificate QSslSocket::localCertificate();
  fn _ZNK10QSslSocket16localCertificateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSslSocket::canReadLine();
  fn _ZNK10QSslSocket11canReadLineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSslSocket::waitForBytesWritten(int msecs);
  fn _ZN10QSslSocket19waitForBytesWrittenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QSslSocket::setSslConfiguration(const QSslConfiguration & config);
  fn _ZN10QSslSocket19setSslConfigurationERK17QSslConfiguration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslSocket::disconnectFromHost();
  fn _ZN10QSslSocket18disconnectFromHostEv(qthis: u64 /* *mut c_void*/);
  // proto: static void QSslSocket::addDefaultCaCertificate(const QSslCertificate & certificate);
  fn _ZN10QSslSocket23addDefaultCaCertificateERK15QSslCertificate(arg0: *mut c_void);
  // proto:  qint64 QSslSocket::encryptedBytesAvailable();
  fn _ZNK10QSslSocket23encryptedBytesAvailableEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto: static long QSslSocket::sslLibraryVersionNumber();
  fn _ZN10QSslSocket23sslLibraryVersionNumberEv() -> c_long;
  // proto:  bool QSslSocket::waitForEncrypted(int msecs);
  fn _ZN10QSslSocket16waitForEncryptedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QSslSocket::startServerEncryption();
  fn _ZN10QSslSocket21startServerEncryptionEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslSocket::setLocalCertificate(const QSslCertificate & certificate);
  fn _ZN10QSslSocket19setLocalCertificateERK15QSslCertificate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QSslCertificate> QSslSocket::localCertificateChain();
  fn _ZNK10QSslSocket21localCertificateChainEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSslSocket::waitForReadyRead(int msecs);
  fn _ZN10QSslSocket16waitForReadyReadEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto: static QList<QSslCipher> QSslSocket::supportedCiphers();
  fn _ZN10QSslSocket16supportedCiphersEv();
  // proto:  void QSslSocket::addCaCertificate(const QSslCertificate & certificate);
  fn _ZN10QSslSocket16addCaCertificateERK15QSslCertificate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslSocket::setPeerVerifyName(const QString & hostName);
  fn _ZN10QSslSocket17setPeerVerifyNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QSslCertificate> QSslSocket::peerCertificateChain();
  fn _ZNK10QSslSocket20peerCertificateChainEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslSocket::resume();
  fn _ZN10QSslSocket6resumeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslSocket::abort();
  fn _ZN10QSslSocket5abortEv(qthis: u64 /* *mut c_void*/);
  // proto: static long QSslSocket::sslLibraryBuildVersionNumber();
  fn _ZN10QSslSocket28sslLibraryBuildVersionNumberEv() -> c_long;
  // proto: static QString QSslSocket::sslLibraryBuildVersionString();
  fn _ZN10QSslSocket28sslLibraryBuildVersionStringEv() -> *mut c_void;
  // proto:  void QSslSocket::setCiphers(const QString & ciphers);
  fn _ZN10QSslSocket10setCiphersERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSslSocket::flush();
  fn _ZN10QSslSocket5flushEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QSslSocket::bytesToWrite();
  fn _ZNK10QSslSocket12bytesToWriteEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QSslSocket::setPrivateKey(const QSslKey & key);
  fn _ZN10QSslSocket13setPrivateKeyERK7QSslKey(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSslSocket::waitForConnected(int msecs);
  fn _ZN10QSslSocket16waitForConnectedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  bool QSslSocket::atEnd();
  fn _ZNK10QSslSocket5atEndEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSslSocket::waitForDisconnected(int msecs);
  fn _ZN10QSslSocket19waitForDisconnectedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto: static bool QSslSocket::supportsSsl();
  fn _ZN10QSslSocket11supportsSslEv() -> c_char;
  // proto:  QSslConfiguration QSslSocket::sslConfiguration();
  fn _ZNK10QSslSocket16sslConfigurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QSslCertificate> QSslSocket::caCertificates();
  fn _ZNK10QSslSocket14caCertificatesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslSocket::QSslSocket(QObject * parent);
  fn _ZN10QSslSocketC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslSocket::setPeerVerifyDepth(int depth);
  fn _ZN10QSslSocket18setPeerVerifyDepthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto: static QString QSslSocket::sslLibraryVersionString();
  fn _ZN10QSslSocket23sslLibraryVersionStringEv() -> *mut c_void;
  // proto:  const QMetaObject * QSslSocket::metaObject();
  fn _ZNK10QSslSocket10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto: static QList<QSslCertificate> QSslSocket::defaultCaCertificates();
  fn _ZN10QSslSocket21defaultCaCertificatesEv();
  // proto:  void QSslSocket::~QSslSocket();
  fn _ZN10QSslSocketD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSslKey QSslSocket::privateKey();
  fn _ZNK10QSslSocket10privateKeyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QSslError> QSslSocket::sslErrors();
  fn _ZNK10QSslSocket9sslErrorsEv(qthis: u64 /* *mut c_void*/);
  fn QSslSocket_SlotProxy_connect__ZN10QSslSocket15peerVerifyErrorERK9QSslError(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSslSocket_SlotProxy_connect__ZN10QSslSocket11modeChangedENS_7SslModeE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSslSocket_SlotProxy_connect__ZN10QSslSocket9encryptedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSslSocket_SlotProxy_connect__ZN10QSslSocket34preSharedKeyAuthenticationRequiredEP29QSslPreSharedKeyAuthenticator(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSslSocket_SlotProxy_connect__ZN10QSslSocket21encryptedBytesWrittenEx(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSslSocket)=1
#[derive(Default)]
pub struct QSslSocket {
  qbase: QTcpSocket,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _preSharedKeyAuthenticationRequired: QSslSocket_preSharedKeyAuthenticationRequired_signal,
  pub _modeChanged: QSslSocket_modeChanged_signal,
  pub _encrypted: QSslSocket_encrypted_signal,
  pub _sslErrors: QSslSocket_sslErrors_signal,
  pub _peerVerifyError: QSslSocket_peerVerifyError_signal,
  pub _encryptedBytesWritten: QSslSocket_encryptedBytesWritten_signal,
}

impl /*struct*/ QSslSocket {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslSocket {
    return QSslSocket{qbase: QTcpSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSslSocket {
  type Target = QTcpSocket;

  fn deref(&self) -> &QTcpSocket {
    return & self.qbase;
  }
}
impl AsRef<QTcpSocket> for QSslSocket {
  fn as_ref(& self) -> & QTcpSocket {
    return & self.qbase;
  }
}
  // proto: static QList<QSslCipher> QSslSocket::defaultCiphers();
impl /*struct*/ QSslSocket {
  pub fn defaultCiphers_s<RetType, T: QSslSocket_defaultCiphers_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultCiphers_s();
    // return 1;
  }
}

pub trait QSslSocket_defaultCiphers_s<RetType> {
  fn defaultCiphers_s(self ) -> RetType;
}

  // proto: static QList<QSslCipher> QSslSocket::defaultCiphers();
impl<'a> /*trait*/ QSslSocket_defaultCiphers_s<()> for () {
  fn defaultCiphers_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket14defaultCiphersEv()};
     unsafe {_ZN10QSslSocket14defaultCiphersEv()};
    // return 1;
  }
}

  // proto:  QList<QSslCipher> QSslSocket::ciphers();
impl /*struct*/ QSslSocket {
  pub fn ciphers<RetType, T: QSslSocket_ciphers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ciphers(self);
    // return 1;
  }
}

pub trait QSslSocket_ciphers<RetType> {
  fn ciphers(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QList<QSslCipher> QSslSocket::ciphers();
impl<'a> /*trait*/ QSslSocket_ciphers<()> for () {
  fn ciphers(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket7ciphersEv()};
     unsafe {_ZNK10QSslSocket7ciphersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslSocket::ignoreSslErrors();
impl /*struct*/ QSslSocket {
  pub fn ignoreSslErrors<RetType, T: QSslSocket_ignoreSslErrors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignoreSslErrors(self);
    // return 1;
  }
}

pub trait QSslSocket_ignoreSslErrors<RetType> {
  fn ignoreSslErrors(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::ignoreSslErrors();
impl<'a> /*trait*/ QSslSocket_ignoreSslErrors<()> for () {
  fn ignoreSslErrors(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket15ignoreSslErrorsEv()};
     unsafe {_ZN10QSslSocket15ignoreSslErrorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslSocket::close();
impl /*struct*/ QSslSocket {
  pub fn close<RetType, T: QSslSocket_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QSslSocket_close<RetType> {
  fn close(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::close();
impl<'a> /*trait*/ QSslSocket_close<()> for () {
  fn close(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket5closeEv()};
     unsafe {_ZN10QSslSocket5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSslCertificate QSslSocket::peerCertificate();
impl /*struct*/ QSslSocket {
  pub fn peerCertificate<RetType, T: QSslSocket_peerCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerCertificate(self);
    // return 1;
  }
}

pub trait QSslSocket_peerCertificate<RetType> {
  fn peerCertificate(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QSslCertificate QSslSocket::peerCertificate();
impl<'a> /*trait*/ QSslSocket_peerCertificate<QSslCertificate> for () {
  fn peerCertificate(self , rsthis: & QSslSocket) -> QSslCertificate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket15peerCertificateEv()};
    let mut ret = unsafe {_ZNK10QSslSocket15peerCertificateEv(rsthis.qclsinst)};
    let mut ret1 = QSslCertificate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslSocket::QSslSocket(const QSslSocket & );
impl /*struct*/ QSslSocket {
  pub fn new<T: QSslSocket_new>(value: T) -> QSslSocket {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslSocket_new {
  fn new(self) -> QSslSocket;
}

  // proto:  void QSslSocket::QSslSocket(const QSslSocket & );
impl<'a> /*trait*/ QSslSocket_new for (&'a QSslSocket) {
  fn new(self) -> QSslSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocketC2ERKS_()};
    let ctysz: c_int = unsafe{QSslSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QSslSocketC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslSocket{qbase: QTcpSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSslSocket::startClientEncryption();
impl /*struct*/ QSslSocket {
  pub fn startClientEncryption<RetType, T: QSslSocket_startClientEncryption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startClientEncryption(self);
    // return 1;
  }
}

pub trait QSslSocket_startClientEncryption<RetType> {
  fn startClientEncryption(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::startClientEncryption();
impl<'a> /*trait*/ QSslSocket_startClientEncryption<()> for () {
  fn startClientEncryption(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket21startClientEncryptionEv()};
     unsafe {_ZN10QSslSocket21startClientEncryptionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSslSocket::isEncrypted();
impl /*struct*/ QSslSocket {
  pub fn isEncrypted<RetType, T: QSslSocket_isEncrypted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEncrypted(self);
    // return 1;
  }
}

pub trait QSslSocket_isEncrypted<RetType> {
  fn isEncrypted(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::isEncrypted();
impl<'a> /*trait*/ QSslSocket_isEncrypted<i8> for () {
  fn isEncrypted(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket11isEncryptedEv()};
    let mut ret = unsafe {_ZNK10QSslSocket11isEncryptedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QSslSocket::bytesAvailable();
impl /*struct*/ QSslSocket {
  pub fn bytesAvailable<RetType, T: QSslSocket_bytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable(self);
    // return 1;
  }
}

pub trait QSslSocket_bytesAvailable<RetType> {
  fn bytesAvailable(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  qint64 QSslSocket::bytesAvailable();
impl<'a> /*trait*/ QSslSocket_bytesAvailable<i64> for () {
  fn bytesAvailable(self , rsthis: & QSslSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK10QSslSocket14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  qint64 QSslSocket::encryptedBytesToWrite();
impl /*struct*/ QSslSocket {
  pub fn encryptedBytesToWrite<RetType, T: QSslSocket_encryptedBytesToWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.encryptedBytesToWrite(self);
    // return 1;
  }
}

pub trait QSslSocket_encryptedBytesToWrite<RetType> {
  fn encryptedBytesToWrite(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  qint64 QSslSocket::encryptedBytesToWrite();
impl<'a> /*trait*/ QSslSocket_encryptedBytesToWrite<i64> for () {
  fn encryptedBytesToWrite(self , rsthis: & QSslSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket21encryptedBytesToWriteEv()};
    let mut ret = unsafe {_ZNK10QSslSocket21encryptedBytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QSslCipher QSslSocket::sessionCipher();
impl /*struct*/ QSslSocket {
  pub fn sessionCipher<RetType, T: QSslSocket_sessionCipher<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionCipher(self);
    // return 1;
  }
}

pub trait QSslSocket_sessionCipher<RetType> {
  fn sessionCipher(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QSslCipher QSslSocket::sessionCipher();
impl<'a> /*trait*/ QSslSocket_sessionCipher<QSslCipher> for () {
  fn sessionCipher(self , rsthis: & QSslSocket) -> QSslCipher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket13sessionCipherEv()};
    let mut ret = unsafe {_ZNK10QSslSocket13sessionCipherEv(rsthis.qclsinst)};
    let mut ret1 = QSslCipher::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslSocket::setReadBufferSize(qint64 size);
impl /*struct*/ QSslSocket {
  pub fn setReadBufferSize<RetType, T: QSslSocket_setReadBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadBufferSize(self);
    // return 1;
  }
}

pub trait QSslSocket_setReadBufferSize<RetType> {
  fn setReadBufferSize(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setReadBufferSize(qint64 size);
impl<'a> /*trait*/ QSslSocket_setReadBufferSize<()> for (i64) {
  fn setReadBufferSize(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket17setReadBufferSizeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN10QSslSocket17setReadBufferSizeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSslSocket::peerVerifyName();
impl /*struct*/ QSslSocket {
  pub fn peerVerifyName<RetType, T: QSslSocket_peerVerifyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerVerifyName(self);
    // return 1;
  }
}

pub trait QSslSocket_peerVerifyName<RetType> {
  fn peerVerifyName(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QString QSslSocket::peerVerifyName();
impl<'a> /*trait*/ QSslSocket_peerVerifyName<QString> for () {
  fn peerVerifyName(self , rsthis: & QSslSocket) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket14peerVerifyNameEv()};
    let mut ret = unsafe {_ZNK10QSslSocket14peerVerifyNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSslSocket::peerVerifyDepth();
impl /*struct*/ QSslSocket {
  pub fn peerVerifyDepth<RetType, T: QSslSocket_peerVerifyDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerVerifyDepth(self);
    // return 1;
  }
}

pub trait QSslSocket_peerVerifyDepth<RetType> {
  fn peerVerifyDepth(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  int QSslSocket::peerVerifyDepth();
impl<'a> /*trait*/ QSslSocket_peerVerifyDepth<i32> for () {
  fn peerVerifyDepth(self , rsthis: & QSslSocket) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket15peerVerifyDepthEv()};
    let mut ret = unsafe {_ZNK10QSslSocket15peerVerifyDepthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QList<QSslCertificate> QSslSocket::systemCaCertificates();
impl /*struct*/ QSslSocket {
  pub fn systemCaCertificates_s<RetType, T: QSslSocket_systemCaCertificates_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemCaCertificates_s();
    // return 1;
  }
}

pub trait QSslSocket_systemCaCertificates_s<RetType> {
  fn systemCaCertificates_s(self ) -> RetType;
}

  // proto: static QList<QSslCertificate> QSslSocket::systemCaCertificates();
impl<'a> /*trait*/ QSslSocket_systemCaCertificates_s<()> for () {
  fn systemCaCertificates_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket20systemCaCertificatesEv()};
     unsafe {_ZN10QSslSocket20systemCaCertificatesEv()};
    // return 1;
  }
}

  // proto:  QSslCertificate QSslSocket::localCertificate();
impl /*struct*/ QSslSocket {
  pub fn localCertificate<RetType, T: QSslSocket_localCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localCertificate(self);
    // return 1;
  }
}

pub trait QSslSocket_localCertificate<RetType> {
  fn localCertificate(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QSslCertificate QSslSocket::localCertificate();
impl<'a> /*trait*/ QSslSocket_localCertificate<QSslCertificate> for () {
  fn localCertificate(self , rsthis: & QSslSocket) -> QSslCertificate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket16localCertificateEv()};
    let mut ret = unsafe {_ZNK10QSslSocket16localCertificateEv(rsthis.qclsinst)};
    let mut ret1 = QSslCertificate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSslSocket::canReadLine();
impl /*struct*/ QSslSocket {
  pub fn canReadLine<RetType, T: QSslSocket_canReadLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canReadLine(self);
    // return 1;
  }
}

pub trait QSslSocket_canReadLine<RetType> {
  fn canReadLine(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::canReadLine();
impl<'a> /*trait*/ QSslSocket_canReadLine<i8> for () {
  fn canReadLine(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket11canReadLineEv()};
    let mut ret = unsafe {_ZNK10QSslSocket11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSslSocket::waitForBytesWritten(int msecs);
impl /*struct*/ QSslSocket {
  pub fn waitForBytesWritten<RetType, T: QSslSocket_waitForBytesWritten<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QSslSocket_waitForBytesWritten<RetType> {
  fn waitForBytesWritten(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QSslSocket_waitForBytesWritten<i8> for (i32) {
  fn waitForBytesWritten(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QSslSocket19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSslSocket::setSslConfiguration(const QSslConfiguration & config);
impl /*struct*/ QSslSocket {
  pub fn setSslConfiguration<RetType, T: QSslSocket_setSslConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSslConfiguration(self);
    // return 1;
  }
}

pub trait QSslSocket_setSslConfiguration<RetType> {
  fn setSslConfiguration(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setSslConfiguration(const QSslConfiguration & config);
impl<'a> /*trait*/ QSslSocket_setSslConfiguration<()> for (&'a QSslConfiguration) {
  fn setSslConfiguration(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket19setSslConfigurationERK17QSslConfiguration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket19setSslConfigurationERK17QSslConfiguration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslSocket::disconnectFromHost();
impl /*struct*/ QSslSocket {
  pub fn disconnectFromHost<RetType, T: QSslSocket_disconnectFromHost<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.disconnectFromHost(self);
    // return 1;
  }
}

pub trait QSslSocket_disconnectFromHost<RetType> {
  fn disconnectFromHost(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::disconnectFromHost();
impl<'a> /*trait*/ QSslSocket_disconnectFromHost<()> for () {
  fn disconnectFromHost(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket18disconnectFromHostEv()};
     unsafe {_ZN10QSslSocket18disconnectFromHostEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QSslSocket::addDefaultCaCertificate(const QSslCertificate & certificate);
impl /*struct*/ QSslSocket {
  pub fn addDefaultCaCertificate_s<RetType, T: QSslSocket_addDefaultCaCertificate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addDefaultCaCertificate_s();
    // return 1;
  }
}

pub trait QSslSocket_addDefaultCaCertificate_s<RetType> {
  fn addDefaultCaCertificate_s(self ) -> RetType;
}

  // proto: static void QSslSocket::addDefaultCaCertificate(const QSslCertificate & certificate);
impl<'a> /*trait*/ QSslSocket_addDefaultCaCertificate_s<()> for (&'a QSslCertificate) {
  fn addDefaultCaCertificate_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket23addDefaultCaCertificateERK15QSslCertificate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket23addDefaultCaCertificateERK15QSslCertificate(arg0)};
    // return 1;
  }
}

  // proto:  qint64 QSslSocket::encryptedBytesAvailable();
impl /*struct*/ QSslSocket {
  pub fn encryptedBytesAvailable<RetType, T: QSslSocket_encryptedBytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.encryptedBytesAvailable(self);
    // return 1;
  }
}

pub trait QSslSocket_encryptedBytesAvailable<RetType> {
  fn encryptedBytesAvailable(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  qint64 QSslSocket::encryptedBytesAvailable();
impl<'a> /*trait*/ QSslSocket_encryptedBytesAvailable<i64> for () {
  fn encryptedBytesAvailable(self , rsthis: & QSslSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket23encryptedBytesAvailableEv()};
    let mut ret = unsafe {_ZNK10QSslSocket23encryptedBytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto: static long QSslSocket::sslLibraryVersionNumber();
impl /*struct*/ QSslSocket {
  pub fn sslLibraryVersionNumber_s<RetType, T: QSslSocket_sslLibraryVersionNumber_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sslLibraryVersionNumber_s();
    // return 1;
  }
}

pub trait QSslSocket_sslLibraryVersionNumber_s<RetType> {
  fn sslLibraryVersionNumber_s(self ) -> RetType;
}

  // proto: static long QSslSocket::sslLibraryVersionNumber();
impl<'a> /*trait*/ QSslSocket_sslLibraryVersionNumber_s<i64> for () {
  fn sslLibraryVersionNumber_s(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket23sslLibraryVersionNumberEv()};
    let mut ret = unsafe {_ZN10QSslSocket23sslLibraryVersionNumberEv()};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QSslSocket::waitForEncrypted(int msecs);
impl /*struct*/ QSslSocket {
  pub fn waitForEncrypted<RetType, T: QSslSocket_waitForEncrypted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForEncrypted(self);
    // return 1;
  }
}

pub trait QSslSocket_waitForEncrypted<RetType> {
  fn waitForEncrypted(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::waitForEncrypted(int msecs);
impl<'a> /*trait*/ QSslSocket_waitForEncrypted<i8> for (i32) {
  fn waitForEncrypted(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket16waitForEncryptedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QSslSocket16waitForEncryptedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSslSocket::startServerEncryption();
impl /*struct*/ QSslSocket {
  pub fn startServerEncryption<RetType, T: QSslSocket_startServerEncryption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startServerEncryption(self);
    // return 1;
  }
}

pub trait QSslSocket_startServerEncryption<RetType> {
  fn startServerEncryption(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::startServerEncryption();
impl<'a> /*trait*/ QSslSocket_startServerEncryption<()> for () {
  fn startServerEncryption(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket21startServerEncryptionEv()};
     unsafe {_ZN10QSslSocket21startServerEncryptionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslSocket::setLocalCertificate(const QSslCertificate & certificate);
impl /*struct*/ QSslSocket {
  pub fn setLocalCertificate<RetType, T: QSslSocket_setLocalCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocalCertificate(self);
    // return 1;
  }
}

pub trait QSslSocket_setLocalCertificate<RetType> {
  fn setLocalCertificate(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setLocalCertificate(const QSslCertificate & certificate);
impl<'a> /*trait*/ QSslSocket_setLocalCertificate<()> for (&'a QSslCertificate) {
  fn setLocalCertificate(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket19setLocalCertificateERK15QSslCertificate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket19setLocalCertificateERK15QSslCertificate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QSslCertificate> QSslSocket::localCertificateChain();
impl /*struct*/ QSslSocket {
  pub fn localCertificateChain<RetType, T: QSslSocket_localCertificateChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localCertificateChain(self);
    // return 1;
  }
}

pub trait QSslSocket_localCertificateChain<RetType> {
  fn localCertificateChain(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QList<QSslCertificate> QSslSocket::localCertificateChain();
impl<'a> /*trait*/ QSslSocket_localCertificateChain<()> for () {
  fn localCertificateChain(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket21localCertificateChainEv()};
     unsafe {_ZNK10QSslSocket21localCertificateChainEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSslSocket::waitForReadyRead(int msecs);
impl /*struct*/ QSslSocket {
  pub fn waitForReadyRead<RetType, T: QSslSocket_waitForReadyRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QSslSocket_waitForReadyRead<RetType> {
  fn waitForReadyRead(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QSslSocket_waitForReadyRead<i8> for (i32) {
  fn waitForReadyRead(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QSslSocket16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QList<QSslCipher> QSslSocket::supportedCiphers();
impl /*struct*/ QSslSocket {
  pub fn supportedCiphers_s<RetType, T: QSslSocket_supportedCiphers_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedCiphers_s();
    // return 1;
  }
}

pub trait QSslSocket_supportedCiphers_s<RetType> {
  fn supportedCiphers_s(self ) -> RetType;
}

  // proto: static QList<QSslCipher> QSslSocket::supportedCiphers();
impl<'a> /*trait*/ QSslSocket_supportedCiphers_s<()> for () {
  fn supportedCiphers_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket16supportedCiphersEv()};
     unsafe {_ZN10QSslSocket16supportedCiphersEv()};
    // return 1;
  }
}

  // proto:  void QSslSocket::addCaCertificate(const QSslCertificate & certificate);
impl /*struct*/ QSslSocket {
  pub fn addCaCertificate<RetType, T: QSslSocket_addCaCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addCaCertificate(self);
    // return 1;
  }
}

pub trait QSslSocket_addCaCertificate<RetType> {
  fn addCaCertificate(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::addCaCertificate(const QSslCertificate & certificate);
impl<'a> /*trait*/ QSslSocket_addCaCertificate<()> for (&'a QSslCertificate) {
  fn addCaCertificate(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket16addCaCertificateERK15QSslCertificate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket16addCaCertificateERK15QSslCertificate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslSocket::setPeerVerifyName(const QString & hostName);
impl /*struct*/ QSslSocket {
  pub fn setPeerVerifyName<RetType, T: QSslSocket_setPeerVerifyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPeerVerifyName(self);
    // return 1;
  }
}

pub trait QSslSocket_setPeerVerifyName<RetType> {
  fn setPeerVerifyName(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setPeerVerifyName(const QString & hostName);
impl<'a> /*trait*/ QSslSocket_setPeerVerifyName<()> for (&'a QString) {
  fn setPeerVerifyName(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket17setPeerVerifyNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket17setPeerVerifyNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QSslCertificate> QSslSocket::peerCertificateChain();
impl /*struct*/ QSslSocket {
  pub fn peerCertificateChain<RetType, T: QSslSocket_peerCertificateChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerCertificateChain(self);
    // return 1;
  }
}

pub trait QSslSocket_peerCertificateChain<RetType> {
  fn peerCertificateChain(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QList<QSslCertificate> QSslSocket::peerCertificateChain();
impl<'a> /*trait*/ QSslSocket_peerCertificateChain<()> for () {
  fn peerCertificateChain(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket20peerCertificateChainEv()};
     unsafe {_ZNK10QSslSocket20peerCertificateChainEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslSocket::resume();
impl /*struct*/ QSslSocket {
  pub fn resume<RetType, T: QSslSocket_resume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resume(self);
    // return 1;
  }
}

pub trait QSslSocket_resume<RetType> {
  fn resume(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::resume();
impl<'a> /*trait*/ QSslSocket_resume<()> for () {
  fn resume(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket6resumeEv()};
     unsafe {_ZN10QSslSocket6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslSocket::abort();
impl /*struct*/ QSslSocket {
  pub fn abort<RetType, T: QSslSocket_abort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.abort(self);
    // return 1;
  }
}

pub trait QSslSocket_abort<RetType> {
  fn abort(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::abort();
impl<'a> /*trait*/ QSslSocket_abort<()> for () {
  fn abort(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket5abortEv()};
     unsafe {_ZN10QSslSocket5abortEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static long QSslSocket::sslLibraryBuildVersionNumber();
impl /*struct*/ QSslSocket {
  pub fn sslLibraryBuildVersionNumber_s<RetType, T: QSslSocket_sslLibraryBuildVersionNumber_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sslLibraryBuildVersionNumber_s();
    // return 1;
  }
}

pub trait QSslSocket_sslLibraryBuildVersionNumber_s<RetType> {
  fn sslLibraryBuildVersionNumber_s(self ) -> RetType;
}

  // proto: static long QSslSocket::sslLibraryBuildVersionNumber();
impl<'a> /*trait*/ QSslSocket_sslLibraryBuildVersionNumber_s<i64> for () {
  fn sslLibraryBuildVersionNumber_s(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket28sslLibraryBuildVersionNumberEv()};
    let mut ret = unsafe {_ZN10QSslSocket28sslLibraryBuildVersionNumberEv()};
    return ret as i64;
    // return 1;
  }
}

  // proto: static QString QSslSocket::sslLibraryBuildVersionString();
impl /*struct*/ QSslSocket {
  pub fn sslLibraryBuildVersionString_s<RetType, T: QSslSocket_sslLibraryBuildVersionString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sslLibraryBuildVersionString_s();
    // return 1;
  }
}

pub trait QSslSocket_sslLibraryBuildVersionString_s<RetType> {
  fn sslLibraryBuildVersionString_s(self ) -> RetType;
}

  // proto: static QString QSslSocket::sslLibraryBuildVersionString();
impl<'a> /*trait*/ QSslSocket_sslLibraryBuildVersionString_s<QString> for () {
  fn sslLibraryBuildVersionString_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket28sslLibraryBuildVersionStringEv()};
    let mut ret = unsafe {_ZN10QSslSocket28sslLibraryBuildVersionStringEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslSocket::setCiphers(const QString & ciphers);
impl /*struct*/ QSslSocket {
  pub fn setCiphers<RetType, T: QSslSocket_setCiphers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCiphers(self);
    // return 1;
  }
}

pub trait QSslSocket_setCiphers<RetType> {
  fn setCiphers(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setCiphers(const QString & ciphers);
impl<'a> /*trait*/ QSslSocket_setCiphers<()> for (&'a QString) {
  fn setCiphers(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket10setCiphersERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket10setCiphersERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSslSocket::flush();
impl /*struct*/ QSslSocket {
  pub fn flush<RetType, T: QSslSocket_flush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QSslSocket_flush<RetType> {
  fn flush(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::flush();
impl<'a> /*trait*/ QSslSocket_flush<i8> for () {
  fn flush(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket5flushEv()};
    let mut ret = unsafe {_ZN10QSslSocket5flushEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QSslSocket::bytesToWrite();
impl /*struct*/ QSslSocket {
  pub fn bytesToWrite<RetType, T: QSslSocket_bytesToWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite(self);
    // return 1;
  }
}

pub trait QSslSocket_bytesToWrite<RetType> {
  fn bytesToWrite(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  qint64 QSslSocket::bytesToWrite();
impl<'a> /*trait*/ QSslSocket_bytesToWrite<i64> for () {
  fn bytesToWrite(self , rsthis: & QSslSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK10QSslSocket12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QSslSocket::setPrivateKey(const QSslKey & key);
impl /*struct*/ QSslSocket {
  pub fn setPrivateKey<RetType, T: QSslSocket_setPrivateKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPrivateKey(self);
    // return 1;
  }
}

pub trait QSslSocket_setPrivateKey<RetType> {
  fn setPrivateKey(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setPrivateKey(const QSslKey & key);
impl<'a> /*trait*/ QSslSocket_setPrivateKey<()> for (&'a QSslKey) {
  fn setPrivateKey(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket13setPrivateKeyERK7QSslKey()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslSocket13setPrivateKeyERK7QSslKey(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSslSocket::waitForConnected(int msecs);
impl /*struct*/ QSslSocket {
  pub fn waitForConnected<RetType, T: QSslSocket_waitForConnected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForConnected(self);
    // return 1;
  }
}

pub trait QSslSocket_waitForConnected<RetType> {
  fn waitForConnected(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::waitForConnected(int msecs);
impl<'a> /*trait*/ QSslSocket_waitForConnected<i8> for (i32) {
  fn waitForConnected(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket16waitForConnectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QSslSocket16waitForConnectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSslSocket::atEnd();
impl /*struct*/ QSslSocket {
  pub fn atEnd<RetType, T: QSslSocket_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QSslSocket_atEnd<RetType> {
  fn atEnd(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::atEnd();
impl<'a> /*trait*/ QSslSocket_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket5atEndEv()};
    let mut ret = unsafe {_ZNK10QSslSocket5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSslSocket::waitForDisconnected(int msecs);
impl /*struct*/ QSslSocket {
  pub fn waitForDisconnected<RetType, T: QSslSocket_waitForDisconnected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForDisconnected(self);
    // return 1;
  }
}

pub trait QSslSocket_waitForDisconnected<RetType> {
  fn waitForDisconnected(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  bool QSslSocket::waitForDisconnected(int msecs);
impl<'a> /*trait*/ QSslSocket_waitForDisconnected<i8> for (i32) {
  fn waitForDisconnected(self , rsthis: & QSslSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket19waitForDisconnectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QSslSocket19waitForDisconnectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QSslSocket::supportsSsl();
impl /*struct*/ QSslSocket {
  pub fn supportsSsl_s<RetType, T: QSslSocket_supportsSsl_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportsSsl_s();
    // return 1;
  }
}

pub trait QSslSocket_supportsSsl_s<RetType> {
  fn supportsSsl_s(self ) -> RetType;
}

  // proto: static bool QSslSocket::supportsSsl();
impl<'a> /*trait*/ QSslSocket_supportsSsl_s<i8> for () {
  fn supportsSsl_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket11supportsSslEv()};
    let mut ret = unsafe {_ZN10QSslSocket11supportsSslEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSslConfiguration QSslSocket::sslConfiguration();
impl /*struct*/ QSslSocket {
  pub fn sslConfiguration<RetType, T: QSslSocket_sslConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sslConfiguration(self);
    // return 1;
  }
}

pub trait QSslSocket_sslConfiguration<RetType> {
  fn sslConfiguration(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QSslConfiguration QSslSocket::sslConfiguration();
impl<'a> /*trait*/ QSslSocket_sslConfiguration<QSslConfiguration> for () {
  fn sslConfiguration(self , rsthis: & QSslSocket) -> QSslConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket16sslConfigurationEv()};
    let mut ret = unsafe {_ZNK10QSslSocket16sslConfigurationEv(rsthis.qclsinst)};
    let mut ret1 = QSslConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QSslCertificate> QSslSocket::caCertificates();
impl /*struct*/ QSslSocket {
  pub fn caCertificates<RetType, T: QSslSocket_caCertificates<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.caCertificates(self);
    // return 1;
  }
}

pub trait QSslSocket_caCertificates<RetType> {
  fn caCertificates(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QList<QSslCertificate> QSslSocket::caCertificates();
impl<'a> /*trait*/ QSslSocket_caCertificates<()> for () {
  fn caCertificates(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket14caCertificatesEv()};
     unsafe {_ZNK10QSslSocket14caCertificatesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslSocket::QSslSocket(QObject * parent);
impl<'a> /*trait*/ QSslSocket_new for (&'a QObject) {
  fn new(self) -> QSslSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocketC2EP7QObject()};
    let ctysz: c_int = unsafe{QSslSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QSslSocketC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslSocket{qbase: QTcpSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSslSocket::setPeerVerifyDepth(int depth);
impl /*struct*/ QSslSocket {
  pub fn setPeerVerifyDepth<RetType, T: QSslSocket_setPeerVerifyDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPeerVerifyDepth(self);
    // return 1;
  }
}

pub trait QSslSocket_setPeerVerifyDepth<RetType> {
  fn setPeerVerifyDepth(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::setPeerVerifyDepth(int depth);
impl<'a> /*trait*/ QSslSocket_setPeerVerifyDepth<()> for (i32) {
  fn setPeerVerifyDepth(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket18setPeerVerifyDepthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QSslSocket18setPeerVerifyDepthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QString QSslSocket::sslLibraryVersionString();
impl /*struct*/ QSslSocket {
  pub fn sslLibraryVersionString_s<RetType, T: QSslSocket_sslLibraryVersionString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sslLibraryVersionString_s();
    // return 1;
  }
}

pub trait QSslSocket_sslLibraryVersionString_s<RetType> {
  fn sslLibraryVersionString_s(self ) -> RetType;
}

  // proto: static QString QSslSocket::sslLibraryVersionString();
impl<'a> /*trait*/ QSslSocket_sslLibraryVersionString_s<QString> for () {
  fn sslLibraryVersionString_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket23sslLibraryVersionStringEv()};
    let mut ret = unsafe {_ZN10QSslSocket23sslLibraryVersionStringEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSslSocket::metaObject();
impl /*struct*/ QSslSocket {
  pub fn metaObject<RetType, T: QSslSocket_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSslSocket_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  const QMetaObject * QSslSocket::metaObject();
impl<'a> /*trait*/ QSslSocket_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket10metaObjectEv()};
     unsafe {_ZNK10QSslSocket10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QList<QSslCertificate> QSslSocket::defaultCaCertificates();
impl /*struct*/ QSslSocket {
  pub fn defaultCaCertificates_s<RetType, T: QSslSocket_defaultCaCertificates_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultCaCertificates_s();
    // return 1;
  }
}

pub trait QSslSocket_defaultCaCertificates_s<RetType> {
  fn defaultCaCertificates_s(self ) -> RetType;
}

  // proto: static QList<QSslCertificate> QSslSocket::defaultCaCertificates();
impl<'a> /*trait*/ QSslSocket_defaultCaCertificates_s<()> for () {
  fn defaultCaCertificates_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocket21defaultCaCertificatesEv()};
     unsafe {_ZN10QSslSocket21defaultCaCertificatesEv()};
    // return 1;
  }
}

  // proto:  void QSslSocket::~QSslSocket();
impl /*struct*/ QSslSocket {
  pub fn free<RetType, T: QSslSocket_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslSocket_free<RetType> {
  fn free(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  void QSslSocket::~QSslSocket();
impl<'a> /*trait*/ QSslSocket_free<()> for () {
  fn free(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslSocketD2Ev()};
     unsafe {_ZN10QSslSocketD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSslKey QSslSocket::privateKey();
impl /*struct*/ QSslSocket {
  pub fn privateKey<RetType, T: QSslSocket_privateKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.privateKey(self);
    // return 1;
  }
}

pub trait QSslSocket_privateKey<RetType> {
  fn privateKey(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QSslKey QSslSocket::privateKey();
impl<'a> /*trait*/ QSslSocket_privateKey<QSslKey> for () {
  fn privateKey(self , rsthis: & QSslSocket) -> QSslKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket10privateKeyEv()};
    let mut ret = unsafe {_ZNK10QSslSocket10privateKeyEv(rsthis.qclsinst)};
    let mut ret1 = QSslKey::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QSslError> QSslSocket::sslErrors();
impl /*struct*/ QSslSocket {
  pub fn sslErrors<RetType, T: QSslSocket_sslErrors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sslErrors(self);
    // return 1;
  }
}

pub trait QSslSocket_sslErrors<RetType> {
  fn sslErrors(self , rsthis: & QSslSocket) -> RetType;
}

  // proto:  QList<QSslError> QSslSocket::sslErrors();
impl<'a> /*trait*/ QSslSocket_sslErrors<()> for () {
  fn sslErrors(self , rsthis: & QSslSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslSocket9sslErrorsEv()};
     unsafe {_ZNK10QSslSocket9sslErrorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QSslSocket_preSharedKeyAuthenticationRequired
pub struct QSslSocket_preSharedKeyAuthenticationRequired_signal{poi:u64}
impl /* struct */ QSslSocket {
  pub fn preSharedKeyAuthenticationRequired(&self) -> QSslSocket_preSharedKeyAuthenticationRequired_signal {
     return QSslSocket_preSharedKeyAuthenticationRequired_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSslSocket_preSharedKeyAuthenticationRequired_signal {
  pub fn connect<T: QSslSocket_preSharedKeyAuthenticationRequired_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSslSocket_preSharedKeyAuthenticationRequired_signal_connect {
  fn connect(self, sigthis: QSslSocket_preSharedKeyAuthenticationRequired_signal);
}

#[derive(Default)] // for QSslSocket_modeChanged
pub struct QSslSocket_modeChanged_signal{poi:u64}
impl /* struct */ QSslSocket {
  pub fn modeChanged(&self) -> QSslSocket_modeChanged_signal {
     return QSslSocket_modeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSslSocket_modeChanged_signal {
  pub fn connect<T: QSslSocket_modeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSslSocket_modeChanged_signal_connect {
  fn connect(self, sigthis: QSslSocket_modeChanged_signal);
}

#[derive(Default)] // for QSslSocket_encrypted
pub struct QSslSocket_encrypted_signal{poi:u64}
impl /* struct */ QSslSocket {
  pub fn encrypted(&self) -> QSslSocket_encrypted_signal {
     return QSslSocket_encrypted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSslSocket_encrypted_signal {
  pub fn connect<T: QSslSocket_encrypted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSslSocket_encrypted_signal_connect {
  fn connect(self, sigthis: QSslSocket_encrypted_signal);
}

#[derive(Default)] // for QSslSocket_sslErrors
pub struct QSslSocket_sslErrors_signal{poi:u64}
impl /* struct */ QSslSocket {
  pub fn sslErrors(&self) -> QSslSocket_sslErrors_signal {
     return QSslSocket_sslErrors_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSslSocket_sslErrors_signal {
  pub fn connect<T: QSslSocket_sslErrors_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSslSocket_sslErrors_signal_connect {
  fn connect(self, sigthis: QSslSocket_sslErrors_signal);
}

#[derive(Default)] // for QSslSocket_peerVerifyError
pub struct QSslSocket_peerVerifyError_signal{poi:u64}
impl /* struct */ QSslSocket {
  pub fn peerVerifyError(&self) -> QSslSocket_peerVerifyError_signal {
     return QSslSocket_peerVerifyError_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSslSocket_peerVerifyError_signal {
  pub fn connect<T: QSslSocket_peerVerifyError_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSslSocket_peerVerifyError_signal_connect {
  fn connect(self, sigthis: QSslSocket_peerVerifyError_signal);
}

#[derive(Default)] // for QSslSocket_encryptedBytesWritten
pub struct QSslSocket_encryptedBytesWritten_signal{poi:u64}
impl /* struct */ QSslSocket {
  pub fn encryptedBytesWritten(&self) -> QSslSocket_encryptedBytesWritten_signal {
     return QSslSocket_encryptedBytesWritten_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSslSocket_encryptedBytesWritten_signal {
  pub fn connect<T: QSslSocket_encryptedBytesWritten_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSslSocket_encryptedBytesWritten_signal_connect {
  fn connect(self, sigthis: QSslSocket_encryptedBytesWritten_signal);
}

// peerVerifyError(const class QSslError &)
extern fn QSslSocket_peerVerifyError_signal_connect_cb_0(rsfptr:fn(QSslError), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QSslError::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QSslSocket_peerVerifyError_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QSslError)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QSslError::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSslSocket_peerVerifyError_signal_connect for fn(QSslError) {
  fn connect(self, sigthis: QSslSocket_peerVerifyError_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_peerVerifyError_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket15peerVerifyErrorERK9QSslError(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSslSocket_peerVerifyError_signal_connect for Box<Fn(QSslError)> {
  fn connect(self, sigthis: QSslSocket_peerVerifyError_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_peerVerifyError_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket15peerVerifyErrorERK9QSslError(arg0, arg1, arg2)};
  }
}
// modeChanged(class QSslSocket::SslMode)
extern fn QSslSocket_modeChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QSslSocket_modeChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSslSocket_modeChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QSslSocket_modeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_modeChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket11modeChangedENS_7SslModeE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSslSocket_modeChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QSslSocket_modeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_modeChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket11modeChangedENS_7SslModeE(arg0, arg1, arg2)};
  }
}
// encrypted()
extern fn QSslSocket_encrypted_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QSslSocket_encrypted_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QSslSocket_encrypted_signal_connect for fn() {
  fn connect(self, sigthis: QSslSocket_encrypted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_encrypted_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket9encryptedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSslSocket_encrypted_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QSslSocket_encrypted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_encrypted_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket9encryptedEv(arg0, arg1, arg2)};
  }
}
// preSharedKeyAuthenticationRequired(class QSslPreSharedKeyAuthenticator *)
extern fn QSslSocket_preSharedKeyAuthenticationRequired_signal_connect_cb_3(rsfptr:fn(QSslPreSharedKeyAuthenticator), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QSslPreSharedKeyAuthenticator::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QSslSocket_preSharedKeyAuthenticationRequired_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QSslPreSharedKeyAuthenticator)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QSslPreSharedKeyAuthenticator::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSslSocket_preSharedKeyAuthenticationRequired_signal_connect for fn(QSslPreSharedKeyAuthenticator) {
  fn connect(self, sigthis: QSslSocket_preSharedKeyAuthenticationRequired_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_preSharedKeyAuthenticationRequired_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket34preSharedKeyAuthenticationRequiredEP29QSslPreSharedKeyAuthenticator(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSslSocket_preSharedKeyAuthenticationRequired_signal_connect for Box<Fn(QSslPreSharedKeyAuthenticator)> {
  fn connect(self, sigthis: QSslSocket_preSharedKeyAuthenticationRequired_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_preSharedKeyAuthenticationRequired_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket34preSharedKeyAuthenticationRequiredEP29QSslPreSharedKeyAuthenticator(arg0, arg1, arg2)};
  }
}
// encryptedBytesWritten(qint64)
extern fn QSslSocket_encryptedBytesWritten_signal_connect_cb_4(rsfptr:fn(i64), arg0: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i64;
  rsfptr(rsarg0);
}
extern fn QSslSocket_encryptedBytesWritten_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i64)>, arg0: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSslSocket_encryptedBytesWritten_signal_connect for fn(i64) {
  fn connect(self, sigthis: QSslSocket_encryptedBytesWritten_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_encryptedBytesWritten_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket21encryptedBytesWrittenEx(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSslSocket_encryptedBytesWritten_signal_connect for Box<Fn(i64)> {
  fn connect(self, sigthis: QSslSocket_encryptedBytesWritten_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSslSocket_encryptedBytesWritten_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSslSocket_SlotProxy_connect__ZN10QSslSocket21encryptedBytesWrittenEx(arg0, arg1, arg2)};
  }
}
// <= body block end

