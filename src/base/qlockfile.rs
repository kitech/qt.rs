// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN9QLockFile19removeStaleLockFileEv() -> i32;
  fn _ZNK9QLockFile13staleLockTimeEv() -> i32;
  fn _ZNK9QLockFile8isLockedEv() -> i32;
  fn _ZN9QLockFileC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QLockFileD0Ev() -> i32;
  fn _ZN9QLockFile6unlockEv() -> i32;
  fn _ZN9QLockFile7tryLockEi(arg0: c_int) -> i32;
  fn _ZN9QLockFile4lockEv() -> i32;
  fn _ZN9QLockFile16setStaleLockTimeEi(arg0: c_int) -> i32;
  fn _ZNK9QLockFile11getLockInfoEPxP7QStringS2_(arg0: *mut c_longlong, arg1: *mut c_void, arg2: *mut c_void) -> i32;
  fn _ZN9QLockFileC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QLockFile)=1
pub struct QLockFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLockFile {
  pub fn removeStaleLockFile<T: QLockFile_removeStaleLockFile>(&mut self, value: T) -> i32 {
    value.removeStaleLockFile(self);
    return 1;
  }
}

pub trait QLockFile_removeStaleLockFile {
  fn removeStaleLockFile(self, this: &mut QLockFile) -> i32;
}

// proto: bool QLockFile::removeStaleLockFile();
impl<'a> /*trait*/ QLockFile_removeStaleLockFile for () {
  fn removeStaleLockFile(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile19removeStaleLockFileEv()};
    unsafe {_ZN9QLockFile19removeStaleLockFileEv()};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn staleLockTime<T: QLockFile_staleLockTime>(&mut self, value: T) -> i32 {
    value.staleLockTime(self);
    return 1;
  }
}

pub trait QLockFile_staleLockTime {
  fn staleLockTime(self, this: &mut QLockFile) -> i32;
}

// proto: int QLockFile::staleLockTime();
impl<'a> /*trait*/ QLockFile_staleLockTime for () {
  fn staleLockTime(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile13staleLockTimeEv()};
    unsafe {_ZNK9QLockFile13staleLockTimeEv()};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn isLocked<T: QLockFile_isLocked>(&mut self, value: T) -> i32 {
    value.isLocked(self);
    return 1;
  }
}

pub trait QLockFile_isLocked {
  fn isLocked(self, this: &mut QLockFile) -> i32;
}

// proto: bool QLockFile::isLocked();
impl<'a> /*trait*/ QLockFile_isLocked for () {
  fn isLocked(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile8isLockedEv()};
    unsafe {_ZNK9QLockFile8isLockedEv()};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn NewQLockFile<T: QLockFile_NewQLockFile>(value: T) -> QLockFile {
    let rsthis = value.NewQLockFile();
    return rsthis;
    // return 1;
  }
}

pub trait QLockFile_NewQLockFile {
  fn NewQLockFile(self) -> QLockFile;
}

// proto: void QLockFile::NewQLockFile(const QLockFile & );
impl<'a> /*trait*/ QLockFile_NewQLockFile for (&'a  QLockFile) {
  fn NewQLockFile(self) -> QLockFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLockFileC1ERKS_(qthis, arg0)};
    let rsthis = QLockFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn FreeQLockFile<T: QLockFile_FreeQLockFile>(&mut self, value: T) -> i32 {
    value.FreeQLockFile(self);
    return 1;
  }
}

pub trait QLockFile_FreeQLockFile {
  fn FreeQLockFile(self, this: &mut QLockFile) -> i32;
}

// proto: void QLockFile::FreeQLockFile();
impl<'a> /*trait*/ QLockFile_FreeQLockFile for () {
  fn FreeQLockFile(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileD0Ev()};
    unsafe {_ZN9QLockFileD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn unlock<T: QLockFile_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QLockFile_unlock {
  fn unlock(self, this: &mut QLockFile) -> i32;
}

// proto: void QLockFile::unlock();
impl<'a> /*trait*/ QLockFile_unlock for () {
  fn unlock(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile6unlockEv()};
    unsafe {_ZN9QLockFile6unlockEv()};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn tryLock<T: QLockFile_tryLock>(&mut self, value: T) -> i32 {
    value.tryLock(self);
    return 1;
  }
}

pub trait QLockFile_tryLock {
  fn tryLock(self, this: &mut QLockFile) -> i32;
}

// proto: bool QLockFile::tryLock(int timeout);
impl<'a> /*trait*/ QLockFile_tryLock for (i32) {
  fn tryLock(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile7tryLockEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QLockFile7tryLockEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn lock<T: QLockFile_lock>(&mut self, value: T) -> i32 {
    value.lock(self);
    return 1;
  }
}

pub trait QLockFile_lock {
  fn lock(self, this: &mut QLockFile) -> i32;
}

// proto: bool QLockFile::lock();
impl<'a> /*trait*/ QLockFile_lock for () {
  fn lock(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile4lockEv()};
    unsafe {_ZN9QLockFile4lockEv()};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn setStaleLockTime<T: QLockFile_setStaleLockTime>(&mut self, value: T) -> i32 {
    value.setStaleLockTime(self);
    return 1;
  }
}

pub trait QLockFile_setStaleLockTime {
  fn setStaleLockTime(self, this: &mut QLockFile) -> i32;
}

// proto: void QLockFile::setStaleLockTime(int );
impl<'a> /*trait*/ QLockFile_setStaleLockTime for (i32) {
  fn setStaleLockTime(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile16setStaleLockTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QLockFile16setStaleLockTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn getLockInfo<T: QLockFile_getLockInfo>(&mut self, value: T) -> i32 {
    value.getLockInfo(self);
    return 1;
  }
}

pub trait QLockFile_getLockInfo {
  fn getLockInfo(self, this: &mut QLockFile) -> i32;
}

// proto: bool QLockFile::getLockInfo(qint64 * pid, QString * hostname, QString * appname);
impl<'a> /*trait*/ QLockFile_getLockInfo for (&'a mut i64, &'a mut QString, &'a mut QString) {
  fn getLockInfo(self, this: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile11getLockInfoEPxP7QStringS2_()};
    let arg0 = self.0  as *mut c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK9QLockFile11getLockInfoEPxP7QStringS2_(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QLockFile::NewQLockFile(const QString & fileName);
impl<'a> /*trait*/ QLockFile_NewQLockFile for (&'a  QString) {
  fn NewQLockFile(self) -> QLockFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QLockFileC1ERK7QString(qthis, arg0)};
    let rsthis = QLockFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

