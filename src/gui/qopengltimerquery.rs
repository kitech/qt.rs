// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtGui/qopengltimerquery.h
// dst-file: /src/gui/qopengltimerquery.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLTimerQuery_Class_Size() -> c_int;
  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(const QOpenGLTimerQuery & );
  fn _ZN17QOpenGLTimerQueryC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QOpenGLTimerQuery::create();
  fn _ZN17QOpenGLTimerQuery6createEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QOpenGLTimerQuery::isCreated();
  fn _ZNK17QOpenGLTimerQuery9isCreatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLTimerQuery::end();
  fn _ZN17QOpenGLTimerQuery3endEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLTimerQuery::~QOpenGLTimerQuery();
  fn _ZN17QOpenGLTimerQueryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLTimerQuery::begin();
  fn _ZN17QOpenGLTimerQuery5beginEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(QObject * parent);
  fn _ZN17QOpenGLTimerQueryC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLTimerQuery::destroy();
  fn _ZN17QOpenGLTimerQuery7destroyEv(qthis: u64 /* *mut c_void*/);
  // proto:  GLuint64 QOpenGLTimerQuery::waitForResult();
  fn _ZNK17QOpenGLTimerQuery13waitForResultEv(qthis: u64 /* *mut c_void*/) -> c_ulong;
  // proto:  GLuint QOpenGLTimerQuery::objectId();
  fn _ZNK17QOpenGLTimerQuery8objectIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  GLuint64 QOpenGLTimerQuery::waitForTimestamp();
  fn _ZNK17QOpenGLTimerQuery16waitForTimestampEv(qthis: u64 /* *mut c_void*/) -> c_ulong;
  // proto:  const QMetaObject * QOpenGLTimerQuery::metaObject();
  fn _ZNK17QOpenGLTimerQuery10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLTimerQuery::recordTimestamp();
  fn _ZN17QOpenGLTimerQuery15recordTimestampEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLTimerQuery::isResultAvailable();
  fn _ZNK17QOpenGLTimerQuery17isResultAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QOpenGLTimeMonitor_Class_Size() -> c_int;
  // proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
  fn _ZN18QOpenGLTimeMonitor14setSampleCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QOpenGLTimeMonitor::sampleCount();
  fn _ZNK18QOpenGLTimeMonitor11sampleCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QOpenGLTimeMonitor::destroy();
  fn _ZN18QOpenGLTimeMonitor7destroyEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLTimeMonitor::QOpenGLTimeMonitor(const QOpenGLTimeMonitor & );
  fn _ZN18QOpenGLTimeMonitorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QOpenGLTimeMonitor::create();
  fn _ZN18QOpenGLTimeMonitor6createEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLTimeMonitor::~QOpenGLTimeMonitor();
  fn _ZN18QOpenGLTimeMonitorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLTimeMonitor::isResultAvailable();
  fn _ZNK18QOpenGLTimeMonitor17isResultAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
  fn _ZNK18QOpenGLTimeMonitor16waitForIntervalsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
  fn _ZNK18QOpenGLTimeMonitor9objectIdsEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QOpenGLTimeMonitor::recordSample();
  fn _ZN18QOpenGLTimeMonitor12recordSampleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QOpenGLTimeMonitor::reset();
  fn _ZN18QOpenGLTimeMonitor5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLTimeMonitor::QOpenGLTimeMonitor(QObject * parent);
  fn _ZN18QOpenGLTimeMonitorC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
  fn _ZNK18QOpenGLTimeMonitor14waitForSamplesEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLTimeMonitor::isCreated();
  fn _ZNK18QOpenGLTimeMonitor9isCreatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
  fn _ZNK18QOpenGLTimeMonitor10metaObjectEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLTimerQuery)=1
