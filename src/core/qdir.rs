// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qdir.h
// dst-file: /src/core/qdir.rs
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
use super::qstringlist::QStringList; // 773
use super::qchar::QChar; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDir_Class_Size() -> c_int;
  // proto: static void QDir::addResourceSearchPath(const QString & path);
  fn _ZN4QDir21addResourceSearchPathERK7QString(arg0: *mut c_void);
  // proto:  QString QDir::relativeFilePath(const QString & fileName);
  fn _ZNK4QDir16relativeFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QDir::tempPath();
  fn _ZN4QDir8tempPathEv() -> *mut c_void;
  // proto:  void QDir::QDir(const QString & path);
  fn dector_ZN4QDirC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN4QDirC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QDir::nameFilters();
  fn _ZNK4QDir11nameFiltersEv(qthis: *mut c_void);
  // proto:  bool QDir::cd(const QString & dirName);
  fn _ZN4QDir2cdERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto: static QFileInfoList QDir::drives();
  fn _ZN4QDir6drivesEv();
  // proto: static bool QDir::setCurrent(const QString & path);
  fn _ZN4QDir10setCurrentERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  bool QDir::rmdir(const QString & dirName);
  fn _ZNK4QDir5rmdirERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QDir::cdUp();
  fn _ZN4QDir4cdUpEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QDir::absolutePath();
  fn _ZNK4QDir12absolutePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
  fn _ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QDir::absoluteFilePath(const QString & fileName);
  fn _ZNK4QDir16absoluteFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::rename(const QString & oldName, const QString & newName);
  fn _ZN4QDir6renameERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static bool QDir::match(const QString & filter, const QString & fileName);
  fn _ZN4QDir5matchERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QDir::refresh();
  fn _ZNK4QDir7refreshEv(qthis: *mut c_void);
  // proto:  bool QDir::mkdir(const QString & dirName);
  fn _ZNK4QDir5mkdirERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  uint QDir::count();
  fn _ZNK4QDir5countEv(qthis: *mut c_void) -> c_uint;
  // proto: static QStringList QDir::nameFiltersFromString(const QString & nameFilter);
  fn _ZN4QDir21nameFiltersFromStringERK7QString(arg0: *mut c_void);
  // proto:  QString QDir::filePath(const QString & fileName);
  fn _ZNK4QDir8filePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::rmpath(const QString & dirPath);
  fn _ZNK4QDir6rmpathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QString QDir::path();
  fn _ZNK4QDir4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QDir::toNativeSeparators(const QString & pathName);
  fn _ZN4QDir18toNativeSeparatorsERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QDir::cleanPath(const QString & path);
  fn _ZN4QDir9cleanPathERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::exists();
  fn _ZNK4QDir6existsEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QDir::remove(const QString & fileName);
  fn _ZN4QDir6removeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QDir::~QDir();
  fn _ZN4QDirD0Ev(qthis: *mut c_void);
  // proto: static QString QDir::rootPath();
  fn _ZN4QDir8rootPathEv() -> *mut c_void;
  // proto: static bool QDir::match(const QStringList & filters, const QString & fileName);
  fn _ZN4QDir5matchERK11QStringListRK7QString(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static QString QDir::fromNativeSeparators(const QString & pathName);
  fn _ZN4QDir20fromNativeSeparatorsERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDir::setNameFilters(const QStringList & nameFilters);
  fn _ZN4QDir14setNameFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QChar QDir::separator();
  fn _ZN4QDir9separatorEv() -> *mut c_void;
  // proto:  bool QDir::exists(const QString & name);
  fn _ZNK4QDir6existsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QDir::mkpath(const QString & dirPath);
  fn _ZNK4QDir6mkpathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto: static void QDir::addSearchPath(const QString & prefix, const QString & path);
  fn _ZN4QDir13addSearchPathERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QDir::dirName();
  fn _ZNK4QDir7dirNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QStringList QDir::searchPaths(const QString & prefix);
  fn _ZN4QDir11searchPathsERK7QString(arg0: *mut c_void);
  // proto:  bool QDir::makeAbsolute();
  fn _ZN4QDir12makeAbsoluteEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QDir::canonicalPath();
  fn _ZNK4QDir13canonicalPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::isReadable();
  fn _ZNK4QDir10isReadableEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QDir::isRelative();
  fn _ZNK4QDir10isRelativeEv(qthis: *mut c_void) -> c_char;
  // proto: static QString QDir::currentPath();
  fn _ZN4QDir11currentPathEv() -> *mut c_void;
  // proto:  bool QDir::isRoot();
  fn _ZNK4QDir6isRootEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QDir::removeRecursively();
  fn _ZN4QDir17removeRecursivelyEv(qthis: *mut c_void) -> c_char;
  // proto: static QString QDir::homePath();
  fn _ZN4QDir8homePathEv() -> *mut c_void;
  // proto:  void QDir::QDir(const QDir & );
  fn dector_ZN4QDirC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN4QDirC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDir::setPath(const QString & path);
  fn _ZN4QDir7setPathERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static bool QDir::isRelativePath(const QString & path);
  fn _ZN4QDir14isRelativePathERK7QString(arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QDir)=1
pub struct QDir {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDir {
  pub fn inheritFrom(qthis: *mut c_void) -> QDir {
    return QDir{qclsinst: qthis};
  }
}
  // proto: static void QDir::addResourceSearchPath(const QString & path);
impl /*struct*/ QDir {
  pub fn addResourceSearchPath_s<RetType, T: QDir_addResourceSearchPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addResourceSearchPath_s();
    // return 1;
  }
}

pub trait QDir_addResourceSearchPath_s<RetType> {
  fn addResourceSearchPath_s(self ) -> RetType;
}

  // proto: static void QDir::addResourceSearchPath(const QString & path);
impl<'a> /*trait*/ QDir_addResourceSearchPath_s<()> for (&'a QString) {
  fn addResourceSearchPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21addResourceSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir21addResourceSearchPathERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  QString QDir::relativeFilePath(const QString & fileName);
impl /*struct*/ QDir {
  pub fn relativeFilePath<RetType, T: QDir_relativeFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.relativeFilePath(self);
    // return 1;
  }
}

