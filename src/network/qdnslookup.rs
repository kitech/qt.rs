// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qdnslookup.h
// dst-file: /src/network/qdnslookup.rs
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
use super::super::core::qstring::QString; // 771
use super::qhostaddress::QHostAddress; // 773
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDnsTextRecord_Class_Size() -> c_int;
  // proto:  QString QDnsTextRecord::name();
  fn _ZNK14QDnsTextRecord4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QByteArray> QDnsTextRecord::values();
  fn _ZNK14QDnsTextRecord6valuesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsTextRecord::QDnsTextRecord();
  fn _ZN14QDnsTextRecordC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsTextRecord::QDnsTextRecord(const QDnsTextRecord & other);
  fn _ZN14QDnsTextRecordC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  quint32 QDnsTextRecord::timeToLive();
  fn _ZNK14QDnsTextRecord10timeToLiveEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QDnsTextRecord::swap(QDnsTextRecord & other);
  fn _ZN14QDnsTextRecord4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDnsTextRecord::~QDnsTextRecord();
  fn _ZN14QDnsTextRecordD2Ev(qthis: u64 /* *mut c_void*/);
  fn QDnsDomainNameRecord_Class_Size() -> c_int;
  // proto:  quint32 QDnsDomainNameRecord::timeToLive();
  fn _ZNK20QDnsDomainNameRecord10timeToLiveEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QDnsDomainNameRecord::QDnsDomainNameRecord();
  fn _ZN20QDnsDomainNameRecordC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QDnsDomainNameRecord::value();
  fn _ZNK20QDnsDomainNameRecord5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDnsDomainNameRecord::swap(QDnsDomainNameRecord & other);
  fn _ZN20QDnsDomainNameRecord4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDnsDomainNameRecord::~QDnsDomainNameRecord();
  fn _ZN20QDnsDomainNameRecordD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsDomainNameRecord::QDnsDomainNameRecord(const QDnsDomainNameRecord & other);
  fn _ZN20QDnsDomainNameRecordC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QDnsDomainNameRecord::name();
  fn _ZNK20QDnsDomainNameRecord4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QDnsHostAddressRecord_Class_Size() -> c_int;
  // proto:  QHostAddress QDnsHostAddressRecord::value();
  fn _ZNK21QDnsHostAddressRecord5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDnsHostAddressRecord::QDnsHostAddressRecord();
  fn _ZN21QDnsHostAddressRecordC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QDnsHostAddressRecord::name();
  fn _ZNK21QDnsHostAddressRecord4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDnsHostAddressRecord::~QDnsHostAddressRecord();
  fn _ZN21QDnsHostAddressRecordD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  quint32 QDnsHostAddressRecord::timeToLive();
  fn _ZNK21QDnsHostAddressRecord10timeToLiveEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QDnsHostAddressRecord::swap(QDnsHostAddressRecord & other);
  fn _ZN21QDnsHostAddressRecord4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDnsHostAddressRecord::QDnsHostAddressRecord(const QDnsHostAddressRecord & other);
  fn _ZN21QDnsHostAddressRecordC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QDnsMailExchangeRecord_Class_Size() -> c_int;
  // proto:  QString QDnsMailExchangeRecord::exchange();
  fn _ZNK22QDnsMailExchangeRecord8exchangeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDnsMailExchangeRecord::QDnsMailExchangeRecord(const QDnsMailExchangeRecord & other);
  fn _ZN22QDnsMailExchangeRecordC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDnsMailExchangeRecord::~QDnsMailExchangeRecord();
  fn _ZN22QDnsMailExchangeRecordD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  quint16 QDnsMailExchangeRecord::preference();
  fn _ZNK22QDnsMailExchangeRecord10preferenceEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  quint32 QDnsMailExchangeRecord::timeToLive();
  fn _ZNK22QDnsMailExchangeRecord10timeToLiveEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  QString QDnsMailExchangeRecord::name();
  fn _ZNK22QDnsMailExchangeRecord4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDnsMailExchangeRecord::QDnsMailExchangeRecord();
  fn _ZN22QDnsMailExchangeRecordC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsMailExchangeRecord::swap(QDnsMailExchangeRecord & other);
  fn _ZN22QDnsMailExchangeRecord4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QDnsServiceRecord_Class_Size() -> c_int;
  // proto:  void QDnsServiceRecord::QDnsServiceRecord(const QDnsServiceRecord & other);
  fn _ZN17QDnsServiceRecordC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  quint32 QDnsServiceRecord::timeToLive();
  fn _ZNK17QDnsServiceRecord10timeToLiveEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  QString QDnsServiceRecord::target();
  fn _ZNK17QDnsServiceRecord6targetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDnsServiceRecord::swap(QDnsServiceRecord & other);
  fn _ZN17QDnsServiceRecord4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QDnsServiceRecord::name();
  fn _ZNK17QDnsServiceRecord4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  quint16 QDnsServiceRecord::weight();
  fn _ZNK17QDnsServiceRecord6weightEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  void QDnsServiceRecord::~QDnsServiceRecord();
  fn _ZN17QDnsServiceRecordD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsServiceRecord::QDnsServiceRecord();
  fn _ZN17QDnsServiceRecordC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  quint16 QDnsServiceRecord::port();
  fn _ZNK17QDnsServiceRecord4portEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  quint16 QDnsServiceRecord::priority();
  fn _ZNK17QDnsServiceRecord8priorityEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  fn QDnsLookup_Class_Size() -> c_int;
  // proto:  QString QDnsLookup::name();
  fn _ZNK10QDnsLookup4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::pointerRecords();
  fn _ZNK10QDnsLookup14pointerRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QHostAddress QDnsLookup::nameserver();
  fn _ZNK10QDnsLookup10nameserverEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::nameServerRecords();
  fn _ZNK10QDnsLookup17nameServerRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsLookup::abort();
  fn _ZN10QDnsLookup5abortEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QDnsHostAddressRecord> QDnsLookup::hostAddressRecords();
  fn _ZNK10QDnsLookup18hostAddressRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsLookup::lookup();
  fn _ZN10QDnsLookup6lookupEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QDnsTextRecord> QDnsLookup::textRecords();
  fn _ZNK10QDnsLookup11textRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QDnsLookup::isFinished();
  fn _ZNK10QDnsLookup10isFinishedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QDnsLookup::setName(const QString & name);
  fn _ZN10QDnsLookup7setNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QDnsServiceRecord> QDnsLookup::serviceRecords();
  fn _ZNK10QDnsLookup14serviceRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsLookup::setNameserver(const QHostAddress & nameserver);
  fn _ZN10QDnsLookup13setNameserverERK12QHostAddress(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::canonicalNameRecords();
  fn _ZNK10QDnsLookup20canonicalNameRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QDnsMailExchangeRecord> QDnsLookup::mailExchangeRecords();
  fn _ZNK10QDnsLookup19mailExchangeRecordsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QDnsLookup::errorString();
  fn _ZNK10QDnsLookup11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QDnsLookup::metaObject();
  fn _ZNK10QDnsLookup10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDnsLookup::QDnsLookup(QObject * parent);
  fn _ZN10QDnsLookupC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDnsLookup::~QDnsLookup();
  fn _ZN10QDnsLookupD2Ev(qthis: u64 /* *mut c_void*/);
  fn QDnsLookup_SlotProxy_connect__ZN10QDnsLookup11nameChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDnsLookup_SlotProxy_connect__ZN10QDnsLookup8finishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDnsLookup_SlotProxy_connect__ZN10QDnsLookup11typeChangedENS_4TypeE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDnsLookup_SlotProxy_connect__ZN10QDnsLookup17nameserverChangedERK12QHostAddress(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDnsTextRecord)=1
#[derive(Default)]
pub struct QDnsTextRecord {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDnsDomainNameRecord)=1
#[derive(Default)]
pub struct QDnsDomainNameRecord {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDnsHostAddressRecord)=1
#[derive(Default)]
pub struct QDnsHostAddressRecord {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDnsMailExchangeRecord)=1
#[derive(Default)]
pub struct QDnsMailExchangeRecord {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDnsServiceRecord)=1
#[derive(Default)]
pub struct QDnsServiceRecord {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDnsLookup)=1
#[derive(Default)]
pub struct QDnsLookup {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _nameChanged: QDnsLookup_nameChanged_signal,
  pub _finished: QDnsLookup_finished_signal,
  pub _nameserverChanged: QDnsLookup_nameserverChanged_signal,
  pub _typeChanged: QDnsLookup_typeChanged_signal,
}

impl /*struct*/ QDnsTextRecord {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDnsTextRecord {
    return QDnsTextRecord{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QDnsTextRecord::name();
impl /*struct*/ QDnsTextRecord {
  pub fn name<RetType, T: QDnsTextRecord_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QDnsTextRecord_name<RetType> {
  fn name(self , rsthis: & QDnsTextRecord) -> RetType;
}

  // proto:  QString QDnsTextRecord::name();
impl<'a> /*trait*/ QDnsTextRecord_name<QString> for () {
  fn name(self , rsthis: & QDnsTextRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDnsTextRecord4nameEv()};
    let mut ret = unsafe {_ZNK14QDnsTextRecord4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QDnsTextRecord::values();
impl /*struct*/ QDnsTextRecord {
  pub fn values<RetType, T: QDnsTextRecord_values<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.values(self);
    // return 1;
  }
}

pub trait QDnsTextRecord_values<RetType> {
  fn values(self , rsthis: & QDnsTextRecord) -> RetType;
}

  // proto:  QList<QByteArray> QDnsTextRecord::values();
impl<'a> /*trait*/ QDnsTextRecord_values<()> for () {
  fn values(self , rsthis: & QDnsTextRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDnsTextRecord6valuesEv()};
     unsafe {_ZNK14QDnsTextRecord6valuesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsTextRecord::QDnsTextRecord();
impl /*struct*/ QDnsTextRecord {
  pub fn new<T: QDnsTextRecord_new>(value: T) -> QDnsTextRecord {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDnsTextRecord_new {
  fn new(self) -> QDnsTextRecord;
}

  // proto:  void QDnsTextRecord::QDnsTextRecord();
impl<'a> /*trait*/ QDnsTextRecord_new for () {
  fn new(self) -> QDnsTextRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDnsTextRecordC2Ev()};
    let ctysz: c_int = unsafe{QDnsTextRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN14QDnsTextRecordC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsTextRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDnsTextRecord::QDnsTextRecord(const QDnsTextRecord & other);
impl<'a> /*trait*/ QDnsTextRecord_new for (&'a QDnsTextRecord) {
  fn new(self) -> QDnsTextRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDnsTextRecordC2ERKS_()};
    let ctysz: c_int = unsafe{QDnsTextRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QDnsTextRecordC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsTextRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  quint32 QDnsTextRecord::timeToLive();
impl /*struct*/ QDnsTextRecord {
  pub fn timeToLive<RetType, T: QDnsTextRecord_timeToLive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeToLive(self);
    // return 1;
  }
}

pub trait QDnsTextRecord_timeToLive<RetType> {
  fn timeToLive(self , rsthis: & QDnsTextRecord) -> RetType;
}

  // proto:  quint32 QDnsTextRecord::timeToLive();
impl<'a> /*trait*/ QDnsTextRecord_timeToLive<u32> for () {
  fn timeToLive(self , rsthis: & QDnsTextRecord) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDnsTextRecord10timeToLiveEv()};
    let mut ret = unsafe {_ZNK14QDnsTextRecord10timeToLiveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QDnsTextRecord::swap(QDnsTextRecord & other);
impl /*struct*/ QDnsTextRecord {
  pub fn swap<RetType, T: QDnsTextRecord_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDnsTextRecord_swap<RetType> {
  fn swap(self , rsthis: & QDnsTextRecord) -> RetType;
}

  // proto:  void QDnsTextRecord::swap(QDnsTextRecord & other);
impl<'a> /*trait*/ QDnsTextRecord_swap<()> for (&'a QDnsTextRecord) {
  fn swap(self , rsthis: & QDnsTextRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDnsTextRecord4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDnsTextRecord4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDnsTextRecord::~QDnsTextRecord();
impl /*struct*/ QDnsTextRecord {
  pub fn free<RetType, T: QDnsTextRecord_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDnsTextRecord_free<RetType> {
  fn free(self , rsthis: & QDnsTextRecord) -> RetType;
}

  // proto:  void QDnsTextRecord::~QDnsTextRecord();
impl<'a> /*trait*/ QDnsTextRecord_free<()> for () {
  fn free(self , rsthis: & QDnsTextRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDnsTextRecordD2Ev()};
     unsafe {_ZN14QDnsTextRecordD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDnsDomainNameRecord {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDnsDomainNameRecord {
    return QDnsDomainNameRecord{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  quint32 QDnsDomainNameRecord::timeToLive();
impl /*struct*/ QDnsDomainNameRecord {
  pub fn timeToLive<RetType, T: QDnsDomainNameRecord_timeToLive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeToLive(self);
    // return 1;
  }
}

pub trait QDnsDomainNameRecord_timeToLive<RetType> {
  fn timeToLive(self , rsthis: & QDnsDomainNameRecord) -> RetType;
}

  // proto:  quint32 QDnsDomainNameRecord::timeToLive();
impl<'a> /*trait*/ QDnsDomainNameRecord_timeToLive<u32> for () {
  fn timeToLive(self , rsthis: & QDnsDomainNameRecord) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDnsDomainNameRecord10timeToLiveEv()};
    let mut ret = unsafe {_ZNK20QDnsDomainNameRecord10timeToLiveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QDnsDomainNameRecord::QDnsDomainNameRecord();
impl /*struct*/ QDnsDomainNameRecord {
  pub fn new<T: QDnsDomainNameRecord_new>(value: T) -> QDnsDomainNameRecord {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDnsDomainNameRecord_new {
  fn new(self) -> QDnsDomainNameRecord;
}

  // proto:  void QDnsDomainNameRecord::QDnsDomainNameRecord();
impl<'a> /*trait*/ QDnsDomainNameRecord_new for () {
  fn new(self) -> QDnsDomainNameRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDnsDomainNameRecordC2Ev()};
    let ctysz: c_int = unsafe{QDnsDomainNameRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN20QDnsDomainNameRecordC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsDomainNameRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QDnsDomainNameRecord::value();
impl /*struct*/ QDnsDomainNameRecord {
  pub fn value<RetType, T: QDnsDomainNameRecord_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QDnsDomainNameRecord_value<RetType> {
  fn value(self , rsthis: & QDnsDomainNameRecord) -> RetType;
}

  // proto:  QString QDnsDomainNameRecord::value();
impl<'a> /*trait*/ QDnsDomainNameRecord_value<QString> for () {
  fn value(self , rsthis: & QDnsDomainNameRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDnsDomainNameRecord5valueEv()};
    let mut ret = unsafe {_ZNK20QDnsDomainNameRecord5valueEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDnsDomainNameRecord::swap(QDnsDomainNameRecord & other);
impl /*struct*/ QDnsDomainNameRecord {
  pub fn swap<RetType, T: QDnsDomainNameRecord_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDnsDomainNameRecord_swap<RetType> {
  fn swap(self , rsthis: & QDnsDomainNameRecord) -> RetType;
}

  // proto:  void QDnsDomainNameRecord::swap(QDnsDomainNameRecord & other);
impl<'a> /*trait*/ QDnsDomainNameRecord_swap<()> for (&'a QDnsDomainNameRecord) {
  fn swap(self , rsthis: & QDnsDomainNameRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDnsDomainNameRecord4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QDnsDomainNameRecord4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDnsDomainNameRecord::~QDnsDomainNameRecord();
impl /*struct*/ QDnsDomainNameRecord {
  pub fn free<RetType, T: QDnsDomainNameRecord_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDnsDomainNameRecord_free<RetType> {
  fn free(self , rsthis: & QDnsDomainNameRecord) -> RetType;
}

  // proto:  void QDnsDomainNameRecord::~QDnsDomainNameRecord();
impl<'a> /*trait*/ QDnsDomainNameRecord_free<()> for () {
  fn free(self , rsthis: & QDnsDomainNameRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDnsDomainNameRecordD2Ev()};
     unsafe {_ZN20QDnsDomainNameRecordD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsDomainNameRecord::QDnsDomainNameRecord(const QDnsDomainNameRecord & other);
impl<'a> /*trait*/ QDnsDomainNameRecord_new for (&'a QDnsDomainNameRecord) {
  fn new(self) -> QDnsDomainNameRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDnsDomainNameRecordC2ERKS_()};
    let ctysz: c_int = unsafe{QDnsDomainNameRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QDnsDomainNameRecordC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsDomainNameRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QDnsDomainNameRecord::name();
impl /*struct*/ QDnsDomainNameRecord {
  pub fn name<RetType, T: QDnsDomainNameRecord_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QDnsDomainNameRecord_name<RetType> {
  fn name(self , rsthis: & QDnsDomainNameRecord) -> RetType;
}

  // proto:  QString QDnsDomainNameRecord::name();
impl<'a> /*trait*/ QDnsDomainNameRecord_name<QString> for () {
  fn name(self , rsthis: & QDnsDomainNameRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDnsDomainNameRecord4nameEv()};
    let mut ret = unsafe {_ZNK20QDnsDomainNameRecord4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDnsHostAddressRecord {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDnsHostAddressRecord {
    return QDnsHostAddressRecord{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QHostAddress QDnsHostAddressRecord::value();
impl /*struct*/ QDnsHostAddressRecord {
  pub fn value<RetType, T: QDnsHostAddressRecord_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QDnsHostAddressRecord_value<RetType> {
  fn value(self , rsthis: & QDnsHostAddressRecord) -> RetType;
}

  // proto:  QHostAddress QDnsHostAddressRecord::value();
impl<'a> /*trait*/ QDnsHostAddressRecord_value<QHostAddress> for () {
  fn value(self , rsthis: & QDnsHostAddressRecord) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QDnsHostAddressRecord5valueEv()};
    let mut ret = unsafe {_ZNK21QDnsHostAddressRecord5valueEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDnsHostAddressRecord::QDnsHostAddressRecord();
impl /*struct*/ QDnsHostAddressRecord {
  pub fn new<T: QDnsHostAddressRecord_new>(value: T) -> QDnsHostAddressRecord {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDnsHostAddressRecord_new {
  fn new(self) -> QDnsHostAddressRecord;
}

  // proto:  void QDnsHostAddressRecord::QDnsHostAddressRecord();
impl<'a> /*trait*/ QDnsHostAddressRecord_new for () {
  fn new(self) -> QDnsHostAddressRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QDnsHostAddressRecordC2Ev()};
    let ctysz: c_int = unsafe{QDnsHostAddressRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN21QDnsHostAddressRecordC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsHostAddressRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QDnsHostAddressRecord::name();
impl /*struct*/ QDnsHostAddressRecord {
  pub fn name<RetType, T: QDnsHostAddressRecord_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QDnsHostAddressRecord_name<RetType> {
  fn name(self , rsthis: & QDnsHostAddressRecord) -> RetType;
}

  // proto:  QString QDnsHostAddressRecord::name();
impl<'a> /*trait*/ QDnsHostAddressRecord_name<QString> for () {
  fn name(self , rsthis: & QDnsHostAddressRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QDnsHostAddressRecord4nameEv()};
    let mut ret = unsafe {_ZNK21QDnsHostAddressRecord4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDnsHostAddressRecord::~QDnsHostAddressRecord();
impl /*struct*/ QDnsHostAddressRecord {
  pub fn free<RetType, T: QDnsHostAddressRecord_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDnsHostAddressRecord_free<RetType> {
  fn free(self , rsthis: & QDnsHostAddressRecord) -> RetType;
}

  // proto:  void QDnsHostAddressRecord::~QDnsHostAddressRecord();
impl<'a> /*trait*/ QDnsHostAddressRecord_free<()> for () {
  fn free(self , rsthis: & QDnsHostAddressRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QDnsHostAddressRecordD2Ev()};
     unsafe {_ZN21QDnsHostAddressRecordD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  quint32 QDnsHostAddressRecord::timeToLive();
impl /*struct*/ QDnsHostAddressRecord {
  pub fn timeToLive<RetType, T: QDnsHostAddressRecord_timeToLive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeToLive(self);
    // return 1;
  }
}

pub trait QDnsHostAddressRecord_timeToLive<RetType> {
  fn timeToLive(self , rsthis: & QDnsHostAddressRecord) -> RetType;
}

  // proto:  quint32 QDnsHostAddressRecord::timeToLive();
impl<'a> /*trait*/ QDnsHostAddressRecord_timeToLive<u32> for () {
  fn timeToLive(self , rsthis: & QDnsHostAddressRecord) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QDnsHostAddressRecord10timeToLiveEv()};
    let mut ret = unsafe {_ZNK21QDnsHostAddressRecord10timeToLiveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QDnsHostAddressRecord::swap(QDnsHostAddressRecord & other);
impl /*struct*/ QDnsHostAddressRecord {
  pub fn swap<RetType, T: QDnsHostAddressRecord_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDnsHostAddressRecord_swap<RetType> {
  fn swap(self , rsthis: & QDnsHostAddressRecord) -> RetType;
}

  // proto:  void QDnsHostAddressRecord::swap(QDnsHostAddressRecord & other);
impl<'a> /*trait*/ QDnsHostAddressRecord_swap<()> for (&'a QDnsHostAddressRecord) {
  fn swap(self , rsthis: & QDnsHostAddressRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QDnsHostAddressRecord4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QDnsHostAddressRecord4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDnsHostAddressRecord::QDnsHostAddressRecord(const QDnsHostAddressRecord & other);
impl<'a> /*trait*/ QDnsHostAddressRecord_new for (&'a QDnsHostAddressRecord) {
  fn new(self) -> QDnsHostAddressRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QDnsHostAddressRecordC2ERKS_()};
    let ctysz: c_int = unsafe{QDnsHostAddressRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QDnsHostAddressRecordC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsHostAddressRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDnsMailExchangeRecord {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDnsMailExchangeRecord {
    return QDnsMailExchangeRecord{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QDnsMailExchangeRecord::exchange();
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn exchange<RetType, T: QDnsMailExchangeRecord_exchange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exchange(self);
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_exchange<RetType> {
  fn exchange(self , rsthis: & QDnsMailExchangeRecord) -> RetType;
}

  // proto:  QString QDnsMailExchangeRecord::exchange();
impl<'a> /*trait*/ QDnsMailExchangeRecord_exchange<QString> for () {
  fn exchange(self , rsthis: & QDnsMailExchangeRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QDnsMailExchangeRecord8exchangeEv()};
    let mut ret = unsafe {_ZNK22QDnsMailExchangeRecord8exchangeEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDnsMailExchangeRecord::QDnsMailExchangeRecord(const QDnsMailExchangeRecord & other);
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn new<T: QDnsMailExchangeRecord_new>(value: T) -> QDnsMailExchangeRecord {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_new {
  fn new(self) -> QDnsMailExchangeRecord;
}

  // proto:  void QDnsMailExchangeRecord::QDnsMailExchangeRecord(const QDnsMailExchangeRecord & other);
impl<'a> /*trait*/ QDnsMailExchangeRecord_new for (&'a QDnsMailExchangeRecord) {
  fn new(self) -> QDnsMailExchangeRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QDnsMailExchangeRecordC2ERKS_()};
    let ctysz: c_int = unsafe{QDnsMailExchangeRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QDnsMailExchangeRecordC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsMailExchangeRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDnsMailExchangeRecord::~QDnsMailExchangeRecord();
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn free<RetType, T: QDnsMailExchangeRecord_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_free<RetType> {
  fn free(self , rsthis: & QDnsMailExchangeRecord) -> RetType;
}

  // proto:  void QDnsMailExchangeRecord::~QDnsMailExchangeRecord();
impl<'a> /*trait*/ QDnsMailExchangeRecord_free<()> for () {
  fn free(self , rsthis: & QDnsMailExchangeRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QDnsMailExchangeRecordD2Ev()};
     unsafe {_ZN22QDnsMailExchangeRecordD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  quint16 QDnsMailExchangeRecord::preference();
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn preference<RetType, T: QDnsMailExchangeRecord_preference<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preference(self);
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_preference<RetType> {
  fn preference(self , rsthis: & QDnsMailExchangeRecord) -> RetType;
}

  // proto:  quint16 QDnsMailExchangeRecord::preference();
impl<'a> /*trait*/ QDnsMailExchangeRecord_preference<u16> for () {
  fn preference(self , rsthis: & QDnsMailExchangeRecord) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QDnsMailExchangeRecord10preferenceEv()};
    let mut ret = unsafe {_ZNK22QDnsMailExchangeRecord10preferenceEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  quint32 QDnsMailExchangeRecord::timeToLive();
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn timeToLive<RetType, T: QDnsMailExchangeRecord_timeToLive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeToLive(self);
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_timeToLive<RetType> {
  fn timeToLive(self , rsthis: & QDnsMailExchangeRecord) -> RetType;
}

  // proto:  quint32 QDnsMailExchangeRecord::timeToLive();
impl<'a> /*trait*/ QDnsMailExchangeRecord_timeToLive<u32> for () {
  fn timeToLive(self , rsthis: & QDnsMailExchangeRecord) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QDnsMailExchangeRecord10timeToLiveEv()};
    let mut ret = unsafe {_ZNK22QDnsMailExchangeRecord10timeToLiveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QString QDnsMailExchangeRecord::name();
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn name<RetType, T: QDnsMailExchangeRecord_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_name<RetType> {
  fn name(self , rsthis: & QDnsMailExchangeRecord) -> RetType;
}

  // proto:  QString QDnsMailExchangeRecord::name();
impl<'a> /*trait*/ QDnsMailExchangeRecord_name<QString> for () {
  fn name(self , rsthis: & QDnsMailExchangeRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QDnsMailExchangeRecord4nameEv()};
    let mut ret = unsafe {_ZNK22QDnsMailExchangeRecord4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDnsMailExchangeRecord::QDnsMailExchangeRecord();
impl<'a> /*trait*/ QDnsMailExchangeRecord_new for () {
  fn new(self) -> QDnsMailExchangeRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QDnsMailExchangeRecordC2Ev()};
    let ctysz: c_int = unsafe{QDnsMailExchangeRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN22QDnsMailExchangeRecordC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsMailExchangeRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDnsMailExchangeRecord::swap(QDnsMailExchangeRecord & other);
impl /*struct*/ QDnsMailExchangeRecord {
  pub fn swap<RetType, T: QDnsMailExchangeRecord_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDnsMailExchangeRecord_swap<RetType> {
  fn swap(self , rsthis: & QDnsMailExchangeRecord) -> RetType;
}

  // proto:  void QDnsMailExchangeRecord::swap(QDnsMailExchangeRecord & other);
impl<'a> /*trait*/ QDnsMailExchangeRecord_swap<()> for (&'a QDnsMailExchangeRecord) {
  fn swap(self , rsthis: & QDnsMailExchangeRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QDnsMailExchangeRecord4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QDnsMailExchangeRecord4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDnsServiceRecord {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDnsServiceRecord {
    return QDnsServiceRecord{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QDnsServiceRecord::QDnsServiceRecord(const QDnsServiceRecord & other);
impl /*struct*/ QDnsServiceRecord {
  pub fn new<T: QDnsServiceRecord_new>(value: T) -> QDnsServiceRecord {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDnsServiceRecord_new {
  fn new(self) -> QDnsServiceRecord;
}

  // proto:  void QDnsServiceRecord::QDnsServiceRecord(const QDnsServiceRecord & other);
impl<'a> /*trait*/ QDnsServiceRecord_new for (&'a QDnsServiceRecord) {
  fn new(self) -> QDnsServiceRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDnsServiceRecordC2ERKS_()};
    let ctysz: c_int = unsafe{QDnsServiceRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QDnsServiceRecordC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsServiceRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  quint32 QDnsServiceRecord::timeToLive();
impl /*struct*/ QDnsServiceRecord {
  pub fn timeToLive<RetType, T: QDnsServiceRecord_timeToLive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeToLive(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_timeToLive<RetType> {
  fn timeToLive(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  quint32 QDnsServiceRecord::timeToLive();
impl<'a> /*trait*/ QDnsServiceRecord_timeToLive<u32> for () {
  fn timeToLive(self , rsthis: & QDnsServiceRecord) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDnsServiceRecord10timeToLiveEv()};
    let mut ret = unsafe {_ZNK17QDnsServiceRecord10timeToLiveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QString QDnsServiceRecord::target();
impl /*struct*/ QDnsServiceRecord {
  pub fn target<RetType, T: QDnsServiceRecord_target<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_target<RetType> {
  fn target(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  QString QDnsServiceRecord::target();
impl<'a> /*trait*/ QDnsServiceRecord_target<QString> for () {
  fn target(self , rsthis: & QDnsServiceRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDnsServiceRecord6targetEv()};
    let mut ret = unsafe {_ZNK17QDnsServiceRecord6targetEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDnsServiceRecord::swap(QDnsServiceRecord & other);
impl /*struct*/ QDnsServiceRecord {
  pub fn swap<RetType, T: QDnsServiceRecord_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_swap<RetType> {
  fn swap(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  void QDnsServiceRecord::swap(QDnsServiceRecord & other);
impl<'a> /*trait*/ QDnsServiceRecord_swap<()> for (&'a QDnsServiceRecord) {
  fn swap(self , rsthis: & QDnsServiceRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDnsServiceRecord4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDnsServiceRecord4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QDnsServiceRecord::name();
impl /*struct*/ QDnsServiceRecord {
  pub fn name<RetType, T: QDnsServiceRecord_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_name<RetType> {
  fn name(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  QString QDnsServiceRecord::name();
impl<'a> /*trait*/ QDnsServiceRecord_name<QString> for () {
  fn name(self , rsthis: & QDnsServiceRecord) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDnsServiceRecord4nameEv()};
    let mut ret = unsafe {_ZNK17QDnsServiceRecord4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  quint16 QDnsServiceRecord::weight();
impl /*struct*/ QDnsServiceRecord {
  pub fn weight<RetType, T: QDnsServiceRecord_weight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.weight(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_weight<RetType> {
  fn weight(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  quint16 QDnsServiceRecord::weight();
impl<'a> /*trait*/ QDnsServiceRecord_weight<u16> for () {
  fn weight(self , rsthis: & QDnsServiceRecord) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDnsServiceRecord6weightEv()};
    let mut ret = unsafe {_ZNK17QDnsServiceRecord6weightEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  void QDnsServiceRecord::~QDnsServiceRecord();
impl /*struct*/ QDnsServiceRecord {
  pub fn free<RetType, T: QDnsServiceRecord_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_free<RetType> {
  fn free(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  void QDnsServiceRecord::~QDnsServiceRecord();
impl<'a> /*trait*/ QDnsServiceRecord_free<()> for () {
  fn free(self , rsthis: & QDnsServiceRecord) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDnsServiceRecordD2Ev()};
     unsafe {_ZN17QDnsServiceRecordD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsServiceRecord::QDnsServiceRecord();
impl<'a> /*trait*/ QDnsServiceRecord_new for () {
  fn new(self) -> QDnsServiceRecord {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDnsServiceRecordC2Ev()};
    let ctysz: c_int = unsafe{QDnsServiceRecord_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QDnsServiceRecordC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsServiceRecord{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  quint16 QDnsServiceRecord::port();
impl /*struct*/ QDnsServiceRecord {
  pub fn port<RetType, T: QDnsServiceRecord_port<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.port(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_port<RetType> {
  fn port(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  quint16 QDnsServiceRecord::port();
impl<'a> /*trait*/ QDnsServiceRecord_port<u16> for () {
  fn port(self , rsthis: & QDnsServiceRecord) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDnsServiceRecord4portEv()};
    let mut ret = unsafe {_ZNK17QDnsServiceRecord4portEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  quint16 QDnsServiceRecord::priority();
impl /*struct*/ QDnsServiceRecord {
  pub fn priority<RetType, T: QDnsServiceRecord_priority<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.priority(self);
    // return 1;
  }
}

pub trait QDnsServiceRecord_priority<RetType> {
  fn priority(self , rsthis: & QDnsServiceRecord) -> RetType;
}

  // proto:  quint16 QDnsServiceRecord::priority();
impl<'a> /*trait*/ QDnsServiceRecord_priority<u16> for () {
  fn priority(self , rsthis: & QDnsServiceRecord) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDnsServiceRecord8priorityEv()};
    let mut ret = unsafe {_ZNK17QDnsServiceRecord8priorityEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

impl /*struct*/ QDnsLookup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDnsLookup {
    return QDnsLookup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDnsLookup {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QDnsLookup {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QString QDnsLookup::name();
impl /*struct*/ QDnsLookup {
  pub fn name<RetType, T: QDnsLookup_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QDnsLookup_name<RetType> {
  fn name(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QString QDnsLookup::name();
impl<'a> /*trait*/ QDnsLookup_name<QString> for () {
  fn name(self , rsthis: & QDnsLookup) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup4nameEv()};
    let mut ret = unsafe {_ZNK10QDnsLookup4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::pointerRecords();
impl /*struct*/ QDnsLookup {
  pub fn pointerRecords<RetType, T: QDnsLookup_pointerRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pointerRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_pointerRecords<RetType> {
  fn pointerRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::pointerRecords();
impl<'a> /*trait*/ QDnsLookup_pointerRecords<()> for () {
  fn pointerRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup14pointerRecordsEv()};
     unsafe {_ZNK10QDnsLookup14pointerRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QHostAddress QDnsLookup::nameserver();
impl /*struct*/ QDnsLookup {
  pub fn nameserver<RetType, T: QDnsLookup_nameserver<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameserver(self);
    // return 1;
  }
}

pub trait QDnsLookup_nameserver<RetType> {
  fn nameserver(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QHostAddress QDnsLookup::nameserver();
impl<'a> /*trait*/ QDnsLookup_nameserver<QHostAddress> for () {
  fn nameserver(self , rsthis: & QDnsLookup) -> QHostAddress {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup10nameserverEv()};
    let mut ret = unsafe {_ZNK10QDnsLookup10nameserverEv(rsthis.qclsinst)};
    let mut ret1 = QHostAddress::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::nameServerRecords();
impl /*struct*/ QDnsLookup {
  pub fn nameServerRecords<RetType, T: QDnsLookup_nameServerRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameServerRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_nameServerRecords<RetType> {
  fn nameServerRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::nameServerRecords();
impl<'a> /*trait*/ QDnsLookup_nameServerRecords<()> for () {
  fn nameServerRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup17nameServerRecordsEv()};
     unsafe {_ZNK10QDnsLookup17nameServerRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsLookup::abort();
impl /*struct*/ QDnsLookup {
  pub fn abort<RetType, T: QDnsLookup_abort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.abort(self);
    // return 1;
  }
}

pub trait QDnsLookup_abort<RetType> {
  fn abort(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  void QDnsLookup::abort();
impl<'a> /*trait*/ QDnsLookup_abort<()> for () {
  fn abort(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDnsLookup5abortEv()};
     unsafe {_ZN10QDnsLookup5abortEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QDnsHostAddressRecord> QDnsLookup::hostAddressRecords();
impl /*struct*/ QDnsLookup {
  pub fn hostAddressRecords<RetType, T: QDnsLookup_hostAddressRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hostAddressRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_hostAddressRecords<RetType> {
  fn hostAddressRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsHostAddressRecord> QDnsLookup::hostAddressRecords();
impl<'a> /*trait*/ QDnsLookup_hostAddressRecords<()> for () {
  fn hostAddressRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup18hostAddressRecordsEv()};
     unsafe {_ZNK10QDnsLookup18hostAddressRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsLookup::lookup();
impl /*struct*/ QDnsLookup {
  pub fn lookup<RetType, T: QDnsLookup_lookup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lookup(self);
    // return 1;
  }
}

pub trait QDnsLookup_lookup<RetType> {
  fn lookup(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  void QDnsLookup::lookup();
impl<'a> /*trait*/ QDnsLookup_lookup<()> for () {
  fn lookup(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDnsLookup6lookupEv()};
     unsafe {_ZN10QDnsLookup6lookupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QDnsTextRecord> QDnsLookup::textRecords();
impl /*struct*/ QDnsLookup {
  pub fn textRecords<RetType, T: QDnsLookup_textRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_textRecords<RetType> {
  fn textRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsTextRecord> QDnsLookup::textRecords();
impl<'a> /*trait*/ QDnsLookup_textRecords<()> for () {
  fn textRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup11textRecordsEv()};
     unsafe {_ZNK10QDnsLookup11textRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDnsLookup::isFinished();
impl /*struct*/ QDnsLookup {
  pub fn isFinished<RetType, T: QDnsLookup_isFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinished(self);
    // return 1;
  }
}

pub trait QDnsLookup_isFinished<RetType> {
  fn isFinished(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  bool QDnsLookup::isFinished();
impl<'a> /*trait*/ QDnsLookup_isFinished<i8> for () {
  fn isFinished(self , rsthis: & QDnsLookup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup10isFinishedEv()};
    let mut ret = unsafe {_ZNK10QDnsLookup10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDnsLookup::setName(const QString & name);
impl /*struct*/ QDnsLookup {
  pub fn setName<RetType, T: QDnsLookup_setName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setName(self);
    // return 1;
  }
}

pub trait QDnsLookup_setName<RetType> {
  fn setName(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  void QDnsLookup::setName(const QString & name);
impl<'a> /*trait*/ QDnsLookup_setName<()> for (&'a QString) {
  fn setName(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDnsLookup7setNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QDnsLookup7setNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QDnsServiceRecord> QDnsLookup::serviceRecords();
impl /*struct*/ QDnsLookup {
  pub fn serviceRecords<RetType, T: QDnsLookup_serviceRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.serviceRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_serviceRecords<RetType> {
  fn serviceRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsServiceRecord> QDnsLookup::serviceRecords();
impl<'a> /*trait*/ QDnsLookup_serviceRecords<()> for () {
  fn serviceRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup14serviceRecordsEv()};
     unsafe {_ZNK10QDnsLookup14serviceRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsLookup::setNameserver(const QHostAddress & nameserver);
impl /*struct*/ QDnsLookup {
  pub fn setNameserver<RetType, T: QDnsLookup_setNameserver<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNameserver(self);
    // return 1;
  }
}

pub trait QDnsLookup_setNameserver<RetType> {
  fn setNameserver(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  void QDnsLookup::setNameserver(const QHostAddress & nameserver);
impl<'a> /*trait*/ QDnsLookup_setNameserver<()> for (&'a QHostAddress) {
  fn setNameserver(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDnsLookup13setNameserverERK12QHostAddress()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QDnsLookup13setNameserverERK12QHostAddress(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::canonicalNameRecords();
impl /*struct*/ QDnsLookup {
  pub fn canonicalNameRecords<RetType, T: QDnsLookup_canonicalNameRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canonicalNameRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_canonicalNameRecords<RetType> {
  fn canonicalNameRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsDomainNameRecord> QDnsLookup::canonicalNameRecords();
impl<'a> /*trait*/ QDnsLookup_canonicalNameRecords<()> for () {
  fn canonicalNameRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup20canonicalNameRecordsEv()};
     unsafe {_ZNK10QDnsLookup20canonicalNameRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QDnsMailExchangeRecord> QDnsLookup::mailExchangeRecords();
impl /*struct*/ QDnsLookup {
  pub fn mailExchangeRecords<RetType, T: QDnsLookup_mailExchangeRecords<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mailExchangeRecords(self);
    // return 1;
  }
}

pub trait QDnsLookup_mailExchangeRecords<RetType> {
  fn mailExchangeRecords(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QList<QDnsMailExchangeRecord> QDnsLookup::mailExchangeRecords();
impl<'a> /*trait*/ QDnsLookup_mailExchangeRecords<()> for () {
  fn mailExchangeRecords(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup19mailExchangeRecordsEv()};
     unsafe {_ZNK10QDnsLookup19mailExchangeRecordsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QDnsLookup::errorString();
impl /*struct*/ QDnsLookup {
  pub fn errorString<RetType, T: QDnsLookup_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QDnsLookup_errorString<RetType> {
  fn errorString(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  QString QDnsLookup::errorString();
impl<'a> /*trait*/ QDnsLookup_errorString<QString> for () {
  fn errorString(self , rsthis: & QDnsLookup) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup11errorStringEv()};
    let mut ret = unsafe {_ZNK10QDnsLookup11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDnsLookup::metaObject();
impl /*struct*/ QDnsLookup {
  pub fn metaObject<RetType, T: QDnsLookup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDnsLookup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  const QMetaObject * QDnsLookup::metaObject();
impl<'a> /*trait*/ QDnsLookup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDnsLookup10metaObjectEv()};
     unsafe {_ZNK10QDnsLookup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDnsLookup::QDnsLookup(QObject * parent);
impl /*struct*/ QDnsLookup {
  pub fn new<T: QDnsLookup_new>(value: T) -> QDnsLookup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDnsLookup_new {
  fn new(self) -> QDnsLookup;
}

  // proto:  void QDnsLookup::QDnsLookup(QObject * parent);
impl<'a> /*trait*/ QDnsLookup_new for (&'a QObject) {
  fn new(self) -> QDnsLookup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDnsLookupC2EP7QObject()};
    let ctysz: c_int = unsafe{QDnsLookup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QDnsLookupC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDnsLookup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDnsLookup::~QDnsLookup();
impl /*struct*/ QDnsLookup {
  pub fn free<RetType, T: QDnsLookup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDnsLookup_free<RetType> {
  fn free(self , rsthis: & QDnsLookup) -> RetType;
}

  // proto:  void QDnsLookup::~QDnsLookup();
impl<'a> /*trait*/ QDnsLookup_free<()> for () {
  fn free(self , rsthis: & QDnsLookup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDnsLookupD2Ev()};
     unsafe {_ZN10QDnsLookupD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QDnsLookup_nameChanged
pub struct QDnsLookup_nameChanged_signal{poi:u64}
impl /* struct */ QDnsLookup {
  pub fn nameChanged(&self) -> QDnsLookup_nameChanged_signal {
     return QDnsLookup_nameChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDnsLookup_nameChanged_signal {
  pub fn connect<T: QDnsLookup_nameChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDnsLookup_nameChanged_signal_connect {
  fn connect(self, sigthis: QDnsLookup_nameChanged_signal);
}

#[derive(Default)] // for QDnsLookup_finished
pub struct QDnsLookup_finished_signal{poi:u64}
impl /* struct */ QDnsLookup {
  pub fn finished(&self) -> QDnsLookup_finished_signal {
     return QDnsLookup_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDnsLookup_finished_signal {
  pub fn connect<T: QDnsLookup_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDnsLookup_finished_signal_connect {
  fn connect(self, sigthis: QDnsLookup_finished_signal);
}

#[derive(Default)] // for QDnsLookup_nameserverChanged
pub struct QDnsLookup_nameserverChanged_signal{poi:u64}
impl /* struct */ QDnsLookup {
  pub fn nameserverChanged(&self) -> QDnsLookup_nameserverChanged_signal {
     return QDnsLookup_nameserverChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDnsLookup_nameserverChanged_signal {
  pub fn connect<T: QDnsLookup_nameserverChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDnsLookup_nameserverChanged_signal_connect {
  fn connect(self, sigthis: QDnsLookup_nameserverChanged_signal);
}

#[derive(Default)] // for QDnsLookup_typeChanged
pub struct QDnsLookup_typeChanged_signal{poi:u64}
impl /* struct */ QDnsLookup {
  pub fn typeChanged(&self) -> QDnsLookup_typeChanged_signal {
     return QDnsLookup_typeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDnsLookup_typeChanged_signal {
  pub fn connect<T: QDnsLookup_typeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDnsLookup_typeChanged_signal_connect {
  fn connect(self, sigthis: QDnsLookup_typeChanged_signal);
}

// nameChanged(const class QString &)
extern fn QDnsLookup_nameChanged_signal_connect_cb_0(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDnsLookup_nameChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDnsLookup_nameChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QDnsLookup_nameChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_nameChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup11nameChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDnsLookup_nameChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QDnsLookup_nameChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_nameChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup11nameChangedERK7QString(arg0, arg1, arg2)};
  }
}
// finished()
extern fn QDnsLookup_finished_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QDnsLookup_finished_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QDnsLookup_finished_signal_connect for fn() {
  fn connect(self, sigthis: QDnsLookup_finished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_finished_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup8finishedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDnsLookup_finished_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QDnsLookup_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_finished_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup8finishedEv(arg0, arg1, arg2)};
  }
}
// typeChanged(enum QDnsLookup::Type)
extern fn QDnsLookup_typeChanged_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QDnsLookup_typeChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDnsLookup_typeChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDnsLookup_typeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_typeChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup11typeChangedENS_4TypeE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDnsLookup_typeChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QDnsLookup_typeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_typeChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup11typeChangedENS_4TypeE(arg0, arg1, arg2)};
  }
}
// nameserverChanged(const class QHostAddress &)
extern fn QDnsLookup_nameserverChanged_signal_connect_cb_3(rsfptr:fn(QHostAddress), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QHostAddress::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDnsLookup_nameserverChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QHostAddress)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QHostAddress::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDnsLookup_nameserverChanged_signal_connect for fn(QHostAddress) {
  fn connect(self, sigthis: QDnsLookup_nameserverChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_nameserverChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup17nameserverChangedERK12QHostAddress(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDnsLookup_nameserverChanged_signal_connect for Box<Fn(QHostAddress)> {
  fn connect(self, sigthis: QDnsLookup_nameserverChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDnsLookup_nameserverChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDnsLookup_SlotProxy_connect__ZN10QDnsLookup17nameserverChangedERK12QHostAddress(arg0, arg1, arg2)};
  }
}
// <= body block end

