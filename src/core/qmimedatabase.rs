// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qmimedatabase.h
// dst-file: /src/core/qmimedatabase.rs
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
use super::qurl::*; // 773
use super::qmimetype::*; // 773
use super::qfileinfo::*; // 773
use super::qbytearray::*; // 773
use super::qstring::*; // 773
// use super::qlist::*; // 775
use super::qiodevice::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMimeDatabase_Class_Size() -> c_int;
  // proto:  QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl & url);
  fn C_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMimeDatabase::QMimeDatabase();
  fn C_ZN13QMimeDatabaseC2Ev() -> u64;
  // proto:  QMimeType QMimeDatabase::mimeTypeForData(const QByteArray & data);
  fn C_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForName(const QString & nameOrAlias);
  fn C_ZNK13QMimeDatabase15mimeTypeForNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QMimeDatabase::suffixForFileName(const QString & fileName);
  fn C_ZNK13QMimeDatabase17suffixForFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString & fileName);
  fn C_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, QIODevice * device);
  fn C_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForData(QIODevice * device);
  fn C_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMimeDatabase::~QMimeDatabase();
  fn C_ZN13QMimeDatabaseD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, const QByteArray & data);
  fn C_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QList<QMimeType> QMimeDatabase::allMimeTypes();
  fn C_ZNK13QMimeDatabase12allMimeTypesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QMimeDatabase)=8
#[derive(Default)]
pub struct QMimeDatabase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMimeDatabase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMimeDatabase {
    return QMimeDatabase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl & url);
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForUrl<RetType, T: QMimeDatabase_mimeTypeForUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForUrl(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForUrl<RetType> {
  fn mimeTypeForUrl(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl & url);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForUrl<QMimeType> for (&'a QUrl) {
  fn mimeTypeForUrl(self , rsthis: & QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeDatabase::QMimeDatabase();
impl /*struct*/ QMimeDatabase {
  pub fn new<T: QMimeDatabase_new>(value: T) -> QMimeDatabase {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeDatabase_new {
  fn new(self) -> QMimeDatabase;
}

  // proto:  void QMimeDatabase::QMimeDatabase();
impl<'a> /*trait*/ QMimeDatabase_new for () {
  fn new(self) -> QMimeDatabase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseC2Ev()};
    let ctysz: c_int = unsafe{QMimeDatabase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN13QMimeDatabaseC2Ev()};
    let rsthis = QMimeDatabase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForData(const QByteArray & data);
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForData<RetType, T: QMimeDatabase_mimeTypeForData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForData(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForData<RetType> {
  fn mimeTypeForData(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForData(const QByteArray & data);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForData<QMimeType> for (&'a QByteArray) {
  fn mimeTypeForData(self , rsthis: & QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForName(const QString & nameOrAlias);
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForName<RetType, T: QMimeDatabase_mimeTypeForName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForName(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForName<RetType> {
  fn mimeTypeForName(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForName(const QString & nameOrAlias);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForName<QMimeType> for (&'a QString) {
  fn mimeTypeForName(self , rsthis: & QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase15mimeTypeForNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase15mimeTypeForNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QMimeDatabase::suffixForFileName(const QString & fileName);
impl /*struct*/ QMimeDatabase {
  pub fn suffixForFileName<RetType, T: QMimeDatabase_suffixForFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.suffixForFileName(self);
    // return 1;
  }
}

pub trait QMimeDatabase_suffixForFileName<RetType> {
  fn suffixForFileName(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QString QMimeDatabase::suffixForFileName(const QString & fileName);
impl<'a> /*trait*/ QMimeDatabase_suffixForFileName<QString> for (&'a QString) {
  fn suffixForFileName(self , rsthis: & QMimeDatabase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase17suffixForFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase17suffixForFileNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString & fileName);
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypesForFileName<RetType, T: QMimeDatabase_mimeTypesForFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypesForFileName(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypesForFileName<RetType> {
  fn mimeTypesForFileName(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString & fileName);
impl<'a> /*trait*/ QMimeDatabase_mimeTypesForFileName<u64> for (&'a QString) {
  fn mimeTypesForFileName(self , rsthis: & QMimeDatabase) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, QIODevice * device);
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForFileNameAndData<RetType, T: QMimeDatabase_mimeTypeForFileNameAndData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForFileNameAndData(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForFileNameAndData<RetType> {
  fn mimeTypeForFileNameAndData(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, QIODevice * device);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFileNameAndData<QMimeType> for (&'a QString, &'a QIODevice) {
  fn mimeTypeForFileNameAndData(self , rsthis: & QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMimeType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForData(QIODevice * device);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForData<QMimeType> for (&'a QIODevice) {
  fn mimeTypeForData(self , rsthis: & QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeDatabase::~QMimeDatabase();
impl /*struct*/ QMimeDatabase {
  pub fn free<RetType, T: QMimeDatabase_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMimeDatabase_free<RetType> {
  fn free(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  void QMimeDatabase::~QMimeDatabase();
impl<'a> /*trait*/ QMimeDatabase_free<()> for () {
  fn free(self , rsthis: & QMimeDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseD2Ev()};
     unsafe {C_ZN13QMimeDatabaseD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, const QByteArray & data);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFileNameAndData<QMimeType> for (&'a QString, &'a QByteArray) {
  fn mimeTypeForFileNameAndData(self , rsthis: & QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMimeType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QMimeType> QMimeDatabase::allMimeTypes();
impl /*struct*/ QMimeDatabase {
  pub fn allMimeTypes<RetType, T: QMimeDatabase_allMimeTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allMimeTypes(self);
    // return 1;
  }
}

pub trait QMimeDatabase_allMimeTypes<RetType> {
  fn allMimeTypes(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  QList<QMimeType> QMimeDatabase::allMimeTypes();
impl<'a> /*trait*/ QMimeDatabase_allMimeTypes<u64> for () {
  fn allMimeTypes(self , rsthis: & QMimeDatabase) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase12allMimeTypesEv()};
    let mut ret = unsafe {C_ZNK13QMimeDatabase12allMimeTypesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

// <= body block end

