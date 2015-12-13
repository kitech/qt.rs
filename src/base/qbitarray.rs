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
  fn _ZN9QBitArrayC1Eib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) -> i32;
  fn _ZNK9QBitArray7isEmptyEv() -> i32;
  fn _ZN9QBitArray6setBitEi(arg0: c_int) -> i32;
  fn _ZNK9QBitArray4sizeEv() -> i32;
  fn _ZN9QBitArray4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK9QBitArray5countEv() -> i32;
  fn _ZNK9QBitArray5countEb(arg0: int8_t) -> i32;
  fn _ZN9QBitArray6detachEv() -> i32;
  fn _ZN9QBitArrayC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK9QBitArray2atEi(arg0: c_int) -> i32;
  fn _ZN9QBitArray5clearEv() -> i32;
  fn _ZN9QBitArray8clearBitEi(arg0: c_int) -> i32;
  fn _ZNK9QBitArray7testBitEi(arg0: c_int) -> i32;
  fn _ZN9QBitArray8truncateEi(arg0: c_int) -> i32;
  fn _ZN9QBitArray9toggleBitEi(arg0: c_int) -> i32;
  fn _ZN9QBitArrayC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QBitArray4fillEbii(arg0: int8_t, arg1: c_int, arg2: c_int) -> i32;
  fn _ZNK9QBitArray6isNullEv() -> i32;
  fn _ZN9QBitArray6setBitEib(arg0: c_int, arg1: int8_t) -> i32;
  fn _ZN9QBitArray6resizeEi(arg0: c_int) -> i32;
  fn _ZNK9QBitArray10isDetachedEv() -> i32;
  fn _ZN9QBitArray4fillEbi(arg0: int8_t, arg1: c_int) -> i32;
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
  pub fn isEmpty<T: QBitArray_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QBitArray_isEmpty {
  fn isEmpty(self, this: &mut QBitArray) -> i32;
}

// proto: bool QBitArray::isEmpty();
impl<'a> /*trait*/ QBitArray_isEmpty for () {
  fn isEmpty(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray7isEmptyEv()};
    unsafe {_ZNK9QBitArray7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn setBit<T: QBitArray_setBit>(&mut self, value: T) -> i32 {
    value.setBit(self);
    return 1;
  }
}

