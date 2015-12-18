// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfile::QFile;
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTemporaryFile::autoRemove();
  fn _ZNK14QTemporaryFile10autoRemoveEv(qthis: *mut c_void) -> int8_t;
  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(QFile & file);
  fn _ZN14QTemporaryFile15createLocalFileER5QFile(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryFile::NewQTemporaryFile(const QString & templateName);
  fn _ZN14QTemporaryFileC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTemporaryFile::NewQTemporaryFile();
  fn _ZN14QTemporaryFileC1Ev(qthis: *mut c_void) ;
  // proto:  void QTemporaryFile::NewQTemporaryFile(QObject * parent);
  fn _ZN14QTemporaryFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTemporaryFile::FreeQTemporaryFile();
  fn _ZN14QTemporaryFileD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QTemporaryFile::metaObject();
  fn _ZNK14QTemporaryFile10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTemporaryFile::setAutoRemove(bool b);
  fn _ZN14QTemporaryFile13setAutoRemoveEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QTemporaryFile::fileName();
  fn _ZNK14QTemporaryFile8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTemporaryFile::fileTemplate();
  fn _ZNK14QTemporaryFile12fileTemplateEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(const QString & fileName);
  fn _ZN14QTemporaryFile16createNativeFileERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTemporaryFile::open();
  fn _ZN14QTemporaryFile4openEv(qthis: *mut c_void) -> int8_t;
  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(const QString & fileName);
  fn _ZN14QTemporaryFile15createLocalFileERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(QFile & file);
  fn _ZN14QTemporaryFile16createNativeFileER5QFile(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryFile::setFileTemplate(const QString & name);
  fn _ZN14QTemporaryFile15setFileTemplateERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTemporaryFile::NewQTemporaryFile(const QTemporaryFile & );
  fn _ZN14QTemporaryFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTemporaryFile::NewQTemporaryFile(const QString & templateName, QObject * parent);
  fn _ZN14QTemporaryFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QTemporaryFile)=1
pub struct QTemporaryFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTemporaryFile {
  pub fn autoRemove<RetType, T: QTemporaryFile_autoRemove<RetType>>(&mut self, value: T) -> RetType {
    return value.autoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryFile_autoRemove<RetType> {
  fn autoRemove(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  bool QTemporaryFile::autoRemove();
impl<'a> /*trait*/ QTemporaryFile_autoRemove<i8> for () {
  fn autoRemove(self, rsthis: &mut QTemporaryFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile10autoRemoveEv()};
    let mut ret = unsafe {_ZNK14QTemporaryFile10autoRemoveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn createLocalFile<RetType, T: QTemporaryFile_createLocalFile<RetType>>(&mut self, value: T) -> RetType {
    return value.createLocalFile(self);
    // return 1;
  }
}

pub trait QTemporaryFile_createLocalFile<RetType> {
  fn createLocalFile(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto: static QTemporaryFile * QTemporaryFile::createLocalFile(QFile & file);
impl<'a> /*trait*/ QTemporaryFile_createLocalFile<QTemporaryFile> for (&'a mut QFile) {
  fn createLocalFile(self, rsthis: &mut QTemporaryFile) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15createLocalFileER5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QTemporaryFile15createLocalFileER5QFile(arg0)};
    let mut ret1 = QTemporaryFile{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn NewQTemporaryFile<T: QTemporaryFile_NewQTemporaryFile>(value: T) -> QTemporaryFile {
    let rsthis = value.NewQTemporaryFile();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryFile_NewQTemporaryFile {
  fn NewQTemporaryFile(self) -> QTemporaryFile;
}

// proto: void QTemporaryFile::NewQTemporaryFile(const QString & templateName);
impl<'a> /*trait*/ QTemporaryFile_NewQTemporaryFile for (&'a  QString) {
  fn NewQTemporaryFile(self) -> QTemporaryFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFileC1ERK7QString(qthis, arg0)};
    let rsthis = QTemporaryFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTemporaryFile::NewQTemporaryFile();
impl<'a> /*trait*/ QTemporaryFile_NewQTemporaryFile for () {
  fn NewQTemporaryFile(self) -> QTemporaryFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1Ev()};
    unsafe {_ZN14QTemporaryFileC1Ev(qthis)};
    let rsthis = QTemporaryFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTemporaryFile::NewQTemporaryFile(QObject * parent);
impl<'a> /*trait*/ QTemporaryFile_NewQTemporaryFile for (&'a mut QObject) {
  fn NewQTemporaryFile(self) -> QTemporaryFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFileC1EP7QObject(qthis, arg0)};
    let rsthis = QTemporaryFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn FreeQTemporaryFile<RetType, T: QTemporaryFile_FreeQTemporaryFile<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTemporaryFile(self);
    // return 1;
  }
}

pub trait QTemporaryFile_FreeQTemporaryFile<RetType> {
  fn FreeQTemporaryFile(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  void QTemporaryFile::FreeQTemporaryFile();
impl<'a> /*trait*/ QTemporaryFile_FreeQTemporaryFile<()> for () {
  fn FreeQTemporaryFile(self, rsthis: &mut QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileD0Ev()};
     unsafe {_ZN14QTemporaryFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn metaObject<RetType, T: QTemporaryFile_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QTemporaryFile_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  const QMetaObject * QTemporaryFile::metaObject();
impl<'a> /*trait*/ QTemporaryFile_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile10metaObjectEv()};
     unsafe {_ZNK14QTemporaryFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn setAutoRemove<RetType, T: QTemporaryFile_setAutoRemove<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryFile_setAutoRemove<RetType> {
  fn setAutoRemove(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  void QTemporaryFile::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryFile_setAutoRemove<()> for (i8) {
  fn setAutoRemove(self, rsthis: &mut QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile13setAutoRemoveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QTemporaryFile13setAutoRemoveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn fileName<RetType, T: QTemporaryFile_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QTemporaryFile_fileName<RetType> {
  fn fileName(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  QString QTemporaryFile::fileName();
impl<'a> /*trait*/ QTemporaryFile_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QTemporaryFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile8fileNameEv()};
    let mut ret = unsafe {_ZNK14QTemporaryFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn fileTemplate<RetType, T: QTemporaryFile_fileTemplate<RetType>>(&mut self, value: T) -> RetType {
    return value.fileTemplate(self);
    // return 1;
  }
}

pub trait QTemporaryFile_fileTemplate<RetType> {
  fn fileTemplate(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  QString QTemporaryFile::fileTemplate();
impl<'a> /*trait*/ QTemporaryFile_fileTemplate<QString> for () {
  fn fileTemplate(self, rsthis: &mut QTemporaryFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile12fileTemplateEv()};
    let mut ret = unsafe {_ZNK14QTemporaryFile12fileTemplateEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn createNativeFile<RetType, T: QTemporaryFile_createNativeFile<RetType>>(&mut self, value: T) -> RetType {
    return value.createNativeFile(self);
    // return 1;
  }
}

pub trait QTemporaryFile_createNativeFile<RetType> {
  fn createNativeFile(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto: static QTemporaryFile * QTemporaryFile::createNativeFile(const QString & fileName);
impl<'a> /*trait*/ QTemporaryFile_createNativeFile<QTemporaryFile> for (&'a  QString) {
  fn createNativeFile(self, rsthis: &mut QTemporaryFile) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile16createNativeFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QTemporaryFile16createNativeFileERK7QString(arg0)};
    let mut ret1 = QTemporaryFile{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn open<RetType, T: QTemporaryFile_open<RetType>>(&mut self, value: T) -> RetType {
    return value.open(self);
    // return 1;
  }
}

pub trait QTemporaryFile_open<RetType> {
  fn open(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  bool QTemporaryFile::open();
impl<'a> /*trait*/ QTemporaryFile_open<i8> for () {
  fn open(self, rsthis: &mut QTemporaryFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile4openEv()};
    let mut ret = unsafe {_ZN14QTemporaryFile4openEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static QTemporaryFile * QTemporaryFile::createLocalFile(const QString & fileName);
impl<'a> /*trait*/ QTemporaryFile_createLocalFile<QTemporaryFile> for (&'a  QString) {
  fn createLocalFile(self, rsthis: &mut QTemporaryFile) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15createLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QTemporaryFile15createLocalFileERK7QString(arg0)};
    let mut ret1 = QTemporaryFile{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QTemporaryFile * QTemporaryFile::createNativeFile(QFile & file);
impl<'a> /*trait*/ QTemporaryFile_createNativeFile<QTemporaryFile> for (&'a mut QFile) {
  fn createNativeFile(self, rsthis: &mut QTemporaryFile) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile16createNativeFileER5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QTemporaryFile16createNativeFileER5QFile(arg0)};
    let mut ret1 = QTemporaryFile{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn setFileTemplate<RetType, T: QTemporaryFile_setFileTemplate<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileTemplate(self);
    // return 1;
  }
}

pub trait QTemporaryFile_setFileTemplate<RetType> {
  fn setFileTemplate(self, rsthis: &mut QTemporaryFile) -> RetType;
}

// proto:  void QTemporaryFile::setFileTemplate(const QString & name);
impl<'a> /*trait*/ QTemporaryFile_setFileTemplate<()> for (&'a  QString) {
  fn setFileTemplate(self, rsthis: &mut QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15setFileTemplateERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QTemporaryFile15setFileTemplateERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTemporaryFile::NewQTemporaryFile(const QTemporaryFile & );
impl<'a> /*trait*/ QTemporaryFile_NewQTemporaryFile for (&'a  QTemporaryFile) {
  fn NewQTemporaryFile(self) -> QTemporaryFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFileC1ERKS_(qthis, arg0)};
    let rsthis = QTemporaryFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTemporaryFile::NewQTemporaryFile(const QString & templateName, QObject * parent);
impl<'a> /*trait*/ QTemporaryFile_NewQTemporaryFile for (&'a  QString, &'a mut QObject) {
  fn NewQTemporaryFile(self) -> QTemporaryFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QTemporaryFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

