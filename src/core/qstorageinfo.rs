// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtCore/qstorageinfo.h
// dst-file: /src/core/qstorageinfo.rs
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
use super::qbytearray::QByteArray; // 773
use super::qstring::QString; // 773
use super::qdir::QDir; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  qint64 QStorageInfo::bytesFree();
  fn _ZNK12QStorageInfo9bytesFreeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QStorageInfo::QStorageInfo(const QStorageInfo & other);
  fn _ZN12QStorageInfoC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QStorageInfo::isRoot();
  fn _ZNK12QStorageInfo6isRootEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QStorageInfo::isReadOnly();
  fn _ZNK12QStorageInfo10isReadOnlyEv(qthis: *mut c_void) -> c_char;
  // proto:  QByteArray QStorageInfo::fileSystemType();
  fn _ZNK12QStorageInfo14fileSystemTypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::setPath(const QString & path);
  fn _ZN12QStorageInfo7setPathERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QList<QStorageInfo> QStorageInfo::mountedVolumes();
  fn _ZN12QStorageInfo14mountedVolumesEv();
  // proto:  QString QStorageInfo::name();
  fn _ZNK12QStorageInfo4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::refresh();
  fn _ZN12QStorageInfo7refreshEv(qthis: *mut c_void);
  // proto:  bool QStorageInfo::isValid();
  fn _ZNK12QStorageInfo7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QStorageInfo::isReady();
  fn _ZNK12QStorageInfo7isReadyEv(qthis: *mut c_void) -> c_char;
  // proto:  qint64 QStorageInfo::bytesTotal();
  fn _ZNK12QStorageInfo10bytesTotalEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QString QStorageInfo::rootPath();
  fn _ZNK12QStorageInfo8rootPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::~QStorageInfo();
  fn _ZN12QStorageInfoD0Ev(qthis: *mut c_void);
  // proto:  qint64 QStorageInfo::bytesAvailable();
  fn _ZNK12QStorageInfo14bytesAvailableEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QStorageInfo::QStorageInfo();
  fn _ZN12QStorageInfoC1Ev(qthis: *mut c_void);
  // proto:  void QStorageInfo::QStorageInfo(const QDir & dir);
  fn _ZN12QStorageInfoC1ERK4QDir(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QStorageInfo QStorageInfo::root();
  fn _ZN12QStorageInfo4rootEv() -> *mut c_void;
  // proto:  void QStorageInfo::QStorageInfo(const QString & path);
  fn _ZN12QStorageInfoC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QStorageInfo::device();
  fn _ZNK12QStorageInfo6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QStorageInfo::displayName();
  fn _ZNK12QStorageInfo11displayNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStorageInfo::swap(QStorageInfo & other);
  fn _ZN12QStorageInfo4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStorageInfo)=1
pub struct QStorageInfo {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStorageInfo {
  pub fn inheritFrom(qthis: *mut c_void) -> QStorageInfo {
    return QStorageInfo{qclsinst: qthis};
  }
}
  // proto:  qint64 QStorageInfo::bytesFree();
impl /*struct*/ QStorageInfo {
  pub fn bytesFree<RetType, T: QStorageInfo_bytesFree<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesFree(self);
    // return 1;
  }
}

