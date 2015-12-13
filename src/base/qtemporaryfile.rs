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
  fn _ZNK14QTemporaryFile10autoRemoveEv() -> i32;
  fn _ZN14QTemporaryFile15createLocalFileER5QFile(arg0: *mut c_void) -> i32;
  fn _ZN14QTemporaryFileC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN14QTemporaryFileC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN14QTemporaryFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN14QTemporaryFileD0Ev() -> i32;
  fn _ZNK14QTemporaryFile10metaObjectEv() -> i32;
  fn _ZN14QTemporaryFile13setAutoRemoveEb(arg0: int8_t) -> i32;
  fn _ZNK14QTemporaryFile8fileNameEv() -> i32;
  fn _ZNK14QTemporaryFile12fileTemplateEv() -> i32;
  fn _ZN14QTemporaryFile16createNativeFileERK7QString(arg0: *const c_void) -> i32;
  fn _ZN14QTemporaryFile4openEv() -> i32;
  fn _ZN14QTemporaryFile15createLocalFileERK7QString(arg0: *const c_void) -> i32;
  fn _ZN14QTemporaryFile16createNativeFileER5QFile(arg0: *mut c_void) -> i32;
  fn _ZN14QTemporaryFile15setFileTemplateERK7QString(arg0: *const c_void) -> i32;
  fn _ZN14QTemporaryFileC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN14QTemporaryFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QTemporaryFile)=1
pub struct QTemporaryFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTemporaryFile {
  pub fn autoRemove<T: QTemporaryFile_autoRemove>(&mut self, value: T) -> i32 {
    value.autoRemove(self);
    return 1;
  }
}

pub trait QTemporaryFile_autoRemove {
  fn autoRemove(self, this: &mut QTemporaryFile) -> i32;
}

// proto: bool QTemporaryFile::autoRemove();
impl<'a> /*trait*/ QTemporaryFile_autoRemove for () {
  fn autoRemove(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile10autoRemoveEv()};
    unsafe {_ZNK14QTemporaryFile10autoRemoveEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn createLocalFile<T: QTemporaryFile_createLocalFile>(&mut self, value: T) -> i32 {
    value.createLocalFile(self);
    return 1;
  }
}

pub trait QTemporaryFile_createLocalFile {
  fn createLocalFile(self, this: &mut QTemporaryFile) -> i32;
}

