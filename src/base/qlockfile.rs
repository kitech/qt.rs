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
  // proto:  bool QLockFile::removeStaleLockFile();
  fn _ZN9QLockFile19removeStaleLockFileEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QLockFile::staleLockTime();
  fn _ZNK9QLockFile13staleLockTimeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QLockFile::isLocked();
  fn _ZNK9QLockFile8isLockedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLockFile::NewQLockFile(const QLockFile & );
  fn _ZN9QLockFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLockFile::FreeQLockFile();
  fn _ZN9QLockFileD0Ev(qthis: *mut c_void) ;
  // proto:  void QLockFile::unlock();
  fn _ZN9QLockFile6unlockEv(qthis: *mut c_void) ;
  // proto:  bool QLockFile::tryLock(int timeout);
  fn _ZN9QLockFile7tryLockEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  bool QLockFile::lock();
  fn _ZN9QLockFile4lockEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLockFile::setStaleLockTime(int );
  fn _ZN9QLockFile16setStaleLockTimeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QLockFile::getLockInfo(qint64 * pid, QString * hostname, QString * appname);
  fn _ZNK9QLockFile11getLockInfoEPxP7QStringS2_(qthis: *mut c_void, arg0: *mut c_longlong, arg1: *mut c_void, arg2: *mut c_void) -> int8_t;
  // proto:  void QLockFile::NewQLockFile(const QString & fileName);
  fn _ZN9QLockFileC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QLockFile)=1
pub struct QLockFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLockFile {
  pub fn removeStaleLockFile<T: QLockFile_removeStaleLockFile>(&mut self, value: T) -> i8 {
    return value.removeStaleLockFile(self);
    // return 1;
  }
}

pub trait QLockFile_removeStaleLockFile {
  fn removeStaleLockFile(self, rsthis: &mut QLockFile) -> i8;
}

// proto:  bool QLockFile::removeStaleLockFile();
impl<'a> /*trait*/ QLockFile_removeStaleLockFile for () {
  fn removeStaleLockFile(self, rsthis: &mut QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile19removeStaleLockFileEv()};
    let mut ret = unsafe {_ZN9QLockFile19removeStaleLockFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn staleLockTime<T: QLockFile_staleLockTime>(&mut self, value: T) -> i32 {
    return value.staleLockTime(self);
    // return 1;
  }
}

pub trait QLockFile_staleLockTime {
  fn staleLockTime(self, rsthis: &mut QLockFile) -> i32;
}

// proto:  int QLockFile::staleLockTime();
impl<'a> /*trait*/ QLockFile_staleLockTime for () {
  fn staleLockTime(self, rsthis: &mut QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile13staleLockTimeEv()};
    let mut ret = unsafe {_ZNK9QLockFile13staleLockTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn isLocked<T: QLockFile_isLocked>(&mut self, value: T) -> i8 {
    return value.isLocked(self);
    // return 1;
  }
}

pub trait QLockFile_isLocked {
  fn isLocked(self, rsthis: &mut QLockFile) -> i8;
}

// proto:  bool QLockFile::isLocked();
impl<'a> /*trait*/ QLockFile_isLocked for () {
  fn isLocked(self, rsthis: &mut QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile8isLockedEv()};
    let mut ret = unsafe {_ZNK9QLockFile8isLockedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLockFileC1ERKS_(qthis, arg0)};
    let rsthis = QLockFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn FreeQLockFile<T: QLockFile_FreeQLockFile>(&mut self, value: T)  {
     value.FreeQLockFile(self);
    // return 1;
  }
}

pub trait QLockFile_FreeQLockFile {
  fn FreeQLockFile(self, rsthis: &mut QLockFile) ;
}

// proto:  void QLockFile::FreeQLockFile();
impl<'a> /*trait*/ QLockFile_FreeQLockFile for () {
  fn FreeQLockFile(self, rsthis: &mut QLockFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileD0Ev()};
     unsafe {_ZN9QLockFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn unlock<T: QLockFile_unlock>(&mut self, value: T)  {
     value.unlock(self);
    // return 1;
  }
}

pub trait QLockFile_unlock {
  fn unlock(self, rsthis: &mut QLockFile) ;
}

// proto:  void QLockFile::unlock();
impl<'a> /*trait*/ QLockFile_unlock for () {
  fn unlock(self, rsthis: &mut QLockFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile6unlockEv()};
     unsafe {_ZN9QLockFile6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn tryLock<T: QLockFile_tryLock>(&mut self, value: T) -> i8 {
    return value.tryLock(self);
    // return 1;
  }
}

pub trait QLockFile_tryLock {
  fn tryLock(self, rsthis: &mut QLockFile) -> i8;
}

// proto:  bool QLockFile::tryLock(int timeout);
impl<'a> /*trait*/ QLockFile_tryLock for (i32) {
  fn tryLock(self, rsthis: &mut QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile7tryLockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QLockFile7tryLockEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn lock<T: QLockFile_lock>(&mut self, value: T) -> i8 {
    return value.lock(self);
    // return 1;
  }
}

pub trait QLockFile_lock {
  fn lock(self, rsthis: &mut QLockFile) -> i8;
}

// proto:  bool QLockFile::lock();
impl<'a> /*trait*/ QLockFile_lock for () {
  fn lock(self, rsthis: &mut QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile4lockEv()};
    let mut ret = unsafe {_ZN9QLockFile4lockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn setStaleLockTime<T: QLockFile_setStaleLockTime>(&mut self, value: T)  {
     value.setStaleLockTime(self);
    // return 1;
  }
}

pub trait QLockFile_setStaleLockTime {
  fn setStaleLockTime(self, rsthis: &mut QLockFile) ;
}

// proto:  void QLockFile::setStaleLockTime(int );
impl<'a> /*trait*/ QLockFile_setStaleLockTime for (i32) {
  fn setStaleLockTime(self, rsthis: &mut QLockFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFile16setStaleLockTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QLockFile16setStaleLockTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLockFile {
  pub fn getLockInfo<T: QLockFile_getLockInfo>(&mut self, value: T) -> i8 {
    return value.getLockInfo(self);
    // return 1;
  }
}

pub trait QLockFile_getLockInfo {
  fn getLockInfo(self, rsthis: &mut QLockFile) -> i8;
}

// proto:  bool QLockFile::getLockInfo(qint64 * pid, QString * hostname, QString * appname);
impl<'a> /*trait*/ QLockFile_getLockInfo for (&'a mut i64, &'a mut QString, &'a mut QString) {
  fn getLockInfo(self, rsthis: &mut QLockFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QLockFile11getLockInfoEPxP7QStringS2_()};
    let arg0 = self.0  as *mut c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QLockFile11getLockInfoEPxP7QStringS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QLockFile::NewQLockFile(const QString & fileName);
impl<'a> /*trait*/ QLockFile_NewQLockFile for (&'a  QString) {
  fn NewQLockFile(self) -> QLockFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QLockFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QLockFileC1ERK7QString(qthis, arg0)};
    let rsthis = QLockFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

