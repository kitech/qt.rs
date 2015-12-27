// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qlcdnumber.h
// dst-file: /src/widgets/qlcdnumber.rs
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
use super::qframe::QFrame; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLCDNumber_Class_Size() -> c_int;
  // proto:  void QLCDNumber::display(int num);
  fn _ZN10QLCDNumber7displayEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QLCDNumber::setHexMode();
  fn _ZN10QLCDNumber10setHexModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLCDNumber::display(double num);
  fn _ZN10QLCDNumber7displayEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  const QMetaObject * QLCDNumber::metaObject();
  fn _ZNK10QLCDNumber10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLCDNumber::QLCDNumber(const QLCDNumber & );
  fn dector_ZN10QLCDNumberC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QLCDNumberC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QLCDNumber::digitCount();
  fn _ZNK10QLCDNumber10digitCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLCDNumber::~QLCDNumber();
  fn _ZN10QLCDNumberD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLCDNumber::checkOverflow(int num);
  fn _ZNK10QLCDNumber13checkOverflowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QLCDNumber::setDecMode();
  fn _ZN10QLCDNumber10setDecModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
  fn dector_ZN10QLCDNumberC1EjP7QWidget(arg0: c_uint, arg1: *mut c_void) -> *mut c_void;
  fn _ZN10QLCDNumberC1EjP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: *mut c_void);
  // proto:  bool QLCDNumber::checkOverflow(double num);
  fn _ZNK10QLCDNumber13checkOverflowEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_char;
  // proto:  QSize QLCDNumber::sizeHint();
  fn _ZNK10QLCDNumber8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLCDNumber::display(const QString & str);
  fn _ZN10QLCDNumber7displayERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLCDNumber::QLCDNumber(QWidget * parent);
  fn dector_ZN10QLCDNumberC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QLCDNumberC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  double QLCDNumber::value();
  fn _ZNK10QLCDNumber5valueEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QLCDNumber::setBinMode();
  fn _ZN10QLCDNumber10setBinModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QLCDNumber::intValue();
  fn _ZNK10QLCDNumber8intValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLCDNumber::setDigitCount(int nDigits);
  fn _ZN10QLCDNumber13setDigitCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
  fn _ZN10QLCDNumber20setSmallDecimalPointEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QLCDNumber::smallDecimalPoint();
  fn _ZNK10QLCDNumber17smallDecimalPointEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLCDNumber::setOctMode();
  fn _ZN10QLCDNumber10setOctModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLCDNumber::overflow();
  fn _ZN10QLCDNumber8overflowEv(qthis: u64 /* *mut c_void*/);
  fn QLCDNumber_SlotProxy_connect__ZN10QLCDNumber8overflowEv(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLCDNumber)=1
#[derive(Default)]
pub struct QLCDNumber {
  qbase: QFrame,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _overflow_1: QLCDNumber_overflow_signal,
}

