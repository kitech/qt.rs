// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qfile::QFile;
use super::qdir::QDir;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK9QFileInfo8isHiddenEv() -> i32;
  fn _ZN9QFileInfoC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK9QFileInfo14completeSuffixEv() -> i32;
  fn _ZNK9QFileInfo13canonicalPathEv() -> i32;
  fn _ZN9QFileInfo6existsERK7QString(arg0: *const c_void) -> i32;
  fn _ZN9QFileInfo12makeAbsoluteEv() -> i32;
  fn _ZNK9QFileInfo6isRootEv() -> i32;
  fn _ZNK9QFileInfo17canonicalFilePathEv() -> i32;
  fn _ZNK9QFileInfo5isDirEv() -> i32;
  fn _ZN9QFileInfoC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QFileInfo13symLinkTargetEv() -> i32;
  fn _ZN9QFileInfo7setFileERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QFileInfo6isFileEv() -> i32;
  fn _ZN9QFileInfoC1ERK5QFile(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QFileInfo7setFileERK5QFile(arg0: *const c_void) -> i32;
  fn _ZNK9QFileInfo7ownerIdEv() -> i32;
  fn _ZNK9QFileInfo8readLinkEv() -> i32;
  fn _ZNK9QFileInfo8filePathEv() -> i32;
  fn _ZN9QFileInfoC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QFileInfo9isSymLinkEv() -> i32;
  fn _ZNK9QFileInfo8lastReadEv() -> i32;
  fn _ZN9QFileInfo7refreshEv() -> i32;
  fn _ZN9QFileInfoC1ERK4QDirRK7QString(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK9QFileInfo4pathEv() -> i32;
  fn _ZNK9QFileInfo11absoluteDirEv() -> i32;
  fn _ZNK9QFileInfo8isBundleEv() -> i32;
  fn _ZN9QFileInfo7setFileERK4QDirRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK9QFileInfo10isRelativeEv() -> i32;
  fn _ZN9QFileInfo10setCachingEb(arg0: int8_t) -> i32;
  fn _ZNK9QFileInfo7createdEv() -> i32;
  fn _ZNK9QFileInfo7cachingEv() -> i32;
  fn _ZN9QFileInfoD0Ev() -> i32;
  fn _ZNK9QFileInfo16completeBaseNameEv() -> i32;
  fn _ZNK9QFileInfo8baseNameEv() -> i32;
  fn _ZNK9QFileInfo12isExecutableEv() -> i32;
  fn _ZNK9QFileInfo10bundleNameEv() -> i32;
  fn _ZNK9QFileInfo7groupIdEv() -> i32;
  fn _ZNK9QFileInfo8fileNameEv() -> i32;
  fn _ZNK9QFileInfo4sizeEv() -> i32;
  fn _ZNK9QFileInfo16absoluteFilePathEv() -> i32;
  fn _ZNK9QFileInfo6suffixEv() -> i32;
  fn _ZNK9QFileInfo5groupEv() -> i32;
  fn _ZNK9QFileInfo10isAbsoluteEv() -> i32;
  fn _ZNK9QFileInfo12isNativePathEv() -> i32;
  fn _ZNK9QFileInfo10isWritableEv() -> i32;
  fn _ZNK9QFileInfo5ownerEv() -> i32;
  fn _ZNK9QFileInfo10isReadableEv() -> i32;
  fn _ZNK9QFileInfo3dirEv() -> i32;
  fn _ZN9QFileInfo4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK9QFileInfo6existsEv() -> i32;
  fn _ZNK9QFileInfo12lastModifiedEv() -> i32;
  fn _ZNK9QFileInfo12absolutePathEv() -> i32;
}

// body block begin
// class sizeof(QFileInfo)=1
pub struct QFileInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileInfo {
  pub fn isHidden<T: QFileInfo_isHidden>(&mut self, value: T) -> i32 {
    value.isHidden(self);
    return 1;
  }
}

pub trait QFileInfo_isHidden {
  fn isHidden(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isHidden();
impl<'a> /*trait*/ QFileInfo_isHidden for () {
  fn isHidden(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isHiddenEv()};
    unsafe {_ZNK9QFileInfo8isHiddenEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn NewQFileInfo<T: QFileInfo_NewQFileInfo>(value: T) -> QFileInfo {
    let rsthis = value.NewQFileInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_NewQFileInfo {
  fn NewQFileInfo(self) -> QFileInfo;
}

// proto: void QFileInfo::NewQFileInfo();
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for () {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1Ev()};
    unsafe {_ZN9QFileInfoC1Ev(qthis)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn completeSuffix<T: QFileInfo_completeSuffix>(&mut self, value: T) -> i32 {
    value.completeSuffix(self);
    return 1;
  }
}

pub trait QFileInfo_completeSuffix {
  fn completeSuffix(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::completeSuffix();
impl<'a> /*trait*/ QFileInfo_completeSuffix for () {
  fn completeSuffix(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo14completeSuffixEv()};
    unsafe {_ZNK9QFileInfo14completeSuffixEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn canonicalPath<T: QFileInfo_canonicalPath>(&mut self, value: T) -> i32 {
    value.canonicalPath(self);
    return 1;
  }
}

pub trait QFileInfo_canonicalPath {
  fn canonicalPath(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::canonicalPath();
impl<'a> /*trait*/ QFileInfo_canonicalPath for () {
  fn canonicalPath(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13canonicalPathEv()};
    unsafe {_ZNK9QFileInfo13canonicalPathEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn exists<T: QFileInfo_exists>(&mut self, value: T) -> i32 {
    value.exists(self);
    return 1;
  }
}

pub trait QFileInfo_exists {
  fn exists(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::exists(const QString & file);
impl<'a> /*trait*/ QFileInfo_exists for (&'a  QString) {
  fn exists(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo6existsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfo6existsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn makeAbsolute<T: QFileInfo_makeAbsolute>(&mut self, value: T) -> i32 {
    value.makeAbsolute(self);
    return 1;
  }
}

pub trait QFileInfo_makeAbsolute {
  fn makeAbsolute(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::makeAbsolute();
impl<'a> /*trait*/ QFileInfo_makeAbsolute for () {
  fn makeAbsolute(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo12makeAbsoluteEv()};
    unsafe {_ZN9QFileInfo12makeAbsoluteEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isRoot<T: QFileInfo_isRoot>(&mut self, value: T) -> i32 {
    value.isRoot(self);
    return 1;
  }
}

pub trait QFileInfo_isRoot {
  fn isRoot(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isRoot();
impl<'a> /*trait*/ QFileInfo_isRoot for () {
  fn isRoot(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isRootEv()};
    unsafe {_ZNK9QFileInfo6isRootEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn canonicalFilePath<T: QFileInfo_canonicalFilePath>(&mut self, value: T) -> i32 {
    value.canonicalFilePath(self);
    return 1;
  }
}

pub trait QFileInfo_canonicalFilePath {
  fn canonicalFilePath(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::canonicalFilePath();
impl<'a> /*trait*/ QFileInfo_canonicalFilePath for () {
  fn canonicalFilePath(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo17canonicalFilePathEv()};
    unsafe {_ZNK9QFileInfo17canonicalFilePathEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isDir<T: QFileInfo_isDir>(&mut self, value: T) -> i32 {
    value.isDir(self);
    return 1;
  }
}

pub trait QFileInfo_isDir {
  fn isDir(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isDir();
impl<'a> /*trait*/ QFileInfo_isDir for () {
  fn isDir(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5isDirEv()};
    unsafe {_ZNK9QFileInfo5isDirEv()};
    return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QString & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QString) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfoC1ERK7QString(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn symLinkTarget<T: QFileInfo_symLinkTarget>(&mut self, value: T) -> i32 {
    value.symLinkTarget(self);
    return 1;
  }
}

pub trait QFileInfo_symLinkTarget {
  fn symLinkTarget(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::symLinkTarget();
impl<'a> /*trait*/ QFileInfo_symLinkTarget for () {
  fn symLinkTarget(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13symLinkTargetEv()};
    unsafe {_ZNK9QFileInfo13symLinkTargetEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn setFile<T: QFileInfo_setFile>(&mut self, value: T) -> i32 {
    value.setFile(self);
    return 1;
  }
}

pub trait QFileInfo_setFile {
  fn setFile(self, this: &mut QFileInfo) -> i32;
}

// proto: void QFileInfo::setFile(const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile for (&'a  QString) {
  fn setFile(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfo7setFileERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isFile<T: QFileInfo_isFile>(&mut self, value: T) -> i32 {
    value.isFile(self);
    return 1;
  }
}

pub trait QFileInfo_isFile {
  fn isFile(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isFile();
impl<'a> /*trait*/ QFileInfo_isFile for () {
  fn isFile(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isFileEv()};
    unsafe {_ZNK9QFileInfo6isFileEv()};
    return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QFile & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QFile) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK5QFile()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfoC1ERK5QFile(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFileInfo::setFile(const QFile & file);
impl<'a> /*trait*/ QFileInfo_setFile for (&'a  QFile) {
  fn setFile(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK5QFile()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfo7setFileERK5QFile(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn ownerId<T: QFileInfo_ownerId>(&mut self, value: T) -> i32 {
    value.ownerId(self);
    return 1;
  }
}

pub trait QFileInfo_ownerId {
  fn ownerId(self, this: &mut QFileInfo) -> i32;
}

// proto: unsigned int QFileInfo::ownerId();
impl<'a> /*trait*/ QFileInfo_ownerId for () {
  fn ownerId(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7ownerIdEv()};
    unsafe {_ZNK9QFileInfo7ownerIdEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn readLink<T: QFileInfo_readLink>(&mut self, value: T) -> i32 {
    value.readLink(self);
    return 1;
  }
}

pub trait QFileInfo_readLink {
  fn readLink(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::readLink();
impl<'a> /*trait*/ QFileInfo_readLink for () {
  fn readLink(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8readLinkEv()};
    unsafe {_ZNK9QFileInfo8readLinkEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn filePath<T: QFileInfo_filePath>(&mut self, value: T) -> i32 {
    value.filePath(self);
    return 1;
  }
}

pub trait QFileInfo_filePath {
  fn filePath(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::filePath();
impl<'a> /*trait*/ QFileInfo_filePath for () {
  fn filePath(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8filePathEv()};
    unsafe {_ZNK9QFileInfo8filePathEv()};
    return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QFileInfo & fileinfo);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QFileInfo) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfoC1ERKS_(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isSymLink<T: QFileInfo_isSymLink>(&mut self, value: T) -> i32 {
    value.isSymLink(self);
    return 1;
  }
}

pub trait QFileInfo_isSymLink {
  fn isSymLink(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isSymLink();
impl<'a> /*trait*/ QFileInfo_isSymLink for () {
  fn isSymLink(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo9isSymLinkEv()};
    unsafe {_ZNK9QFileInfo9isSymLinkEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn lastRead<T: QFileInfo_lastRead>(&mut self, value: T) -> i32 {
    value.lastRead(self);
    return 1;
  }
}

pub trait QFileInfo_lastRead {
  fn lastRead(self, this: &mut QFileInfo) -> i32;
}

// proto: QDateTime QFileInfo::lastRead();
impl<'a> /*trait*/ QFileInfo_lastRead for () {
  fn lastRead(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8lastReadEv()};
    unsafe {_ZNK9QFileInfo8lastReadEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn refresh<T: QFileInfo_refresh>(&mut self, value: T) -> i32 {
    value.refresh(self);
    return 1;
  }
}

pub trait QFileInfo_refresh {
  fn refresh(self, this: &mut QFileInfo) -> i32;
}

// proto: void QFileInfo::refresh();
impl<'a> /*trait*/ QFileInfo_refresh for () {
  fn refresh(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7refreshEv()};
    unsafe {_ZN9QFileInfo7refreshEv()};
    return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QDir, &'a  QString) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfoC1ERK4QDirRK7QString(qthis, arg0, arg1)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn path<T: QFileInfo_path>(&mut self, value: T) -> i32 {
    value.path(self);
    return 1;
  }
}

pub trait QFileInfo_path {
  fn path(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::path();
impl<'a> /*trait*/ QFileInfo_path for () {
  fn path(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4pathEv()};
    unsafe {_ZNK9QFileInfo4pathEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absoluteDir<T: QFileInfo_absoluteDir>(&mut self, value: T) -> i32 {
    value.absoluteDir(self);
    return 1;
  }
}

pub trait QFileInfo_absoluteDir {
  fn absoluteDir(self, this: &mut QFileInfo) -> i32;
}

// proto: QDir QFileInfo::absoluteDir();
impl<'a> /*trait*/ QFileInfo_absoluteDir for () {
  fn absoluteDir(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo11absoluteDirEv()};
    unsafe {_ZNK9QFileInfo11absoluteDirEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isBundle<T: QFileInfo_isBundle>(&mut self, value: T) -> i32 {
    value.isBundle(self);
    return 1;
  }
}

pub trait QFileInfo_isBundle {
  fn isBundle(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isBundle();
impl<'a> /*trait*/ QFileInfo_isBundle for () {
  fn isBundle(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isBundleEv()};
    unsafe {_ZNK9QFileInfo8isBundleEv()};
    return 1;
  }
}

// proto: void QFileInfo::setFile(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile for (&'a  QDir, &'a  QString) {
  fn setFile(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QFileInfo7setFileERK4QDirRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isRelative<T: QFileInfo_isRelative>(&mut self, value: T) -> i32 {
    value.isRelative(self);
    return 1;
  }
}

pub trait QFileInfo_isRelative {
  fn isRelative(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isRelative();
impl<'a> /*trait*/ QFileInfo_isRelative for () {
  fn isRelative(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isRelativeEv()};
    unsafe {_ZNK9QFileInfo10isRelativeEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn setCaching<T: QFileInfo_setCaching>(&mut self, value: T) -> i32 {
    value.setCaching(self);
    return 1;
  }
}

pub trait QFileInfo_setCaching {
  fn setCaching(self, this: &mut QFileInfo) -> i32;
}

// proto: void QFileInfo::setCaching(bool on);
impl<'a> /*trait*/ QFileInfo_setCaching for (i8) {
  fn setCaching(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo10setCachingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QFileInfo10setCachingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn created<T: QFileInfo_created>(&mut self, value: T) -> i32 {
    value.created(self);
    return 1;
  }
}

pub trait QFileInfo_created {
  fn created(self, this: &mut QFileInfo) -> i32;
}

// proto: QDateTime QFileInfo::created();
impl<'a> /*trait*/ QFileInfo_created for () {
  fn created(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7createdEv()};
    unsafe {_ZNK9QFileInfo7createdEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn caching<T: QFileInfo_caching>(&mut self, value: T) -> i32 {
    value.caching(self);
    return 1;
  }
}

pub trait QFileInfo_caching {
  fn caching(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::caching();
impl<'a> /*trait*/ QFileInfo_caching for () {
  fn caching(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7cachingEv()};
    unsafe {_ZNK9QFileInfo7cachingEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn FreeQFileInfo<T: QFileInfo_FreeQFileInfo>(&mut self, value: T) -> i32 {
    value.FreeQFileInfo(self);
    return 1;
  }
}

pub trait QFileInfo_FreeQFileInfo {
  fn FreeQFileInfo(self, this: &mut QFileInfo) -> i32;
}

// proto: void QFileInfo::FreeQFileInfo();
impl<'a> /*trait*/ QFileInfo_FreeQFileInfo for () {
  fn FreeQFileInfo(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoD0Ev()};
    unsafe {_ZN9QFileInfoD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn completeBaseName<T: QFileInfo_completeBaseName>(&mut self, value: T) -> i32 {
    value.completeBaseName(self);
    return 1;
  }
}

pub trait QFileInfo_completeBaseName {
  fn completeBaseName(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::completeBaseName();
impl<'a> /*trait*/ QFileInfo_completeBaseName for () {
  fn completeBaseName(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16completeBaseNameEv()};
    unsafe {_ZNK9QFileInfo16completeBaseNameEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn baseName<T: QFileInfo_baseName>(&mut self, value: T) -> i32 {
    value.baseName(self);
    return 1;
  }
}

pub trait QFileInfo_baseName {
  fn baseName(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::baseName();
impl<'a> /*trait*/ QFileInfo_baseName for () {
  fn baseName(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8baseNameEv()};
    unsafe {_ZNK9QFileInfo8baseNameEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isExecutable<T: QFileInfo_isExecutable>(&mut self, value: T) -> i32 {
    value.isExecutable(self);
    return 1;
  }
}

pub trait QFileInfo_isExecutable {
  fn isExecutable(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isExecutable();
impl<'a> /*trait*/ QFileInfo_isExecutable for () {
  fn isExecutable(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isExecutableEv()};
    unsafe {_ZNK9QFileInfo12isExecutableEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn bundleName<T: QFileInfo_bundleName>(&mut self, value: T) -> i32 {
    value.bundleName(self);
    return 1;
  }
}

pub trait QFileInfo_bundleName {
  fn bundleName(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::bundleName();
impl<'a> /*trait*/ QFileInfo_bundleName for () {
  fn bundleName(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10bundleNameEv()};
    unsafe {_ZNK9QFileInfo10bundleNameEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn groupId<T: QFileInfo_groupId>(&mut self, value: T) -> i32 {
    value.groupId(self);
    return 1;
  }
}

pub trait QFileInfo_groupId {
  fn groupId(self, this: &mut QFileInfo) -> i32;
}

// proto: unsigned int QFileInfo::groupId();
impl<'a> /*trait*/ QFileInfo_groupId for () {
  fn groupId(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7groupIdEv()};
    unsafe {_ZNK9QFileInfo7groupIdEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn fileName<T: QFileInfo_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QFileInfo_fileName {
  fn fileName(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::fileName();
impl<'a> /*trait*/ QFileInfo_fileName for () {
  fn fileName(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8fileNameEv()};
    unsafe {_ZNK9QFileInfo8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn size<T: QFileInfo_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QFileInfo_size {
  fn size(self, this: &mut QFileInfo) -> i32;
}

// proto: long long QFileInfo::size();
impl<'a> /*trait*/ QFileInfo_size for () {
  fn size(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4sizeEv()};
    unsafe {_ZNK9QFileInfo4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absoluteFilePath<T: QFileInfo_absoluteFilePath>(&mut self, value: T) -> i32 {
    value.absoluteFilePath(self);
    return 1;
  }
}

pub trait QFileInfo_absoluteFilePath {
  fn absoluteFilePath(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::absoluteFilePath();
impl<'a> /*trait*/ QFileInfo_absoluteFilePath for () {
  fn absoluteFilePath(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16absoluteFilePathEv()};
    unsafe {_ZNK9QFileInfo16absoluteFilePathEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn suffix<T: QFileInfo_suffix>(&mut self, value: T) -> i32 {
    value.suffix(self);
    return 1;
  }
}

pub trait QFileInfo_suffix {
  fn suffix(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::suffix();
impl<'a> /*trait*/ QFileInfo_suffix for () {
  fn suffix(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6suffixEv()};
    unsafe {_ZNK9QFileInfo6suffixEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn group<T: QFileInfo_group>(&mut self, value: T) -> i32 {
    value.group(self);
    return 1;
  }
}

pub trait QFileInfo_group {
  fn group(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::group();
impl<'a> /*trait*/ QFileInfo_group for () {
  fn group(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5groupEv()};
    unsafe {_ZNK9QFileInfo5groupEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isAbsolute<T: QFileInfo_isAbsolute>(&mut self, value: T) -> i32 {
    value.isAbsolute(self);
    return 1;
  }
}

pub trait QFileInfo_isAbsolute {
  fn isAbsolute(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isAbsolute();
impl<'a> /*trait*/ QFileInfo_isAbsolute for () {
  fn isAbsolute(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isAbsoluteEv()};
    unsafe {_ZNK9QFileInfo10isAbsoluteEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isNativePath<T: QFileInfo_isNativePath>(&mut self, value: T) -> i32 {
    value.isNativePath(self);
    return 1;
  }
}

pub trait QFileInfo_isNativePath {
  fn isNativePath(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isNativePath();
impl<'a> /*trait*/ QFileInfo_isNativePath for () {
  fn isNativePath(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isNativePathEv()};
    unsafe {_ZNK9QFileInfo12isNativePathEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isWritable<T: QFileInfo_isWritable>(&mut self, value: T) -> i32 {
    value.isWritable(self);
    return 1;
  }
}

pub trait QFileInfo_isWritable {
  fn isWritable(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isWritable();
impl<'a> /*trait*/ QFileInfo_isWritable for () {
  fn isWritable(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isWritableEv()};
    unsafe {_ZNK9QFileInfo10isWritableEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn owner<T: QFileInfo_owner>(&mut self, value: T) -> i32 {
    value.owner(self);
    return 1;
  }
}

pub trait QFileInfo_owner {
  fn owner(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::owner();
impl<'a> /*trait*/ QFileInfo_owner for () {
  fn owner(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5ownerEv()};
    unsafe {_ZNK9QFileInfo5ownerEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isReadable<T: QFileInfo_isReadable>(&mut self, value: T) -> i32 {
    value.isReadable(self);
    return 1;
  }
}

pub trait QFileInfo_isReadable {
  fn isReadable(self, this: &mut QFileInfo) -> i32;
}

// proto: bool QFileInfo::isReadable();
impl<'a> /*trait*/ QFileInfo_isReadable for () {
  fn isReadable(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isReadableEv()};
    unsafe {_ZNK9QFileInfo10isReadableEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn dir<T: QFileInfo_dir>(&mut self, value: T) -> i32 {
    value.dir(self);
    return 1;
  }
}

pub trait QFileInfo_dir {
  fn dir(self, this: &mut QFileInfo) -> i32;
}

// proto: QDir QFileInfo::dir();
impl<'a> /*trait*/ QFileInfo_dir for () {
  fn dir(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo3dirEv()};
    unsafe {_ZNK9QFileInfo3dirEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn swap<T: QFileInfo_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QFileInfo_swap {
  fn swap(self, this: &mut QFileInfo) -> i32;
}

// proto: void QFileInfo::swap(QFileInfo & other);
impl<'a> /*trait*/ QFileInfo_swap for (&'a mut QFileInfo) {
  fn swap(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfo4swapERS_(arg0)};
    return 1;
  }
}

// proto: bool QFileInfo::exists();
impl<'a> /*trait*/ QFileInfo_exists for () {
  fn exists(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6existsEv()};
    unsafe {_ZNK9QFileInfo6existsEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn lastModified<T: QFileInfo_lastModified>(&mut self, value: T) -> i32 {
    value.lastModified(self);
    return 1;
  }
}

pub trait QFileInfo_lastModified {
  fn lastModified(self, this: &mut QFileInfo) -> i32;
}

// proto: QDateTime QFileInfo::lastModified();
impl<'a> /*trait*/ QFileInfo_lastModified for () {
  fn lastModified(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12lastModifiedEv()};
    unsafe {_ZNK9QFileInfo12lastModifiedEv()};
    return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absolutePath<T: QFileInfo_absolutePath>(&mut self, value: T) -> i32 {
    value.absolutePath(self);
    return 1;
  }
}

pub trait QFileInfo_absolutePath {
  fn absolutePath(self, this: &mut QFileInfo) -> i32;
}

// proto: QString QFileInfo::absolutePath();
impl<'a> /*trait*/ QFileInfo_absolutePath for () {
  fn absolutePath(self, this: &mut QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12absolutePathEv()};
    unsafe {_ZNK9QFileInfo12absolutePathEv()};
    return 1;
  }
}

