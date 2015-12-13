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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN4QDir21addResourceSearchPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir14isAbsolutePathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir16relativeFilePathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir8tempPathEv() -> i32;
  fn _ZN4QDirC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK4QDir11nameFiltersEv() -> i32;
  fn _ZN4QDir2cdERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir6drivesEv() -> i32;
  fn _ZN4QDir10setCurrentERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir5rmdirERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir4cdUpEv() -> i32;
  fn _ZNK4QDir12absolutePathEv() -> i32;
  fn _ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK4QDir16absoluteFilePathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDirC1ERK7QStringS2_6QFlagsINS_8SortFlagEES3_INS_6FilterEE(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN4QDir6renameERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN4QDir5matchERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK4QDir7refreshEv() -> i32;
  fn _ZNK4QDir5mkdirERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir5countEv() -> i32;
  fn _ZN4QDir4rootEv() -> i32;
  fn _ZN4QDir21nameFiltersFromStringERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir8filePathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir6rmpathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir4pathEv() -> i32;
  fn _ZN4QDir18toNativeSeparatorsERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir9cleanPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir6existsEv() -> i32;
  fn _ZN4QDir6removeERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDirD0Ev() -> i32;
  fn _ZNK4QDir13entryInfoListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  fn _ZN4QDir8rootPathEv() -> i32;
  fn _ZN4QDir7currentEv() -> i32;
  fn _ZN4QDir5matchERK11QStringListRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN4QDir20fromNativeSeparatorsERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir4homeEv() -> i32;
  fn _ZN4QDir10setSortingE6QFlagsINS_8SortFlagEE(arg0: c_int) -> i32;
  fn _ZN4QDir14setNameFiltersERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN4QDir9separatorEv() -> i32;
  fn _ZN4QDir4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN4QDir4tempEv() -> i32;
  fn _ZNK4QDir6existsERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QDir9entryListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  fn _ZNK4QDir6mkpathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir13addSearchPathERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK4QDir7dirNameEv() -> i32;
  fn _ZN4QDir11searchPathsERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir12makeAbsoluteEv() -> i32;
  fn _ZNK4QDir13entryInfoListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN4QDir9setFilterE6QFlagsINS_6FilterEE(arg0: c_int) -> i32;
  fn _ZNK4QDir13canonicalPathEv() -> i32;
  fn _ZNK4QDir10isReadableEv() -> i32;
  fn _ZNK4QDir10isRelativeEv() -> i32;
  fn _ZN4QDir11currentPathEv() -> i32;
  fn _ZNK4QDir6isRootEv() -> i32;
  fn _ZN4QDir17removeRecursivelyEv() -> i32;
  fn _ZNK4QDir9entryListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK4QDir10isAbsoluteEv() -> i32;
  fn _ZN4QDir8homePathEv() -> i32;
  fn _ZN4QDirC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN4QDir7setPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QDir14isRelativePathERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QDir)=1
pub struct QDir {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDir {
  pub fn addResourceSearchPath<T: QDir_addResourceSearchPath>(&mut self, value: T) -> i32 {
    value.addResourceSearchPath(self);
    return 1;
  }
}

pub trait QDir_addResourceSearchPath {
  fn addResourceSearchPath(self, this: &mut QDir) -> i32;
}

// proto: void QDir::addResourceSearchPath(const QString & path);
impl<'a> /*trait*/ QDir_addResourceSearchPath for (&'a  QString) {
  fn addResourceSearchPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21addResourceSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir21addResourceSearchPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isAbsolutePath<T: QDir_isAbsolutePath>(&mut self, value: T) -> i32 {
    value.isAbsolutePath(self);
    return 1;
  }
}