impl /*struct*/ QLCDNumber {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLCDNumber {
    return QLCDNumber{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QLCDNumber {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return & self.qbase;
  }
}
impl AsRef<QFrame> for QLCDNumber {
  fn as_ref(& self) -> & QFrame {
    return & self.qbase;
  }
}
  // proto:  void QLCDNumber::display(int num);
impl /*struct*/ QLCDNumber {
  pub fn display<RetType, T: QLCDNumber_display<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.display(self);
    // return 1;
  }
}

pub trait QLCDNumber_display<RetType> {
  fn display(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::display(int num);
impl<'a> /*trait*/ QLCDNumber_display<()> for (i32) {
  fn display(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QLCDNumber7displayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::setHexMode();
impl /*struct*/ QLCDNumber {
  pub fn setHexMode<RetType, T: QLCDNumber_setHexMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHexMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setHexMode<RetType> {
  fn setHexMode(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setHexMode();
impl<'a> /*trait*/ QLCDNumber_setHexMode<()> for () {
  fn setHexMode(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setHexModeEv()};
     unsafe {_ZN10QLCDNumber10setHexModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::display(double num);
impl<'a> /*trait*/ QLCDNumber_display<()> for (f64) {
  fn display(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QLCDNumber7displayEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QLCDNumber::metaObject();
impl /*struct*/ QLCDNumber {
  pub fn metaObject<RetType, T: QLCDNumber_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLCDNumber_metaObject<RetType> {
  fn metaObject(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  const QMetaObject * QLCDNumber::metaObject();
impl<'a> /*trait*/ QLCDNumber_metaObject<()> for () {
  fn metaObject(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10metaObjectEv()};
     unsafe {_ZNK10QLCDNumber10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(const QLCDNumber & );
impl /*struct*/ QLCDNumber {
  pub fn New<T: QLCDNumber_New>(value: T) -> QLCDNumber {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLCDNumber_New {
  fn New(self) -> QLCDNumber;
}

  // proto:  void QLCDNumber::QLCDNumber(const QLCDNumber & );
impl<'a> /*trait*/ QLCDNumber_New for (&'a QLCDNumber) {
  fn New(self) -> QLCDNumber {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1ERKS_()};
    let ctysz: c_int = unsafe{QLCDNumber_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QLCDNumberC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QLCDNumberC1ERKS_(arg0)} as u64;
    let rsthis = QLCDNumber{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLCDNumber::digitCount();
impl /*struct*/ QLCDNumber {
  pub fn digitCount<RetType, T: QLCDNumber_digitCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.digitCount(self);
    // return 1;
  }
}

pub trait QLCDNumber_digitCount<RetType> {
  fn digitCount(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  int QLCDNumber::digitCount();
impl<'a> /*trait*/ QLCDNumber_digitCount<i32> for () {
  fn digitCount(self , rsthis: & QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10digitCountEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber10digitCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLCDNumber::~QLCDNumber();
impl /*struct*/ QLCDNumber {
  pub fn Free<RetType, T: QLCDNumber_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QLCDNumber_Free<RetType> {
  fn Free(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::~QLCDNumber();
impl<'a> /*trait*/ QLCDNumber_Free<()> for () {
  fn Free(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberD0Ev()};
     unsafe {_ZN10QLCDNumberD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLCDNumber::checkOverflow(int num);
impl /*struct*/ QLCDNumber {
  pub fn checkOverflow<RetType, T: QLCDNumber_checkOverflow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.checkOverflow(self);
    // return 1;
  }
}

pub trait QLCDNumber_checkOverflow<RetType> {
  fn checkOverflow(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  bool QLCDNumber::checkOverflow(int num);
impl<'a> /*trait*/ QLCDNumber_checkOverflow<i8> for (i32) {
  fn checkOverflow(self , rsthis: & QLCDNumber) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber13checkOverflowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QLCDNumber13checkOverflowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setDecMode();
impl /*struct*/ QLCDNumber {
  pub fn setDecMode<RetType, T: QLCDNumber_setDecMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDecMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setDecMode<RetType> {
  fn setDecMode(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setDecMode();
impl<'a> /*trait*/ QLCDNumber_setDecMode<()> for () {
  fn setDecMode(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setDecModeEv()};
     unsafe {_ZN10QLCDNumber10setDecModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_New for (u32, &'a QWidget) {
  fn New(self) -> QLCDNumber {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1EjP7QWidget()};
    let ctysz: c_int = unsafe{QLCDNumber_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN10QLCDNumberC1EjP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN10QLCDNumberC1EjP7QWidget(arg0, arg1)} as u64;
    let rsthis = QLCDNumber{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QLCDNumber::checkOverflow(double num);
impl<'a> /*trait*/ QLCDNumber_checkOverflow<i8> for (f64) {
  fn checkOverflow(self , rsthis: & QLCDNumber) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber13checkOverflowEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK10QLCDNumber13checkOverflowEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QLCDNumber::sizeHint();
impl /*struct*/ QLCDNumber {
  pub fn sizeHint<RetType, T: QLCDNumber_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLCDNumber_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  QSize QLCDNumber::sizeHint();
impl<'a> /*trait*/ QLCDNumber_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QLCDNumber) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLCDNumber::display(const QString & str);
impl<'a> /*trait*/ QLCDNumber_display<()> for (&'a QString) {
  fn display(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QLCDNumber7displayERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_New for (&'a QWidget) {
  fn New(self) -> QLCDNumber {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1EP7QWidget()};
    let ctysz: c_int = unsafe{QLCDNumber_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QLCDNumberC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QLCDNumberC1EP7QWidget(arg0)} as u64;
    let rsthis = QLCDNumber{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  double QLCDNumber::value();
impl /*struct*/ QLCDNumber {
  pub fn value<RetType, T: QLCDNumber_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QLCDNumber_value<RetType> {
  fn value(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  double QLCDNumber::value();
impl<'a> /*trait*/ QLCDNumber_value<f64> for () {
  fn value(self , rsthis: & QLCDNumber) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber5valueEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setBinMode();
impl /*struct*/ QLCDNumber {
  pub fn setBinMode<RetType, T: QLCDNumber_setBinMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBinMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setBinMode<RetType> {
  fn setBinMode(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setBinMode();
impl<'a> /*trait*/ QLCDNumber_setBinMode<()> for () {
  fn setBinMode(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setBinModeEv()};
     unsafe {_ZN10QLCDNumber10setBinModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QLCDNumber::intValue();
impl /*struct*/ QLCDNumber {
  pub fn intValue<RetType, T: QLCDNumber_intValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intValue(self);
    // return 1;
  }
}

pub trait QLCDNumber_intValue<RetType> {
  fn intValue(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  int QLCDNumber::intValue();
impl<'a> /*trait*/ QLCDNumber_intValue<i32> for () {
  fn intValue(self , rsthis: & QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber8intValueEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber8intValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setDigitCount(int nDigits);
impl /*struct*/ QLCDNumber {
  pub fn setDigitCount<RetType, T: QLCDNumber_setDigitCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDigitCount(self);
    // return 1;
  }
}

pub trait QLCDNumber_setDigitCount<RetType> {
  fn setDigitCount(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setDigitCount(int nDigits);
impl<'a> /*trait*/ QLCDNumber_setDigitCount<()> for (i32) {
  fn setDigitCount(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber13setDigitCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QLCDNumber13setDigitCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
impl /*struct*/ QLCDNumber {
  pub fn setSmallDecimalPoint<RetType, T: QLCDNumber_setSmallDecimalPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSmallDecimalPoint(self);
    // return 1;
  }
}

pub trait QLCDNumber_setSmallDecimalPoint<RetType> {
  fn setSmallDecimalPoint(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
impl<'a> /*trait*/ QLCDNumber_setSmallDecimalPoint<()> for (i8) {
  fn setSmallDecimalPoint(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber20setSmallDecimalPointEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QLCDNumber20setSmallDecimalPointEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QLCDNumber::smallDecimalPoint();
impl /*struct*/ QLCDNumber {
  pub fn smallDecimalPoint<RetType, T: QLCDNumber_smallDecimalPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.smallDecimalPoint(self);
    // return 1;
  }
}

pub trait QLCDNumber_smallDecimalPoint<RetType> {
  fn smallDecimalPoint(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  bool QLCDNumber::smallDecimalPoint();
impl<'a> /*trait*/ QLCDNumber_smallDecimalPoint<i8> for () {
  fn smallDecimalPoint(self , rsthis: & QLCDNumber) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber17smallDecimalPointEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber17smallDecimalPointEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setOctMode();
impl /*struct*/ QLCDNumber {
  pub fn setOctMode<RetType, T: QLCDNumber_setOctMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOctMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setOctMode<RetType> {
  fn setOctMode(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setOctMode();
impl<'a> /*trait*/ QLCDNumber_setOctMode<()> for () {
  fn setOctMode(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setOctModeEv()};
     unsafe {_ZN10QLCDNumber10setOctModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::overflow();
impl /*struct*/ QLCDNumber {
  pub fn overflow<RetType, T: QLCDNumber_overflow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overflow(self);
    // return 1;
  }
}

pub trait QLCDNumber_overflow<RetType> {
  fn overflow(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::overflow();
impl<'a> /*trait*/ QLCDNumber_overflow<()> for () {
  fn overflow(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber8overflowEv()};
     unsafe {_ZN10QLCDNumber8overflowEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QLCDNumber_overflow
pub struct QLCDNumber_overflow_signal{poi:u64}
impl /* struct */ QLCDNumber {
  pub fn overflow_1(self) -> QLCDNumber_overflow_signal {
     return QLCDNumber_overflow_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QLCDNumber_overflow_signal {
  pub fn connect<T: QLCDNumber_overflow_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QLCDNumber_overflow_signal_connect {
  fn connect(self, sigthis: QLCDNumber_overflow_signal);
}

// overflow()
extern fn QLCDNumber_overflow_signal_connect_cb_0() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QLCDNumber_overflow_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QLCDNumber_overflow_signal) {
    // do smth...
    unsafe {QLCDNumber_SlotProxy_connect__ZN10QLCDNumber8overflowEv(sigthis.poi as *mut c_void, QLCDNumber_overflow_signal_connect_cb_0 as *mut c_void)};
  }
}
// <= body block end

