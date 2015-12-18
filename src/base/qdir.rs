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
  pub fn addResourceSearchPath<RetType, T: QDir_addResourceSearchPath<RetType>>(&mut self, value: T) -> RetType {
    return value.addResourceSearchPath(self);
    // return 1;
  }
}

pub trait QDir_addResourceSearchPath<RetType> {
  fn addResourceSearchPath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static void QDir::addResourceSearchPath(const QString & path);
impl<'a> /*trait*/ QDir_addResourceSearchPath<()> for (&'a  QString) {
  fn addResourceSearchPath(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21addResourceSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir21addResourceSearchPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isAbsolutePath<RetType, T: QDir_isAbsolutePath<RetType>>(&mut self, value: T) -> RetType {
    return value.isAbsolutePath(self);
    // return 1;
  }
}

pub trait QDir_isAbsolutePath<RetType> {
  fn isAbsolutePath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static bool QDir::isAbsolutePath(const QString & path);
impl<'a> /*trait*/ QDir_isAbsolutePath<i8> for (&'a  QString) {
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
  pub fn relativeFilePath<RetType, T: QDir_relativeFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.relativeFilePath(self);
    // return 1;
  }
}

pub trait QDir_relativeFilePath<RetType> {
  fn relativeFilePath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::relativeFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_relativeFilePath<QString> for (&'a  QString) {
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
  pub fn tempPath<RetType, T: QDir_tempPath<RetType>>(&mut self, value: T) -> RetType {
    return value.tempPath(self);
    // return 1;
  }
}

pub trait QDir_tempPath<RetType> {
  fn tempPath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::tempPath();
impl<'a> /*trait*/ QDir_tempPath<QString> for () {
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
  pub fn nameFilters<RetType, T: QDir_nameFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.nameFilters(self);
    // return 1;
  }
}

