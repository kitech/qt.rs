// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qthread::QThread;
use super::qeventloop::QEventLoop;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
  fn _ZN16QEventLoopLockerC1EP7QThread(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventLoopLocker::QEventLoopLocker(QEventLoop * loop);
  fn _ZN16QEventLoopLockerC1EP10QEventLoop(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventLoopLocker::QEventLoopLocker();
  fn _ZN16QEventLoopLockerC1Ev(qthis: *mut c_void);
  // proto:  void QEventLoopLocker::QEventLoopLocker(const QEventLoopLocker & );
  fn _ZN16QEventLoopLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventLoopLocker::~QEventLoopLocker();
  fn _ZN16QEventLoopLockerD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QEventLoopLocker)=8
pub struct QEventLoopLocker {
  pub qclsinst: *mut c_void,
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl /*struct*/ QEventLoopLocker {
  pub fn NewQEventLoopLocker<T: QEventLoopLocker_NewQEventLoopLocker>(value: T) -> QEventLoopLocker {
    let rsthis = value.NewQEventLoopLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_NewQEventLoopLocker {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker;
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for (QThread) {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1EP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventLoopLockerC1EP7QThread(qthis, arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QEventLoop * loop);
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for (QEventLoop) {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1EP10QEventLoop()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventLoopLockerC1EP10QEventLoop(qthis, arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for () {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1Ev()};
    unsafe {_ZN16QEventLoopLockerC1Ev(qthis)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(const QEventLoopLocker & );
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for (QEventLoopLocker) {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventLoopLockerC1ERKS_(qthis, arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl /*struct*/ QEventLoopLocker {
  pub fn FreeQEventLoopLocker<RetType, T: QEventLoopLocker_FreeQEventLoopLocker<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQEventLoopLocker(self);
    // return 1;
  }
}

pub trait QEventLoopLocker_FreeQEventLoopLocker<RetType> {
  fn FreeQEventLoopLocker(self , rsthis: &mut QEventLoopLocker) -> RetType;
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_FreeQEventLoopLocker<()> for () {
  fn FreeQEventLoopLocker(self , rsthis: &mut QEventLoopLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerD0Ev()};
     unsafe {_ZN16QEventLoopLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

