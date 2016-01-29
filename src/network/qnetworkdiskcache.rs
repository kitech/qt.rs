// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworkdiskcache.h
// dst-file: /src/network/qnetworkdiskcache.rs
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
use super::qabstractnetworkcache::QAbstractNetworkCache; // 773
use std::ops::Deref;
use super::super::core::qurl::QUrl; // 771
use super::qabstractnetworkcache::QNetworkCacheMetaData; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkDiskCache_Class_Size() -> c_int;
  // proto:  qint64 QNetworkDiskCache::cacheSize();
  fn _ZNK17QNetworkDiskCache9cacheSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QNetworkCacheMetaData QNetworkDiskCache::metaData(const QUrl & url);
  fn _ZN17QNetworkDiskCache8metaDataERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QNetworkDiskCache::cacheDirectory();
  fn _ZNK17QNetworkDiskCache14cacheDirectoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkDiskCache::~QNetworkDiskCache();
  fn _ZN17QNetworkDiskCacheD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkDiskCache::remove(const QUrl & url);
  fn _ZN17QNetworkDiskCache6removeERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QIODevice * QNetworkDiskCache::data(const QUrl & url);
  fn _ZN17QNetworkDiskCache4dataERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkDiskCache::QNetworkDiskCache(const QNetworkDiskCache & );
  fn _ZN17QNetworkDiskCacheC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QNetworkDiskCache::metaObject();
  fn _ZNK17QNetworkDiskCache10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkDiskCache::QNetworkDiskCache(QObject * parent);
  fn _ZN17QNetworkDiskCacheC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkDiskCache::updateMetaData(const QNetworkCacheMetaData & metaData);
  fn _ZN17QNetworkDiskCache14updateMetaDataERK21QNetworkCacheMetaData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QIODevice * QNetworkDiskCache::prepare(const QNetworkCacheMetaData & metaData);
  fn _ZN17QNetworkDiskCache7prepareERK21QNetworkCacheMetaData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkDiskCache::setCacheDirectory(const QString & cacheDir);
  fn _ZN17QNetworkDiskCache17setCacheDirectoryERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkDiskCache::setMaximumCacheSize(qint64 size);
  fn _ZN17QNetworkDiskCache19setMaximumCacheSizeEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  // proto:  void QNetworkDiskCache::insert(QIODevice * device);
  fn _ZN17QNetworkDiskCache6insertEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkCacheMetaData QNetworkDiskCache::fileMetaData(const QString & fileName);
  fn _ZNK17QNetworkDiskCache12fileMetaDataERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkDiskCache::clear();
  fn _ZN17QNetworkDiskCache5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  qint64 QNetworkDiskCache::maximumCacheSize();
  fn _ZNK17QNetworkDiskCache16maximumCacheSizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkDiskCache)=1
