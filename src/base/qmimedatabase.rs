// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurl::QUrl;
use super::qmimetype::QMimeType;
use super::qbytearray::QByteArray;
use super::qstring::QString;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl & url);
  fn _ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMimeDatabase::NewQMimeDatabase();
  fn _ZN13QMimeDatabaseC1Ev(qthis: *mut c_void) ;
  // proto:  QMimeType QMimeDatabase::mimeTypeForData(const QByteArray & data);
  fn _ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForName(const QString & nameOrAlias);
  fn _ZNK13QMimeDatabase15mimeTypeForNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QMimeDatabase::suffixForFileName(const QString & fileName);
  fn _ZNK13QMimeDatabase17suffixForFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString & fileName);
  fn _ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, QIODevice * device);
  fn _ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QMimeType QMimeDatabase::mimeTypeForData(QIODevice * device);
  fn _ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMimeDatabase::FreeQMimeDatabase();
  fn _ZN13QMimeDatabaseD0Ev(qthis: *mut c_void) ;
  // proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, const QByteArray & data);
  fn _ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QList<QMimeType> QMimeDatabase::allMimeTypes();
  fn _ZNK13QMimeDatabase12allMimeTypesEv(qthis: *mut c_void) ;
  // proto:  void QMimeDatabase::NewQMimeDatabase(const QMimeDatabase & );
  fn _ZN13QMimeDatabaseC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QMimeDatabase)=8
pub struct QMimeDatabase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForUrl<T: QMimeDatabase_mimeTypeForUrl>(&mut self, value: T) -> QMimeType {
    return value.mimeTypeForUrl(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForUrl {
  fn mimeTypeForUrl(self, rsthis: &mut QMimeDatabase) -> QMimeType;
}

// proto:  QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl & url);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForUrl for (&'a  QUrl) {
  fn mimeTypeForUrl(self, rsthis: &mut QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn NewQMimeDatabase<T: QMimeDatabase_NewQMimeDatabase>(value: T) -> QMimeDatabase {
    let rsthis = value.NewQMimeDatabase();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeDatabase_NewQMimeDatabase {
  fn NewQMimeDatabase(self) -> QMimeDatabase;
}

// proto: void QMimeDatabase::NewQMimeDatabase();
impl<'a> /*trait*/ QMimeDatabase_NewQMimeDatabase for () {
  fn NewQMimeDatabase(self) -> QMimeDatabase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseC1Ev()};
    unsafe {_ZN13QMimeDatabaseC1Ev(qthis)};
    let rsthis = QMimeDatabase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForData<T: QMimeDatabase_mimeTypeForData>(&mut self, value: T) -> QMimeType {
    return value.mimeTypeForData(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForData {
  fn mimeTypeForData(self, rsthis: &mut QMimeDatabase) -> QMimeType;
}

// proto:  QMimeType QMimeDatabase::mimeTypeForData(const QByteArray & data);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForData for (&'a  QByteArray) {
  fn mimeTypeForData(self, rsthis: &mut QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForName<T: QMimeDatabase_mimeTypeForName>(&mut self, value: T) -> QMimeType {
    return value.mimeTypeForName(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForName {
  fn mimeTypeForName(self, rsthis: &mut QMimeDatabase) -> QMimeType;
}

// proto:  QMimeType QMimeDatabase::mimeTypeForName(const QString & nameOrAlias);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForName for (&'a  QString) {
  fn mimeTypeForName(self, rsthis: &mut QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase15mimeTypeForNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase15mimeTypeForNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn suffixForFileName<T: QMimeDatabase_suffixForFileName>(&mut self, value: T) -> QString {
    return value.suffixForFileName(self);
    // return 1;
  }
}

pub trait QMimeDatabase_suffixForFileName {
  fn suffixForFileName(self, rsthis: &mut QMimeDatabase) -> QString;
}

// proto:  QString QMimeDatabase::suffixForFileName(const QString & fileName);
impl<'a> /*trait*/ QMimeDatabase_suffixForFileName for (&'a  QString) {
  fn suffixForFileName(self, rsthis: &mut QMimeDatabase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase17suffixForFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase17suffixForFileNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn mimeTypesForFileName<T: QMimeDatabase_mimeTypesForFileName>(&mut self, value: T)  {
     value.mimeTypesForFileName(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypesForFileName {
  fn mimeTypesForFileName(self, rsthis: &mut QMimeDatabase) ;
}

// proto:  QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString & fileName);
impl<'a> /*trait*/ QMimeDatabase_mimeTypesForFileName for (&'a  QString) {
  fn mimeTypesForFileName(self, rsthis: &mut QMimeDatabase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QMimeDatabase20mimeTypesForFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForFileNameAndData<T: QMimeDatabase_mimeTypeForFileNameAndData>(&mut self, value: T) -> QMimeType {
    return value.mimeTypeForFileNameAndData(self);
    // return 1;
  }
}

pub trait QMimeDatabase_mimeTypeForFileNameAndData {
  fn mimeTypeForFileNameAndData(self, rsthis: &mut QMimeDatabase) -> QMimeType;
}

// proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, QIODevice * device);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFileNameAndData for (&'a  QString, &'a mut QIODevice) {
  fn mimeTypeForFileNameAndData(self, rsthis: &mut QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMimeType{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMimeType QMimeDatabase::mimeTypeForData(QIODevice * device);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForData for (&'a mut QIODevice) {
  fn mimeTypeForData(self, rsthis: &mut QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice(rsthis.qclsinst, arg0)};
    let mut ret1 = QMimeType{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn FreeQMimeDatabase<T: QMimeDatabase_FreeQMimeDatabase>(&mut self, value: T)  {
     value.FreeQMimeDatabase(self);
    // return 1;
  }
}

pub trait QMimeDatabase_FreeQMimeDatabase {
  fn FreeQMimeDatabase(self, rsthis: &mut QMimeDatabase) ;
}

// proto:  void QMimeDatabase::FreeQMimeDatabase();
impl<'a> /*trait*/ QMimeDatabase_FreeQMimeDatabase for () {
  fn FreeQMimeDatabase(self, rsthis: &mut QMimeDatabase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseD0Ev()};
     unsafe {_ZN13QMimeDatabaseD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString & fileName, const QByteArray & data);
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFileNameAndData for (&'a  QString, &'a  QByteArray) {
  fn mimeTypeForFileNameAndData(self, rsthis: &mut QMimeDatabase) -> QMimeType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMimeType{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeDatabase {
  pub fn allMimeTypes<T: QMimeDatabase_allMimeTypes>(&mut self, value: T)  {
     value.allMimeTypes(self);
    // return 1;
  }
}

pub trait QMimeDatabase_allMimeTypes {
  fn allMimeTypes(self, rsthis: &mut QMimeDatabase) ;
}

// proto:  QList<QMimeType> QMimeDatabase::allMimeTypes();
impl<'a> /*trait*/ QMimeDatabase_allMimeTypes for () {
  fn allMimeTypes(self, rsthis: &mut QMimeDatabase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMimeDatabase12allMimeTypesEv()};
     unsafe {_ZNK13QMimeDatabase12allMimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QMimeDatabase::NewQMimeDatabase(const QMimeDatabase & );
impl<'a> /*trait*/ QMimeDatabase_NewQMimeDatabase for (&'a  QMimeDatabase) {
  fn NewQMimeDatabase(self) -> QMimeDatabase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMimeDatabaseC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QMimeDatabaseC1ERKS_(qthis, arg0)};
    let rsthis = QMimeDatabase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

