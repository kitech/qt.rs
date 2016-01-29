// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworkconfiguration.h
// dst-file: /src/network/qnetworkconfiguration.rs
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
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkConfiguration_Class_Size() -> c_int;
  // proto:  void QNetworkConfiguration::QNetworkConfiguration();
  fn _ZN21QNetworkConfigurationC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkConfiguration::isValid();
  fn _ZNK21QNetworkConfiguration7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QNetworkConfiguration::isRoamingAvailable();
  fn _ZNK21QNetworkConfiguration18isRoamingAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QNetworkConfiguration::identifier();
  fn _ZNK21QNetworkConfiguration10identifierEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QNetworkConfiguration::bearerTypeName();
  fn _ZNK21QNetworkConfiguration14bearerTypeNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkConfiguration::~QNetworkConfiguration();
  fn _ZN21QNetworkConfigurationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkConfiguration::QNetworkConfiguration(const QNetworkConfiguration & other);
  fn _ZN21QNetworkConfigurationC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QNetworkConfiguration> QNetworkConfiguration::children();
  fn _ZNK21QNetworkConfiguration8childrenEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QNetworkConfiguration::name();
  fn _ZNK21QNetworkConfiguration4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkConfiguration::swap(QNetworkConfiguration & other);
  fn _ZN21QNetworkConfiguration4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkConfiguration)=1
#[derive(Default)]
pub struct QNetworkConfiguration {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkConfiguration {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkConfiguration {
    return QNetworkConfiguration{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QNetworkConfiguration::QNetworkConfiguration();
impl /*struct*/ QNetworkConfiguration {
  pub fn new<T: QNetworkConfiguration_new>(value: T) -> QNetworkConfiguration {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkConfiguration_new {
  fn new(self) -> QNetworkConfiguration;
}

  // proto:  void QNetworkConfiguration::QNetworkConfiguration();
impl<'a> /*trait*/ QNetworkConfiguration_new for () {
  fn new(self) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkConfigurationC2Ev()};
    let ctysz: c_int = unsafe{QNetworkConfiguration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN21QNetworkConfigurationC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkConfiguration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QNetworkConfiguration::isValid();
impl /*struct*/ QNetworkConfiguration {
  pub fn isValid<RetType, T: QNetworkConfiguration_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_isValid<RetType> {
  fn isValid(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  bool QNetworkConfiguration::isValid();
impl<'a> /*trait*/ QNetworkConfiguration_isValid<i8> for () {
  fn isValid(self , rsthis: & QNetworkConfiguration) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkConfiguration7isValidEv()};
    let mut ret = unsafe {_ZNK21QNetworkConfiguration7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QNetworkConfiguration::isRoamingAvailable();
impl /*struct*/ QNetworkConfiguration {
  pub fn isRoamingAvailable<RetType, T: QNetworkConfiguration_isRoamingAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRoamingAvailable(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_isRoamingAvailable<RetType> {
  fn isRoamingAvailable(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  bool QNetworkConfiguration::isRoamingAvailable();
impl<'a> /*trait*/ QNetworkConfiguration_isRoamingAvailable<i8> for () {
  fn isRoamingAvailable(self , rsthis: & QNetworkConfiguration) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkConfiguration18isRoamingAvailableEv()};
    let mut ret = unsafe {_ZNK21QNetworkConfiguration18isRoamingAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QNetworkConfiguration::identifier();
impl /*struct*/ QNetworkConfiguration {
  pub fn identifier<RetType, T: QNetworkConfiguration_identifier<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.identifier(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_identifier<RetType> {
  fn identifier(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  QString QNetworkConfiguration::identifier();
impl<'a> /*trait*/ QNetworkConfiguration_identifier<QString> for () {
  fn identifier(self , rsthis: & QNetworkConfiguration) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkConfiguration10identifierEv()};
    let mut ret = unsafe {_ZNK21QNetworkConfiguration10identifierEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QNetworkConfiguration::bearerTypeName();
impl /*struct*/ QNetworkConfiguration {
  pub fn bearerTypeName<RetType, T: QNetworkConfiguration_bearerTypeName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bearerTypeName(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_bearerTypeName<RetType> {
  fn bearerTypeName(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  QString QNetworkConfiguration::bearerTypeName();
impl<'a> /*trait*/ QNetworkConfiguration_bearerTypeName<QString> for () {
  fn bearerTypeName(self , rsthis: & QNetworkConfiguration) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkConfiguration14bearerTypeNameEv()};
    let mut ret = unsafe {_ZNK21QNetworkConfiguration14bearerTypeNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkConfiguration::~QNetworkConfiguration();
impl /*struct*/ QNetworkConfiguration {
  pub fn free<RetType, T: QNetworkConfiguration_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_free<RetType> {
  fn free(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  void QNetworkConfiguration::~QNetworkConfiguration();
impl<'a> /*trait*/ QNetworkConfiguration_free<()> for () {
  fn free(self , rsthis: & QNetworkConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkConfigurationD2Ev()};
     unsafe {_ZN21QNetworkConfigurationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkConfiguration::QNetworkConfiguration(const QNetworkConfiguration & other);
impl<'a> /*trait*/ QNetworkConfiguration_new for (&'a QNetworkConfiguration) {
  fn new(self) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkConfigurationC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkConfiguration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QNetworkConfigurationC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkConfiguration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QNetworkConfiguration> QNetworkConfiguration::children();
impl /*struct*/ QNetworkConfiguration {
  pub fn children<RetType, T: QNetworkConfiguration_children<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.children(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_children<RetType> {
  fn children(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  QList<QNetworkConfiguration> QNetworkConfiguration::children();
impl<'a> /*trait*/ QNetworkConfiguration_children<()> for () {
  fn children(self , rsthis: & QNetworkConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkConfiguration8childrenEv()};
     unsafe {_ZNK21QNetworkConfiguration8childrenEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QNetworkConfiguration::name();
impl /*struct*/ QNetworkConfiguration {
  pub fn name<RetType, T: QNetworkConfiguration_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_name<RetType> {
  fn name(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  QString QNetworkConfiguration::name();
impl<'a> /*trait*/ QNetworkConfiguration_name<QString> for () {
  fn name(self , rsthis: & QNetworkConfiguration) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QNetworkConfiguration4nameEv()};
    let mut ret = unsafe {_ZNK21QNetworkConfiguration4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkConfiguration::swap(QNetworkConfiguration & other);
impl /*struct*/ QNetworkConfiguration {
  pub fn swap<RetType, T: QNetworkConfiguration_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkConfiguration_swap<RetType> {
  fn swap(self , rsthis: & QNetworkConfiguration) -> RetType;
}

  // proto:  void QNetworkConfiguration::swap(QNetworkConfiguration & other);
impl<'a> /*trait*/ QNetworkConfiguration_swap<()> for (&'a QNetworkConfiguration) {
  fn swap(self , rsthis: & QNetworkConfiguration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QNetworkConfiguration4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QNetworkConfiguration4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

