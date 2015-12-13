// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN7QBuffer4seekEx(arg0: c_longlong) -> i32;
  fn _ZNK7QBuffer11canReadLineEv() -> i32;
  fn _ZN7QBufferD0Ev() -> i32;
  fn _ZN7QBuffer4openE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: c_int) -> i32;
  fn _ZN7QBuffer7setDataERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK7QBuffer4dataEv() -> i32;
  fn _ZN7QBufferC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN7QBuffer9setBufferEP10QByteArray(arg0: *mut c_void) -> i32;
  fn _ZN7QBuffer6bufferEv() -> i32;
  fn _ZNK7QBuffer3posEv() -> i32;
  fn _ZN7QBufferC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN7QBuffer5closeEv() -> i32;
  fn _ZNK7QBuffer10metaObjectEv() -> i32;
  fn _ZNK7QBuffer4sizeEv() -> i32;
  fn _ZN7QBufferC1EP10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZNK7QBuffer5atEndEv() -> i32;
  fn _ZN7QBuffer7setDataEPKci(arg0: *const c_char, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QBuffer)=1
pub struct QBuffer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBuffer {
  pub fn seek<T: QBuffer_seek>(&mut self, value: T) -> i32 {
    value.seek(self);
    return 1;
  }
}

pub trait QBuffer_seek {
  fn seek(self, this: &mut QBuffer) -> i32;
}

// proto: bool QBuffer::seek(qint64 off);
impl<'a> /*trait*/ QBuffer_seek for (i64) {
  fn seek(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer4seekEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN7QBuffer4seekEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn canReadLine<T: QBuffer_canReadLine>(&mut self, value: T) -> i32 {
    value.canReadLine(self);
    return 1;
  }
}

pub trait QBuffer_canReadLine {
  fn canReadLine(self, this: &mut QBuffer) -> i32;
}

// proto: bool QBuffer::canReadLine();
impl<'a> /*trait*/ QBuffer_canReadLine for () {
  fn canReadLine(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer11canReadLineEv()};
    unsafe {_ZNK7QBuffer11canReadLineEv()};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn FreeQBuffer<T: QBuffer_FreeQBuffer>(&mut self, value: T) -> i32 {
    value.FreeQBuffer(self);
    return 1;
  }
}

pub trait QBuffer_FreeQBuffer {
  fn FreeQBuffer(self, this: &mut QBuffer) -> i32;
}

// proto: void QBuffer::FreeQBuffer();
impl<'a> /*trait*/ QBuffer_FreeQBuffer for () {
  fn FreeQBuffer(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBufferD0Ev()};
    unsafe {_ZN7QBufferD0Ev()};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn open<T: QBuffer_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QBuffer_open {
  fn open(self, this: &mut QBuffer) -> i32;
}

// proto: bool QBuffer::open(OpenMode openMode);
impl<'a> /*trait*/ QBuffer_open for (i32) {
  fn open(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer4openE6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QBuffer4openE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn setData<T: QBuffer_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QBuffer_setData {
  fn setData(self, this: &mut QBuffer) -> i32;
}

// proto: void QBuffer::setData(const QByteArray & data);
impl<'a> /*trait*/ QBuffer_setData for (&'a  QByteArray) {
  fn setData(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer7setDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QBuffer7setDataERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn data<T: QBuffer_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QBuffer_data {
  fn data(self, this: &mut QBuffer) -> i32;
}

// proto: const QByteArray & QBuffer::data();
impl<'a> /*trait*/ QBuffer_data for () {
  fn data(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer4dataEv()};
    unsafe {_ZNK7QBuffer4dataEv()};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn NewQBuffer<T: QBuffer_NewQBuffer>(value: T) -> QBuffer {
    let rsthis = value.NewQBuffer();
    return rsthis;
    // return 1;
  }
}

pub trait QBuffer_NewQBuffer {
  fn NewQBuffer(self) -> QBuffer;
}

// proto: void QBuffer::NewQBuffer(QObject * parent);
impl<'a> /*trait*/ QBuffer_NewQBuffer for (&'a mut QObject) {
  fn NewQBuffer(self) -> QBuffer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBufferC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QBufferC1EP7QObject(qthis, arg0)};
    let rsthis = QBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn setBuffer<T: QBuffer_setBuffer>(&mut self, value: T) -> i32 {
    value.setBuffer(self);
    return 1;
  }
}

