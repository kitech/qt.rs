// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QOpenGLBuffer::read(int offset, void * data, int count);
  fn _ZN13QOpenGLBuffer4readEiPvi(qthis: *mut c_void, arg0: c_int, arg1: *mut uint8_t, arg2: c_int) -> int8_t;
  // proto:  bool QOpenGLBuffer::bind();
  fn _ZN13QOpenGLBuffer4bindEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLBuffer::destroy();
  fn _ZN13QOpenGLBuffer7destroyEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLBuffer::allocate(int count);
  fn _ZN13QOpenGLBuffer8allocateEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QOpenGLBuffer::unmap();
  fn _ZN13QOpenGLBuffer5unmapEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLBuffer::NewQOpenGLBuffer(const QOpenGLBuffer & other);
  fn _ZN13QOpenGLBufferC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QOpenGLBuffer::size();
  fn _ZNK13QOpenGLBuffer4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLBuffer::allocate(const void * data, int count);
  fn _ZN13QOpenGLBuffer8allocateEPKvi(qthis: *mut c_void, arg0: *const uint8_t, arg1: c_int) ;
  // proto:  QOpenGLBuffer::GLuint QOpenGLBuffer::bufferId();
  fn _ZNK13QOpenGLBuffer8bufferIdEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLBuffer::NewQOpenGLBuffer();
  fn _ZN13QOpenGLBufferC1Ev(qthis: *mut c_void) ;
  // proto:  bool QOpenGLBuffer::create();
  fn _ZN13QOpenGLBuffer6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLBuffer::FreeQOpenGLBuffer();
  fn _ZN13QOpenGLBufferD0Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLBuffer::release();
  fn _ZN13QOpenGLBuffer7releaseEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLBuffer::isCreated();
  fn _ZNK13QOpenGLBuffer9isCreatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLBuffer::write(int offset, const void * data, int count);
  fn _ZN13QOpenGLBuffer5writeEiPKvi(qthis: *mut c_void, arg0: c_int, arg1: *const uint8_t, arg2: c_int) ;
}

