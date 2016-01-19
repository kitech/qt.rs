// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qabstractnetworkcache.h
// dst-file: /src/network/qabstractnetworkcache.rs
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
use super::super::core::qdatetime::QDateTime; // 771
use super::super::core::qurl::QUrl; // 771
use super::super::core::qobject::QObject; // 771
// use super::qabstractnetworkcache::QNetworkCacheMetaData; // 773
use super::super::core::qiodevice::QIODevice; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkCacheMetaData_Class_Size() -> c_int;
  // proto:  void QNetworkCacheMetaData::setLastModified(const QDateTime & dateTime);
  fn _ZN21QNetworkCacheMetaData15setLastModifiedERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QDateTime QNetworkCacheMetaData::lastModified();
  fn _ZNK21QNetworkCacheMetaData12lastModifiedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkCacheMetaData::QNetworkCacheMetaData(const QNetworkCacheMetaData & other);
  fn _ZN21QNetworkCacheMetaDataC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCacheMetaData::~QNetworkCacheMetaData();
  fn _ZN21QNetworkCacheMetaDataD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  RawHeaderList QNetworkCacheMetaData::rawHeaders();
  fn _ZNK21QNetworkCacheMetaData10rawHeadersEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkCacheMetaData::swap(QNetworkCacheMetaData & other);
  fn _ZN21QNetworkCacheMetaData4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QUrl QNetworkCacheMetaData::url();
  fn _ZNK21QNetworkCacheMetaData3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDateTime QNetworkCacheMetaData::expirationDate();
  fn _ZNK21QNetworkCacheMetaData14expirationDateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkCacheMetaData::setExpirationDate(const QDateTime & dateTime);
  fn _ZN21QNetworkCacheMetaData17setExpirationDateERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QNetworkCacheMetaData::saveToDisk();
  fn _ZNK21QNetworkCacheMetaData10saveToDiskEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QNetworkCacheMetaData::QNetworkCacheMetaData();
  fn _ZN21QNetworkCacheMetaDataC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkCacheMetaData::setUrl(const QUrl & url);
  fn _ZN21QNetworkCacheMetaData6setUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCacheMetaData::setSaveToDisk(bool allow);
  fn _ZN21QNetworkCacheMetaData13setSaveToDiskEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QNetworkCacheMetaData::isValid();
  fn _ZNK21QNetworkCacheMetaData7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QAbstractNetworkCache_Class_Size() -> c_int;
  // proto:  void QAbstractNetworkCache::~QAbstractNetworkCache();
  fn _ZN21QAbstractNetworkCacheD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkCacheMetaData QAbstractNetworkCache::metaData(const QUrl & url);
  fn _ZN21QAbstractNetworkCache8metaDataERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractNetworkCache::QAbstractNetworkCache(const QAbstractNetworkCache & );
  fn _ZN21QAbstractNetworkCacheC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractNetworkCache::metaObject();
  fn _ZNK21QAbstractNetworkCache10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QIODevice * QAbstractNetworkCache::data(const QUrl & url);
  fn _ZN21QAbstractNetworkCache4dataERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractNetworkCache::insert(QIODevice * device);
  fn _ZN21QAbstractNetworkCache6insertEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QAbstractNetworkCache::remove(const QUrl & url);
  fn _ZN21QAbstractNetworkCache6removeERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QIODevice * QAbstractNetworkCache::prepare(const QNetworkCacheMetaData & metaData);
  fn _ZN21QAbstractNetworkCache7prepareERK21QNetworkCacheMetaData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractNetworkCache::clear();
  fn _ZN21QAbstractNetworkCache5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractNetworkCache::QAbstractNetworkCache(QObject * parent);
  fn _ZN21QAbstractNetworkCacheC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qint64 QAbstractNetworkCache::cacheSize();
  fn _ZNK21QAbstractNetworkCache9cacheSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QAbstractNetworkCache::updateMetaData(const QNetworkCacheMetaData & metaData);
  fn _ZN21QAbstractNetworkCache14updateMetaDataERK21QNetworkCacheMetaData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkCacheMetaData)=1
