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
  // proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
  fn _ZN18QOpenGLTimeMonitor14setSampleCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QOpenGLTimeMonitor::sampleCount();
  fn _ZNK18QOpenGLTimeMonitor11sampleCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTimeMonitor::destroy();
  fn _ZN18QOpenGLTimeMonitor7destroyEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTimeMonitor::NewQOpenGLTimeMonitor(const QOpenGLTimeMonitor & );
  fn _ZN18QOpenGLTimeMonitorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QOpenGLTimeMonitor::create();
  fn _ZN18QOpenGLTimeMonitor6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLTimeMonitor::FreeQOpenGLTimeMonitor();
  fn _ZN18QOpenGLTimeMonitorD0Ev(qthis: *mut c_void) ;
  // proto:  bool QOpenGLTimeMonitor::isResultAvailable();
  fn _ZNK18QOpenGLTimeMonitor17isResultAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
  fn _ZNK18QOpenGLTimeMonitor16waitForIntervalsEv(qthis: *mut c_void) ;
  // proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
  fn _ZNK18QOpenGLTimeMonitor9objectIdsEv(qthis: *mut c_void) ;
  // proto:  int QOpenGLTimeMonitor::recordSample();
  fn _ZN18QOpenGLTimeMonitor12recordSampleEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTimeMonitor::reset();
  fn _ZN18QOpenGLTimeMonitor5resetEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTimeMonitor::NewQOpenGLTimeMonitor(QObject * parent);
  fn _ZN18QOpenGLTimeMonitorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
  fn _ZNK18QOpenGLTimeMonitor14waitForSamplesEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLTimeMonitor::isCreated();
  fn _ZNK18QOpenGLTimeMonitor9isCreatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
  fn _ZNK18QOpenGLTimeMonitor10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLTimeMonitor)=1
pub struct QOpenGLTimeMonitor {
  pub qclsinst: *mut c_void,
}

// proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn setSampleCount<RetType, T: QOpenGLTimeMonitor_setSampleCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSampleCount(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_setSampleCount<RetType> {
  fn setSampleCount(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
impl<'a> /*trait*/ QOpenGLTimeMonitor_setSampleCount<()> for (i32) {
  fn setSampleCount(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor14setSampleCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QOpenGLTimeMonitor14setSampleCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QOpenGLTimeMonitor::sampleCount();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn sampleCount<RetType, T: QOpenGLTimeMonitor_sampleCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sampleCount(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_sampleCount<RetType> {
  fn sampleCount(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  int QOpenGLTimeMonitor::sampleCount();
impl<'a> /*trait*/ QOpenGLTimeMonitor_sampleCount<i32> for () {
  fn sampleCount(self , rsthis: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor11sampleCountEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor11sampleCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTimeMonitor::destroy();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn destroy<RetType, T: QOpenGLTimeMonitor_destroy<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_destroy<RetType> {
  fn destroy(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  void QOpenGLTimeMonitor::destroy();
impl<'a> /*trait*/ QOpenGLTimeMonitor_destroy<()> for () {
  fn destroy(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor7destroyEv()};
     unsafe {_ZN18QOpenGLTimeMonitor7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn NewQOpenGLTimeMonitor<T: QOpenGLTimeMonitor_NewQOpenGLTimeMonitor>(value: T) -> QOpenGLTimeMonitor {
    let rsthis = value.NewQOpenGLTimeMonitor();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_NewQOpenGLTimeMonitor {
  fn NewQOpenGLTimeMonitor(self) -> QOpenGLTimeMonitor;
}

// proto: void QOpenGLTimeMonitor::NewQOpenGLTimeMonitor(const QOpenGLTimeMonitor & );
impl<'a> /*trait*/ QOpenGLTimeMonitor_NewQOpenGLTimeMonitor for (&'a  QOpenGLTimeMonitor) {
  fn NewQOpenGLTimeMonitor(self) -> QOpenGLTimeMonitor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLTimeMonitorC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLTimeMonitor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QOpenGLTimeMonitor::create();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn create<RetType, T: QOpenGLTimeMonitor_create<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_create<RetType> {
  fn create(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  bool QOpenGLTimeMonitor::create();
impl<'a> /*trait*/ QOpenGLTimeMonitor_create<i8> for () {
  fn create(self , rsthis: &mut QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor6createEv()};
    let mut ret = unsafe {_ZN18QOpenGLTimeMonitor6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTimeMonitor::FreeQOpenGLTimeMonitor();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn FreeQOpenGLTimeMonitor<RetType, T: QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLTimeMonitor(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor<RetType> {
  fn FreeQOpenGLTimeMonitor(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  void QOpenGLTimeMonitor::FreeQOpenGLTimeMonitor();
impl<'a> /*trait*/ QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor<()> for () {
  fn FreeQOpenGLTimeMonitor(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorD0Ev()};
     unsafe {_ZN18QOpenGLTimeMonitorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QOpenGLTimeMonitor::isResultAvailable();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isResultAvailable<RetType, T: QOpenGLTimeMonitor_isResultAvailable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isResultAvailable(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_isResultAvailable<RetType> {
  fn isResultAvailable(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  bool QOpenGLTimeMonitor::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isResultAvailable<i8> for () {
  fn isResultAvailable(self , rsthis: &mut QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor17isResultAvailableEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor17isResultAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForIntervals<RetType, T: QOpenGLTimeMonitor_waitForIntervals<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.waitForIntervals(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForIntervals<RetType> {
  fn waitForIntervals(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForIntervals<()> for () {
  fn waitForIntervals(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn objectIds<RetType, T: QOpenGLTimeMonitor_objectIds<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.objectIds(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_objectIds<RetType> {
  fn objectIds(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
impl<'a> /*trait*/ QOpenGLTimeMonitor_objectIds<()> for () {
  fn objectIds(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9objectIdsEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor9objectIdsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QOpenGLTimeMonitor::recordSample();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn recordSample<RetType, T: QOpenGLTimeMonitor_recordSample<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.recordSample(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_recordSample<RetType> {
  fn recordSample(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  int QOpenGLTimeMonitor::recordSample();
impl<'a> /*trait*/ QOpenGLTimeMonitor_recordSample<i32> for () {
  fn recordSample(self , rsthis: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor12recordSampleEv()};
    let mut ret = unsafe {_ZN18QOpenGLTimeMonitor12recordSampleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTimeMonitor::reset();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn reset<RetType, T: QOpenGLTimeMonitor_reset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_reset<RetType> {
  fn reset(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  void QOpenGLTimeMonitor::reset();
impl<'a> /*trait*/ QOpenGLTimeMonitor_reset<()> for () {
  fn reset(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor5resetEv()};
     unsafe {_ZN18QOpenGLTimeMonitor5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLTimeMonitor::NewQOpenGLTimeMonitor(QObject * parent);
impl<'a> /*trait*/ QOpenGLTimeMonitor_NewQOpenGLTimeMonitor for (&'a mut QObject) {
  fn NewQOpenGLTimeMonitor(self) -> QOpenGLTimeMonitor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLTimeMonitorC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLTimeMonitor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForSamples<RetType, T: QOpenGLTimeMonitor_waitForSamples<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.waitForSamples(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForSamples<RetType> {
  fn waitForSamples(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForSamples<()> for () {
  fn waitForSamples(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor14waitForSamplesEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor14waitForSamplesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QOpenGLTimeMonitor::isCreated();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isCreated<RetType, T: QOpenGLTimeMonitor_isCreated<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_isCreated<RetType> {
  fn isCreated(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  bool QOpenGLTimeMonitor::isCreated();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isCreated<i8> for () {
  fn isCreated(self , rsthis: &mut QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9isCreatedEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn metaObject<RetType, T: QOpenGLTimeMonitor_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QOpenGLTimeMonitor) -> RetType;
}

// proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
impl<'a> /*trait*/ QOpenGLTimeMonitor_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor10metaObjectEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