pub trait QStorageInfo_bytesFree<RetType> {
  fn bytesFree(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  qint64 QStorageInfo::bytesFree();
impl<'a> /*trait*/ QStorageInfo_bytesFree<i64> for () {
  fn bytesFree(self , rsthis: & QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo9bytesFreeEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo9bytesFreeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QStorageInfo::QStorageInfo(const QStorageInfo & other);
impl /*struct*/ QStorageInfo {
  pub fn New<T: QStorageInfo_New>(value: T) -> QStorageInfo {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStorageInfo_New {
  fn New(self) -> QStorageInfo;
}

  // proto:  void QStorageInfo::QStorageInfo(const QStorageInfo & other);
impl<'a> /*trait*/ QStorageInfo_New for (&'a QStorageInfo) {
  fn New(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfoC1ERKS_(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStorageInfo::isRoot();
impl /*struct*/ QStorageInfo {
  pub fn isRoot<RetType, T: QStorageInfo_isRoot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRoot(self);
    // return 1;
  }
}

pub trait QStorageInfo_isRoot<RetType> {
  fn isRoot(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  bool QStorageInfo::isRoot();
impl<'a> /*trait*/ QStorageInfo_isRoot<i8> for () {
  fn isRoot(self , rsthis: & QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo6isRootEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QStorageInfo::isReadOnly();
impl /*struct*/ QStorageInfo {
  pub fn isReadOnly<RetType, T: QStorageInfo_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QStorageInfo_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  bool QStorageInfo::isReadOnly();
impl<'a> /*trait*/ QStorageInfo_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QStorageInfo::fileSystemType();
impl /*struct*/ QStorageInfo {
  pub fn fileSystemType<RetType, T: QStorageInfo_fileSystemType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileSystemType(self);
    // return 1;
  }
}

pub trait QStorageInfo_fileSystemType<RetType> {
  fn fileSystemType(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  QByteArray QStorageInfo::fileSystemType();
impl<'a> /*trait*/ QStorageInfo_fileSystemType<QByteArray> for () {
  fn fileSystemType(self , rsthis: & QStorageInfo) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo14fileSystemTypeEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo14fileSystemTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStorageInfo::setPath(const QString & path);
impl /*struct*/ QStorageInfo {
  pub fn setPath<RetType, T: QStorageInfo_setPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPath(self);
    // return 1;
  }
}

pub trait QStorageInfo_setPath<RetType> {
  fn setPath(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  void QStorageInfo::setPath(const QString & path);
impl<'a> /*trait*/ QStorageInfo_setPath<()> for (&'a QString) {
  fn setPath(self , rsthis: & QStorageInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo7setPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStorageInfo7setPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QList<QStorageInfo> QStorageInfo::mountedVolumes();
impl /*struct*/ QStorageInfo {
  pub fn mountedVolumes_s<RetType, T: QStorageInfo_mountedVolumes_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.mountedVolumes_s();
    // return 1;
  }
}

pub trait QStorageInfo_mountedVolumes_s<RetType> {
  fn mountedVolumes_s(self ) -> RetType;
}

  // proto: static QList<QStorageInfo> QStorageInfo::mountedVolumes();
impl<'a> /*trait*/ QStorageInfo_mountedVolumes_s<()> for () {
  fn mountedVolumes_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo14mountedVolumesEv()};
     unsafe {_ZN12QStorageInfo14mountedVolumesEv()};
    // return 1;
  }
}

  // proto:  QString QStorageInfo::name();
impl /*struct*/ QStorageInfo {
  pub fn name<RetType, T: QStorageInfo_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QStorageInfo_name<RetType> {
  fn name(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  QString QStorageInfo::name();
impl<'a> /*trait*/ QStorageInfo_name<QString> for () {
  fn name(self , rsthis: & QStorageInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo4nameEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStorageInfo::refresh();
impl /*struct*/ QStorageInfo {
  pub fn refresh<RetType, T: QStorageInfo_refresh<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refresh(self);
    // return 1;
  }
}

pub trait QStorageInfo_refresh<RetType> {
  fn refresh(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  void QStorageInfo::refresh();
impl<'a> /*trait*/ QStorageInfo_refresh<()> for () {
  fn refresh(self , rsthis: & QStorageInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo7refreshEv()};
     unsafe {_ZN12QStorageInfo7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QStorageInfo::isValid();
impl /*struct*/ QStorageInfo {
  pub fn isValid<RetType, T: QStorageInfo_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QStorageInfo_isValid<RetType> {
  fn isValid(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  bool QStorageInfo::isValid();
impl<'a> /*trait*/ QStorageInfo_isValid<i8> for () {
  fn isValid(self , rsthis: & QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo7isValidEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QStorageInfo::isReady();
impl /*struct*/ QStorageInfo {
  pub fn isReady<RetType, T: QStorageInfo_isReady<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReady(self);
    // return 1;
  }
}

pub trait QStorageInfo_isReady<RetType> {
  fn isReady(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  bool QStorageInfo::isReady();
impl<'a> /*trait*/ QStorageInfo_isReady<i8> for () {
  fn isReady(self , rsthis: & QStorageInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo7isReadyEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo7isReadyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QStorageInfo::bytesTotal();
impl /*struct*/ QStorageInfo {
  pub fn bytesTotal<RetType, T: QStorageInfo_bytesTotal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesTotal(self);
    // return 1;
  }
}

pub trait QStorageInfo_bytesTotal<RetType> {
  fn bytesTotal(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  qint64 QStorageInfo::bytesTotal();
impl<'a> /*trait*/ QStorageInfo_bytesTotal<i64> for () {
  fn bytesTotal(self , rsthis: & QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo10bytesTotalEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo10bytesTotalEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QString QStorageInfo::rootPath();
impl /*struct*/ QStorageInfo {
  pub fn rootPath<RetType, T: QStorageInfo_rootPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootPath(self);
    // return 1;
  }
}

pub trait QStorageInfo_rootPath<RetType> {
  fn rootPath(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  QString QStorageInfo::rootPath();
impl<'a> /*trait*/ QStorageInfo_rootPath<QString> for () {
  fn rootPath(self , rsthis: & QStorageInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo8rootPathEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo8rootPathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStorageInfo::~QStorageInfo();
impl /*struct*/ QStorageInfo {
  pub fn Free<RetType, T: QStorageInfo_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStorageInfo_Free<RetType> {
  fn Free(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  void QStorageInfo::~QStorageInfo();
impl<'a> /*trait*/ QStorageInfo_Free<()> for () {
  fn Free(self , rsthis: & QStorageInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoD0Ev()};
     unsafe {_ZN12QStorageInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QStorageInfo::bytesAvailable();
impl /*struct*/ QStorageInfo {
  pub fn bytesAvailable<RetType, T: QStorageInfo_bytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable(self);
    // return 1;
  }
}

pub trait QStorageInfo_bytesAvailable<RetType> {
  fn bytesAvailable(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  qint64 QStorageInfo::bytesAvailable();
impl<'a> /*trait*/ QStorageInfo_bytesAvailable<i64> for () {
  fn bytesAvailable(self , rsthis: & QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QStorageInfo::QStorageInfo();
impl<'a> /*trait*/ QStorageInfo_New for () {
  fn New(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1Ev()};
    unsafe {_ZN12QStorageInfoC1Ev(qthis)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStorageInfo::QStorageInfo(const QDir & dir);
impl<'a> /*trait*/ QStorageInfo_New for (&'a QDir) {
  fn New(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERK4QDir()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfoC1ERK4QDir(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QStorageInfo QStorageInfo::root();
impl /*struct*/ QStorageInfo {
  pub fn root_s<RetType, T: QStorageInfo_root_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.root_s();
    // return 1;
  }
}

pub trait QStorageInfo_root_s<RetType> {
  fn root_s(self ) -> RetType;
}

  // proto: static QStorageInfo QStorageInfo::root();
impl<'a> /*trait*/ QStorageInfo_root_s<QStorageInfo> for () {
  fn root_s(self ) -> QStorageInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo4rootEv()};
    let mut ret = unsafe {_ZN12QStorageInfo4rootEv()};
    let mut ret1 = QStorageInfo::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStorageInfo::QStorageInfo(const QString & path);
impl<'a> /*trait*/ QStorageInfo_New for (&'a QString) {
  fn New(self) -> QStorageInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfoC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStorageInfoC1ERK7QString(qthis, arg0)};
    let rsthis = QStorageInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QStorageInfo::device();
impl /*struct*/ QStorageInfo {
  pub fn device<RetType, T: QStorageInfo_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QStorageInfo_device<RetType> {
  fn device(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  QByteArray QStorageInfo::device();
impl<'a> /*trait*/ QStorageInfo_device<QByteArray> for () {
  fn device(self , rsthis: & QStorageInfo) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo6deviceEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QStorageInfo::displayName();
impl /*struct*/ QStorageInfo {
  pub fn displayName<RetType, T: QStorageInfo_displayName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.displayName(self);
    // return 1;
  }
}

pub trait QStorageInfo_displayName<RetType> {
  fn displayName(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  QString QStorageInfo::displayName();
impl<'a> /*trait*/ QStorageInfo_displayName<QString> for () {
  fn displayName(self , rsthis: & QStorageInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStorageInfo11displayNameEv()};
    let mut ret = unsafe {_ZNK12QStorageInfo11displayNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStorageInfo::swap(QStorageInfo & other);
impl /*struct*/ QStorageInfo {
  pub fn swap<RetType, T: QStorageInfo_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QStorageInfo_swap<RetType> {
  fn swap(self , rsthis: & QStorageInfo) -> RetType;
}

  // proto:  void QStorageInfo::swap(QStorageInfo & other);
impl<'a> /*trait*/ QStorageInfo_swap<()> for (&'a QStorageInfo) {
  fn swap(self , rsthis: & QStorageInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStorageInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStorageInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

