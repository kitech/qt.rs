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

// proto:  void QThreadPool::FreeQThreadPool();
impl /*struct*/ QThreadPool {
  pub fn FreeQThreadPool<RetType, T: QThreadPool_FreeQThreadPool<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQThreadPool(self);
    // return 1;
  }
}

pub trait QThreadPool_FreeQThreadPool<RetType> {
  fn FreeQThreadPool(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::FreeQThreadPool();
impl<'a> /*trait*/ QThreadPool_FreeQThreadPool<()> for () {
  fn FreeQThreadPool(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPoolD0Ev()};
     unsafe {_ZN11QThreadPoolD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QThreadPool::expiryTimeout();
impl /*struct*/ QThreadPool {
  pub fn expiryTimeout<RetType, T: QThreadPool_expiryTimeout<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.expiryTimeout(self);
    // return 1;
  }
}

pub trait QThreadPool_expiryTimeout<RetType> {
  fn expiryTimeout(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  int QThreadPool::expiryTimeout();
impl<'a> /*trait*/ QThreadPool_expiryTimeout<i32> for () {
  fn expiryTimeout(self , rsthis: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool13expiryTimeoutEv()};
    let mut ret = unsafe {_ZNK11QThreadPool13expiryTimeoutEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QThreadPool::waitForDone(int msecs);
impl /*struct*/ QThreadPool {
  pub fn waitForDone<RetType, T: QThreadPool_waitForDone<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.waitForDone(self);
    // return 1;
  }
}

pub trait QThreadPool_waitForDone<RetType> {
  fn waitForDone(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  bool QThreadPool::waitForDone(int msecs);
impl<'a> /*trait*/ QThreadPool_waitForDone<i8> for (i32) {
  fn waitForDone(self , rsthis: &mut QThreadPool) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool11waitForDoneEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QThreadPool11waitForDoneEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  const QMetaObject * QThreadPool::metaObject();
impl /*struct*/ QThreadPool {
  pub fn metaObject<RetType, T: QThreadPool_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QThreadPool_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  const QMetaObject * QThreadPool::metaObject();
impl<'a> /*trait*/ QThreadPool_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool10metaObjectEv()};
     unsafe {_ZNK11QThreadPool10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QThreadPool::cancel(QRunnable * runnable);
impl /*struct*/ QThreadPool {
  pub fn cancel<RetType, T: QThreadPool_cancel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QThreadPool_cancel<RetType> {
  fn cancel(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::cancel(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_cancel<()> for (&'a mut QRunnable) {
  fn cancel(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool6cancelEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QThreadPool6cancelEP9QRunnable(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QThreadPool::tryStart(QRunnable * runnable);
impl /*struct*/ QThreadPool {
  pub fn tryStart<RetType, T: QThreadPool_tryStart<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tryStart(self);
    // return 1;
  }
}

pub trait QThreadPool_tryStart<RetType> {
  fn tryStart(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  bool QThreadPool::tryStart(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_tryStart<i8> for (&'a mut QRunnable) {
  fn tryStart(self , rsthis: &mut QThreadPool) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool8tryStartEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QThreadPool8tryStartEP9QRunnable(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static QThreadPool * QThreadPool::globalInstance();
impl /*struct*/ QThreadPool {
  pub fn globalInstance_s<RetType, T: QThreadPool_globalInstance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalInstance_s();
    // return 1;
  }
}

pub trait QThreadPool_globalInstance_s<RetType> {
  fn globalInstance_s(self ) -> RetType;
}

// proto: static QThreadPool * QThreadPool::globalInstance();
impl<'a> /*trait*/ QThreadPool_globalInstance_s<QThreadPool> for () {
  fn globalInstance_s(self ) -> QThreadPool {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool14globalInstanceEv()};
    let mut ret = unsafe {_ZN11QThreadPool14globalInstanceEv()};
    let mut ret1 = QThreadPool{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
impl /*struct*/ QThreadPool {
  pub fn setMaxThreadCount<RetType, T: QThreadPool_setMaxThreadCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMaxThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_setMaxThreadCount<RetType> {
  fn setMaxThreadCount(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
impl<'a> /*trait*/ QThreadPool_setMaxThreadCount<()> for (i32) {
  fn setMaxThreadCount(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool17setMaxThreadCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QThreadPool17setMaxThreadCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
impl /*struct*/ QThreadPool {
  pub fn setExpiryTimeout<RetType, T: QThreadPool_setExpiryTimeout<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setExpiryTimeout(self);
    // return 1;
  }
}

pub trait QThreadPool_setExpiryTimeout<RetType> {
  fn setExpiryTimeout(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
impl<'a> /*trait*/ QThreadPool_setExpiryTimeout<()> for (i32) {
  fn setExpiryTimeout(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool16setExpiryTimeoutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QThreadPool16setExpiryTimeoutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QThreadPool::reserveThread();
impl /*struct*/ QThreadPool {
  pub fn reserveThread<RetType, T: QThreadPool_reserveThread<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.reserveThread(self);
    // return 1;
  }
}

pub trait QThreadPool_reserveThread<RetType> {
  fn reserveThread(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::reserveThread();
impl<'a> /*trait*/ QThreadPool_reserveThread<()> for () {
  fn reserveThread(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13reserveThreadEv()};
     unsafe {_ZN11QThreadPool13reserveThreadEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QThreadPool::clear();
impl /*struct*/ QThreadPool {
  pub fn clear<RetType, T: QThreadPool_clear<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QThreadPool_clear<RetType> {
  fn clear(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::clear();
impl<'a> /*trait*/ QThreadPool_clear<()> for () {
  fn clear(self , rsthis: &mut QThreadPool) -> () {
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

// proto:  void QThreadPool::start(QRunnable * runnable, int priority);
impl /*struct*/ QThreadPool {
  pub fn start<RetType, T: QThreadPool_start<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QThreadPool_start<RetType> {
  fn start(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::start(QRunnable * runnable, int priority);
impl<'a> /*trait*/ QThreadPool_start<()> for (&'a mut QRunnable, i32) {
  fn start(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5startEP9QRunnablei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QThreadPool5startEP9QRunnablei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QThreadPool::maxThreadCount();
impl /*struct*/ QThreadPool {
  pub fn maxThreadCount<RetType, T: QThreadPool_maxThreadCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maxThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_maxThreadCount<RetType> {
  fn maxThreadCount(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  int QThreadPool::maxThreadCount();
impl<'a> /*trait*/ QThreadPool_maxThreadCount<i32> for () {
  fn maxThreadCount(self , rsthis: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool14maxThreadCountEv()};
    let mut ret = unsafe {_ZNK11QThreadPool14maxThreadCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QThreadPool::releaseThread();
impl /*struct*/ QThreadPool {
  pub fn releaseThread<RetType, T: QThreadPool_releaseThread<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.releaseThread(self);
    // return 1;
  }
}

pub trait QThreadPool_releaseThread<RetType> {
  fn releaseThread(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  void QThreadPool::releaseThread();
impl<'a> /*trait*/ QThreadPool_releaseThread<()> for () {
  fn releaseThread(self , rsthis: &mut QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13releaseThreadEv()};
     unsafe {_ZN11QThreadPool13releaseThreadEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QThreadPool::activeThreadCount();
impl /*struct*/ QThreadPool {
  pub fn activeThreadCount<RetType, T: QThreadPool_activeThreadCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.activeThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_activeThreadCount<RetType> {
  fn activeThreadCount(self , rsthis: &mut QThreadPool) -> RetType;
}

// proto:  int QThreadPool::activeThreadCount();
impl<'a> /*trait*/ QThreadPool_activeThreadCount<i32> for () {
  fn activeThreadCount(self , rsthis: &mut QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool17activeThreadCountEv()};
    let mut ret = unsafe {_ZNK11QThreadPool17activeThreadCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

