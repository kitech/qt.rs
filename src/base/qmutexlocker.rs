// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbasicmutex::QBasicMutex;
use super::qmutex::QMutex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMutexLocker::NewQMutexLocker(QBasicMutex * m);
  fn _ZN12QMutexLockerC1EP11QBasicMutex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMutex * QMutexLocker::mutex();
  fn _ZNK12QMutexLocker5mutexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMutexLocker::NewQMutexLocker(const QMutexLocker & );
  fn _ZN12QMutexLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMutexLocker::relock();
  fn _ZN12QMutexLocker6relockEv(qthis: *mut c_void) ;
  // proto:  void QMutexLocker::unlock();
  fn _ZN12QMutexLocker6unlockEv(qthis: *mut c_void) ;
  // proto:  void QMutexLocker::FreeQMutexLocker();
  fn _ZN12QMutexLockerD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QMutexLocker)=4
pub struct QMutexLocker {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMutexLocker {
  pub fn NewQMutexLocker<T: QMutexLocker_NewQMutexLocker>(value: T) -> QMutexLocker {
    let rsthis = value.NewQMutexLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QMutexLocker_NewQMutexLocker {
  fn NewQMutexLocker(self) -> QMutexLocker;
}

// proto: void QMutexLocker::NewQMutexLocker(QBasicMutex * m);
impl<'a> /*trait*/ QMutexLocker_NewQMutexLocker for (&'a mut QBasicMutex) {
  fn NewQMutexLocker(self) -> QMutexLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1EP11QBasicMutex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QMutexLockerC1EP11QBasicMutex(qthis, arg0)};
    let rsthis = QMutexLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QMutex * QMutexLocker::mutex();
impl /*struct*/ QMutexLocker {
  pub fn mutex<RetType, T: QMutexLocker_mutex<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mutex(self);
    // return 1;
  }
}

pub trait QMutexLocker_mutex<RetType> {
  fn mutex(self , rsthis: &mut QMutexLocker) -> RetType;
}

// proto:  QMutex * QMutexLocker::mutex();
impl<'a> /*trait*/ QMutexLocker_mutex<QMutex> for () {
  fn mutex(self , rsthis: &mut QMutexLocker) -> QMutex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QMutexLocker5mutexEv()};
    let mut ret = unsafe {_ZNK12QMutexLocker5mutexEv(rsthis.qclsinst)};
    let mut ret1 = QMutex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QMutexLocker::NewQMutexLocker(const QMutexLocker & );
impl<'a> /*trait*/ QMutexLocker_NewQMutexLocker for (&'a  QMutexLocker) {
  fn NewQMutexLocker(self) -> QMutexLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QMutexLockerC1ERKS_(qthis, arg0)};
    let rsthis = QMutexLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QMutexLocker::relock();
impl /*struct*/ QMutexLocker {
  pub fn relock<RetType, T: QMutexLocker_relock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QMutexLocker_relock<RetType> {
  fn relock(self , rsthis: &mut QMutexLocker) -> RetType;
}

// proto:  void QMutexLocker::relock();
impl<'a> /*trait*/ QMutexLocker_relock<()> for () {
  fn relock(self , rsthis: &mut QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6relockEv()};
     unsafe {_ZN12QMutexLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QMutexLocker::unlock();
impl /*struct*/ QMutexLocker {
  pub fn unlock<RetType, T: QMutexLocker_unlock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QMutexLocker_unlock<RetType> {
  fn unlock(self , rsthis: &mut QMutexLocker) -> RetType;
}

// proto:  void QMutexLocker::unlock();
impl<'a> /*trait*/ QMutexLocker_unlock<()> for () {
  fn unlock(self , rsthis: &mut QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6unlockEv()};
     unsafe {_ZN12QMutexLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QMutexLocker::FreeQMutexLocker();
impl /*struct*/ QMutexLocker {
  pub fn FreeQMutexLocker<RetType, T: QMutexLocker_FreeQMutexLocker<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQMutexLocker(self);
    // return 1;
  }
}

pub trait QMutexLocker_FreeQMutexLocker<RetType> {
  fn FreeQMutexLocker(self , rsthis: &mut QMutexLocker) -> RetType;
}

// proto:  void QMutexLocker::FreeQMutexLocker();
impl<'a> /*trait*/ QMutexLocker_FreeQMutexLocker<()> for () {
  fn FreeQMutexLocker(self , rsthis: &mut QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerD0Ev()};
     unsafe {_ZN12QMutexLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

