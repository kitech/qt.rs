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
  fn _ZN16QAnimationDriver7advanceEv() -> i32;
  fn _ZN16QAnimationDriverD0Ev() -> i32;
  fn _ZN16QAnimationDriverC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK16QAnimationDriver7elapsedEv() -> i32;
  fn _ZN16QAnimationDriver7installEv() -> i32;
  fn _ZNK16QAnimationDriver10metaObjectEv() -> i32;
  fn _ZN16QAnimationDriver9uninstallEv() -> i32;
  fn _ZN16QAnimationDriver7stoppedEv() -> i32;
  fn _ZNK16QAnimationDriver9isRunningEv() -> i32;
  fn _ZN16QAnimationDriver7startedEv() -> i32;
  fn _ZNK16QAnimationDriver9startTimeEv() -> i32;
  fn _ZN16QAnimationDriver12setStartTimeEx(arg0: c_longlong) -> i32;
}

// body block begin
// class sizeof(QAnimationDriver)=1
pub struct QAnimationDriver {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAnimationDriver {
  pub fn advance<T: QAnimationDriver_advance>(&mut self, value: T) -> i32 {
    value.advance(self);
    return 1;
  }
}

pub trait QAnimationDriver_advance {
  fn advance(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::advance();
impl<'a> /*trait*/ QAnimationDriver_advance for () {
  fn advance(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7advanceEv()};
    unsafe {_ZN16QAnimationDriver7advanceEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn FreeQAnimationDriver<T: QAnimationDriver_FreeQAnimationDriver>(&mut self, value: T) -> i32 {
    value.FreeQAnimationDriver(self);
    return 1;
  }
}

pub trait QAnimationDriver_FreeQAnimationDriver {
  fn FreeQAnimationDriver(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::FreeQAnimationDriver();
impl<'a> /*trait*/ QAnimationDriver_FreeQAnimationDriver for () {
  fn FreeQAnimationDriver(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriverD0Ev()};
    unsafe {_ZN16QAnimationDriverD0Ev()};
    return 1;
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
  pub fn elapsed<T: QAnimationDriver_elapsed>(&mut self, value: T) -> i32 {
    value.elapsed(self);
    return 1;
  }
}

pub trait QAnimationDriver_elapsed {
  fn elapsed(self, this: &mut QAnimationDriver) -> i32;
}

// proto: long long QAnimationDriver::elapsed();
impl<'a> /*trait*/ QAnimationDriver_elapsed for () {
  fn elapsed(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver7elapsedEv()};
    unsafe {_ZNK16QAnimationDriver7elapsedEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn install<T: QAnimationDriver_install>(&mut self, value: T) -> i32 {
    value.install(self);
    return 1;
  }
}

pub trait QAnimationDriver_install {
  fn install(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::install();
impl<'a> /*trait*/ QAnimationDriver_install for () {
  fn install(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7installEv()};
    unsafe {_ZN16QAnimationDriver7installEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn metaObject<T: QAnimationDriver_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QAnimationDriver_metaObject {
  fn metaObject(self, this: &mut QAnimationDriver) -> i32;
}

// proto: const QMetaObject * QAnimationDriver::metaObject();
impl<'a> /*trait*/ QAnimationDriver_metaObject for () {
  fn metaObject(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver10metaObjectEv()};
    unsafe {_ZNK16QAnimationDriver10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn uninstall<T: QAnimationDriver_uninstall>(&mut self, value: T) -> i32 {
    value.uninstall(self);
    return 1;
  }
}

pub trait QAnimationDriver_uninstall {
  fn uninstall(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::uninstall();
impl<'a> /*trait*/ QAnimationDriver_uninstall for () {
  fn uninstall(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver9uninstallEv()};
    unsafe {_ZN16QAnimationDriver9uninstallEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn stopped<T: QAnimationDriver_stopped>(&mut self, value: T) -> i32 {
    value.stopped(self);
    return 1;
  }
}

pub trait QAnimationDriver_stopped {
  fn stopped(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::stopped();
impl<'a> /*trait*/ QAnimationDriver_stopped for () {
  fn stopped(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7stoppedEv()};
    unsafe {_ZN16QAnimationDriver7stoppedEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn isRunning<T: QAnimationDriver_isRunning>(&mut self, value: T) -> i32 {
    value.isRunning(self);
    return 1;
  }
}

pub trait QAnimationDriver_isRunning {
  fn isRunning(self, this: &mut QAnimationDriver) -> i32;
}

// proto: bool QAnimationDriver::isRunning();
impl<'a> /*trait*/ QAnimationDriver_isRunning for () {
  fn isRunning(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9isRunningEv()};
    unsafe {_ZNK16QAnimationDriver9isRunningEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn started<T: QAnimationDriver_started>(&mut self, value: T) -> i32 {
    value.started(self);
    return 1;
  }
}

pub trait QAnimationDriver_started {
  fn started(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::started();
impl<'a> /*trait*/ QAnimationDriver_started for () {
  fn started(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7startedEv()};
    unsafe {_ZN16QAnimationDriver7startedEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn startTime<T: QAnimationDriver_startTime>(&mut self, value: T) -> i32 {
    value.startTime(self);
    return 1;
  }
}

pub trait QAnimationDriver_startTime {
  fn startTime(self, this: &mut QAnimationDriver) -> i32;
}

// proto: long long QAnimationDriver::startTime();
impl<'a> /*trait*/ QAnimationDriver_startTime for () {
  fn startTime(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9startTimeEv()};
    unsafe {_ZNK16QAnimationDriver9startTimeEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn setStartTime<T: QAnimationDriver_setStartTime>(&mut self, value: T) -> i32 {
    value.setStartTime(self);
    return 1;
  }
}

pub trait QAnimationDriver_setStartTime {
  fn setStartTime(self, this: &mut QAnimationDriver) -> i32;
}

// proto: void QAnimationDriver::setStartTime(qint64 startTime);
impl<'a> /*trait*/ QAnimationDriver_setStartTime for (i64) {
  fn setStartTime(self, this: &mut QAnimationDriver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver12setStartTimeEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN16QAnimationDriver12setStartTimeEx(arg0)};
    return 1;
  }
}

