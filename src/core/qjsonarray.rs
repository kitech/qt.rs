// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qjsonarray.h
// dst-file: /src/core/qjsonarray.rs
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
use super::qstringlist::QStringList; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJsonArray_Class_Size() -> c_int;
  // proto:  QJsonValue QJsonArray::first();
  fn _ZNK10QJsonArray5firstEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QJsonArray::empty();
  fn _ZNK10QJsonArray5emptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QJsonValue QJsonArray::takeAt(int i);
  fn _ZN10QJsonArray6takeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QJsonArray::removeLast();
  fn _ZN10QJsonArray10removeLastEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QJsonArray::pop_front();
  fn _ZN10QJsonArray9pop_frontEv(qthis: u64 /* *mut c_void*/);
  // proto:  QVariantList QJsonArray::toVariantList();
  fn _ZNK10QJsonArray13toVariantListEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QJsonArray::~QJsonArray();
  fn _ZN10QJsonArrayD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QJsonArray::size();
  fn _ZNK10QJsonArray4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QJsonArray::count();
  fn _ZNK10QJsonArray5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QJsonArray::QJsonArray();
  fn _ZN10QJsonArrayC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QJsonValue QJsonArray::at(int i);
  fn _ZNK10QJsonArray2atEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QJsonArray::pop_back();
  fn _ZN10QJsonArray8pop_backEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QJsonArray::isEmpty();
  fn _ZNK10QJsonArray7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
  fn _ZN10QJsonArray14fromStringListERK11QStringList(arg0: *mut c_void);
  // proto:  QJsonValue QJsonArray::last();
  fn _ZNK10QJsonArray4lastEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QJsonArray::removeFirst();
  fn _ZN10QJsonArray11removeFirstEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QJsonArray::removeAt(int i);
  fn _ZN10QJsonArray8removeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QJsonArray)=16
