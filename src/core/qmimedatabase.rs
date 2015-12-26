// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
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
use super::qurl::QUrl; // 773
use super::qmimetype::QMimeType; // 773
use super::qfileinfo::QFileInfo; // 773
use super::qbytearray::QByteArray; // 773
use super::qstring::QString; // 773
use super::qiodevice::QIODevice; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMimeDatabase_Class_Size() -> c_int;
  // proto:  QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl & url);
  fn _ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMimeDatabase::QMimeDatabase();
  fn dector_ZN13QMimeDatabaseC1Ev() -> *mut c_void;
  fn _ZN13QMimeDatabaseC1Ev(qthis: *mut c_void);
  // proto:  QMimeType QMimeDatabase::mimeTypeForData(const QByteArray & data);
  fn _ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForName(const QString & nameOrAlias);
  fn _ZNK13QMimeDatabase15mimeTypeForNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QMimeDatabase::suffixForFileName(const QString & fileName);
  fn _ZNK13QMimeDatabase17suffixForFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString & fileName);
  fn _ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, QIODevice * device);
  fn _ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForData(QIODevice * device);
  fn _ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMimeDatabase::~QMimeDatabase();
  fn _ZN13QMimeDatabaseD0Ev(qthis: *mut c_void);
  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, const QByteArray & data);
  fn _ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QList<QMimeType> QMimeDatabase::allMimeTypes();
  fn _ZNK13QMimeDatabase12allMimeTypesEv(qthis: *mut c_void);
  // proto:  void QMimeDatabase::QMimeDatabase(const QMimeDatabase & );
  fn dector_ZN13QMimeDatabaseC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QMimeDatabaseC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMimeDatabase)=8
pub struct QMimeDatabase {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMimeDatabase {
  pub fn inheritFrom(qthis: *mut c_void) -> QMimeDatabase {
    return QMimeDatabase{qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeDatabase::QMimeDatabase();
impl /*struct*/ QMimeDatabase {
  pub fn New<T: QMimeDatabase_New>(value: T) -> QMimeDatabase {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeDatabase_New {
  fn New(self) -> QMimeDatabase;
}

  // proto:  void QMimeDatabase::QMimeDatabase();
impl<'a> /*trait*/ QMimeDatabase_New for () {
  fn New(self) -> QMimeDatabase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseC1Ev()};
    let ctysz: c_int = unsafe{QMimeDatabase_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN13QMimeDatabaseC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN13QMimeDatabaseC1Ev()};
    let rsthis = QMimeDatabase{qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK13QMimeDatabase15mimeTypeForNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK13QMimeDatabase17suffixForFileNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
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
impl<'a> /*trait*/ QMimeDatabase_mimeTypesForFileName<()> for (&'a QString) {
  fn mimeTypesForFileName(self , rsthis: & QMimeDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMimeType::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeDatabase::~QMimeDatabase();
impl /*struct*/ QMimeDatabase {
  pub fn Free<RetType, T: QMimeDatabase_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMimeDatabase_Free<RetType> {
  fn Free(self , rsthis: & QMimeDatabase) -> RetType;
}

  // proto:  void QMimeDatabase::~QMimeDatabase();
impl<'a> /*trait*/ QMimeDatabase_Free<()> for () {
  fn Free(self , rsthis: & QMimeDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseD0Ev()};
     unsafe {_ZN13QMimeDatabaseD0Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMimeType::inheritFrom(ret);
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
impl<'a> /*trait*/ QMimeDatabase_allMimeTypes<()> for () {
  fn allMimeTypes(self , rsthis: & QMimeDatabase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase12allMimeTypesEv()};
     unsafe {_ZNK13QMimeDatabase12allMimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMimeDatabase::QMimeDatabase(const QMimeDatabase & );
impl<'a> /*trait*/ QMimeDatabase_New for (&'a QMimeDatabase) {
  fn New(self) -> QMimeDatabase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseC1ERKS_()};
    let ctysz: c_int = unsafe{QMimeDatabase_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QMimeDatabaseC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QMimeDatabaseC1ERKS_(arg0)};
    let rsthis = QMimeDatabase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

