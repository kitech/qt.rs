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
  // proto:  void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
  fn _ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QOpenGLVertexArrayObject::GLuint QOpenGLVertexArrayObject::objectId();
  fn _ZNK24QOpenGLVertexArrayObject8objectIdEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLVertexArrayObject::release();
  fn _ZN24QOpenGLVertexArrayObject7releaseEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
  fn _ZNK24QOpenGLVertexArrayObject10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(QObject * parent);
  fn _ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLVertexArrayObject::bind();
  fn _ZN24QOpenGLVertexArrayObject4bindEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLVertexArrayObject::isCreated();
  fn _ZNK24QOpenGLVertexArrayObject9isCreatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLVertexArrayObject::destroy();
  fn _ZN24QOpenGLVertexArrayObject7destroyEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLVertexArrayObject::create();
  fn _ZN24QOpenGLVertexArrayObject6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLVertexArrayObject::FreeQOpenGLVertexArrayObject();
  fn _ZN24QOpenGLVertexArrayObjectD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLVertexArrayObject)=1
pub struct QOpenGLVertexArrayObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn NewQOpenGLVertexArrayObject<T: QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject>(value: T) -> QOpenGLVertexArrayObject {
    let rsthis = value.NewQOpenGLVertexArrayObject();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject;
}

// proto: void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
impl<'a> /*trait*/ QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject for (&'a  QOpenGLVertexArrayObject) {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLVertexArrayObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn objectId<T: QOpenGLVertexArrayObject_objectId>(&mut self, value: T)  {
     value.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_objectId {
  fn objectId(self, rsthis: &mut QOpenGLVertexArrayObject) ;
}

// proto:  QOpenGLVertexArrayObject::GLuint QOpenGLVertexArrayObject::objectId();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_objectId for () {
  fn objectId(self, rsthis: &mut QOpenGLVertexArrayObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject8objectIdEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject8objectIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn release<T: QOpenGLVertexArrayObject_release>(&mut self, value: T)  {
     value.release(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_release {
  fn release(self, rsthis: &mut QOpenGLVertexArrayObject) ;
}

// proto:  void QOpenGLVertexArrayObject::release();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_release for () {
  fn release(self, rsthis: &mut QOpenGLVertexArrayObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7releaseEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn metaObject<T: QOpenGLVertexArrayObject_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLVertexArrayObject) ;
}

// proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLVertexArrayObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject10metaObjectEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(QObject * parent);
impl<'a> /*trait*/ QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject for (&'a mut QObject) {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLVertexArrayObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn bind<T: QOpenGLVertexArrayObject_bind>(&mut self, value: T)  {
     value.bind(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_bind {
  fn bind(self, rsthis: &mut QOpenGLVertexArrayObject) ;
}

// proto:  void QOpenGLVertexArrayObject::bind();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_bind for () {
  fn bind(self, rsthis: &mut QOpenGLVertexArrayObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject4bindEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn isCreated<T: QOpenGLVertexArrayObject_isCreated>(&mut self, value: T) -> i8 {
    return value.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_isCreated {
  fn isCreated(self, rsthis: &mut QOpenGLVertexArrayObject) -> i8;
}

// proto:  bool QOpenGLVertexArrayObject::isCreated();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_isCreated for () {
  fn isCreated(self, rsthis: &mut QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject9isCreatedEv()};
    let mut ret = unsafe {_ZNK24QOpenGLVertexArrayObject9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn destroy<T: QOpenGLVertexArrayObject_destroy>(&mut self, value: T)  {
     value.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_destroy {
  fn destroy(self, rsthis: &mut QOpenGLVertexArrayObject) ;
}

// proto:  void QOpenGLVertexArrayObject::destroy();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_destroy for () {
  fn destroy(self, rsthis: &mut QOpenGLVertexArrayObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7destroyEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn create<T: QOpenGLVertexArrayObject_create>(&mut self, value: T) -> i8 {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_create {
  fn create(self, rsthis: &mut QOpenGLVertexArrayObject) -> i8;
}

// proto:  bool QOpenGLVertexArrayObject::create();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_create for () {
  fn create(self, rsthis: &mut QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject6createEv()};
    let mut ret = unsafe {_ZN24QOpenGLVertexArrayObject6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn FreeQOpenGLVertexArrayObject<T: QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject>(&mut self, value: T)  {
     value.FreeQOpenGLVertexArrayObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject {
  fn FreeQOpenGLVertexArrayObject(self, rsthis: &mut QOpenGLVertexArrayObject) ;
}

// proto:  void QOpenGLVertexArrayObject::FreeQOpenGLVertexArrayObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject for () {
  fn FreeQOpenGLVertexArrayObject(self, rsthis: &mut QOpenGLVertexArrayObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectD0Ev()};
     unsafe {_ZN24QOpenGLVertexArrayObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

