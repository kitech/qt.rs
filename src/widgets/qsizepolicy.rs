// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qsizepolicy.h
// dst-file: /src/widgets/qsizepolicy.rs
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
  fn QSizePolicy_Class_Size() -> c_int;
  // proto:  bool QSizePolicy::hasHeightForWidth();
  fn _ZNK11QSizePolicy17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSizePolicy::retainSizeWhenHidden();
  fn _ZNK11QSizePolicy20retainSizeWhenHiddenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSizePolicy::hasWidthForHeight();
  fn _ZNK11QSizePolicy17hasWidthForHeightEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSizePolicy::transpose();
  fn _ZN11QSizePolicy9transposeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSizePolicy::setWidthForHeight(bool b);
  fn _ZN11QSizePolicy17setWidthForHeightEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QSizePolicy::setVerticalStretch(int stretchFactor);
  fn _ZN11QSizePolicy18setVerticalStretchEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSizePolicy::setHeightForWidth(bool b);
  fn _ZN11QSizePolicy17setHeightForWidthEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
  fn _ZN11QSizePolicy23setRetainSizeWhenHiddenEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QSizePolicy::horizontalStretch();
  fn _ZNK11QSizePolicy17horizontalStretchEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSizePolicy::setHorizontalStretch(int stretchFactor);
  fn _ZN11QSizePolicy20setHorizontalStretchEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSizePolicy::QSizePolicy(int i);
  fn dector_ZN11QSizePolicyC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN11QSizePolicyC1Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSizePolicy::QSizePolicy();
  fn dector_ZN11QSizePolicyC1Ev() -> *mut c_void;
  fn _ZN11QSizePolicyC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QSizePolicy::verticalStretch();
  fn _ZNK11QSizePolicy15verticalStretchEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QSizePolicy)=4
