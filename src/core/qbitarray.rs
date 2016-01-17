// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qbitarray.h
// dst-file: /src/core/qbitarray.rs
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
// use super::qbitarray::QBitArray; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QBitRef_Class_Size() -> c_int;
  // proto:  void QBitRef::QBitRef(QBitArray & array, int idx);
  fn _ZN7QBitRefC2ER9QBitArrayi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  fn QBitArray_Class_Size() -> c_int;
  // proto:  void QBitArray::QBitArray(int size, bool val);
  fn _ZN9QBitArrayC2Eib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  bool QBitArray::isEmpty();
  fn _ZNK9QBitArray7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QBitArray::setBit(int i);
  fn _ZN9QBitArray6setBitEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QBitArray::size();
  fn _ZNK9QBitArray4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QBitArray::swap(QBitArray & other);
  fn _ZN9QBitArray4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QBitArray::count();
  fn _ZNK9QBitArray5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QBitArray::count(bool on);
  fn _ZNK9QBitArray5countEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_int;
  // proto:  void QBitArray::detach();
  fn _ZN9QBitArray6detachEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QBitArray::QBitArray();
  fn _ZN9QBitArrayC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QBitArray::at(int i);
  fn _ZNK9QBitArray2atEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QBitArray::clear();
  fn _ZN9QBitArray5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QBitArray::clearBit(int i);
  fn _ZN9QBitArray8clearBitEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QBitArray::testBit(int i);
  fn _ZNK9QBitArray7testBitEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QBitArray::truncate(int pos);
  fn _ZN9QBitArray8truncateEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QBitArray::toggleBit(int i);
  fn _ZN9QBitArray9toggleBitEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QBitArray::QBitArray(const QBitArray & other);
  fn _ZN9QBitArrayC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBitArray::fill(bool val, int first, int last);
  fn _ZN9QBitArray4fillEbii(qthis: u64 /* *mut c_void*/, arg0: c_char, arg1: c_int, arg2: c_int);
  // proto:  bool QBitArray::isNull();
  fn _ZNK9QBitArray6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QBitArray::setBit(int i, bool val);
  fn _ZN9QBitArray6setBitEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QBitArray::resize(int size);
  fn _ZN9QBitArray6resizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QBitArray::isDetached();
  fn _ZNK9QBitArray10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QBitArray::fill(bool val, int size);
  fn _ZN9QBitArray4fillEbi(qthis: u64 /* *mut c_void*/, arg0: c_char, arg1: c_int) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QBitRef)=16