// body block begin
// class sizeof(QOpenGLBuffer)=8
pub struct QOpenGLBuffer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLBuffer {
  pub fn read<T: QOpenGLBuffer_read>(&mut self, value: T) -> i8 {
    return value.read(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_read {
  fn read(self, rsthis: &mut QOpenGLBuffer) -> i8;
}

// proto:  bool QOpenGLBuffer::read(int offset, void * data, int count);
impl<'a> /*trait*/ QOpenGLBuffer_read for (i32, &'a mut u8, i32) {
  fn read(self, rsthis: &mut QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer4readEiPvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN13QOpenGLBuffer4readEiPvi(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn bind<T: QOpenGLBuffer_bind>(&mut self, value: T) -> i8 {
    return value.bind(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_bind {
  fn bind(self, rsthis: &mut QOpenGLBuffer) -> i8;
}

// proto:  bool QOpenGLBuffer::bind();
impl<'a> /*trait*/ QOpenGLBuffer_bind for () {
  fn bind(self, rsthis: &mut QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer4bindEv()};
    let mut ret = unsafe {_ZN13QOpenGLBuffer4bindEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn destroy<T: QOpenGLBuffer_destroy>(&mut self, value: T)  {
     value.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_destroy {
  fn destroy(self, rsthis: &mut QOpenGLBuffer) ;
}

// proto:  void QOpenGLBuffer::destroy();
impl<'a> /*trait*/ QOpenGLBuffer_destroy for () {
  fn destroy(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer7destroyEv()};
     unsafe {_ZN13QOpenGLBuffer7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn allocate<T: QOpenGLBuffer_allocate>(&mut self, value: T)  {
     value.allocate(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_allocate {
  fn allocate(self, rsthis: &mut QOpenGLBuffer) ;
}

// proto:  void QOpenGLBuffer::allocate(int count);
impl<'a> /*trait*/ QOpenGLBuffer_allocate for (i32) {
  fn allocate(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer8allocateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QOpenGLBuffer8allocateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn unmap<T: QOpenGLBuffer_unmap>(&mut self, value: T) -> i8 {
    return value.unmap(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_unmap {
  fn unmap(self, rsthis: &mut QOpenGLBuffer) -> i8;
}

// proto:  bool QOpenGLBuffer::unmap();
impl<'a> /*trait*/ QOpenGLBuffer_unmap for () {
  fn unmap(self, rsthis: &mut QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer5unmapEv()};
    let mut ret = unsafe {_ZN13QOpenGLBuffer5unmapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn NewQOpenGLBuffer<T: QOpenGLBuffer_NewQOpenGLBuffer>(value: T) -> QOpenGLBuffer {
    let rsthis = value.NewQOpenGLBuffer();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLBuffer_NewQOpenGLBuffer {
  fn NewQOpenGLBuffer(self) -> QOpenGLBuffer;
}

// proto: void QOpenGLBuffer::NewQOpenGLBuffer(const QOpenGLBuffer & other);
impl<'a> /*trait*/ QOpenGLBuffer_NewQOpenGLBuffer for (&'a  QOpenGLBuffer) {
  fn NewQOpenGLBuffer(self) -> QOpenGLBuffer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBufferC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QOpenGLBufferC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn size<T: QOpenGLBuffer_size>(&mut self, value: T) -> i32 {
    return value.size(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_size {
  fn size(self, rsthis: &mut QOpenGLBuffer) -> i32;
}

// proto:  int QOpenGLBuffer::size();
impl<'a> /*trait*/ QOpenGLBuffer_size for () {
  fn size(self, rsthis: &mut QOpenGLBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLBuffer4sizeEv()};
    let mut ret = unsafe {_ZNK13QOpenGLBuffer4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLBuffer::allocate(const void * data, int count);
impl<'a> /*trait*/ QOpenGLBuffer_allocate for (&'a  u8, i32) {
  fn allocate(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer8allocateEPKvi()};
    let arg0 = self.0  as *const uint8_t;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QOpenGLBuffer8allocateEPKvi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn bufferId<T: QOpenGLBuffer_bufferId>(&mut self, value: T)  {
     value.bufferId(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_bufferId {
  fn bufferId(self, rsthis: &mut QOpenGLBuffer) ;
}

// proto:  QOpenGLBuffer::GLuint QOpenGLBuffer::bufferId();
impl<'a> /*trait*/ QOpenGLBuffer_bufferId for () {
  fn bufferId(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLBuffer8bufferIdEv()};
     unsafe {_ZNK13QOpenGLBuffer8bufferIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLBuffer::NewQOpenGLBuffer();
impl<'a> /*trait*/ QOpenGLBuffer_NewQOpenGLBuffer for () {
  fn NewQOpenGLBuffer(self) -> QOpenGLBuffer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBufferC1Ev()};
    unsafe {_ZN13QOpenGLBufferC1Ev(qthis)};
    let rsthis = QOpenGLBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn create<T: QOpenGLBuffer_create>(&mut self, value: T) -> i8 {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_create {
  fn create(self, rsthis: &mut QOpenGLBuffer) -> i8;
}

// proto:  bool QOpenGLBuffer::create();
impl<'a> /*trait*/ QOpenGLBuffer_create for () {
  fn create(self, rsthis: &mut QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer6createEv()};
    let mut ret = unsafe {_ZN13QOpenGLBuffer6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn FreeQOpenGLBuffer<T: QOpenGLBuffer_FreeQOpenGLBuffer>(&mut self, value: T)  {
     value.FreeQOpenGLBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_FreeQOpenGLBuffer {
  fn FreeQOpenGLBuffer(self, rsthis: &mut QOpenGLBuffer) ;
}

// proto:  void QOpenGLBuffer::FreeQOpenGLBuffer();
impl<'a> /*trait*/ QOpenGLBuffer_FreeQOpenGLBuffer for () {
  fn FreeQOpenGLBuffer(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBufferD0Ev()};
     unsafe {_ZN13QOpenGLBufferD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn release<T: QOpenGLBuffer_release>(&mut self, value: T)  {
     value.release(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_release {
  fn release(self, rsthis: &mut QOpenGLBuffer) ;
}

// proto:  void QOpenGLBuffer::release();
impl<'a> /*trait*/ QOpenGLBuffer_release for () {
  fn release(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer7releaseEv()};
     unsafe {_ZN13QOpenGLBuffer7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn isCreated<T: QOpenGLBuffer_isCreated>(&mut self, value: T) -> i8 {
    return value.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_isCreated {
  fn isCreated(self, rsthis: &mut QOpenGLBuffer) -> i8;
}

// proto:  bool QOpenGLBuffer::isCreated();
impl<'a> /*trait*/ QOpenGLBuffer_isCreated for () {
  fn isCreated(self, rsthis: &mut QOpenGLBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLBuffer9isCreatedEv()};
    let mut ret = unsafe {_ZNK13QOpenGLBuffer9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLBuffer {
  pub fn write<T: QOpenGLBuffer_write>(&mut self, value: T)  {
     value.write(self);
    // return 1;
  }
}

pub trait QOpenGLBuffer_write {
  fn write(self, rsthis: &mut QOpenGLBuffer) ;
}

// proto:  void QOpenGLBuffer::write(int offset, const void * data, int count);
impl<'a> /*trait*/ QOpenGLBuffer_write for (i32, &'a  u8, i32) {
  fn write(self, rsthis: &mut QOpenGLBuffer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLBuffer5writeEiPKvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const uint8_t;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QOpenGLBuffer5writeEiPKvi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

