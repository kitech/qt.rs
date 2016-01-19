// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qlocalserver.h
// dst-file: /src/network/qlocalserver.rs
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
use super::super::core::qstring::QString; // 771
use super::qlocalsocket::QLocalSocket; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLocalServer_Class_Size() -> c_int;
  // proto:  bool QLocalServer::isListening();
  fn _ZNK12QLocalServer11isListeningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QLocalServer::fullServerName();
  fn _ZNK12QLocalServer14fullServerNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocalServer::serverName();
  fn _ZNK12QLocalServer10serverNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLocalServer::close();
  fn _ZN12QLocalServer5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLocalServer::listen(const QString & name);
  fn _ZN12QLocalServer6listenERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QLocalServer::listen(qintptr socketDescriptor);
  fn _ZN12QLocalServer6listenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  bool QLocalServer::hasPendingConnections();
  fn _ZNK12QLocalServer21hasPendingConnectionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QLocalServer::metaObject();
  fn _ZNK12QLocalServer10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLocalServer::~QLocalServer();
  fn _ZN12QLocalServerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QLocalServer::errorString();
  fn _ZNK12QLocalServer11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QLocalServer::removeServer(const QString & name);
  fn _ZN12QLocalServer12removeServerERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  bool QLocalServer::waitForNewConnection(int msec, bool * timedOut);
  fn _ZN12QLocalServer20waitForNewConnectionEiPb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_char) -> c_char;
  // proto:  QLocalSocket * QLocalServer::nextPendingConnection();
  fn _ZN12QLocalServer21nextPendingConnectionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLocalServer::QLocalServer(const QLocalServer & );
  fn _ZN12QLocalServerC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLocalServer::setMaxPendingConnections(int numConnections);
  fn _ZN12QLocalServer24setMaxPendingConnectionsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QLocalServer::QLocalServer(QObject * parent);
  fn _ZN12QLocalServerC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QLocalServer::maxPendingConnections();
  fn _ZNK12QLocalServer21maxPendingConnectionsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QLocalServer_SlotProxy_connect__ZN12QLocalServer13newConnectionEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLocalServer)=1
#[derive(Default)]
pub struct QLocalServer {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _newConnection: QLocalServer_newConnection_signal,
}

