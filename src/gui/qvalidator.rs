// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
// use super::qvalidator::QValidator; // 773
use std::ops::Deref;
use super::super::core::qregularexpression::QRegularExpression; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qlocale::QLocale; // 771
use super::super::core::qregexp::QRegExp; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRegularExpressionValidator_Class_Size() -> c_int;
  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
  fn C_ZNK27QRegularExpressionValidator17regularExpressionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpressionValidator::~QRegularExpressionValidator();
  fn C_ZN27QRegularExpressionValidatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
  fn C_ZN27QRegularExpressionValidatorC2ERK18QRegularExpressionP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
  fn C_ZNK27QRegularExpressionValidator10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(QObject * parent);
  fn C_ZN27QRegularExpressionValidatorC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
  fn C_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QDoubleValidator_Class_Size() -> c_int;
  // proto:  int QDoubleValidator::decimals();
  fn C_ZNK16QDoubleValidator8decimalsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDoubleValidator::~QDoubleValidator();
  fn C_ZN16QDoubleValidatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  double QDoubleValidator::top();
  fn C_ZNK16QDoubleValidator3topEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  double QDoubleValidator::bottom();
  fn C_ZNK16QDoubleValidator6bottomEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QDoubleValidator::setDecimals(int );
  fn C_ZN16QDoubleValidator11setDecimalsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDoubleValidator::setBottom(double );
  fn C_ZN16QDoubleValidator9setBottomEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
  fn C_ZN16QDoubleValidator8setRangeEddi(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_int);
  // proto:  void QDoubleValidator::QDoubleValidator(QObject * parent);
  fn C_ZN16QDoubleValidatorC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QDoubleValidator::QDoubleValidator(double bottom, double top, int decimals, QObject * parent);
  fn C_ZN16QDoubleValidatorC2EddiP7QObject(arg0: c_double, arg1: c_double, arg2: c_int, arg3: *mut c_void) -> u64;
  // proto:  const QMetaObject * QDoubleValidator::metaObject();
  fn C_ZNK16QDoubleValidator10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDoubleValidator::setTop(double );
  fn C_ZN16QDoubleValidator6setTopEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  fn QIntValidator_Class_Size() -> c_int;
  // proto:  void QIntValidator::QIntValidator(QObject * parent);
  fn C_ZN13QIntValidatorC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QIntValidator::setBottom(int );
  fn C_ZN13QIntValidator9setBottomEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QIntValidator::setRange(int bottom, int top);
  fn C_ZN13QIntValidator8setRangeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  const QMetaObject * QIntValidator::metaObject();
  fn C_ZNK13QIntValidator10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QIntValidator::top();
  fn C_ZNK13QIntValidator3topEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QIntValidator::fixup(QString & input);
  fn C_ZNK13QIntValidator5fixupER7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QIntValidator::~QIntValidator();
  fn C_ZN13QIntValidatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QIntValidator::setTop(int );
  fn C_ZN13QIntValidator6setTopEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QIntValidator::bottom();
  fn C_ZNK13QIntValidator6bottomEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QIntValidator::QIntValidator(int bottom, int top, QObject * parent);
  fn C_ZN13QIntValidatorC2EiiP7QObject(arg0: c_int, arg1: c_int, arg2: *mut c_void) -> u64;
  fn QValidator_Class_Size() -> c_int;
  // proto:  const QMetaObject * QValidator::metaObject();
  fn C_ZNK10QValidator10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QValidator::setLocale(const QLocale & locale);
  fn C_ZN10QValidator9setLocaleERK7QLocale(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QValidator::fixup(QString & );
  fn C_ZNK10QValidator5fixupER7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QValidator::~QValidator();
  fn C_ZN10QValidatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QValidator::QValidator(QObject * parent);
  fn C_ZN10QValidatorC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QLocale QValidator::locale();
  fn C_ZNK10QValidator6localeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QRegExpValidator_Class_Size() -> c_int;
  // proto:  void QRegExpValidator::~QRegExpValidator();
  fn C_ZN16QRegExpValidatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QRegExp & QRegExpValidator::regExp();
  fn C_ZNK16QRegExpValidator6regExpEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QRegExpValidator::metaObject();
  fn C_ZNK16QRegExpValidator10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExp & rx, QObject * parent);
  fn C_ZN16QRegExpValidatorC2ERK7QRegExpP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QRegExpValidator::setRegExp(const QRegExp & rx);
  fn C_ZN16QRegExpValidator9setRegExpERK7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRegExpValidator::QRegExpValidator(QObject * parent);
  fn C_ZN16QRegExpValidatorC2EP7QObject(arg0: *mut c_void) -> u64;
  fn QRegularExpressionValidator_SlotProxy_connect__ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator13bottomChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator15decimalsChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator10topChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QIntValidator_SlotProxy_connect__ZN13QIntValidator10topChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QIntValidator_SlotProxy_connect__ZN13QIntValidator13bottomChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QValidator_SlotProxy_connect__ZN10QValidator7changedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QRegExpValidator_SlotProxy_connect__ZN16QRegExpValidator13regExpChangedERK7QRegExp(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRegularExpressionValidator)=1
