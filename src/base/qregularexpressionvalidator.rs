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
  // proto: QRegularExpression QRegularExpressionValidator::regularExpression();
  fn _ZNK27QRegularExpressionValidator17regularExpressionEv() -> i32;
  // proto: void QRegularExpressionValidator::FreeQRegularExpressionValidator();
  fn _ZN27QRegularExpressionValidatorD0Ev() -> i32;
  // proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpression & re, QObject * parent);
  fn _ZN27QRegularExpressionValidatorC1ERK18QRegularExpressionP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: const QMetaObject * QRegularExpressionValidator::metaObject();
  fn _ZNK27QRegularExpressionValidator10metaObjectEv() -> i32;
  // proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpressionValidator & );
  fn _ZN27QRegularExpressionValidatorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(QObject * parent);
  fn _ZN27QRegularExpressionValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
  fn _ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(arg0: *const c_void) -> i32;
  // proto: void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
  fn _ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRegularExpressionValidator)=1
pub struct QRegularExpressionValidator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpression<T: QRegularExpressionValidator_regularExpression>(&mut self, value: T) -> i32 {
    value.regularExpression(self);
    return 1;
  }
}

pub trait QRegularExpressionValidator_regularExpression {
  fn regularExpression(self, this: &mut QRegularExpressionValidator) -> i32;
}

// proto: QRegularExpression QRegularExpressionValidator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpression for () {
  fn regularExpression(self, this: &mut QRegularExpressionValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QRegularExpressionValidator17regularExpressionEv()};
    unsafe {_ZNK27QRegularExpressionValidator17regularExpressionEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionValidator {
  pub fn FreeQRegularExpressionValidator<T: QRegularExpressionValidator_FreeQRegularExpressionValidator>(&mut self, value: T) -> i32 {
    value.FreeQRegularExpressionValidator(self);
    return 1;
  }
}

pub trait QRegularExpressionValidator_FreeQRegularExpressionValidator {
  fn FreeQRegularExpressionValidator(self, this: &mut QRegularExpressionValidator) -> i32;
}

// proto: void QRegularExpressionValidator::FreeQRegularExpressionValidator();
impl<'a> /*trait*/ QRegularExpressionValidator_FreeQRegularExpressionValidator for () {
  fn FreeQRegularExpressionValidator(self, this: &mut QRegularExpressionValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorD0Ev()};
    unsafe {_ZN27QRegularExpressionValidatorD0Ev()};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN27QRegularExpressionValidatorC1ERK18QRegularExpressionP7QObject(qthis, arg0, arg1)};
    let rsthis = QRegularExpressionValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionValidator {
  pub fn metaObject<T: QRegularExpressionValidator_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QRegularExpressionValidator_metaObject {
  fn metaObject(self, this: &mut QRegularExpressionValidator) -> i32;
}

// proto: const QMetaObject * QRegularExpressionValidator::metaObject();
impl<'a> /*trait*/ QRegularExpressionValidator_metaObject for () {
  fn metaObject(self, this: &mut QRegularExpressionValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QRegularExpressionValidator10metaObjectEv()};
    unsafe {_ZNK27QRegularExpressionValidator10metaObjectEv()};
    return 1;
  }
}

// proto: void QRegularExpressionValidator::NewQRegularExpressionValidator(const QRegularExpressionValidator & );
impl<'a> /*trait*/ QRegularExpressionValidator_NewQRegularExpressionValidator for (&'a  QRegularExpressionValidator) {
  fn NewQRegularExpressionValidator(self) -> QRegularExpressionValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
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

impl /*struct*/ QRegularExpressionValidator {
  pub fn setRegularExpression<T: QRegularExpressionValidator_setRegularExpression>(&mut self, value: T) -> i32 {
    value.setRegularExpression(self);
    return 1;
  }
}

pub trait QRegularExpressionValidator_setRegularExpression {
  fn setRegularExpression(self, this: &mut QRegularExpressionValidator) -> i32;
}

// proto: void QRegularExpressionValidator::setRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_setRegularExpression for (&'a  QRegularExpression) {
  fn setRegularExpression(self, this: &mut QRegularExpressionValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpressionChanged<T: QRegularExpressionValidator_regularExpressionChanged>(&mut self, value: T) -> i32 {
    value.regularExpressionChanged(self);
    return 1;
  }
}

pub trait QRegularExpressionValidator_regularExpressionChanged {
  fn regularExpressionChanged(self, this: &mut QRegularExpressionValidator) -> i32;
}

// proto: void QRegularExpressionValidator::regularExpressionChanged(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpressionChanged for (&'a  QRegularExpression) {
  fn regularExpressionChanged(self, this: &mut QRegularExpressionValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression(arg0)};
    return 1;
  }
}