#[derive(Default)]
pub struct QOpenGLTimerQuery {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLTimeMonitor)=1
#[derive(Default)]
pub struct QOpenGLTimeMonitor {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLTimerQuery {
    return QOpenGLTimerQuery{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLTimerQuery {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLTimerQuery {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(const QOpenGLTimerQuery & );
impl /*struct*/ QOpenGLTimerQuery {
  pub fn new<T: QOpenGLTimerQuery_new>(value: T) -> QOpenGLTimerQuery {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_new {
  fn new(self) -> QOpenGLTimerQuery;
}

  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(const QOpenGLTimerQuery & );
impl<'a> /*trait*/ QOpenGLTimerQuery_new for (&'a QOpenGLTimerQuery) {
  fn new(self) -> QOpenGLTimerQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryC2ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLTimerQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOpenGLTimerQueryC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QOpenGLTimerQuery{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLTimerQuery::create();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn create<RetType, T: QOpenGLTimerQuery_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_create<RetType> {
  fn create(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  bool QOpenGLTimerQuery::create();
impl<'a> /*trait*/ QOpenGLTimerQuery_create<i8> for () {
  fn create(self , rsthis: & QOpenGLTimerQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery6createEv()};
    let mut ret = unsafe {_ZN17QOpenGLTimerQuery6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QOpenGLTimerQuery::isCreated();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn isCreated<RetType, T: QOpenGLTimerQuery_isCreated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_isCreated<RetType> {
  fn isCreated(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  bool QOpenGLTimerQuery::isCreated();
impl<'a> /*trait*/ QOpenGLTimerQuery_isCreated<i8> for () {
  fn isCreated(self , rsthis: & QOpenGLTimerQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery9isCreatedEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::end();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn end<RetType, T: QOpenGLTimerQuery_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_end<RetType> {
  fn end(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::end();
impl<'a> /*trait*/ QOpenGLTimerQuery_end<()> for () {
  fn end(self , rsthis: & QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery3endEv()};
     unsafe {_ZN17QOpenGLTimerQuery3endEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::~QOpenGLTimerQuery();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn free<RetType, T: QOpenGLTimerQuery_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_free<RetType> {
  fn free(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::~QOpenGLTimerQuery();
impl<'a> /*trait*/ QOpenGLTimerQuery_free<()> for () {
  fn free(self , rsthis: & QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryD2Ev()};
     unsafe {_ZN17QOpenGLTimerQueryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::begin();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn begin<RetType, T: QOpenGLTimerQuery_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_begin<RetType> {
  fn begin(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::begin();
impl<'a> /*trait*/ QOpenGLTimerQuery_begin<()> for () {
  fn begin(self , rsthis: & QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery5beginEv()};
     unsafe {_ZN17QOpenGLTimerQuery5beginEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(QObject * parent);
impl<'a> /*trait*/ QOpenGLTimerQuery_new for (&'a QObject) {
  fn new(self) -> QOpenGLTimerQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryC2EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLTimerQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOpenGLTimerQueryC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QOpenGLTimerQuery{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::destroy();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn destroy<RetType, T: QOpenGLTimerQuery_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_destroy<RetType> {
  fn destroy(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::destroy();
impl<'a> /*trait*/ QOpenGLTimerQuery_destroy<()> for () {
  fn destroy(self , rsthis: & QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery7destroyEv()};
     unsafe {_ZN17QOpenGLTimerQuery7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForResult();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn waitForResult<RetType, T: QOpenGLTimerQuery_waitForResult<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForResult(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_waitForResult<RetType> {
  fn waitForResult(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForResult();
impl<'a> /*trait*/ QOpenGLTimerQuery_waitForResult<u64> for () {
  fn waitForResult(self , rsthis: & QOpenGLTimerQuery) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery13waitForResultEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery13waitForResultEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLTimerQuery::objectId();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn objectId<RetType, T: QOpenGLTimerQuery_objectId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_objectId<RetType> {
  fn objectId(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  GLuint QOpenGLTimerQuery::objectId();
impl<'a> /*trait*/ QOpenGLTimerQuery_objectId<u32> for () {
  fn objectId(self , rsthis: & QOpenGLTimerQuery) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery8objectIdEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery8objectIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForTimestamp();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn waitForTimestamp<RetType, T: QOpenGLTimerQuery_waitForTimestamp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForTimestamp(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_waitForTimestamp<RetType> {
  fn waitForTimestamp(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForTimestamp();
impl<'a> /*trait*/ QOpenGLTimerQuery_waitForTimestamp<u64> for () {
  fn waitForTimestamp(self , rsthis: & QOpenGLTimerQuery) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery16waitForTimestampEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery16waitForTimestampEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLTimerQuery::metaObject();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn metaObject<RetType, T: QOpenGLTimerQuery_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLTimerQuery::metaObject();
impl<'a> /*trait*/ QOpenGLTimerQuery_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery10metaObjectEv()};
     unsafe {_ZNK17QOpenGLTimerQuery10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::recordTimestamp();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn recordTimestamp<RetType, T: QOpenGLTimerQuery_recordTimestamp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.recordTimestamp(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_recordTimestamp<RetType> {
  fn recordTimestamp(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::recordTimestamp();
impl<'a> /*trait*/ QOpenGLTimerQuery_recordTimestamp<()> for () {
  fn recordTimestamp(self , rsthis: & QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery15recordTimestampEv()};
     unsafe {_ZN17QOpenGLTimerQuery15recordTimestampEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLTimerQuery::isResultAvailable();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn isResultAvailable<RetType, T: QOpenGLTimerQuery_isResultAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isResultAvailable(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_isResultAvailable<RetType> {
  fn isResultAvailable(self , rsthis: & QOpenGLTimerQuery) -> RetType;
}

  // proto:  bool QOpenGLTimerQuery::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimerQuery_isResultAvailable<i8> for () {
  fn isResultAvailable(self , rsthis: & QOpenGLTimerQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery17isResultAvailableEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery17isResultAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimeMonitor {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLTimeMonitor {
    return QOpenGLTimeMonitor{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLTimeMonitor {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLTimeMonitor {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn setSampleCount<RetType, T: QOpenGLTimeMonitor_setSampleCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSampleCount(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_setSampleCount<RetType> {
  fn setSampleCount(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  void QOpenGLTimeMonitor::setSampleCount(int sampleCount);
impl<'a> /*trait*/ QOpenGLTimeMonitor_setSampleCount<()> for (i32) {
  fn setSampleCount(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor14setSampleCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QOpenGLTimeMonitor14setSampleCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QOpenGLTimeMonitor::sampleCount();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn sampleCount<RetType, T: QOpenGLTimeMonitor_sampleCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sampleCount(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_sampleCount<RetType> {
  fn sampleCount(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  int QOpenGLTimeMonitor::sampleCount();
impl<'a> /*trait*/ QOpenGLTimeMonitor_sampleCount<i32> for () {
  fn sampleCount(self , rsthis: & QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor11sampleCountEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor11sampleCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLTimeMonitor::destroy();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn destroy<RetType, T: QOpenGLTimeMonitor_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_destroy<RetType> {
  fn destroy(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  void QOpenGLTimeMonitor::destroy();
impl<'a> /*trait*/ QOpenGLTimeMonitor_destroy<()> for () {
  fn destroy(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor7destroyEv()};
     unsafe {_ZN18QOpenGLTimeMonitor7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimeMonitor::QOpenGLTimeMonitor(const QOpenGLTimeMonitor & );
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn new<T: QOpenGLTimeMonitor_new>(value: T) -> QOpenGLTimeMonitor {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_new {
  fn new(self) -> QOpenGLTimeMonitor;
}

  // proto:  void QOpenGLTimeMonitor::QOpenGLTimeMonitor(const QOpenGLTimeMonitor & );
impl<'a> /*trait*/ QOpenGLTimeMonitor_new for (&'a QOpenGLTimeMonitor) {
  fn new(self) -> QOpenGLTimeMonitor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorC2ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLTimeMonitor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLTimeMonitorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QOpenGLTimeMonitor{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLTimeMonitor::create();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn create<RetType, T: QOpenGLTimeMonitor_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_create<RetType> {
  fn create(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  bool QOpenGLTimeMonitor::create();
impl<'a> /*trait*/ QOpenGLTimeMonitor_create<i8> for () {
  fn create(self , rsthis: & QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor6createEv()};
    let mut ret = unsafe {_ZN18QOpenGLTimeMonitor6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLTimeMonitor::~QOpenGLTimeMonitor();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn free<RetType, T: QOpenGLTimeMonitor_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_free<RetType> {
  fn free(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  void QOpenGLTimeMonitor::~QOpenGLTimeMonitor();
impl<'a> /*trait*/ QOpenGLTimeMonitor_free<()> for () {
  fn free(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorD2Ev()};
     unsafe {_ZN18QOpenGLTimeMonitorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLTimeMonitor::isResultAvailable();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isResultAvailable<RetType, T: QOpenGLTimeMonitor_isResultAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isResultAvailable(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_isResultAvailable<RetType> {
  fn isResultAvailable(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  bool QOpenGLTimeMonitor::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isResultAvailable<i8> for () {
  fn isResultAvailable(self , rsthis: & QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor17isResultAvailableEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor17isResultAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForIntervals<RetType, T: QOpenGLTimeMonitor_waitForIntervals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForIntervals(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForIntervals<RetType> {
  fn waitForIntervals(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForIntervals<()> for () {
  fn waitForIntervals(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor16waitForIntervalsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn objectIds<RetType, T: QOpenGLTimeMonitor_objectIds<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectIds(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_objectIds<RetType> {
  fn objectIds(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  QVector<GLuint> QOpenGLTimeMonitor::objectIds();
impl<'a> /*trait*/ QOpenGLTimeMonitor_objectIds<()> for () {
  fn objectIds(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9objectIdsEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor9objectIdsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QOpenGLTimeMonitor::recordSample();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn recordSample<RetType, T: QOpenGLTimeMonitor_recordSample<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.recordSample(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_recordSample<RetType> {
  fn recordSample(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  int QOpenGLTimeMonitor::recordSample();
impl<'a> /*trait*/ QOpenGLTimeMonitor_recordSample<i32> for () {
  fn recordSample(self , rsthis: & QOpenGLTimeMonitor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor12recordSampleEv()};
    let mut ret = unsafe {_ZN18QOpenGLTimeMonitor12recordSampleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLTimeMonitor::reset();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn reset<RetType, T: QOpenGLTimeMonitor_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_reset<RetType> {
  fn reset(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  void QOpenGLTimeMonitor::reset();
impl<'a> /*trait*/ QOpenGLTimeMonitor_reset<()> for () {
  fn reset(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitor5resetEv()};
     unsafe {_ZN18QOpenGLTimeMonitor5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimeMonitor::QOpenGLTimeMonitor(QObject * parent);
impl<'a> /*trait*/ QOpenGLTimeMonitor_new for (&'a QObject) {
  fn new(self) -> QOpenGLTimeMonitor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLTimeMonitorC2EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLTimeMonitor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLTimeMonitorC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QOpenGLTimeMonitor{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn waitForSamples<RetType, T: QOpenGLTimeMonitor_waitForSamples<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForSamples(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_waitForSamples<RetType> {
  fn waitForSamples(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples();
impl<'a> /*trait*/ QOpenGLTimeMonitor_waitForSamples<()> for () {
  fn waitForSamples(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor14waitForSamplesEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor14waitForSamplesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLTimeMonitor::isCreated();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn isCreated<RetType, T: QOpenGLTimeMonitor_isCreated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_isCreated<RetType> {
  fn isCreated(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  bool QOpenGLTimeMonitor::isCreated();
impl<'a> /*trait*/ QOpenGLTimeMonitor_isCreated<i8> for () {
  fn isCreated(self , rsthis: & QOpenGLTimeMonitor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor9isCreatedEv()};
    let mut ret = unsafe {_ZNK18QOpenGLTimeMonitor9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
impl /*struct*/ QOpenGLTimeMonitor {
  pub fn metaObject<RetType, T: QOpenGLTimeMonitor_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLTimeMonitor_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLTimeMonitor) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLTimeMonitor::metaObject();
impl<'a> /*trait*/ QOpenGLTimeMonitor_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLTimeMonitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLTimeMonitor10metaObjectEv()};
     unsafe {_ZNK18QOpenGLTimeMonitor10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

