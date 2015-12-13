// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qdir::QDir;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK12QStorageInfo9bytesFreeEv() -> i32;
  fn _ZN12QStorageInfoC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK12QStorageInfo6isRootEv() -> i32;
  fn _ZNK12QStorageInfo10isReadOnlyEv() -> i32;
  fn _ZNK12QStorageInfo14fileSystemTypeEv() -> i32;
  fn _ZN12QStorageInfo7setPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN12QStorageInfo14mountedVolumesEv() -> i32;
  fn _ZNK12QStorageInfo4nameEv() -> i32;
  fn _ZN12QStorageInfo7refreshEv() -> i32;
  fn _ZNK12QStorageInfo7isValidEv() -> i32;
  fn _ZNK12QStorageInfo7isReadyEv() -> i32;
  fn _ZNK12QStorageInfo10bytesTotalEv() -> i32;
  fn _ZNK12QStorageInfo8rootPathEv() -> i32;
  fn _ZN12QStorageInfoD0Ev() -> i32;
  fn _ZNK12QStorageInfo14bytesAvailableEv() -> i32;
  fn _ZN12QStorageInfoC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN12QStorageInfoC1ERK4QDir(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QStorageInfo4rootEv() -> i32;
  fn _ZN12QStorageInfoC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK12QStorageInfo6deviceEv() -> i32;
  fn _ZNK12QStorageInfo11displayNameEv() -> i32;
  fn _ZN12QStorageInfo4swapERS_(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QStorageInfo)=1
pub struct QStorageInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStorageInfo {
  pub fn bytesFree<T: QStorageInfo_bytesFree>(&mut self, value: T) -> i32 {
    value.bytesFree(self);
    return 1;
  }
}

pub trait QStorageInfo_bytesFree {
  fn bytesFree(self, this: &mut QStorageInfo) -> i32;
}

// proto: long long QStorageInfo::bytesFree();
impl<'a> /*trait*/ QStorageInfo_bytesFree for () {
  fn bytesFree(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo9bytesFreeEv()};
    unsafe {_ZNK12QStorageInfo9bytesFreeEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QStorageInfoC1ERKS_(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isRoot<T: QStorageInfo_isRoot>(&mut self, value: T) -> i32 {
    value.isRoot(self);
    return 1;
  }
}

pub trait QStorageInfo_isRoot {
  fn isRoot(self, this: &mut QStorageInfo) -> i32;
}

// proto: bool QStorageInfo::isRoot();
impl<'a> /*trait*/ QStorageInfo_isRoot for () {
  fn isRoot(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo6isRootEv()};
    unsafe {_ZNK12QStorageInfo6isRootEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isReadOnly<T: QStorageInfo_isReadOnly>(&mut self, value: T) -> i32 {
    value.isReadOnly(self);
    return 1;
  }
}

pub trait QStorageInfo_isReadOnly {
  fn isReadOnly(self, this: &mut QStorageInfo) -> i32;
}

// proto: bool QStorageInfo::isReadOnly();
impl<'a> /*trait*/ QStorageInfo_isReadOnly for () {
  fn isReadOnly(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo10isReadOnlyEv()};
    unsafe {_ZNK12QStorageInfo10isReadOnlyEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn fileSystemType<T: QStorageInfo_fileSystemType>(&mut self, value: T) -> i32 {
    value.fileSystemType(self);
    return 1;
  }
}

pub trait QStorageInfo_fileSystemType {
  fn fileSystemType(self, this: &mut QStorageInfo) -> i32;
}

// proto: QByteArray QStorageInfo::fileSystemType();
impl<'a> /*trait*/ QStorageInfo_fileSystemType for () {
  fn fileSystemType(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo14fileSystemTypeEv()};
    unsafe {_ZNK12QStorageInfo14fileSystemTypeEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn setPath<T: QStorageInfo_setPath>(&mut self, value: T) -> i32 {
    value.setPath(self);
    return 1;
  }
}

pub trait QStorageInfo_setPath {
  fn setPath(self, this: &mut QStorageInfo) -> i32;
}

// proto: void QStorageInfo::setPath(const QString & path);
impl<'a> /*trait*/ QStorageInfo_setPath for (&'a  QString) {
  fn setPath(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo7setPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QStorageInfo7setPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn mountedVolumes<T: QStorageInfo_mountedVolumes>(&mut self, value: T) -> i32 {
    value.mountedVolumes(self);
    return 1;
  }
}

pub trait QStorageInfo_mountedVolumes {
  fn mountedVolumes(self, this: &mut QStorageInfo) -> i32;
}

// proto: QList<QStorageInfo> QStorageInfo::mountedVolumes();
impl<'a> /*trait*/ QStorageInfo_mountedVolumes for () {
  fn mountedVolumes(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo14mountedVolumesEv()};
    unsafe {_ZN12QStorageInfo14mountedVolumesEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn name<T: QStorageInfo_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QStorageInfo_name {
  fn name(self, this: &mut QStorageInfo) -> i32;
}

// proto: QString QStorageInfo::name();
impl<'a> /*trait*/ QStorageInfo_name for () {
  fn name(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo4nameEv()};
    unsafe {_ZNK12QStorageInfo4nameEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn refresh<T: QStorageInfo_refresh>(&mut self, value: T) -> i32 {
    value.refresh(self);
    return 1;
  }
}

pub trait QStorageInfo_refresh {
  fn refresh(self, this: &mut QStorageInfo) -> i32;
}

// proto: void QStorageInfo::refresh();
impl<'a> /*trait*/ QStorageInfo_refresh for () {
  fn refresh(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo7refreshEv()};
    unsafe {_ZN12QStorageInfo7refreshEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isValid<T: QStorageInfo_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QStorageInfo_isValid {
  fn isValid(self, this: &mut QStorageInfo) -> i32;
}

// proto: bool QStorageInfo::isValid();
impl<'a> /*trait*/ QStorageInfo_isValid for () {
  fn isValid(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo7isValidEv()};
    unsafe {_ZNK12QStorageInfo7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn isReady<T: QStorageInfo_isReady>(&mut self, value: T) -> i32 {
    value.isReady(self);
    return 1;
  }
}

pub trait QStorageInfo_isReady {
  fn isReady(self, this: &mut QStorageInfo) -> i32;
}

// proto: bool QStorageInfo::isReady();
impl<'a> /*trait*/ QStorageInfo_isReady for () {
  fn isReady(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo7isReadyEv()};
    unsafe {_ZNK12QStorageInfo7isReadyEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn bytesTotal<T: QStorageInfo_bytesTotal>(&mut self, value: T) -> i32 {
    value.bytesTotal(self);
    return 1;
  }
}

pub trait QStorageInfo_bytesTotal {
  fn bytesTotal(self, this: &mut QStorageInfo) -> i32;
}

// proto: long long QStorageInfo::bytesTotal();
impl<'a> /*trait*/ QStorageInfo_bytesTotal for () {
  fn bytesTotal(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo10bytesTotalEv()};
    unsafe {_ZNK12QStorageInfo10bytesTotalEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn rootPath<T: QStorageInfo_rootPath>(&mut self, value: T) -> i32 {
    value.rootPath(self);
    return 1;
  }
}

pub trait QStorageInfo_rootPath {
  fn rootPath(self, this: &mut QStorageInfo) -> i32;
}

// proto: QString QStorageInfo::rootPath();
impl<'a> /*trait*/ QStorageInfo_rootPath for () {
  fn rootPath(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo8rootPathEv()};
    unsafe {_ZNK12QStorageInfo8rootPathEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn FreeQStorageInfo<T: QStorageInfo_FreeQStorageInfo>(&mut self, value: T) -> i32 {
    value.FreeQStorageInfo(self);
    return 1;
  }
}

pub trait QStorageInfo_FreeQStorageInfo {
  fn FreeQStorageInfo(self, this: &mut QStorageInfo) -> i32;
}

// proto: void QStorageInfo::FreeQStorageInfo();
impl<'a> /*trait*/ QStorageInfo_FreeQStorageInfo for () {
  fn FreeQStorageInfo(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoD0Ev()};
    unsafe {_ZN12QStorageInfoD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn bytesAvailable<T: QStorageInfo_bytesAvailable>(&mut self, value: T) -> i32 {
    value.bytesAvailable(self);
    return 1;
  }
}

pub trait QStorageInfo_bytesAvailable {
  fn bytesAvailable(self, this: &mut QStorageInfo) -> i32;
}

// proto: long long QStorageInfo::bytesAvailable();
impl<'a> /*trait*/ QStorageInfo_bytesAvailable for () {
  fn bytesAvailable(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo14bytesAvailableEv()};
    unsafe {_ZNK12QStorageInfo14bytesAvailableEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QStorageInfoC1ERK4QDir(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn root<T: QStorageInfo_root>(&mut self, value: T) -> i32 {
    value.root(self);
    return 1;
  }
}

pub trait QStorageInfo_root {
  fn root(self, this: &mut QStorageInfo) -> i32;
}

// proto: QStorageInfo QStorageInfo::root();
impl<'a> /*trait*/ QStorageInfo_root for () {
  fn root(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo4rootEv()};
    unsafe {_ZN12QStorageInfo4rootEv()};
    return 1;
  }
}

// proto: void QStorageInfo::NewQStorageInfo(const QString & path);
impl<'a> /*trait*/ QStorageInfo_NewQStorageInfo for (&'a  QString) {
  fn NewQStorageInfo(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QStorageInfoC1ERK7QString(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn device<T: QStorageInfo_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QStorageInfo_device {
  fn device(self, this: &mut QStorageInfo) -> i32;
}

// proto: QByteArray QStorageInfo::device();
impl<'a> /*trait*/ QStorageInfo_device for () {
  fn device(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo6deviceEv()};
    unsafe {_ZNK12QStorageInfo6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn displayName<T: QStorageInfo_displayName>(&mut self, value: T) -> i32 {
    value.displayName(self);
    return 1;
  }
}

pub trait QStorageInfo_displayName {
  fn displayName(self, this: &mut QStorageInfo) -> i32;
}

// proto: QString QStorageInfo::displayName();
impl<'a> /*trait*/ QStorageInfo_displayName for () {
  fn displayName(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo11displayNameEv()};
    unsafe {_ZNK12QStorageInfo11displayNameEv()};
    return 1;
  }
}

impl /*struct*/ QStorageInfo {
  pub fn swap<T: QStorageInfo_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QStorageInfo_swap {
  fn swap(self, this: &mut QStorageInfo) -> i32;
}

// proto: void QStorageInfo::swap(QStorageInfo & other);
impl<'a> /*trait*/ QStorageInfo_swap for (&'a mut QStorageInfo) {
  fn swap(self, this: &mut QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfo4swapERS_(arg0)};
    return 1;
  }
}

