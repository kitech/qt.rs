// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qstring::QString;
use super::qdir::QDir;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  long long QStorageInfo::bytesFree();
  fn _ZNK12QStorageInfo9bytesFreeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QStorageInfo::NewQStorageInfo(const QStorageInfo & other);
  fn _ZN12QStorageInfoC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QStorageInfo::isRoot();
  fn _ZNK12QStorageInfo6isRootEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QStorageInfo::isReadOnly();
  fn _ZNK12QStorageInfo10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QByteArray QStorageInfo::fileSystemType();
  fn _ZNK12QStorageInfo14fileSystemTypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::setPath(const QString & path);
  fn _ZN12QStorageInfo7setPathERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QList<QStorageInfo> QStorageInfo::mountedVolumes();
  fn _ZN12QStorageInfo14mountedVolumesEv() ;
  // proto:  QString QStorageInfo::name();
  fn _ZNK12QStorageInfo4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::refresh();
  fn _ZN12QStorageInfo7refreshEv(qthis: *mut c_void) ;
  // proto:  bool QStorageInfo::isValid();
  fn _ZNK12QStorageInfo7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QStorageInfo::isReady();
  fn _ZNK12QStorageInfo7isReadyEv(qthis: *mut c_void) -> int8_t;
  // proto:  long long QStorageInfo::bytesTotal();
  fn _ZNK12QStorageInfo10bytesTotalEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QString QStorageInfo::rootPath();
  fn _ZNK12QStorageInfo8rootPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::FreeQStorageInfo();
  fn _ZN12QStorageInfoD0Ev(qthis: *mut c_void) ;
  // proto:  long long QStorageInfo::bytesAvailable();
  fn _ZNK12QStorageInfo14bytesAvailableEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QStorageInfo::NewQStorageInfo();
  fn _ZN12QStorageInfoC1Ev(qthis: *mut c_void) ;
  // proto:  void QStorageInfo::NewQStorageInfo(const QDir & dir);
  fn _ZN12QStorageInfoC1ERK4QDir(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QStorageInfo QStorageInfo::root();
  fn _ZN12QStorageInfo4rootEv() -> *mut c_void;
  // proto:  void QStorageInfo::NewQStorageInfo(const QString & path);
  fn _ZN12QStorageInfoC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QStorageInfo::device();
  fn _ZNK12QStorageInfo6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QStorageInfo::displayName();
  fn _ZNK12QStorageInfo11displayNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::swap(QStorageInfo & other);
  fn _ZN12QStorageInfo4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStorageInfo)=1
pub struct QStorageInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStorageInfo {
  pub fn bytesFree<T: QStorageInfo_bytesFree>(&mut self, value: T) -> i64 {
    return value.bytesFree(self);
    // return 1;
  }
}

pub trait QStorageInfo_bytesFree {
  fn bytesFree(self, rsthis: &mut QStorageInfo) -> i64;
}

