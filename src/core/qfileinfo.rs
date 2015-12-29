// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qfileinfo.h
// dst-file: /src/core/qfileinfo.rs
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
use super::qstring::QString; // 773
use super::qfile::QFile; // 773
use super::qdatetime::QDateTime; // 773
use super::qdir::QDir; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFileInfo_Class_Size() -> c_int;
  // proto:  bool QFileInfo::isHidden();
  fn _ZNK9QFileInfo8isHiddenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFileInfo::QFileInfo();
  fn dector_ZN9QFileInfoC1Ev() -> *mut c_void;
  fn _ZN9QFileInfoC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QFileInfo::completeSuffix();
  fn _ZNK9QFileInfo14completeSuffixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QFileInfo::canonicalPath();
  fn _ZNK9QFileInfo13canonicalPathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QFileInfo::exists(const QString & file);
  fn _ZN9QFileInfo6existsERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  bool QFileInfo::makeAbsolute();
  fn _ZN9QFileInfo12makeAbsoluteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFileInfo::isRoot();
  fn _ZNK9QFileInfo6isRootEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QFileInfo::canonicalFilePath();
  fn _ZNK9QFileInfo17canonicalFilePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFileInfo::isDir();
  fn _ZNK9QFileInfo5isDirEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFileInfo::QFileInfo(const QString & file);
  fn dector_ZN9QFileInfoC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QFileInfoC1ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QFileInfo::symLinkTarget();
  fn demth_ZNK9QFileInfo13symLinkTargetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFileInfo::setFile(const QString & file);
  fn _ZN9QFileInfo7setFileERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QFileInfo::isFile();
  fn _ZNK9QFileInfo6isFileEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFileInfo::QFileInfo(const QFile & file);
  fn dector_ZN9QFileInfoC1ERK5QFile(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QFileInfoC1ERK5QFile(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFileInfo::setFile(const QFile & file);
  fn _ZN9QFileInfo7setFileERK5QFile(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  uint QFileInfo::ownerId();
  fn _ZNK9QFileInfo7ownerIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  QString QFileInfo::readLink();
  fn _ZNK9QFileInfo8readLinkEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QFileInfo::filePath();
  fn _ZNK9QFileInfo8filePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFileInfo::QFileInfo(const QFileInfo & fileinfo);
  fn dector_ZN9QFileInfoC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QFileInfoC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QFileInfo::isSymLink();
  fn _ZNK9QFileInfo9isSymLinkEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QDateTime QFileInfo::lastRead();
  fn _ZNK9QFileInfo8lastReadEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFileInfo::refresh();
  fn _ZN9QFileInfo7refreshEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFileInfo::QFileInfo(const QDir & dir, const QString & file);
  fn dector_ZN9QFileInfoC1ERK4QDirRK7QString(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QFileInfoC1ERK4QDirRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QFileInfo::path();
  fn _ZNK9QFileInfo4pathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDir QFileInfo::absoluteDir();
  fn _ZNK9QFileInfo11absoluteDirEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFileInfo::isBundle();
  fn _ZNK9QFileInfo8isBundleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFileInfo::setFile(const QDir & dir, const QString & file);
  fn _ZN9QFileInfo7setFileERK4QDirRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QFileInfo::isRelative();
  fn _ZNK9QFileInfo10isRelativeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFileInfo::setCaching(bool on);
  fn _ZN9QFileInfo10setCachingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QDateTime QFileInfo::created();
  fn _ZNK9QFileInfo7createdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFileInfo::caching();
  fn _ZNK9QFileInfo7cachingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFileInfo::~QFileInfo();
  fn _ZN9QFileInfoD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QFileInfo::completeBaseName();
  fn _ZNK9QFileInfo16completeBaseNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QFileInfo::baseName();
  fn _ZNK9QFileInfo8baseNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFileInfo::isExecutable();
  fn _ZNK9QFileInfo12isExecutableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QFileInfo::bundleName();
  fn _ZNK9QFileInfo10bundleNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  uint QFileInfo::groupId();
  fn _ZNK9QFileInfo7groupIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  QString QFileInfo::fileName();
  fn _ZNK9QFileInfo8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QFileInfo::size();
  fn _ZNK9QFileInfo4sizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QString QFileInfo::absoluteFilePath();
  fn _ZNK9QFileInfo16absoluteFilePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QFileInfo::suffix();
  fn _ZNK9QFileInfo6suffixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QFileInfo::group();
  fn _ZNK9QFileInfo5groupEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFileInfo::isAbsolute();
  fn demth_ZNK9QFileInfo10isAbsoluteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFileInfo::isNativePath();
  fn _ZNK9QFileInfo12isNativePathEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFileInfo::isWritable();
  fn _ZNK9QFileInfo10isWritableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QFileInfo::owner();
  fn _ZNK9QFileInfo5ownerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFileInfo::isReadable();
  fn _ZNK9QFileInfo10isReadableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QDir QFileInfo::dir();
  fn _ZNK9QFileInfo3dirEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFileInfo::swap(QFileInfo & other);
  fn demth_ZN9QFileInfo4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QFileInfo::exists();
  fn _ZNK9QFileInfo6existsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QDateTime QFileInfo::lastModified();
  fn _ZNK9QFileInfo12lastModifiedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QFileInfo::absolutePath();
  fn _ZNK9QFileInfo12absolutePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QFileInfo)=1
#[derive(Default)]
pub struct QFileInfo {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFileInfo {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFileInfo {
    return QFileInfo{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QFileInfo::isHidden();
impl /*struct*/ QFileInfo {
  pub fn isHidden<RetType, T: QFileInfo_isHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHidden(self);
    // return 1;
  }
}

pub trait QFileInfo_isHidden<RetType> {
  fn isHidden(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isHidden();
impl<'a> /*trait*/ QFileInfo_isHidden<i8> for () {
  fn isHidden(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isHiddenEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileInfo::QFileInfo();
impl /*struct*/ QFileInfo {
  pub fn New<T: QFileInfo_New>(value: T) -> QFileInfo {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_New {
  fn New(self) -> QFileInfo;
}

  // proto:  void QFileInfo::QFileInfo();
impl<'a> /*trait*/ QFileInfo_New for () {
  fn New(self) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1Ev()};
    let ctysz: c_int = unsafe{QFileInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN9QFileInfoC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN9QFileInfoC1Ev()} as u64;
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFileInfo::completeSuffix();
impl /*struct*/ QFileInfo {
  pub fn completeSuffix<RetType, T: QFileInfo_completeSuffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completeSuffix(self);
    // return 1;
  }
}

pub trait QFileInfo_completeSuffix<RetType> {
  fn completeSuffix(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::completeSuffix();
impl<'a> /*trait*/ QFileInfo_completeSuffix<QString> for () {
  fn completeSuffix(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo14completeSuffixEv()};
    let mut ret = unsafe {_ZNK9QFileInfo14completeSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileInfo::canonicalPath();
impl /*struct*/ QFileInfo {
  pub fn canonicalPath<RetType, T: QFileInfo_canonicalPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canonicalPath(self);
    // return 1;
  }
}

pub trait QFileInfo_canonicalPath<RetType> {
  fn canonicalPath(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::canonicalPath();
impl<'a> /*trait*/ QFileInfo_canonicalPath<QString> for () {
  fn canonicalPath(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13canonicalPathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo13canonicalPathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QFileInfo::exists(const QString & file);
impl /*struct*/ QFileInfo {
  pub fn exists_s<RetType, T: QFileInfo_exists_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exists_s();
    // return 1;
  }
}

pub trait QFileInfo_exists_s<RetType> {
  fn exists_s(self ) -> RetType;
}

  // proto: static bool QFileInfo::exists(const QString & file);
impl<'a> /*trait*/ QFileInfo_exists_s<i8> for (&'a QString) {
  fn exists_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QFileInfo6existsERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileInfo::makeAbsolute();
impl /*struct*/ QFileInfo {
  pub fn makeAbsolute<RetType, T: QFileInfo_makeAbsolute<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeAbsolute(self);
    // return 1;
  }
}

pub trait QFileInfo_makeAbsolute<RetType> {
  fn makeAbsolute(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::makeAbsolute();
impl<'a> /*trait*/ QFileInfo_makeAbsolute<i8> for () {
  fn makeAbsolute(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo12makeAbsoluteEv()};
    let mut ret = unsafe {_ZN9QFileInfo12makeAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isRoot();
impl /*struct*/ QFileInfo {
  pub fn isRoot<RetType, T: QFileInfo_isRoot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRoot(self);
    // return 1;
  }
}

pub trait QFileInfo_isRoot<RetType> {
  fn isRoot(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isRoot();
impl<'a> /*trait*/ QFileInfo_isRoot<i8> for () {
  fn isRoot(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isRootEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFileInfo::canonicalFilePath();
impl /*struct*/ QFileInfo {
  pub fn canonicalFilePath<RetType, T: QFileInfo_canonicalFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canonicalFilePath(self);
    // return 1;
  }
}

pub trait QFileInfo_canonicalFilePath<RetType> {
  fn canonicalFilePath(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::canonicalFilePath();
impl<'a> /*trait*/ QFileInfo_canonicalFilePath<QString> for () {
  fn canonicalFilePath(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo17canonicalFilePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo17canonicalFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isDir();
impl /*struct*/ QFileInfo {
  pub fn isDir<RetType, T: QFileInfo_isDir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDir(self);
    // return 1;
  }
}

pub trait QFileInfo_isDir<RetType> {
  fn isDir(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isDir();
impl<'a> /*trait*/ QFileInfo_isDir<i8> for () {
  fn isDir(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5isDirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5isDirEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileInfo::QFileInfo(const QString & file);
impl<'a> /*trait*/ QFileInfo_New for (&'a QString) {
  fn New(self) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK7QString()};
    let ctysz: c_int = unsafe{QFileInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QFileInfoC1ERK7QString(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QFileInfoC1ERK7QString(arg0)} as u64;
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFileInfo::symLinkTarget();
impl /*struct*/ QFileInfo {
  pub fn symLinkTarget<RetType, T: QFileInfo_symLinkTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.symLinkTarget(self);
    // return 1;
  }
}

pub trait QFileInfo_symLinkTarget<RetType> {
  fn symLinkTarget(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::symLinkTarget();
impl<'a> /*trait*/ QFileInfo_symLinkTarget<QString> for () {
  fn symLinkTarget(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13symLinkTargetEv()};
    let mut ret = unsafe {demth_ZNK9QFileInfo13symLinkTargetEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileInfo::setFile(const QString & file);
impl /*struct*/ QFileInfo {
  pub fn setFile<RetType, T: QFileInfo_setFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFile(self);
    // return 1;
  }
}

pub trait QFileInfo_setFile<RetType> {
  fn setFile(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  void QFileInfo::setFile(const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile<()> for (&'a QString) {
  fn setFile(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFileInfo::isFile();
impl /*struct*/ QFileInfo {
  pub fn isFile<RetType, T: QFileInfo_isFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFile(self);
    // return 1;
  }
}

pub trait QFileInfo_isFile<RetType> {
  fn isFile(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isFile();
impl<'a> /*trait*/ QFileInfo_isFile<i8> for () {
  fn isFile(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isFileEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6isFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileInfo::QFileInfo(const QFile & file);
impl<'a> /*trait*/ QFileInfo_New for (&'a QFile) {
  fn New(self) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK5QFile()};
    let ctysz: c_int = unsafe{QFileInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QFileInfoC1ERK5QFile(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QFileInfoC1ERK5QFile(arg0)} as u64;
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileInfo::setFile(const QFile & file);
impl<'a> /*trait*/ QFileInfo_setFile<()> for (&'a QFile) {
  fn setFile(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK5QFile(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  uint QFileInfo::ownerId();
impl /*struct*/ QFileInfo {
  pub fn ownerId<RetType, T: QFileInfo_ownerId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ownerId(self);
    // return 1;
  }
}

pub trait QFileInfo_ownerId<RetType> {
  fn ownerId(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  uint QFileInfo::ownerId();
impl<'a> /*trait*/ QFileInfo_ownerId<u32> for () {
  fn ownerId(self , rsthis: & QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7ownerIdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7ownerIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QString QFileInfo::readLink();
impl /*struct*/ QFileInfo {
  pub fn readLink<RetType, T: QFileInfo_readLink<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readLink(self);
    // return 1;
  }
}

pub trait QFileInfo_readLink<RetType> {
  fn readLink(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::readLink();
impl<'a> /*trait*/ QFileInfo_readLink<QString> for () {
  fn readLink(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8readLinkEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8readLinkEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileInfo::filePath();
impl /*struct*/ QFileInfo {
  pub fn filePath<RetType, T: QFileInfo_filePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QFileInfo_filePath<RetType> {
  fn filePath(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::filePath();
impl<'a> /*trait*/ QFileInfo_filePath<QString> for () {
  fn filePath(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8filePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileInfo::QFileInfo(const QFileInfo & fileinfo);
impl<'a> /*trait*/ QFileInfo_New for (&'a QFileInfo) {
  fn New(self) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERKS_()};
    let ctysz: c_int = unsafe{QFileInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QFileInfoC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QFileInfoC1ERKS_(arg0)} as u64;
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isSymLink();
impl /*struct*/ QFileInfo {
  pub fn isSymLink<RetType, T: QFileInfo_isSymLink<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSymLink(self);
    // return 1;
  }
}

pub trait QFileInfo_isSymLink<RetType> {
  fn isSymLink(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isSymLink();
impl<'a> /*trait*/ QFileInfo_isSymLink<i8> for () {
  fn isSymLink(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo9isSymLinkEv()};
    let mut ret = unsafe {_ZNK9QFileInfo9isSymLinkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QDateTime QFileInfo::lastRead();
impl /*struct*/ QFileInfo {
  pub fn lastRead<RetType, T: QFileInfo_lastRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastRead(self);
    // return 1;
  }
}

pub trait QFileInfo_lastRead<RetType> {
  fn lastRead(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QDateTime QFileInfo::lastRead();
impl<'a> /*trait*/ QFileInfo_lastRead<QDateTime> for () {
  fn lastRead(self , rsthis: & QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8lastReadEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8lastReadEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileInfo::refresh();
impl /*struct*/ QFileInfo {
  pub fn refresh<RetType, T: QFileInfo_refresh<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refresh(self);
    // return 1;
  }
}

pub trait QFileInfo_refresh<RetType> {
  fn refresh(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  void QFileInfo::refresh();
impl<'a> /*trait*/ QFileInfo_refresh<()> for () {
  fn refresh(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7refreshEv()};
     unsafe {_ZN9QFileInfo7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileInfo::QFileInfo(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_New for (&'a QDir, &'a QString) {
  fn New(self) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK4QDirRK7QString()};
    let ctysz: c_int = unsafe{QFileInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QFileInfoC1ERK4QDirRK7QString(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QFileInfoC1ERK4QDirRK7QString(arg0, arg1)} as u64;
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFileInfo::path();
impl /*struct*/ QFileInfo {
  pub fn path<RetType, T: QFileInfo_path<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QFileInfo_path<RetType> {
  fn path(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::path();
impl<'a> /*trait*/ QFileInfo_path<QString> for () {
  fn path(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4pathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDir QFileInfo::absoluteDir();
impl /*struct*/ QFileInfo {
  pub fn absoluteDir<RetType, T: QFileInfo_absoluteDir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.absoluteDir(self);
    // return 1;
  }
}

pub trait QFileInfo_absoluteDir<RetType> {
  fn absoluteDir(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QDir QFileInfo::absoluteDir();
impl<'a> /*trait*/ QFileInfo_absoluteDir<QDir> for () {
  fn absoluteDir(self , rsthis: & QFileInfo) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo11absoluteDirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo11absoluteDirEv(rsthis.qclsinst)};
    let mut ret1 = QDir::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isBundle();
impl /*struct*/ QFileInfo {
  pub fn isBundle<RetType, T: QFileInfo_isBundle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBundle(self);
    // return 1;
  }
}

pub trait QFileInfo_isBundle<RetType> {
  fn isBundle(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isBundle();
impl<'a> /*trait*/ QFileInfo_isBundle<i8> for () {
  fn isBundle(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isBundleEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8isBundleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileInfo::setFile(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile<()> for (&'a QDir, &'a QString) {
  fn setFile(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK4QDirRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QFileInfo::isRelative();
impl /*struct*/ QFileInfo {
  pub fn isRelative<RetType, T: QFileInfo_isRelative<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRelative(self);
    // return 1;
  }
}

pub trait QFileInfo_isRelative<RetType> {
  fn isRelative(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isRelative();
impl<'a> /*trait*/ QFileInfo_isRelative<i8> for () {
  fn isRelative(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isRelativeEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileInfo::setCaching(bool on);
impl /*struct*/ QFileInfo {
  pub fn setCaching<RetType, T: QFileInfo_setCaching<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCaching(self);
    // return 1;
  }
}

pub trait QFileInfo_setCaching<RetType> {
  fn setCaching(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  void QFileInfo::setCaching(bool on);
impl<'a> /*trait*/ QFileInfo_setCaching<()> for (i8) {
  fn setCaching(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo10setCachingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QFileInfo10setCachingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDateTime QFileInfo::created();
impl /*struct*/ QFileInfo {
  pub fn created<RetType, T: QFileInfo_created<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.created(self);
    // return 1;
  }
}

pub trait QFileInfo_created<RetType> {
  fn created(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QDateTime QFileInfo::created();
impl<'a> /*trait*/ QFileInfo_created<QDateTime> for () {
  fn created(self , rsthis: & QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7createdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7createdEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileInfo::caching();
impl /*struct*/ QFileInfo {
  pub fn caching<RetType, T: QFileInfo_caching<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.caching(self);
    // return 1;
  }
}

pub trait QFileInfo_caching<RetType> {
  fn caching(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::caching();
impl<'a> /*trait*/ QFileInfo_caching<i8> for () {
  fn caching(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7cachingEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7cachingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileInfo::~QFileInfo();
impl /*struct*/ QFileInfo {
  pub fn Free<RetType, T: QFileInfo_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFileInfo_Free<RetType> {
  fn Free(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  void QFileInfo::~QFileInfo();
impl<'a> /*trait*/ QFileInfo_Free<()> for () {
  fn Free(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoD0Ev()};
     unsafe {_ZN9QFileInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QFileInfo::completeBaseName();
impl /*struct*/ QFileInfo {
  pub fn completeBaseName<RetType, T: QFileInfo_completeBaseName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completeBaseName(self);
    // return 1;
  }
}

pub trait QFileInfo_completeBaseName<RetType> {
  fn completeBaseName(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::completeBaseName();
impl<'a> /*trait*/ QFileInfo_completeBaseName<QString> for () {
  fn completeBaseName(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16completeBaseNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo16completeBaseNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileInfo::baseName();
impl /*struct*/ QFileInfo {
  pub fn baseName<RetType, T: QFileInfo_baseName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseName(self);
    // return 1;
  }
}

pub trait QFileInfo_baseName<RetType> {
  fn baseName(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::baseName();
impl<'a> /*trait*/ QFileInfo_baseName<QString> for () {
  fn baseName(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8baseNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8baseNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isExecutable();
impl /*struct*/ QFileInfo {
  pub fn isExecutable<RetType, T: QFileInfo_isExecutable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isExecutable(self);
    // return 1;
  }
}

pub trait QFileInfo_isExecutable<RetType> {
  fn isExecutable(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isExecutable();
impl<'a> /*trait*/ QFileInfo_isExecutable<i8> for () {
  fn isExecutable(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isExecutableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12isExecutableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFileInfo::bundleName();
impl /*struct*/ QFileInfo {
  pub fn bundleName<RetType, T: QFileInfo_bundleName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bundleName(self);
    // return 1;
  }
}

pub trait QFileInfo_bundleName<RetType> {
  fn bundleName(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::bundleName();
impl<'a> /*trait*/ QFileInfo_bundleName<QString> for () {
  fn bundleName(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10bundleNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10bundleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  uint QFileInfo::groupId();
impl /*struct*/ QFileInfo {
  pub fn groupId<RetType, T: QFileInfo_groupId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.groupId(self);
    // return 1;
  }
}

pub trait QFileInfo_groupId<RetType> {
  fn groupId(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  uint QFileInfo::groupId();
impl<'a> /*trait*/ QFileInfo_groupId<u32> for () {
  fn groupId(self , rsthis: & QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7groupIdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7groupIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QString QFileInfo::fileName();
impl /*struct*/ QFileInfo {
  pub fn fileName<RetType, T: QFileInfo_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QFileInfo_fileName<RetType> {
  fn fileName(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::fileName();
impl<'a> /*trait*/ QFileInfo_fileName<QString> for () {
  fn fileName(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8fileNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QFileInfo::size();
impl /*struct*/ QFileInfo {
  pub fn size<RetType, T: QFileInfo_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QFileInfo_size<RetType> {
  fn size(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  qint64 QFileInfo::size();
impl<'a> /*trait*/ QFileInfo_size<i64> for () {
  fn size(self , rsthis: & QFileInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4sizeEv()};
    let mut ret = unsafe {_ZNK9QFileInfo4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QString QFileInfo::absoluteFilePath();
impl /*struct*/ QFileInfo {
  pub fn absoluteFilePath<RetType, T: QFileInfo_absoluteFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QFileInfo_absoluteFilePath<RetType> {
  fn absoluteFilePath(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::absoluteFilePath();
impl<'a> /*trait*/ QFileInfo_absoluteFilePath<QString> for () {
  fn absoluteFilePath(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16absoluteFilePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo16absoluteFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileInfo::suffix();
impl /*struct*/ QFileInfo {
  pub fn suffix<RetType, T: QFileInfo_suffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.suffix(self);
    // return 1;
  }
}

pub trait QFileInfo_suffix<RetType> {
  fn suffix(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::suffix();
impl<'a> /*trait*/ QFileInfo_suffix<QString> for () {
  fn suffix(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6suffixEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileInfo::group();
impl /*struct*/ QFileInfo {
  pub fn group<RetType, T: QFileInfo_group<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QFileInfo_group<RetType> {
  fn group(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::group();
impl<'a> /*trait*/ QFileInfo_group<QString> for () {
  fn group(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5groupEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5groupEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isAbsolute();
impl /*struct*/ QFileInfo {
  pub fn isAbsolute<RetType, T: QFileInfo_isAbsolute<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAbsolute(self);
    // return 1;
  }
}

pub trait QFileInfo_isAbsolute<RetType> {
  fn isAbsolute(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isAbsolute();
impl<'a> /*trait*/ QFileInfo_isAbsolute<i8> for () {
  fn isAbsolute(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isAbsoluteEv()};
    let mut ret = unsafe {demth_ZNK9QFileInfo10isAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isNativePath();
impl /*struct*/ QFileInfo {
  pub fn isNativePath<RetType, T: QFileInfo_isNativePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNativePath(self);
    // return 1;
  }
}

pub trait QFileInfo_isNativePath<RetType> {
  fn isNativePath(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isNativePath();
impl<'a> /*trait*/ QFileInfo_isNativePath<i8> for () {
  fn isNativePath(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isNativePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12isNativePathEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isWritable();
impl /*struct*/ QFileInfo {
  pub fn isWritable<RetType, T: QFileInfo_isWritable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWritable(self);
    // return 1;
  }
}

pub trait QFileInfo_isWritable<RetType> {
  fn isWritable(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isWritable();
impl<'a> /*trait*/ QFileInfo_isWritable<i8> for () {
  fn isWritable(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isWritableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFileInfo::owner();
impl /*struct*/ QFileInfo {
  pub fn owner<RetType, T: QFileInfo_owner<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.owner(self);
    // return 1;
  }
}

pub trait QFileInfo_owner<RetType> {
  fn owner(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::owner();
impl<'a> /*trait*/ QFileInfo_owner<QString> for () {
  fn owner(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5ownerEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5ownerEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileInfo::isReadable();
impl /*struct*/ QFileInfo {
  pub fn isReadable<RetType, T: QFileInfo_isReadable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadable(self);
    // return 1;
  }
}

pub trait QFileInfo_isReadable<RetType> {
  fn isReadable(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::isReadable();
impl<'a> /*trait*/ QFileInfo_isReadable<i8> for () {
  fn isReadable(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isReadableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QDir QFileInfo::dir();
impl /*struct*/ QFileInfo {
  pub fn dir<RetType, T: QFileInfo_dir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dir(self);
    // return 1;
  }
}

pub trait QFileInfo_dir<RetType> {
  fn dir(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QDir QFileInfo::dir();
impl<'a> /*trait*/ QFileInfo_dir<QDir> for () {
  fn dir(self , rsthis: & QFileInfo) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo3dirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo3dirEv(rsthis.qclsinst)};
    let mut ret1 = QDir::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileInfo::swap(QFileInfo & other);
impl /*struct*/ QFileInfo {
  pub fn swap<RetType, T: QFileInfo_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QFileInfo_swap<RetType> {
  fn swap(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  void QFileInfo::swap(QFileInfo & other);
impl<'a> /*trait*/ QFileInfo_swap<()> for (&'a QFileInfo) {
  fn swap(self , rsthis: & QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QFileInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFileInfo::exists();
impl /*struct*/ QFileInfo {
  pub fn exists<RetType, T: QFileInfo_exists<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exists(self);
    // return 1;
  }
}

pub trait QFileInfo_exists<RetType> {
  fn exists(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  bool QFileInfo::exists();
impl<'a> /*trait*/ QFileInfo_exists<i8> for () {
  fn exists(self , rsthis: & QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6existsEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QDateTime QFileInfo::lastModified();
impl /*struct*/ QFileInfo {
  pub fn lastModified<RetType, T: QFileInfo_lastModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastModified(self);
    // return 1;
  }
}

pub trait QFileInfo_lastModified<RetType> {
  fn lastModified(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QDateTime QFileInfo::lastModified();
impl<'a> /*trait*/ QFileInfo_lastModified<QDateTime> for () {
  fn lastModified(self , rsthis: & QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12lastModifiedEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12lastModifiedEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileInfo::absolutePath();
impl /*struct*/ QFileInfo {
  pub fn absolutePath<RetType, T: QFileInfo_absolutePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.absolutePath(self);
    // return 1;
  }
}

pub trait QFileInfo_absolutePath<RetType> {
  fn absolutePath(self , rsthis: & QFileInfo) -> RetType;
}

  // proto:  QString QFileInfo::absolutePath();
impl<'a> /*trait*/ QFileInfo_absolutePath<QString> for () {
  fn absolutePath(self , rsthis: & QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12absolutePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12absolutePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

