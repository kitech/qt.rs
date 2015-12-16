// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMutex::FreeQMutex();
  fn _ZN6QMutexD0Ev(qthis: *mut c_void) ;
  // proto:  bool QMutex::tryLock(int timeout);
  fn _ZN6QMutex7tryLockEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QMutex::NewQMutex(const QMutex & );
  fn _ZN6QMutexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMutex::lock();
  fn _ZN6QMutex4lockEv(qthis: *mut c_void) ;
  // proto:  void QMutex::unlock();
  fn _ZN6QMutex6unlockEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QMutex)=1
pub struct QMutex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMutex {
  pub fn FreeQMutex<T: QMutex_FreeQMutex>(&mut self, value: T)  {
     value.FreeQMutex(self);
    // return 1;
  }
}

pub trait QMutex_FreeQMutex {
  fn FreeQMutex(self, rsthis: &mut QMutex) ;
}

// proto:  void QMutex::FreeQMutex();
impl<'a> /*trait*/ QMutex_FreeQMutex for () {
  fn FreeQMutex(self, rsthis: &mut QMutex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexD0Ev()};
     unsafe {_ZN6QMutexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMutex {
  pub fn tryLock<T: QMutex_tryLock>(&mut self, value: T) -> i8 {
    return value.tryLock(self);
    // return 1;
  }
}

pub trait QMutex_tryLock {
  fn tryLock(self, rsthis: &mut QMutex) -> i8;
}

// proto:  bool QMutex::tryLock(int timeout);
impl<'a> /*trait*/ QMutex_tryLock for (i32) {
  fn tryLock(self, rsthis: &mut QMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex7tryLockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QMutex7tryLockEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMutex {
  pub fn NewQMutex<T: QMutex_NewQMutex>(value: T) -> QMutex {
    let rsthis = value.NewQMutex();
    return rsthis;
    // return 1;
  }
}

pub trait QMutex_NewQMutex {
  fn NewQMutex(self) -> QMutex;
}

// proto: void QMutex::NewQMutex(const QMutex & );
impl<'a> /*trait*/ QMutex_NewQMutex for (&'a  QMutex) {
  fn NewQMutex(self) -> QMutex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMutexC1ERKS_(qthis, arg0)};
    let rsthis = QMutex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMutex {
  pub fn lock<T: QMutex_lock>(&mut self, value: T)  {
     value.lock(self);
    // return 1;
  }
}

pub trait QMutex_lock {
  fn lock(self, rsthis: &mut QMutex) ;
}

// proto:  void QMutex::lock();
impl<'a> /*trait*/ QMutex_lock for () {
  fn lock(self, rsthis: &mut QMutex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex4lockEv()};
     unsafe {_ZN6QMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMutex {
  pub fn unlock<T: QMutex_unlock>(&mut self, value: T)  {
     value.unlock(self);
    // return 1;
  }
}

pub trait QMutex_unlock {
  fn unlock(self, rsthis: &mut QMutex) ;
}

// proto:  void QMutex::unlock();
impl<'a> /*trait*/ QMutex_unlock for () {
  fn unlock(self, rsthis: &mut QMutex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex6unlockEv()};
     unsafe {_ZN6QMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