pub trait QDir_relativeFilePath<RetType> {
  fn relativeFilePath(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::relativeFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_relativeFilePath<QString> for (&'a QString) {
  fn relativeFilePath(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir16relativeFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir16relativeFilePathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QDir::tempPath();
impl /*struct*/ QDir {
  pub fn tempPath_s<RetType, T: QDir_tempPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.tempPath_s();
    // return 1;
  }
}

pub trait QDir_tempPath_s<RetType> {
  fn tempPath_s(self ) -> RetType;
}

  // proto: static QString QDir::tempPath();
impl<'a> /*trait*/ QDir_tempPath_s<QString> for () {
  fn tempPath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8tempPathEv()};
    let mut ret = unsafe {_ZN4QDir8tempPathEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDir::QDir(const QString & path);
impl /*struct*/ QDir {
  pub fn New<T: QDir_New>(value: T) -> QDir {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDir_New {
  fn New(self) -> QDir;
}

  // proto:  void QDir::QDir(const QString & path);
impl<'a> /*trait*/ QDir_New for (&'a QString) {
  fn New(self) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirC1ERK7QString()};
    let ctysz: c_int = unsafe{QDir_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN4QDirC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN4QDirC1ERK7QString(arg0)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QDir::nameFilters();
impl /*struct*/ QDir {
  pub fn nameFilters<RetType, T: QDir_nameFilters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameFilters(self);
    // return 1;
  }
}

pub trait QDir_nameFilters<RetType> {
  fn nameFilters(self , rsthis: & QDir) -> RetType;
}

  // proto:  QStringList QDir::nameFilters();
impl<'a> /*trait*/ QDir_nameFilters<()> for () {
  fn nameFilters(self , rsthis: & QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir11nameFiltersEv()};
     unsafe {_ZNK4QDir11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDir::cd(const QString & dirName);
impl /*struct*/ QDir {
  pub fn cd<RetType, T: QDir_cd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cd(self);
    // return 1;
  }
}

pub trait QDir_cd<RetType> {
  fn cd(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::cd(const QString & dirName);
impl<'a> /*trait*/ QDir_cd<i8> for (&'a QString) {
  fn cd(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir2cdERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir2cdERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QFileInfoList QDir::drives();
impl /*struct*/ QDir {
  pub fn drives_s<RetType, T: QDir_drives_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.drives_s();
    // return 1;
  }
}

pub trait QDir_drives_s<RetType> {
  fn drives_s(self ) -> RetType;
}

  // proto: static QFileInfoList QDir::drives();
impl<'a> /*trait*/ QDir_drives_s<()> for () {
  fn drives_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6drivesEv()};
     unsafe {_ZN4QDir6drivesEv()};
    // return 1;
  }
}

  // proto: static bool QDir::setCurrent(const QString & path);
impl /*struct*/ QDir {
  pub fn setCurrent_s<RetType, T: QDir_setCurrent_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCurrent_s();
    // return 1;
  }
}

pub trait QDir_setCurrent_s<RetType> {
  fn setCurrent_s(self ) -> RetType;
}

  // proto: static bool QDir::setCurrent(const QString & path);
impl<'a> /*trait*/ QDir_setCurrent_s<i8> for (&'a QString) {
  fn setCurrent_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir10setCurrentERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir10setCurrentERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDir::rmdir(const QString & dirName);
impl /*struct*/ QDir {
  pub fn rmdir<RetType, T: QDir_rmdir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rmdir(self);
    // return 1;
  }
}

pub trait QDir_rmdir<RetType> {
  fn rmdir(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::rmdir(const QString & dirName);
impl<'a> /*trait*/ QDir_rmdir<i8> for (&'a QString) {
  fn rmdir(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5rmdirERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir5rmdirERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDir::cdUp();
impl /*struct*/ QDir {
  pub fn cdUp<RetType, T: QDir_cdUp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cdUp(self);
    // return 1;
  }
}

pub trait QDir_cdUp<RetType> {
  fn cdUp(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::cdUp();
impl<'a> /*trait*/ QDir_cdUp<i8> for () {
  fn cdUp(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4cdUpEv()};
    let mut ret = unsafe {_ZN4QDir4cdUpEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QDir::absolutePath();
impl /*struct*/ QDir {
  pub fn absolutePath<RetType, T: QDir_absolutePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.absolutePath(self);
    // return 1;
  }
}

pub trait QDir_absolutePath<RetType> {
  fn absolutePath(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::absolutePath();
impl<'a> /*trait*/ QDir_absolutePath<QString> for () {
  fn absolutePath(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir12absolutePathEv()};
    let mut ret = unsafe {_ZNK4QDir12absolutePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
impl /*struct*/ QDir {
  pub fn setSearchPaths_s<RetType, T: QDir_setSearchPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setSearchPaths_s();
    // return 1;
  }
}

pub trait QDir_setSearchPaths_s<RetType> {
  fn setSearchPaths_s(self ) -> RetType;
}

  // proto: static void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
impl<'a> /*trait*/ QDir_setSearchPaths_s<()> for (&'a QString, &'a QStringList) {
  fn setSearchPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setSearchPathsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QDir::absoluteFilePath(const QString & fileName);
impl /*struct*/ QDir {
  pub fn absoluteFilePath<RetType, T: QDir_absoluteFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QDir_absoluteFilePath<RetType> {
  fn absoluteFilePath(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::absoluteFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_absoluteFilePath<QString> for (&'a QString) {
  fn absoluteFilePath(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir16absoluteFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir16absoluteFilePathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDir::rename(const QString & oldName, const QString & newName);
impl /*struct*/ QDir {
  pub fn rename<RetType, T: QDir_rename<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rename(self);
    // return 1;
  }
}

pub trait QDir_rename<RetType> {
  fn rename(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QDir_rename<i8> for (&'a QString, &'a QString) {
  fn rename(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6renameERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir6renameERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QDir::match(const QString & filter, const QString & fileName);
impl /*struct*/ QDir {
  pub fn match_s<RetType, T: QDir_match_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.match_s();
    // return 1;
  }
}

pub trait QDir_match_s<RetType> {
  fn match_s(self ) -> RetType;
}

  // proto: static bool QDir::match(const QString & filter, const QString & fileName);
impl<'a> /*trait*/ QDir_match_s<i8> for (&'a QString, &'a QString) {
  fn match_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir5matchERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir5matchERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDir::refresh();
impl /*struct*/ QDir {
  pub fn refresh<RetType, T: QDir_refresh<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refresh(self);
    // return 1;
  }
}

pub trait QDir_refresh<RetType> {
  fn refresh(self , rsthis: & QDir) -> RetType;
}

  // proto:  void QDir::refresh();
impl<'a> /*trait*/ QDir_refresh<()> for () {
  fn refresh(self , rsthis: & QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7refreshEv()};
     unsafe {_ZNK4QDir7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDir::mkdir(const QString & dirName);
impl /*struct*/ QDir {
  pub fn mkdir<RetType, T: QDir_mkdir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mkdir(self);
    // return 1;
  }
}

pub trait QDir_mkdir<RetType> {
  fn mkdir(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::mkdir(const QString & dirName);
impl<'a> /*trait*/ QDir_mkdir<i8> for (&'a QString) {
  fn mkdir(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5mkdirERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir5mkdirERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  uint QDir::count();
impl /*struct*/ QDir {
  pub fn count<RetType, T: QDir_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QDir_count<RetType> {
  fn count(self , rsthis: & QDir) -> RetType;
}

  // proto:  uint QDir::count();
impl<'a> /*trait*/ QDir_count<u32> for () {
  fn count(self , rsthis: & QDir) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5countEv()};
    let mut ret = unsafe {_ZNK4QDir5countEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static QStringList QDir::nameFiltersFromString(const QString & nameFilter);
impl /*struct*/ QDir {
  pub fn nameFiltersFromString_s<RetType, T: QDir_nameFiltersFromString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.nameFiltersFromString_s();
    // return 1;
  }
}

pub trait QDir_nameFiltersFromString_s<RetType> {
  fn nameFiltersFromString_s(self ) -> RetType;
}

  // proto: static QStringList QDir::nameFiltersFromString(const QString & nameFilter);
impl<'a> /*trait*/ QDir_nameFiltersFromString_s<()> for (&'a QString) {
  fn nameFiltersFromString_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21nameFiltersFromStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir21nameFiltersFromStringERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  QString QDir::filePath(const QString & fileName);
impl /*struct*/ QDir {
  pub fn filePath<RetType, T: QDir_filePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QDir_filePath<RetType> {
  fn filePath(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::filePath(const QString & fileName);
impl<'a> /*trait*/ QDir_filePath<QString> for (&'a QString) {
  fn filePath(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir8filePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir8filePathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDir::rmpath(const QString & dirPath);
impl /*struct*/ QDir {
  pub fn rmpath<RetType, T: QDir_rmpath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rmpath(self);
    // return 1;
  }
}

pub trait QDir_rmpath<RetType> {
  fn rmpath(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::rmpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_rmpath<i8> for (&'a QString) {
  fn rmpath(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6rmpathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir6rmpathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QDir::path();
impl /*struct*/ QDir {
  pub fn path<RetType, T: QDir_path<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QDir_path<RetType> {
  fn path(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::path();
impl<'a> /*trait*/ QDir_path<QString> for () {
  fn path(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir4pathEv()};
    let mut ret = unsafe {_ZNK4QDir4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QDir::toNativeSeparators(const QString & pathName);
impl /*struct*/ QDir {
  pub fn toNativeSeparators_s<RetType, T: QDir_toNativeSeparators_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toNativeSeparators_s();
    // return 1;
  }
}

pub trait QDir_toNativeSeparators_s<RetType> {
  fn toNativeSeparators_s(self ) -> RetType;
}

  // proto: static QString QDir::toNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_toNativeSeparators_s<QString> for (&'a QString) {
  fn toNativeSeparators_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir18toNativeSeparatorsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir18toNativeSeparatorsERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QDir::cleanPath(const QString & path);
impl /*struct*/ QDir {
  pub fn cleanPath_s<RetType, T: QDir_cleanPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanPath_s();
    // return 1;
  }
}

pub trait QDir_cleanPath_s<RetType> {
  fn cleanPath_s(self ) -> RetType;
}

  // proto: static QString QDir::cleanPath(const QString & path);
impl<'a> /*trait*/ QDir_cleanPath_s<QString> for (&'a QString) {
  fn cleanPath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9cleanPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir9cleanPathERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDir::exists();
impl /*struct*/ QDir {
  pub fn exists<RetType, T: QDir_exists<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exists(self);
    // return 1;
  }
}

pub trait QDir_exists<RetType> {
  fn exists(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::exists();
impl<'a> /*trait*/ QDir_exists<i8> for () {
  fn exists(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsEv()};
    let mut ret = unsafe {_ZNK4QDir6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDir::remove(const QString & fileName);
impl /*struct*/ QDir {
  pub fn remove<RetType, T: QDir_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QDir_remove<RetType> {
  fn remove(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::remove(const QString & fileName);
impl<'a> /*trait*/ QDir_remove<i8> for (&'a QString) {
  fn remove(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir6removeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDir::~QDir();
impl /*struct*/ QDir {
  pub fn Free<RetType, T: QDir_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDir_Free<RetType> {
  fn Free(self , rsthis: & QDir) -> RetType;
}

  // proto:  void QDir::~QDir();
impl<'a> /*trait*/ QDir_Free<()> for () {
  fn Free(self , rsthis: & QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirD0Ev()};
     unsafe {_ZN4QDirD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QString QDir::rootPath();
impl /*struct*/ QDir {
  pub fn rootPath_s<RetType, T: QDir_rootPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.rootPath_s();
    // return 1;
  }
}

pub trait QDir_rootPath_s<RetType> {
  fn rootPath_s(self ) -> RetType;
}

  // proto: static QString QDir::rootPath();
impl<'a> /*trait*/ QDir_rootPath_s<QString> for () {
  fn rootPath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8rootPathEv()};
    let mut ret = unsafe {_ZN4QDir8rootPathEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QDir::match(const QStringList & filters, const QString & fileName);
impl<'a> /*trait*/ QDir_match_s<i8> for (&'a QStringList, &'a QString) {
  fn match_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir5matchERK11QStringListRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir5matchERK11QStringListRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QDir::fromNativeSeparators(const QString & pathName);
impl /*struct*/ QDir {
  pub fn fromNativeSeparators_s<RetType, T: QDir_fromNativeSeparators_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromNativeSeparators_s();
    // return 1;
  }
}

pub trait QDir_fromNativeSeparators_s<RetType> {
  fn fromNativeSeparators_s(self ) -> RetType;
}

  // proto: static QString QDir::fromNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_fromNativeSeparators_s<QString> for (&'a QString) {
  fn fromNativeSeparators_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir20fromNativeSeparatorsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir20fromNativeSeparatorsERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDir::setNameFilters(const QStringList & nameFilters);
impl /*struct*/ QDir {
  pub fn setNameFilters<RetType, T: QDir_setNameFilters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters(self);
    // return 1;
  }
}

pub trait QDir_setNameFilters<RetType> {
  fn setNameFilters(self , rsthis: & QDir) -> RetType;
}

  // proto:  void QDir::setNameFilters(const QStringList & nameFilters);
impl<'a> /*trait*/ QDir_setNameFilters<()> for (&'a QStringList) {
  fn setNameFilters(self , rsthis: & QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QChar QDir::separator();
impl /*struct*/ QDir {
  pub fn separator_s<RetType, T: QDir_separator_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.separator_s();
    // return 1;
  }
}

pub trait QDir_separator_s<RetType> {
  fn separator_s(self ) -> RetType;
}

  // proto: static QChar QDir::separator();
impl<'a> /*trait*/ QDir_separator_s<QChar> for () {
  fn separator_s(self ) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9separatorEv()};
    let mut ret = unsafe {_ZN4QDir9separatorEv()};
    let mut ret1 = QChar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDir::exists(const QString & name);
impl<'a> /*trait*/ QDir_exists<i8> for (&'a QString) {
  fn exists(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir6existsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDir::mkpath(const QString & dirPath);
impl /*struct*/ QDir {
  pub fn mkpath<RetType, T: QDir_mkpath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mkpath(self);
    // return 1;
  }
}

pub trait QDir_mkpath<RetType> {
  fn mkpath(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::mkpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_mkpath<i8> for (&'a QString) {
  fn mkpath(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6mkpathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir6mkpathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QDir::addSearchPath(const QString & prefix, const QString & path);
impl /*struct*/ QDir {
  pub fn addSearchPath_s<RetType, T: QDir_addSearchPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addSearchPath_s();
    // return 1;
  }
}

pub trait QDir_addSearchPath_s<RetType> {
  fn addSearchPath_s(self ) -> RetType;
}

  // proto: static void QDir::addSearchPath(const QString & prefix, const QString & path);
impl<'a> /*trait*/ QDir_addSearchPath_s<()> for (&'a QString, &'a QString) {
  fn addSearchPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir13addSearchPathERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir13addSearchPathERK7QStringS2_(arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QDir::dirName();
impl /*struct*/ QDir {
  pub fn dirName<RetType, T: QDir_dirName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dirName(self);
    // return 1;
  }
}

pub trait QDir_dirName<RetType> {
  fn dirName(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::dirName();
impl<'a> /*trait*/ QDir_dirName<QString> for () {
  fn dirName(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7dirNameEv()};
    let mut ret = unsafe {_ZNK4QDir7dirNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QStringList QDir::searchPaths(const QString & prefix);
impl /*struct*/ QDir {
  pub fn searchPaths_s<RetType, T: QDir_searchPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.searchPaths_s();
    // return 1;
  }
}

pub trait QDir_searchPaths_s<RetType> {
  fn searchPaths_s(self ) -> RetType;
}

  // proto: static QStringList QDir::searchPaths(const QString & prefix);
impl<'a> /*trait*/ QDir_searchPaths_s<()> for (&'a QString) {
  fn searchPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11searchPathsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir11searchPathsERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  bool QDir::makeAbsolute();
impl /*struct*/ QDir {
  pub fn makeAbsolute<RetType, T: QDir_makeAbsolute<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeAbsolute(self);
    // return 1;
  }
}

pub trait QDir_makeAbsolute<RetType> {
  fn makeAbsolute(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::makeAbsolute();
impl<'a> /*trait*/ QDir_makeAbsolute<i8> for () {
  fn makeAbsolute(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir12makeAbsoluteEv()};
    let mut ret = unsafe {_ZN4QDir12makeAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QDir::canonicalPath();
impl /*struct*/ QDir {
  pub fn canonicalPath<RetType, T: QDir_canonicalPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canonicalPath(self);
    // return 1;
  }
}

pub trait QDir_canonicalPath<RetType> {
  fn canonicalPath(self , rsthis: & QDir) -> RetType;
}

  // proto:  QString QDir::canonicalPath();
impl<'a> /*trait*/ QDir_canonicalPath<QString> for () {
  fn canonicalPath(self , rsthis: & QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir13canonicalPathEv()};
    let mut ret = unsafe {_ZNK4QDir13canonicalPathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDir::isReadable();
impl /*struct*/ QDir {
  pub fn isReadable<RetType, T: QDir_isReadable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadable(self);
    // return 1;
  }
}

pub trait QDir_isReadable<RetType> {
  fn isReadable(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::isReadable();
impl<'a> /*trait*/ QDir_isReadable<i8> for () {
  fn isReadable(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isReadableEv()};
    let mut ret = unsafe {_ZNK4QDir10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDir::isRelative();
impl /*struct*/ QDir {
  pub fn isRelative<RetType, T: QDir_isRelative<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRelative(self);
    // return 1;
  }
}

pub trait QDir_isRelative<RetType> {
  fn isRelative(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::isRelative();
impl<'a> /*trait*/ QDir_isRelative<i8> for () {
  fn isRelative(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isRelativeEv()};
    let mut ret = unsafe {_ZNK4QDir10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QDir::currentPath();
impl /*struct*/ QDir {
  pub fn currentPath_s<RetType, T: QDir_currentPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentPath_s();
    // return 1;
  }
}

pub trait QDir_currentPath_s<RetType> {
  fn currentPath_s(self ) -> RetType;
}

  // proto: static QString QDir::currentPath();
impl<'a> /*trait*/ QDir_currentPath_s<QString> for () {
  fn currentPath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11currentPathEv()};
    let mut ret = unsafe {_ZN4QDir11currentPathEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDir::isRoot();
impl /*struct*/ QDir {
  pub fn isRoot<RetType, T: QDir_isRoot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRoot(self);
    // return 1;
  }
}

pub trait QDir_isRoot<RetType> {
  fn isRoot(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::isRoot();
impl<'a> /*trait*/ QDir_isRoot<i8> for () {
  fn isRoot(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6isRootEv()};
    let mut ret = unsafe {_ZNK4QDir6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDir::removeRecursively();
impl /*struct*/ QDir {
  pub fn removeRecursively<RetType, T: QDir_removeRecursively<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRecursively(self);
    // return 1;
  }
}

pub trait QDir_removeRecursively<RetType> {
  fn removeRecursively(self , rsthis: & QDir) -> RetType;
}

  // proto:  bool QDir::removeRecursively();
impl<'a> /*trait*/ QDir_removeRecursively<i8> for () {
  fn removeRecursively(self , rsthis: & QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir17removeRecursivelyEv()};
    let mut ret = unsafe {_ZN4QDir17removeRecursivelyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QDir::homePath();
impl /*struct*/ QDir {
  pub fn homePath_s<RetType, T: QDir_homePath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.homePath_s();
    // return 1;
  }
}

pub trait QDir_homePath_s<RetType> {
  fn homePath_s(self ) -> RetType;
}

  // proto: static QString QDir::homePath();
impl<'a> /*trait*/ QDir_homePath_s<QString> for () {
  fn homePath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8homePathEv()};
    let mut ret = unsafe {_ZN4QDir8homePathEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDir::QDir(const QDir & );
impl<'a> /*trait*/ QDir_New for (&'a QDir) {
  fn New(self) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirC1ERKS_()};
    let ctysz: c_int = unsafe{QDir_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN4QDirC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN4QDirC1ERKS_(arg0)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDir::setPath(const QString & path);
impl /*struct*/ QDir {
  pub fn setPath<RetType, T: QDir_setPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPath(self);
    // return 1;
  }
}

pub trait QDir_setPath<RetType> {
  fn setPath(self , rsthis: & QDir) -> RetType;
}

  // proto:  void QDir::setPath(const QString & path);
impl<'a> /*trait*/ QDir_setPath<()> for (&'a QString) {
  fn setPath(self , rsthis: & QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir7setPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir7setPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QDir::isRelativePath(const QString & path);
impl /*struct*/ QDir {
  pub fn isRelativePath_s<RetType, T: QDir_isRelativePath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isRelativePath_s();
    // return 1;
  }
}

pub trait QDir_isRelativePath_s<RetType> {
  fn isRelativePath_s(self ) -> RetType;
}

  // proto: static bool QDir::isRelativePath(const QString & path);
impl<'a> /*trait*/ QDir_isRelativePath_s<i8> for (&'a QString) {
  fn isRelativePath_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14isRelativePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir14isRelativePathERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

