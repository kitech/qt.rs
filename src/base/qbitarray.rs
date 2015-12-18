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
  // proto:  void QBitArray::NewQBitArray(int size, bool val);
  fn _ZN9QBitArrayC1Eib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  bool QBitArray::isEmpty();
  fn _ZNK9QBitArray7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBitArray::setBit(int i);
  fn _ZN9QBitArray6setBitEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QBitArray::size();
  fn _ZNK9QBitArray4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QBitArray::swap(QBitArray & other);
  fn _ZN9QBitArray4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QBitArray::count();
  fn _ZNK9QBitArray5countEv(qthis: *mut c_void) -> c_int;
  // proto:  int QBitArray::count(bool on);
  fn _ZNK9QBitArray5countEb(qthis: *mut c_void, arg0: int8_t) -> c_int;
  // proto:  void QBitArray::detach();
  fn _ZN9QBitArray6detachEv(qthis: *mut c_void) ;
  // proto:  void QBitArray::NewQBitArray();
  fn _ZN9QBitArrayC1Ev(qthis: *mut c_void) ;
  // proto:  bool QBitArray::at(int i);
  fn _ZNK9QBitArray2atEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QBitArray::clear();
  fn _ZN9QBitArray5clearEv(qthis: *mut c_void) ;
  // proto:  void QBitArray::clearBit(int i);
  fn _ZN9QBitArray8clearBitEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QBitArray::testBit(int i);
  fn _ZNK9QBitArray7testBitEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QBitArray::truncate(int pos);
  fn _ZN9QBitArray8truncateEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QBitArray::toggleBit(int i);
  fn _ZN9QBitArray9toggleBitEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QBitArray::NewQBitArray(const QBitArray & other);
  fn _ZN9QBitArrayC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QBitArray::isNull();
  fn _ZNK9QBitArray6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBitArray::setBit(int i, bool val);
  fn _ZN9QBitArray6setBitEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QBitArray::resize(int size);
  fn _ZN9QBitArray6resizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QBitArray::isDetached();
  fn _ZNK9QBitArray10isDetachedEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QBitArray)=8
pub struct QBitArray {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBitArray {
  pub fn NewQBitArray<T: QBitArray_NewQBitArray>(value: T) -> QBitArray {
    let rsthis = value.NewQBitArray();
    return rsthis;
    // return 1;
  }
}

pub trait QBitArray_NewQBitArray {
  fn NewQBitArray(self) -> QBitArray;
}

// proto: void QBitArray::NewQBitArray(int size, bool val);
impl<'a> /*trait*/ QBitArray_NewQBitArray for (i32, i8) {
  fn NewQBitArray(self) -> QBitArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC1Eib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN9QBitArrayC1Eib(qthis, arg0, arg1)};
    let rsthis = QBitArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn isEmpty<RetType, T: QBitArray_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QBitArray_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  bool QBitArray::isEmpty();
impl<'a> /*trait*/ QBitArray_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QBitArray7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn setBit<RetType, T: QBitArray_setBit<RetType>>(&mut self, value: T) -> RetType {
    return value.setBit(self);
    // return 1;
  }
}