pub trait QBuffer_setBuffer {
  fn setBuffer(self, this: &mut QBuffer) -> i32;
}

// proto: void QBuffer::setBuffer(QByteArray * a);
impl<'a> /*trait*/ QBuffer_setBuffer for (&'a mut QByteArray) {
  fn setBuffer(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer9setBufferEP10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QBuffer9setBufferEP10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn buffer<T: QBuffer_buffer>(&mut self, value: T) -> i32 {
    value.buffer(self);
    return 1;
  }
}

pub trait QBuffer_buffer {
  fn buffer(self, this: &mut QBuffer) -> i32;
}

// proto: QByteArray & QBuffer::buffer();
impl<'a> /*trait*/ QBuffer_buffer for () {
  fn buffer(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer6bufferEv()};
    unsafe {_ZN7QBuffer6bufferEv()};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn pos<T: QBuffer_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QBuffer_pos {
  fn pos(self, this: &mut QBuffer) -> i32;
}

// proto: long long QBuffer::pos();
impl<'a> /*trait*/ QBuffer_pos for () {
  fn pos(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer3posEv()};
    unsafe {_ZNK7QBuffer3posEv()};
    return 1;
  }
}

// proto: void QBuffer::NewQBuffer(const QBuffer & );
impl<'a> /*trait*/ QBuffer_NewQBuffer for (&'a  QBuffer) {
  fn NewQBuffer(self) -> QBuffer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBufferC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QBufferC1ERKS_(qthis, arg0)};
    let rsthis = QBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn close<T: QBuffer_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QBuffer_close {
  fn close(self, this: &mut QBuffer) -> i32;
}

// proto: void QBuffer::close();
impl<'a> /*trait*/ QBuffer_close for () {
  fn close(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer5closeEv()};
    unsafe {_ZN7QBuffer5closeEv()};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn metaObject<T: QBuffer_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QBuffer_metaObject {
  fn metaObject(self, this: &mut QBuffer) -> i32;
}

// proto: const QMetaObject * QBuffer::metaObject();
impl<'a> /*trait*/ QBuffer_metaObject for () {
  fn metaObject(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer10metaObjectEv()};
    unsafe {_ZNK7QBuffer10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn size<T: QBuffer_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QBuffer_size {
  fn size(self, this: &mut QBuffer) -> i32;
}

// proto: long long QBuffer::size();
impl<'a> /*trait*/ QBuffer_size for () {
  fn size(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer4sizeEv()};
    unsafe {_ZNK7QBuffer4sizeEv()};
    return 1;
  }
}

// proto: void QBuffer::NewQBuffer(QByteArray * buf, QObject * parent);
impl<'a> /*trait*/ QBuffer_NewQBuffer for (&'a mut QByteArray, &'a mut QObject) {
  fn NewQBuffer(self) -> QBuffer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBufferC1EP10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QBufferC1EP10QByteArrayP7QObject(qthis, arg0, arg1)};
    let rsthis = QBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn atEnd<T: QBuffer_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QBuffer_atEnd {
  fn atEnd(self, this: &mut QBuffer) -> i32;
}

// proto: bool QBuffer::atEnd();
impl<'a> /*trait*/ QBuffer_atEnd for () {
  fn atEnd(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer5atEndEv()};
    unsafe {_ZNK7QBuffer5atEndEv()};
    return 1;
  }
}

// proto: void QBuffer::setData(const char * data, int len);
impl<'a> /*trait*/ QBuffer_setData for (&'a  String, i32) {
  fn setData(self, this: &mut QBuffer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer7setDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QBuffer7setDataEPKci(arg0, arg1)};
    return 1;
  }
}

