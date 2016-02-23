// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qtcpserver.h
// dst-file: /src/network/qtcpserver.rs
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
use super::qnetworkproxy::QNetworkProxy; // 773
use super::qhostaddress::QHostAddress; // 773
use super::super::core::qstring::QString; // 771
use super::qtcpsocket::QTcpSocket; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTcpServer_Class_Size() -> c_int;
  // proto:  void QTcpServer::setProxy(const QNetworkProxy & networkProxy);
  fn _ZN10QTcpServer8setProxyERK13QNetworkProxy(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTcpServer::waitForNewConnection(int msec, bool * timedOut);
  fn _ZN10QTcpServer20waitForNewConnectionEiPb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_char) -> c_char;
  // proto:  bool QTcpServer::hasPendingConnections();
  fn _ZNK10QTcpServer21hasPendingConnectionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTcpServer::listen(const QHostAddress & address, quint16 port);
  fn _ZN10QTcpServer6listenERK12QHostAddresst(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_ushort) -> c_char;
  // proto:  QString QTcpServer::errorString();
  fn _ZNK10QTcpServer11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QTcpServer::metaObject();
  fn _ZNK10QTcpServer10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTcpServer::pauseAccepting();
  fn _ZN10QTcpServer14pauseAcceptingEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTcpServer::~QTcpServer();
  fn _ZN10QTcpServerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTcpServer::setSocketDescriptor(qintptr socketDescriptor);
  fn _ZN10QTcpServer19setSocketDescriptorEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  qintptr QTcpServer::socketDescriptor();
  fn _ZNK10QTcpServer16socketDescriptorEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTcpServer::resumeAccepting();
  fn _ZN10QTcpServer15resumeAcceptingEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTcpServer::close();
  fn _ZN10QTcpServer5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkProxy QTcpServer::proxy();
  fn _ZNK10QTcpServer5proxyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTcpServer::QTcpServer(const QTcpServer & );
  fn _ZN10QTcpServerC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTcpServer::setMaxPendingConnections(int numConnections);
  fn _ZN10QTcpServer24setMaxPendingConnectionsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTcpServer::maxPendingConnections();
  fn _ZNK10QTcpServer21maxPendingConnectionsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTcpSocket * QTcpServer::nextPendingConnection();
  fn _ZN10QTcpServer21nextPendingConnectionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTcpServer::QTcpServer(QObject * parent);
  fn _ZN10QTcpServerC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTcpServer::isListening();
  fn _ZNK10QTcpServer11isListeningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QHostAddress QTcpServer::serverAddress();
  fn _ZNK10QTcpServer13serverAddressEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  quint16 QTcpServer::serverPort();
  fn _ZNK10QTcpServer10serverPortEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  fn QTcpServer_SlotProxy_connect__ZN10QTcpServer13newConnectionEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTcpServer)=1
#[derive(Default)]
pub struct QTcpServer {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _acceptError: QTcpServer_acceptError_signal,
  pub _newConnection: QTcpServer_newConnection_signal,
}

