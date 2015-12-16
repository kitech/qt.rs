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
use super::qdatetime::QDateTime;
use super::qdir::QDir;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QFileInfo::isHidden();
  fn _ZNK9QFileInfo8isHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::NewQFileInfo();
  fn _ZN9QFileInfoC1Ev(qthis: *mut c_void) ;
  // proto:  QString QFileInfo::completeSuffix();
  fn _ZNK9QFileInfo14completeSuffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::canonicalPath();
  fn _ZNK9QFileInfo13canonicalPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QFileInfo::exists(const QString & file);
  fn _ZN9QFileInfo6existsERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::makeAbsolute();
  fn _ZN9QFileInfo12makeAbsoluteEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::isRoot();
  fn _ZNK9QFileInfo6isRootEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileInfo::canonicalFilePath();
  fn _ZNK9QFileInfo17canonicalFilePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isDir();
  fn _ZNK9QFileInfo5isDirEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::NewQFileInfo(const QString & file);
  fn _ZN9QFileInfoC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFileInfo::symLinkTarget();
  fn _ZNK9QFileInfo13symLinkTargetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::setFile(const QString & file);
  fn _ZN9QFileInfo7setFileERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileInfo::isFile();
  fn _ZNK9QFileInfo6isFileEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::NewQFileInfo(const QFile & file);
  fn _ZN9QFileInfoC1ERK5QFile(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileInfo::setFile(const QFile & file);
  fn _ZN9QFileInfo7setFileERK5QFile(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  unsigned int QFileInfo::ownerId();
  fn _ZNK9QFileInfo7ownerIdEv(qthis: *mut c_void) -> c_uint;
  // proto:  QString QFileInfo::readLink();
  fn _ZNK9QFileInfo8readLinkEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::filePath();
  fn _ZNK9QFileInfo8filePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::NewQFileInfo(const QFileInfo & fileinfo);
  fn _ZN9QFileInfoC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileInfo::isSymLink();
  fn _ZNK9QFileInfo9isSymLinkEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDateTime QFileInfo::lastRead();
  fn _ZNK9QFileInfo8lastReadEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::refresh();
  fn _ZN9QFileInfo7refreshEv(qthis: *mut c_void) ;
  // proto:  void QFileInfo::NewQFileInfo(const QDir & dir, const QString & file);
  fn _ZN9QFileInfoC1ERK4QDirRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QFileInfo::path();
  fn _ZNK9QFileInfo4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDir QFileInfo::absoluteDir();
  fn _ZNK9QFileInfo11absoluteDirEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isBundle();
  fn _ZNK9QFileInfo8isBundleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::setFile(const QDir & dir, const QString & file);
  fn _ZN9QFileInfo7setFileERK4QDirRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QFileInfo::isRelative();
  fn _ZNK9QFileInfo10isRelativeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::setCaching(bool on);
  fn _ZN9QFileInfo10setCachingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QDateTime QFileInfo::created();
  fn _ZNK9QFileInfo7createdEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::caching();
  fn _ZNK9QFileInfo7cachingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::FreeQFileInfo();
  fn _ZN9QFileInfoD0Ev(qthis: *mut c_void) ;
  // proto:  QString QFileInfo::completeBaseName();
  fn _ZNK9QFileInfo16completeBaseNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::baseName();
  fn _ZNK9QFileInfo8baseNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isExecutable();
  fn _ZNK9QFileInfo12isExecutableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileInfo::bundleName();
  fn _ZNK9QFileInfo10bundleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  unsigned int QFileInfo::groupId();
  fn _ZNK9QFileInfo7groupIdEv(qthis: *mut c_void) -> c_uint;
  // proto:  QString QFileInfo::fileName();
  fn _ZNK9QFileInfo8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QFileInfo::size();
  fn _ZNK9QFileInfo4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QString QFileInfo::absoluteFilePath();
  fn _ZNK9QFileInfo16absoluteFilePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::suffix();
  fn _ZNK9QFileInfo6suffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::group();
  fn _ZNK9QFileInfo5groupEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isAbsolute();
  fn _ZNK9QFileInfo10isAbsoluteEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::isNativePath();
  fn _ZNK9QFileInfo12isNativePathEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::isWritable();
  fn _ZNK9QFileInfo10isWritableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileInfo::owner();
  fn _ZNK9QFileInfo5ownerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isReadable();
  fn _ZNK9QFileInfo10isReadableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDir QFileInfo::dir();
  fn _ZNK9QFileInfo3dirEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::swap(QFileInfo & other);
  fn _ZN9QFileInfo4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileInfo::exists();
  fn _ZNK9QFileInfo6existsEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDateTime QFileInfo::lastModified();
  fn _ZNK9QFileInfo12lastModifiedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::absolutePath();
  fn _ZNK9QFileInfo12absolutePathEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QFileInfo)=1
pub struct QFileInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileInfo {
  pub fn isHidden<T: QFileInfo_isHidden>(&mut self, value: T) -> i8 {
    return value.isHidden(self);
    // return 1;
  }
}

pub trait QFileInfo_isHidden {
  fn isHidden(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isHidden();
impl<'a> /*trait*/ QFileInfo_isHidden for () {
  fn isHidden(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isHiddenEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn completeSuffix<T: QFileInfo_completeSuffix>(&mut self, value: T) -> QString {
    return value.completeSuffix(self);
    // return 1;
  }
}

pub trait QFileInfo_completeSuffix {
  fn completeSuffix(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::completeSuffix();
impl<'a> /*trait*/ QFileInfo_completeSuffix for () {
  fn completeSuffix(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo14completeSuffixEv()};
    let mut ret = unsafe {_ZNK9QFileInfo14completeSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn canonicalPath<T: QFileInfo_canonicalPath>(&mut self, value: T) -> QString {
    return value.canonicalPath(self);
    // return 1;
  }
}

pub trait QFileInfo_canonicalPath {
  fn canonicalPath(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::canonicalPath();
impl<'a> /*trait*/ QFileInfo_canonicalPath for () {
  fn canonicalPath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13canonicalPathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo13canonicalPathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn exists<T: QFileInfo_exists>(&mut self, value: T) -> i8 {
    return value.exists(self);
    // return 1;
  }
}

pub trait QFileInfo_exists {
  fn exists(self, rsthis: &mut QFileInfo) -> i8;
}

// proto: static bool QFileInfo::exists(const QString & file);
impl<'a> /*trait*/ QFileInfo_exists for (&'a  QString) {
  fn exists(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QFileInfo6existsERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn makeAbsolute<T: QFileInfo_makeAbsolute>(&mut self, value: T) -> i8 {
    return value.makeAbsolute(self);
    // return 1;
  }
}

pub trait QFileInfo_makeAbsolute {
  fn makeAbsolute(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::makeAbsolute();
impl<'a> /*trait*/ QFileInfo_makeAbsolute for () {
  fn makeAbsolute(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo12makeAbsoluteEv()};
    let mut ret = unsafe {_ZN9QFileInfo12makeAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isRoot<T: QFileInfo_isRoot>(&mut self, value: T) -> i8 {
    return value.isRoot(self);
    // return 1;
  }
}

pub trait QFileInfo_isRoot {
  fn isRoot(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isRoot();
impl<'a> /*trait*/ QFileInfo_isRoot for () {
  fn isRoot(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isRootEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn canonicalFilePath<T: QFileInfo_canonicalFilePath>(&mut self, value: T) -> QString {
    return value.canonicalFilePath(self);
    // return 1;
  }
}

pub trait QFileInfo_canonicalFilePath {
  fn canonicalFilePath(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::canonicalFilePath();
impl<'a> /*trait*/ QFileInfo_canonicalFilePath for () {
  fn canonicalFilePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo17canonicalFilePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo17canonicalFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isDir<T: QFileInfo_isDir>(&mut self, value: T) -> i8 {
    return value.isDir(self);
    // return 1;
  }
}

pub trait QFileInfo_isDir {
  fn isDir(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isDir();
impl<'a> /*trait*/ QFileInfo_isDir for () {
  fn isDir(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5isDirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5isDirEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QString & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QString) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERK7QString(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn symLinkTarget<T: QFileInfo_symLinkTarget>(&mut self, value: T) -> QString {
    return value.symLinkTarget(self);
    // return 1;
  }
}

pub trait QFileInfo_symLinkTarget {
  fn symLinkTarget(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::symLinkTarget();
impl<'a> /*trait*/ QFileInfo_symLinkTarget for () {
  fn symLinkTarget(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13symLinkTargetEv()};
    let mut ret = unsafe {_ZNK9QFileInfo13symLinkTargetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn setFile<T: QFileInfo_setFile>(&mut self, value: T)  {
     value.setFile(self);
    // return 1;
  }
}

pub trait QFileInfo_setFile {
  fn setFile(self, rsthis: &mut QFileInfo) ;
}

// proto:  void QFileInfo::setFile(const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile for (&'a  QString) {
  fn setFile(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isFile<T: QFileInfo_isFile>(&mut self, value: T) -> i8 {
    return value.isFile(self);
    // return 1;
  }
}

pub trait QFileInfo_isFile {
  fn isFile(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isFile();
impl<'a> /*trait*/ QFileInfo_isFile for () {
  fn isFile(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isFileEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6isFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QFile & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QFile) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERK5QFile(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QFileInfo::setFile(const QFile & file);
impl<'a> /*trait*/ QFileInfo_setFile for (&'a  QFile) {
  fn setFile(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK5QFile(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn ownerId<T: QFileInfo_ownerId>(&mut self, value: T) -> u32 {
    return value.ownerId(self);
    // return 1;
  }
}

pub trait QFileInfo_ownerId {
  fn ownerId(self, rsthis: &mut QFileInfo) -> u32;
}

// proto:  unsigned int QFileInfo::ownerId();
impl<'a> /*trait*/ QFileInfo_ownerId for () {
  fn ownerId(self, rsthis: &mut QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7ownerIdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7ownerIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn readLink<T: QFileInfo_readLink>(&mut self, value: T) -> QString {
    return value.readLink(self);
    // return 1;
  }
}

pub trait QFileInfo_readLink {
  fn readLink(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::readLink();
impl<'a> /*trait*/ QFileInfo_readLink for () {
  fn readLink(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8readLinkEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8readLinkEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn filePath<T: QFileInfo_filePath>(&mut self, value: T) -> QString {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QFileInfo_filePath {
  fn filePath(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::filePath();
impl<'a> /*trait*/ QFileInfo_filePath for () {
  fn filePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8filePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QFileInfo & fileinfo);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QFileInfo) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERKS_(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isSymLink<T: QFileInfo_isSymLink>(&mut self, value: T) -> i8 {
    return value.isSymLink(self);
    // return 1;
  }
}

pub trait QFileInfo_isSymLink {
  fn isSymLink(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isSymLink();
impl<'a> /*trait*/ QFileInfo_isSymLink for () {
  fn isSymLink(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo9isSymLinkEv()};
    let mut ret = unsafe {_ZNK9QFileInfo9isSymLinkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn lastRead<T: QFileInfo_lastRead>(&mut self, value: T) -> QDateTime {
    return value.lastRead(self);
    // return 1;
  }
}

pub trait QFileInfo_lastRead {
  fn lastRead(self, rsthis: &mut QFileInfo) -> QDateTime;
}

// proto:  QDateTime QFileInfo::lastRead();
impl<'a> /*trait*/ QFileInfo_lastRead for () {
  fn lastRead(self, rsthis: &mut QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8lastReadEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8lastReadEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn refresh<T: QFileInfo_refresh>(&mut self, value: T)  {
     value.refresh(self);
    // return 1;
  }
}

pub trait QFileInfo_refresh {
  fn refresh(self, rsthis: &mut QFileInfo) ;
}

// proto:  void QFileInfo::refresh();
impl<'a> /*trait*/ QFileInfo_refresh for () {
  fn refresh(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7refreshEv()};
     unsafe {_ZN9QFileInfo7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QDir, &'a  QString) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERK4QDirRK7QString(qthis, arg0, arg1)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn path<T: QFileInfo_path>(&mut self, value: T) -> QString {
    return value.path(self);
    // return 1;
  }
}

pub trait QFileInfo_path {
  fn path(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::path();
impl<'a> /*trait*/ QFileInfo_path for () {
  fn path(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4pathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absoluteDir<T: QFileInfo_absoluteDir>(&mut self, value: T) -> QDir {
    return value.absoluteDir(self);
    // return 1;
  }
}

pub trait QFileInfo_absoluteDir {
  fn absoluteDir(self, rsthis: &mut QFileInfo) -> QDir;
}

// proto:  QDir QFileInfo::absoluteDir();
impl<'a> /*trait*/ QFileInfo_absoluteDir for () {
  fn absoluteDir(self, rsthis: &mut QFileInfo) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo11absoluteDirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo11absoluteDirEv(rsthis.qclsinst)};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isBundle<T: QFileInfo_isBundle>(&mut self, value: T) -> i8 {
    return value.isBundle(self);
    // return 1;
  }
}

pub trait QFileInfo_isBundle {
  fn isBundle(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isBundle();
impl<'a> /*trait*/ QFileInfo_isBundle for () {
  fn isBundle(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isBundleEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8isBundleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QFileInfo::setFile(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile for (&'a  QDir, &'a  QString) {
  fn setFile(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK4QDirRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isRelative<T: QFileInfo_isRelative>(&mut self, value: T) -> i8 {
    return value.isRelative(self);
    // return 1;
  }
}

pub trait QFileInfo_isRelative {
  fn isRelative(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isRelative();
impl<'a> /*trait*/ QFileInfo_isRelative for () {
  fn isRelative(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isRelativeEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn setCaching<T: QFileInfo_setCaching>(&mut self, value: T)  {
     value.setCaching(self);
    // return 1;
  }
}

pub trait QFileInfo_setCaching {
  fn setCaching(self, rsthis: &mut QFileInfo) ;
}

// proto:  void QFileInfo::setCaching(bool on);
impl<'a> /*trait*/ QFileInfo_setCaching for (i8) {
  fn setCaching(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo10setCachingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QFileInfo10setCachingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn created<T: QFileInfo_created>(&mut self, value: T) -> QDateTime {
    return value.created(self);
    // return 1;
  }
}

pub trait QFileInfo_created {
  fn created(self, rsthis: &mut QFileInfo) -> QDateTime;
}

// proto:  QDateTime QFileInfo::created();
impl<'a> /*trait*/ QFileInfo_created for () {
  fn created(self, rsthis: &mut QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7createdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7createdEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn caching<T: QFileInfo_caching>(&mut self, value: T) -> i8 {
    return value.caching(self);
    // return 1;
  }
}

pub trait QFileInfo_caching {
  fn caching(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::caching();
impl<'a> /*trait*/ QFileInfo_caching for () {
  fn caching(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7cachingEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7cachingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn FreeQFileInfo<T: QFileInfo_FreeQFileInfo>(&mut self, value: T)  {
     value.FreeQFileInfo(self);
    // return 1;
  }
}

pub trait QFileInfo_FreeQFileInfo {
  fn FreeQFileInfo(self, rsthis: &mut QFileInfo) ;
}

// proto:  void QFileInfo::FreeQFileInfo();
impl<'a> /*trait*/ QFileInfo_FreeQFileInfo for () {
  fn FreeQFileInfo(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoD0Ev()};
     unsafe {_ZN9QFileInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn completeBaseName<T: QFileInfo_completeBaseName>(&mut self, value: T) -> QString {
    return value.completeBaseName(self);
    // return 1;
  }
}

pub trait QFileInfo_completeBaseName {
  fn completeBaseName(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::completeBaseName();
impl<'a> /*trait*/ QFileInfo_completeBaseName for () {
  fn completeBaseName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16completeBaseNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo16completeBaseNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn baseName<T: QFileInfo_baseName>(&mut self, value: T) -> QString {
    return value.baseName(self);
    // return 1;
  }
}

pub trait QFileInfo_baseName {
  fn baseName(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::baseName();
impl<'a> /*trait*/ QFileInfo_baseName for () {
  fn baseName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8baseNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8baseNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isExecutable<T: QFileInfo_isExecutable>(&mut self, value: T) -> i8 {
    return value.isExecutable(self);
    // return 1;
  }
}

pub trait QFileInfo_isExecutable {
  fn isExecutable(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isExecutable();
impl<'a> /*trait*/ QFileInfo_isExecutable for () {
  fn isExecutable(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isExecutableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12isExecutableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn bundleName<T: QFileInfo_bundleName>(&mut self, value: T) -> QString {
    return value.bundleName(self);
    // return 1;
  }
}

pub trait QFileInfo_bundleName {
  fn bundleName(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::bundleName();
impl<'a> /*trait*/ QFileInfo_bundleName for () {
  fn bundleName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10bundleNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10bundleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn groupId<T: QFileInfo_groupId>(&mut self, value: T) -> u32 {
    return value.groupId(self);
    // return 1;
  }
}

pub trait QFileInfo_groupId {
  fn groupId(self, rsthis: &mut QFileInfo) -> u32;
}

// proto:  unsigned int QFileInfo::groupId();
impl<'a> /*trait*/ QFileInfo_groupId for () {
  fn groupId(self, rsthis: &mut QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7groupIdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7groupIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn fileName<T: QFileInfo_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QFileInfo_fileName {
  fn fileName(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::fileName();
impl<'a> /*trait*/ QFileInfo_fileName for () {
  fn fileName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8fileNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn size<T: QFileInfo_size>(&mut self, value: T) -> i64 {
    return value.size(self);
    // return 1;
  }
}

pub trait QFileInfo_size {
  fn size(self, rsthis: &mut QFileInfo) -> i64;
}

// proto:  long long QFileInfo::size();
impl<'a> /*trait*/ QFileInfo_size for () {
  fn size(self, rsthis: &mut QFileInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4sizeEv()};
    let mut ret = unsafe {_ZNK9QFileInfo4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absoluteFilePath<T: QFileInfo_absoluteFilePath>(&mut self, value: T) -> QString {
    return value.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QFileInfo_absoluteFilePath {
  fn absoluteFilePath(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::absoluteFilePath();
impl<'a> /*trait*/ QFileInfo_absoluteFilePath for () {
  fn absoluteFilePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16absoluteFilePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo16absoluteFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn suffix<T: QFileInfo_suffix>(&mut self, value: T) -> QString {
    return value.suffix(self);
    // return 1;
  }
}

pub trait QFileInfo_suffix {
  fn suffix(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::suffix();
impl<'a> /*trait*/ QFileInfo_suffix for () {
  fn suffix(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6suffixEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn group<T: QFileInfo_group>(&mut self, value: T) -> QString {
    return value.group(self);
    // return 1;
  }
}

pub trait QFileInfo_group {
  fn group(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::group();
impl<'a> /*trait*/ QFileInfo_group for () {
  fn group(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5groupEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5groupEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isAbsolute<T: QFileInfo_isAbsolute>(&mut self, value: T) -> i8 {
    return value.isAbsolute(self);
    // return 1;
  }
}

pub trait QFileInfo_isAbsolute {
  fn isAbsolute(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isAbsolute();
impl<'a> /*trait*/ QFileInfo_isAbsolute for () {
  fn isAbsolute(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isAbsoluteEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isNativePath<T: QFileInfo_isNativePath>(&mut self, value: T) -> i8 {
    return value.isNativePath(self);
    // return 1;
  }
}

pub trait QFileInfo_isNativePath {
  fn isNativePath(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isNativePath();
impl<'a> /*trait*/ QFileInfo_isNativePath for () {
  fn isNativePath(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isNativePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12isNativePathEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isWritable<T: QFileInfo_isWritable>(&mut self, value: T) -> i8 {
    return value.isWritable(self);
    // return 1;
  }
}

pub trait QFileInfo_isWritable {
  fn isWritable(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isWritable();
impl<'a> /*trait*/ QFileInfo_isWritable for () {
  fn isWritable(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isWritableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn owner<T: QFileInfo_owner>(&mut self, value: T) -> QString {
    return value.owner(self);
    // return 1;
  }
}

pub trait QFileInfo_owner {
  fn owner(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::owner();
impl<'a> /*trait*/ QFileInfo_owner for () {
  fn owner(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5ownerEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5ownerEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isReadable<T: QFileInfo_isReadable>(&mut self, value: T) -> i8 {
    return value.isReadable(self);
    // return 1;
  }
}

pub trait QFileInfo_isReadable {
  fn isReadable(self, rsthis: &mut QFileInfo) -> i8;
}

// proto:  bool QFileInfo::isReadable();
impl<'a> /*trait*/ QFileInfo_isReadable for () {
  fn isReadable(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isReadableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn dir<T: QFileInfo_dir>(&mut self, value: T) -> QDir {
    return value.dir(self);
    // return 1;
  }
}

pub trait QFileInfo_dir {
  fn dir(self, rsthis: &mut QFileInfo) -> QDir;
}

// proto:  QDir QFileInfo::dir();
impl<'a> /*trait*/ QFileInfo_dir for () {
  fn dir(self, rsthis: &mut QFileInfo) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo3dirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo3dirEv(rsthis.qclsinst)};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn swap<T: QFileInfo_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QFileInfo_swap {
  fn swap(self, rsthis: &mut QFileInfo) ;
}

// proto:  void QFileInfo::swap(QFileInfo & other);
impl<'a> /*trait*/ QFileInfo_swap for (&'a mut QFileInfo) {
  fn swap(self, rsthis: &mut QFileInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QFileInfo::exists();
impl<'a> /*trait*/ QFileInfo_exists for () {
  fn exists(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6existsEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn lastModified<T: QFileInfo_lastModified>(&mut self, value: T) -> QDateTime {
    return value.lastModified(self);
    // return 1;
  }
}

pub trait QFileInfo_lastModified {
  fn lastModified(self, rsthis: &mut QFileInfo) -> QDateTime;
}

// proto:  QDateTime QFileInfo::lastModified();
impl<'a> /*trait*/ QFileInfo_lastModified for () {
  fn lastModified(self, rsthis: &mut QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12lastModifiedEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12lastModifiedEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absolutePath<T: QFileInfo_absolutePath>(&mut self, value: T) -> QString {
    return value.absolutePath(self);
    // return 1;
  }
}

pub trait QFileInfo_absolutePath {
  fn absolutePath(self, rsthis: &mut QFileInfo) -> QString;
}

// proto:  QString QFileInfo::absolutePath();
impl<'a> /*trait*/ QFileInfo_absolutePath for () {
  fn absolutePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12absolutePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12absolutePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

