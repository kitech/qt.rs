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
  // proto: void QOpenGLTimerQuery::NewQOpenGLTimerQuery(const QOpenGLTimerQuery & );
  fn _ZN17QOpenGLTimerQueryC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QOpenGLTimerQuery::create();
  fn _ZN17QOpenGLTimerQuery6createEv() -> i32;
  // proto: bool QOpenGLTimerQuery::isCreated();
  fn _ZNK17QOpenGLTimerQuery9isCreatedEv() -> i32;
  // proto: void QOpenGLTimerQuery::end();
  fn _ZN17QOpenGLTimerQuery3endEv() -> i32;
  // proto: void QOpenGLTimerQuery::FreeQOpenGLTimerQuery();
  fn _ZN17QOpenGLTimerQueryD0Ev() -> i32;
  // proto: void QOpenGLTimerQuery::begin();
  fn _ZN17QOpenGLTimerQuery5beginEv() -> i32;
  // proto: void QOpenGLTimerQuery::NewQOpenGLTimerQuery(QObject * parent);
  fn _ZN17QOpenGLTimerQueryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QOpenGLTimerQuery::destroy();
  fn _ZN17QOpenGLTimerQuery7destroyEv() -> i32;
  // proto: uint64_t QOpenGLTimerQuery::waitForResult();
  fn _ZNK17QOpenGLTimerQuery13waitForResultEv() -> i32;
  // proto: QOpenGLTimerQuery::GLuint QOpenGLTimerQuery::objectId();
  fn _ZNK17QOpenGLTimerQuery8objectIdEv() -> i32;
  // proto: uint64_t QOpenGLTimerQuery::waitForTimestamp();
  fn _ZNK17QOpenGLTimerQuery16waitForTimestampEv() -> i32;
  // proto: const QMetaObject * QOpenGLTimerQuery::metaObject();
  fn _ZNK17QOpenGLTimerQuery10metaObjectEv() -> i32;
  // proto: void QOpenGLTimerQuery::recordTimestamp();
  fn _ZN17QOpenGLTimerQuery15recordTimestampEv() -> i32;
  // proto: bool QOpenGLTimerQuery::isResultAvailable();
  fn _ZNK17QOpenGLTimerQuery17isResultAvailableEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLTimerQuery)=1
pub struct QOpenGLTimerQuery {
  pub qclsinst: *mut c_void,
}

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

