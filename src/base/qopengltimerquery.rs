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
  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(const QOpenGLTimerQuery & );
  fn _ZN17QOpenGLTimerQueryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QOpenGLTimerQuery::create();
  fn _ZN17QOpenGLTimerQuery6createEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QOpenGLTimerQuery::isCreated();
  fn _ZNK17QOpenGLTimerQuery9isCreatedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLTimerQuery::end();
  fn _ZN17QOpenGLTimerQuery3endEv(qthis: *mut c_void);
  // proto:  void QOpenGLTimerQuery::~QOpenGLTimerQuery();
  fn _ZN17QOpenGLTimerQueryD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLTimerQuery::begin();
  fn _ZN17QOpenGLTimerQuery5beginEv(qthis: *mut c_void);
  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(QObject * parent);
  fn _ZN17QOpenGLTimerQueryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLTimerQuery::destroy();
  fn _ZN17QOpenGLTimerQuery7destroyEv(qthis: *mut c_void);
  // proto:  GLuint64 QOpenGLTimerQuery::waitForResult();
  fn _ZNK17QOpenGLTimerQuery13waitForResultEv(qthis: *mut c_void) -> c_ulong;
  // proto:  GLuint QOpenGLTimerQuery::objectId();
  fn _ZNK17QOpenGLTimerQuery8objectIdEv(qthis: *mut c_void);
  // proto:  GLuint64 QOpenGLTimerQuery::waitForTimestamp();
  fn _ZNK17QOpenGLTimerQuery16waitForTimestampEv(qthis: *mut c_void) -> c_ulong;
  // proto:  const QMetaObject * QOpenGLTimerQuery::metaObject();
  fn _ZNK17QOpenGLTimerQuery10metaObjectEv(qthis: *mut c_void);
  // proto:  void QOpenGLTimerQuery::recordTimestamp();
  fn _ZN17QOpenGLTimerQuery15recordTimestampEv(qthis: *mut c_void);
  // proto:  bool QOpenGLTimerQuery::isResultAvailable();
  fn _ZNK17QOpenGLTimerQuery17isResultAvailableEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QOpenGLTimerQuery)=1
pub struct QOpenGLTimerQuery {
  pub qclsinst: *mut c_void,
}

  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(const QOpenGLTimerQuery & );