#[derive(Default)]
pub struct QRegularExpressionValidator {
  qbase: QValidator,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _regularExpressionChanged: QRegularExpressionValidator_regularExpressionChanged_signal,
}

// class sizeof(QDoubleValidator)=1
#[derive(Default)]
pub struct QDoubleValidator {
  qbase: QValidator,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _topChanged: QDoubleValidator_topChanged_signal,
  pub _notationChanged: QDoubleValidator_notationChanged_signal,
  pub _bottomChanged: QDoubleValidator_bottomChanged_signal,
  pub _decimalsChanged: QDoubleValidator_decimalsChanged_signal,
}

// class sizeof(QIntValidator)=1
#[derive(Default)]
pub struct QIntValidator {
  qbase: QValidator,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _topChanged: QIntValidator_topChanged_signal,
  pub _bottomChanged: QIntValidator_bottomChanged_signal,
}

// class sizeof(QValidator)=1
#[derive(Default)]
pub struct QValidator {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _changed: QValidator_changed_signal,
}

// class sizeof(QRegExpValidator)=1
#[derive(Default)]
pub struct QRegExpValidator {
  qbase: QValidator,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _regExpChanged: QRegExpValidator_regExpChanged_signal,
}

impl /*struct*/ QRegularExpressionValidator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRegularExpressionValidator {
    return QRegularExpressionValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QRegularExpressionValidator {
  type Target = QValidator;

  fn deref(&self) -> &QValidator {
    return & self.qbase;
  }
}
impl AsRef<QValidator> for QRegularExpressionValidator {
  fn as_ref(& self) -> & QValidator {
    return & self.qbase;
  }
}
  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpression<RetType, T: QRegularExpressionValidator_regularExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_regularExpression<RetType> {
  fn regularExpression(self , rsthis: & QRegularExpressionValidator) -> RetType;
}

  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: & QRegularExpressionValidator) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QRegularExpressionValidator17regularExpressionEv()};
    let mut ret = unsafe {C_ZNK27QRegularExpressionValidator17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::~QRegularExpressionValidator();
impl /*struct*/ QRegularExpressionValidator {
  pub fn free<RetType, T: QRegularExpressionValidator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_free<RetType> {
  fn free(self , rsthis: & QRegularExpressionValidator) -> RetType;
}

  // proto:  void QRegularExpressionValidator::~QRegularExpressionValidator();
impl<'a> /*trait*/ QRegularExpressionValidator_free<()> for () {
  fn free(self , rsthis: & QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorD2Ev()};
     unsafe {C_ZN27QRegularExpressionValidatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
impl /*struct*/ QRegularExpressionValidator {
  pub fn new<T: QRegularExpressionValidator_new>(value: T) -> QRegularExpressionValidator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionValidator_new {
  fn new(self) -> QRegularExpressionValidator;
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
impl<'a> /*trait*/ QRegularExpressionValidator_new for (&'a QRegularExpression, &'a QObject) {
  fn new(self) -> QRegularExpressionValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorC2ERK18QRegularExpressionP7QObject()};
    let ctysz: c_int = unsafe{QRegularExpressionValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN27QRegularExpressionValidatorC2ERK18QRegularExpressionP7QObject(arg0, arg1)};
    let rsthis = QRegularExpressionValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
impl /*struct*/ QRegularExpressionValidator {
  pub fn metaObject<RetType, T: QRegularExpressionValidator_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRegularExpressionValidator) -> RetType;
}

  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
impl<'a> /*trait*/ QRegularExpressionValidator_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QRegularExpressionValidator) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QRegularExpressionValidator10metaObjectEv()};
    let mut ret = unsafe {C_ZNK27QRegularExpressionValidator10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::QRegularExpressionValidator(QObject * parent);
impl<'a> /*trait*/ QRegularExpressionValidator_new for (&'a QObject) {
  fn new(self) -> QRegularExpressionValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorC2EP7QObject()};
    let ctysz: c_int = unsafe{QRegularExpressionValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN27QRegularExpressionValidatorC2EP7QObject(arg0)};
    let rsthis = QRegularExpressionValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
impl /*struct*/ QRegularExpressionValidator {
  pub fn setRegularExpression<RetType, T: QRegularExpressionValidator_setRegularExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRegularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_setRegularExpression<RetType> {
  fn setRegularExpression(self , rsthis: & QRegularExpressionValidator) -> RetType;
}

  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_setRegularExpression<()> for (&'a QRegularExpression) {
  fn setRegularExpression(self , rsthis: & QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDoubleValidator {
    return QDoubleValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDoubleValidator {
  type Target = QValidator;

  fn deref(&self) -> &QValidator {
    return & self.qbase;
  }
}
impl AsRef<QValidator> for QDoubleValidator {
  fn as_ref(& self) -> & QValidator {
    return & self.qbase;
  }
}
  // proto:  int QDoubleValidator::decimals();
impl /*struct*/ QDoubleValidator {
  pub fn decimals<RetType, T: QDoubleValidator_decimals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.decimals(self);
    // return 1;
  }
}

pub trait QDoubleValidator_decimals<RetType> {
  fn decimals(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  int QDoubleValidator::decimals();
impl<'a> /*trait*/ QDoubleValidator_decimals<i32> for () {
  fn decimals(self , rsthis: & QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator8decimalsEv()};
    let mut ret = unsafe {C_ZNK16QDoubleValidator8decimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::~QDoubleValidator();
impl /*struct*/ QDoubleValidator {
  pub fn free<RetType, T: QDoubleValidator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDoubleValidator_free<RetType> {
  fn free(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::~QDoubleValidator();
impl<'a> /*trait*/ QDoubleValidator_free<()> for () {
  fn free(self , rsthis: & QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorD2Ev()};
     unsafe {C_ZN16QDoubleValidatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  double QDoubleValidator::top();
impl /*struct*/ QDoubleValidator {
  pub fn top<RetType, T: QDoubleValidator_top<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QDoubleValidator_top<RetType> {
  fn top(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  double QDoubleValidator::top();
impl<'a> /*trait*/ QDoubleValidator_top<f64> for () {
  fn top(self , rsthis: & QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator3topEv()};
    let mut ret = unsafe {C_ZNK16QDoubleValidator3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  double QDoubleValidator::bottom();
impl /*struct*/ QDoubleValidator {
  pub fn bottom<RetType, T: QDoubleValidator_bottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QDoubleValidator_bottom<RetType> {
  fn bottom(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  double QDoubleValidator::bottom();
impl<'a> /*trait*/ QDoubleValidator_bottom<f64> for () {
  fn bottom(self , rsthis: & QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator6bottomEv()};
    let mut ret = unsafe {C_ZNK16QDoubleValidator6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setDecimals(int );
impl /*struct*/ QDoubleValidator {
  pub fn setDecimals<RetType, T: QDoubleValidator_setDecimals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDecimals(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setDecimals<RetType> {
  fn setDecimals(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setDecimals(int );
impl<'a> /*trait*/ QDoubleValidator_setDecimals<()> for (i32) {
  fn setDecimals(self , rsthis: & QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator11setDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN16QDoubleValidator11setDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setBottom(double );
impl /*struct*/ QDoubleValidator {
  pub fn setBottom<RetType, T: QDoubleValidator_setBottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setBottom<RetType> {
  fn setBottom(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setBottom(double );
impl<'a> /*trait*/ QDoubleValidator_setBottom<()> for (f64) {
  fn setBottom(self , rsthis: & QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN16QDoubleValidator9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
impl /*struct*/ QDoubleValidator {
  pub fn setRange<RetType, T: QDoubleValidator_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setRange<RetType> {
  fn setRange(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
impl<'a> /*trait*/ QDoubleValidator_setRange<()> for (f64, f64, i32) {
  fn setRange(self , rsthis: & QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator8setRangeEddi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN16QDoubleValidator8setRangeEddi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QDoubleValidator::QDoubleValidator(QObject * parent);
impl /*struct*/ QDoubleValidator {
  pub fn new<T: QDoubleValidator_new>(value: T) -> QDoubleValidator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleValidator_new {
  fn new(self) -> QDoubleValidator;
}

  // proto:  void QDoubleValidator::QDoubleValidator(QObject * parent);
impl<'a> /*trait*/ QDoubleValidator_new for (&'a QObject) {
  fn new(self) -> QDoubleValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC2EP7QObject()};
    let ctysz: c_int = unsafe{QDoubleValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QDoubleValidatorC2EP7QObject(arg0)};
    let rsthis = QDoubleValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::QDoubleValidator(double bottom, double top, int decimals, QObject * parent);
impl<'a> /*trait*/ QDoubleValidator_new for (f64, f64, i32, &'a QObject) {
  fn new(self) -> QDoubleValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC2EddiP7QObject()};
    let ctysz: c_int = unsafe{QDoubleValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QDoubleValidatorC2EddiP7QObject(arg0, arg1, arg2, arg3)};
    let rsthis = QDoubleValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDoubleValidator::metaObject();
impl /*struct*/ QDoubleValidator {
  pub fn metaObject<RetType, T: QDoubleValidator_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDoubleValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  const QMetaObject * QDoubleValidator::metaObject();
impl<'a> /*trait*/ QDoubleValidator_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QDoubleValidator) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator10metaObjectEv()};
    let mut ret = unsafe {C_ZNK16QDoubleValidator10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDoubleValidator::setTop(double );
impl /*struct*/ QDoubleValidator {
  pub fn setTop<RetType, T: QDoubleValidator_setTop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setTop<RetType> {
  fn setTop(self , rsthis: & QDoubleValidator) -> RetType;
}

  // proto:  void QDoubleValidator::setTop(double );
impl<'a> /*trait*/ QDoubleValidator_setTop<()> for (f64) {
  fn setTop(self , rsthis: & QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN16QDoubleValidator6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntValidator {
    return QIntValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QIntValidator {
  type Target = QValidator;

  fn deref(&self) -> &QValidator {
    return & self.qbase;
  }
}
impl AsRef<QValidator> for QIntValidator {
  fn as_ref(& self) -> & QValidator {
    return & self.qbase;
  }
}
  // proto:  void QIntValidator::QIntValidator(QObject * parent);
impl /*struct*/ QIntValidator {
  pub fn new<T: QIntValidator_new>(value: T) -> QIntValidator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIntValidator_new {
  fn new(self) -> QIntValidator;
}

  // proto:  void QIntValidator::QIntValidator(QObject * parent);
impl<'a> /*trait*/ QIntValidator_new for (&'a QObject) {
  fn new(self) -> QIntValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC2EP7QObject()};
    let ctysz: c_int = unsafe{QIntValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QIntValidatorC2EP7QObject(arg0)};
    let rsthis = QIntValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QIntValidator::setBottom(int );
impl /*struct*/ QIntValidator {
  pub fn setBottom<RetType, T: QIntValidator_setBottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QIntValidator_setBottom<RetType> {
  fn setBottom(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::setBottom(int );
impl<'a> /*trait*/ QIntValidator_setBottom<()> for (i32) {
  fn setBottom(self , rsthis: & QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator9setBottomEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QIntValidator9setBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::setRange(int bottom, int top);
impl /*struct*/ QIntValidator {
  pub fn setRange<RetType, T: QIntValidator_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QIntValidator_setRange<RetType> {
  fn setRange(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::setRange(int bottom, int top);
impl<'a> /*trait*/ QIntValidator_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: & QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QIntValidator8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QIntValidator::metaObject();
impl /*struct*/ QIntValidator {
  pub fn metaObject<RetType, T: QIntValidator_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIntValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  const QMetaObject * QIntValidator::metaObject();
impl<'a> /*trait*/ QIntValidator_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QIntValidator) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QIntValidator10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QIntValidator::top();
impl /*struct*/ QIntValidator {
  pub fn top<RetType, T: QIntValidator_top<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QIntValidator_top<RetType> {
  fn top(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  int QIntValidator::top();
impl<'a> /*trait*/ QIntValidator_top<i32> for () {
  fn top(self , rsthis: & QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator3topEv()};
    let mut ret = unsafe {C_ZNK13QIntValidator3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QIntValidator::fixup(QString & input);
impl /*struct*/ QIntValidator {
  pub fn fixup<RetType, T: QIntValidator_fixup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QIntValidator_fixup<RetType> {
  fn fixup(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::fixup(QString & input);
impl<'a> /*trait*/ QIntValidator_fixup<()> for (&'a QString) {
  fn fixup(self , rsthis: & QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QIntValidator5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIntValidator::~QIntValidator();
impl /*struct*/ QIntValidator {
  pub fn free<RetType, T: QIntValidator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIntValidator_free<RetType> {
  fn free(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::~QIntValidator();
impl<'a> /*trait*/ QIntValidator_free<()> for () {
  fn free(self , rsthis: & QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorD2Ev()};
     unsafe {C_ZN13QIntValidatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIntValidator::setTop(int );
impl /*struct*/ QIntValidator {
  pub fn setTop<RetType, T: QIntValidator_setTop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QIntValidator_setTop<RetType> {
  fn setTop(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  void QIntValidator::setTop(int );
impl<'a> /*trait*/ QIntValidator_setTop<()> for (i32) {
  fn setTop(self , rsthis: & QIntValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator6setTopEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QIntValidator6setTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QIntValidator::bottom();
impl /*struct*/ QIntValidator {
  pub fn bottom<RetType, T: QIntValidator_bottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QIntValidator_bottom<RetType> {
  fn bottom(self , rsthis: & QIntValidator) -> RetType;
}

  // proto:  int QIntValidator::bottom();
impl<'a> /*trait*/ QIntValidator_bottom<i32> for () {
  fn bottom(self , rsthis: & QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator6bottomEv()};
    let mut ret = unsafe {C_ZNK13QIntValidator6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QIntValidator::QIntValidator(int bottom, int top, QObject * parent);
impl<'a> /*trait*/ QIntValidator_new for (i32, i32, &'a QObject) {
  fn new(self) -> QIntValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC2EiiP7QObject()};
    let ctysz: c_int = unsafe{QIntValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QIntValidatorC2EiiP7QObject(arg0, arg1, arg2)};
    let rsthis = QIntValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QValidator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QValidator {
    return QValidator{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QValidator {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QValidator {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QValidator::metaObject();
impl /*struct*/ QValidator {
  pub fn metaObject<RetType, T: QValidator_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: & QValidator) -> RetType;
}

  // proto:  const QMetaObject * QValidator::metaObject();
impl<'a> /*trait*/ QValidator_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QValidator) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QValidator10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QValidator::setLocale(const QLocale & locale);
impl /*struct*/ QValidator {
  pub fn setLocale<RetType, T: QValidator_setLocale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocale(self);
    // return 1;
  }
}

pub trait QValidator_setLocale<RetType> {
  fn setLocale(self , rsthis: & QValidator) -> RetType;
}

  // proto:  void QValidator::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QValidator_setLocale<()> for (&'a QLocale) {
  fn setLocale(self , rsthis: & QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidator9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QValidator9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QValidator::fixup(QString & );
impl /*struct*/ QValidator {
  pub fn fixup<RetType, T: QValidator_fixup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QValidator_fixup<RetType> {
  fn fixup(self , rsthis: & QValidator) -> RetType;
}

  // proto:  void QValidator::fixup(QString & );
impl<'a> /*trait*/ QValidator_fixup<()> for (&'a QString) {
  fn fixup(self , rsthis: & QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK10QValidator5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QValidator::~QValidator();
impl /*struct*/ QValidator {
  pub fn free<RetType, T: QValidator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QValidator_free<RetType> {
  fn free(self , rsthis: & QValidator) -> RetType;
}

  // proto:  void QValidator::~QValidator();
impl<'a> /*trait*/ QValidator_free<()> for () {
  fn free(self , rsthis: & QValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorD2Ev()};
     unsafe {C_ZN10QValidatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QValidator::QValidator(QObject * parent);
impl /*struct*/ QValidator {
  pub fn new<T: QValidator_new>(value: T) -> QValidator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QValidator_new {
  fn new(self) -> QValidator;
}

  // proto:  void QValidator::QValidator(QObject * parent);
impl<'a> /*trait*/ QValidator_new for (&'a QObject) {
  fn new(self) -> QValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorC2EP7QObject()};
    let ctysz: c_int = unsafe{QValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QValidatorC2EP7QObject(arg0)};
    let rsthis = QValidator{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QLocale QValidator::locale();
impl /*struct*/ QValidator {
  pub fn locale<RetType, T: QValidator_locale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QValidator_locale<RetType> {
  fn locale(self , rsthis: & QValidator) -> RetType;
}

  // proto:  QLocale QValidator::locale();
impl<'a> /*trait*/ QValidator_locale<QLocale> for () {
  fn locale(self , rsthis: & QValidator) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator6localeEv()};
    let mut ret = unsafe {C_ZNK10QValidator6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegExpValidator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRegExpValidator {
    return QRegExpValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QRegExpValidator {
  type Target = QValidator;

  fn deref(&self) -> &QValidator {
    return & self.qbase;
  }
}
impl AsRef<QValidator> for QRegExpValidator {
  fn as_ref(& self) -> & QValidator {
    return & self.qbase;
  }
}
  // proto:  void QRegExpValidator::~QRegExpValidator();
impl /*struct*/ QRegExpValidator {
  pub fn free<RetType, T: QRegExpValidator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRegExpValidator_free<RetType> {
  fn free(self , rsthis: & QRegExpValidator) -> RetType;
}

  // proto:  void QRegExpValidator::~QRegExpValidator();
impl<'a> /*trait*/ QRegExpValidator_free<()> for () {
  fn free(self , rsthis: & QRegExpValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorD2Ev()};
     unsafe {C_ZN16QRegExpValidatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QRegExp & QRegExpValidator::regExp();
impl /*struct*/ QRegExpValidator {
  pub fn regExp<RetType, T: QRegExpValidator_regExp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.regExp(self);
    // return 1;
  }
}

pub trait QRegExpValidator_regExp<RetType> {
  fn regExp(self , rsthis: & QRegExpValidator) -> RetType;
}

  // proto:  const QRegExp & QRegExpValidator::regExp();
impl<'a> /*trait*/ QRegExpValidator_regExp<QRegExp> for () {
  fn regExp(self , rsthis: & QRegExpValidator) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QRegExpValidator6regExpEv()};
    let mut ret = unsafe {C_ZNK16QRegExpValidator6regExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRegExpValidator::metaObject();
impl /*struct*/ QRegExpValidator {
  pub fn metaObject<RetType, T: QRegExpValidator_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRegExpValidator_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRegExpValidator) -> RetType;
}

  // proto:  const QMetaObject * QRegExpValidator::metaObject();
impl<'a> /*trait*/ QRegExpValidator_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QRegExpValidator) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QRegExpValidator10metaObjectEv()};
    let mut ret = unsafe {C_ZNK16QRegExpValidator10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExp & rx, QObject * parent);
impl /*struct*/ QRegExpValidator {
  pub fn new<T: QRegExpValidator_new>(value: T) -> QRegExpValidator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExpValidator_new {
  fn new(self) -> QRegExpValidator;
}

  // proto:  void QRegExpValidator::QRegExpValidator(const QRegExp & rx, QObject * parent);
impl<'a> /*trait*/ QRegExpValidator_new for (&'a QRegExp, &'a QObject) {
  fn new(self) -> QRegExpValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC2ERK7QRegExpP7QObject()};
    let ctysz: c_int = unsafe{QRegExpValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QRegExpValidatorC2ERK7QRegExpP7QObject(arg0, arg1)};
    let rsthis = QRegExpValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegExpValidator::setRegExp(const QRegExp & rx);
impl /*struct*/ QRegExpValidator {
  pub fn setRegExp<RetType, T: QRegExpValidator_setRegExp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRegExp(self);
    // return 1;
  }
}

pub trait QRegExpValidator_setRegExp<RetType> {
  fn setRegExp(self , rsthis: & QRegExpValidator) -> RetType;
}

  // proto:  void QRegExpValidator::setRegExp(const QRegExp & rx);
impl<'a> /*trait*/ QRegExpValidator_setRegExp<()> for (&'a QRegExp) {
  fn setRegExp(self , rsthis: & QRegExpValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidator9setRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QRegExpValidator9setRegExpERK7QRegExp(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegExpValidator::QRegExpValidator(QObject * parent);
impl<'a> /*trait*/ QRegExpValidator_new for (&'a QObject) {
  fn new(self) -> QRegExpValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QRegExpValidatorC2EP7QObject()};
    let ctysz: c_int = unsafe{QRegExpValidator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QRegExpValidatorC2EP7QObject(arg0)};
    let rsthis = QRegExpValidator{qbase: QValidator::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QRegularExpressionValidator_regularExpressionChanged
pub struct QRegularExpressionValidator_regularExpressionChanged_signal{poi:u64}
impl /* struct */ QRegularExpressionValidator {
  pub fn regularExpressionChanged(&self) -> QRegularExpressionValidator_regularExpressionChanged_signal {
     return QRegularExpressionValidator_regularExpressionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QRegularExpressionValidator_regularExpressionChanged_signal {
  pub fn connect<T: QRegularExpressionValidator_regularExpressionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QRegularExpressionValidator_regularExpressionChanged_signal_connect {
  fn connect(self, sigthis: QRegularExpressionValidator_regularExpressionChanged_signal);
}

// regularExpressionChanged(const class QRegularExpression &)
extern fn QRegularExpressionValidator_regularExpressionChanged_signal_connect_cb_0(rsfptr:fn(QRegularExpression), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRegularExpression::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QRegularExpressionValidator_regularExpressionChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QRegularExpression)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRegularExpression::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QRegularExpressionValidator_regularExpressionChanged_signal_connect for fn(QRegularExpression) {
  fn connect(self, sigthis: QRegularExpressionValidator_regularExpressionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QRegularExpressionValidator_regularExpressionChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QRegularExpressionValidator_SlotProxy_connect__ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(arg0, arg1, arg2)};
  }
}
impl /* trait */ QRegularExpressionValidator_regularExpressionChanged_signal_connect for Box<Fn(QRegularExpression)> {
  fn connect(self, sigthis: QRegularExpressionValidator_regularExpressionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QRegularExpressionValidator_regularExpressionChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QRegularExpressionValidator_SlotProxy_connect__ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QDoubleValidator_topChanged
pub struct QDoubleValidator_topChanged_signal{poi:u64}
impl /* struct */ QDoubleValidator {
  pub fn topChanged(&self) -> QDoubleValidator_topChanged_signal {
     return QDoubleValidator_topChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDoubleValidator_topChanged_signal {
  pub fn connect<T: QDoubleValidator_topChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDoubleValidator_topChanged_signal_connect {
  fn connect(self, sigthis: QDoubleValidator_topChanged_signal);
}

#[derive(Default)] // for QDoubleValidator_notationChanged
pub struct QDoubleValidator_notationChanged_signal{poi:u64}
impl /* struct */ QDoubleValidator {
  pub fn notationChanged(&self) -> QDoubleValidator_notationChanged_signal {
     return QDoubleValidator_notationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDoubleValidator_notationChanged_signal {
  pub fn connect<T: QDoubleValidator_notationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDoubleValidator_notationChanged_signal_connect {
  fn connect(self, sigthis: QDoubleValidator_notationChanged_signal);
}

#[derive(Default)] // for QDoubleValidator_bottomChanged
pub struct QDoubleValidator_bottomChanged_signal{poi:u64}
impl /* struct */ QDoubleValidator {
  pub fn bottomChanged(&self) -> QDoubleValidator_bottomChanged_signal {
     return QDoubleValidator_bottomChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDoubleValidator_bottomChanged_signal {
  pub fn connect<T: QDoubleValidator_bottomChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDoubleValidator_bottomChanged_signal_connect {
  fn connect(self, sigthis: QDoubleValidator_bottomChanged_signal);
}

#[derive(Default)] // for QDoubleValidator_decimalsChanged
pub struct QDoubleValidator_decimalsChanged_signal{poi:u64}
impl /* struct */ QDoubleValidator {
  pub fn decimalsChanged(&self) -> QDoubleValidator_decimalsChanged_signal {
     return QDoubleValidator_decimalsChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDoubleValidator_decimalsChanged_signal {
  pub fn connect<T: QDoubleValidator_decimalsChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDoubleValidator_decimalsChanged_signal_connect {
  fn connect(self, sigthis: QDoubleValidator_decimalsChanged_signal);
}

// bottomChanged(double)
extern fn QDoubleValidator_bottomChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QDoubleValidator_bottomChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDoubleValidator_bottomChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QDoubleValidator_bottomChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDoubleValidator_bottomChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator13bottomChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDoubleValidator_bottomChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QDoubleValidator_bottomChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDoubleValidator_bottomChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator13bottomChangedEd(arg0, arg1, arg2)};
  }
}
// decimalsChanged(int)
extern fn QDoubleValidator_decimalsChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QDoubleValidator_decimalsChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDoubleValidator_decimalsChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDoubleValidator_decimalsChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDoubleValidator_decimalsChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator15decimalsChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDoubleValidator_decimalsChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QDoubleValidator_decimalsChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDoubleValidator_decimalsChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator15decimalsChangedEi(arg0, arg1, arg2)};
  }
}
// topChanged(double)
extern fn QDoubleValidator_topChanged_signal_connect_cb_2(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QDoubleValidator_topChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDoubleValidator_topChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QDoubleValidator_topChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDoubleValidator_topChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator10topChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDoubleValidator_topChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QDoubleValidator_topChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDoubleValidator_topChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDoubleValidator_SlotProxy_connect__ZN16QDoubleValidator10topChangedEd(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QIntValidator_topChanged
pub struct QIntValidator_topChanged_signal{poi:u64}
impl /* struct */ QIntValidator {
  pub fn topChanged(&self) -> QIntValidator_topChanged_signal {
     return QIntValidator_topChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QIntValidator_topChanged_signal {
  pub fn connect<T: QIntValidator_topChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QIntValidator_topChanged_signal_connect {
  fn connect(self, sigthis: QIntValidator_topChanged_signal);
}

#[derive(Default)] // for QIntValidator_bottomChanged
pub struct QIntValidator_bottomChanged_signal{poi:u64}
impl /* struct */ QIntValidator {
  pub fn bottomChanged(&self) -> QIntValidator_bottomChanged_signal {
     return QIntValidator_bottomChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QIntValidator_bottomChanged_signal {
  pub fn connect<T: QIntValidator_bottomChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QIntValidator_bottomChanged_signal_connect {
  fn connect(self, sigthis: QIntValidator_bottomChanged_signal);
}

// topChanged(int)
extern fn QIntValidator_topChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QIntValidator_topChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QIntValidator_topChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QIntValidator_topChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIntValidator_topChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QIntValidator_SlotProxy_connect__ZN13QIntValidator10topChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QIntValidator_topChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QIntValidator_topChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIntValidator_topChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QIntValidator_SlotProxy_connect__ZN13QIntValidator10topChangedEi(arg0, arg1, arg2)};
  }
}
// bottomChanged(int)
extern fn QIntValidator_bottomChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QIntValidator_bottomChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QIntValidator_bottomChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QIntValidator_bottomChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIntValidator_bottomChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QIntValidator_SlotProxy_connect__ZN13QIntValidator13bottomChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QIntValidator_bottomChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QIntValidator_bottomChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIntValidator_bottomChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QIntValidator_SlotProxy_connect__ZN13QIntValidator13bottomChangedEi(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QValidator_changed
pub struct QValidator_changed_signal{poi:u64}
impl /* struct */ QValidator {
  pub fn changed(&self) -> QValidator_changed_signal {
     return QValidator_changed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QValidator_changed_signal {
  pub fn connect<T: QValidator_changed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QValidator_changed_signal_connect {
  fn connect(self, sigthis: QValidator_changed_signal);
}

// changed()
extern fn QValidator_changed_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QValidator_changed_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QValidator_changed_signal_connect for fn() {
  fn connect(self, sigthis: QValidator_changed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QValidator_changed_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QValidator_SlotProxy_connect__ZN10QValidator7changedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QValidator_changed_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QValidator_changed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QValidator_changed_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QValidator_SlotProxy_connect__ZN10QValidator7changedEv(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QRegExpValidator_regExpChanged
pub struct QRegExpValidator_regExpChanged_signal{poi:u64}
impl /* struct */ QRegExpValidator {
  pub fn regExpChanged(&self) -> QRegExpValidator_regExpChanged_signal {
     return QRegExpValidator_regExpChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QRegExpValidator_regExpChanged_signal {
  pub fn connect<T: QRegExpValidator_regExpChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QRegExpValidator_regExpChanged_signal_connect {
  fn connect(self, sigthis: QRegExpValidator_regExpChanged_signal);
}

// regExpChanged(const class QRegExp &)
extern fn QRegExpValidator_regExpChanged_signal_connect_cb_0(rsfptr:fn(QRegExp), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRegExp::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QRegExpValidator_regExpChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QRegExp)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRegExp::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QRegExpValidator_regExpChanged_signal_connect for fn(QRegExp) {
  fn connect(self, sigthis: QRegExpValidator_regExpChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QRegExpValidator_regExpChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QRegExpValidator_SlotProxy_connect__ZN16QRegExpValidator13regExpChangedERK7QRegExp(arg0, arg1, arg2)};
  }
}
impl /* trait */ QRegExpValidator_regExpChanged_signal_connect for Box<Fn(QRegExp)> {
  fn connect(self, sigthis: QRegExpValidator_regExpChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QRegExpValidator_regExpChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QRegExpValidator_SlotProxy_connect__ZN16QRegExpValidator13regExpChangedERK7QRegExp(arg0, arg1, arg2)};
  }
}
// <= body block end

