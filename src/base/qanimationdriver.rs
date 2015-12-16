// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAnimationDriver::advance();
  fn _ZN16QAnimationDriver7advanceEv(qthis: *mut c_void) ;
  // proto:  void QAnimationDriver::FreeQAnimationDriver();
  fn _ZN16QAnimationDriverD0Ev(qthis: *mut c_void) ;
  // proto:  void QAnimationDriver::NewQAnimationDriver(QObject * parent);
  fn _ZN16QAnimationDriverC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  long long QAnimationDriver::elapsed();
  fn _ZNK16QAnimationDriver7elapsedEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QAnimationDriver::install();
  fn _ZN16QAnimationDriver7installEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QAnimationDriver::metaObject();
  fn _ZNK16QAnimationDriver10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QAnimationDriver::uninstall();
  fn _ZN16QAnimationDriver9uninstallEv(qthis: *mut c_void) ;
  // proto:  void QAnimationDriver::stopped();
  fn _ZN16QAnimationDriver7stoppedEv(qthis: *mut c_void) ;
  // proto:  bool QAnimationDriver::isRunning();
  fn _ZNK16QAnimationDriver9isRunningEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QAnimationDriver::started();
  fn _ZN16QAnimationDriver7startedEv(qthis: *mut c_void) ;
  // proto:  long long QAnimationDriver::startTime();
  fn _ZNK16QAnimationDriver9startTimeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
  fn _ZN16QAnimationDriver12setStartTimeEx(qthis: *mut c_void, arg0: c_longlong) ;
}

// body block begin
// class sizeof(QAnimationDriver)=1
pub struct QAnimationDriver {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAnimationDriver {
  pub fn advance<T: QAnimationDriver_advance>(&mut self, value: T)  {
     value.advance(self);
    // return 1;
  }
}

pub trait QAnimationDriver_advance {
  fn advance(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::advance();
impl<'a> /*trait*/ QAnimationDriver_advance for () {
  fn advance(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7advanceEv()};
     unsafe {_ZN16QAnimationDriver7advanceEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn FreeQAnimationDriver<T: QAnimationDriver_FreeQAnimationDriver>(&mut self, value: T)  {
     value.FreeQAnimationDriver(self);
    // return 1;
  }
}

pub trait QAnimationDriver_FreeQAnimationDriver {
  fn FreeQAnimationDriver(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::FreeQAnimationDriver();
impl<'a> /*trait*/ QAnimationDriver_FreeQAnimationDriver for () {
  fn FreeQAnimationDriver(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriverD0Ev()};
     unsafe {_ZN16QAnimationDriverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn NewQAnimationDriver<T: QAnimationDriver_NewQAnimationDriver>(value: T) -> QAnimationDriver {
    let rsthis = value.NewQAnimationDriver();
    return rsthis;
    // return 1;
  }
}

pub trait QAnimationDriver_NewQAnimationDriver {
  fn NewQAnimationDriver(self) -> QAnimationDriver;
}

// proto: void QAnimationDriver::NewQAnimationDriver(QObject * parent);
impl<'a> /*trait*/ QAnimationDriver_NewQAnimationDriver for (&'a mut QObject) {
  fn NewQAnimationDriver(self) -> QAnimationDriver {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriverC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QAnimationDriverC1EP7QObject(qthis, arg0)};
    let rsthis = QAnimationDriver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn elapsed<T: QAnimationDriver_elapsed>(&mut self, value: T) -> i64 {
    return value.elapsed(self);
    // return 1;
  }
}

pub trait QAnimationDriver_elapsed {
  fn elapsed(self, rsthis: &mut QAnimationDriver) -> i64;
}

// proto:  long long QAnimationDriver::elapsed();
impl<'a> /*trait*/ QAnimationDriver_elapsed for () {
  fn elapsed(self, rsthis: &mut QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver7elapsedEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver7elapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn install<T: QAnimationDriver_install>(&mut self, value: T)  {
     value.install(self);
    // return 1;
  }
}

pub trait QAnimationDriver_install {
  fn install(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::install();
impl<'a> /*trait*/ QAnimationDriver_install for () {
  fn install(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7installEv()};
     unsafe {_ZN16QAnimationDriver7installEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn metaObject<T: QAnimationDriver_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QAnimationDriver_metaObject {
  fn metaObject(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  const QMetaObject * QAnimationDriver::metaObject();
impl<'a> /*trait*/ QAnimationDriver_metaObject for () {
  fn metaObject(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver10metaObjectEv()};
     unsafe {_ZNK16QAnimationDriver10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn uninstall<T: QAnimationDriver_uninstall>(&mut self, value: T)  {
     value.uninstall(self);
    // return 1;
  }
}

pub trait QAnimationDriver_uninstall {
  fn uninstall(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::uninstall();
impl<'a> /*trait*/ QAnimationDriver_uninstall for () {
  fn uninstall(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver9uninstallEv()};
     unsafe {_ZN16QAnimationDriver9uninstallEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn stopped<T: QAnimationDriver_stopped>(&mut self, value: T)  {
     value.stopped(self);
    // return 1;
  }
}

pub trait QAnimationDriver_stopped {
  fn stopped(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::stopped();
impl<'a> /*trait*/ QAnimationDriver_stopped for () {
  fn stopped(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7stoppedEv()};
     unsafe {_ZN16QAnimationDriver7stoppedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn isRunning<T: QAnimationDriver_isRunning>(&mut self, value: T) -> i8 {
    return value.isRunning(self);
    // return 1;
  }
}

pub trait QAnimationDriver_isRunning {
  fn isRunning(self, rsthis: &mut QAnimationDriver) -> i8;
}

// proto:  bool QAnimationDriver::isRunning();
impl<'a> /*trait*/ QAnimationDriver_isRunning for () {
  fn isRunning(self, rsthis: &mut QAnimationDriver) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9isRunningEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn started<T: QAnimationDriver_started>(&mut self, value: T)  {
     value.started(self);
    // return 1;
  }
}

pub trait QAnimationDriver_started {
  fn started(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::started();
impl<'a> /*trait*/ QAnimationDriver_started for () {
  fn started(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7startedEv()};
     unsafe {_ZN16QAnimationDriver7startedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn startTime<T: QAnimationDriver_startTime>(&mut self, value: T) -> i64 {
    return value.startTime(self);
    // return 1;
  }
}

pub trait QAnimationDriver_startTime {
  fn startTime(self, rsthis: &mut QAnimationDriver) -> i64;
}

// proto:  long long QAnimationDriver::startTime();
impl<'a> /*trait*/ QAnimationDriver_startTime for () {
  fn startTime(self, rsthis: &mut QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9startTimeEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver9startTimeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn setStartTime<T: QAnimationDriver_setStartTime>(&mut self, value: T)  {
     value.setStartTime(self);
    // return 1;
  }
}

pub trait QAnimationDriver_setStartTime {
  fn setStartTime(self, rsthis: &mut QAnimationDriver) ;
}

// proto:  void QAnimationDriver::setStartTime(qint64 startTime);
impl<'a> /*trait*/ QAnimationDriver_setStartTime for (i64) {
  fn setStartTime(self, rsthis: &mut QAnimationDriver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver12setStartTimeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN16QAnimationDriver12setStartTimeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