pub trait QBitArray_setBit<RetType> {
  fn setBit(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::setBit(int i);
impl<'a> /*trait*/ QBitArray_setBit<()> for (i32) {
  fn setBit(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6setBitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray6setBitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn size<RetType, T: QBitArray_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QBitArray_size<RetType> {
  fn size(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  int QBitArray::size();
impl<'a> /*trait*/ QBitArray_size<i32> for () {
  fn size(self, rsthis: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray4sizeEv()};
    let mut ret = unsafe {_ZNK9QBitArray4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn swap<RetType, T: QBitArray_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QBitArray_swap<RetType> {
  fn swap(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::swap(QBitArray & other);
impl<'a> /*trait*/ QBitArray_swap<()> for (&'a mut QBitArray) {
  fn swap(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QBitArray4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn count<RetType, T: QBitArray_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QBitArray_count<RetType> {
  fn count(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  int QBitArray::count();
impl<'a> /*trait*/ QBitArray_count<i32> for () {
  fn count(self, rsthis: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray5countEv()};
    let mut ret = unsafe {_ZNK9QBitArray5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QBitArray::count(bool on);
impl<'a> /*trait*/ QBitArray_count<i32> for (i8) {
  fn count(self, rsthis: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray5countEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZNK9QBitArray5countEb(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn detach<RetType, T: QBitArray_detach<RetType>>(&mut self, value: T) -> RetType {
    return value.detach(self);
    // return 1;
  }
}

pub trait QBitArray_detach<RetType> {
  fn detach(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::detach();
impl<'a> /*trait*/ QBitArray_detach<()> for () {
  fn detach(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6detachEv()};
     unsafe {_ZN9QBitArray6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QBitArray::NewQBitArray();
impl<'a> /*trait*/ QBitArray_NewQBitArray for () {
  fn NewQBitArray(self) -> QBitArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC1Ev()};
    unsafe {_ZN9QBitArrayC1Ev(qthis)};
    let rsthis = QBitArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn at<RetType, T: QBitArray_at<RetType>>(&mut self, value: T) -> RetType {
    return value.at(self);
    // return 1;
  }
}

pub trait QBitArray_at<RetType> {
  fn at(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  bool QBitArray::at(int i);
impl<'a> /*trait*/ QBitArray_at<i8> for (i32) {
  fn at(self, rsthis: &mut QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QBitArray2atEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn clear<RetType, T: QBitArray_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QBitArray_clear<RetType> {
  fn clear(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::clear();
impl<'a> /*trait*/ QBitArray_clear<()> for () {
  fn clear(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray5clearEv()};
     unsafe {_ZN9QBitArray5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn clearBit<RetType, T: QBitArray_clearBit<RetType>>(&mut self, value: T) -> RetType {
    return value.clearBit(self);
    // return 1;
  }
}

pub trait QBitArray_clearBit<RetType> {
  fn clearBit(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::clearBit(int i);
impl<'a> /*trait*/ QBitArray_clearBit<()> for (i32) {
  fn clearBit(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray8clearBitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray8clearBitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn testBit<RetType, T: QBitArray_testBit<RetType>>(&mut self, value: T) -> RetType {
    return value.testBit(self);
    // return 1;
  }
}

pub trait QBitArray_testBit<RetType> {
  fn testBit(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  bool QBitArray::testBit(int i);
impl<'a> /*trait*/ QBitArray_testBit<i8> for (i32) {
  fn testBit(self, rsthis: &mut QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray7testBitEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QBitArray7testBitEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn truncate<RetType, T: QBitArray_truncate<RetType>>(&mut self, value: T) -> RetType {
    return value.truncate(self);
    // return 1;
  }
}

pub trait QBitArray_truncate<RetType> {
  fn truncate(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::truncate(int pos);
impl<'a> /*trait*/ QBitArray_truncate<()> for (i32) {
  fn truncate(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray8truncateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray8truncateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn toggleBit<RetType, T: QBitArray_toggleBit<RetType>>(&mut self, value: T) -> RetType {
    return value.toggleBit(self);
    // return 1;
  }
}

pub trait QBitArray_toggleBit<RetType> {
  fn toggleBit(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  bool QBitArray::toggleBit(int i);
impl<'a> /*trait*/ QBitArray_toggleBit<i8> for (i32) {
  fn toggleBit(self, rsthis: &mut QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray9toggleBitEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QBitArray9toggleBitEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QBitArray::NewQBitArray(const QBitArray & other);
impl<'a> /*trait*/ QBitArray_NewQBitArray for (&'a  QBitArray) {
  fn NewQBitArray(self) -> QBitArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QBitArrayC1ERKS_(qthis, arg0)};
    let rsthis = QBitArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn isNull<RetType, T: QBitArray_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QBitArray_isNull<RetType> {
  fn isNull(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  bool QBitArray::isNull();
impl<'a> /*trait*/ QBitArray_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray6isNullEv()};
    let mut ret = unsafe {_ZNK9QBitArray6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QBitArray::setBit(int i, bool val);
impl<'a> /*trait*/ QBitArray_setBit<()> for (i32, i8) {
  fn setBit(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6setBitEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN9QBitArray6setBitEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn resize<RetType, T: QBitArray_resize<RetType>>(&mut self, value: T) -> RetType {
    return value.resize(self);
    // return 1;
  }
}

pub trait QBitArray_resize<RetType> {
  fn resize(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  void QBitArray::resize(int size);
impl<'a> /*trait*/ QBitArray_resize<()> for (i32) {
  fn resize(self, rsthis: &mut QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6resizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray6resizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn isDetached<RetType, T: QBitArray_isDetached<RetType>>(&mut self, value: T) -> RetType {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QBitArray_isDetached<RetType> {
  fn isDetached(self, rsthis: &mut QBitArray) -> RetType;
}

// proto:  bool QBitArray::isDetached();
impl<'a> /*trait*/ QBitArray_isDetached<i8> for () {
  fn isDetached(self, rsthis: &mut QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray10isDetachedEv()};
    let mut ret = unsafe {_ZNK9QBitArray10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