#[derive(Default)]
pub struct QJsonArray {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJsonArray {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonArray {
    return QJsonArray{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJsonValue QJsonArray::first();
impl /*struct*/ QJsonArray {
  pub fn first<RetType, T: QJsonArray_first<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.first(self);
    // return 1;
  }
}

pub trait QJsonArray_first<RetType> {
  fn first(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::first();
impl<'a> /*trait*/ QJsonArray_first<()> for () {
  fn first(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray5firstEv()};
     unsafe {_ZNK10QJsonArray5firstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJsonArray::empty();
impl /*struct*/ QJsonArray {
  pub fn empty<RetType, T: QJsonArray_empty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.empty(self);
    // return 1;
  }
}

pub trait QJsonArray_empty<RetType> {
  fn empty(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  bool QJsonArray::empty();
impl<'a> /*trait*/ QJsonArray_empty<i8> for () {
  fn empty(self , rsthis: & QJsonArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray5emptyEv()};
    let mut ret = unsafe {_ZNK10QJsonArray5emptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QJsonValue QJsonArray::takeAt(int i);
impl /*struct*/ QJsonArray {
  pub fn takeAt<RetType, T: QJsonArray_takeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QJsonArray_takeAt<RetType> {
  fn takeAt(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::takeAt(int i);
impl<'a> /*trait*/ QJsonArray_takeAt<()> for (i32) {
  fn takeAt(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray6takeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QJsonArray6takeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QJsonArray::removeLast();
impl /*struct*/ QJsonArray {
  pub fn removeLast<RetType, T: QJsonArray_removeLast<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeLast(self);
    // return 1;
  }
}

pub trait QJsonArray_removeLast<RetType> {
  fn removeLast(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::removeLast();
impl<'a> /*trait*/ QJsonArray_removeLast<()> for () {
  fn removeLast(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray10removeLastEv()};
     unsafe {_ZN10QJsonArray10removeLastEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::pop_front();
impl /*struct*/ QJsonArray {
  pub fn pop_front<RetType, T: QJsonArray_pop_front<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pop_front(self);
    // return 1;
  }
}

pub trait QJsonArray_pop_front<RetType> {
  fn pop_front(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::pop_front();
impl<'a> /*trait*/ QJsonArray_pop_front<()> for () {
  fn pop_front(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray9pop_frontEv()};
     unsafe {_ZN10QJsonArray9pop_frontEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariantList QJsonArray::toVariantList();
impl /*struct*/ QJsonArray {
  pub fn toVariantList<RetType, T: QJsonArray_toVariantList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariantList(self);
    // return 1;
  }
}

pub trait QJsonArray_toVariantList<RetType> {
  fn toVariantList(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  QVariantList QJsonArray::toVariantList();
impl<'a> /*trait*/ QJsonArray_toVariantList<()> for () {
  fn toVariantList(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray13toVariantListEv()};
     unsafe {_ZNK10QJsonArray13toVariantListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::~QJsonArray();
impl /*struct*/ QJsonArray {
  pub fn free<RetType, T: QJsonArray_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJsonArray_free<RetType> {
  fn free(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::~QJsonArray();
impl<'a> /*trait*/ QJsonArray_free<()> for () {
  fn free(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArrayD2Ev()};
     unsafe {_ZN10QJsonArrayD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QJsonArray::size();
impl /*struct*/ QJsonArray {
  pub fn size<RetType, T: QJsonArray_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QJsonArray_size<RetType> {
  fn size(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  int QJsonArray::size();
impl<'a> /*trait*/ QJsonArray_size<i32> for () {
  fn size(self , rsthis: & QJsonArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray4sizeEv()};
    let mut ret = unsafe {_ZNK10QJsonArray4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QJsonArray::count();
impl /*struct*/ QJsonArray {
  pub fn count<RetType, T: QJsonArray_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QJsonArray_count<RetType> {
  fn count(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  int QJsonArray::count();
impl<'a> /*trait*/ QJsonArray_count<i32> for () {
  fn count(self , rsthis: & QJsonArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray5countEv()};
    let mut ret = unsafe {_ZNK10QJsonArray5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QJsonArray::QJsonArray();
impl /*struct*/ QJsonArray {
  pub fn new<T: QJsonArray_new>(value: T) -> QJsonArray {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonArray_new {
  fn new(self) -> QJsonArray;
}

  // proto:  void QJsonArray::QJsonArray();
impl<'a> /*trait*/ QJsonArray_new for () {
  fn new(self) -> QJsonArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArrayC2Ev()};
    let ctysz: c_int = unsafe{QJsonArray_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN10QJsonArrayC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJsonArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QJsonValue QJsonArray::at(int i);
impl /*struct*/ QJsonArray {
  pub fn at<RetType, T: QJsonArray_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QJsonArray_at<RetType> {
  fn at(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::at(int i);
impl<'a> /*trait*/ QJsonArray_at<()> for (i32) {
  fn at(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray2atEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK10QJsonArray2atEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QJsonArray::pop_back();
impl /*struct*/ QJsonArray {
  pub fn pop_back<RetType, T: QJsonArray_pop_back<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pop_back(self);
    // return 1;
  }
}

pub trait QJsonArray_pop_back<RetType> {
  fn pop_back(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::pop_back();
impl<'a> /*trait*/ QJsonArray_pop_back<()> for () {
  fn pop_back(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray8pop_backEv()};
     unsafe {_ZN10QJsonArray8pop_backEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJsonArray::isEmpty();
impl /*struct*/ QJsonArray {
  pub fn isEmpty<RetType, T: QJsonArray_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QJsonArray_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  bool QJsonArray::isEmpty();
impl<'a> /*trait*/ QJsonArray_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QJsonArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray7isEmptyEv()};
    let mut ret = unsafe {_ZNK10QJsonArray7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
impl /*struct*/ QJsonArray {
  pub fn fromStringList_s<RetType, T: QJsonArray_fromStringList_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStringList_s();
    // return 1;
  }
}

pub trait QJsonArray_fromStringList_s<RetType> {
  fn fromStringList_s(self ) -> RetType;
}

  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
impl<'a> /*trait*/ QJsonArray_fromStringList_s<()> for (&'a QStringList) {
  fn fromStringList_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray14fromStringListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QJsonArray14fromStringListERK11QStringList(arg0)};
    // return 1;
  }
}

  // proto:  QJsonValue QJsonArray::last();
impl /*struct*/ QJsonArray {
  pub fn last<RetType, T: QJsonArray_last<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.last(self);
    // return 1;
  }
}

pub trait QJsonArray_last<RetType> {
  fn last(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::last();
impl<'a> /*trait*/ QJsonArray_last<()> for () {
  fn last(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray4lastEv()};
     unsafe {_ZNK10QJsonArray4lastEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::removeFirst();
impl /*struct*/ QJsonArray {
  pub fn removeFirst<RetType, T: QJsonArray_removeFirst<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeFirst(self);
    // return 1;
  }
}

pub trait QJsonArray_removeFirst<RetType> {
  fn removeFirst(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::removeFirst();
impl<'a> /*trait*/ QJsonArray_removeFirst<()> for () {
  fn removeFirst(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray11removeFirstEv()};
     unsafe {_ZN10QJsonArray11removeFirstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::removeAt(int i);
impl /*struct*/ QJsonArray {
  pub fn removeAt<RetType, T: QJsonArray_removeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QJsonArray_removeAt<RetType> {
  fn removeAt(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::removeAt(int i);
impl<'a> /*trait*/ QJsonArray_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QJsonArray8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