pub trait QDir_isAbsolutePath {
  fn isAbsolutePath(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::isAbsolutePath(const QString & path);
impl<'a> /*trait*/ QDir_isAbsolutePath for (&'a  QString) {
  fn isAbsolutePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14isAbsolutePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir14isAbsolutePathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn relativeFilePath<T: QDir_relativeFilePath>(&mut self, value: T) -> i32 {
    value.relativeFilePath(self);
    return 1;
  }
}

pub trait QDir_relativeFilePath {
  fn relativeFilePath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::relativeFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_relativeFilePath for (&'a  QString) {
  fn relativeFilePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir16relativeFilePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir16relativeFilePathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn tempPath<T: QDir_tempPath>(&mut self, value: T) -> i32 {
    value.tempPath(self);
    return 1;
  }
}

pub trait QDir_tempPath {
  fn tempPath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::tempPath();
impl<'a> /*trait*/ QDir_tempPath for () {
  fn tempPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8tempPathEv()};
    unsafe {_ZN4QDir8tempPathEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDirC1ERK7QString(qthis, arg0)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn nameFilters<T: QDir_nameFilters>(&mut self, value: T) -> i32 {
    value.nameFilters(self);
    return 1;
  }
}

pub trait QDir_nameFilters {
  fn nameFilters(self, this: &mut QDir) -> i32;
}

// proto: QStringList QDir::nameFilters();
impl<'a> /*trait*/ QDir_nameFilters for () {
  fn nameFilters(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir11nameFiltersEv()};
    unsafe {_ZNK4QDir11nameFiltersEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cd<T: QDir_cd>(&mut self, value: T) -> i32 {
    value.cd(self);
    return 1;
  }
}

pub trait QDir_cd {
  fn cd(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::cd(const QString & dirName);
impl<'a> /*trait*/ QDir_cd for (&'a  QString) {
  fn cd(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir2cdERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir2cdERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn drives<T: QDir_drives>(&mut self, value: T) -> i32 {
    value.drives(self);
    return 1;
  }
}

pub trait QDir_drives {
  fn drives(self, this: &mut QDir) -> i32;
}

// proto: QList<QFileInfo> QDir::drives();
impl<'a> /*trait*/ QDir_drives for () {
  fn drives(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6drivesEv()};
    unsafe {_ZN4QDir6drivesEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setCurrent<T: QDir_setCurrent>(&mut self, value: T) -> i32 {
    value.setCurrent(self);
    return 1;
  }
}

pub trait QDir_setCurrent {
  fn setCurrent(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::setCurrent(const QString & path);
impl<'a> /*trait*/ QDir_setCurrent for (&'a  QString) {
  fn setCurrent(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir10setCurrentERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir10setCurrentERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rmdir<T: QDir_rmdir>(&mut self, value: T) -> i32 {
    value.rmdir(self);
    return 1;
  }
}

pub trait QDir_rmdir {
  fn rmdir(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::rmdir(const QString & dirName);
impl<'a> /*trait*/ QDir_rmdir for (&'a  QString) {
  fn rmdir(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5rmdirERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir5rmdirERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cdUp<T: QDir_cdUp>(&mut self, value: T) -> i32 {
    value.cdUp(self);
    return 1;
  }
}

pub trait QDir_cdUp {
  fn cdUp(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::cdUp();
impl<'a> /*trait*/ QDir_cdUp for () {
  fn cdUp(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4cdUpEv()};
    unsafe {_ZN4QDir4cdUpEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn absolutePath<T: QDir_absolutePath>(&mut self, value: T) -> i32 {
    value.absolutePath(self);
    return 1;
  }
}

pub trait QDir_absolutePath {
  fn absolutePath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::absolutePath();
impl<'a> /*trait*/ QDir_absolutePath for () {
  fn absolutePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir12absolutePathEv()};
    unsafe {_ZNK4QDir12absolutePathEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setSearchPaths<T: QDir_setSearchPaths>(&mut self, value: T) -> i32 {
    value.setSearchPaths(self);
    return 1;
  }
}

pub trait QDir_setSearchPaths {
  fn setSearchPaths(self, this: &mut QDir) -> i32;
}

// proto: void QDir::setSearchPaths(const QString & prefix, const QStringList & searchPaths);
impl<'a> /*trait*/ QDir_setSearchPaths for (&'a  QString, &'a  QStringList) {
  fn setSearchPaths(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setSearchPathsERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN4QDir14setSearchPathsERK7QStringRK11QStringList(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn absoluteFilePath<T: QDir_absoluteFilePath>(&mut self, value: T) -> i32 {
    value.absoluteFilePath(self);
    return 1;
  }
}

pub trait QDir_absoluteFilePath {
  fn absoluteFilePath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::absoluteFilePath(const QString & fileName);
impl<'a> /*trait*/ QDir_absoluteFilePath for (&'a  QString) {
  fn absoluteFilePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir16absoluteFilePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir16absoluteFilePathERK7QString(arg0)};
    return 1;
  }
}

// proto: void QDir::NewQDir(const QString & path, const QString & nameFilter, SortFlags sort, Filters filter);
impl<'a> /*trait*/ QDir_NewQDir for (&'a  QString, &'a  QString, i32, i32) {
  fn NewQDir(self) -> QDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirC1ERK7QStringS2_6QFlagsINS_8SortFlagEES3_INS_6FilterEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN4QDirC1ERK7QStringS2_6QFlagsINS_8SortFlagEES3_INS_6FilterEE(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rename<T: QDir_rename>(&mut self, value: T) -> i32 {
    value.rename(self);
    return 1;
  }
}

pub trait QDir_rename {
  fn rename(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QDir_rename for (&'a  QString, &'a  QString) {
  fn rename(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6renameERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN4QDir6renameERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn match_<T: QDir_match_>(&mut self, value: T) -> i32 {
    value.match_(self);
    return 1;
  }
}

pub trait QDir_match_ {
  fn match_(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::match_(const QString & filter, const QString & fileName);
impl<'a> /*trait*/ QDir_match_ for (&'a  QString, &'a  QString) {
  fn match_(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir5matchERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN4QDir5matchERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn refresh<T: QDir_refresh>(&mut self, value: T) -> i32 {
    value.refresh(self);
    return 1;
  }
}

pub trait QDir_refresh {
  fn refresh(self, this: &mut QDir) -> i32;
}

// proto: void QDir::refresh();
impl<'a> /*trait*/ QDir_refresh for () {
  fn refresh(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7refreshEv()};
    unsafe {_ZNK4QDir7refreshEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn mkdir<T: QDir_mkdir>(&mut self, value: T) -> i32 {
    value.mkdir(self);
    return 1;
  }
}

pub trait QDir_mkdir {
  fn mkdir(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::mkdir(const QString & dirName);
impl<'a> /*trait*/ QDir_mkdir for (&'a  QString) {
  fn mkdir(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5mkdirERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir5mkdirERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn count<T: QDir_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QDir_count {
  fn count(self, this: &mut QDir) -> i32;
}

// proto: unsigned int QDir::count();
impl<'a> /*trait*/ QDir_count for () {
  fn count(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir5countEv()};
    unsafe {_ZNK4QDir5countEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn root<T: QDir_root>(&mut self, value: T) -> i32 {
    value.root(self);
    return 1;
  }
}

pub trait QDir_root {
  fn root(self, this: &mut QDir) -> i32;
}

// proto: QDir QDir::root();
impl<'a> /*trait*/ QDir_root for () {
  fn root(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4rootEv()};
    unsafe {_ZN4QDir4rootEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn nameFiltersFromString<T: QDir_nameFiltersFromString>(&mut self, value: T) -> i32 {
    value.nameFiltersFromString(self);
    return 1;
  }
}

pub trait QDir_nameFiltersFromString {
  fn nameFiltersFromString(self, this: &mut QDir) -> i32;
}

// proto: QStringList QDir::nameFiltersFromString(const QString & nameFilter);
impl<'a> /*trait*/ QDir_nameFiltersFromString for (&'a  QString) {
  fn nameFiltersFromString(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir21nameFiltersFromStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir21nameFiltersFromStringERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn filePath<T: QDir_filePath>(&mut self, value: T) -> i32 {
    value.filePath(self);
    return 1;
  }
}

pub trait QDir_filePath {
  fn filePath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::filePath(const QString & fileName);
impl<'a> /*trait*/ QDir_filePath for (&'a  QString) {
  fn filePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir8filePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir8filePathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rmpath<T: QDir_rmpath>(&mut self, value: T) -> i32 {
    value.rmpath(self);
    return 1;
  }
}

pub trait QDir_rmpath {
  fn rmpath(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::rmpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_rmpath for (&'a  QString) {
  fn rmpath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6rmpathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir6rmpathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn path<T: QDir_path>(&mut self, value: T) -> i32 {
    value.path(self);
    return 1;
  }
}

pub trait QDir_path {
  fn path(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::path();
impl<'a> /*trait*/ QDir_path for () {
  fn path(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir4pathEv()};
    unsafe {_ZNK4QDir4pathEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn toNativeSeparators<T: QDir_toNativeSeparators>(&mut self, value: T) -> i32 {
    value.toNativeSeparators(self);
    return 1;
  }
}

pub trait QDir_toNativeSeparators {
  fn toNativeSeparators(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::toNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_toNativeSeparators for (&'a  QString) {
  fn toNativeSeparators(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir18toNativeSeparatorsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir18toNativeSeparatorsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn cleanPath<T: QDir_cleanPath>(&mut self, value: T) -> i32 {
    value.cleanPath(self);
    return 1;
  }
}

pub trait QDir_cleanPath {
  fn cleanPath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::cleanPath(const QString & path);
impl<'a> /*trait*/ QDir_cleanPath for (&'a  QString) {
  fn cleanPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9cleanPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir9cleanPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn exists<T: QDir_exists>(&mut self, value: T) -> i32 {
    value.exists(self);
    return 1;
  }
}

pub trait QDir_exists {
  fn exists(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::exists();
impl<'a> /*trait*/ QDir_exists for () {
  fn exists(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsEv()};
    unsafe {_ZNK4QDir6existsEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn remove<T: QDir_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QDir_remove {
  fn remove(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::remove(const QString & fileName);
impl<'a> /*trait*/ QDir_remove for (&'a  QString) {
  fn remove(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir6removeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir6removeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn FreeQDir<T: QDir_FreeQDir>(&mut self, value: T) -> i32 {
    value.FreeQDir(self);
    return 1;
  }
}

pub trait QDir_FreeQDir {
  fn FreeQDir(self, this: &mut QDir) -> i32;
}

// proto: void QDir::FreeQDir();
impl<'a> /*trait*/ QDir_FreeQDir for () {
  fn FreeQDir(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirD0Ev()};
    unsafe {_ZN4QDirD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn entryInfoList<T: QDir_entryInfoList>(&mut self, value: T) -> i32 {
    value.entryInfoList(self);
    return 1;
  }
}

pub trait QDir_entryInfoList {
  fn entryInfoList(self, this: &mut QDir) -> i32;
}

// proto: QList<QFileInfo> QDir::entryInfoList(const QStringList & nameFilters, Filters filters, SortFlags sort);
impl<'a> /*trait*/ QDir_entryInfoList for (&'a  QStringList, i32, i32) {
  fn entryInfoList(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir13entryInfoListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK4QDir13entryInfoListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn rootPath<T: QDir_rootPath>(&mut self, value: T) -> i32 {
    value.rootPath(self);
    return 1;
  }
}

pub trait QDir_rootPath {
  fn rootPath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::rootPath();
impl<'a> /*trait*/ QDir_rootPath for () {
  fn rootPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8rootPathEv()};
    unsafe {_ZN4QDir8rootPathEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn current<T: QDir_current>(&mut self, value: T) -> i32 {
    value.current(self);
    return 1;
  }
}

pub trait QDir_current {
  fn current(self, this: &mut QDir) -> i32;
}

// proto: QDir QDir::current();
impl<'a> /*trait*/ QDir_current for () {
  fn current(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir7currentEv()};
    unsafe {_ZN4QDir7currentEv()};
    return 1;
  }
}

// proto: bool QDir::match_(const QStringList & filters, const QString & fileName);
impl<'a> /*trait*/ QDir_match_ for (&'a  QStringList, &'a  QString) {
  fn match_(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir5matchERK11QStringListRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN4QDir5matchERK11QStringListRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn fromNativeSeparators<T: QDir_fromNativeSeparators>(&mut self, value: T) -> i32 {
    value.fromNativeSeparators(self);
    return 1;
  }
}

pub trait QDir_fromNativeSeparators {
  fn fromNativeSeparators(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::fromNativeSeparators(const QString & pathName);
impl<'a> /*trait*/ QDir_fromNativeSeparators for (&'a  QString) {
  fn fromNativeSeparators(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir20fromNativeSeparatorsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir20fromNativeSeparatorsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn home<T: QDir_home>(&mut self, value: T) -> i32 {
    value.home(self);
    return 1;
  }
}

pub trait QDir_home {
  fn home(self, this: &mut QDir) -> i32;
}

// proto: QDir QDir::home();
impl<'a> /*trait*/ QDir_home for () {
  fn home(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4homeEv()};
    unsafe {_ZN4QDir4homeEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setSorting<T: QDir_setSorting>(&mut self, value: T) -> i32 {
    value.setSorting(self);
    return 1;
  }
}

pub trait QDir_setSorting {
  fn setSorting(self, this: &mut QDir) -> i32;
}

// proto: void QDir::setSorting(SortFlags sort);
impl<'a> /*trait*/ QDir_setSorting for (i32) {
  fn setSorting(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir10setSortingE6QFlagsINS_8SortFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN4QDir10setSortingE6QFlagsINS_8SortFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setNameFilters<T: QDir_setNameFilters>(&mut self, value: T) -> i32 {
    value.setNameFilters(self);
    return 1;
  }
}

pub trait QDir_setNameFilters {
  fn setNameFilters(self, this: &mut QDir) -> i32;
}

// proto: void QDir::setNameFilters(const QStringList & nameFilters);
impl<'a> /*trait*/ QDir_setNameFilters for (&'a  QStringList) {
  fn setNameFilters(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir14setNameFiltersERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn separator<T: QDir_separator>(&mut self, value: T) -> i32 {
    value.separator(self);
    return 1;
  }
}

pub trait QDir_separator {
  fn separator(self, this: &mut QDir) -> i32;
}

// proto: QChar QDir::separator();
impl<'a> /*trait*/ QDir_separator for () {
  fn separator(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9separatorEv()};
    unsafe {_ZN4QDir9separatorEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn swap<T: QDir_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QDir_swap {
  fn swap(self, this: &mut QDir) -> i32;
}

// proto: void QDir::swap(QDir & other);
impl<'a> /*trait*/ QDir_swap for (&'a mut QDir) {
  fn swap(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QDir4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn temp<T: QDir_temp>(&mut self, value: T) -> i32 {
    value.temp(self);
    return 1;
  }
}

pub trait QDir_temp {
  fn temp(self, this: &mut QDir) -> i32;
}

// proto: QDir QDir::temp();
impl<'a> /*trait*/ QDir_temp for () {
  fn temp(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir4tempEv()};
    unsafe {_ZN4QDir4tempEv()};
    return 1;
  }
}

// proto: bool QDir::exists(const QString & name);
impl<'a> /*trait*/ QDir_exists for (&'a  QString) {
  fn exists(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6existsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir6existsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn entryList<T: QDir_entryList>(&mut self, value: T) -> i32 {
    value.entryList(self);
    return 1;
  }
}

pub trait QDir_entryList {
  fn entryList(self, this: &mut QDir) -> i32;
}

// proto: QStringList QDir::entryList(const QStringList & nameFilters, Filters filters, SortFlags sort);
impl<'a> /*trait*/ QDir_entryList for (&'a  QStringList, i32, i32) {
  fn entryList(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir9entryListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK4QDir9entryListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn mkpath<T: QDir_mkpath>(&mut self, value: T) -> i32 {
    value.mkpath(self);
    return 1;
  }
}

pub trait QDir_mkpath {
  fn mkpath(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::mkpath(const QString & dirPath);
impl<'a> /*trait*/ QDir_mkpath for (&'a  QString) {
  fn mkpath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6mkpathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QDir6mkpathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn addSearchPath<T: QDir_addSearchPath>(&mut self, value: T) -> i32 {
    value.addSearchPath(self);
    return 1;
  }
}

pub trait QDir_addSearchPath {
  fn addSearchPath(self, this: &mut QDir) -> i32;
}

// proto: void QDir::addSearchPath(const QString & prefix, const QString & path);
impl<'a> /*trait*/ QDir_addSearchPath for (&'a  QString, &'a  QString) {
  fn addSearchPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir13addSearchPathERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN4QDir13addSearchPathERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn dirName<T: QDir_dirName>(&mut self, value: T) -> i32 {
    value.dirName(self);
    return 1;
  }
}

pub trait QDir_dirName {
  fn dirName(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::dirName();
impl<'a> /*trait*/ QDir_dirName for () {
  fn dirName(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir7dirNameEv()};
    unsafe {_ZNK4QDir7dirNameEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn searchPaths<T: QDir_searchPaths>(&mut self, value: T) -> i32 {
    value.searchPaths(self);
    return 1;
  }
}

pub trait QDir_searchPaths {
  fn searchPaths(self, this: &mut QDir) -> i32;
}

// proto: QStringList QDir::searchPaths(const QString & prefix);
impl<'a> /*trait*/ QDir_searchPaths for (&'a  QString) {
  fn searchPaths(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11searchPathsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir11searchPathsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn makeAbsolute<T: QDir_makeAbsolute>(&mut self, value: T) -> i32 {
    value.makeAbsolute(self);
    return 1;
  }
}

pub trait QDir_makeAbsolute {
  fn makeAbsolute(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::makeAbsolute();
impl<'a> /*trait*/ QDir_makeAbsolute for () {
  fn makeAbsolute(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir12makeAbsoluteEv()};
    unsafe {_ZN4QDir12makeAbsoluteEv()};
    return 1;
  }
}

// proto: QList<QFileInfo> QDir::entryInfoList(Filters filters, SortFlags sort);
impl<'a> /*trait*/ QDir_entryInfoList for (i32, i32) {
  fn entryInfoList(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir13entryInfoListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK4QDir13entryInfoListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setFilter<T: QDir_setFilter>(&mut self, value: T) -> i32 {
    value.setFilter(self);
    return 1;
  }
}

pub trait QDir_setFilter {
  fn setFilter(self, this: &mut QDir) -> i32;
}

// proto: void QDir::setFilter(Filters filter);
impl<'a> /*trait*/ QDir_setFilter for (i32) {
  fn setFilter(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir9setFilterE6QFlagsINS_6FilterEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN4QDir9setFilterE6QFlagsINS_6FilterEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn canonicalPath<T: QDir_canonicalPath>(&mut self, value: T) -> i32 {
    value.canonicalPath(self);
    return 1;
  }
}

pub trait QDir_canonicalPath {
  fn canonicalPath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::canonicalPath();
impl<'a> /*trait*/ QDir_canonicalPath for () {
  fn canonicalPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir13canonicalPathEv()};
    unsafe {_ZNK4QDir13canonicalPathEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isReadable<T: QDir_isReadable>(&mut self, value: T) -> i32 {
    value.isReadable(self);
    return 1;
  }
}

pub trait QDir_isReadable {
  fn isReadable(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::isReadable();
impl<'a> /*trait*/ QDir_isReadable for () {
  fn isReadable(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isReadableEv()};
    unsafe {_ZNK4QDir10isReadableEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRelative<T: QDir_isRelative>(&mut self, value: T) -> i32 {
    value.isRelative(self);
    return 1;
  }
}

pub trait QDir_isRelative {
  fn isRelative(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::isRelative();
impl<'a> /*trait*/ QDir_isRelative for () {
  fn isRelative(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isRelativeEv()};
    unsafe {_ZNK4QDir10isRelativeEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn currentPath<T: QDir_currentPath>(&mut self, value: T) -> i32 {
    value.currentPath(self);
    return 1;
  }
}

pub trait QDir_currentPath {
  fn currentPath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::currentPath();
impl<'a> /*trait*/ QDir_currentPath for () {
  fn currentPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir11currentPathEv()};
    unsafe {_ZN4QDir11currentPathEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRoot<T: QDir_isRoot>(&mut self, value: T) -> i32 {
    value.isRoot(self);
    return 1;
  }
}

pub trait QDir_isRoot {
  fn isRoot(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::isRoot();
impl<'a> /*trait*/ QDir_isRoot for () {
  fn isRoot(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir6isRootEv()};
    unsafe {_ZNK4QDir6isRootEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn removeRecursively<T: QDir_removeRecursively>(&mut self, value: T) -> i32 {
    value.removeRecursively(self);
    return 1;
  }
}

pub trait QDir_removeRecursively {
  fn removeRecursively(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::removeRecursively();
impl<'a> /*trait*/ QDir_removeRecursively for () {
  fn removeRecursively(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir17removeRecursivelyEv()};
    unsafe {_ZN4QDir17removeRecursivelyEv()};
    return 1;
  }
}

// proto: QStringList QDir::entryList(Filters filters, SortFlags sort);
impl<'a> /*trait*/ QDir_entryList for (i32, i32) {
  fn entryList(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir9entryListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK4QDir9entryListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isAbsolute<T: QDir_isAbsolute>(&mut self, value: T) -> i32 {
    value.isAbsolute(self);
    return 1;
  }
}

pub trait QDir_isAbsolute {
  fn isAbsolute(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::isAbsolute();
impl<'a> /*trait*/ QDir_isAbsolute for () {
  fn isAbsolute(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QDir10isAbsoluteEv()};
    unsafe {_ZNK4QDir10isAbsoluteEv()};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn homePath<T: QDir_homePath>(&mut self, value: T) -> i32 {
    value.homePath(self);
    return 1;
  }
}

pub trait QDir_homePath {
  fn homePath(self, this: &mut QDir) -> i32;
}

// proto: QString QDir::homePath();
impl<'a> /*trait*/ QDir_homePath for () {
  fn homePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir8homePathEv()};
    unsafe {_ZN4QDir8homePathEv()};
    return 1;
  }
}

// proto: void QDir::NewQDir(const QDir & );
impl<'a> /*trait*/ QDir_NewQDir for (&'a  QDir) {
  fn NewQDir(self) -> QDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDirC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDirC1ERKS_(qthis, arg0)};
    let rsthis = QDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDir {
  pub fn setPath<T: QDir_setPath>(&mut self, value: T) -> i32 {
    value.setPath(self);
    return 1;
  }
}

pub trait QDir_setPath {
  fn setPath(self, this: &mut QDir) -> i32;
}

// proto: void QDir::setPath(const QString & path);
impl<'a> /*trait*/ QDir_setPath for (&'a  QString) {
  fn setPath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir7setPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir7setPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDir {
  pub fn isRelativePath<T: QDir_isRelativePath>(&mut self, value: T) -> i32 {
    value.isRelativePath(self);
    return 1;
  }
}

pub trait QDir_isRelativePath {
  fn isRelativePath(self, this: &mut QDir) -> i32;
}

// proto: bool QDir::isRelativePath(const QString & path);
impl<'a> /*trait*/ QDir_isRelativePath for (&'a  QString) {
  fn isRelativePath(self, this: &mut QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QDir14isRelativePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QDir14isRelativePathERK7QString(arg0)};
    return 1;
  }
}

