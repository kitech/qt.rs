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
  // proto: void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
  fn _ZN18QOpenGLTimeMonitor14setSampleCountEi(arg0: c_int) -> i32;
  // proto: int QOpenGLTimeMonitor::sampleCount();
  fn _ZNK18QOpenGLTimeMonitor11sampleCountEv() -> i32;
  // proto: void QOpenGLTimeMonitor::destroy();
  fn _ZN18QOpenGLTimeMonitor7destroyEv() -> i32;
  // proto: void QOpenGLTimeMonitor::NewQOpenGLTimeMonitor(const QOpenGLTimeMonitor & );
  fn _ZN18QOpenGLTimeMonitorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QOpenGLTimeMonitor::create();
  fn _ZN18QOpenGLTimeMonitor6createEv() -> i32;
  // proto: void QOpenGLTimeMonitor::FreeQOpenGLTimeMonitor();
  fn _ZN18QOpenGLTimeMonitorD0Ev() -> i32;
  // proto: bool QOpenGLTimeMonitor::isResultAvailable();
  fn _ZNK18QOpenGLTimeMonitor17isResultAvailableEv() -> i32;
  // proto: QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
  fn _ZNK18QOpenGLTimeMonitor16waitForIntervalsEv() -> i32;
  // proto: QVector<GLuint> QOpenGLTimeMonitor::objectIds();
  fn _ZNK18QOpenGLTimeMonitor9objectIdsEv() -> i32;
  // proto: int QOpenGLTimeMonitor::recordSample();
  fn _ZN18QOpenGLTimeMonitor12recordSampleEv() -> i32;
  // proto: void QOpenGLTimeMonitor::reset();
  fn _ZN18QOpenGLTimeMonitor5resetEv() -> i32;
  // proto: void QOpenGLTimeMonitor::NewQOpenGLTimeMonitor(QObject * parent);
  fn _ZN18QOpenGLTimeMonitorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
  fn _ZNK18QOpenGLTimeMonitor14waitForSamplesEv() -> i32;
  // proto: bool QOpenGLTimeMonitor::isCreated();
  fn _ZNK18QOpenGLTimeMonitor9isCreatedEv() -> i32;
  // proto: const QMetaObject * QOpenGLTimeMonitor::metaObject();
  fn _ZNK18QOpenGLTimeMonitor10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLTimeMonitor)=1
pub struct QOpenGLTimeMonitor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn setSampleCount<T: QOpenGLTimeMonitor_setSampleCount>(&mut self, value: T) -> i32 {
    value.setSampleCount(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_setSampleCount {
  fn setSampleCount(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
impl<'a> /*trait*/ QOpenGLTimeMonitor_setSampleCount for (i32) {
  fn setSampleCount(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor14setSampleCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QOpenGLTimeMonitor14setSampleCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn sampleCount<T: QOpenGLTimeMonitor_sampleCount>(&mut self, value: T) -> i32 {
    value.sampleCount(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_sampleCount {
  fn sampleCount(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: int QOpenGLTimeMonitor::sampleCount();
impl<'a> /*trait*/ QOpenGLTimeMonitor_sampleCount for () {
  fn sampleCount(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor11sampleCountEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor11sampleCountEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn destroy<T: QOpenGLTimeMonitor_destroy>(&mut self, value: T) -> i32 {
    value.destroy(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_destroy {
  fn destroy(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: void QOpenGLTimeMonitor::destroy();
impl<'a> /*trait*/ QOpenGLTimeMonitor_destroy for () {
  fn destroy(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor7destroyEv()};
    unsafe {_ZN18QOpenGLTimeMonitor7destroyEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QOpenGLTimeMonitorC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLTimeMonitor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn create<T: QOpenGLTimeMonitor_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_create {
  fn create(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: bool QOpenGLTimeMonitor::create();
impl<'a> /*trait*/ QOpenGLTimeMonitor_create for () {
  fn create(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor6createEv()};
    unsafe {_ZN18QOpenGLTimeMonitor6createEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn FreeQOpenGLTimeMonitor<T: QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLTimeMonitor(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor {
  fn FreeQOpenGLTimeMonitor(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: void QOpenGLTimeMonitor::FreeQOpenGLTimeMonitor();
impl<'a> /*trait*/ QOpenGLTimeMonitor_FreeQOpenGLTimeMonitor for () {
  fn FreeQOpenGLTimeMonitor(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorD0Ev()};
    unsafe {_ZN18QOpenGLTimeMonitorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isResultAvailable<T: QOpenGLTimeMonitor_isResultAvailable>(&mut self, value: T) -> i32 {
    value.isResultAvailable(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_isResultAvailable {
  fn isResultAvailable(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: bool QOpenGLTimeMonitor::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isResultAvailable for () {
  fn isResultAvailable(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor17isResultAvailableEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor17isResultAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForIntervals<T: QOpenGLTimeMonitor_waitForIntervals>(&mut self, value: T) -> i32 {
    value.waitForIntervals(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForIntervals {
  fn waitForIntervals(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForIntervals for () {
  fn waitForIntervals(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn objectIds<T: QOpenGLTimeMonitor_objectIds>(&mut self, value: T) -> i32 {
    value.objectIds(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_objectIds {
  fn objectIds(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: QVector<GLuint> QOpenGLTimeMonitor::objectIds();
impl<'a> /*trait*/ QOpenGLTimeMonitor_objectIds for () {
  fn objectIds(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9objectIdsEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor9objectIdsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn recordSample<T: QOpenGLTimeMonitor_recordSample>(&mut self, value: T) -> i32 {
    value.recordSample(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_recordSample {
  fn recordSample(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: int QOpenGLTimeMonitor::recordSample();
impl<'a> /*trait*/ QOpenGLTimeMonitor_recordSample for () {
  fn recordSample(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor12recordSampleEv()};
    unsafe {_ZN18QOpenGLTimeMonitor12recordSampleEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn reset<T: QOpenGLTimeMonitor_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_reset {
  fn reset(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: void QOpenGLTimeMonitor::reset();
impl<'a> /*trait*/ QOpenGLTimeMonitor_reset for () {
  fn reset(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor5resetEv()};
    unsafe {_ZN18QOpenGLTimeMonitor5resetEv()};
    return 1;
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
  pub fn waitForSamples<T: QOpenGLTimeMonitor_waitForSamples>(&mut self, value: T) -> i32 {
    value.waitForSamples(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForSamples {
  fn waitForSamples(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForSamples for () {
  fn waitForSamples(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor14waitForSamplesEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor14waitForSamplesEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isCreated<T: QOpenGLTimeMonitor_isCreated>(&mut self, value: T) -> i32 {
    value.isCreated(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_isCreated {
  fn isCreated(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: bool QOpenGLTimeMonitor::isCreated();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isCreated for () {
  fn isCreated(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9isCreatedEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor9isCreatedEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn metaObject<T: QOpenGLTimeMonitor_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLTimeMonitor_metaObject {
  fn metaObject(self, this: &mut QOpenGLTimeMonitor) -> i32;
}

// proto: const QMetaObject * QOpenGLTimeMonitor::metaObject();
impl<'a> /*trait*/ QOpenGLTimeMonitor_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor10metaObjectEv()};
    unsafe {_ZNK18QOpenGLTimeMonitor10metaObjectEv()};
    return 1;
  }
}

