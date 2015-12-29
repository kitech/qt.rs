// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qabstractanimation.h
// dst-file: /src/core/qabstractanimation.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qanimationgroup::QAnimationGroup; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractAnimation_Class_Size() -> c_int;
  // proto:  void QAbstractAnimation::currentLoopChanged(int currentLoop);
  fn _ZN18QAbstractAnimation18currentLoopChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractAnimation::resume();
  fn _ZN18QAbstractAnimation6resumeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractAnimation::QAbstractAnimation(QObject * parent);
  fn dector_ZN18QAbstractAnimationC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QAbstractAnimationC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractAnimation::stop();
  fn _ZN18QAbstractAnimation4stopEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractAnimation::pause();
  fn _ZN18QAbstractAnimation5pauseEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractAnimation::QAbstractAnimation(const QAbstractAnimation & );
  fn dector_ZN18QAbstractAnimationC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QAbstractAnimationC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractAnimation::setLoopCount(int loopCount);
  fn _ZN18QAbstractAnimation12setLoopCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QAbstractAnimation::currentLoop();
  fn _ZNK18QAbstractAnimation11currentLoopEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QAnimationGroup * QAbstractAnimation::group();
  fn _ZNK18QAbstractAnimation5groupEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractAnimation::setPaused(bool );
  fn _ZN18QAbstractAnimation9setPausedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QAbstractAnimation::totalDuration();
  fn _ZNK18QAbstractAnimation13totalDurationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QAbstractAnimation::duration();
  fn _ZNK18QAbstractAnimation8durationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QAbstractAnimation::metaObject();
  fn _ZNK18QAbstractAnimation10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QAbstractAnimation::currentLoopTime();
  fn _ZNK18QAbstractAnimation15currentLoopTimeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QAbstractAnimation::currentTime();
  fn _ZNK18QAbstractAnimation11currentTimeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractAnimation::setCurrentTime(int msecs);
  fn _ZN18QAbstractAnimation14setCurrentTimeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractAnimation::finished();
  fn _ZN18QAbstractAnimation8finishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractAnimation::~QAbstractAnimation();
  fn _ZN18QAbstractAnimationD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QAbstractAnimation::loopCount();
  fn _ZNK18QAbstractAnimation9loopCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QAnimationDriver_Class_Size() -> c_int;
  // proto:  void QAnimationDriver::advance();
  fn _ZN16QAnimationDriver7advanceEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAnimationDriver::~QAnimationDriver();
  fn _ZN16QAnimationDriverD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAnimationDriver::QAnimationDriver(QObject * parent);
  fn dector_ZN16QAnimationDriverC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QAnimationDriverC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qint64 QAnimationDriver::elapsed();
  fn _ZNK16QAnimationDriver7elapsedEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QAnimationDriver::install();
  fn _ZN16QAnimationDriver7installEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QAnimationDriver::metaObject();
  fn _ZNK16QAnimationDriver10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAnimationDriver::uninstall();
  fn _ZN16QAnimationDriver9uninstallEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAnimationDriver::stopped();
  fn _ZN16QAnimationDriver7stoppedEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAnimationDriver::isRunning();
  fn _ZNK16QAnimationDriver9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAnimationDriver::started();
  fn _ZN16QAnimationDriver7startedEv(qthis: u64 /* *mut c_void*/);
  // proto:  qint64 QAnimationDriver::startTime();
  fn _ZNK16QAnimationDriver9startTimeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
  fn _ZN16QAnimationDriver12setStartTimeEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong);
  fn QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation12stateChangedENS_5StateES0_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect_box__ZN18QAbstractAnimation12stateChangedENS_5StateES0_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation18currentLoopChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect_box__ZN18QAbstractAnimation18currentLoopChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation8finishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect_box__ZN18QAbstractAnimation8finishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation16directionChangedENS_9DirectionE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractAnimation_SlotProxy_connect_box__ZN18QAbstractAnimation16directionChangedENS_9DirectionE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAnimationDriver_SlotProxy_connect__ZN16QAnimationDriver7stoppedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAnimationDriver_SlotProxy_connect_box__ZN16QAnimationDriver7stoppedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAnimationDriver_SlotProxy_connect__ZN16QAnimationDriver7startedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAnimationDriver_SlotProxy_connect_box__ZN16QAnimationDriver7startedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractAnimation)=1
