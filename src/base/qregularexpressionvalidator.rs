// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qregularexpression::QRegularExpression;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
  fn _ZNK27QRegularExpressionValidator17regularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionValidator::FreeQRegularExpressionValidator();
  fn _ZN27QRegularExpressionValidatorD0Ev(qthis: *mut c_void) ;
  // proto:  void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
  fn _ZN27QRegularExpressionValidatorC1ERK18QRegularExpressionP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QMetaObject * QRegularExpressionValidator::metaObject();
  fn _ZNK27QRegularExpressionValidator10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpressionValidator & );
  fn _ZN27QRegularExpressionValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRegularExpressionValidator::NewQRegularExpressionValidator(QObject * parent);
  fn _ZN27QRegularExpressionValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
  fn _ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
  fn _ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QRegularExpressionValidator)=1
pub struct QRegularExpressionValidator {
  pub qclsinst: *mut c_void,
}

// proto:  QRegularExpression QRegularExpressionValidator::regularExpression();
impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpression<RetType, T: QRegularExpressionValidator_regularExpression<RetType>>(&mut self, overload_args: T) -> RetType {
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

// proto:  void QRegularExpressionValidator::FreeQRegularExpressionValidator();
impl /*struct*/ QRegularExpressionValidator {
  pub fn FreeQRegularExpressionValidator<RetType, T: QRegularExpressionValidator_FreeQRegularExpressionValidator<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQRegularExpressionValidator(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_FreeQRegularExpressionValidator<RetType> {
  fn FreeQRegularExpressionValidator(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

// proto:  void QRegularExpressionValidator::FreeQRegularExpressionValidator();
impl<'a> /*trait*/ QRegularExpressionValidator_FreeQRegularExpressionValidator<()> for () {
  fn FreeQRegularExpressionValidator(self , rsthis: &mut QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorD0Ev()};
     unsafe {_ZN27QRegularExpressionValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

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

// proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (&'a  QRegularExpression, &'a mut QObject) {
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
  pub fn metaObject<RetType, T: QRegularExpressionValidator_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
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

// proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpressionValidator & );
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (&'a  QRegularExpressionValidator) {
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

// proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(QObject * parent);
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (&'a mut QObject) {
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
  pub fn setRegularExpression<RetType, T: QRegularExpressionValidator_setRegularExpression<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRegularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_setRegularExpression<RetType> {
  fn setRegularExpression(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

// proto:  void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_setRegularExpression<()> for (&'a  QRegularExpression) {
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
  pub fn regularExpressionChanged<RetType, T: QRegularExpressionValidator_regularExpressionChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.regularExpressionChanged(self);
    // return 1;
  }
}

pub trait QRegularExpressionValidator_regularExpressionChanged<RetType> {
  fn regularExpressionChanged(self , rsthis: &mut QRegularExpressionValidator) -> RetType;
}

// proto:  void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpressionChanged<()> for (&'a  QRegularExpression) {
  fn regularExpressionChanged(self , rsthis: &mut QRegularExpressionValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