#[derive(Default)]
pub struct QNetworkCacheMetaData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractNetworkCache)=1
#[derive(Default)]
pub struct QAbstractNetworkCache {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkCacheMetaData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkCacheMetaData {
    return QNetworkCacheMetaData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QNetworkCacheMetaData::setLastModified(const QDateTime & dateTime);
impl /*struct*/ QNetworkCacheMetaData {
  pub fn setLastModified<RetType, T: QNetworkCacheMetaData_setLastModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastModified(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_setLastModified<RetType> {
  fn setLastModified(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  void QNetworkCacheMetaData::setLastModified(const QDateTime & dateTime);
impl<'a> /*trait*/ QNetworkCacheMetaData_setLastModified<()> for (&'a QDateTime) {
  fn setLastModified(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaData15setLastModifiedERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkCacheMetaData15setLastModifiedERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDateTime QNetworkCacheMetaData::lastModified();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn lastModified<RetType, T: QNetworkCacheMetaData_lastModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastModified(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_lastModified<RetType> {
  fn lastModified(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  QDateTime QNetworkCacheMetaData::lastModified();
impl<'a> /*trait*/ QNetworkCacheMetaData_lastModified<QDateTime> for () {
  fn lastModified(self , rsthis: & QNetworkCacheMetaData) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkCacheMetaData12lastModifiedEv()};
    let mut ret = unsafe {_ZNK21QNetworkCacheMetaData12lastModifiedEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::QNetworkCacheMetaData(const QNetworkCacheMetaData & other);
impl /*struct*/ QNetworkCacheMetaData {
  pub fn new<T: QNetworkCacheMetaData_new>(value: T) -> QNetworkCacheMetaData {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_new {
  fn new(self) -> QNetworkCacheMetaData;
}

  // proto:  void QNetworkCacheMetaData::QNetworkCacheMetaData(const QNetworkCacheMetaData & other);
impl<'a> /*trait*/ QNetworkCacheMetaData_new for (&'a QNetworkCacheMetaData) {
  fn new(self) -> QNetworkCacheMetaData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaDataC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkCacheMetaData_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QNetworkCacheMetaDataC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkCacheMetaData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::~QNetworkCacheMetaData();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn free<RetType, T: QNetworkCacheMetaData_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_free<RetType> {
  fn free(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  void QNetworkCacheMetaData::~QNetworkCacheMetaData();
impl<'a> /*trait*/ QNetworkCacheMetaData_free<()> for () {
  fn free(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaDataD2Ev()};
     unsafe {_ZN21QNetworkCacheMetaDataD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  RawHeaderList QNetworkCacheMetaData::rawHeaders();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn rawHeaders<RetType, T: QNetworkCacheMetaData_rawHeaders<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawHeaders(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_rawHeaders<RetType> {
  fn rawHeaders(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  RawHeaderList QNetworkCacheMetaData::rawHeaders();
impl<'a> /*trait*/ QNetworkCacheMetaData_rawHeaders<()> for () {
  fn rawHeaders(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkCacheMetaData10rawHeadersEv()};
     unsafe {_ZNK21QNetworkCacheMetaData10rawHeadersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::swap(QNetworkCacheMetaData & other);
impl /*struct*/ QNetworkCacheMetaData {
  pub fn swap<RetType, T: QNetworkCacheMetaData_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_swap<RetType> {
  fn swap(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  void QNetworkCacheMetaData::swap(QNetworkCacheMetaData & other);
impl<'a> /*trait*/ QNetworkCacheMetaData_swap<()> for (&'a QNetworkCacheMetaData) {
  fn swap(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaData4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkCacheMetaData4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QNetworkCacheMetaData::url();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn url<RetType, T: QNetworkCacheMetaData_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_url<RetType> {
  fn url(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  QUrl QNetworkCacheMetaData::url();
impl<'a> /*trait*/ QNetworkCacheMetaData_url<QUrl> for () {
  fn url(self , rsthis: & QNetworkCacheMetaData) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkCacheMetaData3urlEv()};
    let mut ret = unsafe {_ZNK21QNetworkCacheMetaData3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QNetworkCacheMetaData::expirationDate();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn expirationDate<RetType, T: QNetworkCacheMetaData_expirationDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expirationDate(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_expirationDate<RetType> {
  fn expirationDate(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  QDateTime QNetworkCacheMetaData::expirationDate();
impl<'a> /*trait*/ QNetworkCacheMetaData_expirationDate<QDateTime> for () {
  fn expirationDate(self , rsthis: & QNetworkCacheMetaData) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkCacheMetaData14expirationDateEv()};
    let mut ret = unsafe {_ZNK21QNetworkCacheMetaData14expirationDateEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::setExpirationDate(const QDateTime & dateTime);
impl /*struct*/ QNetworkCacheMetaData {
  pub fn setExpirationDate<RetType, T: QNetworkCacheMetaData_setExpirationDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpirationDate(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_setExpirationDate<RetType> {
  fn setExpirationDate(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  void QNetworkCacheMetaData::setExpirationDate(const QDateTime & dateTime);
impl<'a> /*trait*/ QNetworkCacheMetaData_setExpirationDate<()> for (&'a QDateTime) {
  fn setExpirationDate(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaData17setExpirationDateERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkCacheMetaData17setExpirationDateERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QNetworkCacheMetaData::saveToDisk();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn saveToDisk<RetType, T: QNetworkCacheMetaData_saveToDisk<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveToDisk(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_saveToDisk<RetType> {
  fn saveToDisk(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  bool QNetworkCacheMetaData::saveToDisk();
impl<'a> /*trait*/ QNetworkCacheMetaData_saveToDisk<i8> for () {
  fn saveToDisk(self , rsthis: & QNetworkCacheMetaData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkCacheMetaData10saveToDiskEv()};
    let mut ret = unsafe {_ZNK21QNetworkCacheMetaData10saveToDiskEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::QNetworkCacheMetaData();
impl<'a> /*trait*/ QNetworkCacheMetaData_new for () {
  fn new(self) -> QNetworkCacheMetaData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaDataC2Ev()};
    let ctysz: c_int = unsafe{QNetworkCacheMetaData_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN21QNetworkCacheMetaDataC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkCacheMetaData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::setUrl(const QUrl & url);
impl /*struct*/ QNetworkCacheMetaData {
  pub fn setUrl<RetType, T: QNetworkCacheMetaData_setUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUrl(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_setUrl<RetType> {
  fn setUrl(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  void QNetworkCacheMetaData::setUrl(const QUrl & url);
impl<'a> /*trait*/ QNetworkCacheMetaData_setUrl<()> for (&'a QUrl) {
  fn setUrl(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaData6setUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkCacheMetaData6setUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCacheMetaData::setSaveToDisk(bool allow);
impl /*struct*/ QNetworkCacheMetaData {
  pub fn setSaveToDisk<RetType, T: QNetworkCacheMetaData_setSaveToDisk<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSaveToDisk(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_setSaveToDisk<RetType> {
  fn setSaveToDisk(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  void QNetworkCacheMetaData::setSaveToDisk(bool allow);
impl<'a> /*trait*/ QNetworkCacheMetaData_setSaveToDisk<()> for (i8) {
  fn setSaveToDisk(self , rsthis: & QNetworkCacheMetaData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkCacheMetaData13setSaveToDiskEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN21QNetworkCacheMetaData13setSaveToDiskEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QNetworkCacheMetaData::isValid();
impl /*struct*/ QNetworkCacheMetaData {
  pub fn isValid<RetType, T: QNetworkCacheMetaData_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QNetworkCacheMetaData_isValid<RetType> {
  fn isValid(self , rsthis: & QNetworkCacheMetaData) -> RetType;
}

  // proto:  bool QNetworkCacheMetaData::isValid();
impl<'a> /*trait*/ QNetworkCacheMetaData_isValid<i8> for () {
  fn isValid(self , rsthis: & QNetworkCacheMetaData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkCacheMetaData7isValidEv()};
    let mut ret = unsafe {_ZNK21QNetworkCacheMetaData7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAbstractNetworkCache {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractNetworkCache {
    return QAbstractNetworkCache{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractNetworkCache {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractNetworkCache {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAbstractNetworkCache::~QAbstractNetworkCache();
impl /*struct*/ QAbstractNetworkCache {
  pub fn free<RetType, T: QAbstractNetworkCache_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_free<RetType> {
  fn free(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  void QAbstractNetworkCache::~QAbstractNetworkCache();
impl<'a> /*trait*/ QAbstractNetworkCache_free<()> for () {
  fn free(self , rsthis: & QAbstractNetworkCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCacheD2Ev()};
     unsafe {_ZN21QAbstractNetworkCacheD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkCacheMetaData QAbstractNetworkCache::metaData(const QUrl & url);
impl /*struct*/ QAbstractNetworkCache {
  pub fn metaData<RetType, T: QAbstractNetworkCache_metaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaData(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_metaData<RetType> {
  fn metaData(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  QNetworkCacheMetaData QAbstractNetworkCache::metaData(const QUrl & url);
impl<'a> /*trait*/ QAbstractNetworkCache_metaData<QNetworkCacheMetaData> for (&'a QUrl) {
  fn metaData(self , rsthis: & QAbstractNetworkCache) -> QNetworkCacheMetaData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache8metaDataERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QAbstractNetworkCache8metaDataERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkCacheMetaData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractNetworkCache::QAbstractNetworkCache(const QAbstractNetworkCache & );
impl /*struct*/ QAbstractNetworkCache {
  pub fn new<T: QAbstractNetworkCache_new>(value: T) -> QAbstractNetworkCache {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractNetworkCache_new {
  fn new(self) -> QAbstractNetworkCache;
}

  // proto:  void QAbstractNetworkCache::QAbstractNetworkCache(const QAbstractNetworkCache & );
impl<'a> /*trait*/ QAbstractNetworkCache_new for (&'a QAbstractNetworkCache) {
  fn new(self) -> QAbstractNetworkCache {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCacheC2ERKS_()};
    let ctysz: c_int = unsafe{QAbstractNetworkCache_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QAbstractNetworkCacheC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAbstractNetworkCache{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractNetworkCache::metaObject();
impl /*struct*/ QAbstractNetworkCache {
  pub fn metaObject<RetType, T: QAbstractNetworkCache_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  const QMetaObject * QAbstractNetworkCache::metaObject();
impl<'a> /*trait*/ QAbstractNetworkCache_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractNetworkCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractNetworkCache10metaObjectEv()};
     unsafe {_ZNK21QAbstractNetworkCache10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QIODevice * QAbstractNetworkCache::data(const QUrl & url);
impl /*struct*/ QAbstractNetworkCache {
  pub fn data<RetType, T: QAbstractNetworkCache_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_data<RetType> {
  fn data(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  QIODevice * QAbstractNetworkCache::data(const QUrl & url);
impl<'a> /*trait*/ QAbstractNetworkCache_data<QIODevice> for (&'a QUrl) {
  fn data(self , rsthis: & QAbstractNetworkCache) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache4dataERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QAbstractNetworkCache4dataERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractNetworkCache::insert(QIODevice * device);
impl /*struct*/ QAbstractNetworkCache {
  pub fn insert<RetType, T: QAbstractNetworkCache_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_insert<RetType> {
  fn insert(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  void QAbstractNetworkCache::insert(QIODevice * device);
impl<'a> /*trait*/ QAbstractNetworkCache_insert<()> for (&'a QIODevice) {
  fn insert(self , rsthis: & QAbstractNetworkCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache6insertEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QAbstractNetworkCache6insertEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractNetworkCache::remove(const QUrl & url);
impl /*struct*/ QAbstractNetworkCache {
  pub fn remove<RetType, T: QAbstractNetworkCache_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_remove<RetType> {
  fn remove(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  bool QAbstractNetworkCache::remove(const QUrl & url);
impl<'a> /*trait*/ QAbstractNetworkCache_remove<i8> for (&'a QUrl) {
  fn remove(self , rsthis: & QAbstractNetworkCache) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache6removeERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QAbstractNetworkCache6removeERK4QUrl(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QIODevice * QAbstractNetworkCache::prepare(const QNetworkCacheMetaData & metaData);
impl /*struct*/ QAbstractNetworkCache {
  pub fn prepare<RetType, T: QAbstractNetworkCache_prepare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepare(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_prepare<RetType> {
  fn prepare(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  QIODevice * QAbstractNetworkCache::prepare(const QNetworkCacheMetaData & metaData);
impl<'a> /*trait*/ QAbstractNetworkCache_prepare<QIODevice> for (&'a QNetworkCacheMetaData) {
  fn prepare(self , rsthis: & QAbstractNetworkCache) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache7prepareERK21QNetworkCacheMetaData()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QAbstractNetworkCache7prepareERK21QNetworkCacheMetaData(rsthis.qclsinst, arg0)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractNetworkCache::clear();
impl /*struct*/ QAbstractNetworkCache {
  pub fn clear<RetType, T: QAbstractNetworkCache_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_clear<RetType> {
  fn clear(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  void QAbstractNetworkCache::clear();
impl<'a> /*trait*/ QAbstractNetworkCache_clear<()> for () {
  fn clear(self , rsthis: & QAbstractNetworkCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache5clearEv()};
     unsafe {_ZN21QAbstractNetworkCache5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractNetworkCache::QAbstractNetworkCache(QObject * parent);
impl<'a> /*trait*/ QAbstractNetworkCache_new for (&'a QObject) {
  fn new(self) -> QAbstractNetworkCache {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCacheC2EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractNetworkCache_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QAbstractNetworkCacheC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAbstractNetworkCache{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QAbstractNetworkCache::cacheSize();
impl /*struct*/ QAbstractNetworkCache {
  pub fn cacheSize<RetType, T: QAbstractNetworkCache_cacheSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheSize(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_cacheSize<RetType> {
  fn cacheSize(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  qint64 QAbstractNetworkCache::cacheSize();
impl<'a> /*trait*/ QAbstractNetworkCache_cacheSize<i64> for () {
  fn cacheSize(self , rsthis: & QAbstractNetworkCache) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractNetworkCache9cacheSizeEv()};
    let mut ret = unsafe {_ZNK21QAbstractNetworkCache9cacheSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QAbstractNetworkCache::updateMetaData(const QNetworkCacheMetaData & metaData);
impl /*struct*/ QAbstractNetworkCache {
  pub fn updateMetaData<RetType, T: QAbstractNetworkCache_updateMetaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateMetaData(self);
    // return 1;
  }
}

pub trait QAbstractNetworkCache_updateMetaData<RetType> {
  fn updateMetaData(self , rsthis: & QAbstractNetworkCache) -> RetType;
}

  // proto:  void QAbstractNetworkCache::updateMetaData(const QNetworkCacheMetaData & metaData);
impl<'a> /*trait*/ QAbstractNetworkCache_updateMetaData<()> for (&'a QNetworkCacheMetaData) {
  fn updateMetaData(self , rsthis: & QAbstractNetworkCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractNetworkCache14updateMetaDataERK21QNetworkCacheMetaData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QAbstractNetworkCache14updateMetaDataERK21QNetworkCacheMetaData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

