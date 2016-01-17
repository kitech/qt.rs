// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qlockfile.h
// dst-file: /src/core/qlockfile.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLockFile_Class_Size() -> c_int;
  // proto:  bool QLockFile::removeStaleLockFile();
  fn _ZN9QLockFile19removeStaleLockFileEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QLockFile::staleLockTime();
  fn _ZNK9QLockFile13staleLockTimeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QLockFile::isLocked();
  fn _ZNK9QLockFile8isLockedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLockFile::QLockFile(const QLockFile & );
  fn _ZN9QLockFileC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLockFile::~QLockFile();
  fn _ZN9QLockFileD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QLockFile::unlock();
  fn _ZN9QLockFile6unlockEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLockFile::tryLock(int timeout);
  fn _ZN9QLockFile7tryLockEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  bool QLockFile::lock();
  fn _ZN9QLockFile4lockEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLockFile::setStaleLockTime(int );
  fn _ZN9QLockFile16setStaleLockTimeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QLockFile::getLockInfo(qint64 * pid, QString * hostname, QString * appname);
  fn _ZNK9QLockFile11getLockInfoEPxP7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_longlong, arg1: *mut c_void, arg2: *mut c_void) -> c_char;
  // proto:  void QLockFile::QLockFile(const QString & fileName);
  fn _ZN9QLockFileC2ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLockFile)=1
#[derive(Default)]
pub struct QLockFile {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QLockFile {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLockFile {
    return QLockFile{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QLockFile::removeStaleLockFile();
impl /*struct*/ QLockFile {
  pub fn removeStaleLockFile<RetType, T: QLockFile_removeStaleLockFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeStaleLockFile(self);
    // return 1;
  }
}

pub trait QLockFile_removeStaleLockFile<RetType> {
  fn removeStaleLockFile(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  bool QLockFile::removeStaleLockFile();
impl<'a> /*trait*/ QLockFile_removeStaleLockFile<i8> for () {
  fn removeStaleLockFile(self , rsthis: & QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile19removeStaleLockFileEv()};
    let mut ret = unsafe {_ZN9QLockFile19removeStaleLockFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QLockFile::staleLockTime();
impl /*struct*/ QLockFile {
  pub fn staleLockTime<RetType, T: QLockFile_staleLockTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.staleLockTime(self);
    // return 1;
  }
}

pub trait QLockFile_staleLockTime<RetType> {
  fn staleLockTime(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  int QLockFile::staleLockTime();
impl<'a> /*trait*/ QLockFile_staleLockTime<i32> for () {
  fn staleLockTime(self , rsthis: & QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile13staleLockTimeEv()};
    let mut ret = unsafe {_ZNK9QLockFile13staleLockTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QLockFile::isLocked();
impl /*struct*/ QLockFile {
  pub fn isLocked<RetType, T: QLockFile_isLocked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLocked(self);
    // return 1;
  }
}

pub trait QLockFile_isLocked<RetType> {
  fn isLocked(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  bool QLockFile::isLocked();
impl<'a> /*trait*/ QLockFile_isLocked<i8> for () {
  fn isLocked(self , rsthis: & QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile8isLockedEv()};
    let mut ret = unsafe {_ZNK9QLockFile8isLockedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLockFile::QLockFile(const QLockFile & );
impl /*struct*/ QLockFile {
  pub fn new<T: QLockFile_new>(value: T) -> QLockFile {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLockFile_new {
  fn new(self) -> QLockFile;
}

  // proto:  void QLockFile::QLockFile(const QLockFile & );
impl<'a> /*trait*/ QLockFile_new for (&'a QLockFile) {
  fn new(self) -> QLockFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileC2ERKS_()};
    let ctysz: c_int = unsafe{QLockFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLockFileC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QLockFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLockFile::~QLockFile();
impl /*struct*/ QLockFile {
  pub fn free<RetType, T: QLockFile_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLockFile_free<RetType> {
  fn free(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  void QLockFile::~QLockFile();
impl<'a> /*trait*/ QLockFile_free<()> for () {
  fn free(self , rsthis: & QLockFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileD2Ev()};
     unsafe {_ZN9QLockFileD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLockFile::unlock();
impl /*struct*/ QLockFile {
  pub fn unlock<RetType, T: QLockFile_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QLockFile_unlock<RetType> {
  fn unlock(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  void QLockFile::unlock();
impl<'a> /*trait*/ QLockFile_unlock<()> for () {
  fn unlock(self , rsthis: & QLockFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile6unlockEv()};
     unsafe {_ZN9QLockFile6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLockFile::tryLock(int timeout);
impl /*struct*/ QLockFile {
  pub fn tryLock<RetType, T: QLockFile_tryLock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryLock(self);
    // return 1;
  }
}

pub trait QLockFile_tryLock<RetType> {
  fn tryLock(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  bool QLockFile::tryLock(int timeout);
impl<'a> /*trait*/ QLockFile_tryLock<i8> for (i32) {
  fn tryLock(self , rsthis: & QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile7tryLockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QLockFile7tryLockEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLockFile::lock();
impl /*struct*/ QLockFile {
  pub fn lock<RetType, T: QLockFile_lock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QLockFile_lock<RetType> {
  fn lock(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  bool QLockFile::lock();
impl<'a> /*trait*/ QLockFile_lock<i8> for () {
  fn lock(self , rsthis: & QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile4lockEv()};
    let mut ret = unsafe {_ZN9QLockFile4lockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLockFile::setStaleLockTime(int );
impl /*struct*/ QLockFile {
  pub fn setStaleLockTime<RetType, T: QLockFile_setStaleLockTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStaleLockTime(self);
    // return 1;
  }
}

pub trait QLockFile_setStaleLockTime<RetType> {
  fn setStaleLockTime(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  void QLockFile::setStaleLockTime(int );
impl<'a> /*trait*/ QLockFile_setStaleLockTime<()> for (i32) {
  fn setStaleLockTime(self , rsthis: & QLockFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile16setStaleLockTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QLockFile16setStaleLockTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QLockFile::getLockInfo(qint64 * pid, QString * hostname, QString * appname);
impl /*struct*/ QLockFile {
  pub fn getLockInfo<RetType, T: QLockFile_getLockInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getLockInfo(self);
    // return 1;
  }
}

pub trait QLockFile_getLockInfo<RetType> {
  fn getLockInfo(self , rsthis: & QLockFile) -> RetType;
}

  // proto:  bool QLockFile::getLockInfo(qint64 * pid, QString * hostname, QString * appname);
impl<'a> /*trait*/ QLockFile_getLockInfo<i8> for (&'a mut Vec<i64>, &'a QString, &'a QString) {
  fn getLockInfo(self , rsthis: & QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile11getLockInfoEPxP7QStringS2_()};
    let arg0 = self.0.as_ptr()  as *mut c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QLockFile11getLockInfoEPxP7QStringS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLockFile::QLockFile(const QString & fileName);
impl<'a> /*trait*/ QLockFile_new for (&'a QString) {
  fn new(self) -> QLockFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileC2ERK7QString()};
    let ctysz: c_int = unsafe{QLockFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLockFileC2ERK7QString(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QLockFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