impl /*struct*/ QOpenGLTimerQuery {
  pub fn NewQOpenGLTimerQuery<T: QOpenGLTimerQuery_NewQOpenGLTimerQuery>(value: T) -> QOpenGLTimerQuery {
    let rsthis = value.NewQOpenGLTimerQuery();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_NewQOpenGLTimerQuery {
  fn NewQOpenGLTimerQuery(self) -> QOpenGLTimerQuery;
}

  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(const QOpenGLTimerQuery & );
impl<'a> /*trait*/ QOpenGLTimerQuery_NewQOpenGLTimerQuery for (QOpenGLTimerQuery) {
  fn NewQOpenGLTimerQuery(self) -> QOpenGLTimerQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOpenGLTimerQueryC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLTimerQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLTimerQuery::create();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn create<RetType, T: QOpenGLTimerQuery_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_create<RetType> {
  fn create(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  bool QOpenGLTimerQuery::create();
impl<'a> /*trait*/ QOpenGLTimerQuery_create<i8> for () {
  fn create(self , rsthis: &mut QOpenGLTimerQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery6createEv()};
    let mut ret = unsafe {_ZN17QOpenGLTimerQuery6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QOpenGLTimerQuery::isCreated();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn isCreated<RetType, T: QOpenGLTimerQuery_isCreated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_isCreated<RetType> {
  fn isCreated(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  bool QOpenGLTimerQuery::isCreated();
impl<'a> /*trait*/ QOpenGLTimerQuery_isCreated<i8> for () {
  fn isCreated(self , rsthis: &mut QOpenGLTimerQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery9isCreatedEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::end();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn end<RetType, T: QOpenGLTimerQuery_end<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_end<RetType> {
  fn end(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::end();
impl<'a> /*trait*/ QOpenGLTimerQuery_end<()> for () {
  fn end(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery3endEv()};
     unsafe {_ZN17QOpenGLTimerQuery3endEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::~QOpenGLTimerQuery();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn FreeQOpenGLTimerQuery<RetType, T: QOpenGLTimerQuery_FreeQOpenGLTimerQuery<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLTimerQuery(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_FreeQOpenGLTimerQuery<RetType> {
  fn FreeQOpenGLTimerQuery(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::~QOpenGLTimerQuery();
impl<'a> /*trait*/ QOpenGLTimerQuery_FreeQOpenGLTimerQuery<()> for () {
  fn FreeQOpenGLTimerQuery(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryD0Ev()};
     unsafe {_ZN17QOpenGLTimerQueryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::begin();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn begin<RetType, T: QOpenGLTimerQuery_begin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_begin<RetType> {
  fn begin(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::begin();
impl<'a> /*trait*/ QOpenGLTimerQuery_begin<()> for () {
  fn begin(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery5beginEv()};
     unsafe {_ZN17QOpenGLTimerQuery5beginEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::QOpenGLTimerQuery(QObject * parent);
impl<'a> /*trait*/ QOpenGLTimerQuery_NewQOpenGLTimerQuery for (QObject) {
  fn NewQOpenGLTimerQuery(self) -> QOpenGLTimerQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOpenGLTimerQueryC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLTimerQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::destroy();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn destroy<RetType, T: QOpenGLTimerQuery_destroy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_destroy<RetType> {
  fn destroy(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::destroy();
impl<'a> /*trait*/ QOpenGLTimerQuery_destroy<()> for () {
  fn destroy(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery7destroyEv()};
     unsafe {_ZN17QOpenGLTimerQuery7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForResult();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn waitForResult<RetType, T: QOpenGLTimerQuery_waitForResult<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.waitForResult(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_waitForResult<RetType> {
  fn waitForResult(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForResult();
impl<'a> /*trait*/ QOpenGLTimerQuery_waitForResult<u64> for () {
  fn waitForResult(self , rsthis: &mut QOpenGLTimerQuery) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery13waitForResultEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery13waitForResultEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLTimerQuery::objectId();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn objectId<RetType, T: QOpenGLTimerQuery_objectId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_objectId<RetType> {
  fn objectId(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  GLuint QOpenGLTimerQuery::objectId();
impl<'a> /*trait*/ QOpenGLTimerQuery_objectId<()> for () {
  fn objectId(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery8objectIdEv()};
     unsafe {_ZNK17QOpenGLTimerQuery8objectIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForTimestamp();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn waitForTimestamp<RetType, T: QOpenGLTimerQuery_waitForTimestamp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.waitForTimestamp(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_waitForTimestamp<RetType> {
  fn waitForTimestamp(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  GLuint64 QOpenGLTimerQuery::waitForTimestamp();
impl<'a> /*trait*/ QOpenGLTimerQuery_waitForTimestamp<u64> for () {
  fn waitForTimestamp(self , rsthis: &mut QOpenGLTimerQuery) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery16waitForTimestampEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery16waitForTimestampEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLTimerQuery::metaObject();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn metaObject<RetType, T: QOpenGLTimerQuery_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLTimerQuery::metaObject();
impl<'a> /*trait*/ QOpenGLTimerQuery_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery10metaObjectEv()};
     unsafe {_ZNK17QOpenGLTimerQuery10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLTimerQuery::recordTimestamp();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn recordTimestamp<RetType, T: QOpenGLTimerQuery_recordTimestamp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.recordTimestamp(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_recordTimestamp<RetType> {
  fn recordTimestamp(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  void QOpenGLTimerQuery::recordTimestamp();
impl<'a> /*trait*/ QOpenGLTimerQuery_recordTimestamp<()> for () {
  fn recordTimestamp(self , rsthis: &mut QOpenGLTimerQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery15recordTimestampEv()};
     unsafe {_ZN17QOpenGLTimerQuery15recordTimestampEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLTimerQuery::isResultAvailable();
impl /*struct*/ QOpenGLTimerQuery {
  pub fn isResultAvailable<RetType, T: QOpenGLTimerQuery_isResultAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isResultAvailable(self);
    // return 1;
  }
}

pub trait QOpenGLTimerQuery_isResultAvailable<RetType> {
  fn isResultAvailable(self , rsthis: &mut QOpenGLTimerQuery) -> RetType;
}

  // proto:  bool QOpenGLTimerQuery::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimerQuery_isResultAvailable<i8> for () {
  fn isResultAvailable(self , rsthis: &mut QOpenGLTimerQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery17isResultAvailableEv()};
    let mut ret = unsafe {_ZNK17QOpenGLTimerQuery17isResultAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