pub trait QBitArray_setBit {
  fn setBit(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::setBit(int i);
impl<'a> /*trait*/ QBitArray_setBit for (i32) {
  fn setBit(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6setBitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QBitArray6setBitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn size<T: QBitArray_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QBitArray_size {
  fn size(self, this: &mut QBitArray) -> i32;
}

// proto: int QBitArray::size();
impl<'a> /*trait*/ QBitArray_size for () {
  fn size(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray4sizeEv()};
    unsafe {_ZNK9QBitArray4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn swap<T: QBitArray_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QBitArray_swap {
  fn swap(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::swap(QBitArray & other);
impl<'a> /*trait*/ QBitArray_swap for (&'a mut QBitArray) {
  fn swap(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QBitArray4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn count<T: QBitArray_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QBitArray_count {
  fn count(self, this: &mut QBitArray) -> i32;
}

// proto: int QBitArray::count();
impl<'a> /*trait*/ QBitArray_count for () {
  fn count(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray5countEv()};
    unsafe {_ZNK9QBitArray5countEv()};
    return 1;
  }
}

// proto: int QBitArray::count(bool on);
impl<'a> /*trait*/ QBitArray_count for (i8) {
  fn count(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray5countEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZNK9QBitArray5countEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn detach<T: QBitArray_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QBitArray_detach {
  fn detach(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::detach();
impl<'a> /*trait*/ QBitArray_detach for () {
  fn detach(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6detachEv()};
    unsafe {_ZN9QBitArray6detachEv()};
    return 1;
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
  pub fn at<T: QBitArray_at>(&mut self, value: T) -> i32 {
    value.at(self);
    return 1;
  }
}

pub trait QBitArray_at {
  fn at(self, this: &mut QBitArray) -> i32;
}

// proto: bool QBitArray::at(int i);
impl<'a> /*trait*/ QBitArray_at for (i32) {
  fn at(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray2atEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QBitArray2atEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn clear<T: QBitArray_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QBitArray_clear {
  fn clear(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::clear();
impl<'a> /*trait*/ QBitArray_clear for () {
  fn clear(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray5clearEv()};
    unsafe {_ZN9QBitArray5clearEv()};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn clearBit<T: QBitArray_clearBit>(&mut self, value: T) -> i32 {
    value.clearBit(self);
    return 1;
  }
}

pub trait QBitArray_clearBit {
  fn clearBit(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::clearBit(int i);
impl<'a> /*trait*/ QBitArray_clearBit for (i32) {
  fn clearBit(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray8clearBitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QBitArray8clearBitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn testBit<T: QBitArray_testBit>(&mut self, value: T) -> i32 {
    value.testBit(self);
    return 1;
  }
}

pub trait QBitArray_testBit {
  fn testBit(self, this: &mut QBitArray) -> i32;
}

// proto: bool QBitArray::testBit(int i);
impl<'a> /*trait*/ QBitArray_testBit for (i32) {
  fn testBit(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray7testBitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QBitArray7testBitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn truncate<T: QBitArray_truncate>(&mut self, value: T) -> i32 {
    value.truncate(self);
    return 1;
  }
}

pub trait QBitArray_truncate {
  fn truncate(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::truncate(int pos);
impl<'a> /*trait*/ QBitArray_truncate for (i32) {
  fn truncate(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray8truncateEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QBitArray8truncateEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn toggleBit<T: QBitArray_toggleBit>(&mut self, value: T) -> i32 {
    value.toggleBit(self);
    return 1;
  }
}

pub trait QBitArray_toggleBit {
  fn toggleBit(self, this: &mut QBitArray) -> i32;
}

// proto: bool QBitArray::toggleBit(int i);
impl<'a> /*trait*/ QBitArray_toggleBit for (i32) {
  fn toggleBit(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray9toggleBitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QBitArray9toggleBitEi(arg0)};
    return 1;
  }
}

// proto: void QBitArray::NewQBitArray(const QBitArray & other);
impl<'a> /*trait*/ QBitArray_NewQBitArray for (&'a  QBitArray) {
  fn NewQBitArray(self) -> QBitArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QBitArrayC1ERKS_(qthis, arg0)};
    let rsthis = QBitArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn fill<T: QBitArray_fill>(&mut self, value: T) -> i32 {
    value.fill(self);
    return 1;
  }
}

pub trait QBitArray_fill {
  fn fill(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::fill(bool val, int first, int last);
impl<'a> /*trait*/ QBitArray_fill for (i8, i32, i32) {
  fn fill(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4fillEbii()};
    let arg0 = self.0  as int8_t;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN9QBitArray4fillEbii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn isNull<T: QBitArray_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QBitArray_isNull {
  fn isNull(self, this: &mut QBitArray) -> i32;
}

// proto: bool QBitArray::isNull();
impl<'a> /*trait*/ QBitArray_isNull for () {
  fn isNull(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray6isNullEv()};
    unsafe {_ZNK9QBitArray6isNullEv()};
    return 1;
  }
}

// proto: void QBitArray::setBit(int i, bool val);
impl<'a> /*trait*/ QBitArray_setBit for (i32, i8) {
  fn setBit(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6setBitEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN9QBitArray6setBitEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn resize<T: QBitArray_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QBitArray_resize {
  fn resize(self, this: &mut QBitArray) -> i32;
}

// proto: void QBitArray::resize(int size);
impl<'a> /*trait*/ QBitArray_resize for (i32) {
  fn resize(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6resizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QBitArray6resizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn isDetached<T: QBitArray_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QBitArray_isDetached {
  fn isDetached(self, this: &mut QBitArray) -> i32;
}

// proto: bool QBitArray::isDetached();
impl<'a> /*trait*/ QBitArray_isDetached for () {
  fn isDetached(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray10isDetachedEv()};
    unsafe {_ZNK9QBitArray10isDetachedEv()};
    return 1;
  }
}

// proto: bool QBitArray::fill(bool val, int size);
impl<'a> /*trait*/ QBitArray_fill for (i8, i32) {
  fn fill(self, this: &mut QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4fillEbi()};
    let arg0 = self.0  as int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QBitArray4fillEbi(arg0, arg1)};
    return 1;
  }
}

