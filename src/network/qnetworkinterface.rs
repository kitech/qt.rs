// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qnetworkinterface.h
// dst-file: /src/network/qnetworkinterface.rs
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
use super::qhostaddress::QHostAddress; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkAddressEntry_Class_Size() -> c_int;
  // proto:  void QNetworkAddressEntry::swap(QNetworkAddressEntry & other);
  fn _ZN20QNetworkAddressEntry4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkAddressEntry::setPrefixLength(int length);
  fn _ZN20QNetworkAddressEntry15setPrefixLengthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QNetworkAddressEntry::prefixLength();
  fn _ZNK20QNetworkAddressEntry12prefixLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QHostAddress QNetworkAddressEntry::netmask();
  fn _ZNK20QNetworkAddressEntry7netmaskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkAddressEntry::setBroadcast(const QHostAddress & newBroadcast);
  fn _ZN20QNetworkAddressEntry12setBroadcastERK12QHostAddress(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkAddressEntry::QNetworkAddressEntry();
  fn _ZN20QNetworkAddressEntryC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkAddressEntry::setIp(const QHostAddress & newIp);
  fn _ZN20QNetworkAddressEntry5setIpERK12QHostAddress(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkAddressEntry::setNetmask(const QHostAddress & newNetmask);
  fn _ZN20QNetworkAddressEntry10setNetmaskERK12QHostAddress(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkAddressEntry::QNetworkAddressEntry(const QNetworkAddressEntry & other);
  fn _ZN20QNetworkAddressEntryC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QHostAddress QNetworkAddressEntry::broadcast();
  fn _ZNK20QNetworkAddressEntry9broadcastEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkAddressEntry::~QNetworkAddressEntry();
  fn _ZN20QNetworkAddressEntryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QHostAddress QNetworkAddressEntry::ip();
  fn _ZNK20QNetworkAddressEntry2ipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QNetworkInterface_Class_Size() -> c_int;
  // proto: static QNetworkInterface QNetworkInterface::interfaceFromName(const QString & name);
  fn _ZN17QNetworkInterface17interfaceFromNameERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkInterface::QNetworkInterface();
  fn _ZN17QNetworkInterfaceC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QNetworkInterface::hardwareAddress();
  fn _ZNK17QNetworkInterface15hardwareAddressEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QNetworkInterface::name();
  fn _ZNK17QNetworkInterface4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkInterface::swap(QNetworkInterface & other);
  fn _ZN17QNetworkInterface4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QList<QHostAddress> QNetworkInterface::allAddresses();
  fn _ZN17QNetworkInterface12allAddressesEv();
  // proto:  QString QNetworkInterface::humanReadableName();
  fn _ZNK17QNetworkInterface17humanReadableNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QNetworkInterface::index();
  fn _ZNK17QNetworkInterface5indexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QNetworkInterface::isValid();
  fn _ZNK17QNetworkInterface7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QNetworkInterface::QNetworkInterface(const QNetworkInterface & other);
  fn _ZN17QNetworkInterfaceC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QList<QNetworkInterface> QNetworkInterface::allInterfaces();
  fn _ZN17QNetworkInterface13allInterfacesEv();
  // proto:  QList<QNetworkAddressEntry> QNetworkInterface::addressEntries();
  fn _ZNK17QNetworkInterface14addressEntriesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkInterface::~QNetworkInterface();
  fn _ZN17QNetworkInterfaceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static QNetworkInterface QNetworkInterface::interfaceFromIndex(int index);
  fn _ZN17QNetworkInterface18interfaceFromIndexEi(arg0: c_int) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkAddressEntry)=1
#[derive(Default)]
pub struct QNetworkAddressEntry {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QNetworkInterface)=1
#[derive(Default)]
pub struct QNetworkInterface {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkAddressEntry {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkAddressEntry {
    return QNetworkAddressEntry{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QNetworkAddressEntry::swap(QNetworkAddressEntry & other);
impl /*struct*/ QNetworkAddressEntry {
  pub fn swap<RetType, T: QNetworkAddressEntry_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_swap<RetType> {
  fn swap(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  void QNetworkAddressEntry::swap(QNetworkAddressEntry & other);
impl<'a> /*trait*/ QNetworkAddressEntry_swap<()> for (&'a QNetworkAddressEntry) {
  fn swap(self , rsthis: & QNetworkAddressEntry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntry4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkAddressEntry4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::setPrefixLength(int length);
impl /*struct*/ QNetworkAddressEntry {
  pub fn setPrefixLength<RetType, T: QNetworkAddressEntry_setPrefixLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPrefixLength(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_setPrefixLength<RetType> {
  fn setPrefixLength(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  void QNetworkAddressEntry::setPrefixLength(int length);
impl<'a> /*trait*/ QNetworkAddressEntry_setPrefixLength<()> for (i32) {
  fn setPrefixLength(self , rsthis: & QNetworkAddressEntry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntry15setPrefixLengthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QNetworkAddressEntry15setPrefixLengthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QNetworkAddressEntry::prefixLength();
impl /*struct*/ QNetworkAddressEntry {
  pub fn prefixLength<RetType, T: QNetworkAddressEntry_prefixLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prefixLength(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_prefixLength<RetType> {
  fn prefixLength(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  int QNetworkAddressEntry::prefixLength();
impl<'a> /*trait*/ QNetworkAddressEntry_prefixLength<i32> for () {
  fn prefixLength(self , rsthis: & QNetworkAddressEntry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QNetworkAddressEntry12prefixLengthEv()};
    let mut ret = unsafe {_ZNK20QNetworkAddressEntry12prefixLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QHostAddress QNetworkAddressEntry::netmask();
impl /*struct*/ QNetworkAddressEntry {
  pub fn netmask<RetType, T: QNetworkAddressEntry_netmask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.netmask(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_netmask<RetType> {
  fn netmask(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  QHostAddress QNetworkAddressEntry::netmask();
impl<'a> /*trait*/ QNetworkAddressEntry_netmask<QHostAddress> for () {
  fn netmask(self , rsthis: & QNetworkAddressEntry) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QNetworkAddressEntry7netmaskEv()};
    let mut ret = unsafe {_ZNK20QNetworkAddressEntry7netmaskEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::setBroadcast(const QHostAddress & newBroadcast);
impl /*struct*/ QNetworkAddressEntry {
  pub fn setBroadcast<RetType, T: QNetworkAddressEntry_setBroadcast<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBroadcast(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_setBroadcast<RetType> {
  fn setBroadcast(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  void QNetworkAddressEntry::setBroadcast(const QHostAddress & newBroadcast);
impl<'a> /*trait*/ QNetworkAddressEntry_setBroadcast<()> for (&'a QHostAddress) {
  fn setBroadcast(self , rsthis: & QNetworkAddressEntry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntry12setBroadcastERK12QHostAddress()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkAddressEntry12setBroadcastERK12QHostAddress(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::QNetworkAddressEntry();
impl /*struct*/ QNetworkAddressEntry {
  pub fn new<T: QNetworkAddressEntry_new>(value: T) -> QNetworkAddressEntry {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkAddressEntry_new {
  fn new(self) -> QNetworkAddressEntry;
}

  // proto:  void QNetworkAddressEntry::QNetworkAddressEntry();
impl<'a> /*trait*/ QNetworkAddressEntry_new for () {
  fn new(self) -> QNetworkAddressEntry {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntryC2Ev()};
    let ctysz: c_int = unsafe{QNetworkAddressEntry_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN20QNetworkAddressEntryC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkAddressEntry{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::setIp(const QHostAddress & newIp);
impl /*struct*/ QNetworkAddressEntry {
  pub fn setIp<RetType, T: QNetworkAddressEntry_setIp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIp(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_setIp<RetType> {
  fn setIp(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  void QNetworkAddressEntry::setIp(const QHostAddress & newIp);
impl<'a> /*trait*/ QNetworkAddressEntry_setIp<()> for (&'a QHostAddress) {
  fn setIp(self , rsthis: & QNetworkAddressEntry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntry5setIpERK12QHostAddress()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkAddressEntry5setIpERK12QHostAddress(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::setNetmask(const QHostAddress & newNetmask);
impl /*struct*/ QNetworkAddressEntry {
  pub fn setNetmask<RetType, T: QNetworkAddressEntry_setNetmask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNetmask(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_setNetmask<RetType> {
  fn setNetmask(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  void QNetworkAddressEntry::setNetmask(const QHostAddress & newNetmask);
impl<'a> /*trait*/ QNetworkAddressEntry_setNetmask<()> for (&'a QHostAddress) {
  fn setNetmask(self , rsthis: & QNetworkAddressEntry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntry10setNetmaskERK12QHostAddress()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QNetworkAddressEntry10setNetmaskERK12QHostAddress(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::QNetworkAddressEntry(const QNetworkAddressEntry & other);
impl<'a> /*trait*/ QNetworkAddressEntry_new for (&'a QNetworkAddressEntry) {
  fn new(self) -> QNetworkAddressEntry {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntryC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkAddressEntry_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QNetworkAddressEntryC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkAddressEntry{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QHostAddress QNetworkAddressEntry::broadcast();
impl /*struct*/ QNetworkAddressEntry {
  pub fn broadcast<RetType, T: QNetworkAddressEntry_broadcast<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.broadcast(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_broadcast<RetType> {
  fn broadcast(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  QHostAddress QNetworkAddressEntry::broadcast();
impl<'a> /*trait*/ QNetworkAddressEntry_broadcast<QHostAddress> for () {
  fn broadcast(self , rsthis: & QNetworkAddressEntry) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QNetworkAddressEntry9broadcastEv()};
    let mut ret = unsafe {_ZNK20QNetworkAddressEntry9broadcastEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkAddressEntry::~QNetworkAddressEntry();
impl /*struct*/ QNetworkAddressEntry {
  pub fn free<RetType, T: QNetworkAddressEntry_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_free<RetType> {
  fn free(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  void QNetworkAddressEntry::~QNetworkAddressEntry();
impl<'a> /*trait*/ QNetworkAddressEntry_free<()> for () {
  fn free(self , rsthis: & QNetworkAddressEntry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QNetworkAddressEntryD2Ev()};
     unsafe {_ZN20QNetworkAddressEntryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QHostAddress QNetworkAddressEntry::ip();
impl /*struct*/ QNetworkAddressEntry {
  pub fn ip<RetType, T: QNetworkAddressEntry_ip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ip(self);
    // return 1;
  }
}

pub trait QNetworkAddressEntry_ip<RetType> {
  fn ip(self , rsthis: & QNetworkAddressEntry) -> RetType;
}

  // proto:  QHostAddress QNetworkAddressEntry::ip();
impl<'a> /*trait*/ QNetworkAddressEntry_ip<QHostAddress> for () {
  fn ip(self , rsthis: & QNetworkAddressEntry) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QNetworkAddressEntry2ipEv()};
    let mut ret = unsafe {_ZNK20QNetworkAddressEntry2ipEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QNetworkInterface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkInterface {
    return QNetworkInterface{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QNetworkInterface QNetworkInterface::interfaceFromName(const QString & name);
impl /*struct*/ QNetworkInterface {
  pub fn interfaceFromName_s<RetType, T: QNetworkInterface_interfaceFromName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.interfaceFromName_s();
    // return 1;
  }
}

pub trait QNetworkInterface_interfaceFromName_s<RetType> {
  fn interfaceFromName_s(self ) -> RetType;
}

  // proto: static QNetworkInterface QNetworkInterface::interfaceFromName(const QString & name);
impl<'a> /*trait*/ QNetworkInterface_interfaceFromName_s<QNetworkInterface> for (&'a QString) {
  fn interfaceFromName_s(self ) -> QNetworkInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterface17interfaceFromNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkInterface17interfaceFromNameERK7QString(arg0)};
    let mut ret1 = QNetworkInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkInterface::QNetworkInterface();
impl /*struct*/ QNetworkInterface {
  pub fn new<T: QNetworkInterface_new>(value: T) -> QNetworkInterface {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkInterface_new {
  fn new(self) -> QNetworkInterface;
}

  // proto:  void QNetworkInterface::QNetworkInterface();
impl<'a> /*trait*/ QNetworkInterface_new for () {
  fn new(self) -> QNetworkInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterfaceC2Ev()};
    let ctysz: c_int = unsafe{QNetworkInterface_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QNetworkInterfaceC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkInterface{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QNetworkInterface::hardwareAddress();
impl /*struct*/ QNetworkInterface {
  pub fn hardwareAddress<RetType, T: QNetworkInterface_hardwareAddress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hardwareAddress(self);
    // return 1;
  }
}

pub trait QNetworkInterface_hardwareAddress<RetType> {
  fn hardwareAddress(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  QString QNetworkInterface::hardwareAddress();
impl<'a> /*trait*/ QNetworkInterface_hardwareAddress<QString> for () {
  fn hardwareAddress(self , rsthis: & QNetworkInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkInterface15hardwareAddressEv()};
    let mut ret = unsafe {_ZNK17QNetworkInterface15hardwareAddressEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QNetworkInterface::name();
impl /*struct*/ QNetworkInterface {
  pub fn name<RetType, T: QNetworkInterface_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QNetworkInterface_name<RetType> {
  fn name(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  QString QNetworkInterface::name();
impl<'a> /*trait*/ QNetworkInterface_name<QString> for () {
  fn name(self , rsthis: & QNetworkInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkInterface4nameEv()};
    let mut ret = unsafe {_ZNK17QNetworkInterface4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkInterface::swap(QNetworkInterface & other);
impl /*struct*/ QNetworkInterface {
  pub fn swap<RetType, T: QNetworkInterface_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkInterface_swap<RetType> {
  fn swap(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  void QNetworkInterface::swap(QNetworkInterface & other);
impl<'a> /*trait*/ QNetworkInterface_swap<()> for (&'a QNetworkInterface) {
  fn swap(self , rsthis: & QNetworkInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterface4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QNetworkInterface4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QList<QHostAddress> QNetworkInterface::allAddresses();
impl /*struct*/ QNetworkInterface {
  pub fn allAddresses_s<RetType, T: QNetworkInterface_allAddresses_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.allAddresses_s();
    // return 1;
  }
}

pub trait QNetworkInterface_allAddresses_s<RetType> {
  fn allAddresses_s(self ) -> RetType;
}

  // proto: static QList<QHostAddress> QNetworkInterface::allAddresses();
impl<'a> /*trait*/ QNetworkInterface_allAddresses_s<()> for () {
  fn allAddresses_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterface12allAddressesEv()};
     unsafe {_ZN17QNetworkInterface12allAddressesEv()};
    // return 1;
  }
}

  // proto:  QString QNetworkInterface::humanReadableName();
impl /*struct*/ QNetworkInterface {
  pub fn humanReadableName<RetType, T: QNetworkInterface_humanReadableName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.humanReadableName(self);
    // return 1;
  }
}

pub trait QNetworkInterface_humanReadableName<RetType> {
  fn humanReadableName(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  QString QNetworkInterface::humanReadableName();
impl<'a> /*trait*/ QNetworkInterface_humanReadableName<QString> for () {
  fn humanReadableName(self , rsthis: & QNetworkInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkInterface17humanReadableNameEv()};
    let mut ret = unsafe {_ZNK17QNetworkInterface17humanReadableNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QNetworkInterface::index();
impl /*struct*/ QNetworkInterface {
  pub fn index<RetType, T: QNetworkInterface_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QNetworkInterface_index<RetType> {
  fn index(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  int QNetworkInterface::index();
impl<'a> /*trait*/ QNetworkInterface_index<i32> for () {
  fn index(self , rsthis: & QNetworkInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkInterface5indexEv()};
    let mut ret = unsafe {_ZNK17QNetworkInterface5indexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QNetworkInterface::isValid();
impl /*struct*/ QNetworkInterface {
  pub fn isValid<RetType, T: QNetworkInterface_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QNetworkInterface_isValid<RetType> {
  fn isValid(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  bool QNetworkInterface::isValid();
impl<'a> /*trait*/ QNetworkInterface_isValid<i8> for () {
  fn isValid(self , rsthis: & QNetworkInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkInterface7isValidEv()};
    let mut ret = unsafe {_ZNK17QNetworkInterface7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkInterface::QNetworkInterface(const QNetworkInterface & other);
impl<'a> /*trait*/ QNetworkInterface_new for (&'a QNetworkInterface) {
  fn new(self) -> QNetworkInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterfaceC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkInterface_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QNetworkInterfaceC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkInterface{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QList<QNetworkInterface> QNetworkInterface::allInterfaces();
impl /*struct*/ QNetworkInterface {
  pub fn allInterfaces_s<RetType, T: QNetworkInterface_allInterfaces_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.allInterfaces_s();
    // return 1;
  }
}

pub trait QNetworkInterface_allInterfaces_s<RetType> {
  fn allInterfaces_s(self ) -> RetType;
}

  // proto: static QList<QNetworkInterface> QNetworkInterface::allInterfaces();
impl<'a> /*trait*/ QNetworkInterface_allInterfaces_s<()> for () {
  fn allInterfaces_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterface13allInterfacesEv()};
     unsafe {_ZN17QNetworkInterface13allInterfacesEv()};
    // return 1;
  }
}

  // proto:  QList<QNetworkAddressEntry> QNetworkInterface::addressEntries();
impl /*struct*/ QNetworkInterface {
  pub fn addressEntries<RetType, T: QNetworkInterface_addressEntries<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addressEntries(self);
    // return 1;
  }
}

pub trait QNetworkInterface_addressEntries<RetType> {
  fn addressEntries(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  QList<QNetworkAddressEntry> QNetworkInterface::addressEntries();
impl<'a> /*trait*/ QNetworkInterface_addressEntries<()> for () {
  fn addressEntries(self , rsthis: & QNetworkInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkInterface14addressEntriesEv()};
     unsafe {_ZNK17QNetworkInterface14addressEntriesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkInterface::~QNetworkInterface();
impl /*struct*/ QNetworkInterface {
  pub fn free<RetType, T: QNetworkInterface_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkInterface_free<RetType> {
  fn free(self , rsthis: & QNetworkInterface) -> RetType;
}

  // proto:  void QNetworkInterface::~QNetworkInterface();
impl<'a> /*trait*/ QNetworkInterface_free<()> for () {
  fn free(self , rsthis: & QNetworkInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterfaceD2Ev()};
     unsafe {_ZN17QNetworkInterfaceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QNetworkInterface QNetworkInterface::interfaceFromIndex(int index);
impl /*struct*/ QNetworkInterface {
  pub fn interfaceFromIndex_s<RetType, T: QNetworkInterface_interfaceFromIndex_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.interfaceFromIndex_s();
    // return 1;
  }
}

pub trait QNetworkInterface_interfaceFromIndex_s<RetType> {
  fn interfaceFromIndex_s(self ) -> RetType;
}

  // proto: static QNetworkInterface QNetworkInterface::interfaceFromIndex(int index);
impl<'a> /*trait*/ QNetworkInterface_interfaceFromIndex_s<QNetworkInterface> for (i32) {
  fn interfaceFromIndex_s(self ) -> QNetworkInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkInterface18interfaceFromIndexEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN17QNetworkInterface18interfaceFromIndexEi(arg0)};
    let mut ret1 = QNetworkInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

