// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qlocalsocket.h
// dst-file: /src/network/qlocalsocket.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLocalSocket_Class_Size() -> c_int;
  // proto:  void QLocalSocket::QLocalSocket(QObject * parent);
  fn _ZN12QLocalSocketC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qint64 QLocalSocket::readBufferSize();
  fn _ZNK12QLocalSocket14readBufferSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QString QLocalSocket::fullServerName();
  fn _ZNK12QLocalSocket14fullServerNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLocalSocket::disconnectFromServer();
  fn _ZN12QLocalSocket20disconnectFromServerEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLocalSocket::waitForBytesWritten(int msecs);
  fn _ZN12QLocalSocket19waitForBytesWrittenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QLocalSocket::QLocalSocket(const QLocalSocket & );
  fn _ZN12QLocalSocketC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qintptr QLocalSocket::socketDescriptor();
  fn _ZNK12QLocalSocket16socketDescriptorEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qint64 QLocalSocket::bytesAvailable();
  fn _ZNK12QLocalSocket14bytesAvailableEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QLocalSocket::setServerName(const QString & name);
  fn _ZN12QLocalSocket13setServerNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLocalSocket::setReadBufferSize(qint64 size);
  fn _ZN12QLocalSocket17setReadBufferSizeEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  void QLocalSocket::close();
  fn _ZN12QLocalSocket5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLocalSocket::isValid();
  fn _ZNK12QLocalSocket7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QLocalSocket::flush();
  fn _ZN12QLocalSocket5flushEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QLocalSocket::canReadLine();
  fn _ZNK12QLocalSocket11canReadLineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QLocalSocket::metaObject();
  fn _ZNK12QLocalSocket10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLocalSocket::isSequential();
  fn _ZNK12QLocalSocket12isSequentialEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QLocalSocket::bytesToWrite();
  fn _ZNK12QLocalSocket12bytesToWriteEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QLocalSocket::waitForDisconnected(int msecs);
  fn _ZN12QLocalSocket19waitForDisconnectedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QLocalSocket::~QLocalSocket();
  fn _ZN12QLocalSocketD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLocalSocket::waitForConnected(int msecs);
  fn _ZN12QLocalSocket16waitForConnectedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  bool QLocalSocket::waitForReadyRead(int msecs);
  fn _ZN12QLocalSocket16waitForReadyReadEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QLocalSocket::abort();
  fn _ZN12QLocalSocket5abortEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QLocalSocket::serverName();
  fn _ZNK12QLocalSocket10serverNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QLocalSocket_SlotProxy_connect__ZN12QLocalSocket12disconnectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QLocalSocket_SlotProxy_connect__ZN12QLocalSocket9connectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QLocalSocket_SlotProxy_connect__ZN12QLocalSocket12stateChangedENS_16LocalSocketStateE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QLocalSocket_SlotProxy_connect__ZN12QLocalSocket5errorENS_16LocalSocketErrorE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLocalSocket)=1
#[derive(Default)]
pub struct QLocalSocket {
  qbase: QIODevice,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _error: QLocalSocket_error_signal,
  pub _connected: QLocalSocket_connected_signal,
  pub _disconnected: QLocalSocket_disconnected_signal,
  pub _stateChanged: QLocalSocket_stateChanged_signal,
}

