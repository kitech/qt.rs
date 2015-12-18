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
  // proto:  QString QSystemSemaphore::key();
  fn _ZNK16QSystemSemaphore3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSystemSemaphore::release(int n);
  fn _ZN16QSystemSemaphore7releaseEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QString QSystemSemaphore::errorString();
  fn _ZNK16QSystemSemaphore11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSystemSemaphore::NewQSystemSemaphore(const QSystemSemaphore & );
  fn _ZN16QSystemSemaphoreC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSystemSemaphore::acquire();
  fn _ZN16QSystemSemaphore7acquireEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSystemSemaphore::FreeQSystemSemaphore();
  fn _ZN16QSystemSemaphoreD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QSystemSemaphore)=1
pub struct QSystemSemaphore {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSystemSemaphore {
  pub fn key<RetType, T: QSystemSemaphore_key<RetType>>(&mut self, value: T) -> RetType {
    return value.key(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_key<RetType> {
  fn key(self, rsthis: &mut QSystemSemaphore) -> RetType;
}

// proto:  QString QSystemSemaphore::key();
impl<'a> /*trait*/ QSystemSemaphore_key<QString> for () {
  fn key(self, rsthis: &mut QSystemSemaphore) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSystemSemaphore3keyEv()};
    let mut ret = unsafe {_ZNK16QSystemSemaphore3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn release<RetType, T: QSystemSemaphore_release<RetType>>(&mut self, value: T) -> RetType {
    return value.release(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_release<RetType> {
  fn release(self, rsthis: &mut QSystemSemaphore) -> RetType;
}

// proto:  bool QSystemSemaphore::release(int n);
impl<'a> /*trait*/ QSystemSemaphore_release<i8> for (i32) {
  fn release(self, rsthis: &mut QSystemSemaphore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphore7releaseEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN16QSystemSemaphore7releaseEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn errorString<RetType, T: QSystemSemaphore_errorString<RetType>>(&mut self, value: T) -> RetType {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_errorString<RetType> {
  fn errorString(self, rsthis: &mut QSystemSemaphore) -> RetType;
}

// proto:  QString QSystemSemaphore::errorString();
impl<'a> /*trait*/ QSystemSemaphore_errorString<QString> for () {
  fn errorString(self, rsthis: &mut QSystemSemaphore) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSystemSemaphore11errorStringEv()};
    let mut ret = unsafe {_ZNK16QSystemSemaphore11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn NewQSystemSemaphore<T: QSystemSemaphore_NewQSystemSemaphore>(value: T) -> QSystemSemaphore {
    let rsthis = value.NewQSystemSemaphore();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemSemaphore_NewQSystemSemaphore {
  fn NewQSystemSemaphore(self) -> QSystemSemaphore;
}

// proto: void QSystemSemaphore::NewQSystemSemaphore(const QSystemSemaphore & );
impl<'a> /*trait*/ QSystemSemaphore_NewQSystemSemaphore for (&'a  QSystemSemaphore) {
  fn NewQSystemSemaphore(self) -> QSystemSemaphore {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphoreC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QSystemSemaphoreC1ERKS_(qthis, arg0)};
    let rsthis = QSystemSemaphore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn acquire<RetType, T: QSystemSemaphore_acquire<RetType>>(&mut self, value: T) -> RetType {
    return value.acquire(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_acquire<RetType> {
  fn acquire(self, rsthis: &mut QSystemSemaphore) -> RetType;
}

// proto:  bool QSystemSemaphore::acquire();
impl<'a> /*trait*/ QSystemSemaphore_acquire<i8> for () {
  fn acquire(self, rsthis: &mut QSystemSemaphore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphore7acquireEv()};
    let mut ret = unsafe {_ZN16QSystemSemaphore7acquireEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn FreeQSystemSemaphore<RetType, T: QSystemSemaphore_FreeQSystemSemaphore<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSystemSemaphore(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_FreeQSystemSemaphore<RetType> {
  fn FreeQSystemSemaphore(self, rsthis: &mut QSystemSemaphore) -> RetType;
}

// proto:  void QSystemSemaphore::FreeQSystemSemaphore();
impl<'a> /*trait*/ QSystemSemaphore_FreeQSystemSemaphore<()> for () {
  fn FreeQSystemSemaphore(self, rsthis: &mut QSystemSemaphore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphoreD0Ev()};
     unsafe {_ZN16QSystemSemaphoreD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

