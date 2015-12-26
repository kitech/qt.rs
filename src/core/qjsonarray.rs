// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
  // proto:  bool QJsonArray::isEmpty();
  fn _ZNK10QJsonArray7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QJsonValue QJsonArray::first();
  fn _ZNK10QJsonArray5firstEv(qthis: *mut c_void);
  // proto:  QJsonValue QJsonArray::takeAt(int i);
  fn _ZN10QJsonArray6takeAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QVariantList QJsonArray::toVariantList();
  fn _ZNK10QJsonArray13toVariantListEv(qthis: *mut c_void);
  // proto:  void QJsonArray::~QJsonArray();
  fn _ZN10QJsonArrayD0Ev(qthis: *mut c_void);
  // proto:  int QJsonArray::size();
  fn _ZNK10QJsonArray4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QJsonArray::QJsonArray();
  fn dector_ZN10QJsonArrayC1Ev() -> *mut c_void;
  fn _ZN10QJsonArrayC1Ev(qthis: *mut c_void);
  // proto:  QJsonValue QJsonArray::at(int i);
  fn _ZNK10QJsonArray2atEi(qthis: *mut c_void, arg0: c_int);
  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
  fn _ZN10QJsonArray14fromStringListERK11QStringList(arg0: *mut c_void);
  // proto:  QJsonValue QJsonArray::last();
  fn _ZNK10QJsonArray4lastEv(qthis: *mut c_void);
  // proto:  void QJsonArray::removeAt(int i);
  fn _ZN10QJsonArray8removeAtEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QJsonArray)=16
pub struct QJsonArray {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QJsonArray {
  pub fn inheritFrom(qthis: *mut c_void) -> QJsonArray {
    return QJsonArray{qclsinst: qthis};
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
  pub fn Free<RetType, T: QJsonArray_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QJsonArray_Free<RetType> {
  fn Free(self , rsthis: & QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::~QJsonArray();
impl<'a> /*trait*/ QJsonArray_Free<()> for () {
  fn Free(self , rsthis: & QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArrayD0Ev()};
     unsafe {_ZN10QJsonArrayD0Ev(rsthis.qclsinst)};
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

  // proto:  void QJsonArray::QJsonArray();
impl /*struct*/ QJsonArray {
  pub fn New<T: QJsonArray_New>(value: T) -> QJsonArray {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonArray_New {
  fn New(self) -> QJsonArray;
}

  // proto:  void QJsonArray::QJsonArray();
impl<'a> /*trait*/ QJsonArray_New for () {
  fn New(self) -> QJsonArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArrayC1Ev()};
    let ctysz: c_int = unsafe{QJsonArray_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN10QJsonArrayC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN10QJsonArrayC1Ev()};
    let rsthis = QJsonArray{qclsinst: qthis};
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