#[derive(Default)]
pub struct QSizePolicy {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSizePolicy {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSizePolicy {
    return QSizePolicy{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QSizePolicy::hasHeightForWidth();
impl /*struct*/ QSizePolicy {
  pub fn hasHeightForWidth<RetType, T: QSizePolicy_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QSizePolicy_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  bool QSizePolicy::hasHeightForWidth();
impl<'a> /*trait*/ QSizePolicy_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QSizePolicy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSizePolicy::retainSizeWhenHidden();
impl /*struct*/ QSizePolicy {
  pub fn retainSizeWhenHidden<RetType, T: QSizePolicy_retainSizeWhenHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.retainSizeWhenHidden(self);
    // return 1;
  }
}

pub trait QSizePolicy_retainSizeWhenHidden<RetType> {
  fn retainSizeWhenHidden(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  bool QSizePolicy::retainSizeWhenHidden();
impl<'a> /*trait*/ QSizePolicy_retainSizeWhenHidden<i8> for () {
  fn retainSizeWhenHidden(self , rsthis: & QSizePolicy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy20retainSizeWhenHiddenEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy20retainSizeWhenHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSizePolicy::hasWidthForHeight();
impl /*struct*/ QSizePolicy {
  pub fn hasWidthForHeight<RetType, T: QSizePolicy_hasWidthForHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasWidthForHeight(self);
    // return 1;
  }
}

pub trait QSizePolicy_hasWidthForHeight<RetType> {
  fn hasWidthForHeight(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  bool QSizePolicy::hasWidthForHeight();
impl<'a> /*trait*/ QSizePolicy_hasWidthForHeight<i8> for () {
  fn hasWidthForHeight(self , rsthis: & QSizePolicy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17hasWidthForHeightEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy17hasWidthForHeightEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSizePolicy::transpose();
impl /*struct*/ QSizePolicy {
  pub fn transpose<RetType, T: QSizePolicy_transpose<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transpose(self);
    // return 1;
  }
}

pub trait QSizePolicy_transpose<RetType> {
  fn transpose(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::transpose();
impl<'a> /*trait*/ QSizePolicy_transpose<()> for () {
  fn transpose(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy9transposeEv()};
     unsafe {_ZN11QSizePolicy9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSizePolicy::setWidthForHeight(bool b);
impl /*struct*/ QSizePolicy {
  pub fn setWidthForHeight<RetType, T: QSizePolicy_setWidthForHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidthForHeight(self);
    // return 1;
  }
}

pub trait QSizePolicy_setWidthForHeight<RetType> {
  fn setWidthForHeight(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::setWidthForHeight(bool b);
impl<'a> /*trait*/ QSizePolicy_setWidthForHeight<()> for (i8) {
  fn setWidthForHeight(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy17setWidthForHeightEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QSizePolicy17setWidthForHeightEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSizePolicy::setVerticalStretch(int stretchFactor);
impl /*struct*/ QSizePolicy {
  pub fn setVerticalStretch<RetType, T: QSizePolicy_setVerticalStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_setVerticalStretch<RetType> {
  fn setVerticalStretch(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::setVerticalStretch(int stretchFactor);
impl<'a> /*trait*/ QSizePolicy_setVerticalStretch<()> for (i32) {
  fn setVerticalStretch(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy18setVerticalStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QSizePolicy18setVerticalStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSizePolicy::setHeightForWidth(bool b);
impl /*struct*/ QSizePolicy {
  pub fn setHeightForWidth<RetType, T: QSizePolicy_setHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeightForWidth(self);
    // return 1;
  }
}

pub trait QSizePolicy_setHeightForWidth<RetType> {
  fn setHeightForWidth(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::setHeightForWidth(bool b);
impl<'a> /*trait*/ QSizePolicy_setHeightForWidth<()> for (i8) {
  fn setHeightForWidth(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy17setHeightForWidthEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QSizePolicy17setHeightForWidthEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
impl /*struct*/ QSizePolicy {
  pub fn setRetainSizeWhenHidden<RetType, T: QSizePolicy_setRetainSizeWhenHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRetainSizeWhenHidden(self);
    // return 1;
  }
}

pub trait QSizePolicy_setRetainSizeWhenHidden<RetType> {
  fn setRetainSizeWhenHidden(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
impl<'a> /*trait*/ QSizePolicy_setRetainSizeWhenHidden<()> for (i8) {
  fn setRetainSizeWhenHidden(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy23setRetainSizeWhenHiddenEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QSizePolicy23setRetainSizeWhenHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSizePolicy::horizontalStretch();
impl /*struct*/ QSizePolicy {
  pub fn horizontalStretch<RetType, T: QSizePolicy_horizontalStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_horizontalStretch<RetType> {
  fn horizontalStretch(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  int QSizePolicy::horizontalStretch();
impl<'a> /*trait*/ QSizePolicy_horizontalStretch<i32> for () {
  fn horizontalStretch(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17horizontalStretchEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy17horizontalStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSizePolicy::setHorizontalStretch(int stretchFactor);
impl /*struct*/ QSizePolicy {
  pub fn setHorizontalStretch<RetType, T: QSizePolicy_setHorizontalStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_setHorizontalStretch<RetType> {
  fn setHorizontalStretch(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::setHorizontalStretch(int stretchFactor);
impl<'a> /*trait*/ QSizePolicy_setHorizontalStretch<()> for (i32) {
  fn setHorizontalStretch(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy20setHorizontalStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QSizePolicy20setHorizontalStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSizePolicy::QSizePolicy(int i);
impl /*struct*/ QSizePolicy {
  pub fn New<T: QSizePolicy_New>(value: T) -> QSizePolicy {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSizePolicy_New {
  fn New(self) -> QSizePolicy;
}

  // proto:  void QSizePolicy::QSizePolicy(int i);
impl<'a> /*trait*/ QSizePolicy_New for (i32) {
  fn New(self) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicyC1Ei()};
    let ctysz: c_int = unsafe{QSizePolicy_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    // unsafe {_ZN11QSizePolicyC1Ei(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QSizePolicyC1Ei(arg0)} as u64;
    let rsthis = QSizePolicy{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSizePolicy::QSizePolicy();
impl<'a> /*trait*/ QSizePolicy_New for () {
  fn New(self) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicyC1Ev()};
    let ctysz: c_int = unsafe{QSizePolicy_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN11QSizePolicyC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN11QSizePolicyC1Ev()} as u64;
    let rsthis = QSizePolicy{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QSizePolicy::verticalStretch();
impl /*struct*/ QSizePolicy {
  pub fn verticalStretch<RetType, T: QSizePolicy_verticalStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_verticalStretch<RetType> {
  fn verticalStretch(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  int QSizePolicy::verticalStretch();
impl<'a> /*trait*/ QSizePolicy_verticalStretch<i32> for () {
  fn verticalStretch(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy15verticalStretchEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy15verticalStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

