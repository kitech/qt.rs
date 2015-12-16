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
  // proto:  void QThreadPool::FreeQThreadPool();
  fn _ZN11QThreadPoolD0Ev(qthis: *mut c_void) ;
  // proto:  int QThreadPool::expiryTimeout();
  fn _ZNK11QThreadPool13expiryTimeoutEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QThreadPool::waitForDone(int msecs);
  fn _ZN11QThreadPool11waitForDoneEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  const QMetaObject * QThreadPool::metaObject();
  fn _ZNK11QThreadPool10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QThreadPool::cancel(QRunnable * runnable);
  fn _ZN11QThreadPool6cancelEP9QRunnable(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QThreadPool::tryStart(QRunnable * runnable);
  fn _ZN11QThreadPool8tryStartEP9QRunnable(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static QThreadPool * QThreadPool::globalInstance();
  fn _ZN11QThreadPool14globalInstanceEv() -> *mut c_void;
  // proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
  fn _ZN11QThreadPool17setMaxThreadCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
  fn _ZN11QThreadPool16setExpiryTimeoutEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QThreadPool::reserveThread();
  fn _ZN11QThreadPool13reserveThreadEv(qthis: *mut c_void) ;
  // proto:  void QThreadPool::clear();
  fn _ZN11QThreadPool5clearEv(qthis: *mut c_void) ;
  // proto:  void QThreadPool::NewQThreadPool(QObject * parent);
  fn _ZN11QThreadPoolC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QThreadPool::start(QRunnable * runnable, int priority);
  fn _ZN11QThreadPool5startEP9QRunnablei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  int QThreadPool::maxThreadCount();
  fn _ZNK11QThreadPool14maxThreadCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QThreadPool::releaseThread();
  fn _ZN11QThreadPool13releaseThreadEv(qthis: *mut c_void) ;
  // proto:  int QThreadPool::activeThreadCount();
  fn _ZNK11QThreadPool17activeThreadCountEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QThreadPool)=1
pub struct QThreadPool {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QThreadPool {
  pub fn FreeQThreadPool<T: QThreadPool_FreeQThreadPool>(&mut self, value: T)  {
     value.FreeQThreadPool(self);
    // return 1;
  }
}

pub trait QThreadPool_FreeQThreadPool {
  fn FreeQThreadPool(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::FreeQThreadPool();
impl<'a> /*trait*/ QThreadPool_FreeQThreadPool for () {
  fn FreeQThreadPool(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPoolD0Ev()};
     unsafe {_ZN11QThreadPoolD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn expiryTimeout<T: QThreadPool_expiryTimeout>(&mut self, value: T) -> i32 {
    return value.expiryTimeout(self);
    // return 1;
  }
}

pub trait QThreadPool_expiryTimeout {
  fn expiryTimeout(self, rsthis: &mut QThreadPool) -> i32;
}

// proto:  int QThreadPool::expiryTimeout();
impl<'a> /*trait*/ QThreadPool_expiryTimeout for () {
  fn expiryTimeout(self, rsthis: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool13expiryTimeoutEv()};
    let mut ret = unsafe {_ZNK11QThreadPool13expiryTimeoutEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn waitForDone<T: QThreadPool_waitForDone>(&mut self, value: T) -> i8 {
    return value.waitForDone(self);
    // return 1;
  }
}

pub trait QThreadPool_waitForDone {
  fn waitForDone(self, rsthis: &mut QThreadPool) -> i8;
}

// proto:  bool QThreadPool::waitForDone(int msecs);
impl<'a> /*trait*/ QThreadPool_waitForDone for (i32) {
  fn waitForDone(self, rsthis: &mut QThreadPool) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool11waitForDoneEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QThreadPool11waitForDoneEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn metaObject<T: QThreadPool_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QThreadPool_metaObject {
  fn metaObject(self, rsthis: &mut QThreadPool) ;
}

// proto:  const QMetaObject * QThreadPool::metaObject();
impl<'a> /*trait*/ QThreadPool_metaObject for () {
  fn metaObject(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool10metaObjectEv()};
     unsafe {_ZNK11QThreadPool10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn cancel<T: QThreadPool_cancel>(&mut self, value: T)  {
     value.cancel(self);
    // return 1;
  }
}

pub trait QThreadPool_cancel {
  fn cancel(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::cancel(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_cancel for (&'a mut QRunnable) {
  fn cancel(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool6cancelEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QThreadPool6cancelEP9QRunnable(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn tryStart<T: QThreadPool_tryStart>(&mut self, value: T) -> i8 {
    return value.tryStart(self);
    // return 1;
  }
}

pub trait QThreadPool_tryStart {
  fn tryStart(self, rsthis: &mut QThreadPool) -> i8;
}

// proto:  bool QThreadPool::tryStart(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_tryStart for (&'a mut QRunnable) {
  fn tryStart(self, rsthis: &mut QThreadPool) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool8tryStartEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QThreadPool8tryStartEP9QRunnable(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn globalInstance<T: QThreadPool_globalInstance>(&mut self, value: T) -> QThreadPool {
    return value.globalInstance(self);
    // return 1;
  }
}

pub trait QThreadPool_globalInstance {
  fn globalInstance(self, rsthis: &mut QThreadPool) -> QThreadPool;
}

// proto: static QThreadPool * QThreadPool::globalInstance();
impl<'a> /*trait*/ QThreadPool_globalInstance for () {
  fn globalInstance(self, rsthis: &mut QThreadPool) -> QThreadPool {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool14globalInstanceEv()};
    let mut ret = unsafe {_ZN11QThreadPool14globalInstanceEv()};
    let mut ret1 = QThreadPool{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn setMaxThreadCount<T: QThreadPool_setMaxThreadCount>(&mut self, value: T)  {
     value.setMaxThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_setMaxThreadCount {
  fn setMaxThreadCount(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
impl<'a> /*trait*/ QThreadPool_setMaxThreadCount for (i32) {
  fn setMaxThreadCount(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool17setMaxThreadCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QThreadPool17setMaxThreadCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn setExpiryTimeout<T: QThreadPool_setExpiryTimeout>(&mut self, value: T)  {
     value.setExpiryTimeout(self);
    // return 1;
  }
}

pub trait QThreadPool_setExpiryTimeout {
  fn setExpiryTimeout(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
impl<'a> /*trait*/ QThreadPool_setExpiryTimeout for (i32) {
  fn setExpiryTimeout(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool16setExpiryTimeoutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QThreadPool16setExpiryTimeoutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn reserveThread<T: QThreadPool_reserveThread>(&mut self, value: T)  {
     value.reserveThread(self);
    // return 1;
  }
}

pub trait QThreadPool_reserveThread {
  fn reserveThread(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::reserveThread();
impl<'a> /*trait*/ QThreadPool_reserveThread for () {
  fn reserveThread(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13reserveThreadEv()};
     unsafe {_ZN11QThreadPool13reserveThreadEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn clear<T: QThreadPool_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QThreadPool_clear {
  fn clear(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::clear();
impl<'a> /*trait*/ QThreadPool_clear for () {
  fn clear(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5clearEv()};
     unsafe {_ZN11QThreadPool5clearEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn start<T: QThreadPool_start>(&mut self, value: T)  {
     value.start(self);
    // return 1;
  }
}

pub trait QThreadPool_start {
  fn start(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::start(QRunnable * runnable, int priority);
impl<'a> /*trait*/ QThreadPool_start for (&'a mut QRunnable, i32) {
  fn start(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5startEP9QRunnablei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QThreadPool5startEP9QRunnablei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn maxThreadCount<T: QThreadPool_maxThreadCount>(&mut self, value: T) -> i32 {
    return value.maxThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_maxThreadCount {
  fn maxThreadCount(self, rsthis: &mut QThreadPool) -> i32;
}

// proto:  int QThreadPool::maxThreadCount();
impl<'a> /*trait*/ QThreadPool_maxThreadCount for () {
  fn maxThreadCount(self, rsthis: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool14maxThreadCountEv()};
    let mut ret = unsafe {_ZNK11QThreadPool14maxThreadCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn releaseThread<T: QThreadPool_releaseThread>(&mut self, value: T)  {
     value.releaseThread(self);
    // return 1;
  }
}

pub trait QThreadPool_releaseThread {
  fn releaseThread(self, rsthis: &mut QThreadPool) ;
}

// proto:  void QThreadPool::releaseThread();
impl<'a> /*trait*/ QThreadPool_releaseThread for () {
  fn releaseThread(self, rsthis: &mut QThreadPool)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13releaseThreadEv()};
     unsafe {_ZN11QThreadPool13releaseThreadEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThreadPool {
  pub fn activeThreadCount<T: QThreadPool_activeThreadCount>(&mut self, value: T) -> i32 {
    return value.activeThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_activeThreadCount {
  fn activeThreadCount(self, rsthis: &mut QThreadPool) -> i32;
}

// proto:  int QThreadPool::activeThreadCount();
impl<'a> /*trait*/ QThreadPool_activeThreadCount for () {
  fn activeThreadCount(self, rsthis: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool17activeThreadCountEv()};
    let mut ret = unsafe {_ZNK11QThreadPool17activeThreadCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

