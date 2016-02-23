// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qtcpsocket.h
// dst-file: /src/network/qtcpsocket.rs
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
use super::qabstractsocket::QAbstractSocket; // 773
use std::ops::Deref;
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTcpSocket_Class_Size() -> c_int;
  // proto:  const QMetaObject * QTcpSocket::metaObject();
  fn _ZNK10QTcpSocket10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTcpSocket::QTcpSocket(const QTcpSocket & );
  fn _ZN10QTcpSocketC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTcpSocket::~QTcpSocket();
  fn _ZN10QTcpSocketD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTcpSocket::QTcpSocket(QObject * parent);
  fn _ZN10QTcpSocketC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTcpSocket)=1
#[derive(Default)]
pub struct QTcpSocket {
  qbase: QAbstractSocket,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTcpSocket {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTcpSocket {
    return QTcpSocket{qbase: QAbstractSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTcpSocket {
  type Target = QAbstractSocket;

  fn deref(&self) -> &QAbstractSocket {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSocket> for QTcpSocket {
  fn as_ref(& self) -> & QAbstractSocket {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QTcpSocket::metaObject();
impl /*struct*/ QTcpSocket {
  pub fn metaObject<RetType, T: QTcpSocket_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTcpSocket_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTcpSocket) -> RetType;
}

  // proto:  const QMetaObject * QTcpSocket::metaObject();
impl<'a> /*trait*/ QTcpSocket_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTcpSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTcpSocket10metaObjectEv()};
     unsafe {_ZNK10QTcpSocket10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTcpSocket::QTcpSocket(const QTcpSocket & );
impl /*struct*/ QTcpSocket {
  pub fn new<T: QTcpSocket_new>(value: T) -> QTcpSocket {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTcpSocket_new {
  fn new(self) -> QTcpSocket;
}

  // proto:  void QTcpSocket::QTcpSocket(const QTcpSocket & );
impl<'a> /*trait*/ QTcpSocket_new for (&'a QTcpSocket) {
  fn new(self) -> QTcpSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpSocketC2ERKS_()};
    let ctysz: c_int = unsafe{QTcpSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTcpSocketC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTcpSocket{qbase: QAbstractSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTcpSocket::~QTcpSocket();
impl /*struct*/ QTcpSocket {
  pub fn free<RetType, T: QTcpSocket_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTcpSocket_free<RetType> {
  fn free(self , rsthis: & QTcpSocket) -> RetType;
}

  // proto:  void QTcpSocket::~QTcpSocket();
impl<'a> /*trait*/ QTcpSocket_free<()> for () {
  fn free(self , rsthis: & QTcpSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpSocketD2Ev()};
     unsafe {_ZN10QTcpSocketD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTcpSocket::QTcpSocket(QObject * parent);
impl<'a> /*trait*/ QTcpSocket_new for (&'a QObject) {
  fn new(self) -> QTcpSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTcpSocketC2EP7QObject()};
    let ctysz: c_int = unsafe{QTcpSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTcpSocketC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTcpSocket{qbase: QAbstractSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