#[derive(Default)]
pub struct QAbstractAnimation {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _currentLoopChanged_1: QAbstractAnimation_currentLoopChanged_signal,
  pub _finished_1: QAbstractAnimation_finished_signal,
  pub _stateChanged_1: QAbstractAnimation_stateChanged_signal,
  pub _directionChanged_1: QAbstractAnimation_directionChanged_signal,
}

// class sizeof(QAnimationDriver)=1
#[derive(Default)]
pub struct QAnimationDriver {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _started_1: QAnimationDriver_started_signal,
  pub _stopped_1: QAnimationDriver_stopped_signal,
}

impl /*struct*/ QAbstractAnimation {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractAnimation {
    return QAbstractAnimation{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractAnimation {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractAnimation {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAbstractAnimation::currentLoopChanged(int currentLoop);
impl /*struct*/ QAbstractAnimation {
  pub fn currentLoopChanged<RetType, T: QAbstractAnimation_currentLoopChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentLoopChanged(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_currentLoopChanged<RetType> {
  fn currentLoopChanged(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::currentLoopChanged(int currentLoop);
impl<'a> /*trait*/ QAbstractAnimation_currentLoopChanged<()> for (i32) {
  fn currentLoopChanged(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation18currentLoopChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QAbstractAnimation18currentLoopChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::resume();
impl /*struct*/ QAbstractAnimation {
  pub fn resume<RetType, T: QAbstractAnimation_resume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resume(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_resume<RetType> {
  fn resume(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::resume();
impl<'a> /*trait*/ QAbstractAnimation_resume<()> for () {
  fn resume(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation6resumeEv()};
     unsafe {_ZN18QAbstractAnimation6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::QAbstractAnimation(QObject * parent);
impl /*struct*/ QAbstractAnimation {
  pub fn New<T: QAbstractAnimation_New>(value: T) -> QAbstractAnimation {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractAnimation_New {
  fn New(self) -> QAbstractAnimation;
}

  // proto:  void QAbstractAnimation::QAbstractAnimation(QObject * parent);
impl<'a> /*trait*/ QAbstractAnimation_New for (&'a QObject) {
  fn New(self) -> QAbstractAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimationC1EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QAbstractAnimationC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QAbstractAnimationC1EP7QObject(arg0)} as u64;
    let rsthis = QAbstractAnimation{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::stop();
impl /*struct*/ QAbstractAnimation {
  pub fn stop<RetType, T: QAbstractAnimation_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_stop<RetType> {
  fn stop(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::stop();
impl<'a> /*trait*/ QAbstractAnimation_stop<()> for () {
  fn stop(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation4stopEv()};
     unsafe {_ZN18QAbstractAnimation4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::pause();
impl /*struct*/ QAbstractAnimation {
  pub fn pause<RetType, T: QAbstractAnimation_pause<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pause(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_pause<RetType> {
  fn pause(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::pause();
impl<'a> /*trait*/ QAbstractAnimation_pause<()> for () {
  fn pause(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation5pauseEv()};
     unsafe {_ZN18QAbstractAnimation5pauseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::QAbstractAnimation(const QAbstractAnimation & );
impl<'a> /*trait*/ QAbstractAnimation_New for (&'a QAbstractAnimation) {
  fn New(self) -> QAbstractAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimationC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QAbstractAnimationC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QAbstractAnimationC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractAnimation{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::setLoopCount(int loopCount);
impl /*struct*/ QAbstractAnimation {
  pub fn setLoopCount<RetType, T: QAbstractAnimation_setLoopCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLoopCount(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_setLoopCount<RetType> {
  fn setLoopCount(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::setLoopCount(int loopCount);
impl<'a> /*trait*/ QAbstractAnimation_setLoopCount<()> for (i32) {
  fn setLoopCount(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation12setLoopCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QAbstractAnimation12setLoopCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractAnimation::currentLoop();
impl /*struct*/ QAbstractAnimation {
  pub fn currentLoop<RetType, T: QAbstractAnimation_currentLoop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentLoop(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_currentLoop<RetType> {
  fn currentLoop(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  int QAbstractAnimation::currentLoop();
impl<'a> /*trait*/ QAbstractAnimation_currentLoop<i32> for () {
  fn currentLoop(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation11currentLoopEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation11currentLoopEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAnimationGroup * QAbstractAnimation::group();
impl /*struct*/ QAbstractAnimation {
  pub fn group<RetType, T: QAbstractAnimation_group<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_group<RetType> {
  fn group(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  QAnimationGroup * QAbstractAnimation::group();
impl<'a> /*trait*/ QAbstractAnimation_group<QAnimationGroup> for () {
  fn group(self , rsthis: & QAbstractAnimation) -> QAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation5groupEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation5groupEv(rsthis.qclsinst)};
    let mut ret1 = QAnimationGroup::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::setPaused(bool );
impl /*struct*/ QAbstractAnimation {
  pub fn setPaused<RetType, T: QAbstractAnimation_setPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaused(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_setPaused<RetType> {
  fn setPaused(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::setPaused(bool );
impl<'a> /*trait*/ QAbstractAnimation_setPaused<()> for (i8) {
  fn setPaused(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation9setPausedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN18QAbstractAnimation9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractAnimation::totalDuration();
impl /*struct*/ QAbstractAnimation {
  pub fn totalDuration<RetType, T: QAbstractAnimation_totalDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.totalDuration(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_totalDuration<RetType> {
  fn totalDuration(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  int QAbstractAnimation::totalDuration();
impl<'a> /*trait*/ QAbstractAnimation_totalDuration<i32> for () {
  fn totalDuration(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation13totalDurationEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation13totalDurationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAbstractAnimation::duration();
impl /*struct*/ QAbstractAnimation {
  pub fn duration<RetType, T: QAbstractAnimation_duration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_duration<RetType> {
  fn duration(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  int QAbstractAnimation::duration();
impl<'a> /*trait*/ QAbstractAnimation_duration<i32> for () {
  fn duration(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation8durationEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractAnimation::metaObject();
impl /*struct*/ QAbstractAnimation {
  pub fn metaObject<RetType, T: QAbstractAnimation_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  const QMetaObject * QAbstractAnimation::metaObject();
impl<'a> /*trait*/ QAbstractAnimation_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation10metaObjectEv()};
     unsafe {_ZNK18QAbstractAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QAbstractAnimation::currentLoopTime();
impl /*struct*/ QAbstractAnimation {
  pub fn currentLoopTime<RetType, T: QAbstractAnimation_currentLoopTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentLoopTime(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_currentLoopTime<RetType> {
  fn currentLoopTime(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  int QAbstractAnimation::currentLoopTime();
impl<'a> /*trait*/ QAbstractAnimation_currentLoopTime<i32> for () {
  fn currentLoopTime(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation15currentLoopTimeEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation15currentLoopTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAbstractAnimation::currentTime();
impl /*struct*/ QAbstractAnimation {
  pub fn currentTime<RetType, T: QAbstractAnimation_currentTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentTime(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_currentTime<RetType> {
  fn currentTime(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  int QAbstractAnimation::currentTime();
impl<'a> /*trait*/ QAbstractAnimation_currentTime<i32> for () {
  fn currentTime(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation11currentTimeEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation11currentTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::setCurrentTime(int msecs);
impl /*struct*/ QAbstractAnimation {
  pub fn setCurrentTime<RetType, T: QAbstractAnimation_setCurrentTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentTime(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_setCurrentTime<RetType> {
  fn setCurrentTime(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::setCurrentTime(int msecs);
impl<'a> /*trait*/ QAbstractAnimation_setCurrentTime<()> for (i32) {
  fn setCurrentTime(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation14setCurrentTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QAbstractAnimation14setCurrentTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::finished();
impl /*struct*/ QAbstractAnimation {
  pub fn finished<RetType, T: QAbstractAnimation_finished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.finished(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_finished<RetType> {
  fn finished(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::finished();
impl<'a> /*trait*/ QAbstractAnimation_finished<()> for () {
  fn finished(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimation8finishedEv()};
     unsafe {_ZN18QAbstractAnimation8finishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractAnimation::~QAbstractAnimation();
impl /*struct*/ QAbstractAnimation {
  pub fn Free<RetType, T: QAbstractAnimation_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_Free<RetType> {
  fn Free(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  void QAbstractAnimation::~QAbstractAnimation();
impl<'a> /*trait*/ QAbstractAnimation_Free<()> for () {
  fn Free(self , rsthis: & QAbstractAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractAnimationD0Ev()};
     unsafe {_ZN18QAbstractAnimationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QAbstractAnimation::loopCount();
impl /*struct*/ QAbstractAnimation {
  pub fn loopCount<RetType, T: QAbstractAnimation_loopCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loopCount(self);
    // return 1;
  }
}

pub trait QAbstractAnimation_loopCount<RetType> {
  fn loopCount(self , rsthis: & QAbstractAnimation) -> RetType;
}

  // proto:  int QAbstractAnimation::loopCount();
impl<'a> /*trait*/ QAbstractAnimation_loopCount<i32> for () {
  fn loopCount(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractAnimation9loopCountEv()};
    let mut ret = unsafe {_ZNK18QAbstractAnimation9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAnimationDriver {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAnimationDriver {
    return QAnimationDriver{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAnimationDriver {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAnimationDriver {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAnimationDriver::advance();
impl /*struct*/ QAnimationDriver {
  pub fn advance<RetType, T: QAnimationDriver_advance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.advance(self);
    // return 1;
  }
}

pub trait QAnimationDriver_advance<RetType> {
  fn advance(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::advance();
impl<'a> /*trait*/ QAnimationDriver_advance<()> for () {
  fn advance(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7advanceEv()};
     unsafe {_ZN16QAnimationDriver7advanceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::~QAnimationDriver();
impl /*struct*/ QAnimationDriver {
  pub fn Free<RetType, T: QAnimationDriver_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAnimationDriver_Free<RetType> {
  fn Free(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::~QAnimationDriver();
impl<'a> /*trait*/ QAnimationDriver_Free<()> for () {
  fn Free(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriverD0Ev()};
     unsafe {_ZN16QAnimationDriverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::QAnimationDriver(QObject * parent);
impl /*struct*/ QAnimationDriver {
  pub fn New<T: QAnimationDriver_New>(value: T) -> QAnimationDriver {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAnimationDriver_New {
  fn New(self) -> QAnimationDriver;
}

  // proto:  void QAnimationDriver::QAnimationDriver(QObject * parent);
impl<'a> /*trait*/ QAnimationDriver_New for (&'a QObject) {
  fn New(self) -> QAnimationDriver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriverC1EP7QObject()};
    let ctysz: c_int = unsafe{QAnimationDriver_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QAnimationDriverC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QAnimationDriverC1EP7QObject(arg0)} as u64;
    let rsthis = QAnimationDriver{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QAnimationDriver::elapsed();
impl /*struct*/ QAnimationDriver {
  pub fn elapsed<RetType, T: QAnimationDriver_elapsed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.elapsed(self);
    // return 1;
  }
}

pub trait QAnimationDriver_elapsed<RetType> {
  fn elapsed(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  qint64 QAnimationDriver::elapsed();
impl<'a> /*trait*/ QAnimationDriver_elapsed<i64> for () {
  fn elapsed(self , rsthis: & QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver7elapsedEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver7elapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QAnimationDriver::install();
impl /*struct*/ QAnimationDriver {
  pub fn install<RetType, T: QAnimationDriver_install<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.install(self);
    // return 1;
  }
}

pub trait QAnimationDriver_install<RetType> {
  fn install(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::install();
impl<'a> /*trait*/ QAnimationDriver_install<()> for () {
  fn install(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7installEv()};
     unsafe {_ZN16QAnimationDriver7installEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAnimationDriver::metaObject();
impl /*struct*/ QAnimationDriver {
  pub fn metaObject<RetType, T: QAnimationDriver_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAnimationDriver_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  const QMetaObject * QAnimationDriver::metaObject();
impl<'a> /*trait*/ QAnimationDriver_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver10metaObjectEv()};
     unsafe {_ZNK16QAnimationDriver10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::uninstall();
impl /*struct*/ QAnimationDriver {
  pub fn uninstall<RetType, T: QAnimationDriver_uninstall<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uninstall(self);
    // return 1;
  }
}

pub trait QAnimationDriver_uninstall<RetType> {
  fn uninstall(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::uninstall();
impl<'a> /*trait*/ QAnimationDriver_uninstall<()> for () {
  fn uninstall(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver9uninstallEv()};
     unsafe {_ZN16QAnimationDriver9uninstallEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationDriver::stopped();
impl /*struct*/ QAnimationDriver {
  pub fn stopped<RetType, T: QAnimationDriver_stopped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stopped(self);
    // return 1;
  }
}

pub trait QAnimationDriver_stopped<RetType> {
  fn stopped(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::stopped();
impl<'a> /*trait*/ QAnimationDriver_stopped<()> for () {
  fn stopped(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7stoppedEv()};
     unsafe {_ZN16QAnimationDriver7stoppedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAnimationDriver::isRunning();
impl /*struct*/ QAnimationDriver {
  pub fn isRunning<RetType, T: QAnimationDriver_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QAnimationDriver_isRunning<RetType> {
  fn isRunning(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  bool QAnimationDriver::isRunning();
impl<'a> /*trait*/ QAnimationDriver_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QAnimationDriver) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9isRunningEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAnimationDriver::started();
impl /*struct*/ QAnimationDriver {
  pub fn started<RetType, T: QAnimationDriver_started<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.started(self);
    // return 1;
  }
}

pub trait QAnimationDriver_started<RetType> {
  fn started(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::started();
impl<'a> /*trait*/ QAnimationDriver_started<()> for () {
  fn started(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver7startedEv()};
     unsafe {_ZN16QAnimationDriver7startedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QAnimationDriver::startTime();
impl /*struct*/ QAnimationDriver {
  pub fn startTime<RetType, T: QAnimationDriver_startTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startTime(self);
    // return 1;
  }
}

pub trait QAnimationDriver_startTime<RetType> {
  fn startTime(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  qint64 QAnimationDriver::startTime();
impl<'a> /*trait*/ QAnimationDriver_startTime<i64> for () {
  fn startTime(self , rsthis: & QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAnimationDriver9startTimeEv()};
    let mut ret = unsafe {_ZNK16QAnimationDriver9startTimeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
impl /*struct*/ QAnimationDriver {
  pub fn setStartTime<RetType, T: QAnimationDriver_setStartTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartTime(self);
    // return 1;
  }
}

pub trait QAnimationDriver_setStartTime<RetType> {
  fn setStartTime(self , rsthis: & QAnimationDriver) -> RetType;
}

  // proto:  void QAnimationDriver::setStartTime(qint64 startTime);
impl<'a> /*trait*/ QAnimationDriver_setStartTime<()> for (i64) {
  fn setStartTime(self , rsthis: & QAnimationDriver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAnimationDriver12setStartTimeEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN16QAnimationDriver12setStartTimeEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QAbstractAnimation_currentLoopChanged
pub struct QAbstractAnimation_currentLoopChanged_signal{poi:u64}
impl /* struct */ QAbstractAnimation {
  pub fn currentLoopChanged_1(&self) -> QAbstractAnimation_currentLoopChanged_signal {
     return QAbstractAnimation_currentLoopChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractAnimation_currentLoopChanged_signal {
  pub fn connect<T: QAbstractAnimation_currentLoopChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractAnimation_currentLoopChanged_signal_connect {
  fn connect(self, sigthis: QAbstractAnimation_currentLoopChanged_signal);
}

#[derive(Default)] // for QAbstractAnimation_finished
pub struct QAbstractAnimation_finished_signal{poi:u64}
impl /* struct */ QAbstractAnimation {
  pub fn finished_1(&self) -> QAbstractAnimation_finished_signal {
     return QAbstractAnimation_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractAnimation_finished_signal {
  pub fn connect<T: QAbstractAnimation_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractAnimation_finished_signal_connect {
  fn connect(self, sigthis: QAbstractAnimation_finished_signal);
}

#[derive(Default)] // for QAbstractAnimation_stateChanged
pub struct QAbstractAnimation_stateChanged_signal{poi:u64}
impl /* struct */ QAbstractAnimation {
  pub fn stateChanged_1(&self) -> QAbstractAnimation_stateChanged_signal {
     return QAbstractAnimation_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractAnimation_stateChanged_signal {
  pub fn connect<T: QAbstractAnimation_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractAnimation_stateChanged_signal_connect {
  fn connect(self, sigthis: QAbstractAnimation_stateChanged_signal);
}

#[derive(Default)] // for QAbstractAnimation_directionChanged
pub struct QAbstractAnimation_directionChanged_signal{poi:u64}
impl /* struct */ QAbstractAnimation {
  pub fn directionChanged_1(&self) -> QAbstractAnimation_directionChanged_signal {
     return QAbstractAnimation_directionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractAnimation_directionChanged_signal {
  pub fn connect<T: QAbstractAnimation_directionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractAnimation_directionChanged_signal_connect {
  fn connect(self, sigthis: QAbstractAnimation_directionChanged_signal);
}

// stateChanged(class QAbstractAnimation::State, class QAbstractAnimation::State)
extern fn QAbstractAnimation_stateChanged_signal_connect_cb_0(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QAbstractAnimation_stateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QAbstractAnimation_stateChanged_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QAbstractAnimation_stateChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_stateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation12stateChangedENS_5StateES0_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractAnimation_stateChanged_signal_connect for Box<fn(i32, i32)> {
  fn connect(self, sigthis: QAbstractAnimation_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_stateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation12stateChangedENS_5StateES0_(arg0, arg1, arg2)};
  }
}
// currentLoopChanged(int)
extern fn QAbstractAnimation_currentLoopChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QAbstractAnimation_currentLoopChanged_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QAbstractAnimation_currentLoopChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractAnimation_currentLoopChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_currentLoopChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation18currentLoopChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractAnimation_currentLoopChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QAbstractAnimation_currentLoopChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_currentLoopChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation18currentLoopChangedEi(arg0, arg1, arg2)};
  }
}
// finished()
extern fn QAbstractAnimation_finished_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QAbstractAnimation_finished_signal_connect_cb_box_2(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QAbstractAnimation_finished_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractAnimation_finished_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_finished_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation8finishedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractAnimation_finished_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QAbstractAnimation_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_finished_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation8finishedEv(arg0, arg1, arg2)};
  }
}
// directionChanged(class QAbstractAnimation::Direction)
extern fn QAbstractAnimation_directionChanged_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QAbstractAnimation_directionChanged_signal_connect_cb_box_3(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QAbstractAnimation_directionChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractAnimation_directionChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_directionChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation16directionChangedENS_9DirectionE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractAnimation_directionChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QAbstractAnimation_directionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractAnimation_directionChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QAbstractAnimation_SlotProxy_connect__ZN18QAbstractAnimation16directionChangedENS_9DirectionE(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QAnimationDriver_started
pub struct QAnimationDriver_started_signal{poi:u64}
impl /* struct */ QAnimationDriver {
  pub fn started_1(&self) -> QAnimationDriver_started_signal {
     return QAnimationDriver_started_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAnimationDriver_started_signal {
  pub fn connect<T: QAnimationDriver_started_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAnimationDriver_started_signal_connect {
  fn connect(self, sigthis: QAnimationDriver_started_signal);
}

#[derive(Default)] // for QAnimationDriver_stopped
pub struct QAnimationDriver_stopped_signal{poi:u64}
impl /* struct */ QAnimationDriver {
  pub fn stopped_1(&self) -> QAnimationDriver_stopped_signal {
     return QAnimationDriver_stopped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAnimationDriver_stopped_signal {
  pub fn connect<T: QAnimationDriver_stopped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAnimationDriver_stopped_signal_connect {
  fn connect(self, sigthis: QAnimationDriver_stopped_signal);
}

// stopped()
extern fn QAnimationDriver_stopped_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QAnimationDriver_stopped_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QAnimationDriver_stopped_signal_connect for fn() {
  fn connect(self, sigthis: QAnimationDriver_stopped_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAnimationDriver_stopped_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAnimationDriver_SlotProxy_connect__ZN16QAnimationDriver7stoppedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAnimationDriver_stopped_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QAnimationDriver_stopped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAnimationDriver_stopped_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QAnimationDriver_SlotProxy_connect__ZN16QAnimationDriver7stoppedEv(arg0, arg1, arg2)};
  }
}
// started()
extern fn QAnimationDriver_started_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QAnimationDriver_started_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QAnimationDriver_started_signal_connect for fn() {
  fn connect(self, sigthis: QAnimationDriver_started_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAnimationDriver_started_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAnimationDriver_SlotProxy_connect__ZN16QAnimationDriver7startedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAnimationDriver_started_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QAnimationDriver_started_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAnimationDriver_started_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QAnimationDriver_SlotProxy_connect__ZN16QAnimationDriver7startedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

