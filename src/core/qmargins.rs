// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qmargins.h
// dst-file: /src/core/qmargins.rs
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
// use super::qmargins::QMargins; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMarginsF_Class_Size() -> c_int;
  // proto:  QMargins QMarginsF::toMargins();
  fn C_ZNK9QMarginsF9toMarginsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QMarginsF::right();
  fn C_ZNK9QMarginsF5rightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QMarginsF::isNull();
  fn C_ZNK9QMarginsF6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QMarginsF::setRight(qreal right);
  fn C_ZN9QMarginsF8setRightEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QMarginsF::setTop(qreal top);
  fn C_ZN9QMarginsF6setTopEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QMarginsF::left();
  fn C_ZNK9QMarginsF4leftEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QMarginsF::QMarginsF();
  fn C_ZN9QMarginsFC2Ev() -> u64;
  // proto:  void QMarginsF::QMarginsF(qreal left, qreal top, qreal right, qreal bottom);
  fn C_ZN9QMarginsFC2Edddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> u64;
  // proto:  qreal QMarginsF::bottom();
  fn C_ZNK9QMarginsF6bottomEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QMarginsF::QMarginsF(const QMargins & margins);
  fn C_ZN9QMarginsFC2ERK8QMargins(arg0: *mut c_void) -> u64;
  // proto:  void QMarginsF::setBottom(qreal bottom);
  fn C_ZN9QMarginsF9setBottomEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QMarginsF::top();
  fn C_ZNK9QMarginsF3topEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QMarginsF::setLeft(qreal left);
  fn C_ZN9QMarginsF7setLeftEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  fn QMargins_Class_Size() -> c_int;
  // proto:  void QMargins::setLeft(int left);
  fn C_ZN8QMargins7setLeftEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QMargins::setRight(int right);
  fn C_ZN8QMargins8setRightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QMargins::left();
  fn C_ZNK8QMargins4leftEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMargins::top();
  fn C_ZNK8QMargins3topEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QMargins::setTop(int top);
  fn C_ZN8QMargins6setTopEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QMargins::setBottom(int bottom);
  fn C_ZN8QMargins9setBottomEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QMargins::right();
  fn C_ZNK8QMargins5rightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMargins::bottom();
  fn C_ZNK8QMargins6bottomEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QMargins::isNull();
  fn C_ZNK8QMargins6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QMargins::QMargins();
  fn C_ZN8QMarginsC2Ev() -> u64;
  // proto:  void QMargins::QMargins(int left, int top, int right, int bottom);
  fn C_ZN8QMarginsC2Eiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QMarginsF)=32