pub trait QDir_nameFilters<RetType> {
  fn nameFilters(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QStringList QDir::nameFilters();
impl<'a> /*trait*/ QDir_nameFilters<()> for () {
  fn nameFilters(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir11nameFiltersEv()};
     unsafe {_ZNK4QDir11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cd<RetType, T: QDir_cd<RetType>>(&mut self, value: T) -> RetType {
    return value.cd(self);
    // return 1;
  }
}

pub trait QDir_cd<RetType> {
  fn cd(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::cd(const QString & dirName);
impl<'a> /*trait*/ QDir_cd<i8> for (&'a  QString) {
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
  pub fn drives<RetType, T: QDir_drives<RetType>>(&mut self, value: T) -> RetType {
    return value.drives(self);
    // return 1;
  }
}

pub trait QDir_drives<RetType> {
  fn drives(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QList<QFileInfo> QDir::drives();
impl<'a> /*trait*/ QDir_drives<()> for () {
  fn drives(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6drivesEv()};
     unsafe {_ZN4QDir6drivesEv()};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setCurrent<RetType, T: QDir_setCurrent<RetType>>(&mut self, value: T) -> RetType {
    return value.setCurrent(self);
    // return 1;
  }
}

pub trait QDir_setCurrent<RetType> {
  fn setCurrent(self, rsthis: &mut QDir) -> RetType;
}

// proto: static bool QDir::setCurrent(const QString & path);
impl<'a> /*trait*/ QDir_setCurrent<i8> for (&'a  QString) {
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
  pub fn rmdir<RetType, T: QDir_rmdir<RetType>>(&mut self, value: T) -> RetType {
    return value.rmdir(self);
    // return 1;
  }
}

pub trait QDir_rmdir<RetType> {
  fn rmdir(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::rmdir(const QString & dirName);
impl<'a> /*trait*/ QDir_rmdir<i8> for (&'a  QString) {
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
  pub fn cdUp<RetType, T: QDir_cdUp<RetType>>(&mut self, value: T) -> RetType {
    return value.cdUp(self);
    // return 1;
  }
}

pub trait QDir_cdUp<RetType> {
  fn cdUp(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::cdUp();
impl<'a> /*trait*/ QDir_cdUp<i8> for () {
  fn cdUp(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4cdUpEv()};
    let mut ret = unsafe {_ZN4QDir4cdUpEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn absolutePath<RetType, T: QDir_absolutePath<RetType>>(&mut self, value: T) -> RetType {
    return value.absolutePath(self);
    // return 1;
  }
}

pub trait QDir_absolutePath<RetType> {
  fn absolutePath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::absolutePath();
impl<'a> /*trait*/ QDir_absolutePath<QString> for () {
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
  pub fn setSearchPaths<RetType, T: QDir_setSearchPaths<RetType>>(&mut self, value: T) -> RetType {
    return value.setSearchPaths(self);
    // return 1;
  }
}

pub trait QDir_setSearchPaths<RetType> {
  fn setSearchPaths(self, rsthis: &mut QDir) -> RetType;
}

// proto: static void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
impl<'a> /*trait*/ QDir_setSearchPaths<()> for (&'a  QString, &'a  QStringList) {
  fn setSearchPaths(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setSearchPathsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn absoluteFilePath<RetType, T: QDir_absoluteFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QDir_absoluteFilePath<RetType> {
  fn absoluteFilePath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::absoluteFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_absoluteFilePath<QString> for (&'a  QString) {
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
  pub fn rename<RetType, T: QDir_rename<RetType>>(&mut self, value: T) -> RetType {
    return value.rename(self);
    // return 1;
  }
}

pub trait QDir_rename<RetType> {
  fn rename(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QDir_rename<i8> for (&'a  QString, &'a  QString) {
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
  pub fn match_<RetType, T: QDir_match_<RetType>>(&mut self, value: T) -> RetType {
    return value.match_(self);
    // return 1;
  }
}

pub trait QDir_match_<RetType> {
  fn match_(self, rsthis: &mut QDir) -> RetType;
}

// proto: static bool QDir::match_(const QString & filter, const QString & fileName);
impl<'a> /*trait*/ QDir_match_<i8> for (&'a  QString, &'a  QString) {
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
  pub fn refresh<RetType, T: QDir_refresh<RetType>>(&mut self, value: T) -> RetType {
    return value.refresh(self);
    // return 1;
  }
}

pub trait QDir_refresh<RetType> {
  fn refresh(self, rsthis: &mut QDir) -> RetType;
}

// proto:  void QDir::refresh();
impl<'a> /*trait*/ QDir_refresh<()> for () {
  fn refresh(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7refreshEv()};
     unsafe {_ZNK4QDir7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn mkdir<RetType, T: QDir_mkdir<RetType>>(&mut self, value: T) -> RetType {
    return value.mkdir(self);
    // return 1;
  }
}

pub trait QDir_mkdir<RetType> {
  fn mkdir(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::mkdir(const QString & dirName);
impl<'a> /*trait*/ QDir_mkdir<i8> for (&'a  QString) {
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
  pub fn count<RetType, T: QDir_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QDir_count<RetType> {
  fn count(self, rsthis: &mut QDir) -> RetType;
}

// proto:  unsigned int QDir::count();
impl<'a> /*trait*/ QDir_count<u32> for () {
  fn count(self, rsthis: &mut QDir) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5countEv()};
    let mut ret = unsafe {_ZNK4QDir5countEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn root<RetType, T: QDir_root<RetType>>(&mut self, value: T) -> RetType {
    return value.root(self);
    // return 1;
  }
}

pub trait QDir_root<RetType> {
  fn root(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QDir QDir::root();
impl<'a> /*trait*/ QDir_root<QDir> for () {
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
  pub fn nameFiltersFromString<RetType, T: QDir_nameFiltersFromString<RetType>>(&mut self, value: T) -> RetType {
    return value.nameFiltersFromString(self);
    // return 1;
  }
}

pub trait QDir_nameFiltersFromString<RetType> {
  fn nameFiltersFromString(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QStringList QDir::nameFiltersFromString(const QString & nameFilter);
impl<'a> /*trait*/ QDir_nameFiltersFromString<()> for (&'a  QString) {
  fn nameFiltersFromString(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21nameFiltersFromStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir21nameFiltersFromStringERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn filePath<RetType, T: QDir_filePath<RetType>>(&mut self, value: T) -> RetType {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QDir_filePath<RetType> {
  fn filePath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::filePath(const QString & fileName);
impl<'a> /*trait*/ QDir_filePath<QString> for (&'a  QString) {
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
  pub fn rmpath<RetType, T: QDir_rmpath<RetType>>(&mut self, value: T) -> RetType {
    return value.rmpath(self);
    // return 1;
  }
}

pub trait QDir_rmpath<RetType> {
  fn rmpath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::rmpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_rmpath<i8> for (&'a  QString) {
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
  pub fn path<RetType, T: QDir_path<RetType>>(&mut self, value: T) -> RetType {
    return value.path(self);
    // return 1;
  }
}

pub trait QDir_path<RetType> {
  fn path(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::path();
impl<'a> /*trait*/ QDir_path<QString> for () {
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
  pub fn toNativeSeparators<RetType, T: QDir_toNativeSeparators<RetType>>(&mut self, value: T) -> RetType {
    return value.toNativeSeparators(self);
    // return 1;
  }
}

pub trait QDir_toNativeSeparators<RetType> {
  fn toNativeSeparators(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::toNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_toNativeSeparators<QString> for (&'a  QString) {
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
  pub fn cleanPath<RetType, T: QDir_cleanPath<RetType>>(&mut self, value: T) -> RetType {
    return value.cleanPath(self);
    // return 1;
  }
}

pub trait QDir_cleanPath<RetType> {
  fn cleanPath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::cleanPath(const QString & path);
impl<'a> /*trait*/ QDir_cleanPath<QString> for (&'a  QString) {
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
  pub fn exists<RetType, T: QDir_exists<RetType>>(&mut self, value: T) -> RetType {
    return value.exists(self);
    // return 1;
  }
}

pub trait QDir_exists<RetType> {
  fn exists(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::exists();
impl<'a> /*trait*/ QDir_exists<i8> for () {
  fn exists(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsEv()};
    let mut ret = unsafe {_ZNK4QDir6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn remove<RetType, T: QDir_remove<RetType>>(&mut self, value: T) -> RetType {
    return value.remove(self);
    // return 1;
  }
}

pub trait QDir_remove<RetType> {
  fn remove(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::remove(const QString & fileName);
impl<'a> /*trait*/ QDir_remove<i8> for (&'a  QString) {
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
  pub fn FreeQDir<RetType, T: QDir_FreeQDir<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDir(self);
    // return 1;
  }
}

pub trait QDir_FreeQDir<RetType> {
  fn FreeQDir(self, rsthis: &mut QDir) -> RetType;
}

// proto:  void QDir::FreeQDir();
impl<'a> /*trait*/ QDir_FreeQDir<()> for () {
  fn FreeQDir(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirD0Ev()};
     unsafe {_ZN4QDirD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rootPath<RetType, T: QDir_rootPath<RetType>>(&mut self, value: T) -> RetType {
    return value.rootPath(self);
    // return 1;
  }
}

pub trait QDir_rootPath<RetType> {
  fn rootPath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::rootPath();
impl<'a> /*trait*/ QDir_rootPath<QString> for () {
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
  pub fn current<RetType, T: QDir_current<RetType>>(&mut self, value: T) -> RetType {
    return value.current(self);
    // return 1;
  }
}

pub trait QDir_current<RetType> {
  fn current(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QDir QDir::current();
impl<'a> /*trait*/ QDir_current<QDir> for () {
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
impl<'a> /*trait*/ QDir_match_<i8> for (&'a  QStringList, &'a  QString) {
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
  pub fn fromNativeSeparators<RetType, T: QDir_fromNativeSeparators<RetType>>(&mut self, value: T) -> RetType {
    return value.fromNativeSeparators(self);
    // return 1;
  }
}

pub trait QDir_fromNativeSeparators<RetType> {
  fn fromNativeSeparators(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::fromNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_fromNativeSeparators<QString> for (&'a  QString) {
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
  pub fn home<RetType, T: QDir_home<RetType>>(&mut self, value: T) -> RetType {
    return value.home(self);
    // return 1;
  }
}

pub trait QDir_home<RetType> {
  fn home(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QDir QDir::home();
impl<'a> /*trait*/ QDir_home<QDir> for () {
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
  pub fn setNameFilters<RetType, T: QDir_setNameFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.setNameFilters(self);
    // return 1;
  }
}

pub trait QDir_setNameFilters<RetType> {
  fn setNameFilters(self, rsthis: &mut QDir) -> RetType;
}

// proto:  void QDir::setNameFilters(const QStringList & nameFilters);
impl<'a> /*trait*/ QDir_setNameFilters<()> for (&'a  QStringList) {
  fn setNameFilters(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn separator<RetType, T: QDir_separator<RetType>>(&mut self, value: T) -> RetType {
    return value.separator(self);
    // return 1;
  }
}

pub trait QDir_separator<RetType> {
  fn separator(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QChar QDir::separator();
impl<'a> /*trait*/ QDir_separator<QChar> for () {
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
  pub fn swap<RetType, T: QDir_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QDir_swap<RetType> {
  fn swap(self, rsthis: &mut QDir) -> RetType;
}

// proto:  void QDir::swap(QDir & other);
impl<'a> /*trait*/ QDir_swap<()> for (&'a mut QDir) {
  fn swap(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn temp<RetType, T: QDir_temp<RetType>>(&mut self, value: T) -> RetType {
    return value.temp(self);
    // return 1;
  }
}

pub trait QDir_temp<RetType> {
  fn temp(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QDir QDir::temp();
impl<'a> /*trait*/ QDir_temp<QDir> for () {
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
impl<'a> /*trait*/ QDir_exists<i8> for (&'a  QString) {
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
  pub fn mkpath<RetType, T: QDir_mkpath<RetType>>(&mut self, value: T) -> RetType {
    return value.mkpath(self);
    // return 1;
  }
}

pub trait QDir_mkpath<RetType> {
  fn mkpath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::mkpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_mkpath<i8> for (&'a  QString) {
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
  pub fn addSearchPath<RetType, T: QDir_addSearchPath<RetType>>(&mut self, value: T) -> RetType {
    return value.addSearchPath(self);
    // return 1;
  }
}

pub trait QDir_addSearchPath<RetType> {
  fn addSearchPath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static void QDir::addSearchPath(const QString & prefix, const QString & path);
impl<'a> /*trait*/ QDir_addSearchPath<()> for (&'a  QString, &'a  QString) {
  fn addSearchPath(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir13addSearchPathERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir13addSearchPathERK7QStringS2_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn dirName<RetType, T: QDir_dirName<RetType>>(&mut self, value: T) -> RetType {
    return value.dirName(self);
    // return 1;
  }
}

pub trait QDir_dirName<RetType> {
  fn dirName(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::dirName();
impl<'a> /*trait*/ QDir_dirName<QString> for () {
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
  pub fn searchPaths<RetType, T: QDir_searchPaths<RetType>>(&mut self, value: T) -> RetType {
    return value.searchPaths(self);
    // return 1;
  }
}

pub trait QDir_searchPaths<RetType> {
  fn searchPaths(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QStringList QDir::searchPaths(const QString & prefix);
impl<'a> /*trait*/ QDir_searchPaths<()> for (&'a  QString) {
  fn searchPaths(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11searchPathsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir11searchPathsERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn makeAbsolute<RetType, T: QDir_makeAbsolute<RetType>>(&mut self, value: T) -> RetType {
    return value.makeAbsolute(self);
    // return 1;
  }
}

pub trait QDir_makeAbsolute<RetType> {
  fn makeAbsolute(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::makeAbsolute();
impl<'a> /*trait*/ QDir_makeAbsolute<i8> for () {
  fn makeAbsolute(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir12makeAbsoluteEv()};
    let mut ret = unsafe {_ZN4QDir12makeAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn canonicalPath<RetType, T: QDir_canonicalPath<RetType>>(&mut self, value: T) -> RetType {
    return value.canonicalPath(self);
    // return 1;
  }
}

pub trait QDir_canonicalPath<RetType> {
  fn canonicalPath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  QString QDir::canonicalPath();
impl<'a> /*trait*/ QDir_canonicalPath<QString> for () {
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
  pub fn isReadable<RetType, T: QDir_isReadable<RetType>>(&mut self, value: T) -> RetType {
    return value.isReadable(self);
    // return 1;
  }
}

pub trait QDir_isReadable<RetType> {
  fn isReadable(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::isReadable();
impl<'a> /*trait*/ QDir_isReadable<i8> for () {
  fn isReadable(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isReadableEv()};
    let mut ret = unsafe {_ZNK4QDir10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRelative<RetType, T: QDir_isRelative<RetType>>(&mut self, value: T) -> RetType {
    return value.isRelative(self);
    // return 1;
  }
}

pub trait QDir_isRelative<RetType> {
  fn isRelative(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::isRelative();
impl<'a> /*trait*/ QDir_isRelative<i8> for () {
  fn isRelative(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isRelativeEv()};
    let mut ret = unsafe {_ZNK4QDir10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn currentPath<RetType, T: QDir_currentPath<RetType>>(&mut self, value: T) -> RetType {
    return value.currentPath(self);
    // return 1;
  }
}

pub trait QDir_currentPath<RetType> {
  fn currentPath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::currentPath();
impl<'a> /*trait*/ QDir_currentPath<QString> for () {
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
  pub fn isRoot<RetType, T: QDir_isRoot<RetType>>(&mut self, value: T) -> RetType {
    return value.isRoot(self);
    // return 1;
  }
}

pub trait QDir_isRoot<RetType> {
  fn isRoot(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::isRoot();
impl<'a> /*trait*/ QDir_isRoot<i8> for () {
  fn isRoot(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6isRootEv()};
    let mut ret = unsafe {_ZNK4QDir6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn removeRecursively<RetType, T: QDir_removeRecursively<RetType>>(&mut self, value: T) -> RetType {
    return value.removeRecursively(self);
    // return 1;
  }
}

pub trait QDir_removeRecursively<RetType> {
  fn removeRecursively(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::removeRecursively();
impl<'a> /*trait*/ QDir_removeRecursively<i8> for () {
  fn removeRecursively(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir17removeRecursivelyEv()};
    let mut ret = unsafe {_ZN4QDir17removeRecursivelyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isAbsolute<RetType, T: QDir_isAbsolute<RetType>>(&mut self, value: T) -> RetType {
    return value.isAbsolute(self);
    // return 1;
  }
}

pub trait QDir_isAbsolute<RetType> {
  fn isAbsolute(self, rsthis: &mut QDir) -> RetType;
}

// proto:  bool QDir::isAbsolute();
impl<'a> /*trait*/ QDir_isAbsolute<i8> for () {
  fn isAbsolute(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isAbsoluteEv()};
    let mut ret = unsafe {_ZNK4QDir10isAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn homePath<RetType, T: QDir_homePath<RetType>>(&mut self, value: T) -> RetType {
    return value.homePath(self);
    // return 1;
  }
}

pub trait QDir_homePath<RetType> {
  fn homePath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static QString QDir::homePath();
impl<'a> /*trait*/ QDir_homePath<QString> for () {
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
  pub fn setPath<RetType, T: QDir_setPath<RetType>>(&mut self, value: T) -> RetType {
    return value.setPath(self);
    // return 1;
  }
}

pub trait QDir_setPath<RetType> {
  fn setPath(self, rsthis: &mut QDir) -> RetType;
}

// proto:  void QDir::setPath(const QString & path);
impl<'a> /*trait*/ QDir_setPath<()> for (&'a  QString) {
  fn setPath(self, rsthis: &mut QDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir7setPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QDir7setPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRelativePath<RetType, T: QDir_isRelativePath<RetType>>(&mut self, value: T) -> RetType {
    return value.isRelativePath(self);
    // return 1;
  }
}

pub trait QDir_isRelativePath<RetType> {
  fn isRelativePath(self, rsthis: &mut QDir) -> RetType;
}

// proto: static bool QDir::isRelativePath(const QString & path);
impl<'a> /*trait*/ QDir_isRelativePath<i8> for (&'a  QString) {
  fn isRelativePath(self, rsthis: &mut QDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14isRelativePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QDir14isRelativePathERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

