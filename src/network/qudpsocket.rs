// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qudpsocket.h
// dst-file: /src/network/qudpsocket.rs
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
use super::qhostaddress::QHostAddress; // 773
use super::super::core::qbytearray::QByteArray; // 771
use super::qnetworkinterface::QNetworkInterface; // 773
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QUdpSocket_Class_Size() -> c_int;
  // proto:  qint64 QUdpSocket::writeDatagram(const char * data, qint64 len, const QHostAddress & host, quint16 port);
  fn _ZN10QUdpSocket13writeDatagramEPKcxRK12QHostAddresst(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_longlong, arg2: *mut c_void, arg3: c_ushort) -> c_longlong;
  // proto:  bool QUdpSocket::leaveMulticastGroup(const QHostAddress & groupAddress);
  fn _ZN10QUdpSocket19leaveMulticastGroupERK12QHostAddress(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QUdpSocket::hasPendingDatagrams();
  fn _ZNK10QUdpSocket19hasPendingDatagramsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QUdpSocket::writeDatagram(const QByteArray & datagram, const QHostAddress & host, quint16 port);
  fn _ZN10QUdpSocket13writeDatagramERK10QByteArrayRK12QHostAddresst(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_ushort) -> c_longlong;
  // proto:  void QUdpSocket::QUdpSocket(const QUdpSocket & );
  fn _ZN10QUdpSocketC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QUdpSocket::leaveMulticastGroup(const QHostAddress & groupAddress, const QNetworkInterface & iface);
  fn _ZN10QUdpSocket19leaveMulticastGroupERK12QHostAddressRK17QNetworkInterface(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  qint64 QUdpSocket::pendingDatagramSize();
  fn _ZNK10QUdpSocket19pendingDatagramSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QUdpSocket::QUdpSocket(QObject * parent);
  fn _ZN10QUdpSocketC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QUdpSocket::metaObject();
  fn _ZNK10QUdpSocket10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  qint64 QUdpSocket::readDatagram(char * data, qint64 maxlen, QHostAddress * host, quint16 * port);
  fn _ZN10QUdpSocket12readDatagramEPcxP12QHostAddressPt(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_longlong, arg2: *mut c_void, arg3: *mut c_ushort) -> c_longlong;
  // proto:  bool QUdpSocket::joinMulticastGroup(const QHostAddress & groupAddress, const QNetworkInterface & iface);
  fn _ZN10QUdpSocket18joinMulticastGroupERK12QHostAddressRK17QNetworkInterface(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QNetworkInterface QUdpSocket::multicastInterface();
  fn _ZNK10QUdpSocket18multicastInterfaceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QUdpSocket::joinMulticastGroup(const QHostAddress & groupAddress);
  fn _ZN10QUdpSocket18joinMulticastGroupERK12QHostAddress(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QUdpSocket::~QUdpSocket();
  fn _ZN10QUdpSocketD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QUdpSocket::setMulticastInterface(const QNetworkInterface & iface);
  fn _ZN10QUdpSocket21setMulticastInterfaceERK17QNetworkInterface(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QUdpSocket)=1
#[derive(Default)]
pub struct QUdpSocket {
  qbase: QAbstractSocket,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QUdpSocket {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUdpSocket {
    return QUdpSocket{qbase: QAbstractSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QUdpSocket {
  type Target = QAbstractSocket;

  fn deref(&self) -> &QAbstractSocket {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSocket> for QUdpSocket {
  fn as_ref(& self) -> & QAbstractSocket {
    return & self.qbase;
  }
}
  // proto:  qint64 QUdpSocket::writeDatagram(const char * data, qint64 len, const QHostAddress & host, quint16 port);
impl /*struct*/ QUdpSocket {
  pub fn writeDatagram<RetType, T: QUdpSocket_writeDatagram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeDatagram(self);
    // return 1;
  }
}

pub trait QUdpSocket_writeDatagram<RetType> {
  fn writeDatagram(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  qint64 QUdpSocket::writeDatagram(const char * data, qint64 len, const QHostAddress & host, quint16 port);
impl<'a> /*trait*/ QUdpSocket_writeDatagram<i64> for (&'a  String, i64, &'a QHostAddress, u16) {
  fn writeDatagram(self , rsthis: & QUdpSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket13writeDatagramEPKcxRK12QHostAddresst()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_ushort;
    let mut ret = unsafe {_ZN10QUdpSocket13writeDatagramEPKcxRK12QHostAddresst(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QUdpSocket::leaveMulticastGroup(const QHostAddress & groupAddress);
impl /*struct*/ QUdpSocket {
  pub fn leaveMulticastGroup<RetType, T: QUdpSocket_leaveMulticastGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leaveMulticastGroup(self);
    // return 1;
  }
}

pub trait QUdpSocket_leaveMulticastGroup<RetType> {
  fn leaveMulticastGroup(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  bool QUdpSocket::leaveMulticastGroup(const QHostAddress & groupAddress);
impl<'a> /*trait*/ QUdpSocket_leaveMulticastGroup<i8> for (&'a QHostAddress) {
  fn leaveMulticastGroup(self , rsthis: & QUdpSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket19leaveMulticastGroupERK12QHostAddress()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QUdpSocket19leaveMulticastGroupERK12QHostAddress(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QUdpSocket::hasPendingDatagrams();
impl /*struct*/ QUdpSocket {
  pub fn hasPendingDatagrams<RetType, T: QUdpSocket_hasPendingDatagrams<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasPendingDatagrams(self);
    // return 1;
  }
}

pub trait QUdpSocket_hasPendingDatagrams<RetType> {
  fn hasPendingDatagrams(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  bool QUdpSocket::hasPendingDatagrams();
impl<'a> /*trait*/ QUdpSocket_hasPendingDatagrams<i8> for () {
  fn hasPendingDatagrams(self , rsthis: & QUdpSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUdpSocket19hasPendingDatagramsEv()};
    let mut ret = unsafe {_ZNK10QUdpSocket19hasPendingDatagramsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QUdpSocket::writeDatagram(const QByteArray & datagram, const QHostAddress & host, quint16 port);
impl<'a> /*trait*/ QUdpSocket_writeDatagram<i64> for (&'a QByteArray, &'a QHostAddress, u16) {
  fn writeDatagram(self , rsthis: & QUdpSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket13writeDatagramERK10QByteArrayRK12QHostAddresst()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_ushort;
    let mut ret = unsafe {_ZN10QUdpSocket13writeDatagramERK10QByteArrayRK12QHostAddresst(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QUdpSocket::QUdpSocket(const QUdpSocket & );
impl /*struct*/ QUdpSocket {
  pub fn new<T: QUdpSocket_new>(value: T) -> QUdpSocket {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QUdpSocket_new {
  fn new(self) -> QUdpSocket;
}

  // proto:  void QUdpSocket::QUdpSocket(const QUdpSocket & );
impl<'a> /*trait*/ QUdpSocket_new for (&'a QUdpSocket) {
  fn new(self) -> QUdpSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocketC2ERKS_()};
    let ctysz: c_int = unsafe{QUdpSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUdpSocketC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QUdpSocket{qbase: QAbstractSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QUdpSocket::leaveMulticastGroup(const QHostAddress & groupAddress, const QNetworkInterface & iface);
impl<'a> /*trait*/ QUdpSocket_leaveMulticastGroup<i8> for (&'a QHostAddress, &'a QNetworkInterface) {
  fn leaveMulticastGroup(self , rsthis: & QUdpSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket19leaveMulticastGroupERK12QHostAddressRK17QNetworkInterface()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QUdpSocket19leaveMulticastGroupERK12QHostAddressRK17QNetworkInterface(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QUdpSocket::pendingDatagramSize();
impl /*struct*/ QUdpSocket {
  pub fn pendingDatagramSize<RetType, T: QUdpSocket_pendingDatagramSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pendingDatagramSize(self);
    // return 1;
  }
}

pub trait QUdpSocket_pendingDatagramSize<RetType> {
  fn pendingDatagramSize(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  qint64 QUdpSocket::pendingDatagramSize();
impl<'a> /*trait*/ QUdpSocket_pendingDatagramSize<i64> for () {
  fn pendingDatagramSize(self , rsthis: & QUdpSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUdpSocket19pendingDatagramSizeEv()};
    let mut ret = unsafe {_ZNK10QUdpSocket19pendingDatagramSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QUdpSocket::QUdpSocket(QObject * parent);
impl<'a> /*trait*/ QUdpSocket_new for (&'a QObject) {
  fn new(self) -> QUdpSocket {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocketC2EP7QObject()};
    let ctysz: c_int = unsafe{QUdpSocket_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUdpSocketC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QUdpSocket{qbase: QAbstractSocket::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QUdpSocket::metaObject();
impl /*struct*/ QUdpSocket {
  pub fn metaObject<RetType, T: QUdpSocket_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QUdpSocket_metaObject<RetType> {
  fn metaObject(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  const QMetaObject * QUdpSocket::metaObject();
impl<'a> /*trait*/ QUdpSocket_metaObject<()> for () {
  fn metaObject(self , rsthis: & QUdpSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUdpSocket10metaObjectEv()};
     unsafe {_ZNK10QUdpSocket10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QUdpSocket::readDatagram(char * data, qint64 maxlen, QHostAddress * host, quint16 * port);
impl /*struct*/ QUdpSocket {
  pub fn readDatagram<RetType, T: QUdpSocket_readDatagram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readDatagram(self);
    // return 1;
  }
}

pub trait QUdpSocket_readDatagram<RetType> {
  fn readDatagram(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  qint64 QUdpSocket::readDatagram(char * data, qint64 maxlen, QHostAddress * host, quint16 * port);
impl<'a> /*trait*/ QUdpSocket_readDatagram<i64> for (&'a mut String, i64, &'a QHostAddress, &'a mut Vec<u16>) {
  fn readDatagram(self , rsthis: & QUdpSocket) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket12readDatagramEPcxP12QHostAddressPt()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *mut c_ushort;
    let mut ret = unsafe {_ZN10QUdpSocket12readDatagramEPcxP12QHostAddressPt(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QUdpSocket::joinMulticastGroup(const QHostAddress & groupAddress, const QNetworkInterface & iface);
impl /*struct*/ QUdpSocket {
  pub fn joinMulticastGroup<RetType, T: QUdpSocket_joinMulticastGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.joinMulticastGroup(self);
    // return 1;
  }
}

pub trait QUdpSocket_joinMulticastGroup<RetType> {
  fn joinMulticastGroup(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  bool QUdpSocket::joinMulticastGroup(const QHostAddress & groupAddress, const QNetworkInterface & iface);
impl<'a> /*trait*/ QUdpSocket_joinMulticastGroup<i8> for (&'a QHostAddress, &'a QNetworkInterface) {
  fn joinMulticastGroup(self , rsthis: & QUdpSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket18joinMulticastGroupERK12QHostAddressRK17QNetworkInterface()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QUdpSocket18joinMulticastGroupERK12QHostAddressRK17QNetworkInterface(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QNetworkInterface QUdpSocket::multicastInterface();
impl /*struct*/ QUdpSocket {
  pub fn multicastInterface<RetType, T: QUdpSocket_multicastInterface<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.multicastInterface(self);
    // return 1;
  }
}

pub trait QUdpSocket_multicastInterface<RetType> {
  fn multicastInterface(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  QNetworkInterface QUdpSocket::multicastInterface();
impl<'a> /*trait*/ QUdpSocket_multicastInterface<QNetworkInterface> for () {
  fn multicastInterface(self , rsthis: & QUdpSocket) -> QNetworkInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUdpSocket18multicastInterfaceEv()};
    let mut ret = unsafe {_ZNK10QUdpSocket18multicastInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QUdpSocket::joinMulticastGroup(const QHostAddress & groupAddress);
impl<'a> /*trait*/ QUdpSocket_joinMulticastGroup<i8> for (&'a QHostAddress) {
  fn joinMulticastGroup(self , rsthis: & QUdpSocket) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket18joinMulticastGroupERK12QHostAddress()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QUdpSocket18joinMulticastGroupERK12QHostAddress(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QUdpSocket::~QUdpSocket();
impl /*struct*/ QUdpSocket {
  pub fn free<RetType, T: QUdpSocket_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QUdpSocket_free<RetType> {
  fn free(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  void QUdpSocket::~QUdpSocket();
impl<'a> /*trait*/ QUdpSocket_free<()> for () {
  fn free(self , rsthis: & QUdpSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocketD2Ev()};
     unsafe {_ZN10QUdpSocketD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUdpSocket::setMulticastInterface(const QNetworkInterface & iface);
impl /*struct*/ QUdpSocket {
  pub fn setMulticastInterface<RetType, T: QUdpSocket_setMulticastInterface<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMulticastInterface(self);
    // return 1;
  }
}

pub trait QUdpSocket_setMulticastInterface<RetType> {
  fn setMulticastInterface(self , rsthis: & QUdpSocket) -> RetType;
}

  // proto:  void QUdpSocket::setMulticastInterface(const QNetworkInterface & iface);
impl<'a> /*trait*/ QUdpSocket_setMulticastInterface<()> for (&'a QNetworkInterface) {
  fn setMulticastInterface(self , rsthis: & QUdpSocket) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUdpSocket21setMulticastInterfaceERK17QNetworkInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUdpSocket21setMulticastInterfaceERK17QNetworkInterface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

