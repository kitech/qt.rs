// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN18QCommandLineOption12setValueNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK18QCommandLineOption5namesEv() -> i32;
  fn _ZN18QCommandLineOptionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN18QCommandLineOption14setDescriptionERK7QString(arg0: *const c_void) -> i32;
  fn _ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  fn _ZNK18QCommandLineOption9valueNameEv() -> i32;
  fn _ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  fn _ZN18QCommandLineOption4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK18QCommandLineOption11descriptionEv() -> i32;
  fn _ZN18QCommandLineOptionD0Ev() -> i32;
  fn _ZN18QCommandLineOptionC1ERK11QStringList(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN18QCommandLineOption15setDefaultValueERK7QString(arg0: *const c_void) -> i32;
  fn _ZN18QCommandLineOption16setDefaultValuesERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN18QCommandLineOptionC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK18QCommandLineOption13defaultValuesEv() -> i32;
}

// body block begin
// class sizeof(QCommandLineOption)=1
pub struct QCommandLineOption {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCommandLineOption {
  pub fn setValueName<T: QCommandLineOption_setValueName>(&mut self, value: T) -> i32 {
    value.setValueName(self);
    return 1;
  }
}

pub trait QCommandLineOption_setValueName {
  fn setValueName(self, this: &mut QCommandLineOption) -> i32;
}

// proto: void QCommandLineOption::setValueName(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_setValueName for (&'a  QString) {
  fn setValueName(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption12setValueNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOption12setValueNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn names<T: QCommandLineOption_names>(&mut self, value: T) -> i32 {
    value.names(self);
    return 1;
  }
}

pub trait QCommandLineOption_names {
  fn names(self, this: &mut QCommandLineOption) -> i32;
}

// proto: QStringList QCommandLineOption::names();
impl<'a> /*trait*/ QCommandLineOption_names for () {
  fn names(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption5namesEv()};
    unsafe {_ZNK18QCommandLineOption5namesEv()};
    return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn NewQCommandLineOption<T: QCommandLineOption_NewQCommandLineOption>(value: T) -> QCommandLineOption {
    let rsthis = value.NewQCommandLineOption();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_NewQCommandLineOption {
  fn NewQCommandLineOption(self) -> QCommandLineOption;
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QCommandLineOption) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOptionC1ERKS_(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn setDescription<T: QCommandLineOption_setDescription>(&mut self, value: T) -> i32 {
    value.setDescription(self);
    return 1;
  }
}

pub trait QCommandLineOption_setDescription {
  fn setDescription(self, this: &mut QCommandLineOption) -> i32;
}

// proto: void QCommandLineOption::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLineOption_setDescription for (&'a  QString) {
  fn setDescription(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOption14setDescriptionERK7QString(arg0)};
    return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn valueName<T: QCommandLineOption_valueName>(&mut self, value: T) -> i32 {
    value.valueName(self);
    return 1;
  }
}

pub trait QCommandLineOption_valueName {
  fn valueName(self, this: &mut QCommandLineOption) -> i32;
}

// proto: QString QCommandLineOption::valueName();
impl<'a> /*trait*/ QCommandLineOption_valueName for () {
  fn valueName(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption9valueNameEv()};
    unsafe {_ZNK18QCommandLineOption9valueNameEv()};
    return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QStringList, &'a  QString, &'a  QString, &'a  QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn swap<T: QCommandLineOption_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QCommandLineOption_swap {
  fn swap(self, this: &mut QCommandLineOption) -> i32;
}

// proto: void QCommandLineOption::swap(QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_swap for (&'a mut QCommandLineOption) {
  fn swap(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOption4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn description<T: QCommandLineOption_description>(&mut self, value: T) -> i32 {
    value.description(self);
    return 1;
  }
}

pub trait QCommandLineOption_description {
  fn description(self, this: &mut QCommandLineOption) -> i32;
}

// proto: QString QCommandLineOption::description();
impl<'a> /*trait*/ QCommandLineOption_description for () {
  fn description(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption11descriptionEv()};
    unsafe {_ZNK18QCommandLineOption11descriptionEv()};
    return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn FreeQCommandLineOption<T: QCommandLineOption_FreeQCommandLineOption>(&mut self, value: T) -> i32 {
    value.FreeQCommandLineOption(self);
    return 1;
  }
}

pub trait QCommandLineOption_FreeQCommandLineOption {
  fn FreeQCommandLineOption(self, this: &mut QCommandLineOption) -> i32;
}

// proto: void QCommandLineOption::FreeQCommandLineOption();
impl<'a> /*trait*/ QCommandLineOption_FreeQCommandLineOption for () {
  fn FreeQCommandLineOption(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionD0Ev()};
    unsafe {_ZN18QCommandLineOptionD0Ev()};
    return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QStringList & names);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QStringList) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK11QStringList(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValue<T: QCommandLineOption_setDefaultValue>(&mut self, value: T) -> i32 {
    value.setDefaultValue(self);
    return 1;
  }
}

pub trait QCommandLineOption_setDefaultValue {
  fn setDefaultValue(self, this: &mut QCommandLineOption) -> i32;
}

// proto: void QCommandLineOption::setDefaultValue(const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValue for (&'a  QString) {
  fn setDefaultValue(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption15setDefaultValueERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOption15setDefaultValueERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValues<T: QCommandLineOption_setDefaultValues>(&mut self, value: T) -> i32 {
    value.setDefaultValues(self);
    return 1;
  }
}

pub trait QCommandLineOption_setDefaultValues {
  fn setDefaultValues(self, this: &mut QCommandLineOption) -> i32;
}

// proto: void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValues for (&'a  QStringList) {
  fn setDefaultValues(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption16setDefaultValuesERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOption16setDefaultValuesERK11QStringList(arg0)};
    return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK7QString(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn defaultValues<T: QCommandLineOption_defaultValues>(&mut self, value: T) -> i32 {
    value.defaultValues(self);
    return 1;
  }
}

pub trait QCommandLineOption_defaultValues {
  fn defaultValues(self, this: &mut QCommandLineOption) -> i32;
}

// proto: QStringList QCommandLineOption::defaultValues();
impl<'a> /*trait*/ QCommandLineOption_defaultValues for () {
  fn defaultValues(self, this: &mut QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption13defaultValuesEv()};
    unsafe {_ZNK18QCommandLineOption13defaultValuesEv()};
    return 1;
  }
}

