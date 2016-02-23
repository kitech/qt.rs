// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qsslconfiguration.h
// dst-file: /src/network/qsslconfiguration.rs
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
use super::super::core::qbytearray::QByteArray; // 771
use super::qsslkey::QSslKey; // 773
use super::qsslcertificate::QSslCertificate; // 773
use super::qsslcipher::QSslCipher; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslConfiguration_Class_Size() -> c_int;
  // proto:  QByteArray QSslConfiguration::nextNegotiatedProtocol();
  fn _ZNK17QSslConfiguration22nextNegotiatedProtocolEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslConfiguration::swap(QSslConfiguration & other);
  fn _ZN17QSslConfiguration4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QSslConfiguration::peerVerifyDepth();
  fn _ZNK17QSslConfiguration15peerVerifyDepthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QVector<QSslEllipticCurve> QSslConfiguration::supportedEllipticCurves();
  fn _ZN17QSslConfiguration23supportedEllipticCurvesEv();
  // proto:  QList<QByteArray> QSslConfiguration::allowedNextProtocols();
  fn _ZNK17QSslConfiguration20allowedNextProtocolsEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QSslConfiguration::sessionTicketLifeTimeHint();
  fn _ZNK17QSslConfiguration25sessionTicketLifeTimeHintEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QList<QSslCipher> QSslConfiguration::supportedCiphers();
  fn _ZN17QSslConfiguration16supportedCiphersEv();
  // proto:  void QSslConfiguration::setSessionTicket(const QByteArray & sessionTicket);
  fn _ZN17QSslConfiguration16setSessionTicketERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslConfiguration::QSslConfiguration(const QSslConfiguration & other);
  fn _ZN17QSslConfigurationC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QSslCertificate> QSslConfiguration::localCertificateChain();
  fn _ZNK17QSslConfiguration21localCertificateChainEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QSslCertificate> QSslConfiguration::peerCertificateChain();
  fn _ZNK17QSslConfiguration20peerCertificateChainEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslConfiguration::setPrivateKey(const QSslKey & key);
  fn _ZN17QSslConfiguration13setPrivateKeyERK7QSslKey(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslConfiguration::setLocalCertificate(const QSslCertificate & certificate);
  fn _ZN17QSslConfiguration19setLocalCertificateERK15QSslCertificate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslConfiguration::QSslConfiguration();
  fn _ZN17QSslConfigurationC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSslConfiguration::isNull();
  fn _ZNK17QSslConfiguration6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QList<QSslCertificate> QSslConfiguration::systemCaCertificates();
  fn _ZN17QSslConfiguration20systemCaCertificatesEv();
  // proto:  QSslCertificate QSslConfiguration::peerCertificate();
  fn _ZNK17QSslConfiguration15peerCertificateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslConfiguration::setPeerVerifyDepth(int depth);
  fn _ZN17QSslConfiguration18setPeerVerifyDepthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSslConfiguration::~QSslConfiguration();
  fn _ZN17QSslConfigurationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QSslCertificate> QSslConfiguration::caCertificates();
  fn _ZNK17QSslConfiguration14caCertificatesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSslCertificate QSslConfiguration::localCertificate();
  fn _ZNK17QSslConfiguration16localCertificateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QSslConfiguration::sessionTicket();
  fn _ZNK17QSslConfiguration13sessionTicketEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVector<QSslEllipticCurve> QSslConfiguration::ellipticCurves();
  fn _ZNK17QSslConfiguration14ellipticCurvesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QSslCipher> QSslConfiguration::ciphers();
  fn _ZNK17QSslConfiguration7ciphersEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSslCipher QSslConfiguration::sessionCipher();
  fn _ZNK17QSslConfiguration13sessionCipherEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSslKey QSslConfiguration::privateKey();
  fn _ZNK17QSslConfiguration10privateKeyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QSslConfiguration QSslConfiguration::defaultConfiguration();
  fn _ZN17QSslConfiguration20defaultConfigurationEv() -> *mut c_void;
  // proto: static void QSslConfiguration::setDefaultConfiguration(const QSslConfiguration & configuration);
  fn _ZN17QSslConfiguration23setDefaultConfigurationERKS_(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSslConfiguration)=1
#[derive(Default)]
pub struct QSslConfiguration {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslConfiguration {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslConfiguration {
    return QSslConfiguration{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QSslConfiguration::nextNegotiatedProtocol();
impl /*struct*/ QSslConfiguration {
  pub fn nextNegotiatedProtocol<RetType, T: QSslConfiguration_nextNegotiatedProtocol<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextNegotiatedProtocol(self);
    // return 1;
  }
}

pub trait QSslConfiguration_nextNegotiatedProtocol<RetType> {
  fn nextNegotiatedProtocol(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QByteArray QSslConfiguration::nextNegotiatedProtocol();
impl<'a> /*trait*/ QSslConfiguration_nextNegotiatedProtocol<QByteArray> for () {
  fn nextNegotiatedProtocol(self , rsthis: & QSslConfiguration) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration22nextNegotiatedProtocolEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration22nextNegotiatedProtocolEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslConfiguration::swap(QSslConfiguration & other);
impl /*struct*/ QSslConfiguration {
  pub fn swap<RetType, T: QSslConfiguration_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslConfiguration_swap<RetType> {
  fn swap(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  void QSslConfiguration::swap(QSslConfiguration & other);
impl<'a> /*trait*/ QSslConfiguration_swap<()> for (&'a QSslConfiguration) {
  fn swap(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSslConfiguration4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSslConfiguration::peerVerifyDepth();
impl /*struct*/ QSslConfiguration {
  pub fn peerVerifyDepth<RetType, T: QSslConfiguration_peerVerifyDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerVerifyDepth(self);
    // return 1;
  }
}

pub trait QSslConfiguration_peerVerifyDepth<RetType> {
  fn peerVerifyDepth(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  int QSslConfiguration::peerVerifyDepth();
impl<'a> /*trait*/ QSslConfiguration_peerVerifyDepth<i32> for () {
  fn peerVerifyDepth(self , rsthis: & QSslConfiguration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration15peerVerifyDepthEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration15peerVerifyDepthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QVector<QSslEllipticCurve> QSslConfiguration::supportedEllipticCurves();
impl /*struct*/ QSslConfiguration {
  pub fn supportedEllipticCurves_s<RetType, T: QSslConfiguration_supportedEllipticCurves_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedEllipticCurves_s();
    // return 1;
  }
}

pub trait QSslConfiguration_supportedEllipticCurves_s<RetType> {
  fn supportedEllipticCurves_s(self ) -> RetType;
}

  // proto: static QVector<QSslEllipticCurve> QSslConfiguration::supportedEllipticCurves();
impl<'a> /*trait*/ QSslConfiguration_supportedEllipticCurves_s<()> for () {
  fn supportedEllipticCurves_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration23supportedEllipticCurvesEv()};
     unsafe {_ZN17QSslConfiguration23supportedEllipticCurvesEv()};
    // return 1;
  }
}

  // proto:  QList<QByteArray> QSslConfiguration::allowedNextProtocols();
impl /*struct*/ QSslConfiguration {
  pub fn allowedNextProtocols<RetType, T: QSslConfiguration_allowedNextProtocols<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allowedNextProtocols(self);
    // return 1;
  }
}

pub trait QSslConfiguration_allowedNextProtocols<RetType> {
  fn allowedNextProtocols(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QList<QByteArray> QSslConfiguration::allowedNextProtocols();
impl<'a> /*trait*/ QSslConfiguration_allowedNextProtocols<()> for () {
  fn allowedNextProtocols(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration20allowedNextProtocolsEv()};
     unsafe {_ZNK17QSslConfiguration20allowedNextProtocolsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSslConfiguration::sessionTicketLifeTimeHint();
impl /*struct*/ QSslConfiguration {
  pub fn sessionTicketLifeTimeHint<RetType, T: QSslConfiguration_sessionTicketLifeTimeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionTicketLifeTimeHint(self);
    // return 1;
  }
}

pub trait QSslConfiguration_sessionTicketLifeTimeHint<RetType> {
  fn sessionTicketLifeTimeHint(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  int QSslConfiguration::sessionTicketLifeTimeHint();
impl<'a> /*trait*/ QSslConfiguration_sessionTicketLifeTimeHint<i32> for () {
  fn sessionTicketLifeTimeHint(self , rsthis: & QSslConfiguration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration25sessionTicketLifeTimeHintEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration25sessionTicketLifeTimeHintEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QList<QSslCipher> QSslConfiguration::supportedCiphers();
impl /*struct*/ QSslConfiguration {
  pub fn supportedCiphers_s<RetType, T: QSslConfiguration_supportedCiphers_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedCiphers_s();
    // return 1;
  }
}

pub trait QSslConfiguration_supportedCiphers_s<RetType> {
  fn supportedCiphers_s(self ) -> RetType;
}

  // proto: static QList<QSslCipher> QSslConfiguration::supportedCiphers();
impl<'a> /*trait*/ QSslConfiguration_supportedCiphers_s<()> for () {
  fn supportedCiphers_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration16supportedCiphersEv()};
     unsafe {_ZN17QSslConfiguration16supportedCiphersEv()};
    // return 1;
  }
}

  // proto:  void QSslConfiguration::setSessionTicket(const QByteArray & sessionTicket);
impl /*struct*/ QSslConfiguration {
  pub fn setSessionTicket<RetType, T: QSslConfiguration_setSessionTicket<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSessionTicket(self);
    // return 1;
  }
}

pub trait QSslConfiguration_setSessionTicket<RetType> {
  fn setSessionTicket(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  void QSslConfiguration::setSessionTicket(const QByteArray & sessionTicket);
impl<'a> /*trait*/ QSslConfiguration_setSessionTicket<()> for (&'a QByteArray) {
  fn setSessionTicket(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration16setSessionTicketERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSslConfiguration16setSessionTicketERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslConfiguration::QSslConfiguration(const QSslConfiguration & other);
impl /*struct*/ QSslConfiguration {
  pub fn new<T: QSslConfiguration_new>(value: T) -> QSslConfiguration {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslConfiguration_new {
  fn new(self) -> QSslConfiguration;
}

  // proto:  void QSslConfiguration::QSslConfiguration(const QSslConfiguration & other);
impl<'a> /*trait*/ QSslConfiguration_new for (&'a QSslConfiguration) {
  fn new(self) -> QSslConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfigurationC2ERKS_()};
    let ctysz: c_int = unsafe{QSslConfiguration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QSslConfigurationC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslConfiguration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QSslCertificate> QSslConfiguration::localCertificateChain();
impl /*struct*/ QSslConfiguration {
  pub fn localCertificateChain<RetType, T: QSslConfiguration_localCertificateChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localCertificateChain(self);
    // return 1;
  }
}

pub trait QSslConfiguration_localCertificateChain<RetType> {
  fn localCertificateChain(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QList<QSslCertificate> QSslConfiguration::localCertificateChain();
impl<'a> /*trait*/ QSslConfiguration_localCertificateChain<()> for () {
  fn localCertificateChain(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration21localCertificateChainEv()};
     unsafe {_ZNK17QSslConfiguration21localCertificateChainEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QSslCertificate> QSslConfiguration::peerCertificateChain();
impl /*struct*/ QSslConfiguration {
  pub fn peerCertificateChain<RetType, T: QSslConfiguration_peerCertificateChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerCertificateChain(self);
    // return 1;
  }
}

pub trait QSslConfiguration_peerCertificateChain<RetType> {
  fn peerCertificateChain(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QList<QSslCertificate> QSslConfiguration::peerCertificateChain();
impl<'a> /*trait*/ QSslConfiguration_peerCertificateChain<()> for () {
  fn peerCertificateChain(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration20peerCertificateChainEv()};
     unsafe {_ZNK17QSslConfiguration20peerCertificateChainEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslConfiguration::setPrivateKey(const QSslKey & key);
impl /*struct*/ QSslConfiguration {
  pub fn setPrivateKey<RetType, T: QSslConfiguration_setPrivateKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPrivateKey(self);
    // return 1;
  }
}

pub trait QSslConfiguration_setPrivateKey<RetType> {
  fn setPrivateKey(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  void QSslConfiguration::setPrivateKey(const QSslKey & key);
impl<'a> /*trait*/ QSslConfiguration_setPrivateKey<()> for (&'a QSslKey) {
  fn setPrivateKey(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration13setPrivateKeyERK7QSslKey()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSslConfiguration13setPrivateKeyERK7QSslKey(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslConfiguration::setLocalCertificate(const QSslCertificate & certificate);
impl /*struct*/ QSslConfiguration {
  pub fn setLocalCertificate<RetType, T: QSslConfiguration_setLocalCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocalCertificate(self);
    // return 1;
  }
}

pub trait QSslConfiguration_setLocalCertificate<RetType> {
  fn setLocalCertificate(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  void QSslConfiguration::setLocalCertificate(const QSslCertificate & certificate);
impl<'a> /*trait*/ QSslConfiguration_setLocalCertificate<()> for (&'a QSslCertificate) {
  fn setLocalCertificate(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration19setLocalCertificateERK15QSslCertificate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSslConfiguration19setLocalCertificateERK15QSslCertificate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslConfiguration::QSslConfiguration();
impl<'a> /*trait*/ QSslConfiguration_new for () {
  fn new(self) -> QSslConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfigurationC2Ev()};
    let ctysz: c_int = unsafe{QSslConfiguration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QSslConfigurationC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslConfiguration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSslConfiguration::isNull();
impl /*struct*/ QSslConfiguration {
  pub fn isNull<RetType, T: QSslConfiguration_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSslConfiguration_isNull<RetType> {
  fn isNull(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  bool QSslConfiguration::isNull();
impl<'a> /*trait*/ QSslConfiguration_isNull<i8> for () {
  fn isNull(self , rsthis: & QSslConfiguration) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration6isNullEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QList<QSslCertificate> QSslConfiguration::systemCaCertificates();
impl /*struct*/ QSslConfiguration {
  pub fn systemCaCertificates_s<RetType, T: QSslConfiguration_systemCaCertificates_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemCaCertificates_s();
    // return 1;
  }
}

pub trait QSslConfiguration_systemCaCertificates_s<RetType> {
  fn systemCaCertificates_s(self ) -> RetType;
}

  // proto: static QList<QSslCertificate> QSslConfiguration::systemCaCertificates();
impl<'a> /*trait*/ QSslConfiguration_systemCaCertificates_s<()> for () {
  fn systemCaCertificates_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration20systemCaCertificatesEv()};
     unsafe {_ZN17QSslConfiguration20systemCaCertificatesEv()};
    // return 1;
  }
}

  // proto:  QSslCertificate QSslConfiguration::peerCertificate();
impl /*struct*/ QSslConfiguration {
  pub fn peerCertificate<RetType, T: QSslConfiguration_peerCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerCertificate(self);
    // return 1;
  }
}

pub trait QSslConfiguration_peerCertificate<RetType> {
  fn peerCertificate(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QSslCertificate QSslConfiguration::peerCertificate();
impl<'a> /*trait*/ QSslConfiguration_peerCertificate<QSslCertificate> for () {
  fn peerCertificate(self , rsthis: & QSslConfiguration) -> QSslCertificate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration15peerCertificateEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration15peerCertificateEv(rsthis.qclsinst)};
    let mut ret1 = QSslCertificate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslConfiguration::setPeerVerifyDepth(int depth);
impl /*struct*/ QSslConfiguration {
  pub fn setPeerVerifyDepth<RetType, T: QSslConfiguration_setPeerVerifyDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPeerVerifyDepth(self);
    // return 1;
  }
}

pub trait QSslConfiguration_setPeerVerifyDepth<RetType> {
  fn setPeerVerifyDepth(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  void QSslConfiguration::setPeerVerifyDepth(int depth);
impl<'a> /*trait*/ QSslConfiguration_setPeerVerifyDepth<()> for (i32) {
  fn setPeerVerifyDepth(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration18setPeerVerifyDepthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QSslConfiguration18setPeerVerifyDepthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslConfiguration::~QSslConfiguration();
impl /*struct*/ QSslConfiguration {
  pub fn free<RetType, T: QSslConfiguration_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslConfiguration_free<RetType> {
  fn free(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  void QSslConfiguration::~QSslConfiguration();
impl<'a> /*trait*/ QSslConfiguration_free<()> for () {
  fn free(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfigurationD2Ev()};
     unsafe {_ZN17QSslConfigurationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QSslCertificate> QSslConfiguration::caCertificates();
impl /*struct*/ QSslConfiguration {
  pub fn caCertificates<RetType, T: QSslConfiguration_caCertificates<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.caCertificates(self);
    // return 1;
  }
}

pub trait QSslConfiguration_caCertificates<RetType> {
  fn caCertificates(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QList<QSslCertificate> QSslConfiguration::caCertificates();
impl<'a> /*trait*/ QSslConfiguration_caCertificates<()> for () {
  fn caCertificates(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration14caCertificatesEv()};
     unsafe {_ZNK17QSslConfiguration14caCertificatesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSslCertificate QSslConfiguration::localCertificate();
impl /*struct*/ QSslConfiguration {
  pub fn localCertificate<RetType, T: QSslConfiguration_localCertificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localCertificate(self);
    // return 1;
  }
}

pub trait QSslConfiguration_localCertificate<RetType> {
  fn localCertificate(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QSslCertificate QSslConfiguration::localCertificate();
impl<'a> /*trait*/ QSslConfiguration_localCertificate<QSslCertificate> for () {
  fn localCertificate(self , rsthis: & QSslConfiguration) -> QSslCertificate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration16localCertificateEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration16localCertificateEv(rsthis.qclsinst)};
    let mut ret1 = QSslCertificate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QSslConfiguration::sessionTicket();
impl /*struct*/ QSslConfiguration {
  pub fn sessionTicket<RetType, T: QSslConfiguration_sessionTicket<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionTicket(self);
    // return 1;
  }
}

pub trait QSslConfiguration_sessionTicket<RetType> {
  fn sessionTicket(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QByteArray QSslConfiguration::sessionTicket();
impl<'a> /*trait*/ QSslConfiguration_sessionTicket<QByteArray> for () {
  fn sessionTicket(self , rsthis: & QSslConfiguration) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration13sessionTicketEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration13sessionTicketEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVector<QSslEllipticCurve> QSslConfiguration::ellipticCurves();
impl /*struct*/ QSslConfiguration {
  pub fn ellipticCurves<RetType, T: QSslConfiguration_ellipticCurves<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ellipticCurves(self);
    // return 1;
  }
}

pub trait QSslConfiguration_ellipticCurves<RetType> {
  fn ellipticCurves(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QVector<QSslEllipticCurve> QSslConfiguration::ellipticCurves();
impl<'a> /*trait*/ QSslConfiguration_ellipticCurves<()> for () {
  fn ellipticCurves(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration14ellipticCurvesEv()};
     unsafe {_ZNK17QSslConfiguration14ellipticCurvesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QSslCipher> QSslConfiguration::ciphers();
impl /*struct*/ QSslConfiguration {
  pub fn ciphers<RetType, T: QSslConfiguration_ciphers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ciphers(self);
    // return 1;
  }
}

pub trait QSslConfiguration_ciphers<RetType> {
  fn ciphers(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QList<QSslCipher> QSslConfiguration::ciphers();
impl<'a> /*trait*/ QSslConfiguration_ciphers<()> for () {
  fn ciphers(self , rsthis: & QSslConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration7ciphersEv()};
     unsafe {_ZNK17QSslConfiguration7ciphersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSslCipher QSslConfiguration::sessionCipher();
impl /*struct*/ QSslConfiguration {
  pub fn sessionCipher<RetType, T: QSslConfiguration_sessionCipher<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionCipher(self);
    // return 1;
  }
}

pub trait QSslConfiguration_sessionCipher<RetType> {
  fn sessionCipher(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QSslCipher QSslConfiguration::sessionCipher();
impl<'a> /*trait*/ QSslConfiguration_sessionCipher<QSslCipher> for () {
  fn sessionCipher(self , rsthis: & QSslConfiguration) -> QSslCipher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration13sessionCipherEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration13sessionCipherEv(rsthis.qclsinst)};
    let mut ret1 = QSslCipher::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSslKey QSslConfiguration::privateKey();
impl /*struct*/ QSslConfiguration {
  pub fn privateKey<RetType, T: QSslConfiguration_privateKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.privateKey(self);
    // return 1;
  }
}

pub trait QSslConfiguration_privateKey<RetType> {
  fn privateKey(self , rsthis: & QSslConfiguration) -> RetType;
}

  // proto:  QSslKey QSslConfiguration::privateKey();
impl<'a> /*trait*/ QSslConfiguration_privateKey<QSslKey> for () {
  fn privateKey(self , rsthis: & QSslConfiguration) -> QSslKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslConfiguration10privateKeyEv()};
    let mut ret = unsafe {_ZNK17QSslConfiguration10privateKeyEv(rsthis.qclsinst)};
    let mut ret1 = QSslKey::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QSslConfiguration QSslConfiguration::defaultConfiguration();
impl /*struct*/ QSslConfiguration {
  pub fn defaultConfiguration_s<RetType, T: QSslConfiguration_defaultConfiguration_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultConfiguration_s();
    // return 1;
  }
}

pub trait QSslConfiguration_defaultConfiguration_s<RetType> {
  fn defaultConfiguration_s(self ) -> RetType;
}

  // proto: static QSslConfiguration QSslConfiguration::defaultConfiguration();
impl<'a> /*trait*/ QSslConfiguration_defaultConfiguration_s<QSslConfiguration> for () {
  fn defaultConfiguration_s(self ) -> QSslConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration20defaultConfigurationEv()};
    let mut ret = unsafe {_ZN17QSslConfiguration20defaultConfigurationEv()};
    let mut ret1 = QSslConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QSslConfiguration::setDefaultConfiguration(const QSslConfiguration & configuration);
impl /*struct*/ QSslConfiguration {
  pub fn setDefaultConfiguration_s<RetType, T: QSslConfiguration_setDefaultConfiguration_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultConfiguration_s();
    // return 1;
  }
}

pub trait QSslConfiguration_setDefaultConfiguration_s<RetType> {
  fn setDefaultConfiguration_s(self ) -> RetType;
}

  // proto: static void QSslConfiguration::setDefaultConfiguration(const QSslConfiguration & configuration);
impl<'a> /*trait*/ QSslConfiguration_setDefaultConfiguration_s<()> for (&'a QSslConfiguration) {
  fn setDefaultConfiguration_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslConfiguration23setDefaultConfigurationERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSslConfiguration23setDefaultConfigurationERKS_(arg0)};
    // return 1;
  }
}

// <= body block end

