// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QLCDNumber::display(int num);
  fn _ZN10QLCDNumber7displayEi(arg0: c_int) -> i32;
  // proto: void QLCDNumber::setHexMode();
  fn _ZN10QLCDNumber10setHexModeEv() -> i32;
  // proto: void QLCDNumber::display(double num);
  fn _ZN10QLCDNumber7displayEd(arg0: c_double) -> i32;
  // proto: const QMetaObject * QLCDNumber::metaObject();
  fn _ZNK10QLCDNumber10metaObjectEv() -> i32;
  // proto: void QLCDNumber::NewQLCDNumber(const QLCDNumber & );
  fn _ZN10QLCDNumberC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QLCDNumber::digitCount();
  fn _ZNK10QLCDNumber10digitCountEv() -> i32;
  // proto: void QLCDNumber::FreeQLCDNumber();
  fn _ZN10QLCDNumberD0Ev() -> i32;
  // proto: bool QLCDNumber::checkOverflow(int num);
  fn _ZNK10QLCDNumber13checkOverflowEi(arg0: c_int) -> i32;
  // proto: void QLCDNumber::setDecMode();
  fn _ZN10QLCDNumber10setDecModeEv() -> i32;
  // proto: void QLCDNumber::NewQLCDNumber(uint numDigits, QWidget * parent);
  fn _ZN10QLCDNumberC1EjP7QWidget(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_void) -> i32;
  // proto: bool QLCDNumber::checkOverflow(double num);
  fn _ZNK10QLCDNumber13checkOverflowEd(arg0: c_double) -> i32;
  // proto: QSize QLCDNumber::sizeHint();
  fn _ZNK10QLCDNumber8sizeHintEv() -> i32;
  // proto: void QLCDNumber::display(const QString & str);
  fn _ZN10QLCDNumber7displayERK7QString(arg0: *const c_void) -> i32;
  // proto: void QLCDNumber::NewQLCDNumber(QWidget * parent);
  fn _ZN10QLCDNumberC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: double QLCDNumber::value();
  fn _ZNK10QLCDNumber5valueEv() -> i32;
  // proto: void QLCDNumber::setBinMode();
  fn _ZN10QLCDNumber10setBinModeEv() -> i32;
  // proto: int QLCDNumber::intValue();
  fn _ZNK10QLCDNumber8intValueEv() -> i32;
  // proto: void QLCDNumber::setDigitCount(int nDigits);
  fn _ZN10QLCDNumber13setDigitCountEi(arg0: c_int) -> i32;
  // proto: void QLCDNumber::setSmallDecimalPoint(bool );
  fn _ZN10QLCDNumber20setSmallDecimalPointEb(arg0: int8_t) -> i32;
  // proto: bool QLCDNumber::smallDecimalPoint();
  fn _ZNK10QLCDNumber17smallDecimalPointEv() -> i32;
  // proto: void QLCDNumber::setOctMode();
  fn _ZN10QLCDNumber10setOctModeEv() -> i32;
  // proto: void QLCDNumber::overflow();
  fn _ZN10QLCDNumber8overflowEv() -> i32;
}

