// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK19QProcessEnvironment8containsERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK19QProcessEnvironment4keysEv() -> i32;
  fn _ZN19QProcessEnvironment6removeERK7QString(arg0: *const c_void) -> i32;
  fn _ZN19QProcessEnvironment5clearEv() -> i32;
  fn _ZNK19QProcessEnvironment5valueERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QProcessEnvironment7isEmptyEv() -> i32;
  fn _ZN19QProcessEnvironmentD0Ev() -> i32;
  fn _ZN19QProcessEnvironment4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN19QProcessEnvironmentC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN19QProcessEnvironment17systemEnvironmentEv() -> i32;
  fn _ZN19QProcessEnvironment6insertERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QProcessEnvironment12toStringListEv() -> i32;
  fn _ZN19QProcessEnvironmentC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN19QProcessEnvironment6insertERKS_(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QProcessEnvironment)=1
pub struct QProcessEnvironment {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProcessEnvironment {
  pub fn contains<T: QProcessEnvironment_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QProcessEnvironment_contains {
  fn contains(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: bool QProcessEnvironment::contains(const QString & name);
impl<'a> /*trait*/ QProcessEnvironment_contains for (&'a  QString) {
  fn contains(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment8containsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QProcessEnvironment8containsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn keys<T: QProcessEnvironment_keys>(&mut self, value: T) -> i32 {
    value.keys(self);
    return 1;
  }
}

pub trait QProcessEnvironment_keys {
  fn keys(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: QStringList QProcessEnvironment::keys();
impl<'a> /*trait*/ QProcessEnvironment_keys for () {
  fn keys(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment4keysEv()};
    unsafe {_ZNK19QProcessEnvironment4keysEv()};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn remove<T: QProcessEnvironment_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QProcessEnvironment_remove {
  fn remove(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: void QProcessEnvironment::remove(const QString & name);
impl<'a> /*trait*/ QProcessEnvironment_remove for (&'a  QString) {
  fn remove(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6removeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QProcessEnvironment6removeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn clear<T: QProcessEnvironment_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QProcessEnvironment_clear {
  fn clear(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: void QProcessEnvironment::clear();
impl<'a> /*trait*/ QProcessEnvironment_clear for () {
  fn clear(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment5clearEv()};
    unsafe {_ZN19QProcessEnvironment5clearEv()};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn value<T: QProcessEnvironment_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QProcessEnvironment_value {
  fn value(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
impl<'a> /*trait*/ QProcessEnvironment_value for (&'a  QString, &'a  QString) {
  fn value(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QProcessEnvironment5valueERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn isEmpty<T: QProcessEnvironment_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QProcessEnvironment_isEmpty {
  fn isEmpty(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: bool QProcessEnvironment::isEmpty();
impl<'a> /*trait*/ QProcessEnvironment_isEmpty for () {
  fn isEmpty(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment7isEmptyEv()};
    unsafe {_ZNK19QProcessEnvironment7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn FreeQProcessEnvironment<T: QProcessEnvironment_FreeQProcessEnvironment>(&mut self, value: T) -> i32 {
    value.FreeQProcessEnvironment(self);
    return 1;
  }
}

pub trait QProcessEnvironment_FreeQProcessEnvironment {
  fn FreeQProcessEnvironment(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: void QProcessEnvironment::FreeQProcessEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_FreeQProcessEnvironment for () {
  fn FreeQProcessEnvironment(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentD0Ev()};
    unsafe {_ZN19QProcessEnvironmentD0Ev()};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn swap<T: QProcessEnvironment_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QProcessEnvironment_swap {
  fn swap(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: void QProcessEnvironment::swap(QProcessEnvironment & other);
impl<'a> /*trait*/ QProcessEnvironment_swap for (&'a mut QProcessEnvironment) {
  fn swap(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QProcessEnvironment4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn NewQProcessEnvironment<T: QProcessEnvironment_NewQProcessEnvironment>(value: T) -> QProcessEnvironment {
    let rsthis = value.NewQProcessEnvironment();
    return rsthis;
    // return 1;
  }
}

pub trait QProcessEnvironment_NewQProcessEnvironment {
  fn NewQProcessEnvironment(self) -> QProcessEnvironment;
}

// proto: void QProcessEnvironment::NewQProcessEnvironment(const QProcessEnvironment & other);
impl<'a> /*trait*/ QProcessEnvironment_NewQProcessEnvironment for (&'a  QProcessEnvironment) {
  fn NewQProcessEnvironment(self) -> QProcessEnvironment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QProcessEnvironmentC1ERKS_(qthis, arg0)};
    let rsthis = QProcessEnvironment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn systemEnvironment<T: QProcessEnvironment_systemEnvironment>(&mut self, value: T) -> i32 {
    value.systemEnvironment(self);
    return 1;
  }
}

pub trait QProcessEnvironment_systemEnvironment {
  fn systemEnvironment(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: QProcessEnvironment QProcessEnvironment::systemEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_systemEnvironment for () {
  fn systemEnvironment(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment17systemEnvironmentEv()};
    unsafe {_ZN19QProcessEnvironment17systemEnvironmentEv()};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn insert<T: QProcessEnvironment_insert>(&mut self, value: T) -> i32 {
    value.insert(self);
    return 1;
  }
}

pub trait QProcessEnvironment_insert {
  fn insert(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: void QProcessEnvironment::insert(const QString & name, const QString & value);
impl<'a> /*trait*/ QProcessEnvironment_insert for (&'a  QString, &'a  QString) {
  fn insert(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6insertERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QProcessEnvironment6insertERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn toStringList<T: QProcessEnvironment_toStringList>(&mut self, value: T) -> i32 {
    value.toStringList(self);
    return 1;
  }
}

pub trait QProcessEnvironment_toStringList {
  fn toStringList(self, this: &mut QProcessEnvironment) -> i32;
}

// proto: QStringList QProcessEnvironment::toStringList();
impl<'a> /*trait*/ QProcessEnvironment_toStringList for () {
  fn toStringList(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment12toStringListEv()};
    unsafe {_ZNK19QProcessEnvironment12toStringListEv()};
    return 1;
  }
}

// proto: void QProcessEnvironment::NewQProcessEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_NewQProcessEnvironment for () {
  fn NewQProcessEnvironment(self) -> QProcessEnvironment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentC1Ev()};
    unsafe {_ZN19QProcessEnvironmentC1Ev(qthis)};
    let rsthis = QProcessEnvironment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QProcessEnvironment::insert(const QProcessEnvironment & e);
impl<'a> /*trait*/ QProcessEnvironment_insert for (&'a  QProcessEnvironment) {
  fn insert(self, this: &mut QProcessEnvironment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6insertERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QProcessEnvironment6insertERKS_(arg0)};
    return 1;
  }
}