// proto: void QOpenGLTimerQuery::NewQOpenGLTimerQuery(const QOpenGLTimerQuery & );
impl<'a> /*trait*/ QOpenGLTimerQuery_NewQOpenGLTimerQuery for (&'a  QOpenGLTimerQuery) {
  fn NewQOpenGLTimerQuery(self) -> QOpenGLTimerQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QOpenGLTimerQueryC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLTimerQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn create<T: QOpenGLTimerQuery_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_create {
  fn create(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: bool QOpenGLTimerQuery::create();
impl<'a> /*trait*/ QOpenGLTimerQuery_create for () {
  fn create(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery6createEv()};
    unsafe {_ZN17QOpenGLTimerQuery6createEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn isCreated<T: QOpenGLTimerQuery_isCreated>(&mut self, value: T) -> i32 {
    value.isCreated(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_isCreated {
  fn isCreated(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: bool QOpenGLTimerQuery::isCreated();
impl<'a> /*trait*/ QOpenGLTimerQuery_isCreated for () {
  fn isCreated(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery9isCreatedEv()};
    unsafe {_ZNK17QOpenGLTimerQuery9isCreatedEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn end<T: QOpenGLTimerQuery_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_end {
  fn end(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: void QOpenGLTimerQuery::end();
impl<'a> /*trait*/ QOpenGLTimerQuery_end for () {
  fn end(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery3endEv()};
    unsafe {_ZN17QOpenGLTimerQuery3endEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn FreeQOpenGLTimerQuery<T: QOpenGLTimerQuery_FreeQOpenGLTimerQuery>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLTimerQuery(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_FreeQOpenGLTimerQuery {
  fn FreeQOpenGLTimerQuery(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: void QOpenGLTimerQuery::FreeQOpenGLTimerQuery();
impl<'a> /*trait*/ QOpenGLTimerQuery_FreeQOpenGLTimerQuery for () {
  fn FreeQOpenGLTimerQuery(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQueryD0Ev()};
    unsafe {_ZN17QOpenGLTimerQueryD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn begin<T: QOpenGLTimerQuery_begin>(&mut self, value: T) -> i32 {
    value.begin(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_begin {
  fn begin(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: void QOpenGLTimerQuery::begin();
impl<'a> /*trait*/ QOpenGLTimerQuery_begin for () {
  fn begin(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery5beginEv()};
    unsafe {_ZN17QOpenGLTimerQuery5beginEv()};
    return 1;
  }
}

// proto: void QOpenGLTimerQuery::NewQOpenGLTimerQuery(QObject * parent);
impl<'a> /*trait*/ QOpenGLTimerQuery_NewQOpenGLTimerQuery for (&'a mut QObject) {
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

impl /*struct*/ QOpenGLTimerQuery {
  pub fn destroy<T: QOpenGLTimerQuery_destroy>(&mut self, value: T) -> i32 {
    value.destroy(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_destroy {
  fn destroy(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: void QOpenGLTimerQuery::destroy();
impl<'a> /*trait*/ QOpenGLTimerQuery_destroy for () {
  fn destroy(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery7destroyEv()};
    unsafe {_ZN17QOpenGLTimerQuery7destroyEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn waitForResult<T: QOpenGLTimerQuery_waitForResult>(&mut self, value: T) -> i32 {
    value.waitForResult(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_waitForResult {
  fn waitForResult(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: uint64_t QOpenGLTimerQuery::waitForResult();
impl<'a> /*trait*/ QOpenGLTimerQuery_waitForResult for () {
  fn waitForResult(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery13waitForResultEv()};
    unsafe {_ZNK17QOpenGLTimerQuery13waitForResultEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn objectId<T: QOpenGLTimerQuery_objectId>(&mut self, value: T) -> i32 {
    value.objectId(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_objectId {
  fn objectId(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: QOpenGLTimerQuery::GLuint QOpenGLTimerQuery::objectId();
impl<'a> /*trait*/ QOpenGLTimerQuery_objectId for () {
  fn objectId(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery8objectIdEv()};
    unsafe {_ZNK17QOpenGLTimerQuery8objectIdEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn waitForTimestamp<T: QOpenGLTimerQuery_waitForTimestamp>(&mut self, value: T) -> i32 {
    value.waitForTimestamp(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_waitForTimestamp {
  fn waitForTimestamp(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: uint64_t QOpenGLTimerQuery::waitForTimestamp();
impl<'a> /*trait*/ QOpenGLTimerQuery_waitForTimestamp for () {
  fn waitForTimestamp(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery16waitForTimestampEv()};
    unsafe {_ZNK17QOpenGLTimerQuery16waitForTimestampEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn metaObject<T: QOpenGLTimerQuery_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_metaObject {
  fn metaObject(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: const QMetaObject * QOpenGLTimerQuery::metaObject();
impl<'a> /*trait*/ QOpenGLTimerQuery_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery10metaObjectEv()};
    unsafe {_ZNK17QOpenGLTimerQuery10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn recordTimestamp<T: QOpenGLTimerQuery_recordTimestamp>(&mut self, value: T) -> i32 {
    value.recordTimestamp(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_recordTimestamp {
  fn recordTimestamp(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: void QOpenGLTimerQuery::recordTimestamp();
impl<'a> /*trait*/ QOpenGLTimerQuery_recordTimestamp for () {
  fn recordTimestamp(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOpenGLTimerQuery15recordTimestampEv()};
    unsafe {_ZN17QOpenGLTimerQuery15recordTimestampEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTimerQuery {
  pub fn isResultAvailable<T: QOpenGLTimerQuery_isResultAvailable>(&mut self, value: T) -> i32 {
    value.isResultAvailable(self);
    return 1;
  }
}

pub trait QOpenGLTimerQuery_isResultAvailable {
  fn isResultAvailable(self, this: &mut QOpenGLTimerQuery) -> i32;
}

// proto: bool QOpenGLTimerQuery::isResultAvailable();
impl<'a> /*trait*/ QOpenGLTimerQuery_isResultAvailable for () {
  fn isResultAvailable(self, this: &mut QOpenGLTimerQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOpenGLTimerQuery17isResultAvailableEv()};
    unsafe {_ZNK17QOpenGLTimerQuery17isResultAvailableEv()};
    return 1;
  }
}