// body block begin
// class sizeof(QLCDNumber)=1
pub struct QLCDNumber {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLCDNumber {
  pub fn display<T: QLCDNumber_display>(&mut self, value: T) -> i32 {
    value.display(self);
    return 1;
  }
}

pub trait QLCDNumber_display {
  fn display(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::display(int num);
impl<'a> /*trait*/ QLCDNumber_display for (i32) {
  fn display(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QLCDNumber7displayEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn setHexMode<T: QLCDNumber_setHexMode>(&mut self, value: T) -> i32 {
    value.setHexMode(self);
    return 1;
  }
}

pub trait QLCDNumber_setHexMode {
  fn setHexMode(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::setHexMode();
impl<'a> /*trait*/ QLCDNumber_setHexMode for () {
  fn setHexMode(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setHexModeEv()};
    unsafe {_ZN10QLCDNumber10setHexModeEv()};
    return 1;
  }
}

// proto: void QLCDNumber::display(double num);
impl<'a> /*trait*/ QLCDNumber_display for (f64) {
  fn display(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN10QLCDNumber7displayEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn metaObject<T: QLCDNumber_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QLCDNumber_metaObject {
  fn metaObject(self, this: &mut QLCDNumber) -> i32;
}

// proto: const QMetaObject * QLCDNumber::metaObject();
impl<'a> /*trait*/ QLCDNumber_metaObject for () {
  fn metaObject(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10metaObjectEv()};
    unsafe {_ZNK10QLCDNumber10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn NewQLCDNumber<T: QLCDNumber_NewQLCDNumber>(value: T) -> QLCDNumber {
    let rsthis = value.NewQLCDNumber();
    return rsthis;
    // return 1;
  }
}

pub trait QLCDNumber_NewQLCDNumber {
  fn NewQLCDNumber(self) -> QLCDNumber;
}

// proto: void QLCDNumber::NewQLCDNumber(const QLCDNumber & );
impl<'a> /*trait*/ QLCDNumber_NewQLCDNumber for (&'a  QLCDNumber) {
  fn NewQLCDNumber(self) -> QLCDNumber {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QLCDNumberC1ERKS_(qthis, arg0)};
    let rsthis = QLCDNumber{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn digitCount<T: QLCDNumber_digitCount>(&mut self, value: T) -> i32 {
    value.digitCount(self);
    return 1;
  }
}

pub trait QLCDNumber_digitCount {
  fn digitCount(self, this: &mut QLCDNumber) -> i32;
}

// proto: int QLCDNumber::digitCount();
impl<'a> /*trait*/ QLCDNumber_digitCount for () {
  fn digitCount(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10digitCountEv()};
    unsafe {_ZNK10QLCDNumber10digitCountEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn FreeQLCDNumber<T: QLCDNumber_FreeQLCDNumber>(&mut self, value: T) -> i32 {
    value.FreeQLCDNumber(self);
    return 1;
  }
}

pub trait QLCDNumber_FreeQLCDNumber {
  fn FreeQLCDNumber(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::FreeQLCDNumber();
impl<'a> /*trait*/ QLCDNumber_FreeQLCDNumber for () {
  fn FreeQLCDNumber(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberD0Ev()};
    unsafe {_ZN10QLCDNumberD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn checkOverflow<T: QLCDNumber_checkOverflow>(&mut self, value: T) -> i32 {
    value.checkOverflow(self);
    return 1;
  }
}

pub trait QLCDNumber_checkOverflow {
  fn checkOverflow(self, this: &mut QLCDNumber) -> i32;
}

// proto: bool QLCDNumber::checkOverflow(int num);
impl<'a> /*trait*/ QLCDNumber_checkOverflow for (i32) {
  fn checkOverflow(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber13checkOverflowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QLCDNumber13checkOverflowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn setDecMode<T: QLCDNumber_setDecMode>(&mut self, value: T) -> i32 {
    value.setDecMode(self);
    return 1;
  }
}

pub trait QLCDNumber_setDecMode {
  fn setDecMode(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::setDecMode();
impl<'a> /*trait*/ QLCDNumber_setDecMode for () {
  fn setDecMode(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setDecModeEv()};
    unsafe {_ZN10QLCDNumber10setDecModeEv()};
    return 1;
  }
}

// proto: void QLCDNumber::NewQLCDNumber(uint numDigits, QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_NewQLCDNumber for (u32, &'a mut QWidget) {
  fn NewQLCDNumber(self) -> QLCDNumber {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1EjP7QWidget()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QLCDNumberC1EjP7QWidget(qthis, arg0, arg1)};
    let rsthis = QLCDNumber{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: bool QLCDNumber::checkOverflow(double num);
impl<'a> /*trait*/ QLCDNumber_checkOverflow for (f64) {
  fn checkOverflow(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber13checkOverflowEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK10QLCDNumber13checkOverflowEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn sizeHint<T: QLCDNumber_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QLCDNumber_sizeHint {
  fn sizeHint(self, this: &mut QLCDNumber) -> i32;
}

// proto: QSize QLCDNumber::sizeHint();
impl<'a> /*trait*/ QLCDNumber_sizeHint for () {
  fn sizeHint(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber8sizeHintEv()};
    unsafe {_ZNK10QLCDNumber8sizeHintEv()};
    return 1;
  }
}

// proto: void QLCDNumber::display(const QString & str);
impl<'a> /*trait*/ QLCDNumber_display for (&'a  QString) {
  fn display(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QLCDNumber7displayERK7QString(arg0)};
    return 1;
  }
}

// proto: void QLCDNumber::NewQLCDNumber(QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_NewQLCDNumber for (&'a mut QWidget) {
  fn NewQLCDNumber(self) -> QLCDNumber {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QLCDNumberC1EP7QWidget(qthis, arg0)};
    let rsthis = QLCDNumber{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn value<T: QLCDNumber_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QLCDNumber_value {
  fn value(self, this: &mut QLCDNumber) -> i32;
}

// proto: double QLCDNumber::value();
impl<'a> /*trait*/ QLCDNumber_value for () {
  fn value(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber5valueEv()};
    unsafe {_ZNK10QLCDNumber5valueEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn setBinMode<T: QLCDNumber_setBinMode>(&mut self, value: T) -> i32 {
    value.setBinMode(self);
    return 1;
  }
}

pub trait QLCDNumber_setBinMode {
  fn setBinMode(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::setBinMode();
impl<'a> /*trait*/ QLCDNumber_setBinMode for () {
  fn setBinMode(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setBinModeEv()};
    unsafe {_ZN10QLCDNumber10setBinModeEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn intValue<T: QLCDNumber_intValue>(&mut self, value: T) -> i32 {
    value.intValue(self);
    return 1;
  }
}

pub trait QLCDNumber_intValue {
  fn intValue(self, this: &mut QLCDNumber) -> i32;
}

// proto: int QLCDNumber::intValue();
impl<'a> /*trait*/ QLCDNumber_intValue for () {
  fn intValue(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber8intValueEv()};
    unsafe {_ZNK10QLCDNumber8intValueEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn setDigitCount<T: QLCDNumber_setDigitCount>(&mut self, value: T) -> i32 {
    value.setDigitCount(self);
    return 1;
  }
}

pub trait QLCDNumber_setDigitCount {
  fn setDigitCount(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::setDigitCount(int nDigits);
impl<'a> /*trait*/ QLCDNumber_setDigitCount for (i32) {
  fn setDigitCount(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber13setDigitCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QLCDNumber13setDigitCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn setSmallDecimalPoint<T: QLCDNumber_setSmallDecimalPoint>(&mut self, value: T) -> i32 {
    value.setSmallDecimalPoint(self);
    return 1;
  }
}

pub trait QLCDNumber_setSmallDecimalPoint {
  fn setSmallDecimalPoint(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::setSmallDecimalPoint(bool );
impl<'a> /*trait*/ QLCDNumber_setSmallDecimalPoint for (i8) {
  fn setSmallDecimalPoint(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber20setSmallDecimalPointEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QLCDNumber20setSmallDecimalPointEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn smallDecimalPoint<T: QLCDNumber_smallDecimalPoint>(&mut self, value: T) -> i32 {
    value.smallDecimalPoint(self);
    return 1;
  }
}

pub trait QLCDNumber_smallDecimalPoint {
  fn smallDecimalPoint(self, this: &mut QLCDNumber) -> i32;
}

// proto: bool QLCDNumber::smallDecimalPoint();
impl<'a> /*trait*/ QLCDNumber_smallDecimalPoint for () {
  fn smallDecimalPoint(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber17smallDecimalPointEv()};
    unsafe {_ZNK10QLCDNumber17smallDecimalPointEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn setOctMode<T: QLCDNumber_setOctMode>(&mut self, value: T) -> i32 {
    value.setOctMode(self);
    return 1;
  }
}

pub trait QLCDNumber_setOctMode {
  fn setOctMode(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::setOctMode();
impl<'a> /*trait*/ QLCDNumber_setOctMode for () {
  fn setOctMode(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setOctModeEv()};
    unsafe {_ZN10QLCDNumber10setOctModeEv()};
    return 1;
  }
}

impl /*struct*/ QLCDNumber {
  pub fn overflow<T: QLCDNumber_overflow>(&mut self, value: T) -> i32 {
    value.overflow(self);
    return 1;
  }
}

pub trait QLCDNumber_overflow {
  fn overflow(self, this: &mut QLCDNumber) -> i32;
}

// proto: void QLCDNumber::overflow();
impl<'a> /*trait*/ QLCDNumber_overflow for () {
  fn overflow(self, this: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber8overflowEv()};
    unsafe {_ZN10QLCDNumber8overflowEv()};
    return 1;
  }
}