#[derive(Default)]
pub struct QMarginsF {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QMargins)=16
#[derive(Default)]
pub struct QMargins {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMarginsF {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMarginsF {
    return QMarginsF{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QMargins QMarginsF::toMargins();
impl /*struct*/ QMarginsF {
  pub fn toMargins<RetType, T: QMarginsF_toMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toMargins(self);
    // return 1;
  }
}

pub trait QMarginsF_toMargins<RetType> {
  fn toMargins(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  QMargins QMarginsF::toMargins();
impl<'a> /*trait*/ QMarginsF_toMargins<QMargins> for () {
  fn toMargins(self , rsthis: & QMarginsF) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF9toMarginsEv()};
    let mut ret = unsafe {C_ZNK9QMarginsF9toMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QMarginsF::right();
impl /*struct*/ QMarginsF {
  pub fn right<RetType, T: QMarginsF_right<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QMarginsF_right<RetType> {
  fn right(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  qreal QMarginsF::right();
impl<'a> /*trait*/ QMarginsF_right<f64> for () {
  fn right(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF5rightEv()};
    let mut ret = unsafe {C_ZNK9QMarginsF5rightEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QMarginsF::isNull();
impl /*struct*/ QMarginsF {
  pub fn isNull<RetType, T: QMarginsF_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QMarginsF_isNull<RetType> {
  fn isNull(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  bool QMarginsF::isNull();
impl<'a> /*trait*/ QMarginsF_isNull<i8> for () {
  fn isNull(self , rsthis: & QMarginsF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF6isNullEv()};
    let mut ret = unsafe {C_ZNK9QMarginsF6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMarginsF::setRight(qreal right);
impl /*struct*/ QMarginsF {
  pub fn setRight<RetType, T: QMarginsF_setRight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRight(self);
    // return 1;
  }
}

pub trait QMarginsF_setRight<RetType> {
  fn setRight(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  void QMarginsF::setRight(qreal right);
impl<'a> /*trait*/ QMarginsF_setRight<()> for (f64) {
  fn setRight(self , rsthis: & QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF8setRightEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN9QMarginsF8setRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMarginsF::setTop(qreal top);
impl /*struct*/ QMarginsF {
  pub fn setTop<RetType, T: QMarginsF_setTop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QMarginsF_setTop<RetType> {
  fn setTop(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  void QMarginsF::setTop(qreal top);
impl<'a> /*trait*/ QMarginsF_setTop<()> for (f64) {
  fn setTop(self , rsthis: & QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN9QMarginsF6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QMarginsF::left();
impl /*struct*/ QMarginsF {
  pub fn left<RetType, T: QMarginsF_left<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QMarginsF_left<RetType> {
  fn left(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  qreal QMarginsF::left();
impl<'a> /*trait*/ QMarginsF_left<f64> for () {
  fn left(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF4leftEv()};
    let mut ret = unsafe {C_ZNK9QMarginsF4leftEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QMarginsF::QMarginsF();
impl /*struct*/ QMarginsF {
  pub fn new<T: QMarginsF_new>(value: T) -> QMarginsF {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMarginsF_new {
  fn new(self) -> QMarginsF;
}

  // proto:  void QMarginsF::QMarginsF();
impl<'a> /*trait*/ QMarginsF_new for () {
  fn new(self) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC2Ev()};
    let ctysz: c_int = unsafe{QMarginsF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QMarginsFC2Ev()};
    let rsthis = QMarginsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMarginsF::QMarginsF(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QMarginsF_new for (f64, f64, f64, f64) {
  fn new(self) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC2Edddd()};
    let ctysz: c_int = unsafe{QMarginsF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let qthis: u64 = unsafe {C_ZN9QMarginsFC2Edddd(arg0, arg1, arg2, arg3)};
    let rsthis = QMarginsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QMarginsF::bottom();
impl /*struct*/ QMarginsF {
  pub fn bottom<RetType, T: QMarginsF_bottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QMarginsF_bottom<RetType> {
  fn bottom(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  qreal QMarginsF::bottom();
impl<'a> /*trait*/ QMarginsF_bottom<f64> for () {
  fn bottom(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF6bottomEv()};
    let mut ret = unsafe {C_ZNK9QMarginsF6bottomEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QMarginsF::QMarginsF(const QMargins & margins);
impl<'a> /*trait*/ QMarginsF_new for (&'a QMargins) {
  fn new(self) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC2ERK8QMargins()};
    let ctysz: c_int = unsafe{QMarginsF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QMarginsFC2ERK8QMargins(arg0)};
    let rsthis = QMarginsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMarginsF::setBottom(qreal bottom);
impl /*struct*/ QMarginsF {
  pub fn setBottom<RetType, T: QMarginsF_setBottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QMarginsF_setBottom<RetType> {
  fn setBottom(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  void QMarginsF::setBottom(qreal bottom);
impl<'a> /*trait*/ QMarginsF_setBottom<()> for (f64) {
  fn setBottom(self , rsthis: & QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN9QMarginsF9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QMarginsF::top();
impl /*struct*/ QMarginsF {
  pub fn top<RetType, T: QMarginsF_top<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QMarginsF_top<RetType> {
  fn top(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  qreal QMarginsF::top();
impl<'a> /*trait*/ QMarginsF_top<f64> for () {
  fn top(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF3topEv()};
    let mut ret = unsafe {C_ZNK9QMarginsF3topEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QMarginsF::setLeft(qreal left);
impl /*struct*/ QMarginsF {
  pub fn setLeft<RetType, T: QMarginsF_setLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeft(self);
    // return 1;
  }
}

pub trait QMarginsF_setLeft<RetType> {
  fn setLeft(self , rsthis: & QMarginsF) -> RetType;
}

  // proto:  void QMarginsF::setLeft(qreal left);
impl<'a> /*trait*/ QMarginsF_setLeft<()> for (f64) {
  fn setLeft(self , rsthis: & QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF7setLeftEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN9QMarginsF7setLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMargins {
    return QMargins{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QMargins::setLeft(int left);
impl /*struct*/ QMargins {
  pub fn setLeft<RetType, T: QMargins_setLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeft(self);
    // return 1;
  }
}

pub trait QMargins_setLeft<RetType> {
  fn setLeft(self , rsthis: & QMargins) -> RetType;
}

  // proto:  void QMargins::setLeft(int left);
impl<'a> /*trait*/ QMargins_setLeft<()> for (i32) {
  fn setLeft(self , rsthis: & QMargins) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins7setLeftEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN8QMargins7setLeftEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMargins::setRight(int right);
impl /*struct*/ QMargins {
  pub fn setRight<RetType, T: QMargins_setRight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRight(self);
    // return 1;
  }
}

pub trait QMargins_setRight<RetType> {
  fn setRight(self , rsthis: & QMargins) -> RetType;
}

  // proto:  void QMargins::setRight(int right);
impl<'a> /*trait*/ QMargins_setRight<()> for (i32) {
  fn setRight(self , rsthis: & QMargins) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins8setRightEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN8QMargins8setRightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QMargins::left();
impl /*struct*/ QMargins {
  pub fn left<RetType, T: QMargins_left<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QMargins_left<RetType> {
  fn left(self , rsthis: & QMargins) -> RetType;
}

  // proto:  int QMargins::left();
impl<'a> /*trait*/ QMargins_left<i32> for () {
  fn left(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins4leftEv()};
    let mut ret = unsafe {C_ZNK8QMargins4leftEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMargins::top();
impl /*struct*/ QMargins {
  pub fn top<RetType, T: QMargins_top<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QMargins_top<RetType> {
  fn top(self , rsthis: & QMargins) -> RetType;
}

  // proto:  int QMargins::top();
impl<'a> /*trait*/ QMargins_top<i32> for () {
  fn top(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins3topEv()};
    let mut ret = unsafe {C_ZNK8QMargins3topEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QMargins::setTop(int top);
impl /*struct*/ QMargins {
  pub fn setTop<RetType, T: QMargins_setTop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QMargins_setTop<RetType> {
  fn setTop(self , rsthis: & QMargins) -> RetType;
}

  // proto:  void QMargins::setTop(int top);
impl<'a> /*trait*/ QMargins_setTop<()> for (i32) {
  fn setTop(self , rsthis: & QMargins) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins6setTopEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN8QMargins6setTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMargins::setBottom(int bottom);
impl /*struct*/ QMargins {
  pub fn setBottom<RetType, T: QMargins_setBottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QMargins_setBottom<RetType> {
  fn setBottom(self , rsthis: & QMargins) -> RetType;
}

  // proto:  void QMargins::setBottom(int bottom);
impl<'a> /*trait*/ QMargins_setBottom<()> for (i32) {
  fn setBottom(self , rsthis: & QMargins) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins9setBottomEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN8QMargins9setBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QMargins::right();
impl /*struct*/ QMargins {
  pub fn right<RetType, T: QMargins_right<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QMargins_right<RetType> {
  fn right(self , rsthis: & QMargins) -> RetType;
}

  // proto:  int QMargins::right();
impl<'a> /*trait*/ QMargins_right<i32> for () {
  fn right(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins5rightEv()};
    let mut ret = unsafe {C_ZNK8QMargins5rightEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMargins::bottom();
impl /*struct*/ QMargins {
  pub fn bottom<RetType, T: QMargins_bottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QMargins_bottom<RetType> {
  fn bottom(self , rsthis: & QMargins) -> RetType;
}

  // proto:  int QMargins::bottom();
impl<'a> /*trait*/ QMargins_bottom<i32> for () {
  fn bottom(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins6bottomEv()};
    let mut ret = unsafe {C_ZNK8QMargins6bottomEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QMargins::isNull();
impl /*struct*/ QMargins {
  pub fn isNull<RetType, T: QMargins_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QMargins_isNull<RetType> {
  fn isNull(self , rsthis: & QMargins) -> RetType;
}

  // proto:  bool QMargins::isNull();
impl<'a> /*trait*/ QMargins_isNull<i8> for () {
  fn isNull(self , rsthis: & QMargins) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins6isNullEv()};
    let mut ret = unsafe {C_ZNK8QMargins6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMargins::QMargins();
impl /*struct*/ QMargins {
  pub fn new<T: QMargins_new>(value: T) -> QMargins {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMargins_new {
  fn new(self) -> QMargins;
}

  // proto:  void QMargins::QMargins();
impl<'a> /*trait*/ QMargins_new for () {
  fn new(self) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMarginsC2Ev()};
    let ctysz: c_int = unsafe{QMargins_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN8QMarginsC2Ev()};
    let rsthis = QMargins{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMargins::QMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QMargins_new for (i32, i32, i32, i32) {
  fn new(self) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMarginsC2Eiiii()};
    let ctysz: c_int = unsafe{QMargins_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let qthis: u64 = unsafe {C_ZN8QMarginsC2Eiiii(arg0, arg1, arg2, arg3)};
    let rsthis = QMargins{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

