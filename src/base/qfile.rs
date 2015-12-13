// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK5QFile13symLinkTargetEv() -> i32;
  fn _ZN5QFileC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN5QFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN5QFile4linkERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QFile6renameERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QFile4linkERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFile6resizeERK7QStringx(arg0: *const c_void, arg1: c_longlong) -> i32;
  fn _ZN5QFile6existsERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFileD0Ev() -> i32;
  fn _ZN5QFile4copyERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QFile8readLinkERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK5QFile6existsEv() -> i32;
  fn _ZNK5QFile4sizeEv() -> i32;
  fn _ZN5QFile6resizeEx(arg0: c_longlong) -> i32;
  fn _ZN5QFileC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN5QFile11setFileNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFile14setPermissionsE6QFlagsIN11QFileDevice10PermissionEE(arg0: c_int) -> i32;
  fn _ZN5QFile6removeEv() -> i32;
  fn _ZN5QFile4openE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: c_int) -> i32;
  fn _ZN5QFile4copyERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFile10encodeNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFile14setPermissionsERK7QString6QFlagsIN11QFileDevice10PermissionEE(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN5QFile10decodeNameERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN5QFile6renameERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK5QFile8fileNameEv() -> i32;
  fn _ZN5QFile10decodeNameEPKc(arg0: *const c_char) -> i32;
  fn _ZNK5QFile10metaObjectEv() -> i32;
  fn _ZN5QFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZN5QFile13symLinkTargetERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFile4openEi6QFlagsIN9QIODevice12OpenModeFlagEES0_IN11QFileDevice14FileHandleFlagEE(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  fn _ZN5QFile6removeERK7QString(arg0: *const c_void) -> i32;
  fn _ZN5QFileC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK5QFile8readLinkEv() -> i32;
}

// body block begin
// class sizeof(QFile)=1
pub struct QFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFile {
  pub fn symLinkTarget<T: QFile_symLinkTarget>(&mut self, value: T) -> i32 {
    value.symLinkTarget(self);
    return 1;
  }
}

pub trait QFile_symLinkTarget {
  fn symLinkTarget(self, this: &mut QFile) -> i32;
}

// proto: QString QFile::symLinkTarget();
impl<'a> /*trait*/ QFile_symLinkTarget for () {
  fn symLinkTarget(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile13symLinkTargetEv()};
    unsafe {_ZNK5QFile13symLinkTargetEv()};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn NewQFile<T: QFile_NewQFile>(value: T) -> QFile {
    let rsthis = value.NewQFile();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_NewQFile {
  fn NewQFile(self) -> QFile;
}

// proto: void QFile::NewQFile();
impl<'a> /*trait*/ QFile_NewQFile for () {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1Ev()};
    unsafe {_ZN5QFileC1Ev(qthis)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFile::NewQFile(QObject * parent);
impl<'a> /*trait*/ QFile_NewQFile for (&'a mut QObject) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1EP7QObject(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn link<T: QFile_link>(&mut self, value: T) -> i32 {
    value.link(self);
    return 1;
  }
}

pub trait QFile_link {
  fn link(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::link(const QString & oldname, const QString & newName);
impl<'a> /*trait*/ QFile_link for (&'a  QString, &'a  QString) {
  fn link(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4linkERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QFile4linkERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn rename<T: QFile_rename>(&mut self, value: T) -> i32 {
    value.rename(self);
    return 1;
  }
}

pub trait QFile_rename {
  fn rename(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QFile_rename for (&'a  QString, &'a  QString) {
  fn rename(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6renameERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QFile6renameERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: bool QFile::link(const QString & newName);
impl<'a> /*trait*/ QFile_link for (&'a  QString) {
  fn link(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4linkERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile4linkERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn resize<T: QFile_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QFile_resize {
  fn resize(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::resize(const QString & filename, qint64 sz);
impl<'a> /*trait*/ QFile_resize for (&'a  QString, i64) {
  fn resize(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6resizeERK7QStringx()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_longlong;
    unsafe {_ZN5QFile6resizeERK7QStringx(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn exists<T: QFile_exists>(&mut self, value: T) -> i32 {
    value.exists(self);
    return 1;
  }
}

pub trait QFile_exists {
  fn exists(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::exists(const QString & fileName);
impl<'a> /*trait*/ QFile_exists for (&'a  QString) {
  fn exists(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6existsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile6existsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn FreeQFile<T: QFile_FreeQFile>(&mut self, value: T) -> i32 {
    value.FreeQFile(self);
    return 1;
  }
}

pub trait QFile_FreeQFile {
  fn FreeQFile(self, this: &mut QFile) -> i32;
}

// proto: void QFile::FreeQFile();
impl<'a> /*trait*/ QFile_FreeQFile for () {
  fn FreeQFile(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileD0Ev()};
    unsafe {_ZN5QFileD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn copy<T: QFile_copy>(&mut self, value: T) -> i32 {
    value.copy(self);
    return 1;
  }
}

pub trait QFile_copy {
  fn copy(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::copy(const QString & fileName, const QString & newName);
impl<'a> /*trait*/ QFile_copy for (&'a  QString, &'a  QString) {
  fn copy(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4copyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QFile4copyERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn readLink<T: QFile_readLink>(&mut self, value: T) -> i32 {
    value.readLink(self);
    return 1;
  }
}

pub trait QFile_readLink {
  fn readLink(self, this: &mut QFile) -> i32;
}

// proto: QString QFile::readLink(const QString & fileName);
impl<'a> /*trait*/ QFile_readLink for (&'a  QString) {
  fn readLink(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile8readLinkERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile8readLinkERK7QString(arg0)};
    return 1;
  }
}

// proto: bool QFile::exists();
impl<'a> /*trait*/ QFile_exists for () {
  fn exists(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile6existsEv()};
    unsafe {_ZNK5QFile6existsEv()};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn size<T: QFile_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QFile_size {
  fn size(self, this: &mut QFile) -> i32;
}

// proto: long long QFile::size();
impl<'a> /*trait*/ QFile_size for () {
  fn size(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile4sizeEv()};
    unsafe {_ZNK5QFile4sizeEv()};
    return 1;
  }
}

// proto: bool QFile::resize(qint64 sz);
impl<'a> /*trait*/ QFile_resize for (i64) {
  fn resize(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6resizeEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN5QFile6resizeEx(arg0)};
    return 1;
  }
}

// proto: void QFile::NewQFile(const QFile & );
impl<'a> /*trait*/ QFile_NewQFile for (&'a  QFile) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFileC1ERKS_(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn setFileName<T: QFile_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QFile_setFileName {
  fn setFileName(self, this: &mut QFile) -> i32;
}

// proto: void QFile::setFileName(const QString & name);
impl<'a> /*trait*/ QFile_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile11setFileNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn setPermissions<T: QFile_setPermissions>(&mut self, value: T) -> i32 {
    value.setPermissions(self);
    return 1;
  }
}

pub trait QFile_setPermissions {
  fn setPermissions(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::setPermissions(Permissions permissionSpec);
impl<'a> /*trait*/ QFile_setPermissions for (i32) {
  fn setPermissions(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile14setPermissionsE6QFlagsIN11QFileDevice10PermissionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFile14setPermissionsE6QFlagsIN11QFileDevice10PermissionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn remove<T: QFile_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QFile_remove {
  fn remove(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::remove();
impl<'a> /*trait*/ QFile_remove for () {
  fn remove(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6removeEv()};
    unsafe {_ZN5QFile6removeEv()};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn open<T: QFile_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QFile_open {
  fn open(self, this: &mut QFile) -> i32;
}

// proto: bool QFile::open(OpenMode flags);
impl<'a> /*trait*/ QFile_open for (i32) {
  fn open(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4openE6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFile4openE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0)};
    return 1;
  }
}

// proto: bool QFile::copy(const QString & newName);
impl<'a> /*trait*/ QFile_copy for (&'a  QString) {
  fn copy(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4copyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile4copyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn encodeName<T: QFile_encodeName>(&mut self, value: T) -> i32 {
    value.encodeName(self);
    return 1;
  }
}

pub trait QFile_encodeName {
  fn encodeName(self, this: &mut QFile) -> i32;
}

// proto: QByteArray QFile::encodeName(const QString & fileName);
impl<'a> /*trait*/ QFile_encodeName for (&'a  QString) {
  fn encodeName(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10encodeNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile10encodeNameERK7QString(arg0)};
    return 1;
  }
}

// proto: bool QFile::setPermissions(const QString & filename, Permissions permissionSpec);
impl<'a> /*trait*/ QFile_setPermissions for (&'a  QString, i32) {
  fn setPermissions(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile14setPermissionsERK7QString6QFlagsIN11QFileDevice10PermissionEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QFile14setPermissionsERK7QString6QFlagsIN11QFileDevice10PermissionEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn decodeName<T: QFile_decodeName>(&mut self, value: T) -> i32 {
    value.decodeName(self);
    return 1;
  }
}

pub trait QFile_decodeName {
  fn decodeName(self, this: &mut QFile) -> i32;
}

// proto: QString QFile::decodeName(const QByteArray & localFileName);
impl<'a> /*trait*/ QFile_decodeName for (&'a  QByteArray) {
  fn decodeName(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10decodeNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile10decodeNameERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: bool QFile::rename(const QString & newName);
impl<'a> /*trait*/ QFile_rename for (&'a  QString) {
  fn rename(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6renameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile6renameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn fileName<T: QFile_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QFile_fileName {
  fn fileName(self, this: &mut QFile) -> i32;
}

// proto: QString QFile::fileName();
impl<'a> /*trait*/ QFile_fileName for () {
  fn fileName(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile8fileNameEv()};
    unsafe {_ZNK5QFile8fileNameEv()};
    return 1;
  }
}

// proto: QString QFile::decodeName(const char * localFileName);
impl<'a> /*trait*/ QFile_decodeName for (&'a  String) {
  fn decodeName(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10decodeNameEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN5QFile10decodeNameEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QFile {
  pub fn metaObject<T: QFile_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFile_metaObject {
  fn metaObject(self, this: &mut QFile) -> i32;
}

// proto: const QMetaObject * QFile::metaObject();
impl<'a> /*trait*/ QFile_metaObject for () {
  fn metaObject(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile10metaObjectEv()};
    unsafe {_ZNK5QFile10metaObjectEv()};
    return 1;
  }
}

// proto: void QFile::NewQFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QFile_NewQFile for (&'a  QString, &'a mut QObject) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QString QFile::symLinkTarget(const QString & fileName);
impl<'a> /*trait*/ QFile_symLinkTarget for (&'a  QString) {
  fn symLinkTarget(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile13symLinkTargetERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile13symLinkTargetERK7QString(arg0)};
    return 1;
  }
}

// proto: bool QFile::open(int fd, OpenMode ioFlags, FileHandleFlags handleFlags);
impl<'a> /*trait*/ QFile_open for (i32, i32, i32) {
  fn open(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4openEi6QFlagsIN9QIODevice12OpenModeFlagEES0_IN11QFileDevice14FileHandleFlagEE()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN5QFile4openEi6QFlagsIN9QIODevice12OpenModeFlagEES0_IN11QFileDevice14FileHandleFlagEE(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: bool QFile::remove(const QString & fileName);
impl<'a> /*trait*/ QFile_remove for (&'a  QString) {
  fn remove(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6removeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFile6removeERK7QString(arg0)};
    return 1;
  }
}

// proto: void QFile::NewQFile(const QString & name);
impl<'a> /*trait*/ QFile_NewQFile for (&'a  QString) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QFileC1ERK7QString(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QString QFile::readLink();
impl<'a> /*trait*/ QFile_readLink for () {
  fn readLink(self, this: &mut QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile8readLinkEv()};
    unsafe {_ZNK5QFile8readLinkEv()};
    return 1;
  }
}

