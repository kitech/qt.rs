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

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn setSampleCount<T: QOpenGLTimeMonitor_setSampleCount>(&mut self, value: T)  {
     value.setSampleCount(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_setSampleCount {
  fn setSampleCount(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
impl<'a> /*trait*/ QOpenGLTimeMonitor_setSampleCount for (i32) {
  fn setSampleCount(self, rsthis: &mut QOpenGLTimeMonitor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor14setSampleCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QOpenGLTimeMonitor14setSampleCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn sampleCount<T: QOpenGLTimeMonitor_sampleCount>(&mut self, value: T) -> i32 {
    return value.sampleCount(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_sampleCount {
  fn sampleCount(self, rsthis: &mut QOpenGLTimeMonitor) -> i32;
}

// proto:  int QOpenGLTimeMonitor::sampleCount();
impl<'a> /*trait*/ QOpenGLTimeMonitor_sampleCount for () {
  fn sampleCount(self, rsthis: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor11sampleCountEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor11sampleCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn destroy<T: QOpenGLTimeMonitor_destroy>(&mut self, value: T)  {
     value.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_destroy {
  fn destroy(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  void QOpenGLTimeMonitor::destroy();
impl<'a> /*trait*/ QOpenGLTimeMonitor_destroy for () {
  fn destroy(self, rsthis: &mut QOpenGLTimeMonitor)  {
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

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn create<T: QOpenGLTimeMonitor_create>(&mut self, value: T) -> i8 {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_create {
  fn create(self, rsthis: &mut QOpenGLTimeMonitor) -> i8;
}

// proto:  bool QOpenGLTimeMonitor::create();
impl<'a> /*trait*/ QOpenGLTimeMonitor_create for () {
  fn create(self, rsthis: &mut QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor6createEv()};
    let mut ret = unsafe {_ZN18QOpenGLTimeMonitor6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn FreeQOpenGLTimeMonitor<T: QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor>(&mut self, value: T)  {
     value.FreeQOpenGLTimeMonitor(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor {
  fn FreeQOpenGLTimeMonitor(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  void QOpenGLTimeMonitor::FreeQOpenGLTimeMonitor();
impl<'a> /*trait*/ QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor for () {
  fn FreeQOpenGLTimeMonitor(self, rsthis: &mut QOpenGLTimeMonitor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorD0Ev()};
     unsafe {_ZN18QOpenGLTimeMonitorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isResultAvailable<T: QOpenGLTimeMonitor_isResultAvailable>(&mut self, value: T) -> i8 {
    return value.isResultAvailable(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_isResultAvailable {
  fn isResultAvailable(self, rsthis: &mut QOpenGLTimeMonitor) -> i8;
}

// proto:  bool QOpenGLTimeMonitor::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isResultAvailable for () {
  fn isResultAvailable(self, rsthis: &mut QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor17isResultAvailableEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor17isResultAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForIntervals<T: QOpenGLTimeMonitor_waitForIntervals>(&mut self, value: T)  {
     value.waitForIntervals(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForIntervals {
  fn waitForIntervals(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForIntervals for () {
  fn waitForIntervals(self, rsthis: &mut QOpenGLTimeMonitor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn objectIds<T: QOpenGLTimeMonitor_objectIds>(&mut self, value: T)  {
     value.objectIds(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_objectIds {
  fn objectIds(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
impl<'a> /*trait*/ QOpenGLTimeMonitor_objectIds for () {
  fn objectIds(self, rsthis: &mut QOpenGLTimeMonitor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9objectIdsEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor9objectIdsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn recordSample<T: QOpenGLTimeMonitor_recordSample>(&mut self, value: T) -> i32 {
    return value.recordSample(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_recordSample {
  fn recordSample(self, rsthis: &mut QOpenGLTimeMonitor) -> i32;
}

// proto:  int QOpenGLTimeMonitor::recordSample();
impl<'a> /*trait*/ QOpenGLTimeMonitor_recordSample for () {
  fn recordSample(self, rsthis: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor12recordSampleEv()};
    let mut ret = unsafe {_ZN18QOpenGLTimeMonitor12recordSampleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn reset<T: QOpenGLTimeMonitor_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_reset {
  fn reset(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  void QOpenGLTimeMonitor::reset();
impl<'a> /*trait*/ QOpenGLTimeMonitor_reset for () {
  fn reset(self, rsthis: &mut QOpenGLTimeMonitor)  {
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

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForSamples<T: QOpenGLTimeMonitor_waitForSamples>(&mut self, value: T)  {
     value.waitForSamples(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForSamples {
  fn waitForSamples(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForSamples for () {
  fn waitForSamples(self, rsthis: &mut QOpenGLTimeMonitor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor14waitForSamplesEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor14waitForSamplesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isCreated<T: QOpenGLTimeMonitor_isCreated>(&mut self, value: T) -> i8 {
    return value.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_isCreated {
  fn isCreated(self, rsthis: &mut QOpenGLTimeMonitor) -> i8;
}

// proto:  bool QOpenGLTimeMonitor::isCreated();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isCreated for () {
  fn isCreated(self, rsthis: &mut QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9isCreatedEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn metaObject<T: QOpenGLTimeMonitor_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLTimeMonitor) ;
}

// proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
impl<'a> /*trait*/ QOpenGLTimeMonitor_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLTimeMonitor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor10metaObjectEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

