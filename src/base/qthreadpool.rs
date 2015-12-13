// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrunnable::QRunnable;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN11QThreadPoolD0Ev() -> i32;
  fn _ZNK11QThreadPool13expiryTimeoutEv() -> i32;
  fn _ZN11QThreadPool11waitForDoneEi(arg0: c_int) -> i32;
  fn _ZNK11QThreadPool10metaObjectEv() -> i32;
  fn _ZN11QThreadPool6cancelEP9QRunnable(arg0: *mut c_void) -> i32;
  fn _ZN11QThreadPool8tryStartEP9QRunnable(arg0: *mut c_void) -> i32;
  fn _ZN11QThreadPool14globalInstanceEv() -> i32;
  fn _ZN11QThreadPool17setMaxThreadCountEi(arg0: c_int) -> i32;
  fn _ZN11QThreadPool16setExpiryTimeoutEi(arg0: c_int) -> i32;
  fn _ZN11QThreadPool13reserveThreadEv() -> i32;
  fn _ZN11QThreadPool5clearEv() -> i32;
  fn _ZN11QThreadPoolC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN11QThreadPool5startEP9QRunnablei(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK11QThreadPool14maxThreadCountEv() -> i32;
  fn _ZN11QThreadPool13releaseThreadEv() -> i32;
  fn _ZNK11QThreadPool17activeThreadCountEv() -> i32;
}