#[derive(Default)]
pub struct QBitRef {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QBitArray)=8
#[derive(Default)]
pub struct QBitArray {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QBitRef {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBitRef {
    return QBitRef{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QBitRef::QBitRef(QBitArray & array, int idx);
impl /*struct*/ QBitRef {
  pub fn new<T: QBitRef_new>(value: T) -> QBitRef {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QBitRef_new {
  fn new(self) -> QBitRef;
}

  // proto:  void QBitRef::QBitRef(QBitArray & array, int idx);
impl<'a> /*trait*/ QBitRef_new for (&'a QBitArray, i32) {
  fn new(self) -> QBitRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QBitRefC2ER9QBitArrayi()};
    let ctysz: c_int = unsafe{QBitRef_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QBitRefC2ER9QBitArrayi(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QBitRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBitArray {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBitArray {
    return QBitArray{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QBitArray::QBitArray(int size, bool val);
impl /*struct*/ QBitArray {
  pub fn new<T: QBitArray_new>(value: T) -> QBitArray {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QBitArray_new {
  fn new(self) -> QBitArray;
}

  // proto:  void QBitArray::QBitArray(int size, bool val);
impl<'a> /*trait*/ QBitArray_new for (i32, i8) {
  fn new(self) -> QBitArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC2Eib()};
    let ctysz: c_int = unsafe{QBitArray_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    unsafe {_ZN9QBitArrayC2Eib(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QBitArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QBitArray::isEmpty();
impl /*struct*/ QBitArray {
  pub fn isEmpty<RetType, T: QBitArray_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QBitArray_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  bool QBitArray::isEmpty();
impl<'a> /*trait*/ QBitArray_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QBitArray7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBitArray::setBit(int i);
impl /*struct*/ QBitArray {
  pub fn setBit<RetType, T: QBitArray_setBit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBit(self);
    // return 1;
  }
}

pub trait QBitArray_setBit<RetType> {
  fn setBit(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::setBit(int i);
impl<'a> /*trait*/ QBitArray_setBit<()> for (i32) {
  fn setBit(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6setBitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray6setBitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QBitArray::size();
impl /*struct*/ QBitArray {
  pub fn size<RetType, T: QBitArray_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QBitArray_size<RetType> {
  fn size(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  int QBitArray::size();
impl<'a> /*trait*/ QBitArray_size<i32> for () {
  fn size(self , rsthis: & QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray4sizeEv()};
    let mut ret = unsafe {_ZNK9QBitArray4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QBitArray::swap(QBitArray & other);
impl /*struct*/ QBitArray {
  pub fn swap<RetType, T: QBitArray_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QBitArray_swap<RetType> {
  fn swap(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::swap(QBitArray & other);
impl<'a> /*trait*/ QBitArray_swap<()> for (&'a QBitArray) {
  fn swap(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QBitArray4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QBitArray::count();
impl /*struct*/ QBitArray {
  pub fn count<RetType, T: QBitArray_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QBitArray_count<RetType> {
  fn count(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  int QBitArray::count();
impl<'a> /*trait*/ QBitArray_count<i32> for () {
  fn count(self , rsthis: & QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray5countEv()};
    let mut ret = unsafe {_ZNK9QBitArray5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QBitArray::count(bool on);
impl<'a> /*trait*/ QBitArray_count<i32> for (i8) {
  fn count(self , rsthis: & QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray5countEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK9QBitArray5countEb(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QBitArray::detach();
impl /*struct*/ QBitArray {
  pub fn detach<RetType, T: QBitArray_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QBitArray_detach<RetType> {
  fn detach(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::detach();
impl<'a> /*trait*/ QBitArray_detach<()> for () {
  fn detach(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6detachEv()};
     unsafe {_ZN9QBitArray6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBitArray::QBitArray();
impl<'a> /*trait*/ QBitArray_new for () {
  fn new(self) -> QBitArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC2Ev()};
    let ctysz: c_int = unsafe{QBitArray_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN9QBitArrayC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QBitArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QBitArray::at(int i);
impl /*struct*/ QBitArray {
  pub fn at<RetType, T: QBitArray_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QBitArray_at<RetType> {
  fn at(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  bool QBitArray::at(int i);
impl<'a> /*trait*/ QBitArray_at<i8> for (i32) {
  fn at(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QBitArray2atEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBitArray::clear();
impl /*struct*/ QBitArray {
  pub fn clear<RetType, T: QBitArray_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QBitArray_clear<RetType> {
  fn clear(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::clear();
impl<'a> /*trait*/ QBitArray_clear<()> for () {
  fn clear(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray5clearEv()};
     unsafe {_ZN9QBitArray5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBitArray::clearBit(int i);
impl /*struct*/ QBitArray {
  pub fn clearBit<RetType, T: QBitArray_clearBit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearBit(self);
    // return 1;
  }
}

pub trait QBitArray_clearBit<RetType> {
  fn clearBit(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::clearBit(int i);
impl<'a> /*trait*/ QBitArray_clearBit<()> for (i32) {
  fn clearBit(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray8clearBitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray8clearBitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBitArray::testBit(int i);
impl /*struct*/ QBitArray {
  pub fn testBit<RetType, T: QBitArray_testBit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.testBit(self);
    // return 1;
  }
}

pub trait QBitArray_testBit<RetType> {
  fn testBit(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  bool QBitArray::testBit(int i);
impl<'a> /*trait*/ QBitArray_testBit<i8> for (i32) {
  fn testBit(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray7testBitEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QBitArray7testBitEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBitArray::truncate(int pos);
impl /*struct*/ QBitArray {
  pub fn truncate<RetType, T: QBitArray_truncate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.truncate(self);
    // return 1;
  }
}

pub trait QBitArray_truncate<RetType> {
  fn truncate(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::truncate(int pos);
impl<'a> /*trait*/ QBitArray_truncate<()> for (i32) {
  fn truncate(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray8truncateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray8truncateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBitArray::toggleBit(int i);
impl /*struct*/ QBitArray {
  pub fn toggleBit<RetType, T: QBitArray_toggleBit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggleBit(self);
    // return 1;
  }
}

pub trait QBitArray_toggleBit<RetType> {
  fn toggleBit(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  bool QBitArray::toggleBit(int i);
impl<'a> /*trait*/ QBitArray_toggleBit<i8> for (i32) {
  fn toggleBit(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray9toggleBitEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QBitArray9toggleBitEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBitArray::QBitArray(const QBitArray & other);
impl<'a> /*trait*/ QBitArray_new for (&'a QBitArray) {
  fn new(self) -> QBitArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArrayC2ERKS_()};
    let ctysz: c_int = unsafe{QBitArray_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QBitArrayC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QBitArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBitArray::fill(bool val, int first, int last);
impl /*struct*/ QBitArray {
  pub fn fill<RetType, T: QBitArray_fill<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QBitArray_fill<RetType> {
  fn fill(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::fill(bool val, int first, int last);
impl<'a> /*trait*/ QBitArray_fill<()> for (i8, i32, i32) {
  fn fill(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4fillEbii()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN9QBitArray4fillEbii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QBitArray::isNull();
impl /*struct*/ QBitArray {
  pub fn isNull<RetType, T: QBitArray_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QBitArray_isNull<RetType> {
  fn isNull(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  bool QBitArray::isNull();
impl<'a> /*trait*/ QBitArray_isNull<i8> for () {
  fn isNull(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray6isNullEv()};
    let mut ret = unsafe {_ZNK9QBitArray6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBitArray::setBit(int i, bool val);
impl<'a> /*trait*/ QBitArray_setBit<()> for (i32, i8) {
  fn setBit(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6setBitEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN9QBitArray6setBitEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QBitArray::resize(int size);
impl /*struct*/ QBitArray {
  pub fn resize<RetType, T: QBitArray_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QBitArray_resize<RetType> {
  fn resize(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  void QBitArray::resize(int size);
impl<'a> /*trait*/ QBitArray_resize<()> for (i32) {
  fn resize(self , rsthis: & QBitArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray6resizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QBitArray6resizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBitArray::isDetached();
impl /*struct*/ QBitArray {
  pub fn isDetached<RetType, T: QBitArray_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QBitArray_isDetached<RetType> {
  fn isDetached(self , rsthis: & QBitArray) -> RetType;
}

  // proto:  bool QBitArray::isDetached();
impl<'a> /*trait*/ QBitArray_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QBitArray10isDetachedEv()};
    let mut ret = unsafe {_ZNK9QBitArray10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QBitArray::fill(bool val, int size);
impl<'a> /*trait*/ QBitArray_fill<i8> for (i8, i32) {
  fn fill(self , rsthis: & QBitArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QBitArray4fillEbi()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN9QBitArray4fillEbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

