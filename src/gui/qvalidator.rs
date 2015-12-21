// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qvalidator.h
// dst-file: /src/gui/qvalidator.rs
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
use super::super::core::qregularexpression::QRegularExpression; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qlocale::QLocale; // 771
use super::super::core::qregexp::QRegExp; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
  fn _ZNK27QRegularExpressionValidator17regularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionValidator::~QRegularExpressionValidator();
  fn _ZN27QRegularExpressionValidatorD0Ev(qthis: *mut c_void);
  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
  fn _ZN27QRegularExpressionValidatorC1ERK18QRegularExpressionP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
  fn _ZNK27QRegularExpressionValidator10metaObjectEv(qthis: *mut c_void);
  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpressionValidator & );
  fn _ZN27QRegularExpressionValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(QObject * parent);
  fn _ZN27QRegularExpressionValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
  fn _ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
  fn _ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QDoubleValidator::decimals();
  fn _ZNK16QDoubleValidator8decimalsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDoubleValidator::decimalsChanged(int decimals);
  fn _ZN16QDoubleValidator15decimalsChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QDoubleValidator::~QDoubleValidator();
  fn _ZN16QDoubleValidatorD0Ev(qthis: *mut c_void);
  // proto:  double QDoubleValidator::top();
  fn _ZNK16QDoubleValidator3topEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleValidator::bottomChanged(double bottom);
  fn _ZN16QDoubleValidator13bottomChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  double QDoubleValidator::bottom();
  fn _ZNK16QDoubleValidator6bottomEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleValidator::setDecimals(int );
  fn _ZN16QDoubleValidator11setDecimalsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QDoubleValidator::QDoubleValidator(const QDoubleValidator & );
  fn _ZN16QDoubleValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDoubleValidator::setBottom(double );
  fn _ZN16QDoubleValidator9setBottomEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
  fn _ZN16QDoubleValidator8setRangeEddi(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_int);
  // proto:  void QDoubleValidator::QDoubleValidator(QObject * parent);
  fn _ZN16QDoubleValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDoubleValidator::QDoubleValidator(double bottom, double top, int decimals, QObject * parent);
  fn _ZN16QDoubleValidatorC1EddiP7QObject(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_int, arg3: *mut c_void);
  // proto:  void QDoubleValidator::topChanged(double top);
  fn _ZN16QDoubleValidator10topChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  const QMetaObject * QDoubleValidator::metaObject();
  fn _ZNK16QDoubleValidator10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDoubleValidator::setTop(double );
  fn _ZN16QDoubleValidator6setTopEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QIntValidator::QIntValidator(QObject * parent);
  fn _ZN13QIntValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QIntValidator::setBottom(int );
  fn _ZN13QIntValidator9setBottomEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QIntValidator::setRange(int bottom, int top);
  fn _ZN13QIntValidator8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  const QMetaObject * QIntValidator::metaObject();
  fn _ZNK13QIntValidator10metaObjectEv(qthis: *mut c_void);
  // proto:  void QIntValidator::QIntValidator(const QIntValidator & );
  fn _ZN13QIntValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QIntValidator::top();
  fn _ZNK13QIntValidator3topEv(qthis: *mut c_void) -> c_int;
  // proto:  void QIntValidator::fixup(QString & input);
  fn _ZNK13QIntValidator5fixupER7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QIntValidator::~QIntValidator();
  fn _ZN13QIntValidatorD0Ev(qthis: *mut c_void);
  // proto:  void QIntValidator::bottomChanged(int bottom);
  fn _ZN13QIntValidator13bottomChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QIntValidator::topChanged(int top);
  fn _ZN13QIntValidator10topChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QIntValidator::setTop(int );
  fn _ZN13QIntValidator6setTopEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QIntValidator::bottom();
  fn _ZNK13QIntValidator6bottomEv(qthis: *mut c_void) -> c_int;
  // proto:  void QIntValidator::QIntValidator(int bottom, int top, QObject * parent);
  fn _ZN13QIntValidatorC1EiiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  const QMetaObject * QValidator::metaObject();
  fn _ZNK10QValidator10metaObjectEv(qthis: *mut c_void);
  // proto:  void QValidator::QValidator(const QValidator & );
  fn _ZN10QValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QValidator::setLocale(const QLocale & locale);
  fn _ZN10QValidator9setLocaleERK7QLocale(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QValidator::fixup(QString & );
  fn _ZNK10QValidator5fixupER7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QValidator::~QValidator();
  fn _ZN10QValidatorD0Ev(qthis: *mut c_void);
  // proto:  void QValidator::QValidator(QObject * parent);
  fn _ZN10QValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QLocale QValidator::locale();
  fn _ZNK10QValidator6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QValidator::changed();
  fn _ZN10QValidator7changedEv(qthis: *mut c_void);
  // proto:  void QRegExpValidator::~QRegExpValidator();
  fn _ZN16QRegExpValidatorD0Ev(qthis: *mut c_void);
  // proto:  const QRegExp & QRegExpValidator::regExp();
  fn _ZNK16QRegExpValidator6regExpEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QRegExpValidator::metaObject();
  fn _ZNK16QRegExpValidator10metaObjectEv(qthis: *mut c_void);
  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExp & rx, QObject * parent);
  fn _ZN16QRegExpValidatorC1ERK7QRegExpP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QRegExpValidator::setRegExp(const QRegExp & rx);
  fn _ZN16QRegExpValidator9setRegExpERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExpValidator & );
  fn _ZN16QRegExpValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegExpValidator::QRegExpValidator(QObject * parent);
  fn _ZN16QRegExpValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegExpValidator::regExpChanged(const QRegExp & regExp);
  fn _ZN16QRegExpValidator13regExpChangedERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRegularExpressionValidator)=1
