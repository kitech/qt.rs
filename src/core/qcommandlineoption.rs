// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qcommandlineoption.h
// dst-file: /src/core/qcommandlineoption.rs
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
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qstringlist::QStringList; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCommandLineOption_Class_Size() -> c_int;
  // proto:  void QCommandLineOption::setValueName(const QString & name);
  fn _ZN18QCommandLineOption12setValueNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QCommandLineOption::names();
  fn _ZNK18QCommandLineOption5namesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QCommandLineOption::QCommandLineOption(const QCommandLineOption & other);
  fn dector_ZN18QCommandLineOptionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLineOptionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommandLineOption::setDescription(const QString & description);
  fn _ZN18QCommandLineOption14setDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
  fn dector_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  QString QCommandLineOption::valueName();
  fn _ZNK18QCommandLineOption9valueNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
  fn dector_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
  fn demth_ZN18QCommandLineOption4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QCommandLineOption::description();
  fn _ZNK18QCommandLineOption11descriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCommandLineOption::~QCommandLineOption();
  fn _ZN18QCommandLineOptionD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names);
  fn dector_ZN18QCommandLineOptionC1ERK11QStringList(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLineOptionC1ERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
  fn _ZN18QCommandLineOption15setDefaultValueERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
  fn _ZN18QCommandLineOption16setDefaultValuesERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name);
  fn dector_ZN18QCommandLineOptionC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLineOptionC1ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QCommandLineOption::defaultValues();
  fn _ZNK18QCommandLineOption13defaultValuesEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QCommandLineOption)=1
#[derive(Default)]
pub struct QCommandLineOption {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QCommandLineOption {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCommandLineOption {
    return QCommandLineOption{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QCommandLineOption::setValueName(const QString & name);
impl /*struct*/ QCommandLineOption {
  pub fn setValueName<RetType, T: QCommandLineOption_setValueName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValueName(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setValueName<RetType> {
  fn setValueName(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setValueName(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_setValueName<()> for (&'a QString) {
  fn setValueName(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption12setValueNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption12setValueNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QCommandLineOption::names();
impl /*struct*/ QCommandLineOption {
  pub fn names<RetType, T: QCommandLineOption_names<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.names(self);
    // return 1;
  }
}

pub trait QCommandLineOption_names<RetType> {
  fn names(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  QStringList QCommandLineOption::names();
impl<'a> /*trait*/ QCommandLineOption_names<()> for () {
  fn names(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption5namesEv()};
     unsafe {_ZNK18QCommandLineOption5namesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QCommandLineOption & other);
impl /*struct*/ QCommandLineOption {
  pub fn New<T: QCommandLineOption_New>(value: T) -> QCommandLineOption {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_New {
  fn New(self) -> QCommandLineOption;
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_New for (&'a QCommandLineOption) {
  fn New(self) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERKS_()};
    let ctysz: c_int = unsafe{QCommandLineOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLineOptionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QCommandLineOptionC1ERKS_(arg0)} as u64;
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::setDescription(const QString & description);
impl /*struct*/ QCommandLineOption {
  pub fn setDescription<RetType, T: QCommandLineOption_setDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDescription<RetType> {
  fn setDescription(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLineOption_setDescription<()> for (&'a QString) {
  fn setDescription(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_New for (&'a QString, &'a QString, &'a QString, &'a QString) {
  fn New(self) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_()};
    let ctysz: c_int = unsafe{QCommandLineOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis, arg0, arg1, arg2, arg3)};
    let qthis: u64 = unsafe {dector_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(arg0, arg1, arg2, arg3)} as u64;
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QCommandLineOption::valueName();
impl /*struct*/ QCommandLineOption {
  pub fn valueName<RetType, T: QCommandLineOption_valueName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueName(self);
    // return 1;
  }
}

pub trait QCommandLineOption_valueName<RetType> {
  fn valueName(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  QString QCommandLineOption::valueName();
impl<'a> /*trait*/ QCommandLineOption_valueName<QString> for () {
  fn valueName(self , rsthis: & QCommandLineOption) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption9valueNameEv()};
    let mut ret = unsafe {_ZNK18QCommandLineOption9valueNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_New for (&'a QStringList, &'a QString, &'a QString, &'a QString) {
  fn New(self) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_()};
    let ctysz: c_int = unsafe{QCommandLineOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis, arg0, arg1, arg2, arg3)};
    let qthis: u64 = unsafe {dector_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(arg0, arg1, arg2, arg3)} as u64;
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
impl /*struct*/ QCommandLineOption {
  pub fn swap<RetType, T: QCommandLineOption_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QCommandLineOption_swap<RetType> {
  fn swap(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_swap<()> for (&'a QCommandLineOption) {
  fn swap(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN18QCommandLineOption4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QCommandLineOption::description();
impl /*struct*/ QCommandLineOption {
  pub fn description<RetType, T: QCommandLineOption_description<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QCommandLineOption_description<RetType> {
  fn description(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  QString QCommandLineOption::description();
impl<'a> /*trait*/ QCommandLineOption_description<QString> for () {
  fn description(self , rsthis: & QCommandLineOption) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption11descriptionEv()};
    let mut ret = unsafe {_ZNK18QCommandLineOption11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::~QCommandLineOption();
impl /*struct*/ QCommandLineOption {
  pub fn Free<RetType, T: QCommandLineOption_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCommandLineOption_Free<RetType> {
  fn Free(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::~QCommandLineOption();
impl<'a> /*trait*/ QCommandLineOption_Free<()> for () {
  fn Free(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionD0Ev()};
     unsafe {_ZN18QCommandLineOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names);
impl<'a> /*trait*/ QCommandLineOption_New for (&'a QStringList) {
  fn New(self) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringList()};
    let ctysz: c_int = unsafe{QCommandLineOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLineOptionC1ERK11QStringList(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QCommandLineOptionC1ERK11QStringList(arg0)} as u64;
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValue<RetType, T: QCommandLineOption_setDefaultValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultValue(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDefaultValue<RetType> {
  fn setDefaultValue(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValue<()> for (&'a QString) {
  fn setDefaultValue(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption15setDefaultValueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption15setDefaultValueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValues<RetType, T: QCommandLineOption_setDefaultValues<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultValues(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDefaultValues<RetType> {
  fn setDefaultValues(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValues<()> for (&'a QStringList) {
  fn setDefaultValues(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption16setDefaultValuesERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption16setDefaultValuesERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_New for (&'a QString) {
  fn New(self) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QString()};
    let ctysz: c_int = unsafe{QCommandLineOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLineOptionC1ERK7QString(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QCommandLineOptionC1ERK7QString(arg0)} as u64;
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QCommandLineOption::defaultValues();
impl /*struct*/ QCommandLineOption {
  pub fn defaultValues<RetType, T: QCommandLineOption_defaultValues<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultValues(self);
    // return 1;
  }
}

pub trait QCommandLineOption_defaultValues<RetType> {
  fn defaultValues(self , rsthis: & QCommandLineOption) -> RetType;
}

  // proto:  QStringList QCommandLineOption::defaultValues();
impl<'a> /*trait*/ QCommandLineOption_defaultValues<()> for () {
  fn defaultValues(self , rsthis: & QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption13defaultValuesEv()};
     unsafe {_ZNK18QCommandLineOption13defaultValuesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

