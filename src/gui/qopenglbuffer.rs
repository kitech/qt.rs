// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qopenglbuffer.h
// dst-file: /src/gui/qopenglbuffer.rs
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
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLBuffer_Class_Size() -> c_int;
  // proto:  bool QOpenGLBuffer::read(int offset, void * data, int count);
  fn _ZN13QOpenGLBuffer4readEiPvi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  bool QOpenGLBuffer::bind();
  fn _ZN13QOpenGLBuffer4bindEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLBuffer::destroy();
  fn _ZN13QOpenGLBuffer7destroyEv(qthis: *mut c_void);
  // proto:  bool QOpenGLBuffer::unmap();
  fn _ZN13QOpenGLBuffer5unmapEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLBuffer::QOpenGLBuffer(const QOpenGLBuffer & other);
  fn dector_ZN13QOpenGLBufferC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QOpenGLBufferC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QOpenGLBuffer::size();
  fn _ZNK13QOpenGLBuffer4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLBuffer::allocate(const void * data, int count);
  fn _ZN13QOpenGLBuffer8allocateEPKvi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  GLuint QOpenGLBuffer::bufferId();
  fn _ZNK13QOpenGLBuffer8bufferIdEv(qthis: *mut c_void);
  // proto:  void QOpenGLBuffer::QOpenGLBuffer();
  fn dector_ZN13QOpenGLBufferC1Ev() -> *mut c_void;
  fn _ZN13QOpenGLBufferC1Ev(qthis: *mut c_void);
  // proto:  bool QOpenGLBuffer::create();
  fn _ZN13QOpenGLBuffer6createEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLBuffer::~QOpenGLBuffer();
  fn _ZN13QOpenGLBufferD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLBuffer::release();
  fn _ZN13QOpenGLBuffer7releaseEv(qthis: *mut c_void);
  // proto:  bool QOpenGLBuffer::isCreated();
  fn _ZNK13QOpenGLBuffer9isCreatedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLBuffer::write(int offset, const void * data, int count);
  fn _ZN13QOpenGLBuffer5writeEiPKvi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLBuffer)=8
