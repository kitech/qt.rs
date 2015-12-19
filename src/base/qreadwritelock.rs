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
  // proto:  void QReadWriteLock::~QReadWriteLock();
  fn _ZN14QReadWriteLockD0Ev(qthis: *mut c_void);
  // proto:  void QReadWriteLock::QReadWriteLock(const QReadWriteLock & );
  fn _ZN14QReadWriteLockC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForRead();
  fn _ZN14QReadWriteLock14tryLockForReadEv(qthis: *mut c_void) -> c_char;
  // proto:  void QReadWriteLock::lockForWrite();
  fn _ZN14QReadWriteLock12lockForWriteEv(qthis: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForWrite();
  fn _ZN14QReadWriteLock15tryLockForWriteEv(qthis: *mut c_void) -> c_char;
  // proto:  void QReadWriteLock::unlock();
  fn _ZN14QReadWriteLock6unlockEv(qthis: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForRead(int timeout);
  fn _ZN14QReadWriteLock14tryLockForReadEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QReadWriteLock::lockForRead();
  fn _ZN14QReadWriteLock11lockForReadEv(qthis: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForWrite(int timeout);
  fn _ZN14QReadWriteLock15tryLockForWriteEi(qthis: *mut c_void, arg0: c_int) -> c_char;
}

// body block begin
// class sizeof(QReadWriteLock)=8
pub struct QReadWriteLock {
  pub qclsinst: *mut c_void,
}

  // proto:  void QReadWriteLock::~QReadWriteLock();
impl /*struct*/ QReadWriteLock {
  pub fn FreeQReadWriteLock<RetType, T: QReadWriteLock_FreeQReadWriteLock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQReadWriteLock(self);
    // return 1;
  }
}

pub trait QReadWriteLock_FreeQReadWriteLock<RetType> {
  fn FreeQReadWriteLock(self , rsthis: &mut QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::~QReadWriteLock();
impl<'a> /*trait*/ QReadWriteLock_FreeQReadWriteLock<()> for () {
  fn FreeQReadWriteLock(self , rsthis: &mut QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockD0Ev()};
     unsafe {_ZN14QReadWriteLockD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QReadWriteLock::QReadWriteLock(const QReadWriteLock & );
impl /*struct*/ QReadWriteLock {
  pub fn NewQReadWriteLock<T: QReadWriteLock_NewQReadWriteLock>(value: T) -> QReadWriteLock {
    let rsthis = value.NewQReadWriteLock();
    return rsthis;
    // return 1;
  }
}

pub trait QReadWriteLock_NewQReadWriteLock {
  fn NewQReadWriteLock(self) -> QReadWriteLock;
}

  // proto:  void QReadWriteLock::QReadWriteLock(const QReadWriteLock & );
impl<'a> /*trait*/ QReadWriteLock_NewQReadWriteLock for (QReadWriteLock) {
  fn NewQReadWriteLock(self) -> QReadWriteLock {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QReadWriteLockC1ERKS_(qthis, arg0)};
    let rsthis = QReadWriteLock{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForRead();
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForRead<RetType, T: QReadWriteLock_tryLockForRead<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tryLockForRead(self);
    // return 1;
  }
}

pub trait QReadWriteLock_tryLockForRead<RetType> {
  fn tryLockForRead(self , rsthis: &mut QReadWriteLock) -> RetType;
}

  // proto:  bool QReadWriteLock::tryLockForRead();
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead<i8> for () {
  fn tryLockForRead(self , rsthis: &mut QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEv()};
    let mut ret = unsafe {_ZN14QReadWriteLock14tryLockForReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QReadWriteLock::lockForWrite();
impl /*struct*/ QReadWriteLock {
  pub fn lockForWrite<RetType, T: QReadWriteLock_lockForWrite<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lockForWrite(self);
    // return 1;
  }
}

pub trait QReadWriteLock_lockForWrite<RetType> {
  fn lockForWrite(self , rsthis: &mut QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::lockForWrite();
impl<'a> /*trait*/ QReadWriteLock_lockForWrite<()> for () {
  fn lockForWrite(self , rsthis: &mut QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock12lockForWriteEv()};
     unsafe {_ZN14QReadWriteLock12lockForWriteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForWrite();
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForWrite<RetType, T: QReadWriteLock_tryLockForWrite<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tryLockForWrite(self);
    // return 1;
  }
}

pub trait QReadWriteLock_tryLockForWrite<RetType> {
  fn tryLockForWrite(self , rsthis: &mut QReadWriteLock) -> RetType;
}

  // proto:  bool QReadWriteLock::tryLockForWrite();
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite<i8> for () {
  fn tryLockForWrite(self , rsthis: &mut QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEv()};
    let mut ret = unsafe {_ZN14QReadWriteLock15tryLockForWriteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QReadWriteLock::unlock();
impl /*struct*/ QReadWriteLock {
  pub fn unlock<RetType, T: QReadWriteLock_unlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QReadWriteLock_unlock<RetType> {
  fn unlock(self , rsthis: &mut QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::unlock();
impl<'a> /*trait*/ QReadWriteLock_unlock<()> for () {
  fn unlock(self , rsthis: &mut QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock6unlockEv()};
     unsafe {_ZN14QReadWriteLock6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForRead(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead<i8> for (i32) {
  fn tryLockForRead(self , rsthis: &mut QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QReadWriteLock14tryLockForReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QReadWriteLock::lockForRead();
impl /*struct*/ QReadWriteLock {
  pub fn lockForRead<RetType, T: QReadWriteLock_lockForRead<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lockForRead(self);
    // return 1;
  }
}

pub trait QReadWriteLock_lockForRead<RetType> {
  fn lockForRead(self , rsthis: &mut QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::lockForRead();
impl<'a> /*trait*/ QReadWriteLock_lockForRead<()> for () {
  fn lockForRead(self , rsthis: &mut QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock11lockForReadEv()};
     unsafe {_ZN14QReadWriteLock11lockForReadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForWrite(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite<i8> for (i32) {
  fn tryLockForWrite(self , rsthis: &mut QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QReadWriteLock15tryLockForWriteEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