impl /*struct*/ QLocalServer {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLocalServer {
    return QLocalServer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QLocalServer {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QLocalServer {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QLocalServer::isListening();
impl /*struct*/ QLocalServer {
  pub fn isListening<RetType, T: QLocalServer_isListening<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isListening(self);
    // return 1;
  }
}

pub trait QLocalServer_isListening<RetType> {
  fn isListening(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  bool QLocalServer::isListening();
impl<'a> /*trait*/ QLocalServer_isListening<i8> for () {
  fn isListening(self , rsthis: & QLocalServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer11isListeningEv()};
    let mut ret = unsafe {_ZNK12QLocalServer11isListeningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QLocalServer::fullServerName();
impl /*struct*/ QLocalServer {
  pub fn fullServerName<RetType, T: QLocalServer_fullServerName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fullServerName(self);
    // return 1;
  }
}

pub trait QLocalServer_fullServerName<RetType> {
  fn fullServerName(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  QString QLocalServer::fullServerName();
impl<'a> /*trait*/ QLocalServer_fullServerName<QString> for () {
  fn fullServerName(self , rsthis: & QLocalServer) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer14fullServerNameEv()};
    let mut ret = unsafe {_ZNK12QLocalServer14fullServerNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocalServer::serverName();
impl /*struct*/ QLocalServer {
  pub fn serverName<RetType, T: QLocalServer_serverName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.serverName(self);
    // return 1;
  }
}

pub trait QLocalServer_serverName<RetType> {
  fn serverName(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  QString QLocalServer::serverName();
impl<'a> /*trait*/ QLocalServer_serverName<QString> for () {
  fn serverName(self , rsthis: & QLocalServer) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer10serverNameEv()};
    let mut ret = unsafe {_ZNK12QLocalServer10serverNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocalServer::close();
impl /*struct*/ QLocalServer {
  pub fn close<RetType, T: QLocalServer_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QLocalServer_close<RetType> {
  fn close(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  void QLocalServer::close();
impl<'a> /*trait*/ QLocalServer_close<()> for () {
  fn close(self , rsthis: & QLocalServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer5closeEv()};
     unsafe {_ZN12QLocalServer5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLocalServer::listen(const QString & name);
impl /*struct*/ QLocalServer {
  pub fn listen<RetType, T: QLocalServer_listen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.listen(self);
    // return 1;
  }
}

pub trait QLocalServer_listen<RetType> {
  fn listen(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  bool QLocalServer::listen(const QString & name);
impl<'a> /*trait*/ QLocalServer_listen<i8> for (&'a QString) {
  fn listen(self , rsthis: & QLocalServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer6listenERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QLocalServer6listenERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLocalServer::listen(qintptr socketDescriptor);
impl<'a> /*trait*/ QLocalServer_listen<i8> for (i32) {
  fn listen(self , rsthis: & QLocalServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer6listenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QLocalServer6listenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLocalServer::hasPendingConnections();
impl /*struct*/ QLocalServer {
  pub fn hasPendingConnections<RetType, T: QLocalServer_hasPendingConnections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasPendingConnections(self);
    // return 1;
  }
}

pub trait QLocalServer_hasPendingConnections<RetType> {
  fn hasPendingConnections(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  bool QLocalServer::hasPendingConnections();
impl<'a> /*trait*/ QLocalServer_hasPendingConnections<i8> for () {
  fn hasPendingConnections(self , rsthis: & QLocalServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer21hasPendingConnectionsEv()};
    let mut ret = unsafe {_ZNK12QLocalServer21hasPendingConnectionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QLocalServer::metaObject();
impl /*struct*/ QLocalServer {
  pub fn metaObject<RetType, T: QLocalServer_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLocalServer_metaObject<RetType> {
  fn metaObject(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  const QMetaObject * QLocalServer::metaObject();
impl<'a> /*trait*/ QLocalServer_metaObject<()> for () {
  fn metaObject(self , rsthis: & QLocalServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer10metaObjectEv()};
     unsafe {_ZNK12QLocalServer10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLocalServer::~QLocalServer();
impl /*struct*/ QLocalServer {
  pub fn free<RetType, T: QLocalServer_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLocalServer_free<RetType> {
  fn free(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  void QLocalServer::~QLocalServer();
impl<'a> /*trait*/ QLocalServer_free<()> for () {
  fn free(self , rsthis: & QLocalServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServerD2Ev()};
     unsafe {_ZN12QLocalServerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLocalServer::errorString();
impl /*struct*/ QLocalServer {
  pub fn errorString<RetType, T: QLocalServer_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QLocalServer_errorString<RetType> {
  fn errorString(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  QString QLocalServer::errorString();
impl<'a> /*trait*/ QLocalServer_errorString<QString> for () {
  fn errorString(self , rsthis: & QLocalServer) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer11errorStringEv()};
    let mut ret = unsafe {_ZNK12QLocalServer11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QLocalServer::removeServer(const QString & name);
impl /*struct*/ QLocalServer {
  pub fn removeServer_s<RetType, T: QLocalServer_removeServer_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeServer_s();
    // return 1;
  }
}

pub trait QLocalServer_removeServer_s<RetType> {
  fn removeServer_s(self ) -> RetType;
}

  // proto: static bool QLocalServer::removeServer(const QString & name);
impl<'a> /*trait*/ QLocalServer_removeServer_s<i8> for (&'a QString) {
  fn removeServer_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer12removeServerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QLocalServer12removeServerERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLocalServer::waitForNewConnection(int msec, bool * timedOut);
impl /*struct*/ QLocalServer {
  pub fn waitForNewConnection<RetType, T: QLocalServer_waitForNewConnection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForNewConnection(self);
    // return 1;
  }
}

pub trait QLocalServer_waitForNewConnection<RetType> {
  fn waitForNewConnection(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  bool QLocalServer::waitForNewConnection(int msec, bool * timedOut);
impl<'a> /*trait*/ QLocalServer_waitForNewConnection<i8> for (i32, &'a mut Vec<i8>) {
  fn waitForNewConnection(self , rsthis: & QLocalServer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer20waitForNewConnectionEiPb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN12QLocalServer20waitForNewConnectionEiPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QLocalSocket * QLocalServer::nextPendingConnection();
impl /*struct*/ QLocalServer {
  pub fn nextPendingConnection<RetType, T: QLocalServer_nextPendingConnection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextPendingConnection(self);
    // return 1;
  }
}

pub trait QLocalServer_nextPendingConnection<RetType> {
  fn nextPendingConnection(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  QLocalSocket * QLocalServer::nextPendingConnection();
impl<'a> /*trait*/ QLocalServer_nextPendingConnection<QLocalSocket> for () {
  fn nextPendingConnection(self , rsthis: & QLocalServer) -> QLocalSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer21nextPendingConnectionEv()};
    let mut ret = unsafe {_ZN12QLocalServer21nextPendingConnectionEv(rsthis.qclsinst)};
    let mut ret1 = QLocalSocket::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocalServer::QLocalServer(const QLocalServer & );
impl /*struct*/ QLocalServer {
  pub fn new<T: QLocalServer_new>(value: T) -> QLocalServer {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLocalServer_new {
  fn new(self) -> QLocalServer;
}

  // proto:  void QLocalServer::QLocalServer(const QLocalServer & );
impl<'a> /*trait*/ QLocalServer_new for (&'a QLocalServer) {
  fn new(self) -> QLocalServer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServerC2ERKS_()};
    let ctysz: c_int = unsafe{QLocalServer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QLocalServerC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QLocalServer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLocalServer::setMaxPendingConnections(int numConnections);
impl /*struct*/ QLocalServer {
  pub fn setMaxPendingConnections<RetType, T: QLocalServer_setMaxPendingConnections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxPendingConnections(self);
    // return 1;
  }
}

pub trait QLocalServer_setMaxPendingConnections<RetType> {
  fn setMaxPendingConnections(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  void QLocalServer::setMaxPendingConnections(int numConnections);
impl<'a> /*trait*/ QLocalServer_setMaxPendingConnections<()> for (i32) {
  fn setMaxPendingConnections(self , rsthis: & QLocalServer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServer24setMaxPendingConnectionsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QLocalServer24setMaxPendingConnectionsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLocalServer::QLocalServer(QObject * parent);
impl<'a> /*trait*/ QLocalServer_new for (&'a QObject) {
  fn new(self) -> QLocalServer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLocalServerC2EP7QObject()};
    let ctysz: c_int = unsafe{QLocalServer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QLocalServerC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QLocalServer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLocalServer::maxPendingConnections();
impl /*struct*/ QLocalServer {
  pub fn maxPendingConnections<RetType, T: QLocalServer_maxPendingConnections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxPendingConnections(self);
    // return 1;
  }
}

pub trait QLocalServer_maxPendingConnections<RetType> {
  fn maxPendingConnections(self , rsthis: & QLocalServer) -> RetType;
}

  // proto:  int QLocalServer::maxPendingConnections();
impl<'a> /*trait*/ QLocalServer_maxPendingConnections<i32> for () {
  fn maxPendingConnections(self , rsthis: & QLocalServer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QLocalServer21maxPendingConnectionsEv()};
    let mut ret = unsafe {_ZNK12QLocalServer21maxPendingConnectionsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

#[derive(Default)] // for QLocalServer_newConnection
pub struct QLocalServer_newConnection_signal{poi:u64}
impl /* struct */ QLocalServer {
  pub fn newConnection(&self) -> QLocalServer_newConnection_signal {
     return QLocalServer_newConnection_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLocalServer_newConnection_signal {
  pub fn connect<T: QLocalServer_newConnection_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLocalServer_newConnection_signal_connect {
  fn connect(self, sigthis: QLocalServer_newConnection_signal);
}

// newConnection()
extern fn QLocalServer_newConnection_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QLocalServer_newConnection_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QLocalServer_newConnection_signal_connect for fn() {
  fn connect(self, sigthis: QLocalServer_newConnection_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalServer_newConnection_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QLocalServer_SlotProxy_connect__ZN12QLocalServer13newConnectionEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QLocalServer_newConnection_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QLocalServer_newConnection_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLocalServer_newConnection_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QLocalServer_SlotProxy_connect__ZN12QLocalServer13newConnectionEv(arg0, arg1, arg2)};
  }
}
// <= body block end