impl /*struct*/ QLocalSocket {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLocalSocket {
    return QLocalSocket{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QLocalSocket {
  type Target = QIODevice;

  fn deref(&self) -> &QIODevice {
    return & self.qbase;
  }
}
impl AsRef<QIODevice> for QLocalSocket {
  fn as_ref(& self) -> & QIODevice {
    return & self.qbase;
  }
}
  // proto:  void QLocalSocket::QLocalSocket(QObject * parent);
impl /*struct*/ QLocalSocket {
  pub fn new<T: QLocalSocket_new>(value: T) -> QLocalSocket {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLocalSocket_new {
  fn new(self) -> QLocalSocket;
}

  // proto:  void QLocalSocket::QLocalSocket(QObject * parent);
impl<'a> /*trait*/ QLocalSocket_new for (&'a QObject) {
  fn new(self) -> QLocalSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocketC2EP7QObject()};
    let ctysz: c_int = unsafe{QLocalSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QLocalSocketC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QLocalSocket{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QLocalSocket::readBufferSize();
impl /*struct*/ QLocalSocket {
  pub fn readBufferSize<RetType, T: QLocalSocket_readBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readBufferSize(self);
    // return 1;
  }
}

pub trait QLocalSocket_readBufferSize<RetType> {
  fn readBufferSize(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  qint64 QLocalSocket::readBufferSize();
impl<'a> /*trait*/ QLocalSocket_readBufferSize<i64> for () {
  fn readBufferSize(self , rsthis: & QLocalSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket14readBufferSizeEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket14readBufferSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QString QLocalSocket::fullServerName();
impl /*struct*/ QLocalSocket {
  pub fn fullServerName<RetType, T: QLocalSocket_fullServerName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fullServerName(self);
    // return 1;
  }
}

pub trait QLocalSocket_fullServerName<RetType> {
  fn fullServerName(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  QString QLocalSocket::fullServerName();
impl<'a> /*trait*/ QLocalSocket_fullServerName<QString> for () {
  fn fullServerName(self , rsthis: & QLocalSocket) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket14fullServerNameEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket14fullServerNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocalSocket::disconnectFromServer();
impl /*struct*/ QLocalSocket {
  pub fn disconnectFromServer<RetType, T: QLocalSocket_disconnectFromServer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.disconnectFromServer(self);
    // return 1;
  }
}

pub trait QLocalSocket_disconnectFromServer<RetType> {
  fn disconnectFromServer(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  void QLocalSocket::disconnectFromServer();
impl<'a> /*trait*/ QLocalSocket_disconnectFromServer<()> for () {
  fn disconnectFromServer(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket20disconnectFromServerEv()};
     unsafe {_ZN12QLocalSocket20disconnectFromServerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLocalSocket::waitForBytesWritten(int msecs);
impl /*struct*/ QLocalSocket {
  pub fn waitForBytesWritten<RetType, T: QLocalSocket_waitForBytesWritten<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QLocalSocket_waitForBytesWritten<RetType> {
  fn waitForBytesWritten(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QLocalSocket_waitForBytesWritten<i8> for (i32) {
  fn waitForBytesWritten(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QLocalSocket19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLocalSocket::QLocalSocket(const QLocalSocket & );
impl<'a> /*trait*/ QLocalSocket_new for (&'a QLocalSocket) {
  fn new(self) -> QLocalSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocketC2ERKS_()};
    let ctysz: c_int = unsafe{QLocalSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QLocalSocketC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QLocalSocket{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qintptr QLocalSocket::socketDescriptor();
impl /*struct*/ QLocalSocket {
  pub fn socketDescriptor<RetType, T: QLocalSocket_socketDescriptor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.socketDescriptor(self);
    // return 1;
  }
}

pub trait QLocalSocket_socketDescriptor<RetType> {
  fn socketDescriptor(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  qintptr QLocalSocket::socketDescriptor();
impl<'a> /*trait*/ QLocalSocket_socketDescriptor<i32> for () {
  fn socketDescriptor(self , rsthis: & QLocalSocket) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket16socketDescriptorEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket16socketDescriptorEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QLocalSocket::bytesAvailable();
impl /*struct*/ QLocalSocket {
  pub fn bytesAvailable<RetType, T: QLocalSocket_bytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable(self);
    // return 1;
  }
}

pub trait QLocalSocket_bytesAvailable<RetType> {
  fn bytesAvailable(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  qint64 QLocalSocket::bytesAvailable();
impl<'a> /*trait*/ QLocalSocket_bytesAvailable<i64> for () {
  fn bytesAvailable(self , rsthis: & QLocalSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QLocalSocket::setServerName(const QString & name);
impl /*struct*/ QLocalSocket {
  pub fn setServerName<RetType, T: QLocalSocket_setServerName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setServerName(self);
    // return 1;
  }
}

pub trait QLocalSocket_setServerName<RetType> {
  fn setServerName(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  void QLocalSocket::setServerName(const QString & name);
impl<'a> /*trait*/ QLocalSocket_setServerName<()> for (&'a QString) {
  fn setServerName(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket13setServerNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QLocalSocket13setServerNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLocalSocket::setReadBufferSize(qint64 size);
impl /*struct*/ QLocalSocket {
  pub fn setReadBufferSize<RetType, T: QLocalSocket_setReadBufferSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadBufferSize(self);
    // return 1;
  }
}

pub trait QLocalSocket_setReadBufferSize<RetType> {
  fn setReadBufferSize(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  void QLocalSocket::setReadBufferSize(qint64 size);
impl<'a> /*trait*/ QLocalSocket_setReadBufferSize<()> for (i64) {
  fn setReadBufferSize(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket17setReadBufferSizeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN12QLocalSocket17setReadBufferSizeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLocalSocket::close();
impl /*struct*/ QLocalSocket {
  pub fn close<RetType, T: QLocalSocket_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QLocalSocket_close<RetType> {
  fn close(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  void QLocalSocket::close();
impl<'a> /*trait*/ QLocalSocket_close<()> for () {
  fn close(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket5closeEv()};
     unsafe {_ZN12QLocalSocket5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLocalSocket::isValid();
impl /*struct*/ QLocalSocket {
  pub fn isValid<RetType, T: QLocalSocket_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QLocalSocket_isValid<RetType> {
  fn isValid(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::isValid();
impl<'a> /*trait*/ QLocalSocket_isValid<i8> for () {
  fn isValid(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket7isValidEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLocalSocket::flush();
impl /*struct*/ QLocalSocket {
  pub fn flush<RetType, T: QLocalSocket_flush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QLocalSocket_flush<RetType> {
  fn flush(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::flush();
impl<'a> /*trait*/ QLocalSocket_flush<i8> for () {
  fn flush(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket5flushEv()};
    let mut ret = unsafe {_ZN12QLocalSocket5flushEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLocalSocket::canReadLine();
impl /*struct*/ QLocalSocket {
  pub fn canReadLine<RetType, T: QLocalSocket_canReadLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canReadLine(self);
    // return 1;
  }
}

pub trait QLocalSocket_canReadLine<RetType> {
  fn canReadLine(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::canReadLine();
impl<'a> /*trait*/ QLocalSocket_canReadLine<i8> for () {
  fn canReadLine(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket11canReadLineEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QLocalSocket::metaObject();
impl /*struct*/ QLocalSocket {
  pub fn metaObject<RetType, T: QLocalSocket_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLocalSocket_metaObject<RetType> {
  fn metaObject(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  const QMetaObject * QLocalSocket::metaObject();
impl<'a> /*trait*/ QLocalSocket_metaObject<()> for () {
  fn metaObject(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket10metaObjectEv()};
     unsafe {_ZNK12QLocalSocket10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLocalSocket::isSequential();
impl /*struct*/ QLocalSocket {
  pub fn isSequential<RetType, T: QLocalSocket_isSequential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSequential(self);
    // return 1;
  }
}

pub trait QLocalSocket_isSequential<RetType> {
  fn isSequential(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::isSequential();
impl<'a> /*trait*/ QLocalSocket_isSequential<i8> for () {
  fn isSequential(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket12isSequentialEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QLocalSocket::bytesToWrite();
impl /*struct*/ QLocalSocket {
  pub fn bytesToWrite<RetType, T: QLocalSocket_bytesToWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite(self);
    // return 1;
  }
}

pub trait QLocalSocket_bytesToWrite<RetType> {
  fn bytesToWrite(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  qint64 QLocalSocket::bytesToWrite();
impl<'a> /*trait*/ QLocalSocket_bytesToWrite<i64> for () {
  fn bytesToWrite(self , rsthis: & QLocalSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QLocalSocket::waitForDisconnected(int msecs);
impl /*struct*/ QLocalSocket {
  pub fn waitForDisconnected<RetType, T: QLocalSocket_waitForDisconnected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForDisconnected(self);
    // return 1;
  }
}

pub trait QLocalSocket_waitForDisconnected<RetType> {
  fn waitForDisconnected(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::waitForDisconnected(int msecs);
impl<'a> /*trait*/ QLocalSocket_waitForDisconnected<i8> for (i32) {
  fn waitForDisconnected(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket19waitForDisconnectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QLocalSocket19waitForDisconnectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLocalSocket::~QLocalSocket();
impl /*struct*/ QLocalSocket {
  pub fn free<RetType, T: QLocalSocket_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLocalSocket_free<RetType> {
  fn free(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  void QLocalSocket::~QLocalSocket();
impl<'a> /*trait*/ QLocalSocket_free<()> for () {
  fn free(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocketD2Ev()};
     unsafe {_ZN12QLocalSocketD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLocalSocket::waitForConnected(int msecs);
impl /*struct*/ QLocalSocket {
  pub fn waitForConnected<RetType, T: QLocalSocket_waitForConnected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForConnected(self);
    // return 1;
  }
}

pub trait QLocalSocket_waitForConnected<RetType> {
  fn waitForConnected(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::waitForConnected(int msecs);
impl<'a> /*trait*/ QLocalSocket_waitForConnected<i8> for (i32) {
  fn waitForConnected(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket16waitForConnectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QLocalSocket16waitForConnectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLocalSocket::waitForReadyRead(int msecs);
impl /*struct*/ QLocalSocket {
  pub fn waitForReadyRead<RetType, T: QLocalSocket_waitForReadyRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QLocalSocket_waitForReadyRead<RetType> {
  fn waitForReadyRead(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  bool QLocalSocket::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QLocalSocket_waitForReadyRead<i8> for (i32) {
  fn waitForReadyRead(self , rsthis: & QLocalSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QLocalSocket16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLocalSocket::abort();
impl /*struct*/ QLocalSocket {
  pub fn abort<RetType, T: QLocalSocket_abort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.abort(self);
    // return 1;
  }
}

pub trait QLocalSocket_abort<RetType> {
  fn abort(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  void QLocalSocket::abort();
impl<'a> /*trait*/ QLocalSocket_abort<()> for () {
  fn abort(self , rsthis: & QLocalSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalSocket5abortEv()};
     unsafe {_ZN12QLocalSocket5abortEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLocalSocket::serverName();
impl /*struct*/ QLocalSocket {
  pub fn serverName<RetType, T: QLocalSocket_serverName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.serverName(self);
    // return 1;
  }
}

pub trait QLocalSocket_serverName<RetType> {
  fn serverName(self , rsthis: & QLocalSocket) -> RetType;
}

  // proto:  QString QLocalSocket::serverName();
impl<'a> /*trait*/ QLocalSocket_serverName<QString> for () {
  fn serverName(self , rsthis: & QLocalSocket) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalSocket10serverNameEv()};
    let mut ret = unsafe {_ZNK12QLocalSocket10serverNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QLocalSocket_error
pub struct QLocalSocket_error_signal{poi:u64}
impl /* struct */ QLocalSocket {
  pub fn error(&self) -> QLocalSocket_error_signal {
     return QLocalSocket_error_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLocalSocket_error_signal {
  pub fn connect<T: QLocalSocket_error_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLocalSocket_error_signal_connect {
  fn connect(self, sigthis: QLocalSocket_error_signal);
}

#[derive(Default)] // for QLocalSocket_connected
pub struct QLocalSocket_connected_signal{poi:u64}
impl /* struct */ QLocalSocket {
  pub fn connected(&self) -> QLocalSocket_connected_signal {
     return QLocalSocket_connected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLocalSocket_connected_signal {
  pub fn connect<T: QLocalSocket_connected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLocalSocket_connected_signal_connect {
  fn connect(self, sigthis: QLocalSocket_connected_signal);
}

#[derive(Default)] // for QLocalSocket_disconnected
pub struct QLocalSocket_disconnected_signal{poi:u64}
impl /* struct */ QLocalSocket {
  pub fn disconnected(&self) -> QLocalSocket_disconnected_signal {
     return QLocalSocket_disconnected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLocalSocket_disconnected_signal {
  pub fn connect<T: QLocalSocket_disconnected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLocalSocket_disconnected_signal_connect {
  fn connect(self, sigthis: QLocalSocket_disconnected_signal);
}

#[derive(Default)] // for QLocalSocket_stateChanged
pub struct QLocalSocket_stateChanged_signal{poi:u64}
impl /* struct */ QLocalSocket {
  pub fn stateChanged(&self) -> QLocalSocket_stateChanged_signal {
     return QLocalSocket_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLocalSocket_stateChanged_signal {
  pub fn connect<T: QLocalSocket_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLocalSocket_stateChanged_signal_connect {
  fn connect(self, sigthis: QLocalSocket_stateChanged_signal);
}

// disconnected()
extern fn QLocalSocket_disconnected_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QLocalSocket_disconnected_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QLocalSocket_disconnected_signal_connect for fn() {
  fn connect(self, sigthis: QLocalSocket_disconnected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_disconnected_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket12disconnectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QLocalSocket_disconnected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QLocalSocket_disconnected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_disconnected_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket12disconnectedEv(arg0, arg1, arg2)};
  }
}
// connected()
extern fn QLocalSocket_connected_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QLocalSocket_connected_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QLocalSocket_connected_signal_connect for fn() {
  fn connect(self, sigthis: QLocalSocket_connected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_connected_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket9connectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QLocalSocket_connected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QLocalSocket_connected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_connected_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket9connectedEv(arg0, arg1, arg2)};
  }
}
// stateChanged(class QLocalSocket::LocalSocketState)
extern fn QLocalSocket_stateChanged_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QLocalSocket_stateChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QLocalSocket_stateChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QLocalSocket_stateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_stateChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket12stateChangedENS_16LocalSocketStateE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QLocalSocket_stateChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QLocalSocket_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_stateChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket12stateChangedENS_16LocalSocketStateE(arg0, arg1, arg2)};
  }
}
// error(class QLocalSocket::LocalSocketError)
extern fn QLocalSocket_error_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QLocalSocket_error_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QLocalSocket_error_signal_connect for fn(i32) {
  fn connect(self, sigthis: QLocalSocket_error_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_error_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket5errorENS_16LocalSocketErrorE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QLocalSocket_error_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QLocalSocket_error_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalSocket_error_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QLocalSocket_SlotProxy_connect__ZN12QLocalSocket5errorENS_16LocalSocketErrorE(arg0, arg1, arg2)};
  }
}
// <= body block end

