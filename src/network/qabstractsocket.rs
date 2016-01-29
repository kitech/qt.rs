// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qabstractsocket.h
// dst-file: /src/network/qabstractsocket.rs
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
use super::super::core::qobject::QObject; // 771
use super::super::core::qstring::QString; // 771
use super::qhostaddress::QHostAddress; // 773
use super::qnetworkproxy::QNetworkProxy; // 773
use super::super::core::qvariant::QVariant; // 771
use super::qauthenticator::QAuthenticator; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractSocket_Class_Size() -> c_int;
  // proto:  bool QAbstractSocket::waitForReadyRead(int msecs);
  fn _ZN15QAbstractSocket16waitForReadyReadEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QAbstractSocket::~QAbstractSocket();
  fn _ZN15QAbstractSocketD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QHostAddress QAbstractSocket::peerAddress();
  fn _ZNK15QAbstractSocket11peerAddressEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QAbstractSocket::metaObject();
  fn _ZNK15QAbstractSocket10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkProxy QAbstractSocket::proxy();
  fn _ZNK15QAbstractSocket5proxyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QAbstractSocket::bytesToWrite();
  fn _ZNK15QAbstractSocket12bytesToWriteEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QAbstractSocket::flush();
  fn _ZN15QAbstractSocket5flushEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractSocket::setReadBufferSize(qint64 size);
  fn _ZN15QAbstractSocket17setReadBufferSizeEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  bool QAbstractSocket::waitForBytesWritten(int msecs);
  fn _ZN15QAbstractSocket19waitForBytesWrittenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  QHostAddress QAbstractSocket::localAddress();
  fn _ZNK15QAbstractSocket12localAddressEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QAbstractSocket::canReadLine();
  fn _ZNK15QAbstractSocket11canReadLineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractSocket::disconnectFromHost();
  fn _ZN15QAbstractSocket18disconnectFromHostEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSocket::resume();
  fn _ZN15QAbstractSocket6resumeEv(qthis: u64 /* *mut c_void*/);
  // proto:  quint16 QAbstractSocket::peerPort();
  fn _ZNK15QAbstractSocket8peerPortEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  bool QAbstractSocket::waitForDisconnected(int msecs);
  fn _ZN15QAbstractSocket19waitForDisconnectedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  quint16 QAbstractSocket::localPort();
  fn _ZNK15QAbstractSocket9localPortEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  qint64 QAbstractSocket::readBufferSize();
  fn _ZNK15QAbstractSocket14readBufferSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QAbstractSocket::abort();
  fn _ZN15QAbstractSocket5abortEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QAbstractSocket::peerName();
  fn _ZNK15QAbstractSocket8peerNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qintptr QAbstractSocket::socketDescriptor();
  fn _ZNK15QAbstractSocket16socketDescriptorEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractSocket::close();
  fn _ZN15QAbstractSocket5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSocket::QAbstractSocket(const QAbstractSocket & );
  fn _ZN15QAbstractSocketC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QAbstractSocket::waitForConnected(int msecs);
  fn _ZN15QAbstractSocket16waitForConnectedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  qint64 QAbstractSocket::bytesAvailable();
  fn _ZNK15QAbstractSocket14bytesAvailableEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QAbstractSocket::isSequential();
  fn _ZNK15QAbstractSocket12isSequentialEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractSocket::isValid();
  fn _ZNK15QAbstractSocket7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractSocket::atEnd();
  fn _ZNK15QAbstractSocket5atEndEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractSocket::setProxy(const QNetworkProxy & networkProxy);
  fn _ZN15QAbstractSocket8setProxyERK13QNetworkProxy(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket12stateChangedENS_11SocketStateE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket9connectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket9hostFoundEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket5errorENS_11SocketErrorE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket12disconnectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket27proxyAuthenticationRequiredERK13QNetworkProxyP14QAuthenticator(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractSocket)=1
#[derive(Default)]
pub struct QAbstractSocket {
  qbase: QIODevice,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _hostFound: QAbstractSocket_hostFound_signal,
  pub _stateChanged: QAbstractSocket_stateChanged_signal,
  pub _proxyAuthenticationRequired: QAbstractSocket_proxyAuthenticationRequired_signal,
  pub _error: QAbstractSocket_error_signal,
  pub _connected: QAbstractSocket_connected_signal,
  pub _disconnected: QAbstractSocket_disconnected_signal,
}

impl /*struct*/ QAbstractSocket {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractSocket {
    return QAbstractSocket{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractSocket {
  type Target = QIODevice;

  fn deref(&self) -> &QIODevice {
    return & self.qbase;
  }
}
impl AsRef<QIODevice> for QAbstractSocket {
  fn as_ref(& self) -> & QIODevice {
    return & self.qbase;
  }
}
  // proto:  bool QAbstractSocket::waitForReadyRead(int msecs);
impl /*struct*/ QAbstractSocket {
  pub fn waitForReadyRead<RetType, T: QAbstractSocket_waitForReadyRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QAbstractSocket_waitForReadyRead<RetType> {
  fn waitForReadyRead(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QAbstractSocket_waitForReadyRead<i8> for (i32) {
  fn waitForReadyRead(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QAbstractSocket16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSocket::~QAbstractSocket();
impl /*struct*/ QAbstractSocket {
  pub fn free<RetType, T: QAbstractSocket_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractSocket_free<RetType> {
  fn free(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::~QAbstractSocket();
impl<'a> /*trait*/ QAbstractSocket_free<()> for () {
  fn free(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocketD2Ev()};
     unsafe {_ZN15QAbstractSocketD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QHostAddress QAbstractSocket::peerAddress();
impl /*struct*/ QAbstractSocket {
  pub fn peerAddress<RetType, T: QAbstractSocket_peerAddress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerAddress(self);
    // return 1;
  }
}

pub trait QAbstractSocket_peerAddress<RetType> {
  fn peerAddress(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  QHostAddress QAbstractSocket::peerAddress();
impl<'a> /*trait*/ QAbstractSocket_peerAddress<QHostAddress> for () {
  fn peerAddress(self , rsthis: & QAbstractSocket) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket11peerAddressEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket11peerAddressEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractSocket::metaObject();
impl /*struct*/ QAbstractSocket {
  pub fn metaObject<RetType, T: QAbstractSocket_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractSocket_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  const QMetaObject * QAbstractSocket::metaObject();
impl<'a> /*trait*/ QAbstractSocket_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket10metaObjectEv()};
     unsafe {_ZNK15QAbstractSocket10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkProxy QAbstractSocket::proxy();
impl /*struct*/ QAbstractSocket {
  pub fn proxy<RetType, T: QAbstractSocket_proxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.proxy(self);
    // return 1;
  }
}

pub trait QAbstractSocket_proxy<RetType> {
  fn proxy(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  QNetworkProxy QAbstractSocket::proxy();
impl<'a> /*trait*/ QAbstractSocket_proxy<QNetworkProxy> for () {
  fn proxy(self , rsthis: & QAbstractSocket) -> QNetworkProxy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket5proxyEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket5proxyEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkProxy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QAbstractSocket::bytesToWrite();
impl /*struct*/ QAbstractSocket {
  pub fn bytesToWrite<RetType, T: QAbstractSocket_bytesToWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite(self);
    // return 1;
  }
}

pub trait QAbstractSocket_bytesToWrite<RetType> {
  fn bytesToWrite(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  qint64 QAbstractSocket::bytesToWrite();
impl<'a> /*trait*/ QAbstractSocket_bytesToWrite<i64> for () {
  fn bytesToWrite(self , rsthis: & QAbstractSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::flush();
impl /*struct*/ QAbstractSocket {
  pub fn flush<RetType, T: QAbstractSocket_flush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QAbstractSocket_flush<RetType> {
  fn flush(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::flush();
impl<'a> /*trait*/ QAbstractSocket_flush<i8> for () {
  fn flush(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket5flushEv()};
    let mut ret = unsafe {_ZN15QAbstractSocket5flushEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSocket::setReadBufferSize(qint64 size);
impl /*struct*/ QAbstractSocket {
  pub fn setReadBufferSize<RetType, T: QAbstractSocket_setReadBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadBufferSize(self);
    // return 1;
  }
}

pub trait QAbstractSocket_setReadBufferSize<RetType> {
  fn setReadBufferSize(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::setReadBufferSize(qint64 size);
impl<'a> /*trait*/ QAbstractSocket_setReadBufferSize<()> for (i64) {
  fn setReadBufferSize(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket17setReadBufferSizeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN15QAbstractSocket17setReadBufferSizeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::waitForBytesWritten(int msecs);
impl /*struct*/ QAbstractSocket {
  pub fn waitForBytesWritten<RetType, T: QAbstractSocket_waitForBytesWritten<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QAbstractSocket_waitForBytesWritten<RetType> {
  fn waitForBytesWritten(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QAbstractSocket_waitForBytesWritten<i8> for (i32) {
  fn waitForBytesWritten(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QAbstractSocket19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QHostAddress QAbstractSocket::localAddress();
impl /*struct*/ QAbstractSocket {
  pub fn localAddress<RetType, T: QAbstractSocket_localAddress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localAddress(self);
    // return 1;
  }
}

pub trait QAbstractSocket_localAddress<RetType> {
  fn localAddress(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  QHostAddress QAbstractSocket::localAddress();
impl<'a> /*trait*/ QAbstractSocket_localAddress<QHostAddress> for () {
  fn localAddress(self , rsthis: & QAbstractSocket) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket12localAddressEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket12localAddressEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::canReadLine();
impl /*struct*/ QAbstractSocket {
  pub fn canReadLine<RetType, T: QAbstractSocket_canReadLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canReadLine(self);
    // return 1;
  }
}

pub trait QAbstractSocket_canReadLine<RetType> {
  fn canReadLine(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::canReadLine();
impl<'a> /*trait*/ QAbstractSocket_canReadLine<i8> for () {
  fn canReadLine(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket11canReadLineEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSocket::disconnectFromHost();
impl /*struct*/ QAbstractSocket {
  pub fn disconnectFromHost<RetType, T: QAbstractSocket_disconnectFromHost<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.disconnectFromHost(self);
    // return 1;
  }
}

pub trait QAbstractSocket_disconnectFromHost<RetType> {
  fn disconnectFromHost(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::disconnectFromHost();
impl<'a> /*trait*/ QAbstractSocket_disconnectFromHost<()> for () {
  fn disconnectFromHost(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket18disconnectFromHostEv()};
     unsafe {_ZN15QAbstractSocket18disconnectFromHostEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSocket::resume();
impl /*struct*/ QAbstractSocket {
  pub fn resume<RetType, T: QAbstractSocket_resume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resume(self);
    // return 1;
  }
}

pub trait QAbstractSocket_resume<RetType> {
  fn resume(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::resume();
impl<'a> /*trait*/ QAbstractSocket_resume<()> for () {
  fn resume(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket6resumeEv()};
     unsafe {_ZN15QAbstractSocket6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  quint16 QAbstractSocket::peerPort();
impl /*struct*/ QAbstractSocket {
  pub fn peerPort<RetType, T: QAbstractSocket_peerPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerPort(self);
    // return 1;
  }
}

pub trait QAbstractSocket_peerPort<RetType> {
  fn peerPort(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  quint16 QAbstractSocket::peerPort();
impl<'a> /*trait*/ QAbstractSocket_peerPort<u16> for () {
  fn peerPort(self , rsthis: & QAbstractSocket) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket8peerPortEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket8peerPortEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::waitForDisconnected(int msecs);
impl /*struct*/ QAbstractSocket {
  pub fn waitForDisconnected<RetType, T: QAbstractSocket_waitForDisconnected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForDisconnected(self);
    // return 1;
  }
}

pub trait QAbstractSocket_waitForDisconnected<RetType> {
  fn waitForDisconnected(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::waitForDisconnected(int msecs);
impl<'a> /*trait*/ QAbstractSocket_waitForDisconnected<i8> for (i32) {
  fn waitForDisconnected(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket19waitForDisconnectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QAbstractSocket19waitForDisconnectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  quint16 QAbstractSocket::localPort();
impl /*struct*/ QAbstractSocket {
  pub fn localPort<RetType, T: QAbstractSocket_localPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPort(self);
    // return 1;
  }
}

pub trait QAbstractSocket_localPort<RetType> {
  fn localPort(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  quint16 QAbstractSocket::localPort();
impl<'a> /*trait*/ QAbstractSocket_localPort<u16> for () {
  fn localPort(self , rsthis: & QAbstractSocket) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket9localPortEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket9localPortEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  qint64 QAbstractSocket::readBufferSize();
impl /*struct*/ QAbstractSocket {
  pub fn readBufferSize<RetType, T: QAbstractSocket_readBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readBufferSize(self);
    // return 1;
  }
}

pub trait QAbstractSocket_readBufferSize<RetType> {
  fn readBufferSize(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  qint64 QAbstractSocket::readBufferSize();
impl<'a> /*trait*/ QAbstractSocket_readBufferSize<i64> for () {
  fn readBufferSize(self , rsthis: & QAbstractSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket14readBufferSizeEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket14readBufferSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QAbstractSocket::abort();
impl /*struct*/ QAbstractSocket {
  pub fn abort<RetType, T: QAbstractSocket_abort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.abort(self);
    // return 1;
  }
}

pub trait QAbstractSocket_abort<RetType> {
  fn abort(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::abort();
impl<'a> /*trait*/ QAbstractSocket_abort<()> for () {
  fn abort(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket5abortEv()};
     unsafe {_ZN15QAbstractSocket5abortEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QAbstractSocket::peerName();
impl /*struct*/ QAbstractSocket {
  pub fn peerName<RetType, T: QAbstractSocket_peerName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peerName(self);
    // return 1;
  }
}

pub trait QAbstractSocket_peerName<RetType> {
  fn peerName(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  QString QAbstractSocket::peerName();
impl<'a> /*trait*/ QAbstractSocket_peerName<QString> for () {
  fn peerName(self , rsthis: & QAbstractSocket) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket8peerNameEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket8peerNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qintptr QAbstractSocket::socketDescriptor();
impl /*struct*/ QAbstractSocket {
  pub fn socketDescriptor<RetType, T: QAbstractSocket_socketDescriptor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.socketDescriptor(self);
    // return 1;
  }
}

pub trait QAbstractSocket_socketDescriptor<RetType> {
  fn socketDescriptor(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  qintptr QAbstractSocket::socketDescriptor();
impl<'a> /*trait*/ QAbstractSocket_socketDescriptor<i32> for () {
  fn socketDescriptor(self , rsthis: & QAbstractSocket) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket16socketDescriptorEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket16socketDescriptorEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSocket::close();
impl /*struct*/ QAbstractSocket {
  pub fn close<RetType, T: QAbstractSocket_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QAbstractSocket_close<RetType> {
  fn close(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::close();
impl<'a> /*trait*/ QAbstractSocket_close<()> for () {
  fn close(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket5closeEv()};
     unsafe {_ZN15QAbstractSocket5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSocket::QAbstractSocket(const QAbstractSocket & );
impl /*struct*/ QAbstractSocket {
  pub fn new<T: QAbstractSocket_new>(value: T) -> QAbstractSocket {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractSocket_new {
  fn new(self) -> QAbstractSocket;
}

  // proto:  void QAbstractSocket::QAbstractSocket(const QAbstractSocket & );
impl<'a> /*trait*/ QAbstractSocket_new for (&'a QAbstractSocket) {
  fn new(self) -> QAbstractSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocketC2ERKS_()};
    let ctysz: c_int = unsafe{QAbstractSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAbstractSocketC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAbstractSocket{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::waitForConnected(int msecs);
impl /*struct*/ QAbstractSocket {
  pub fn waitForConnected<RetType, T: QAbstractSocket_waitForConnected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForConnected(self);
    // return 1;
  }
}

pub trait QAbstractSocket_waitForConnected<RetType> {
  fn waitForConnected(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::waitForConnected(int msecs);
impl<'a> /*trait*/ QAbstractSocket_waitForConnected<i8> for (i32) {
  fn waitForConnected(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket16waitForConnectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QAbstractSocket16waitForConnectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QAbstractSocket::bytesAvailable();
impl /*struct*/ QAbstractSocket {
  pub fn bytesAvailable<RetType, T: QAbstractSocket_bytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable(self);
    // return 1;
  }
}

pub trait QAbstractSocket_bytesAvailable<RetType> {
  fn bytesAvailable(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  qint64 QAbstractSocket::bytesAvailable();
impl<'a> /*trait*/ QAbstractSocket_bytesAvailable<i64> for () {
  fn bytesAvailable(self , rsthis: & QAbstractSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::isSequential();
impl /*struct*/ QAbstractSocket {
  pub fn isSequential<RetType, T: QAbstractSocket_isSequential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSequential(self);
    // return 1;
  }
}

pub trait QAbstractSocket_isSequential<RetType> {
  fn isSequential(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::isSequential();
impl<'a> /*trait*/ QAbstractSocket_isSequential<i8> for () {
  fn isSequential(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket12isSequentialEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::isValid();
impl /*struct*/ QAbstractSocket {
  pub fn isValid<RetType, T: QAbstractSocket_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QAbstractSocket_isValid<RetType> {
  fn isValid(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::isValid();
impl<'a> /*trait*/ QAbstractSocket_isValid<i8> for () {
  fn isValid(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket7isValidEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSocket::atEnd();
impl /*struct*/ QAbstractSocket {
  pub fn atEnd<RetType, T: QAbstractSocket_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QAbstractSocket_atEnd<RetType> {
  fn atEnd(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  bool QAbstractSocket::atEnd();
impl<'a> /*trait*/ QAbstractSocket_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QAbstractSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSocket5atEndEv()};
    let mut ret = unsafe {_ZNK15QAbstractSocket5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSocket::setProxy(const QNetworkProxy & networkProxy);
impl /*struct*/ QAbstractSocket {
  pub fn setProxy<RetType, T: QAbstractSocket_setProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProxy(self);
    // return 1;
  }
}

pub trait QAbstractSocket_setProxy<RetType> {
  fn setProxy(self , rsthis: & QAbstractSocket) -> RetType;
}

  // proto:  void QAbstractSocket::setProxy(const QNetworkProxy & networkProxy);
impl<'a> /*trait*/ QAbstractSocket_setProxy<()> for (&'a QNetworkProxy) {
  fn setProxy(self , rsthis: & QAbstractSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSocket8setProxyERK13QNetworkProxy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractSocket8setProxyERK13QNetworkProxy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QAbstractSocket_hostFound
pub struct QAbstractSocket_hostFound_signal{poi:u64}
impl /* struct */ QAbstractSocket {
  pub fn hostFound(&self) -> QAbstractSocket_hostFound_signal {
     return QAbstractSocket_hostFound_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSocket_hostFound_signal {
  pub fn connect<T: QAbstractSocket_hostFound_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSocket_hostFound_signal_connect {
  fn connect(self, sigthis: QAbstractSocket_hostFound_signal);
}

#[derive(Default)] // for QAbstractSocket_stateChanged
pub struct QAbstractSocket_stateChanged_signal{poi:u64}
impl /* struct */ QAbstractSocket {
  pub fn stateChanged(&self) -> QAbstractSocket_stateChanged_signal {
     return QAbstractSocket_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSocket_stateChanged_signal {
  pub fn connect<T: QAbstractSocket_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSocket_stateChanged_signal_connect {
  fn connect(self, sigthis: QAbstractSocket_stateChanged_signal);
}

#[derive(Default)] // for QAbstractSocket_proxyAuthenticationRequired
pub struct QAbstractSocket_proxyAuthenticationRequired_signal{poi:u64}
impl /* struct */ QAbstractSocket {
  pub fn proxyAuthenticationRequired(&self) -> QAbstractSocket_proxyAuthenticationRequired_signal {
     return QAbstractSocket_proxyAuthenticationRequired_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSocket_proxyAuthenticationRequired_signal {
  pub fn connect<T: QAbstractSocket_proxyAuthenticationRequired_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSocket_proxyAuthenticationRequired_signal_connect {
  fn connect(self, sigthis: QAbstractSocket_proxyAuthenticationRequired_signal);
}

#[derive(Default)] // for QAbstractSocket_error
pub struct QAbstractSocket_error_signal{poi:u64}
impl /* struct */ QAbstractSocket {
  pub fn error(&self) -> QAbstractSocket_error_signal {
     return QAbstractSocket_error_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSocket_error_signal {
  pub fn connect<T: QAbstractSocket_error_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSocket_error_signal_connect {
  fn connect(self, sigthis: QAbstractSocket_error_signal);
}

#[derive(Default)] // for QAbstractSocket_connected
pub struct QAbstractSocket_connected_signal{poi:u64}
impl /* struct */ QAbstractSocket {
  pub fn connected(&self) -> QAbstractSocket_connected_signal {
     return QAbstractSocket_connected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSocket_connected_signal {
  pub fn connect<T: QAbstractSocket_connected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSocket_connected_signal_connect {
  fn connect(self, sigthis: QAbstractSocket_connected_signal);
}

#[derive(Default)] // for QAbstractSocket_disconnected
pub struct QAbstractSocket_disconnected_signal{poi:u64}
impl /* struct */ QAbstractSocket {
  pub fn disconnected(&self) -> QAbstractSocket_disconnected_signal {
     return QAbstractSocket_disconnected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSocket_disconnected_signal {
  pub fn connect<T: QAbstractSocket_disconnected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSocket_disconnected_signal_connect {
  fn connect(self, sigthis: QAbstractSocket_disconnected_signal);
}

// stateChanged(class QAbstractSocket::SocketState)
extern fn QAbstractSocket_stateChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QAbstractSocket_stateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractSocket_stateChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractSocket_stateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_stateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket12stateChangedENS_11SocketStateE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSocket_stateChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QAbstractSocket_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_stateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket12stateChangedENS_11SocketStateE(arg0, arg1, arg2)};
  }
}
// connected()
extern fn QAbstractSocket_connected_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractSocket_connected_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractSocket_connected_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractSocket_connected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_connected_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket9connectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSocket_connected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractSocket_connected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_connected_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket9connectedEv(arg0, arg1, arg2)};
  }
}
// hostFound()
extern fn QAbstractSocket_hostFound_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractSocket_hostFound_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractSocket_hostFound_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractSocket_hostFound_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_hostFound_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket9hostFoundEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSocket_hostFound_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractSocket_hostFound_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_hostFound_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket9hostFoundEv(arg0, arg1, arg2)};
  }
}
// error(class QAbstractSocket::SocketError)
extern fn QAbstractSocket_error_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QAbstractSocket_error_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractSocket_error_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractSocket_error_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_error_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket5errorENS_11SocketErrorE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSocket_error_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QAbstractSocket_error_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_error_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket5errorENS_11SocketErrorE(arg0, arg1, arg2)};
  }
}
// disconnected()
extern fn QAbstractSocket_disconnected_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractSocket_disconnected_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractSocket_disconnected_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractSocket_disconnected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_disconnected_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket12disconnectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSocket_disconnected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractSocket_disconnected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_disconnected_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket12disconnectedEv(arg0, arg1, arg2)};
  }
}
// proxyAuthenticationRequired(const class QNetworkProxy &, class QAuthenticator *)
extern fn QAbstractSocket_proxyAuthenticationRequired_signal_connect_cb_5(rsfptr:fn(QNetworkProxy, QAuthenticator), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkProxy::inheritFrom(arg0 as u64);
  let rsarg1 = QAuthenticator::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QAbstractSocket_proxyAuthenticationRequired_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QNetworkProxy, QAuthenticator)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkProxy::inheritFrom(arg0 as u64);
  let rsarg1 = QAuthenticator::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QAbstractSocket_proxyAuthenticationRequired_signal_connect for fn(QNetworkProxy, QAuthenticator) {
  fn connect(self, sigthis: QAbstractSocket_proxyAuthenticationRequired_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_proxyAuthenticationRequired_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket27proxyAuthenticationRequiredERK13QNetworkProxyP14QAuthenticator(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSocket_proxyAuthenticationRequired_signal_connect for Box<Fn(QNetworkProxy, QAuthenticator)> {
  fn connect(self, sigthis: QAbstractSocket_proxyAuthenticationRequired_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSocket_proxyAuthenticationRequired_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSocket_SlotProxy_connect__ZN15QAbstractSocket27proxyAuthenticationRequiredERK13QNetworkProxyP14QAuthenticator(arg0, arg1, arg2)};
  }
}
// <= body block end