pub struct QRegularExpressionValidator {
  pub qclsinst: *mut c_void,
}

// class sizeof(QDoubleValidator)=1
pub struct QDoubleValidator {
  pub qclsinst: *mut c_void,
}

// class sizeof(QIntValidator)=1
pub struct QIntValidator {
  pub qclsinst: *mut c_void,
}

// class sizeof(QValidator)=1
pub struct QValidator {
  pub qclsinst: *mut c_void,
}

// class sizeof(QRegExpValidator)=1
pub struct QRegExpValidator {
  pub qclsinst: *mut c_void,
}

  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpression<RetType, T: QRegularExpressionValidator_regularExpression<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_regularExpression<RetType> {
  fn regularExpression(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: &mut QRegularExpressionValidator) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QRegularExpressionValidator17regularExpressionEv()};
    let mut ret = unsafe {_ZNK27QRegularExpressionValidator17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::~QRegularExpressionValidator();
impl /*struct*/ QRegularExpressionValidator {
  pub fn FreeQRegularExpressionValidator<RetType, T: QRegularExpressionValidator_FreeQRegularExpressionValidator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQRegularExpressionValidator(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_FreeQRegularExpressionValidator<RetType> {
  fn FreeQRegularExpressionValidator(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

  // proto:  void QRegularExpressionValidator::~QRegularExpressionValidator();
impl<'a> /*trait*/ QRegularExpressionValidator_FreeQRegularExpressionValidator<()> for () {
  fn FreeQRegularExpressionValidator(self , rsthis: &mut QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorD0Ev()};
     unsafe {_ZN27QRegularExpressionValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
impl /*struct*/ QRegularExpressionValidator {
  pub fn NewQRegularExpressionValidator<T: QRegularExpressionValidator_NewQRegularExpressionValidator>(value: T) -> QRegularExpressionValidator {
    let rsthis = value.NewQRegularExpressionValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionValidator_NewQRegularExpressionValidator {
  fn NewQRegularExpressionValidator(self) -> QRegularExpressionValidator;
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (QRegularExpression, QObject) {
  fn NewQRegularExpressionValidator(self) -> QRegularExpressionValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorC1ERK18QRegularExpressionP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN27QRegularExpressionValidatorC1ERK18QRegularExpressionP7QObject(qthis, arg0, arg1)};
    let rsthis = QRegularExpressionValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
impl /*struct*/ QRegularExpressionValidator {
  pub fn metaObject<RetType, T: QRegularExpressionValidator_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
impl<'a> /*trait*/ QRegularExpressionValidator_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QRegularExpressionValidator10metaObjectEv()};
     unsafe {_ZNK27QRegularExpressionValidator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpressionValidator & );
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (QRegularExpressionValidator) {
  fn NewQRegularExpressionValidator(self) -> QRegularExpressionValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QRegularExpressionValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpressionValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(QObject * parent);
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (QObject) {
  fn NewQRegularExpressionValidator(self) -> QRegularExpressionValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QRegularExpressionValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QRegularExpressionValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
impl /*struct*/ QRegularExpressionValidator {
  pub fn setRegularExpression<RetType, T: QRegularExpressionValidator_setRegularExpression<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRegularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_setRegularExpression<RetType> {
  fn setRegularExpression(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_setRegularExpression<()> for (QRegularExpression) {
  fn setRegularExpression(self , rsthis: &mut QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpressionChanged<RetType, T: QRegularExpressionValidator_regularExpressionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.regularExpressionChanged(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_regularExpressionChanged<RetType> {
  fn regularExpressionChanged(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

  // proto:  void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpressionChanged<()> for (QRegularExpression) {
  fn regularExpressionChanged(self , rsthis: &mut QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDoubleValidator::decimals();
impl /*struct*/ QDoubleValidator {
  pub fn decimals<RetType, T: QDoubleValidator_decimals<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.decimals(self);
    // return 1;
  }
}

pub trait QDoubleValidator_decimals<RetType> {
  fn decimals(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  int QDoubleValidator::decimals();
impl<'a> /*trait*/ QDoubleValidator_decimals<i32> for () {
  fn decimals(self , rsthis: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator8decimalsEv()};
    let mut ret = unsafe {_ZNK16QDoubleValidator8decimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::decimalsChanged(int decimals);
impl /*struct*/ QDoubleValidator {
  pub fn decimalsChanged<RetType, T: QDoubleValidator_decimalsChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.decimalsChanged(self);
    // return 1;
  }
}

pub trait QDoubleValidator_decimalsChanged<RetType> {
  fn decimalsChanged(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::decimalsChanged(int decimals);
impl<'a> /*trait*/ QDoubleValidator_decimalsChanged<()> for (i32) {
  fn decimalsChanged(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator15decimalsChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QDoubleValidator15decimalsChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::~QDoubleValidator();
impl /*struct*/ QDoubleValidator {
  pub fn FreeQDoubleValidator<RetType, T: QDoubleValidator_FreeQDoubleValidator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDoubleValidator(self);
    // return 1;
  }
}

pub trait QDoubleValidator_FreeQDoubleValidator<RetType> {
  fn FreeQDoubleValidator(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::~QDoubleValidator();
impl<'a> /*trait*/ QDoubleValidator_FreeQDoubleValidator<()> for () {
  fn FreeQDoubleValidator(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorD0Ev()};
     unsafe {_ZN16QDoubleValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  double QDoubleValidator::top();
impl /*struct*/ QDoubleValidator {
  pub fn top<RetType, T: QDoubleValidator_top<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QDoubleValidator_top<RetType> {
  fn top(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  double QDoubleValidator::top();
impl<'a> /*trait*/ QDoubleValidator_top<f64> for () {
  fn top(self , rsthis: &mut QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator3topEv()};
    let mut ret = unsafe {_ZNK16QDoubleValidator3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::bottomChanged(double bottom);
impl /*struct*/ QDoubleValidator {
  pub fn bottomChanged<RetType, T: QDoubleValidator_bottomChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomChanged(self);
    // return 1;
  }
}

pub trait QDoubleValidator_bottomChanged<RetType> {
  fn bottomChanged(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::bottomChanged(double bottom);
impl<'a> /*trait*/ QDoubleValidator_bottomChanged<()> for (f64) {
  fn bottomChanged(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator13bottomChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator13bottomChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QDoubleValidator::bottom();
impl /*struct*/ QDoubleValidator {
  pub fn bottom<RetType, T: QDoubleValidator_bottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QDoubleValidator_bottom<RetType> {
  fn bottom(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  double QDoubleValidator::bottom();
impl<'a> /*trait*/ QDoubleValidator_bottom<f64> for () {
  fn bottom(self , rsthis: &mut QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator6bottomEv()};
    let mut ret = unsafe {_ZNK16QDoubleValidator6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setDecimals(int );
impl /*struct*/ QDoubleValidator {
  pub fn setDecimals<RetType, T: QDoubleValidator_setDecimals<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDecimals(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setDecimals<RetType> {
  fn setDecimals(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setDecimals(int );
impl<'a> /*trait*/ QDoubleValidator_setDecimals<()> for (i32) {
  fn setDecimals(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator11setDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QDoubleValidator11setDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::QDoubleValidator(const QDoubleValidator & );
impl /*struct*/ QDoubleValidator {
  pub fn NewQDoubleValidator<T: QDoubleValidator_NewQDoubleValidator>(value: T) -> QDoubleValidator {
    let rsthis = value.NewQDoubleValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleValidator_NewQDoubleValidator {
  fn NewQDoubleValidator(self) -> QDoubleValidator;
}

  // proto:  void QDoubleValidator::QDoubleValidator(const QDoubleValidator & );
impl<'a> /*trait*/ QDoubleValidator_NewQDoubleValidator for (QDoubleValidator) {
  fn NewQDoubleValidator(self) -> QDoubleValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDoubleValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setBottom(double );
impl /*struct*/ QDoubleValidator {
  pub fn setBottom<RetType, T: QDoubleValidator_setBottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setBottom<RetType> {
  fn setBottom(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setBottom(double );
impl<'a> /*trait*/ QDoubleValidator_setBottom<()> for (f64) {
  fn setBottom(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
impl /*struct*/ QDoubleValidator {
  pub fn setRange<RetType, T: QDoubleValidator_setRange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setRange<RetType> {
  fn setRange(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
impl<'a> /*trait*/ QDoubleValidator_setRange<()> for (f64, f64, i32) {
  fn setRange(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator8setRangeEddi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QDoubleValidator8setRangeEddi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::QDoubleValidator(QObject * parent);
impl<'a> /*trait*/ QDoubleValidator_NewQDoubleValidator for (QObject) {
  fn NewQDoubleValidator(self) -> QDoubleValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDoubleValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::QDoubleValidator(double bottom, double top, int decimals, QObject * parent);
impl<'a> /*trait*/ QDoubleValidator_NewQDoubleValidator for (f64, f64, i32, QObject) {
  fn NewQDoubleValidator(self) -> QDoubleValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC1EddiP7QObject()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN16QDoubleValidatorC1EddiP7QObject(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::topChanged(double top);
impl /*struct*/ QDoubleValidator {
  pub fn topChanged<RetType, T: QDoubleValidator_topChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topChanged(self);
    // return 1;
  }
}

pub trait QDoubleValidator_topChanged<RetType> {
  fn topChanged(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::topChanged(double top);
impl<'a> /*trait*/ QDoubleValidator_topChanged<()> for (f64) {
  fn topChanged(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator10topChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator10topChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDoubleValidator::metaObject();
impl /*struct*/ QDoubleValidator {
  pub fn metaObject<RetType, T: QDoubleValidator_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDoubleValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  const QMetaObject * QDoubleValidator::metaObject();
impl<'a> /*trait*/ QDoubleValidator_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator10metaObjectEv()};
     unsafe {_ZNK16QDoubleValidator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setTop(double );
impl /*struct*/ QDoubleValidator {
  pub fn setTop<RetType, T: QDoubleValidator_setTop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setTop<RetType> {
  fn setTop(self , rsthis: &mut QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setTop(double );
impl<'a> /*trait*/ QDoubleValidator_setTop<()> for (f64) {
  fn setTop(self , rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::QIntValidator(QObject * parent);
impl /*struct*/ QIntValidator {
  pub fn NewQIntValidator<T: QIntValidator_NewQIntValidator>(value: T) -> QIntValidator {
    let rsthis = value.NewQIntValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QIntValidator_NewQIntValidator {
  fn NewQIntValidator(self) -> QIntValidator;
}

  // proto:  void QIntValidator::QIntValidator(QObject * parent);
impl<'a> /*trait*/ QIntValidator_NewQIntValidator for (QObject) {
  fn NewQIntValidator(self) -> QIntValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QIntValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QIntValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QIntValidator::setBottom(int );
impl /*struct*/ QIntValidator {
  pub fn setBottom<RetType, T: QIntValidator_setBottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QIntValidator_setBottom<RetType> {
  fn setBottom(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::setBottom(int );
impl<'a> /*trait*/ QIntValidator_setBottom<()> for (i32) {
  fn setBottom(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator9setBottomEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QIntValidator9setBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::setRange(int bottom, int top);
impl /*struct*/ QIntValidator {
  pub fn setRange<RetType, T: QIntValidator_setRange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QIntValidator_setRange<RetType> {
  fn setRange(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::setRange(int bottom, int top);
impl<'a> /*trait*/ QIntValidator_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QIntValidator8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QIntValidator::metaObject();
impl /*struct*/ QIntValidator {
  pub fn metaObject<RetType, T: QIntValidator_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIntValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  const QMetaObject * QIntValidator::metaObject();
impl<'a> /*trait*/ QIntValidator_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator10metaObjectEv()};
     unsafe {_ZNK13QIntValidator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIntValidator::QIntValidator(const QIntValidator & );
impl<'a> /*trait*/ QIntValidator_NewQIntValidator for (QIntValidator) {
  fn NewQIntValidator(self) -> QIntValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QIntValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QIntValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QIntValidator::top();
impl /*struct*/ QIntValidator {
  pub fn top<RetType, T: QIntValidator_top<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QIntValidator_top<RetType> {
  fn top(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  int QIntValidator::top();
impl<'a> /*trait*/ QIntValidator_top<i32> for () {
  fn top(self , rsthis: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator3topEv()};
    let mut ret = unsafe {_ZNK13QIntValidator3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QIntValidator::fixup(QString & input);
impl /*struct*/ QIntValidator {
  pub fn fixup<RetType, T: QIntValidator_fixup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QIntValidator_fixup<RetType> {
  fn fixup(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::fixup(QString & input);
impl<'a> /*trait*/ QIntValidator_fixup<()> for (QString) {
  fn fixup(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QIntValidator5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::~QIntValidator();
impl /*struct*/ QIntValidator {
  pub fn FreeQIntValidator<RetType, T: QIntValidator_FreeQIntValidator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQIntValidator(self);
    // return 1;
  }
}

pub trait QIntValidator_FreeQIntValidator<RetType> {
  fn FreeQIntValidator(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::~QIntValidator();
impl<'a> /*trait*/ QIntValidator_FreeQIntValidator<()> for () {
  fn FreeQIntValidator(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorD0Ev()};
     unsafe {_ZN13QIntValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIntValidator::bottomChanged(int bottom);
impl /*struct*/ QIntValidator {
  pub fn bottomChanged<RetType, T: QIntValidator_bottomChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomChanged(self);
    // return 1;
  }
}

pub trait QIntValidator_bottomChanged<RetType> {
  fn bottomChanged(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::bottomChanged(int bottom);
impl<'a> /*trait*/ QIntValidator_bottomChanged<()> for (i32) {
  fn bottomChanged(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator13bottomChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QIntValidator13bottomChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::topChanged(int top);
impl /*struct*/ QIntValidator {
  pub fn topChanged<RetType, T: QIntValidator_topChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topChanged(self);
    // return 1;
  }
}

pub trait QIntValidator_topChanged<RetType> {
  fn topChanged(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::topChanged(int top);
impl<'a> /*trait*/ QIntValidator_topChanged<()> for (i32) {
  fn topChanged(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator10topChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QIntValidator10topChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::setTop(int );
impl /*struct*/ QIntValidator {
  pub fn setTop<RetType, T: QIntValidator_setTop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QIntValidator_setTop<RetType> {
  fn setTop(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::setTop(int );
impl<'a> /*trait*/ QIntValidator_setTop<()> for (i32) {
  fn setTop(self , rsthis: &mut QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator6setTopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QIntValidator6setTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QIntValidator::bottom();
impl /*struct*/ QIntValidator {
  pub fn bottom<RetType, T: QIntValidator_bottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QIntValidator_bottom<RetType> {
  fn bottom(self , rsthis: &mut QIntValidator) -> RetType;
}

  // proto:  int QIntValidator::bottom();
impl<'a> /*trait*/ QIntValidator_bottom<i32> for () {
  fn bottom(self , rsthis: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator6bottomEv()};
    let mut ret = unsafe {_ZNK13QIntValidator6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QIntValidator::QIntValidator(int bottom, int top, QObject * parent);
impl<'a> /*trait*/ QIntValidator_NewQIntValidator for (i32, i32, QObject) {
  fn NewQIntValidator(self) -> QIntValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC1EiiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QIntValidatorC1EiiP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QIntValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QValidator::metaObject();
impl /*struct*/ QValidator {
  pub fn metaObject<RetType, T: QValidator_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QValidator) -> RetType;
}

  // proto:  const QMetaObject * QValidator::metaObject();
impl<'a> /*trait*/ QValidator_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator10metaObjectEv()};
     unsafe {_ZNK10QValidator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QValidator::QValidator(const QValidator & );
impl /*struct*/ QValidator {
  pub fn NewQValidator<T: QValidator_NewQValidator>(value: T) -> QValidator {
    let rsthis = value.NewQValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QValidator_NewQValidator {
  fn NewQValidator(self) -> QValidator;
}

  // proto:  void QValidator::QValidator(const QValidator & );
impl<'a> /*trait*/ QValidator_NewQValidator for (QValidator) {
  fn NewQValidator(self) -> QValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QValidator::setLocale(const QLocale & locale);
impl /*struct*/ QValidator {
  pub fn setLocale<RetType, T: QValidator_setLocale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLocale(self);
    // return 1;
  }
}

pub trait QValidator_setLocale<RetType> {
  fn setLocale(self , rsthis: &mut QValidator) -> RetType;
}

  // proto:  void QValidator::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QValidator_setLocale<()> for (QLocale) {
  fn setLocale(self , rsthis: &mut QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidator9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QValidator9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QValidator::fixup(QString & );
impl /*struct*/ QValidator {
  pub fn fixup<RetType, T: QValidator_fixup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QValidator_fixup<RetType> {
  fn fixup(self , rsthis: &mut QValidator) -> RetType;
}

  // proto:  void QValidator::fixup(QString & );
impl<'a> /*trait*/ QValidator_fixup<()> for (QString) {
  fn fixup(self , rsthis: &mut QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK10QValidator5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QValidator::~QValidator();
impl /*struct*/ QValidator {
  pub fn FreeQValidator<RetType, T: QValidator_FreeQValidator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQValidator(self);
    // return 1;
  }
}

pub trait QValidator_FreeQValidator<RetType> {
  fn FreeQValidator(self , rsthis: &mut QValidator) -> RetType;
}

  // proto:  void QValidator::~QValidator();
impl<'a> /*trait*/ QValidator_FreeQValidator<()> for () {
  fn FreeQValidator(self , rsthis: &mut QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorD0Ev()};
     unsafe {_ZN10QValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QValidator::QValidator(QObject * parent);
impl<'a> /*trait*/ QValidator_NewQValidator for (QObject) {
  fn NewQValidator(self) -> QValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QLocale QValidator::locale();
impl /*struct*/ QValidator {
  pub fn locale<RetType, T: QValidator_locale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QValidator_locale<RetType> {
  fn locale(self , rsthis: &mut QValidator) -> RetType;
}

  // proto:  QLocale QValidator::locale();
impl<'a> /*trait*/ QValidator_locale<QLocale> for () {
  fn locale(self , rsthis: &mut QValidator) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator6localeEv()};
    let mut ret = unsafe {_ZNK10QValidator6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QValidator::changed();
impl /*struct*/ QValidator {
  pub fn changed<RetType, T: QValidator_changed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.changed(self);
    // return 1;
  }
}

pub trait QValidator_changed<RetType> {
  fn changed(self , rsthis: &mut QValidator) -> RetType;
}

  // proto:  void QValidator::changed();
impl<'a> /*trait*/ QValidator_changed<()> for () {
  fn changed(self , rsthis: &mut QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidator7changedEv()};
     unsafe {_ZN10QValidator7changedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegExpValidator::~QRegExpValidator();
impl /*struct*/ QRegExpValidator {
  pub fn FreeQRegExpValidator<RetType, T: QRegExpValidator_FreeQRegExpValidator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQRegExpValidator(self);
    // return 1;
  }
}

pub trait QRegExpValidator_FreeQRegExpValidator<RetType> {
  fn FreeQRegExpValidator(self , rsthis: &mut QRegExpValidator) -> RetType;
}

  // proto:  void QRegExpValidator::~QRegExpValidator();
impl<'a> /*trait*/ QRegExpValidator_FreeQRegExpValidator<()> for () {
  fn FreeQRegExpValidator(self , rsthis: &mut QRegExpValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorD0Ev()};
     unsafe {_ZN16QRegExpValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QRegExp & QRegExpValidator::regExp();
impl /*struct*/ QRegExpValidator {
  pub fn regExp<RetType, T: QRegExpValidator_regExp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.regExp(self);
    // return 1;
  }
}

pub trait QRegExpValidator_regExp<RetType> {
  fn regExp(self , rsthis: &mut QRegExpValidator) -> RetType;
}

  // proto:  const QRegExp & QRegExpValidator::regExp();
impl<'a> /*trait*/ QRegExpValidator_regExp<QRegExp> for () {
  fn regExp(self , rsthis: &mut QRegExpValidator) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QRegExpValidator6regExpEv()};
    let mut ret = unsafe {_ZNK16QRegExpValidator6regExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRegExpValidator::metaObject();
impl /*struct*/ QRegExpValidator {
  pub fn metaObject<RetType, T: QRegExpValidator_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRegExpValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QRegExpValidator) -> RetType;
}

  // proto:  const QMetaObject * QRegExpValidator::metaObject();
impl<'a> /*trait*/ QRegExpValidator_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QRegExpValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QRegExpValidator10metaObjectEv()};
     unsafe {_ZNK16QRegExpValidator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExp & rx, QObject * parent);
impl /*struct*/ QRegExpValidator {
  pub fn NewQRegExpValidator<T: QRegExpValidator_NewQRegExpValidator>(value: T) -> QRegExpValidator {
    let rsthis = value.NewQRegExpValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExpValidator_NewQRegExpValidator {
  fn NewQRegExpValidator(self) -> QRegExpValidator;
}

  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExp & rx, QObject * parent);
impl<'a> /*trait*/ QRegExpValidator_NewQRegExpValidator for (QRegExp, QObject) {
  fn NewQRegExpValidator(self) -> QRegExpValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC1ERK7QRegExpP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QRegExpValidatorC1ERK7QRegExpP7QObject(qthis, arg0, arg1)};
    let rsthis = QRegExpValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegExpValidator::setRegExp(const QRegExp & rx);
impl /*struct*/ QRegExpValidator {
  pub fn setRegExp<RetType, T: QRegExpValidator_setRegExp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRegExp(self);
    // return 1;
  }
}

pub trait QRegExpValidator_setRegExp<RetType> {
  fn setRegExp(self , rsthis: &mut QRegExpValidator) -> RetType;
}

  // proto:  void QRegExpValidator::setRegExp(const QRegExp & rx);
impl<'a> /*trait*/ QRegExpValidator_setRegExp<()> for (QRegExp) {
  fn setRegExp(self , rsthis: &mut QRegExpValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidator9setRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QRegExpValidator9setRegExpERK7QRegExp(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExpValidator & );
impl<'a> /*trait*/ QRegExpValidator_NewQRegExpValidator for (QRegExpValidator) {
  fn NewQRegExpValidator(self) -> QRegExpValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QRegExpValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QRegExpValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegExpValidator::QRegExpValidator(QObject * parent);
impl<'a> /*trait*/ QRegExpValidator_NewQRegExpValidator for (QObject) {
  fn NewQRegExpValidator(self) -> QRegExpValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QRegExpValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QRegExpValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegExpValidator::regExpChanged(const QRegExp & regExp);
impl /*struct*/ QRegExpValidator {
  pub fn regExpChanged<RetType, T: QRegExpValidator_regExpChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.regExpChanged(self);
    // return 1;
  }
}

pub trait QRegExpValidator_regExpChanged<RetType> {
  fn regExpChanged(self , rsthis: &mut QRegExpValidator) -> RetType;
}

  // proto:  void QRegExpValidator::regExpChanged(const QRegExp & regExp);
impl<'a> /*trait*/ QRegExpValidator_regExpChanged<()> for (QRegExp) {
  fn regExpChanged(self , rsthis: &mut QRegExpValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidator13regExpChangedERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QRegExpValidator13regExpChangedERK7QRegExp(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