impl /*struct*/ QTcpServer {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTcpServer {
    return QTcpServer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTcpServer {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QTcpServer {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QTcpServer::setProxy(const QNetworkProxy & networkProxy);
impl /*struct*/ QTcpServer {
  pub fn setProxy<RetType, T: QTcpServer_setProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProxy(self);
    // return 1;
  }
}

pub trait QTcpServer_setProxy<RetType> {
  fn setProxy(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  void QTcpServer::setProxy(const QNetworkProxy & networkProxy);
impl<'a> /*trait*/ QTcpServer_setProxy<()> for (&'a QNetworkProxy) {
  fn setProxy(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer8setProxyERK13QNetworkProxy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTcpServer8setProxyERK13QNetworkProxy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTcpServer::waitForNewConnection(int msec, bool * timedOut);
impl /*struct*/ QTcpServer {
  pub fn waitForNewConnection<RetType, T: QTcpServer_waitForNewConnection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForNewConnection(self);
    // return 1;
  }
}

pub trait QTcpServer_waitForNewConnection<RetType> {
  fn waitForNewConnection(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  bool QTcpServer::waitForNewConnection(int msec, bool * timedOut);
impl<'a> /*trait*/ QTcpServer_waitForNewConnection<i8> for (i32, &'a mut Vec<i8>) {
  fn waitForNewConnection(self , rsthis: & QTcpServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer20waitForNewConnectionEiPb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QTcpServer20waitForNewConnectionEiPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTcpServer::hasPendingConnections();
impl /*struct*/ QTcpServer {
  pub fn hasPendingConnections<RetType, T: QTcpServer_hasPendingConnections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasPendingConnections(self);
    // return 1;
  }
}

pub trait QTcpServer_hasPendingConnections<RetType> {
  fn hasPendingConnections(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  bool QTcpServer::hasPendingConnections();
impl<'a> /*trait*/ QTcpServer_hasPendingConnections<i8> for () {
  fn hasPendingConnections(self , rsthis: & QTcpServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer21hasPendingConnectionsEv()};
    let mut ret = unsafe {_ZNK10QTcpServer21hasPendingConnectionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTcpServer::listen(const QHostAddress & address, quint16 port);
impl /*struct*/ QTcpServer {
  pub fn listen<RetType, T: QTcpServer_listen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.listen(self);
    // return 1;
  }
}

pub trait QTcpServer_listen<RetType> {
  fn listen(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  bool QTcpServer::listen(const QHostAddress & address, quint16 port);
impl<'a> /*trait*/ QTcpServer_listen<i8> for (&'a QHostAddress, u16) {
  fn listen(self , rsthis: & QTcpServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer6listenERK12QHostAddresst()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ushort;
    let mut ret = unsafe {_ZN10QTcpServer6listenERK12QHostAddresst(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QTcpServer::errorString();
impl /*struct*/ QTcpServer {
  pub fn errorString<RetType, T: QTcpServer_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QTcpServer_errorString<RetType> {
  fn errorString(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  QString QTcpServer::errorString();
impl<'a> /*trait*/ QTcpServer_errorString<QString> for () {
  fn errorString(self , rsthis: & QTcpServer) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer11errorStringEv()};
    let mut ret = unsafe {_ZNK10QTcpServer11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTcpServer::metaObject();
impl /*struct*/ QTcpServer {
  pub fn metaObject<RetType, T: QTcpServer_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTcpServer_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  const QMetaObject * QTcpServer::metaObject();
impl<'a> /*trait*/ QTcpServer_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer10metaObjectEv()};
     unsafe {_ZNK10QTcpServer10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTcpServer::pauseAccepting();
impl /*struct*/ QTcpServer {
  pub fn pauseAccepting<RetType, T: QTcpServer_pauseAccepting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pauseAccepting(self);
    // return 1;
  }
}

pub trait QTcpServer_pauseAccepting<RetType> {
  fn pauseAccepting(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  void QTcpServer::pauseAccepting();
impl<'a> /*trait*/ QTcpServer_pauseAccepting<()> for () {
  fn pauseAccepting(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer14pauseAcceptingEv()};
     unsafe {_ZN10QTcpServer14pauseAcceptingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTcpServer::~QTcpServer();
impl /*struct*/ QTcpServer {
  pub fn free<RetType, T: QTcpServer_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTcpServer_free<RetType> {
  fn free(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  void QTcpServer::~QTcpServer();
impl<'a> /*trait*/ QTcpServer_free<()> for () {
  fn free(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServerD2Ev()};
     unsafe {_ZN10QTcpServerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTcpServer::setSocketDescriptor(qintptr socketDescriptor);
impl /*struct*/ QTcpServer {
  pub fn setSocketDescriptor<RetType, T: QTcpServer_setSocketDescriptor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSocketDescriptor(self);
    // return 1;
  }
}

pub trait QTcpServer_setSocketDescriptor<RetType> {
  fn setSocketDescriptor(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  bool QTcpServer::setSocketDescriptor(qintptr socketDescriptor);
impl<'a> /*trait*/ QTcpServer_setSocketDescriptor<i8> for (i32) {
  fn setSocketDescriptor(self , rsthis: & QTcpServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer19setSocketDescriptorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QTcpServer19setSocketDescriptorEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qintptr QTcpServer::socketDescriptor();
impl /*struct*/ QTcpServer {
  pub fn socketDescriptor<RetType, T: QTcpServer_socketDescriptor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.socketDescriptor(self);
    // return 1;
  }
}

pub trait QTcpServer_socketDescriptor<RetType> {
  fn socketDescriptor(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  qintptr QTcpServer::socketDescriptor();
impl<'a> /*trait*/ QTcpServer_socketDescriptor<i32> for () {
  fn socketDescriptor(self , rsthis: & QTcpServer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer16socketDescriptorEv()};
    let mut ret = unsafe {_ZNK10QTcpServer16socketDescriptorEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTcpServer::resumeAccepting();
impl /*struct*/ QTcpServer {
  pub fn resumeAccepting<RetType, T: QTcpServer_resumeAccepting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resumeAccepting(self);
    // return 1;
  }
}

pub trait QTcpServer_resumeAccepting<RetType> {
  fn resumeAccepting(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  void QTcpServer::resumeAccepting();
impl<'a> /*trait*/ QTcpServer_resumeAccepting<()> for () {
  fn resumeAccepting(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer15resumeAcceptingEv()};
     unsafe {_ZN10QTcpServer15resumeAcceptingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTcpServer::close();
impl /*struct*/ QTcpServer {
  pub fn close<RetType, T: QTcpServer_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QTcpServer_close<RetType> {
  fn close(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  void QTcpServer::close();
impl<'a> /*trait*/ QTcpServer_close<()> for () {
  fn close(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer5closeEv()};
     unsafe {_ZN10QTcpServer5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkProxy QTcpServer::proxy();
impl /*struct*/ QTcpServer {
  pub fn proxy<RetType, T: QTcpServer_proxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.proxy(self);
    // return 1;
  }
}

pub trait QTcpServer_proxy<RetType> {
  fn proxy(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  QNetworkProxy QTcpServer::proxy();
impl<'a> /*trait*/ QTcpServer_proxy<QNetworkProxy> for () {
  fn proxy(self , rsthis: & QTcpServer) -> QNetworkProxy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer5proxyEv()};
    let mut ret = unsafe {_ZNK10QTcpServer5proxyEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkProxy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTcpServer::QTcpServer(const QTcpServer & );
impl /*struct*/ QTcpServer {
  pub fn new<T: QTcpServer_new>(value: T) -> QTcpServer {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTcpServer_new {
  fn new(self) -> QTcpServer;
}

  // proto:  void QTcpServer::QTcpServer(const QTcpServer & );
impl<'a> /*trait*/ QTcpServer_new for (&'a QTcpServer) {
  fn new(self) -> QTcpServer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServerC2ERKS_()};
    let ctysz: c_int = unsafe{QTcpServer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTcpServerC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTcpServer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTcpServer::setMaxPendingConnections(int numConnections);
impl /*struct*/ QTcpServer {
  pub fn setMaxPendingConnections<RetType, T: QTcpServer_setMaxPendingConnections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxPendingConnections(self);
    // return 1;
  }
}

pub trait QTcpServer_setMaxPendingConnections<RetType> {
  fn setMaxPendingConnections(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  void QTcpServer::setMaxPendingConnections(int numConnections);
impl<'a> /*trait*/ QTcpServer_setMaxPendingConnections<()> for (i32) {
  fn setMaxPendingConnections(self , rsthis: & QTcpServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer24setMaxPendingConnectionsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTcpServer24setMaxPendingConnectionsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTcpServer::maxPendingConnections();
impl /*struct*/ QTcpServer {
  pub fn maxPendingConnections<RetType, T: QTcpServer_maxPendingConnections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxPendingConnections(self);
    // return 1;
  }
}

pub trait QTcpServer_maxPendingConnections<RetType> {
  fn maxPendingConnections(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  int QTcpServer::maxPendingConnections();
impl<'a> /*trait*/ QTcpServer_maxPendingConnections<i32> for () {
  fn maxPendingConnections(self , rsthis: & QTcpServer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer21maxPendingConnectionsEv()};
    let mut ret = unsafe {_ZNK10QTcpServer21maxPendingConnectionsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTcpSocket * QTcpServer::nextPendingConnection();
impl /*struct*/ QTcpServer {
  pub fn nextPendingConnection<RetType, T: QTcpServer_nextPendingConnection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextPendingConnection(self);
    // return 1;
  }
}

pub trait QTcpServer_nextPendingConnection<RetType> {
  fn nextPendingConnection(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  QTcpSocket * QTcpServer::nextPendingConnection();
impl<'a> /*trait*/ QTcpServer_nextPendingConnection<QTcpSocket> for () {
  fn nextPendingConnection(self , rsthis: & QTcpServer) -> QTcpSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServer21nextPendingConnectionEv()};
    let mut ret = unsafe {_ZN10QTcpServer21nextPendingConnectionEv(rsthis.qclsinst)};
    let mut ret1 = QTcpSocket::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTcpServer::QTcpServer(QObject * parent);
impl<'a> /*trait*/ QTcpServer_new for (&'a QObject) {
  fn new(self) -> QTcpServer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpServerC2EP7QObject()};
    let ctysz: c_int = unsafe{QTcpServer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTcpServerC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTcpServer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTcpServer::isListening();
impl /*struct*/ QTcpServer {
  pub fn isListening<RetType, T: QTcpServer_isListening<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isListening(self);
    // return 1;
  }
}

pub trait QTcpServer_isListening<RetType> {
  fn isListening(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  bool QTcpServer::isListening();
impl<'a> /*trait*/ QTcpServer_isListening<i8> for () {
  fn isListening(self , rsthis: & QTcpServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer11isListeningEv()};
    let mut ret = unsafe {_ZNK10QTcpServer11isListeningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QHostAddress QTcpServer::serverAddress();
impl /*struct*/ QTcpServer {
  pub fn serverAddress<RetType, T: QTcpServer_serverAddress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.serverAddress(self);
    // return 1;
  }
}

pub trait QTcpServer_serverAddress<RetType> {
  fn serverAddress(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  QHostAddress QTcpServer::serverAddress();
impl<'a> /*trait*/ QTcpServer_serverAddress<QHostAddress> for () {
  fn serverAddress(self , rsthis: & QTcpServer) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer13serverAddressEv()};
    let mut ret = unsafe {_ZNK10QTcpServer13serverAddressEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  quint16 QTcpServer::serverPort();
impl /*struct*/ QTcpServer {
  pub fn serverPort<RetType, T: QTcpServer_serverPort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.serverPort(self);
    // return 1;
  }
}

pub trait QTcpServer_serverPort<RetType> {
  fn serverPort(self , rsthis: & QTcpServer) -> RetType;
}

  // proto:  quint16 QTcpServer::serverPort();
impl<'a> /*trait*/ QTcpServer_serverPort<u16> for () {
  fn serverPort(self , rsthis: & QTcpServer) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpServer10serverPortEv()};
    let mut ret = unsafe {_ZNK10QTcpServer10serverPortEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

#[derive(Default)] // for QTcpServer_acceptError
pub struct QTcpServer_acceptError_signal{poi:u64}
impl /* struct */ QTcpServer {
  pub fn acceptError(&self) -> QTcpServer_acceptError_signal {
     return QTcpServer_acceptError_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTcpServer_acceptError_signal {
  pub fn connect<T: QTcpServer_acceptError_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTcpServer_acceptError_signal_connect {
  fn connect(self, sigthis: QTcpServer_acceptError_signal);
}

#[derive(Default)] // for QTcpServer_newConnection
pub struct QTcpServer_newConnection_signal{poi:u64}
impl /* struct */ QTcpServer {
  pub fn newConnection(&self) -> QTcpServer_newConnection_signal {
     return QTcpServer_newConnection_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTcpServer_newConnection_signal {
  pub fn connect<T: QTcpServer_newConnection_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTcpServer_newConnection_signal_connect {
  fn connect(self, sigthis: QTcpServer_newConnection_signal);
}

// newConnection()
extern fn QTcpServer_newConnection_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTcpServer_newConnection_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QTcpServer_newConnection_signal_connect for fn() {
  fn connect(self, sigthis: QTcpServer_newConnection_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTcpServer_newConnection_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTcpServer_SlotProxy_connect__ZN10QTcpServer13newConnectionEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTcpServer_newConnection_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTcpServer_newConnection_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTcpServer_newConnection_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTcpServer_SlotProxy_connect__ZN10QTcpServer13newConnectionEv(arg0, arg1, arg2)};
  }
}
// <= body block end

