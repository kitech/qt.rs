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
  // proto:  void QCommandLineOption::setValueName(const QString & name);
  fn _ZN18QCommandLineOption12setValueNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QCommandLineOption::names();
  fn _ZNK18QCommandLineOption5namesEv(qthis: *mut c_void) ;
  // proto:  void QCommandLineOption::NewQCommandLineOption(const QCommandLineOption & other);
  fn _ZN18QCommandLineOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCommandLineOption::setDescription(const QString & description);
  fn _ZN18QCommandLineOption14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCommandLineOption::NewQCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
  fn _ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  QString QCommandLineOption::valueName();
  fn _ZNK18QCommandLineOption9valueNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCommandLineOption::NewQCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
  fn _ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
  fn _ZN18QCommandLineOption4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QCommandLineOption::description();
  fn _ZNK18QCommandLineOption11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCommandLineOption::FreeQCommandLineOption();
  fn _ZN18QCommandLineOptionD0Ev(qthis: *mut c_void) ;
  // proto:  void QCommandLineOption::NewQCommandLineOption(const QStringList & names);
  fn _ZN18QCommandLineOptionC1ERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
  fn _ZN18QCommandLineOption15setDefaultValueERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
  fn _ZN18QCommandLineOption16setDefaultValuesERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCommandLineOption::NewQCommandLineOption(const QString & name);
  fn _ZN18QCommandLineOptionC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QCommandLineOption::defaultValues();
  fn _ZNK18QCommandLineOption13defaultValuesEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QCommandLineOption)=1
pub struct QCommandLineOption {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCommandLineOption {
  pub fn setValueName<T: QCommandLineOption_setValueName>(&mut self, value: T)  {
     value.setValueName(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setValueName {
  fn setValueName(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  void QCommandLineOption::setValueName(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_setValueName for (&'a  QString) {
  fn setValueName(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption12setValueNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption12setValueNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn names<T: QCommandLineOption_names>(&mut self, value: T)  {
     value.names(self);
    // return 1;
  }
}

pub trait QCommandLineOption_names {
  fn names(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  QStringList QCommandLineOption::names();
impl<'a> /*trait*/ QCommandLineOption_names for () {
  fn names(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption5namesEv()};
     unsafe {_ZNK18QCommandLineOption5namesEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERKS_(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn setDescription<T: QCommandLineOption_setDescription>(&mut self, value: T)  {
     value.setDescription(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDescription {
  fn setDescription(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  void QCommandLineOption::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLineOption_setDescription for (&'a  QString) {
  fn setDescription(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn valueName<T: QCommandLineOption_valueName>(&mut self, value: T) -> QString {
    return value.valueName(self);
    // return 1;
  }
}

pub trait QCommandLineOption_valueName {
  fn valueName(self, rsthis: &mut QCommandLineOption) -> QString;
}

// proto:  QString QCommandLineOption::valueName();
impl<'a> /*trait*/ QCommandLineOption_valueName for () {
  fn valueName(self, rsthis: &mut QCommandLineOption) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption9valueNameEv()};
    let mut ret = unsafe {_ZNK18QCommandLineOption9valueNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QStringList, &'a  QString, &'a  QString, &'a  QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn swap<T: QCommandLineOption_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QCommandLineOption_swap {
  fn swap(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  void QCommandLineOption::swap(QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_swap for (&'a mut QCommandLineOption) {
  fn swap(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn description<T: QCommandLineOption_description>(&mut self, value: T) -> QString {
    return value.description(self);
    // return 1;
  }
}

pub trait QCommandLineOption_description {
  fn description(self, rsthis: &mut QCommandLineOption) -> QString;
}

// proto:  QString QCommandLineOption::description();
impl<'a> /*trait*/ QCommandLineOption_description for () {
  fn description(self, rsthis: &mut QCommandLineOption) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption11descriptionEv()};
    let mut ret = unsafe {_ZNK18QCommandLineOption11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn FreeQCommandLineOption<T: QCommandLineOption_FreeQCommandLineOption>(&mut self, value: T)  {
     value.FreeQCommandLineOption(self);
    // return 1;
  }
}

pub trait QCommandLineOption_FreeQCommandLineOption {
  fn FreeQCommandLineOption(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  void QCommandLineOption::FreeQCommandLineOption();
impl<'a> /*trait*/ QCommandLineOption_FreeQCommandLineOption for () {
  fn FreeQCommandLineOption(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionD0Ev()};
     unsafe {_ZN18QCommandLineOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QStringList & names);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QStringList) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK11QStringList(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValue<T: QCommandLineOption_setDefaultValue>(&mut self, value: T)  {
     value.setDefaultValue(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDefaultValue {
  fn setDefaultValue(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValue for (&'a  QString) {
  fn setDefaultValue(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption15setDefaultValueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption15setDefaultValueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValues<T: QCommandLineOption_setDefaultValues>(&mut self, value: T)  {
     value.setDefaultValues(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDefaultValues {
  fn setDefaultValues(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValues for (&'a  QStringList) {
  fn setDefaultValues(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption16setDefaultValuesERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption16setDefaultValuesERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QCommandLineOption::NewQCommandLineOption(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (&'a  QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK7QString(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLineOption {
  pub fn defaultValues<T: QCommandLineOption_defaultValues>(&mut self, value: T)  {
     value.defaultValues(self);
    // return 1;
  }
}

pub trait QCommandLineOption_defaultValues {
  fn defaultValues(self, rsthis: &mut QCommandLineOption) ;
}

// proto:  QStringList QCommandLineOption::defaultValues();
impl<'a> /*trait*/ QCommandLineOption_defaultValues for () {
  fn defaultValues(self, rsthis: &mut QCommandLineOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption13defaultValuesEv()};
     unsafe {_ZNK18QCommandLineOption13defaultValuesEv(rsthis.qclsinst)};
    // return 1;
  }
}

