// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qreadwritelock::QReadWriteLock;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QReadWriteLock * QReadLocker::readWriteLock();
  fn _ZNK11QReadLocker13readWriteLockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QReadLocker::FreeQReadLocker();
  fn _ZN11QReadLockerD0Ev(qthis: *mut c_void) ;
  // proto:  void QReadLocker::NewQReadLocker(QReadWriteLock * readWriteLock);
  fn _ZN11QReadLockerC1EP14QReadWriteLock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QReadLocker::NewQReadLocker(const QReadLocker & );
  fn _ZN11QReadLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QReadLocker::relock();
  fn _ZN11QReadLocker6relockEv(qthis: *mut c_void) ;
  // proto:  void QReadLocker::unlock();
  fn _ZN11QReadLocker6unlockEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QReadLocker)=4
pub struct QReadLocker {
  pub qclsinst: *mut c_void,
}

// proto:  QReadWriteLock * QReadLocker::readWriteLock();
impl /*struct*/ QReadLocker {
  pub fn readWriteLock<RetType, T: QReadLocker_readWriteLock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.readWriteLock(self);
    // return 1;
  }
}

pub trait QReadLocker_readWriteLock<RetType> {
  fn readWriteLock(self , rsthis: &mut QReadLocker) -> RetType;
}

// proto:  QReadWriteLock * QReadLocker::readWriteLock();
impl<'a> /*trait*/ QReadLocker_readWriteLock<QReadWriteLock> for () {
  fn readWriteLock(self , rsthis: &mut QReadLocker) -> QReadWriteLock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QReadLocker13readWriteLockEv()};
    let mut ret = unsafe {_ZNK11QReadLocker13readWriteLockEv(rsthis.qclsinst)};
    let mut ret1 = QReadWriteLock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QReadLocker::FreeQReadLocker();
impl /*struct*/ QReadLocker {
  pub fn FreeQReadLocker<RetType, T: QReadLocker_FreeQReadLocker<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQReadLocker(self);
    // return 1;
  }
}

pub trait QReadLocker_FreeQReadLocker<RetType> {
  fn FreeQReadLocker(self , rsthis: &mut QReadLocker) -> RetType;
}

// proto:  void QReadLocker::FreeQReadLocker();
impl<'a> /*trait*/ QReadLocker_FreeQReadLocker<()> for () {
  fn FreeQReadLocker(self , rsthis: &mut QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerD0Ev()};
     unsafe {_ZN11QReadLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn NewQReadLocker<T: QReadLocker_NewQReadLocker>(value: T) -> QReadLocker {
    let rsthis = value.NewQReadLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QReadLocker_NewQReadLocker {
  fn NewQReadLocker(self) -> QReadLocker;
}

// proto: void QReadLocker::NewQReadLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QReadLocker_NewQReadLocker for (&'a mut QReadWriteLock) {
  fn NewQReadLocker(self) -> QReadLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC1EP14QReadWriteLock()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QReadLockerC1EP14QReadWriteLock(qthis, arg0)};
    let rsthis = QReadLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QReadLocker::NewQReadLocker(const QReadLocker & );
impl<'a> /*trait*/ QReadLocker_NewQReadLocker for (&'a  QReadLocker) {
  fn NewQReadLocker(self) -> QReadLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QReadLockerC1ERKS_(qthis, arg0)};
    let rsthis = QReadLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QReadLocker::relock();
impl /*struct*/ QReadLocker {
  pub fn relock<RetType, T: QReadLocker_relock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QReadLocker_relock<RetType> {
  fn relock(self , rsthis: &mut QReadLocker) -> RetType;
}

// proto:  void QReadLocker::relock();
impl<'a> /*trait*/ QReadLocker_relock<()> for () {
  fn relock(self , rsthis: &mut QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLocker6relockEv()};
     unsafe {_ZN11QReadLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QReadLocker::unlock();
impl /*struct*/ QReadLocker {
  pub fn unlock<RetType, T: QReadLocker_unlock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QReadLocker_unlock<RetType> {
  fn unlock(self , rsthis: &mut QReadLocker) -> RetType;
}

// proto:  void QReadLocker::unlock();
impl<'a> /*trait*/ QReadLocker_unlock<()> for () {
  fn unlock(self , rsthis: &mut QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLocker6unlockEv()};
     unsafe {_ZN11QReadLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

