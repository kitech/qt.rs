// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QDir::addResourceSearchPath(const QString & path);
  fn _ZN4QDir21addResourceSearchPathERK7QString(arg0: *mut c_void) ;
  // proto: static bool QDir::isAbsolutePath(const QString & path);
  fn _ZN4QDir14isAbsolutePathERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  QString QDir::relativeFilePath(const QString & fileName);
  fn _ZNK4QDir16relativeFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QDir::tempPath();
  fn _ZN4QDir8tempPathEv() -> *mut c_void;
  // proto:  void QDir::NewQDir(const QString & path);
  fn _ZN4QDirC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QDir::nameFilters();
  fn _ZNK4QDir11nameFiltersEv(qthis: *mut c_void) ;
  // proto:  bool QDir::cd(const QString & dirName);
  fn _ZN4QDir2cdERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static QList<QFileInfo> QDir::drives();
  fn _ZN4QDir6drivesEv() ;
  // proto: static bool QDir::setCurrent(const QString & path);
  fn _ZN4QDir10setCurrentERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  bool QDir::rmdir(const QString & dirName);
  fn _ZNK4QDir5rmdirERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QDir::cdUp();
  fn _ZN4QDir4cdUpEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QDir::absolutePath();
  fn _ZNK4QDir12absolutePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
  fn _ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QDir::absoluteFilePath(const QString & fileName);
  fn _ZNK4QDir16absoluteFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::rename(const QString & oldName, const QString & newName);
  fn _ZN4QDir6renameERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static bool QDir::match_(const QString & filter, const QString & fileName);
  fn _ZN4QDir5matchERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  void QDir::refresh();
  fn _ZNK4QDir7refreshEv(qthis: *mut c_void) ;
  // proto:  bool QDir::mkdir(const QString & dirName);
  fn _ZNK4QDir5mkdirERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  unsigned int QDir::count();
  fn _ZNK4QDir5countEv(qthis: *mut c_void) -> c_uint;
  // proto: static QDir QDir::root();
  fn _ZN4QDir4rootEv() -> *mut c_void;
  // proto: static QStringList QDir::nameFiltersFromString(const QString & nameFilter);
  fn _ZN4QDir21nameFiltersFromStringERK7QString(arg0: *mut c_void) ;
  // proto:  QString QDir::filePath(const QString & fileName);
  fn _ZNK4QDir8filePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::rmpath(const QString & dirPath);
  fn _ZNK4QDir6rmpathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QDir::path();
  fn _ZNK4QDir4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QDir::toNativeSeparators(const QString & pathName);
  fn _ZN4QDir18toNativeSeparatorsERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QDir::cleanPath(const QString & path);
  fn _ZN4QDir9cleanPathERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::exists();
  fn _ZNK4QDir6existsEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QDir::remove(const QString & fileName);
  fn _ZN4QDir6removeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QDir::FreeQDir();
  fn _ZN4QDirD0Ev(qthis: *mut c_void) ;
  // proto: static QString QDir::rootPath();
  fn _ZN4QDir8rootPathEv() -> *mut c_void;
  // proto: static QDir QDir::current();
  fn _ZN4QDir7currentEv() -> *mut c_void;
  // proto: static bool QDir::match_(const QStringList & filters, const QString & fileName);
  fn _ZN4QDir5matchERK11QStringListRK7QString(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static QString QDir::fromNativeSeparators(const QString & pathName);
  fn _ZN4QDir20fromNativeSeparatorsERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QDir QDir::home();
  fn _ZN4QDir4homeEv() -> *mut c_void;
  // proto:  void QDir::setNameFilters(const QStringList & nameFilters);
  fn _ZN4QDir14setNameFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QChar QDir::separator();
  fn _ZN4QDir9separatorEv() -> *mut c_void;
  // proto:  void QDir::swap(QDir & other);
  fn _ZN4QDir4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QDir QDir::temp();
  fn _ZN4QDir4tempEv() -> *mut c_void;
  // proto:  bool QDir::exists(const QString & name);
  fn _ZNK4QDir6existsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QDir::mkpath(const QString & dirPath);
  fn _ZNK4QDir6mkpathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static void QDir::addSearchPath(const QString & prefix, const QString & path);
  fn _ZN4QDir13addSearchPathERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QDir::dirName();
  fn _ZNK4QDir7dirNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QStringList QDir::searchPaths(const QString & prefix);
  fn _ZN4QDir11searchPathsERK7QString(arg0: *mut c_void) ;
  // proto:  bool QDir::makeAbsolute();
  fn _ZN4QDir12makeAbsoluteEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QDir::canonicalPath();
  fn _ZNK4QDir13canonicalPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDir::isReadable();
  fn _ZNK4QDir10isReadableEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QDir::isRelative();
  fn _ZNK4QDir10isRelativeEv(qthis: *mut c_void) -> int8_t;
  // proto: static QString QDir::currentPath();
  fn _ZN4QDir11currentPathEv() -> *mut c_void;
  // proto:  bool QDir::isRoot();
  fn _ZNK4QDir6isRootEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QDir::removeRecursively();
  fn _ZN4QDir17removeRecursivelyEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QDir::isAbsolute();
  fn _ZNK4QDir10isAbsoluteEv(qthis: *mut c_void) -> int8_t;
  // proto: static QString QDir::homePath();
  fn _ZN4QDir8homePathEv() -> *mut c_void;
  // proto:  void QDir::NewQDir(const QDir & );
  fn _ZN4QDirC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDir::setPath(const QString & path);
  fn _ZN4QDir7setPathERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QDir::isRelativePath(const QString & path);
  fn _ZN4QDir14isRelativePathERK7QString(arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QDir)=1
pub struct QDir {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDir {
  pub fn addResourceSearchPath<T: QDir_addResourceSearchPath>(&mut self, value: T)  {
     value.addResourceSearchPath(self);
    // return 1;
  }
}

pub trait QDir_addResourceSearchPath {
  fn addResourceSearchPath(self, rsthis: &mut QDir) ;
}

// proto: static void QDir::addResourceSearchPath(const QString & path);
impl<'a> /*trait*/ QDir_addResourceSearchPath for (&'a  QString) {
  fn addResourceSearchPath(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21addResourceSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir21addResourceSearchPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isAbsolutePath<T: QDir_isAbsolutePath>(&mut self, value: T) -> i8 {
    return value.isAbsolutePath(self);
    // return 1;
  }
}

pub trait QDir_isAbsolutePath {
  fn isAbsolutePath(self, rsthis: &mut QDir) -> i8;
}

// proto: static bool QDir::isAbsolutePath(const QString & path);
impl<'a> /*trait*/ QDir_isAbsolutePath for (&'a  QString) {
  fn isAbsolutePath(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14isAbsolutePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir14isAbsolutePathERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn relativeFilePath<T: QDir_relativeFilePath>(&mut self, value: T) -> QString {
    return value.relativeFilePath(self);
    // return 1;
  }
}

pub trait QDir_relativeFilePath {
  fn relativeFilePath(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::relativeFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_relativeFilePath for (&'a  QString) {
  fn relativeFilePath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir16relativeFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir16relativeFilePathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn tempPath<T: QDir_tempPath>(&mut self, value: T) -> QString {
    return value.tempPath(self);
    // return 1;
  }
}

pub trait QDir_tempPath {
  fn tempPath(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::tempPath();
impl<'a> /*trait*/ QDir_tempPath for () {
  fn tempPath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8tempPathEv()};
    let mut ret = unsafe {_ZN4QDir8tempPathEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn NewQDir<T: QDir_NewQDir>(value: T) -> QDir {
    let rsthis = value.NewQDir();
    return rsthis;
    // return 1;
  }
}

pub trait QDir_NewQDir {
  fn NewQDir(self) -> QDir;
}

// proto: void QDir::NewQDir(const QString & path);
impl<'a> /*trait*/ QDir_NewQDir for (&'a  QString) {
  fn NewQDir(self) -> QDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QDirC1ERK7QString(qthis, arg0)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn nameFilters<T: QDir_nameFilters>(&mut self, value: T)  {
     value.nameFilters(self);
    // return 1;
  }
}

pub trait QDir_nameFilters {
  fn nameFilters(self, rsthis: &mut QDir) ;
}

// proto:  QStringList QDir::nameFilters();
impl<'a> /*trait*/ QDir_nameFilters for () {
  fn nameFilters(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir11nameFiltersEv()};
     unsafe {_ZNK4QDir11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cd<T: QDir_cd>(&mut self, value: T) -> i8 {
    return value.cd(self);
    // return 1;
  }
}

pub trait QDir_cd {
  fn cd(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::cd(const QString & dirName);
impl<'a> /*trait*/ QDir_cd for (&'a  QString) {
  fn cd(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir2cdERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir2cdERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn drives<T: QDir_drives>(&mut self, value: T)  {
     value.drives(self);
    // return 1;
  }
}

pub trait QDir_drives {
  fn drives(self, rsthis: &mut QDir) ;
}

// proto: static QList<QFileInfo> QDir::drives();
impl<'a> /*trait*/ QDir_drives for () {
  fn drives(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6drivesEv()};
     unsafe {_ZN4QDir6drivesEv()};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setCurrent<T: QDir_setCurrent>(&mut self, value: T) -> i8 {
    return value.setCurrent(self);
    // return 1;
  }
}

pub trait QDir_setCurrent {
  fn setCurrent(self, rsthis: &mut QDir) -> i8;
}

// proto: static bool QDir::setCurrent(const QString & path);
impl<'a> /*trait*/ QDir_setCurrent for (&'a  QString) {
  fn setCurrent(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir10setCurrentERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir10setCurrentERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rmdir<T: QDir_rmdir>(&mut self, value: T) -> i8 {
    return value.rmdir(self);
    // return 1;
  }
}

pub trait QDir_rmdir {
  fn rmdir(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::rmdir(const QString & dirName);
impl<'a> /*trait*/ QDir_rmdir for (&'a  QString) {
  fn rmdir(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5rmdirERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir5rmdirERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cdUp<T: QDir_cdUp>(&mut self, value: T) -> i8 {
    return value.cdUp(self);
    // return 1;
  }
}

pub trait QDir_cdUp {
  fn cdUp(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::cdUp();
impl<'a> /*trait*/ QDir_cdUp for () {
  fn cdUp(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4cdUpEv()};
    let mut ret = unsafe {_ZN4QDir4cdUpEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn absolutePath<T: QDir_absolutePath>(&mut self, value: T) -> QString {
    return value.absolutePath(self);
    // return 1;
  }
}

pub trait QDir_absolutePath {
  fn absolutePath(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::absolutePath();
impl<'a> /*trait*/ QDir_absolutePath for () {
  fn absolutePath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir12absolutePathEv()};
    let mut ret = unsafe {_ZNK4QDir12absolutePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setSearchPaths<T: QDir_setSearchPaths>(&mut self, value: T)  {
     value.setSearchPaths(self);
    // return 1;
  }
}

pub trait QDir_setSearchPaths {
  fn setSearchPaths(self, rsthis: &mut QDir) ;
}

// proto: static void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
impl<'a> /*trait*/ QDir_setSearchPaths for (&'a  QString, &'a  QStringList) {
  fn setSearchPaths(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setSearchPathsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn absoluteFilePath<T: QDir_absoluteFilePath>(&mut self, value: T) -> QString {
    return value.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QDir_absoluteFilePath {
  fn absoluteFilePath(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::absoluteFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_absoluteFilePath for (&'a  QString) {
  fn absoluteFilePath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir16absoluteFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir16absoluteFilePathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rename<T: QDir_rename>(&mut self, value: T) -> i8 {
    return value.rename(self);
    // return 1;
  }
}

pub trait QDir_rename {
  fn rename(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QDir_rename for (&'a  QString, &'a  QString) {
  fn rename(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6renameERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir6renameERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn match_<T: QDir_match_>(&mut self, value: T) -> i8 {
    return value.match_(self);
    // return 1;
  }
}

pub trait QDir_match_ {
  fn match_(self, rsthis: &mut QDir) -> i8;
}

// proto: static bool QDir::match_(const QString & filter, const QString & fileName);
impl<'a> /*trait*/ QDir_match_ for (&'a  QString, &'a  QString) {
  fn match_(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir5matchERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir5matchERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn refresh<T: QDir_refresh>(&mut self, value: T)  {
     value.refresh(self);
    // return 1;
  }
}

pub trait QDir_refresh {
  fn refresh(self, rsthis: &mut QDir) ;
}

// proto:  void QDir::refresh();
impl<'a> /*trait*/ QDir_refresh for () {
  fn refresh(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7refreshEv()};
     unsafe {_ZNK4QDir7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn mkdir<T: QDir_mkdir>(&mut self, value: T) -> i8 {
    return value.mkdir(self);
    // return 1;
  }
}

pub trait QDir_mkdir {
  fn mkdir(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::mkdir(const QString & dirName);
impl<'a> /*trait*/ QDir_mkdir for (&'a  QString) {
  fn mkdir(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5mkdirERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir5mkdirERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn count<T: QDir_count>(&mut self, value: T) -> u32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QDir_count {
  fn count(self, rsthis: &mut QDir) -> u32;
}

// proto:  unsigned int QDir::count();
impl<'a> /*trait*/ QDir_count for () {
  fn count(self, rsthis: &mut QDir) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5countEv()};
    let mut ret = unsafe {_ZNK4QDir5countEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn root<T: QDir_root>(&mut self, value: T) -> QDir {
    return value.root(self);
    // return 1;
  }
}

pub trait QDir_root {
  fn root(self, rsthis: &mut QDir) -> QDir;
}

// proto: static QDir QDir::root();
impl<'a> /*trait*/ QDir_root for () {
  fn root(self, rsthis: &mut QDir) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4rootEv()};
    let mut ret = unsafe {_ZN4QDir4rootEv()};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn nameFiltersFromString<T: QDir_nameFiltersFromString>(&mut self, value: T)  {
     value.nameFiltersFromString(self);
    // return 1;
  }
}

pub trait QDir_nameFiltersFromString {
  fn nameFiltersFromString(self, rsthis: &mut QDir) ;
}

// proto: static QStringList QDir::nameFiltersFromString(const QString & nameFilter);
impl<'a> /*trait*/ QDir_nameFiltersFromString for (&'a  QString) {
  fn nameFiltersFromString(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21nameFiltersFromStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir21nameFiltersFromStringERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn filePath<T: QDir_filePath>(&mut self, value: T) -> QString {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QDir_filePath {
  fn filePath(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::filePath(const QString & fileName);
impl<'a> /*trait*/ QDir_filePath for (&'a  QString) {
  fn filePath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir8filePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir8filePathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rmpath<T: QDir_rmpath>(&mut self, value: T) -> i8 {
    return value.rmpath(self);
    // return 1;
  }
}

pub trait QDir_rmpath {
  fn rmpath(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::rmpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_rmpath for (&'a  QString) {
  fn rmpath(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6rmpathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir6rmpathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn path<T: QDir_path>(&mut self, value: T) -> QString {
    return value.path(self);
    // return 1;
  }
}

pub trait QDir_path {
  fn path(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::path();
impl<'a> /*trait*/ QDir_path for () {
  fn path(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir4pathEv()};
    let mut ret = unsafe {_ZNK4QDir4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn toNativeSeparators<T: QDir_toNativeSeparators>(&mut self, value: T) -> QString {
    return value.toNativeSeparators(self);
    // return 1;
  }
}

pub trait QDir_toNativeSeparators {
  fn toNativeSeparators(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::toNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_toNativeSeparators for (&'a  QString) {
  fn toNativeSeparators(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir18toNativeSeparatorsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir18toNativeSeparatorsERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cleanPath<T: QDir_cleanPath>(&mut self, value: T) -> QString {
    return value.cleanPath(self);
    // return 1;
  }
}

pub trait QDir_cleanPath {
  fn cleanPath(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::cleanPath(const QString & path);
impl<'a> /*trait*/ QDir_cleanPath for (&'a  QString) {
  fn cleanPath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9cleanPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir9cleanPathERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn exists<T: QDir_exists>(&mut self, value: T) -> i8 {
    return value.exists(self);
    // return 1;
  }
}

pub trait QDir_exists {
  fn exists(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::exists();
impl<'a> /*trait*/ QDir_exists for () {
  fn exists(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsEv()};
    let mut ret = unsafe {_ZNK4QDir6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn remove<T: QDir_remove>(&mut self, value: T) -> i8 {
    return value.remove(self);
    // return 1;
  }
}

pub trait QDir_remove {
  fn remove(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::remove(const QString & fileName);
impl<'a> /*trait*/ QDir_remove for (&'a  QString) {
  fn remove(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir6removeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn FreeQDir<T: QDir_FreeQDir>(&mut self, value: T)  {
     value.FreeQDir(self);
    // return 1;
  }
}

pub trait QDir_FreeQDir {
  fn FreeQDir(self, rsthis: &mut QDir) ;
}

// proto:  void QDir::FreeQDir();
impl<'a> /*trait*/ QDir_FreeQDir for () {
  fn FreeQDir(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirD0Ev()};
     unsafe {_ZN4QDirD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rootPath<T: QDir_rootPath>(&mut self, value: T) -> QString {
    return value.rootPath(self);
    // return 1;
  }
}

pub trait QDir_rootPath {
  fn rootPath(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::rootPath();
impl<'a> /*trait*/ QDir_rootPath for () {
  fn rootPath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8rootPathEv()};
    let mut ret = unsafe {_ZN4QDir8rootPathEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn current<T: QDir_current>(&mut self, value: T) -> QDir {
    return value.current(self);
    // return 1;
  }
}

pub trait QDir_current {
  fn current(self, rsthis: &mut QDir) -> QDir;
}

// proto: static QDir QDir::current();
impl<'a> /*trait*/ QDir_current for () {
  fn current(self, rsthis: &mut QDir) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir7currentEv()};
    let mut ret = unsafe {_ZN4QDir7currentEv()};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static bool QDir::match_(const QStringList & filters, const QString & fileName);
impl<'a> /*trait*/ QDir_match_ for (&'a  QStringList, &'a  QString) {
  fn match_(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir5matchERK11QStringListRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir5matchERK11QStringListRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn fromNativeSeparators<T: QDir_fromNativeSeparators>(&mut self, value: T) -> QString {
    return value.fromNativeSeparators(self);
    // return 1;
  }
}

pub trait QDir_fromNativeSeparators {
  fn fromNativeSeparators(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::fromNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_fromNativeSeparators for (&'a  QString) {
  fn fromNativeSeparators(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir20fromNativeSeparatorsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir20fromNativeSeparatorsERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn home<T: QDir_home>(&mut self, value: T) -> QDir {
    return value.home(self);
    // return 1;
  }
}

pub trait QDir_home {
  fn home(self, rsthis: &mut QDir) -> QDir;
}

// proto: static QDir QDir::home();
impl<'a> /*trait*/ QDir_home for () {
  fn home(self, rsthis: &mut QDir) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4homeEv()};
    let mut ret = unsafe {_ZN4QDir4homeEv()};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setNameFilters<T: QDir_setNameFilters>(&mut self, value: T)  {
     value.setNameFilters(self);
    // return 1;
  }
}

pub trait QDir_setNameFilters {
  fn setNameFilters(self, rsthis: &mut QDir) ;
}

// proto:  void QDir::setNameFilters(const QStringList & nameFilters);
impl<'a> /*trait*/ QDir_setNameFilters for (&'a  QStringList) {
  fn setNameFilters(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn separator<T: QDir_separator>(&mut self, value: T) -> QChar {
    return value.separator(self);
    // return 1;
  }
}

pub trait QDir_separator {
  fn separator(self, rsthis: &mut QDir) -> QChar;
}

// proto: static QChar QDir::separator();
impl<'a> /*trait*/ QDir_separator for () {
  fn separator(self, rsthis: &mut QDir) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9separatorEv()};
    let mut ret = unsafe {_ZN4QDir9separatorEv()};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn swap<T: QDir_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QDir_swap {
  fn swap(self, rsthis: &mut QDir) ;
}

// proto:  void QDir::swap(QDir & other);
impl<'a> /*trait*/ QDir_swap for (&'a mut QDir) {
  fn swap(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn temp<T: QDir_temp>(&mut self, value: T) -> QDir {
    return value.temp(self);
    // return 1;
  }
}

pub trait QDir_temp {
  fn temp(self, rsthis: &mut QDir) -> QDir;
}

// proto: static QDir QDir::temp();
impl<'a> /*trait*/ QDir_temp for () {
  fn temp(self, rsthis: &mut QDir) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4tempEv()};
    let mut ret = unsafe {_ZN4QDir4tempEv()};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QDir::exists(const QString & name);
impl<'a> /*trait*/ QDir_exists for (&'a  QString) {
  fn exists(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir6existsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn mkpath<T: QDir_mkpath>(&mut self, value: T) -> i8 {
    return value.mkpath(self);
    // return 1;
  }
}

pub trait QDir_mkpath {
  fn mkpath(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::mkpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_mkpath for (&'a  QString) {
  fn mkpath(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6mkpathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QDir6mkpathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn addSearchPath<T: QDir_addSearchPath>(&mut self, value: T)  {
     value.addSearchPath(self);
    // return 1;
  }
}

pub trait QDir_addSearchPath {
  fn addSearchPath(self, rsthis: &mut QDir) ;
}

// proto: static void QDir::addSearchPath(const QString & prefix, const QString & path);
impl<'a> /*trait*/ QDir_addSearchPath for (&'a  QString, &'a  QString) {
  fn addSearchPath(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir13addSearchPathERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir13addSearchPathERK7QStringS2_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn dirName<T: QDir_dirName>(&mut self, value: T) -> QString {
    return value.dirName(self);
    // return 1;
  }
}

pub trait QDir_dirName {
  fn dirName(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::dirName();
impl<'a> /*trait*/ QDir_dirName for () {
  fn dirName(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7dirNameEv()};
    let mut ret = unsafe {_ZNK4QDir7dirNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn searchPaths<T: QDir_searchPaths>(&mut self, value: T)  {
     value.searchPaths(self);
    // return 1;
  }
}

pub trait QDir_searchPaths {
  fn searchPaths(self, rsthis: &mut QDir) ;
}

// proto: static QStringList QDir::searchPaths(const QString & prefix);
impl<'a> /*trait*/ QDir_searchPaths for (&'a  QString) {
  fn searchPaths(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11searchPathsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir11searchPathsERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn makeAbsolute<T: QDir_makeAbsolute>(&mut self, value: T) -> i8 {
    return value.makeAbsolute(self);
    // return 1;
  }
}

pub trait QDir_makeAbsolute {
  fn makeAbsolute(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::makeAbsolute();
impl<'a> /*trait*/ QDir_makeAbsolute for () {
  fn makeAbsolute(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir12makeAbsoluteEv()};
    let mut ret = unsafe {_ZN4QDir12makeAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn canonicalPath<T: QDir_canonicalPath>(&mut self, value: T) -> QString {
    return value.canonicalPath(self);
    // return 1;
  }
}

pub trait QDir_canonicalPath {
  fn canonicalPath(self, rsthis: &mut QDir) -> QString;
}

// proto:  QString QDir::canonicalPath();
impl<'a> /*trait*/ QDir_canonicalPath for () {
  fn canonicalPath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir13canonicalPathEv()};
    let mut ret = unsafe {_ZNK4QDir13canonicalPathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isReadable<T: QDir_isReadable>(&mut self, value: T) -> i8 {
    return value.isReadable(self);
    // return 1;
  }
}

pub trait QDir_isReadable {
  fn isReadable(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::isReadable();
impl<'a> /*trait*/ QDir_isReadable for () {
  fn isReadable(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isReadableEv()};
    let mut ret = unsafe {_ZNK4QDir10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRelative<T: QDir_isRelative>(&mut self, value: T) -> i8 {
    return value.isRelative(self);
    // return 1;
  }
}

pub trait QDir_isRelative {
  fn isRelative(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::isRelative();
impl<'a> /*trait*/ QDir_isRelative for () {
  fn isRelative(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isRelativeEv()};
    let mut ret = unsafe {_ZNK4QDir10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn currentPath<T: QDir_currentPath>(&mut self, value: T) -> QString {
    return value.currentPath(self);
    // return 1;
  }
}

pub trait QDir_currentPath {
  fn currentPath(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::currentPath();
impl<'a> /*trait*/ QDir_currentPath for () {
  fn currentPath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11currentPathEv()};
    let mut ret = unsafe {_ZN4QDir11currentPathEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRoot<T: QDir_isRoot>(&mut self, value: T) -> i8 {
    return value.isRoot(self);
    // return 1;
  }
}

pub trait QDir_isRoot {
  fn isRoot(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::isRoot();
impl<'a> /*trait*/ QDir_isRoot for () {
  fn isRoot(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6isRootEv()};
    let mut ret = unsafe {_ZNK4QDir6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn removeRecursively<T: QDir_removeRecursively>(&mut self, value: T) -> i8 {
    return value.removeRecursively(self);
    // return 1;
  }
}

pub trait QDir_removeRecursively {
  fn removeRecursively(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::removeRecursively();
impl<'a> /*trait*/ QDir_removeRecursively for () {
  fn removeRecursively(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir17removeRecursivelyEv()};
    let mut ret = unsafe {_ZN4QDir17removeRecursivelyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isAbsolute<T: QDir_isAbsolute>(&mut self, value: T) -> i8 {
    return value.isAbsolute(self);
    // return 1;
  }
}

pub trait QDir_isAbsolute {
  fn isAbsolute(self, rsthis: &mut QDir) -> i8;
}

// proto:  bool QDir::isAbsolute();
impl<'a> /*trait*/ QDir_isAbsolute for () {
  fn isAbsolute(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isAbsoluteEv()};
    let mut ret = unsafe {_ZNK4QDir10isAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn homePath<T: QDir_homePath>(&mut self, value: T) -> QString {
    return value.homePath(self);
    // return 1;
  }
}

pub trait QDir_homePath {
  fn homePath(self, rsthis: &mut QDir) -> QString;
}

// proto: static QString QDir::homePath();
impl<'a> /*trait*/ QDir_homePath for () {
  fn homePath(self, rsthis: &mut QDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8homePathEv()};
    let mut ret = unsafe {_ZN4QDir8homePathEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QDir::NewQDir(const QDir & );
impl<'a> /*trait*/ QDir_NewQDir for (&'a  QDir) {
  fn NewQDir(self) -> QDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QDirC1ERKS_(qthis, arg0)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setPath<T: QDir_setPath>(&mut self, value: T)  {
     value.setPath(self);
    // return 1;
  }
}

pub trait QDir_setPath {
  fn setPath(self, rsthis: &mut QDir) ;
}

// proto:  void QDir::setPath(const QString & path);
impl<'a> /*trait*/ QDir_setPath for (&'a  QString) {
  fn setPath(self, rsthis: &mut QDir)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir7setPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir7setPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRelativePath<T: QDir_isRelativePath>(&mut self, value: T) -> i8 {
    return value.isRelativePath(self);
    // return 1;
  }
}

pub trait QDir_isRelativePath {
  fn isRelativePath(self, rsthis: &mut QDir) -> i8;
}

// proto: static bool QDir::isRelativePath(const QString & path);
impl<'a> /*trait*/ QDir_isRelativePath for (&'a  QString) {
  fn isRelativePath(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14isRelativePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir14isRelativePathERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

