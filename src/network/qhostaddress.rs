// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qhostaddress.h
// dst-file: /src/network/qhostaddress.rs
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
// use super::qhostaddress::QIPv6Address; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QHostAddress_Class_Size() -> c_int;
  // proto:  void QHostAddress::QHostAddress(const Q_IPV6ADDR & ip6Addr);
  fn _ZN12QHostAddressC2ERK12QIPv6Address(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  Q_IPV6ADDR QHostAddress::toIPv6Address();
  fn _ZNK12QHostAddress13toIPv6AddressEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QHostAddress::QHostAddress(const quint8 * ip6Addr);
  fn _ZN12QHostAddressC2EPKh(qthis: u64 /* *mut c_void*/, arg0: *mut c_uchar);
  // proto:  bool QHostAddress::isNull();
  fn _ZNK12QHostAddress6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QHostAddress::QHostAddress(const QString & address);
  fn _ZN12QHostAddressC2ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHostAddress::clear();
  fn _ZN12QHostAddress5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHostAddress::setAddress(quint8 * ip6Addr);
  fn _ZN12QHostAddress10setAddressEPh(qthis: u64 /* *mut c_void*/, arg0: *mut c_uchar);
  // proto:  void QHostAddress::setAddress(quint32 ip4Addr);
  fn _ZN12QHostAddress10setAddressEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QHostAddress::setAddress(const Q_IPV6ADDR & ip6Addr);
  fn _ZN12QHostAddress10setAddressERK12QIPv6Address(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHostAddress::QHostAddress(quint8 * ip6Addr);
  fn _ZN12QHostAddressC2EPh(qthis: u64 /* *mut c_void*/, arg0: *mut c_uchar);
  // proto:  void QHostAddress::QHostAddress();
  fn _ZN12QHostAddressC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QHostAddress::QHostAddress(quint32 ip4Addr);
  fn _ZN12QHostAddressC2Ej(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QHostAddress::QHostAddress(const QHostAddress & copy);
  fn _ZN12QHostAddressC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QHostAddress::isInSubnet(const QHostAddress & subnet, int netmask);
  fn _ZNK12QHostAddress10isInSubnetERKS_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto:  QString QHostAddress::toString();
  fn _ZNK12QHostAddress8toStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QHostAddress::isLoopback();
  fn _ZNK12QHostAddress10isLoopbackEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QHostAddress::scopeId();
  fn _ZNK12QHostAddress7scopeIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QHostAddress::setAddress(const quint8 * ip6Addr);
  fn _ZN12QHostAddress10setAddressEPKh(qthis: u64 /* *mut c_void*/, arg0: *mut c_uchar);
  // proto:  void QHostAddress::setScopeId(const QString & id);
  fn _ZN12QHostAddress10setScopeIdERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHostAddress::~QHostAddress();
  fn _ZN12QHostAddressD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  quint32 QHostAddress::toIPv4Address(bool * ok);
  fn _ZNK12QHostAddress13toIPv4AddressEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_uint;
  // proto: static QPair<QHostAddress, int> QHostAddress::parseSubnet(const QString & subnet);
  fn _ZN12QHostAddress11parseSubnetERK7QString(arg0: *mut c_void);
  // proto:  quint32 QHostAddress::toIPv4Address();
  fn _ZNK12QHostAddress13toIPv4AddressEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  bool QHostAddress::setAddress(const QString & address);
  fn _ZN12QHostAddress10setAddressERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  fn QIPv6Address_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QHostAddress)=1
#[derive(Default)]
pub struct QHostAddress {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIPv6Address)=16
#[derive(Default)]
pub struct QIPv6Address {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QHostAddress {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHostAddress {
    return QHostAddress{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QHostAddress::QHostAddress(const Q_IPV6ADDR & ip6Addr);
impl /*struct*/ QHostAddress {
  pub fn new<T: QHostAddress_new>(value: T) -> QHostAddress {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QHostAddress_new {
  fn new(self) -> QHostAddress;
}

  // proto:  void QHostAddress::QHostAddress(const Q_IPV6ADDR & ip6Addr);
impl<'a> /*trait*/ QHostAddress_new for (&'a QIPv6Address) {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2ERK12QIPv6Address()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QHostAddressC2ERK12QIPv6Address(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  Q_IPV6ADDR QHostAddress::toIPv6Address();
impl /*struct*/ QHostAddress {
  pub fn toIPv6Address<RetType, T: QHostAddress_toIPv6Address<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toIPv6Address(self);
    // return 1;
  }
}

pub trait QHostAddress_toIPv6Address<RetType> {
  fn toIPv6Address(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  Q_IPV6ADDR QHostAddress::toIPv6Address();
impl<'a> /*trait*/ QHostAddress_toIPv6Address<QIPv6Address> for () {
  fn toIPv6Address(self , rsthis: & QHostAddress) -> QIPv6Address {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress13toIPv6AddressEv()};
    let mut ret = unsafe {_ZNK12QHostAddress13toIPv6AddressEv(rsthis.qclsinst)};
    let mut ret1 = QIPv6Address::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QHostAddress::QHostAddress(const quint8 * ip6Addr);
impl<'a> /*trait*/ QHostAddress_new for (&'a  String) {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2EPKh()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_uchar;
    unsafe {_ZN12QHostAddressC2EPKh(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QHostAddress::isNull();
impl /*struct*/ QHostAddress {
  pub fn isNull<RetType, T: QHostAddress_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QHostAddress_isNull<RetType> {
  fn isNull(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  bool QHostAddress::isNull();
impl<'a> /*trait*/ QHostAddress_isNull<i8> for () {
  fn isNull(self , rsthis: & QHostAddress) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress6isNullEv()};
    let mut ret = unsafe {_ZNK12QHostAddress6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QHostAddress::QHostAddress(const QString & address);
impl<'a> /*trait*/ QHostAddress_new for (&'a QString) {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2ERK7QString()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QHostAddressC2ERK7QString(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHostAddress::clear();
impl /*struct*/ QHostAddress {
  pub fn clear<RetType, T: QHostAddress_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QHostAddress_clear<RetType> {
  fn clear(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  void QHostAddress::clear();
impl<'a> /*trait*/ QHostAddress_clear<()> for () {
  fn clear(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress5clearEv()};
     unsafe {_ZN12QHostAddress5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHostAddress::setAddress(quint8 * ip6Addr);
impl /*struct*/ QHostAddress {
  pub fn setAddress<RetType, T: QHostAddress_setAddress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAddress(self);
    // return 1;
  }
}

pub trait QHostAddress_setAddress<RetType> {
  fn setAddress(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  void QHostAddress::setAddress(quint8 * ip6Addr);
impl<'a> /*trait*/ QHostAddress_setAddress<()> for (&'a mut String) {
  fn setAddress(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress10setAddressEPh()};
    let arg0 = self.as_ptr()  as *mut c_uchar;
     unsafe {_ZN12QHostAddress10setAddressEPh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHostAddress::setAddress(quint32 ip4Addr);
impl<'a> /*trait*/ QHostAddress_setAddress<()> for (u32) {
  fn setAddress(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress10setAddressEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN12QHostAddress10setAddressEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHostAddress::setAddress(const Q_IPV6ADDR & ip6Addr);
impl<'a> /*trait*/ QHostAddress_setAddress<()> for (&'a QIPv6Address) {
  fn setAddress(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress10setAddressERK12QIPv6Address()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QHostAddress10setAddressERK12QIPv6Address(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHostAddress::QHostAddress(quint8 * ip6Addr);
impl<'a> /*trait*/ QHostAddress_new for (&'a mut String) {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2EPh()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_uchar;
    unsafe {_ZN12QHostAddressC2EPh(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHostAddress::QHostAddress();
impl<'a> /*trait*/ QHostAddress_new for () {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2Ev()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN12QHostAddressC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHostAddress::QHostAddress(quint32 ip4Addr);
impl<'a> /*trait*/ QHostAddress_new for (u32) {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2Ej()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_uint;
    unsafe {_ZN12QHostAddressC2Ej(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHostAddress::QHostAddress(const QHostAddress & copy);
impl<'a> /*trait*/ QHostAddress_new for (&'a QHostAddress) {
  fn new(self) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressC2ERKS_()};
    let ctysz: c_int = unsafe{QHostAddress_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QHostAddressC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostAddress{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QHostAddress::isInSubnet(const QHostAddress & subnet, int netmask);
impl /*struct*/ QHostAddress {
  pub fn isInSubnet<RetType, T: QHostAddress_isInSubnet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isInSubnet(self);
    // return 1;
  }
}

pub trait QHostAddress_isInSubnet<RetType> {
  fn isInSubnet(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  bool QHostAddress::isInSubnet(const QHostAddress & subnet, int netmask);
impl<'a> /*trait*/ QHostAddress_isInSubnet<i8> for (&'a QHostAddress, i32) {
  fn isInSubnet(self , rsthis: & QHostAddress) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress10isInSubnetERKS_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QHostAddress10isInSubnetERKS_i(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QHostAddress::toString();
impl /*struct*/ QHostAddress {
  pub fn toString<RetType, T: QHostAddress_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QHostAddress_toString<RetType> {
  fn toString(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  QString QHostAddress::toString();
impl<'a> /*trait*/ QHostAddress_toString<QString> for () {
  fn toString(self , rsthis: & QHostAddress) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress8toStringEv()};
    let mut ret = unsafe {_ZNK12QHostAddress8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QHostAddress::isLoopback();
impl /*struct*/ QHostAddress {
  pub fn isLoopback<RetType, T: QHostAddress_isLoopback<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLoopback(self);
    // return 1;
  }
}

pub trait QHostAddress_isLoopback<RetType> {
  fn isLoopback(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  bool QHostAddress::isLoopback();
impl<'a> /*trait*/ QHostAddress_isLoopback<i8> for () {
  fn isLoopback(self , rsthis: & QHostAddress) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress10isLoopbackEv()};
    let mut ret = unsafe {_ZNK12QHostAddress10isLoopbackEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QHostAddress::scopeId();
impl /*struct*/ QHostAddress {
  pub fn scopeId<RetType, T: QHostAddress_scopeId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scopeId(self);
    // return 1;
  }
}

pub trait QHostAddress_scopeId<RetType> {
  fn scopeId(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  QString QHostAddress::scopeId();
impl<'a> /*trait*/ QHostAddress_scopeId<QString> for () {
  fn scopeId(self , rsthis: & QHostAddress) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress7scopeIdEv()};
    let mut ret = unsafe {_ZNK12QHostAddress7scopeIdEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QHostAddress::setAddress(const quint8 * ip6Addr);
impl<'a> /*trait*/ QHostAddress_setAddress<()> for (&'a  String) {
  fn setAddress(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress10setAddressEPKh()};
    let arg0 = self.as_ptr()  as *mut c_uchar;
     unsafe {_ZN12QHostAddress10setAddressEPKh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHostAddress::setScopeId(const QString & id);
impl /*struct*/ QHostAddress {
  pub fn setScopeId<RetType, T: QHostAddress_setScopeId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScopeId(self);
    // return 1;
  }
}

pub trait QHostAddress_setScopeId<RetType> {
  fn setScopeId(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  void QHostAddress::setScopeId(const QString & id);
impl<'a> /*trait*/ QHostAddress_setScopeId<()> for (&'a QString) {
  fn setScopeId(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress10setScopeIdERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QHostAddress10setScopeIdERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHostAddress::~QHostAddress();
impl /*struct*/ QHostAddress {
  pub fn free<RetType, T: QHostAddress_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHostAddress_free<RetType> {
  fn free(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  void QHostAddress::~QHostAddress();
impl<'a> /*trait*/ QHostAddress_free<()> for () {
  fn free(self , rsthis: & QHostAddress) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddressD2Ev()};
     unsafe {_ZN12QHostAddressD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  quint32 QHostAddress::toIPv4Address(bool * ok);
impl /*struct*/ QHostAddress {
  pub fn toIPv4Address<RetType, T: QHostAddress_toIPv4Address<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toIPv4Address(self);
    // return 1;
  }
}

pub trait QHostAddress_toIPv4Address<RetType> {
  fn toIPv4Address(self , rsthis: & QHostAddress) -> RetType;
}

  // proto:  quint32 QHostAddress::toIPv4Address(bool * ok);
impl<'a> /*trait*/ QHostAddress_toIPv4Address<u32> for (&'a mut Vec<i8>) {
  fn toIPv4Address(self , rsthis: & QHostAddress) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress13toIPv4AddressEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK12QHostAddress13toIPv4AddressEPb(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static QPair<QHostAddress, int> QHostAddress::parseSubnet(const QString & subnet);
impl /*struct*/ QHostAddress {
  pub fn parseSubnet_s<RetType, T: QHostAddress_parseSubnet_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.parseSubnet_s();
    // return 1;
  }
}

pub trait QHostAddress_parseSubnet_s<RetType> {
  fn parseSubnet_s(self ) -> RetType;
}

  // proto: static QPair<QHostAddress, int> QHostAddress::parseSubnet(const QString & subnet);
impl<'a> /*trait*/ QHostAddress_parseSubnet_s<()> for (&'a QString) {
  fn parseSubnet_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress11parseSubnetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QHostAddress11parseSubnetERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  quint32 QHostAddress::toIPv4Address();
impl<'a> /*trait*/ QHostAddress_toIPv4Address<u32> for () {
  fn toIPv4Address(self , rsthis: & QHostAddress) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QHostAddress13toIPv4AddressEv()};
    let mut ret = unsafe {_ZNK12QHostAddress13toIPv4AddressEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  bool QHostAddress::setAddress(const QString & address);
impl<'a> /*trait*/ QHostAddress_setAddress<i8> for (&'a QString) {
  fn setAddress(self , rsthis: & QHostAddress) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QHostAddress10setAddressERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QHostAddress10setAddressERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIPv6Address {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIPv6Address {
    return QIPv6Address{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