// proto:  long long QStorageInfo::bytesFree();
impl<'a> /*trait*/ QStorageInfo_bytesFree for () {
  fn bytesFree(self, rsthis: &mut QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo9bytesFreeEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo9bytesFreeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn NewQStorageInfo<T: QStorageInfo_NewQStorageInfo>(value: T) -> QStorageInfo {
    let rsthis = value.NewQStorageInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QStorageInfo_NewQStorageInfo {
  fn NewQStorageInfo(self) -> QStorageInfo;
}

// proto: void QStorageInfo::NewQStorageInfo(const QStorageInfo & other);
impl<'a> /*trait*/ QStorageInfo_NewQStorageInfo for (&'a  QStorageInfo) {
  fn NewQStorageInfo(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfoC1ERKS_(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isRoot<T: QStorageInfo_isRoot>(&mut self, value: T) -> i8 {
    return value.isRoot(self);
    // return 1;
  }
}

pub trait QStorageInfo_isRoot {
  fn isRoot(self, rsthis: &mut QStorageInfo) -> i8;
}

// proto:  bool QStorageInfo::isRoot();
impl<'a> /*trait*/ QStorageInfo_isRoot for () {
  fn isRoot(self, rsthis: &mut QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo6isRootEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isReadOnly<T: QStorageInfo_isReadOnly>(&mut self, value: T) -> i8 {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QStorageInfo_isReadOnly {
  fn isReadOnly(self, rsthis: &mut QStorageInfo) -> i8;
}

// proto:  bool QStorageInfo::isReadOnly();
impl<'a> /*trait*/ QStorageInfo_isReadOnly for () {
  fn isReadOnly(self, rsthis: &mut QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn fileSystemType<T: QStorageInfo_fileSystemType>(&mut self, value: T) -> QByteArray {
    return value.fileSystemType(self);
    // return 1;
  }
}

pub trait QStorageInfo_fileSystemType {
  fn fileSystemType(self, rsthis: &mut QStorageInfo) -> QByteArray;
}

// proto:  QByteArray QStorageInfo::fileSystemType();
impl<'a> /*trait*/ QStorageInfo_fileSystemType for () {
  fn fileSystemType(self, rsthis: &mut QStorageInfo) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo14fileSystemTypeEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo14fileSystemTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn setPath<T: QStorageInfo_setPath>(&mut self, value: T)  {
     value.setPath(self);
    // return 1;
  }
}

pub trait QStorageInfo_setPath {
  fn setPath(self, rsthis: &mut QStorageInfo) ;
}

// proto:  void QStorageInfo::setPath(const QString & path);
impl<'a> /*trait*/ QStorageInfo_setPath for (&'a  QString) {
  fn setPath(self, rsthis: &mut QStorageInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo7setPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStorageInfo7setPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn mountedVolumes<T: QStorageInfo_mountedVolumes>(&mut self, value: T)  {
     value.mountedVolumes(self);
    // return 1;
  }
}

pub trait QStorageInfo_mountedVolumes {
  fn mountedVolumes(self, rsthis: &mut QStorageInfo) ;
}

// proto: static QList<QStorageInfo> QStorageInfo::mountedVolumes();
impl<'a> /*trait*/ QStorageInfo_mountedVolumes for () {
  fn mountedVolumes(self, rsthis: &mut QStorageInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo14mountedVolumesEv()};
     unsafe {_ZN12QStorageInfo14mountedVolumesEv()};
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn name<T: QStorageInfo_name>(&mut self, value: T) -> QString {
    return value.name(self);
    // return 1;
  }
}

pub trait QStorageInfo_name {
  fn name(self, rsthis: &mut QStorageInfo) -> QString;
}

// proto:  QString QStorageInfo::name();
impl<'a> /*trait*/ QStorageInfo_name for () {
  fn name(self, rsthis: &mut QStorageInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo4nameEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn refresh<T: QStorageInfo_refresh>(&mut self, value: T)  {
     value.refresh(self);
    // return 1;
  }
}

pub trait QStorageInfo_refresh {
  fn refresh(self, rsthis: &mut QStorageInfo) ;
}

// proto:  void QStorageInfo::refresh();
impl<'a> /*trait*/ QStorageInfo_refresh for () {
  fn refresh(self, rsthis: &mut QStorageInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo7refreshEv()};
     unsafe {_ZN12QStorageInfo7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isValid<T: QStorageInfo_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QStorageInfo_isValid {
  fn isValid(self, rsthis: &mut QStorageInfo) -> i8;
}

// proto:  bool QStorageInfo::isValid();
impl<'a> /*trait*/ QStorageInfo_isValid for () {
  fn isValid(self, rsthis: &mut QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo7isValidEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isReady<T: QStorageInfo_isReady>(&mut self, value: T) -> i8 {
    return value.isReady(self);
    // return 1;
  }
}

pub trait QStorageInfo_isReady {
  fn isReady(self, rsthis: &mut QStorageInfo) -> i8;
}

// proto:  bool QStorageInfo::isReady();
impl<'a> /*trait*/ QStorageInfo_isReady for () {
  fn isReady(self, rsthis: &mut QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo7isReadyEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo7isReadyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn bytesTotal<T: QStorageInfo_bytesTotal>(&mut self, value: T) -> i64 {
    return value.bytesTotal(self);
    // return 1;
  }
}

pub trait QStorageInfo_bytesTotal {
  fn bytesTotal(self, rsthis: &mut QStorageInfo) -> i64;
}

// proto:  long long QStorageInfo::bytesTotal();
impl<'a> /*trait*/ QStorageInfo_bytesTotal for () {
  fn bytesTotal(self, rsthis: &mut QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo10bytesTotalEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo10bytesTotalEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn rootPath<T: QStorageInfo_rootPath>(&mut self, value: T) -> QString {
    return value.rootPath(self);
    // return 1;
  }
}

pub trait QStorageInfo_rootPath {
  fn rootPath(self, rsthis: &mut QStorageInfo) -> QString;
}

// proto:  QString QStorageInfo::rootPath();
impl<'a> /*trait*/ QStorageInfo_rootPath for () {
  fn rootPath(self, rsthis: &mut QStorageInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo8rootPathEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo8rootPathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn FreeQStorageInfo<T: QStorageInfo_FreeQStorageInfo>(&mut self, value: T)  {
     value.FreeQStorageInfo(self);
    // return 1;
  }
}

pub trait QStorageInfo_FreeQStorageInfo {
  fn FreeQStorageInfo(self, rsthis: &mut QStorageInfo) ;
}

// proto:  void QStorageInfo::FreeQStorageInfo();
impl<'a> /*trait*/ QStorageInfo_FreeQStorageInfo for () {
  fn FreeQStorageInfo(self, rsthis: &mut QStorageInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoD0Ev()};
     unsafe {_ZN12QStorageInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn bytesAvailable<T: QStorageInfo_bytesAvailable>(&mut self, value: T) -> i64 {
    return value.bytesAvailable(self);
    // return 1;
  }
}

pub trait QStorageInfo_bytesAvailable {
  fn bytesAvailable(self, rsthis: &mut QStorageInfo) -> i64;
}

// proto:  long long QStorageInfo::bytesAvailable();
impl<'a> /*trait*/ QStorageInfo_bytesAvailable for () {
  fn bytesAvailable(self, rsthis: &mut QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto: void QStorageInfo::NewQStorageInfo();
impl<'a> /*trait*/ QStorageInfo_NewQStorageInfo for () {
  fn NewQStorageInfo(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1Ev()};
    unsafe {_ZN12QStorageInfoC1Ev(qthis)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStorageInfo::NewQStorageInfo(const QDir & dir);
impl<'a> /*trait*/ QStorageInfo_NewQStorageInfo for (&'a  QDir) {
  fn NewQStorageInfo(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERK4QDir()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfoC1ERK4QDir(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn root<T: QStorageInfo_root>(&mut self, value: T) -> QStorageInfo {
    return value.root(self);
    // return 1;
  }
}

pub trait QStorageInfo_root {
  fn root(self, rsthis: &mut QStorageInfo) -> QStorageInfo;
}

// proto: static QStorageInfo QStorageInfo::root();
impl<'a> /*trait*/ QStorageInfo_root for () {
  fn root(self, rsthis: &mut QStorageInfo) -> QStorageInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo4rootEv()};
    let mut ret = unsafe {_ZN12QStorageInfo4rootEv()};
    let mut ret1 = QStorageInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QStorageInfo::NewQStorageInfo(const QString & path);
impl<'a> /*trait*/ QStorageInfo_NewQStorageInfo for (&'a  QString) {
  fn NewQStorageInfo(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfoC1ERK7QString(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn device<T: QStorageInfo_device>(&mut self, value: T) -> QByteArray {
    return value.device(self);
    // return 1;
  }
}

pub trait QStorageInfo_device {
  fn device(self, rsthis: &mut QStorageInfo) -> QByteArray;
}

// proto:  QByteArray QStorageInfo::device();
impl<'a> /*trait*/ QStorageInfo_device for () {
  fn device(self, rsthis: &mut QStorageInfo) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo6deviceEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn displayName<T: QStorageInfo_displayName>(&mut self, value: T) -> QString {
    return value.displayName(self);
    // return 1;
  }
}

pub trait QStorageInfo_displayName {
  fn displayName(self, rsthis: &mut QStorageInfo) -> QString;
}

// proto:  QString QStorageInfo::displayName();
impl<'a> /*trait*/ QStorageInfo_displayName for () {
  fn displayName(self, rsthis: &mut QStorageInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo11displayNameEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo11displayNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn swap<T: QStorageInfo_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QStorageInfo_swap {
  fn swap(self, rsthis: &mut QStorageInfo) ;
}

// proto:  void QStorageInfo::swap(QStorageInfo & other);
impl<'a> /*trait*/ QStorageInfo_swap for (&'a mut QStorageInfo) {
  fn swap(self, rsthis: &mut QStorageInfo)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStorageInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

