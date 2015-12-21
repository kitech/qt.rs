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
  fn _ZN16QAnimationDriver7advanceEv(qthis: *mut c_void);
  // proto:  void QAnimationDriver::~QAnimationDriver();
  fn _ZN16QAnimationDriverD0Ev(qthis: *mut c_void);
  // proto:  void QAnimationDriver::QAnimationDriver(QObject * parent);
  fn _ZN16QAnimationDriverC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qint64 QAnimationDriver::elapsed();
  fn _ZNK16QAnimationDriver7elapsedEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QAnimationDriver::install();
  fn _ZN16QAnimationDriver7installEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QAnimationDriver::metaObject();
  fn _ZNK16QAnimationDriver10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAnimationDriver::uninstall();
  fn _ZN16QAnimationDriver9uninstallEv(qthis: *mut c_void);
  // proto:  void QAnimationDriver::stopped();
  fn _ZN16QAnimationDriver7stoppedEv(qthis: *mut c_void);
  // proto:  bool QAnimationDriver::isRunning();
  fn _ZNK16QAnimationDriver9isRunningEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAnimationDriver::started();
  fn _ZN16QAnimationDriver7startedEv(qthis: *mut c_void);
  // proto:  qint64 QAnimationDriver::startTime();
  fn _ZNK16QAnimationDriver9startTimeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
  fn _ZN16QAnimationDriver12setStartTimeEx(qthis: *mut c_void, arg0: c_longlong);
}

// body block begin
// class sizeof(QAnimationDriver)=1
pub struct QAnimationDriver {
  pub qclsinst: *mut c_void,
}

  // proto:  void QAnimationDriver::advance();
impl /*struct*/ QAnimationDriver {
  pub fn advance<RetType, T: QAnimationDriver_advance<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.advance(self);
    // return 1;
  }
}

pub trait QAnimationDriver_advance<RetType> {
  fn advance(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::advance();
impl<'a> /*trait*/ QAnimationDriver_advance<()> for () {
  fn advance(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7advanceEv()};
     unsafe {_ZN16QAnimationDriver7advanceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::~QAnimationDriver();
impl /*struct*/ QAnimationDriver {
  pub fn FreeQAnimationDriver<RetType, T: QAnimationDriver_FreeQAnimationDriver<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAnimationDriver(self);
    // return 1;
  }
}

pub trait QAnimationDriver_FreeQAnimationDriver<RetType> {
  fn FreeQAnimationDriver(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::~QAnimationDriver();
impl<'a> /*trait*/ QAnimationDriver_FreeQAnimationDriver<()> for () {
  fn FreeQAnimationDriver(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriverD0Ev()};
     unsafe {_ZN16QAnimationDriverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::QAnimationDriver(QObject * parent);
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

  // proto:  void QAnimationDriver::QAnimationDriver(QObject * parent);
impl<'a> /*trait*/ QAnimationDriver_NewQAnimationDriver for (QObject) {
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

  // proto:  qint64 QAnimationDriver::elapsed();
impl /*struct*/ QAnimationDriver {
  pub fn elapsed<RetType, T: QAnimationDriver_elapsed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.elapsed(self);
    // return 1;
  }
}

pub trait QAnimationDriver_elapsed<RetType> {
  fn elapsed(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  qint64 QAnimationDriver::elapsed();
impl<'a> /*trait*/ QAnimationDriver_elapsed<i64> for () {
  fn elapsed(self , rsthis: &mut QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver7elapsedEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver7elapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QAnimationDriver::install();
impl /*struct*/ QAnimationDriver {
  pub fn install<RetType, T: QAnimationDriver_install<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.install(self);
    // return 1;
  }
}

pub trait QAnimationDriver_install<RetType> {
  fn install(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::install();
impl<'a> /*trait*/ QAnimationDriver_install<()> for () {
  fn install(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7installEv()};
     unsafe {_ZN16QAnimationDriver7installEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAnimationDriver::metaObject();
impl /*struct*/ QAnimationDriver {
  pub fn metaObject<RetType, T: QAnimationDriver_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAnimationDriver_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  const QMetaObject * QAnimationDriver::metaObject();
impl<'a> /*trait*/ QAnimationDriver_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver10metaObjectEv()};
     unsafe {_ZNK16QAnimationDriver10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::uninstall();
impl /*struct*/ QAnimationDriver {
  pub fn uninstall<RetType, T: QAnimationDriver_uninstall<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.uninstall(self);
    // return 1;
  }
}

pub trait QAnimationDriver_uninstall<RetType> {
  fn uninstall(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::uninstall();
impl<'a> /*trait*/ QAnimationDriver_uninstall<()> for () {
  fn uninstall(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver9uninstallEv()};
     unsafe {_ZN16QAnimationDriver9uninstallEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::stopped();
impl /*struct*/ QAnimationDriver {
  pub fn stopped<RetType, T: QAnimationDriver_stopped<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stopped(self);
    // return 1;
  }
}

pub trait QAnimationDriver_stopped<RetType> {
  fn stopped(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::stopped();
impl<'a> /*trait*/ QAnimationDriver_stopped<()> for () {
  fn stopped(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7stoppedEv()};
     unsafe {_ZN16QAnimationDriver7stoppedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAnimationDriver::isRunning();
impl /*struct*/ QAnimationDriver {
  pub fn isRunning<RetType, T: QAnimationDriver_isRunning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QAnimationDriver_isRunning<RetType> {
  fn isRunning(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  bool QAnimationDriver::isRunning();
impl<'a> /*trait*/ QAnimationDriver_isRunning<i8> for () {
  fn isRunning(self , rsthis: &mut QAnimationDriver) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9isRunningEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAnimationDriver::started();
impl /*struct*/ QAnimationDriver {
  pub fn started<RetType, T: QAnimationDriver_started<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.started(self);
    // return 1;
  }
}

pub trait QAnimationDriver_started<RetType> {
  fn started(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::started();
impl<'a> /*trait*/ QAnimationDriver_started<()> for () {
  fn started(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7startedEv()};
     unsafe {_ZN16QAnimationDriver7startedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QAnimationDriver::startTime();
impl /*struct*/ QAnimationDriver {
  pub fn startTime<RetType, T: QAnimationDriver_startTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.startTime(self);
    // return 1;
  }
}

pub trait QAnimationDriver_startTime<RetType> {
  fn startTime(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  qint64 QAnimationDriver::startTime();
impl<'a> /*trait*/ QAnimationDriver_startTime<i64> for () {
  fn startTime(self , rsthis: &mut QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9startTimeEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver9startTimeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
impl /*struct*/ QAnimationDriver {
  pub fn setStartTime<RetType, T: QAnimationDriver_setStartTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStartTime(self);
    // return 1;
  }
}

pub trait QAnimationDriver_setStartTime<RetType> {
  fn setStartTime(self , rsthis: &mut QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
impl<'a> /*trait*/ QAnimationDriver_setStartTime<()> for (i64) {
  fn setStartTime(self , rsthis: &mut QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver12setStartTimeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN16QAnimationDriver12setStartTimeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