#[derive(Default)]
pub struct QNetworkDiskCache {
  qbase: QAbstractNetworkCache,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkDiskCache {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkDiskCache {
    return QNetworkDiskCache{qbase: QAbstractNetworkCache::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNetworkDiskCache {
  type Target = QAbstractNetworkCache;

  fn deref(&self) -> &QAbstractNetworkCache {
    return & self.qbase;
  }
}
impl AsRef<QAbstractNetworkCache> for QNetworkDiskCache {
  fn as_ref(& self) -> & QAbstractNetworkCache {
    return & self.qbase;
  }
}
  // proto:  qint64 QNetworkDiskCache::cacheSize();
impl /*struct*/ QNetworkDiskCache {
  pub fn cacheSize<RetType, T: QNetworkDiskCache_cacheSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheSize(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_cacheSize<RetType> {
  fn cacheSize(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  qint64 QNetworkDiskCache::cacheSize();
impl<'a> /*trait*/ QNetworkDiskCache_cacheSize<i64> for () {
  fn cacheSize(self , rsthis: & QNetworkDiskCache) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkDiskCache9cacheSizeEv()};
    let mut ret = unsafe {_ZNK17QNetworkDiskCache9cacheSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QNetworkCacheMetaData QNetworkDiskCache::metaData(const QUrl & url);
impl /*struct*/ QNetworkDiskCache {
  pub fn metaData<RetType, T: QNetworkDiskCache_metaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaData(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_metaData<RetType> {
  fn metaData(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  QNetworkCacheMetaData QNetworkDiskCache::metaData(const QUrl & url);
impl<'a> /*trait*/ QNetworkDiskCache_metaData<QNetworkCacheMetaData> for (&'a QUrl) {
  fn metaData(self , rsthis: & QNetworkDiskCache) -> QNetworkCacheMetaData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache8metaDataERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkDiskCache8metaDataERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkCacheMetaData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QNetworkDiskCache::cacheDirectory();
impl /*struct*/ QNetworkDiskCache {
  pub fn cacheDirectory<RetType, T: QNetworkDiskCache_cacheDirectory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheDirectory(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_cacheDirectory<RetType> {
  fn cacheDirectory(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  QString QNetworkDiskCache::cacheDirectory();
impl<'a> /*trait*/ QNetworkDiskCache_cacheDirectory<QString> for () {
  fn cacheDirectory(self , rsthis: & QNetworkDiskCache) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkDiskCache14cacheDirectoryEv()};
    let mut ret = unsafe {_ZNK17QNetworkDiskCache14cacheDirectoryEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::~QNetworkDiskCache();
impl /*struct*/ QNetworkDiskCache {
  pub fn free<RetType, T: QNetworkDiskCache_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_free<RetType> {
  fn free(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  void QNetworkDiskCache::~QNetworkDiskCache();
impl<'a> /*trait*/ QNetworkDiskCache_free<()> for () {
  fn free(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCacheD2Ev()};
     unsafe {_ZN17QNetworkDiskCacheD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QNetworkDiskCache::remove(const QUrl & url);
impl /*struct*/ QNetworkDiskCache {
  pub fn remove<RetType, T: QNetworkDiskCache_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_remove<RetType> {
  fn remove(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  bool QNetworkDiskCache::remove(const QUrl & url);
impl<'a> /*trait*/ QNetworkDiskCache_remove<i8> for (&'a QUrl) {
  fn remove(self , rsthis: & QNetworkDiskCache) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache6removeERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkDiskCache6removeERK4QUrl(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QIODevice * QNetworkDiskCache::data(const QUrl & url);
impl /*struct*/ QNetworkDiskCache {
  pub fn data<RetType, T: QNetworkDiskCache_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_data<RetType> {
  fn data(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  QIODevice * QNetworkDiskCache::data(const QUrl & url);
impl<'a> /*trait*/ QNetworkDiskCache_data<QIODevice> for (&'a QUrl) {
  fn data(self , rsthis: & QNetworkDiskCache) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache4dataERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkDiskCache4dataERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::QNetworkDiskCache(const QNetworkDiskCache & );
impl /*struct*/ QNetworkDiskCache {
  pub fn new<T: QNetworkDiskCache_new>(value: T) -> QNetworkDiskCache {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkDiskCache_new {
  fn new(self) -> QNetworkDiskCache;
}

  // proto:  void QNetworkDiskCache::QNetworkDiskCache(const QNetworkDiskCache & );
impl<'a> /*trait*/ QNetworkDiskCache_new for (&'a QNetworkDiskCache) {
  fn new(self) -> QNetworkDiskCache {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCacheC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkDiskCache_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QNetworkDiskCacheC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkDiskCache{qbase: QAbstractNetworkCache::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QNetworkDiskCache::metaObject();
impl /*struct*/ QNetworkDiskCache {
  pub fn metaObject<RetType, T: QNetworkDiskCache_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_metaObject<RetType> {
  fn metaObject(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  const QMetaObject * QNetworkDiskCache::metaObject();
impl<'a> /*trait*/ QNetworkDiskCache_metaObject<()> for () {
  fn metaObject(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkDiskCache10metaObjectEv()};
     unsafe {_ZNK17QNetworkDiskCache10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::QNetworkDiskCache(QObject * parent);
impl<'a> /*trait*/ QNetworkDiskCache_new for (&'a QObject) {
  fn new(self) -> QNetworkDiskCache {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCacheC2EP7QObject()};
    let ctysz: c_int = unsafe{QNetworkDiskCache_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QNetworkDiskCacheC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkDiskCache{qbase: QAbstractNetworkCache::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::updateMetaData(const QNetworkCacheMetaData & metaData);
impl /*struct*/ QNetworkDiskCache {
  pub fn updateMetaData<RetType, T: QNetworkDiskCache_updateMetaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateMetaData(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_updateMetaData<RetType> {
  fn updateMetaData(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  void QNetworkDiskCache::updateMetaData(const QNetworkCacheMetaData & metaData);
impl<'a> /*trait*/ QNetworkDiskCache_updateMetaData<()> for (&'a QNetworkCacheMetaData) {
  fn updateMetaData(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache14updateMetaDataERK21QNetworkCacheMetaData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QNetworkDiskCache14updateMetaDataERK21QNetworkCacheMetaData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIODevice * QNetworkDiskCache::prepare(const QNetworkCacheMetaData & metaData);
impl /*struct*/ QNetworkDiskCache {
  pub fn prepare<RetType, T: QNetworkDiskCache_prepare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepare(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_prepare<RetType> {
  fn prepare(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  QIODevice * QNetworkDiskCache::prepare(const QNetworkCacheMetaData & metaData);
impl<'a> /*trait*/ QNetworkDiskCache_prepare<QIODevice> for (&'a QNetworkCacheMetaData) {
  fn prepare(self , rsthis: & QNetworkDiskCache) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache7prepareERK21QNetworkCacheMetaData()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkDiskCache7prepareERK21QNetworkCacheMetaData(rsthis.qclsinst, arg0)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::setCacheDirectory(const QString & cacheDir);
impl /*struct*/ QNetworkDiskCache {
  pub fn setCacheDirectory<RetType, T: QNetworkDiskCache_setCacheDirectory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCacheDirectory(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_setCacheDirectory<RetType> {
  fn setCacheDirectory(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  void QNetworkDiskCache::setCacheDirectory(const QString & cacheDir);
impl<'a> /*trait*/ QNetworkDiskCache_setCacheDirectory<()> for (&'a QString) {
  fn setCacheDirectory(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache17setCacheDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QNetworkDiskCache17setCacheDirectoryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::setMaximumCacheSize(qint64 size);
impl /*struct*/ QNetworkDiskCache {
  pub fn setMaximumCacheSize<RetType, T: QNetworkDiskCache_setMaximumCacheSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumCacheSize(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_setMaximumCacheSize<RetType> {
  fn setMaximumCacheSize(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  void QNetworkDiskCache::setMaximumCacheSize(qint64 size);
impl<'a> /*trait*/ QNetworkDiskCache_setMaximumCacheSize<()> for (i64) {
  fn setMaximumCacheSize(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache19setMaximumCacheSizeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN17QNetworkDiskCache19setMaximumCacheSizeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::insert(QIODevice * device);
impl /*struct*/ QNetworkDiskCache {
  pub fn insert<RetType, T: QNetworkDiskCache_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_insert<RetType> {
  fn insert(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  void QNetworkDiskCache::insert(QIODevice * device);
impl<'a> /*trait*/ QNetworkDiskCache_insert<()> for (&'a QIODevice) {
  fn insert(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache6insertEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QNetworkDiskCache6insertEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QNetworkCacheMetaData QNetworkDiskCache::fileMetaData(const QString & fileName);
impl /*struct*/ QNetworkDiskCache {
  pub fn fileMetaData<RetType, T: QNetworkDiskCache_fileMetaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileMetaData(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_fileMetaData<RetType> {
  fn fileMetaData(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  QNetworkCacheMetaData QNetworkDiskCache::fileMetaData(const QString & fileName);
impl<'a> /*trait*/ QNetworkDiskCache_fileMetaData<QNetworkCacheMetaData> for (&'a QString) {
  fn fileMetaData(self , rsthis: & QNetworkDiskCache) -> QNetworkCacheMetaData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkDiskCache12fileMetaDataERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QNetworkDiskCache12fileMetaDataERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkCacheMetaData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkDiskCache::clear();
impl /*struct*/ QNetworkDiskCache {
  pub fn clear<RetType, T: QNetworkDiskCache_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_clear<RetType> {
  fn clear(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  void QNetworkDiskCache::clear();
impl<'a> /*trait*/ QNetworkDiskCache_clear<()> for () {
  fn clear(self , rsthis: & QNetworkDiskCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkDiskCache5clearEv()};
     unsafe {_ZN17QNetworkDiskCache5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QNetworkDiskCache::maximumCacheSize();
impl /*struct*/ QNetworkDiskCache {
  pub fn maximumCacheSize<RetType, T: QNetworkDiskCache_maximumCacheSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumCacheSize(self);
    // return 1;
  }
}

pub trait QNetworkDiskCache_maximumCacheSize<RetType> {
  fn maximumCacheSize(self , rsthis: & QNetworkDiskCache) -> RetType;
}

  // proto:  qint64 QNetworkDiskCache::maximumCacheSize();
impl<'a> /*trait*/ QNetworkDiskCache_maximumCacheSize<i64> for () {
  fn maximumCacheSize(self , rsthis: & QNetworkDiskCache) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkDiskCache16maximumCacheSizeEv()};
    let mut ret = unsafe {_ZNK17QNetworkDiskCache16maximumCacheSizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// <= body block end