pub struct QOpenGLBuffer {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLBuffer {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLBuffer {
    return QOpenGLBuffer{qclsinst: qthis};
  }
}
  // proto:  bool QOpenGLBuffer::read(int offset, void * data, int count);
impl /*struct*/ QOpenGLBuffer {
  pub fn read<RetType, T: QOpenGLBuffer_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_read<RetType> {
  fn read(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  bool QOpenGLBuffer::read(int offset, void * data, int count);
impl<'a> /*trait*/ QOpenGLBuffer_read<i8> for (i32, *mut c_void, i32) {
  fn read(self , rsthis: & QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer4readEiPvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN13QOpenGLBuffer4readEiPvi(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QOpenGLBuffer::bind();
impl /*struct*/ QOpenGLBuffer {
  pub fn bind<RetType, T: QOpenGLBuffer_bind<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_bind<RetType> {
  fn bind(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  bool QOpenGLBuffer::bind();
impl<'a> /*trait*/ QOpenGLBuffer_bind<i8> for () {
  fn bind(self , rsthis: & QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer4bindEv()};
    let mut ret = unsafe {_ZN13QOpenGLBuffer4bindEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::destroy();
impl /*struct*/ QOpenGLBuffer {
  pub fn destroy<RetType, T: QOpenGLBuffer_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_destroy<RetType> {
  fn destroy(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  void QOpenGLBuffer::destroy();
impl<'a> /*trait*/ QOpenGLBuffer_destroy<()> for () {
  fn destroy(self , rsthis: & QOpenGLBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer7destroyEv()};
     unsafe {_ZN13QOpenGLBuffer7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLBuffer::unmap();
impl /*struct*/ QOpenGLBuffer {
  pub fn unmap<RetType, T: QOpenGLBuffer_unmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unmap(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_unmap<RetType> {
  fn unmap(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  bool QOpenGLBuffer::unmap();
impl<'a> /*trait*/ QOpenGLBuffer_unmap<i8> for () {
  fn unmap(self , rsthis: & QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer5unmapEv()};
    let mut ret = unsafe {_ZN13QOpenGLBuffer5unmapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::QOpenGLBuffer(const QOpenGLBuffer & other);
impl /*struct*/ QOpenGLBuffer {
  pub fn New<T: QOpenGLBuffer_New>(value: T) -> QOpenGLBuffer {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLBuffer_New {
  fn New(self) -> QOpenGLBuffer;
}

  // proto:  void QOpenGLBuffer::QOpenGLBuffer(const QOpenGLBuffer & other);
impl<'a> /*trait*/ QOpenGLBuffer_New for (&'a QOpenGLBuffer) {
  fn New(self) -> QOpenGLBuffer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBufferC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLBuffer_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QOpenGLBufferC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QOpenGLBufferC1ERKS_(arg0)};
    let rsthis = QOpenGLBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QOpenGLBuffer::size();
impl /*struct*/ QOpenGLBuffer {
  pub fn size<RetType, T: QOpenGLBuffer_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_size<RetType> {
  fn size(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  int QOpenGLBuffer::size();
impl<'a> /*trait*/ QOpenGLBuffer_size<i32> for () {
  fn size(self , rsthis: & QOpenGLBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLBuffer4sizeEv()};
    let mut ret = unsafe {_ZNK13QOpenGLBuffer4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::allocate(const void * data, int count);
impl /*struct*/ QOpenGLBuffer {
  pub fn allocate<RetType, T: QOpenGLBuffer_allocate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allocate(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_allocate<RetType> {
  fn allocate(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  void QOpenGLBuffer::allocate(const void * data, int count);
impl<'a> /*trait*/ QOpenGLBuffer_allocate<()> for (*mut c_void, i32) {
  fn allocate(self , rsthis: & QOpenGLBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer8allocateEPKvi()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QOpenGLBuffer8allocateEPKvi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLBuffer::bufferId();
impl /*struct*/ QOpenGLBuffer {
  pub fn bufferId<RetType, T: QOpenGLBuffer_bufferId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bufferId(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_bufferId<RetType> {
  fn bufferId(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  GLuint QOpenGLBuffer::bufferId();
impl<'a> /*trait*/ QOpenGLBuffer_bufferId<()> for () {
  fn bufferId(self , rsthis: & QOpenGLBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLBuffer8bufferIdEv()};
     unsafe {_ZNK13QOpenGLBuffer8bufferIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::QOpenGLBuffer();
impl<'a> /*trait*/ QOpenGLBuffer_New for () {
  fn New(self) -> QOpenGLBuffer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBufferC1Ev()};
    let ctysz: c_int = unsafe{QOpenGLBuffer_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN13QOpenGLBufferC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN13QOpenGLBufferC1Ev()};
    let rsthis = QOpenGLBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLBuffer::create();
impl /*struct*/ QOpenGLBuffer {
  pub fn create<RetType, T: QOpenGLBuffer_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_create<RetType> {
  fn create(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  bool QOpenGLBuffer::create();
impl<'a> /*trait*/ QOpenGLBuffer_create<i8> for () {
  fn create(self , rsthis: & QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer6createEv()};
    let mut ret = unsafe {_ZN13QOpenGLBuffer6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::~QOpenGLBuffer();
impl /*struct*/ QOpenGLBuffer {
  pub fn Free<RetType, T: QOpenGLBuffer_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  void QOpenGLBuffer::~QOpenGLBuffer();
impl<'a> /*trait*/ QOpenGLBuffer_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBufferD0Ev()};
     unsafe {_ZN13QOpenGLBufferD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::release();
impl /*struct*/ QOpenGLBuffer {
  pub fn release<RetType, T: QOpenGLBuffer_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_release<RetType> {
  fn release(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  void QOpenGLBuffer::release();
impl<'a> /*trait*/ QOpenGLBuffer_release<()> for () {
  fn release(self , rsthis: & QOpenGLBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer7releaseEv()};
     unsafe {_ZN13QOpenGLBuffer7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLBuffer::isCreated();
impl /*struct*/ QOpenGLBuffer {
  pub fn isCreated<RetType, T: QOpenGLBuffer_isCreated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_isCreated<RetType> {
  fn isCreated(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  bool QOpenGLBuffer::isCreated();
impl<'a> /*trait*/ QOpenGLBuffer_isCreated<i8> for () {
  fn isCreated(self , rsthis: & QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLBuffer9isCreatedEv()};
    let mut ret = unsafe {_ZNK13QOpenGLBuffer9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLBuffer::write(int offset, const void * data, int count);
impl /*struct*/ QOpenGLBuffer {
  pub fn write<RetType, T: QOpenGLBuffer_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_write<RetType> {
  fn write(self , rsthis: & QOpenGLBuffer) -> RetType;
}

  // proto:  void QOpenGLBuffer::write(int offset, const void * data, int count);
impl<'a> /*trait*/ QOpenGLBuffer_write<()> for (i32, *mut c_void, i32) {
  fn write(self , rsthis: & QOpenGLBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer5writeEiPKvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QOpenGLBuffer5writeEiPKvi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// <= body block end