// proto: QTemporaryFile * QTemporaryFile::createLocalFile(QFile & file);
impl<'a> /*trait*/ QTemporaryFile_createLocalFile for (&'a mut QFile) {
  fn createLocalFile(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15createLocalFileER5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFile15createLocalFileER5QFile(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
  pub fn FreeQTemporaryFile<T: QTemporaryFile_FreeQTemporaryFile>(&mut self, value: T) -> i32 {
    value.FreeQTemporaryFile(self);
    return 1;
  }
}

pub trait QTemporaryFile_FreeQTemporaryFile {
  fn FreeQTemporaryFile(self, this: &mut QTemporaryFile) -> i32;
}

// proto: void QTemporaryFile::FreeQTemporaryFile();
impl<'a> /*trait*/ QTemporaryFile_FreeQTemporaryFile for () {
  fn FreeQTemporaryFile(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileD0Ev()};
    unsafe {_ZN14QTemporaryFileD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn metaObject<T: QTemporaryFile_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTemporaryFile_metaObject {
  fn metaObject(self, this: &mut QTemporaryFile) -> i32;
}

// proto: const QMetaObject * QTemporaryFile::metaObject();
impl<'a> /*trait*/ QTemporaryFile_metaObject for () {
  fn metaObject(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile10metaObjectEv()};
    unsafe {_ZNK14QTemporaryFile10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn setAutoRemove<T: QTemporaryFile_setAutoRemove>(&mut self, value: T) -> i32 {
    value.setAutoRemove(self);
    return 1;
  }
}

pub trait QTemporaryFile_setAutoRemove {
  fn setAutoRemove(self, this: &mut QTemporaryFile) -> i32;
}

// proto: void QTemporaryFile::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryFile_setAutoRemove for (i8) {
  fn setAutoRemove(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile13setAutoRemoveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QTemporaryFile13setAutoRemoveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn fileName<T: QTemporaryFile_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QTemporaryFile_fileName {
  fn fileName(self, this: &mut QTemporaryFile) -> i32;
}

// proto: QString QTemporaryFile::fileName();
impl<'a> /*trait*/ QTemporaryFile_fileName for () {
  fn fileName(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile8fileNameEv()};
    unsafe {_ZNK14QTemporaryFile8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn fileTemplate<T: QTemporaryFile_fileTemplate>(&mut self, value: T) -> i32 {
    value.fileTemplate(self);
    return 1;
  }
}

pub trait QTemporaryFile_fileTemplate {
  fn fileTemplate(self, this: &mut QTemporaryFile) -> i32;
}

// proto: QString QTemporaryFile::fileTemplate();
impl<'a> /*trait*/ QTemporaryFile_fileTemplate for () {
  fn fileTemplate(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile12fileTemplateEv()};
    unsafe {_ZNK14QTemporaryFile12fileTemplateEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn createNativeFile<T: QTemporaryFile_createNativeFile>(&mut self, value: T) -> i32 {
    value.createNativeFile(self);
    return 1;
  }
}

pub trait QTemporaryFile_createNativeFile {
  fn createNativeFile(self, this: &mut QTemporaryFile) -> i32;
}

// proto: QTemporaryFile * QTemporaryFile::createNativeFile(const QString & fileName);
impl<'a> /*trait*/ QTemporaryFile_createNativeFile for (&'a  QString) {
  fn createNativeFile(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile16createNativeFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QTemporaryFile16createNativeFileERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn open<T: QTemporaryFile_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QTemporaryFile_open {
  fn open(self, this: &mut QTemporaryFile) -> i32;
}

// proto: bool QTemporaryFile::open();
impl<'a> /*trait*/ QTemporaryFile_open for () {
  fn open(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile4openEv()};
    unsafe {_ZN14QTemporaryFile4openEv()};
    return 1;
  }
}

// proto: QTemporaryFile * QTemporaryFile::createLocalFile(const QString & fileName);
impl<'a> /*trait*/ QTemporaryFile_createLocalFile for (&'a  QString) {
  fn createLocalFile(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15createLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QTemporaryFile15createLocalFileERK7QString(arg0)};
    return 1;
  }
}

// proto: QTemporaryFile * QTemporaryFile::createNativeFile(QFile & file);
impl<'a> /*trait*/ QTemporaryFile_createNativeFile for (&'a mut QFile) {
  fn createNativeFile(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile16createNativeFileER5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFile16createNativeFileER5QFile(arg0)};
    return 1;
  }
}

impl /*struct*/ QTemporaryFile {
  pub fn setFileTemplate<T: QTemporaryFile_setFileTemplate>(&mut self, value: T) -> i32 {
    value.setFileTemplate(self);
    return 1;
  }
}

pub trait QTemporaryFile_setFileTemplate {
  fn setFileTemplate(self, this: &mut QTemporaryFile) -> i32;
}

// proto: void QTemporaryFile::setFileTemplate(const QString & name);
impl<'a> /*trait*/ QTemporaryFile_setFileTemplate for (&'a  QString) {
  fn setFileTemplate(self, this: &mut QTemporaryFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15setFileTemplateERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QTemporaryFile15setFileTemplateERK7QString(arg0)};
    return 1;
  }
}

// proto: void QTemporaryFile::NewQTemporaryFile(const QTemporaryFile & );
impl<'a> /*trait*/ QTemporaryFile_NewQTemporaryFile for (&'a  QTemporaryFile) {
  fn NewQTemporaryFile(self) -> QTemporaryFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QTemporaryFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QTemporaryFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