// body block begin
// class sizeof(QThreadPool)=1
pub struct QThreadPool {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QThreadPool {
  pub fn FreeQThreadPool<T: QThreadPool_FreeQThreadPool>(&mut self, value: T) -> i32 {
    value.FreeQThreadPool(self);
    return 1;
  }
}

pub trait QThreadPool_FreeQThreadPool {
  fn FreeQThreadPool(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::FreeQThreadPool();
impl<'a> /*trait*/ QThreadPool_FreeQThreadPool for () {
  fn FreeQThreadPool(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPoolD0Ev()};
    unsafe {_ZN11QThreadPoolD0Ev()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn expiryTimeout<T: QThreadPool_expiryTimeout>(&mut self, value: T) -> i32 {
    value.expiryTimeout(self);
    return 1;
  }
}

pub trait QThreadPool_expiryTimeout {
  fn expiryTimeout(self, this: &mut QThreadPool) -> i32;
}

// proto: int QThreadPool::expiryTimeout();
impl<'a> /*trait*/ QThreadPool_expiryTimeout for () {
  fn expiryTimeout(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool13expiryTimeoutEv()};
    unsafe {_ZNK11QThreadPool13expiryTimeoutEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn waitForDone<T: QThreadPool_waitForDone>(&mut self, value: T) -> i32 {
    value.waitForDone(self);
    return 1;
  }
}

pub trait QThreadPool_waitForDone {
  fn waitForDone(self, this: &mut QThreadPool) -> i32;
}

// proto: bool QThreadPool::waitForDone(int msecs);
impl<'a> /*trait*/ QThreadPool_waitForDone for (i32) {
  fn waitForDone(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool11waitForDoneEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QThreadPool11waitForDoneEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn metaObject<T: QThreadPool_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QThreadPool_metaObject {
  fn metaObject(self, this: &mut QThreadPool) -> i32;
}

// proto: const QMetaObject * QThreadPool::metaObject();
impl<'a> /*trait*/ QThreadPool_metaObject for () {
  fn metaObject(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool10metaObjectEv()};
    unsafe {_ZNK11QThreadPool10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn cancel<T: QThreadPool_cancel>(&mut self, value: T) -> i32 {
    value.cancel(self);
    return 1;
  }
}

pub trait QThreadPool_cancel {
  fn cancel(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::cancel(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_cancel for (&'a mut QRunnable) {
  fn cancel(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool6cancelEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QThreadPool6cancelEP9QRunnable(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn tryStart<T: QThreadPool_tryStart>(&mut self, value: T) -> i32 {
    value.tryStart(self);
    return 1;
  }
}

pub trait QThreadPool_tryStart {
  fn tryStart(self, this: &mut QThreadPool) -> i32;
}

// proto: bool QThreadPool::tryStart(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_tryStart for (&'a mut QRunnable) {
  fn tryStart(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool8tryStartEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QThreadPool8tryStartEP9QRunnable(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn globalInstance<T: QThreadPool_globalInstance>(&mut self, value: T) -> i32 {
    value.globalInstance(self);
    return 1;
  }
}

pub trait QThreadPool_globalInstance {
  fn globalInstance(self, this: &mut QThreadPool) -> i32;
}

// proto: QThreadPool * QThreadPool::globalInstance();
impl<'a> /*trait*/ QThreadPool_globalInstance for () {
  fn globalInstance(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool14globalInstanceEv()};
    unsafe {_ZN11QThreadPool14globalInstanceEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn setMaxThreadCount<T: QThreadPool_setMaxThreadCount>(&mut self, value: T) -> i32 {
    value.setMaxThreadCount(self);
    return 1;
  }
}

pub trait QThreadPool_setMaxThreadCount {
  fn setMaxThreadCount(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::setMaxThreadCount(int maxThreadCount);
impl<'a> /*trait*/ QThreadPool_setMaxThreadCount for (i32) {
  fn setMaxThreadCount(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool17setMaxThreadCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QThreadPool17setMaxThreadCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn setExpiryTimeout<T: QThreadPool_setExpiryTimeout>(&mut self, value: T) -> i32 {
    value.setExpiryTimeout(self);
    return 1;
  }
}

pub trait QThreadPool_setExpiryTimeout {
  fn setExpiryTimeout(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::setExpiryTimeout(int expiryTimeout);
impl<'a> /*trait*/ QThreadPool_setExpiryTimeout for (i32) {
  fn setExpiryTimeout(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool16setExpiryTimeoutEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QThreadPool16setExpiryTimeoutEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn reserveThread<T: QThreadPool_reserveThread>(&mut self, value: T) -> i32 {
    value.reserveThread(self);
    return 1;
  }
}

pub trait QThreadPool_reserveThread {
  fn reserveThread(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::reserveThread();
impl<'a> /*trait*/ QThreadPool_reserveThread for () {
  fn reserveThread(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13reserveThreadEv()};
    unsafe {_ZN11QThreadPool13reserveThreadEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn clear<T: QThreadPool_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QThreadPool_clear {
  fn clear(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::clear();
impl<'a> /*trait*/ QThreadPool_clear for () {
  fn clear(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5clearEv()};
    unsafe {_ZN11QThreadPool5clearEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn NewQThreadPool<T: QThreadPool_NewQThreadPool>(value: T) -> QThreadPool {
    let rsthis = value.NewQThreadPool();
    return rsthis;
    // return 1;
  }
}

pub trait QThreadPool_NewQThreadPool {
  fn NewQThreadPool(self) -> QThreadPool;
}

// proto: void QThreadPool::NewQThreadPool(QObject * parent);
impl<'a> /*trait*/ QThreadPool_NewQThreadPool for (&'a mut QObject) {
  fn NewQThreadPool(self) -> QThreadPool {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPoolC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QThreadPoolC1EP7QObject(qthis, arg0)};
    let rsthis = QThreadPool{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn start<T: QThreadPool_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QThreadPool_start {
  fn start(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::start(QRunnable * runnable, int priority);
impl<'a> /*trait*/ QThreadPool_start for (&'a mut QRunnable, i32) {
  fn start(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5startEP9QRunnablei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QThreadPool5startEP9QRunnablei(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn maxThreadCount<T: QThreadPool_maxThreadCount>(&mut self, value: T) -> i32 {
    value.maxThreadCount(self);
    return 1;
  }
}

pub trait QThreadPool_maxThreadCount {
  fn maxThreadCount(self, this: &mut QThreadPool) -> i32;
}

// proto: int QThreadPool::maxThreadCount();
impl<'a> /*trait*/ QThreadPool_maxThreadCount for () {
  fn maxThreadCount(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool14maxThreadCountEv()};
    unsafe {_ZNK11QThreadPool14maxThreadCountEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn releaseThread<T: QThreadPool_releaseThread>(&mut self, value: T) -> i32 {
    value.releaseThread(self);
    return 1;
  }
}

pub trait QThreadPool_releaseThread {
  fn releaseThread(self, this: &mut QThreadPool) -> i32;
}

// proto: void QThreadPool::releaseThread();
impl<'a> /*trait*/ QThreadPool_releaseThread for () {
  fn releaseThread(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13releaseThreadEv()};
    unsafe {_ZN11QThreadPool13releaseThreadEv()};
    return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn activeThreadCount<T: QThreadPool_activeThreadCount>(&mut self, value: T) -> i32 {
    value.activeThreadCount(self);
    return 1;
  }
}

pub trait QThreadPool_activeThreadCount {
  fn activeThreadCount(self, this: &mut QThreadPool) -> i32;
}

// proto: int QThreadPool::activeThreadCount();
impl<'a> /*trait*/ QThreadPool_activeThreadCount for () {
  fn activeThreadCount(self, this: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool17activeThreadCountEv()};
    unsafe {_ZNK11QThreadPool17activeThreadCountEv()};
    return 1;
  }
}

