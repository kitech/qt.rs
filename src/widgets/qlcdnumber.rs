// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
use super::super::core::qobjectdefs::QMetaObject; // 771
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
  fn C_ZN10QLCDNumber7displayEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QLCDNumber::setHexMode();
  fn C_ZN10QLCDNumber10setHexModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLCDNumber::display(double num);
  fn C_ZN10QLCDNumber7displayEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  const QMetaObject * QLCDNumber::metaObject();
  fn C_ZNK10QLCDNumber10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QLCDNumber::digitCount();
  fn C_ZNK10QLCDNumber10digitCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLCDNumber::~QLCDNumber();
  fn C_ZN10QLCDNumberD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLCDNumber::checkOverflow(int num);
  fn C_ZNK10QLCDNumber13checkOverflowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QLCDNumber::setDecMode();
  fn C_ZN10QLCDNumber10setDecModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
  fn C_ZN10QLCDNumberC2EjP7QWidget(arg0: c_uint, arg1: *mut c_void) -> u64;
  // proto:  bool QLCDNumber::checkOverflow(double num);
  fn C_ZNK10QLCDNumber13checkOverflowEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_char;
  // proto:  QSize QLCDNumber::sizeHint();
  fn C_ZNK10QLCDNumber8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLCDNumber::display(const QString & str);
  fn C_ZN10QLCDNumber7displayERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLCDNumber::QLCDNumber(QWidget * parent);
  fn C_ZN10QLCDNumberC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  double QLCDNumber::value();
  fn C_ZNK10QLCDNumber5valueEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QLCDNumber::setBinMode();
  fn C_ZN10QLCDNumber10setBinModeEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QLCDNumber::intValue();
  fn C_ZNK10QLCDNumber8intValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLCDNumber::setDigitCount(int nDigits);
  fn C_ZN10QLCDNumber13setDigitCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
  fn C_ZN10QLCDNumber20setSmallDecimalPointEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QLCDNumber::smallDecimalPoint();
  fn C_ZNK10QLCDNumber17smallDecimalPointEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLCDNumber::setOctMode();
  fn C_ZN10QLCDNumber10setOctModeEv(qthis: u64 /* *mut c_void*/);
  fn QLCDNumber_SlotProxy_connect__ZN10QLCDNumber8overflowEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLCDNumber)=1
#[derive(Default)]
pub struct QLCDNumber {
  qbase: QFrame,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _overflow: QLCDNumber_overflow_signal,
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
     unsafe {C_ZN10QLCDNumber7displayEi(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN10QLCDNumber10setHexModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::display(double num);
impl<'a> /*trait*/ QLCDNumber_display<()> for (f64) {
  fn display(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN10QLCDNumber7displayEd(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QLCDNumber_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QLCDNumber) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QLCDNumber10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK10QLCDNumber10digitCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLCDNumber::~QLCDNumber();
impl /*struct*/ QLCDNumber {
  pub fn free<RetType, T: QLCDNumber_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLCDNumber_free<RetType> {
  fn free(self , rsthis: & QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::~QLCDNumber();
impl<'a> /*trait*/ QLCDNumber_free<()> for () {
  fn free(self , rsthis: & QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberD2Ev()};
     unsafe {C_ZN10QLCDNumberD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK10QLCDNumber13checkOverflowEi(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN10QLCDNumber10setDecModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
impl /*struct*/ QLCDNumber {
  pub fn new<T: QLCDNumber_new>(value: T) -> QLCDNumber {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLCDNumber_new {
  fn new(self) -> QLCDNumber;
}

  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_new for (u32, &'a QWidget) {
  fn new(self) -> QLCDNumber {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC2EjP7QWidget()};
    let ctysz: c_int = unsafe{QLCDNumber_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QLCDNumberC2EjP7QWidget(arg0, arg1)};
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
    let mut ret = unsafe {C_ZNK10QLCDNumber13checkOverflowEd(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK10QLCDNumber8sizeHintEv(rsthis.qclsinst)};
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
     unsafe {C_ZN10QLCDNumber7displayERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_new for (&'a QWidget) {
  fn new(self) -> QLCDNumber {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC2EP7QWidget()};
    let ctysz: c_int = unsafe{QLCDNumber_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QLCDNumberC2EP7QWidget(arg0)};
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
    let mut ret = unsafe {C_ZNK10QLCDNumber5valueEv(rsthis.qclsinst)};
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
     unsafe {C_ZN10QLCDNumber10setBinModeEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK10QLCDNumber8intValueEv(rsthis.qclsinst)};
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
     unsafe {C_ZN10QLCDNumber13setDigitCountEi(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN10QLCDNumber20setSmallDecimalPointEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK10QLCDNumber17smallDecimalPointEv(rsthis.qclsinst)};
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
     unsafe {C_ZN10QLCDNumber10setOctModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QLCDNumber_overflow
pub struct QLCDNumber_overflow_signal{poi:u64}
impl /* struct */ QLCDNumber {
  pub fn overflow(&self) -> QLCDNumber_overflow_signal {
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
extern fn QLCDNumber_overflow_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QLCDNumber_overflow_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QLCDNumber_overflow_signal_connect for fn() {
  fn connect(self, sigthis: QLCDNumber_overflow_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLCDNumber_overflow_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QLCDNumber_SlotProxy_connect__ZN10QLCDNumber8overflowEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QLCDNumber_overflow_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QLCDNumber_overflow_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QLCDNumber_overflow_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QLCDNumber_SlotProxy_connect__ZN10QLCDNumber8overflowEv(arg0, arg1, arg2)};
  }
}
// <= body block end

