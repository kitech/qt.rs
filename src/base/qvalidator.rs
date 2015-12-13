// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qlocale::QLocale;
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QValidator::metaObject();
  fn _ZNK10QValidator10metaObjectEv() -> i32;
  // proto: void QValidator::NewQValidator(const QValidator & );
  fn _ZN10QValidatorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QValidator::setLocale(const QLocale & locale);
  fn _ZN10QValidator9setLocaleERK7QLocale(arg0: *const c_void) -> i32;
  // proto: void QValidator::fixup(QString & );
  fn _ZNK10QValidator5fixupER7QString(arg0: *mut c_void) -> i32;
  // proto: void QValidator::FreeQValidator();
  fn _ZN10QValidatorD0Ev() -> i32;
  // proto: void QValidator::NewQValidator(QObject * parent);
  fn _ZN10QValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QLocale QValidator::locale();
  fn _ZNK10QValidator6localeEv() -> i32;
  // proto: void QValidator::changed();
  fn _ZN10QValidator7changedEv() -> i32;
}

// body block begin
// class sizeof(QValidator)=1
pub struct QValidator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QValidator {
  pub fn metaObject<T: QValidator_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QValidator_metaObject {
  fn metaObject(self, this: &mut QValidator) -> i32;
}

// proto: const QMetaObject * QValidator::metaObject();
impl<'a> /*trait*/ QValidator_metaObject for () {
  fn metaObject(self, this: &mut QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator10metaObjectEv()};
    unsafe {_ZNK10QValidator10metaObjectEv()};
    return 1;
  }
}

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

// proto: void QValidator::NewQValidator(const QValidator & );
impl<'a> /*trait*/ QValidator_NewQValidator for (&'a  QValidator) {
  fn NewQValidator(self) -> QValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QValidator {
  pub fn setLocale<T: QValidator_setLocale>(&mut self, value: T) -> i32 {
    value.setLocale(self);
    return 1;
  }
}

pub trait QValidator_setLocale {
  fn setLocale(self, this: &mut QValidator) -> i32;
}

// proto: void QValidator::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QValidator_setLocale for (&'a  QLocale) {
  fn setLocale(self, this: &mut QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidator9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QValidator9setLocaleERK7QLocale(arg0)};
    return 1;
  }
}

impl /*struct*/ QValidator {
  pub fn fixup<T: QValidator_fixup>(&mut self, value: T) -> i32 {
    value.fixup(self);
    return 1;
  }
}

pub trait QValidator_fixup {
  fn fixup(self, this: &mut QValidator) -> i32;
}

// proto: void QValidator::fixup(QString & );
impl<'a> /*trait*/ QValidator_fixup for (&'a mut QString) {
  fn fixup(self, this: &mut QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK10QValidator5fixupER7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QValidator {
  pub fn FreeQValidator<T: QValidator_FreeQValidator>(&mut self, value: T) -> i32 {
    value.FreeQValidator(self);
    return 1;
  }
}

pub trait QValidator_FreeQValidator {
  fn FreeQValidator(self, this: &mut QValidator) -> i32;
}

// proto: void QValidator::FreeQValidator();
impl<'a> /*trait*/ QValidator_FreeQValidator for () {
  fn FreeQValidator(self, this: &mut QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidatorD0Ev()};
    unsafe {_ZN10QValidatorD0Ev()};
    return 1;
  }
}

// proto: void QValidator::NewQValidator(QObject * parent);
impl<'a> /*trait*/ QValidator_NewQValidator for (&'a mut QObject) {
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

impl /*struct*/ QValidator {
  pub fn locale<T: QValidator_locale>(&mut self, value: T) -> i32 {
    value.locale(self);
    return 1;
  }
}

pub trait QValidator_locale {
  fn locale(self, this: &mut QValidator) -> i32;
}

// proto: QLocale QValidator::locale();
impl<'a> /*trait*/ QValidator_locale for () {
  fn locale(self, this: &mut QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QValidator6localeEv()};
    unsafe {_ZNK10QValidator6localeEv()};
    return 1;
  }
}

impl /*struct*/ QValidator {
  pub fn changed<T: QValidator_changed>(&mut self, value: T) -> i32 {
    value.changed(self);
    return 1;
  }
}

pub trait QValidator_changed {
  fn changed(self, this: &mut QValidator) -> i32;
}

// proto: void QValidator::changed();
impl<'a> /*trait*/ QValidator_changed for () {
  fn changed(self, this: &mut QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QValidator7changedEv()};
    unsafe {_ZN10QValidator7changedEv()};
    return 1;
  }
}

