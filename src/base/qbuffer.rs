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
  // proto:  bool QBuffer::seek(qint64 off);
  fn _ZN7QBuffer4seekEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
  // proto:  bool QBuffer::canReadLine();
  fn _ZNK7QBuffer11canReadLineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBuffer::FreeQBuffer();
  fn _ZN7QBufferD0Ev(qthis: *mut c_void) ;
  // proto:  void QBuffer::setData(const QByteArray & data);
  fn _ZN7QBuffer7setDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QByteArray & QBuffer::data();
  fn _ZNK7QBuffer4dataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBuffer::NewQBuffer(QObject * parent);
  fn _ZN7QBufferC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBuffer::setBuffer(QByteArray * a);
  fn _ZN7QBuffer9setBufferEP10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray & QBuffer::buffer();
  fn _ZN7QBuffer6bufferEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QBuffer::pos();
  fn _ZNK7QBuffer3posEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QBuffer::NewQBuffer(const QBuffer & );
  fn _ZN7QBufferC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBuffer::close();
  fn _ZN7QBuffer5closeEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QBuffer::metaObject();
  fn _ZNK7QBuffer10metaObjectEv(qthis: *mut c_void) ;
  // proto:  long long QBuffer::size();
  fn _ZNK7QBuffer4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QBuffer::NewQBuffer(QByteArray * buf, QObject * parent);
  fn _ZN7QBufferC1EP10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QBuffer::atEnd();
  fn _ZNK7QBuffer5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBuffer::setData(const char * data, int len);
  fn _ZN7QBuffer7setDataEPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) ;
}

// body block begin
// class sizeof(QBuffer)=1
pub struct QBuffer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBuffer {
  pub fn seek<RetType, T: QBuffer_seek<RetType>>(&mut self, value: T) -> RetType {
    return value.seek(self);
    // return 1;
  }
}

pub trait QBuffer_seek<RetType> {
  fn seek(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  bool QBuffer::seek(qint64 off);
impl<'a> /*trait*/ QBuffer_seek<i8> for (i64) {
  fn seek(self, rsthis: &mut QBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN7QBuffer4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn canReadLine<RetType, T: QBuffer_canReadLine<RetType>>(&mut self, value: T) -> RetType {
    return value.canReadLine(self);
    // return 1;
  }
}

pub trait QBuffer_canReadLine<RetType> {
  fn canReadLine(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  bool QBuffer::canReadLine();
impl<'a> /*trait*/ QBuffer_canReadLine<i8> for () {
  fn canReadLine(self, rsthis: &mut QBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer11canReadLineEv()};
    let mut ret = unsafe {_ZNK7QBuffer11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn FreeQBuffer<RetType, T: QBuffer_FreeQBuffer<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQBuffer(self);
    // return 1;
  }
}

pub trait QBuffer_FreeQBuffer<RetType> {
  fn FreeQBuffer(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  void QBuffer::FreeQBuffer();
impl<'a> /*trait*/ QBuffer_FreeQBuffer<()> for () {
  fn FreeQBuffer(self, rsthis: &mut QBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBufferD0Ev()};
     unsafe {_ZN7QBufferD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn setData<RetType, T: QBuffer_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QBuffer_setData<RetType> {
  fn setData(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  void QBuffer::setData(const QByteArray & data);
impl<'a> /*trait*/ QBuffer_setData<()> for (&'a  QByteArray) {
  fn setData(self, rsthis: &mut QBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer7setDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QBuffer7setDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn data<RetType, T: QBuffer_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QBuffer_data<RetType> {
  fn data(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  const QByteArray & QBuffer::data();
impl<'a> /*trait*/ QBuffer_data<QByteArray> for () {
  fn data(self, rsthis: &mut QBuffer) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer4dataEv()};
    let mut ret = unsafe {_ZNK7QBuffer4dataEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setBuffer<RetType, T: QBuffer_setBuffer<RetType>>(&mut self, value: T) -> RetType {
    return value.setBuffer(self);
    // return 1;
  }
}

pub trait QBuffer_setBuffer<RetType> {
  fn setBuffer(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  void QBuffer::setBuffer(QByteArray * a);
impl<'a> /*trait*/ QBuffer_setBuffer<()> for (&'a mut QByteArray) {
  fn setBuffer(self, rsthis: &mut QBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer9setBufferEP10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QBuffer9setBufferEP10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn buffer<RetType, T: QBuffer_buffer<RetType>>(&mut self, value: T) -> RetType {
    return value.buffer(self);
    // return 1;
  }
}

pub trait QBuffer_buffer<RetType> {
  fn buffer(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  QByteArray & QBuffer::buffer();
impl<'a> /*trait*/ QBuffer_buffer<QByteArray> for () {
  fn buffer(self, rsthis: &mut QBuffer) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer6bufferEv()};
    let mut ret = unsafe {_ZN7QBuffer6bufferEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn pos<RetType, T: QBuffer_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QBuffer_pos<RetType> {
  fn pos(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  long long QBuffer::pos();
impl<'a> /*trait*/ QBuffer_pos<i64> for () {
  fn pos(self, rsthis: &mut QBuffer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer3posEv()};
    let mut ret = unsafe {_ZNK7QBuffer3posEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto: void QBuffer::NewQBuffer(const QBuffer & );
impl<'a> /*trait*/ QBuffer_NewQBuffer for (&'a  QBuffer) {
  fn NewQBuffer(self) -> QBuffer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBufferC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QBufferC1ERKS_(qthis, arg0)};
    let rsthis = QBuffer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn close<RetType, T: QBuffer_close<RetType>>(&mut self, value: T) -> RetType {
    return value.close(self);
    // return 1;
  }
}

pub trait QBuffer_close<RetType> {
  fn close(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  void QBuffer::close();
impl<'a> /*trait*/ QBuffer_close<()> for () {
  fn close(self, rsthis: &mut QBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer5closeEv()};
     unsafe {_ZN7QBuffer5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn metaObject<RetType, T: QBuffer_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QBuffer_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  const QMetaObject * QBuffer::metaObject();
impl<'a> /*trait*/ QBuffer_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer10metaObjectEv()};
     unsafe {_ZNK7QBuffer10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBuffer {
  pub fn size<RetType, T: QBuffer_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QBuffer_size<RetType> {
  fn size(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  long long QBuffer::size();
impl<'a> /*trait*/ QBuffer_size<i64> for () {
  fn size(self, rsthis: &mut QBuffer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer4sizeEv()};
    let mut ret = unsafe {_ZNK7QBuffer4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
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
  pub fn atEnd<RetType, T: QBuffer_atEnd<RetType>>(&mut self, value: T) -> RetType {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QBuffer_atEnd<RetType> {
  fn atEnd(self, rsthis: &mut QBuffer) -> RetType;
}

// proto:  bool QBuffer::atEnd();
impl<'a> /*trait*/ QBuffer_atEnd<i8> for () {
  fn atEnd(self, rsthis: &mut QBuffer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QBuffer5atEndEv()};
    let mut ret = unsafe {_ZNK7QBuffer5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QBuffer::setData(const char * data, int len);
impl<'a> /*trait*/ QBuffer_setData<()> for (&'a  String, i32) {
  fn setData(self, rsthis: &mut QBuffer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBuffer7setDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QBuffer7setDataEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

