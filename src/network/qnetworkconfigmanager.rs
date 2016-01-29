// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworkconfigmanager.h
// dst-file: /src/network/qnetworkconfigmanager.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qnetworkconfiguration::QNetworkConfiguration; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkConfigurationManager_Class_Size() -> c_int;
  // proto:  bool QNetworkConfigurationManager::isOnline();
  fn _ZNK28QNetworkConfigurationManager8isOnlineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QNetworkConfiguration QNetworkConfigurationManager::configurationFromIdentifier(const QString & identifier);
  fn _ZNK28QNetworkConfigurationManager27configurationFromIdentifierERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QNetworkConfigurationManager::updateConfigurations();
  fn _ZN28QNetworkConfigurationManager20updateConfigurationsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkConfigurationManager::QNetworkConfigurationManager(const QNetworkConfigurationManager & );
  fn _ZN28QNetworkConfigurationManagerC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QNetworkConfiguration QNetworkConfigurationManager::defaultConfiguration();
  fn _ZNK28QNetworkConfigurationManager20defaultConfigurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkConfigurationManager::QNetworkConfigurationManager(QObject * parent);
  fn _ZN28QNetworkConfigurationManagerC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QNetworkConfigurationManager::metaObject();
  fn _ZNK28QNetworkConfigurationManager10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkConfigurationManager::~QNetworkConfigurationManager();
  fn _ZN28QNetworkConfigurationManagerD2Ev(qthis: u64 /* *mut c_void*/);
  fn QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager18onlineStateChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager15updateCompletedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager20configurationChangedERK21QNetworkConfiguration(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager20configurationRemovedERK21QNetworkConfiguration(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager18configurationAddedERK21QNetworkConfiguration(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkConfigurationManager)=1
#[derive(Default)]
pub struct QNetworkConfigurationManager {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _configurationAdded: QNetworkConfigurationManager_configurationAdded_signal,
  pub _onlineStateChanged: QNetworkConfigurationManager_onlineStateChanged_signal,
  pub _configurationRemoved: QNetworkConfigurationManager_configurationRemoved_signal,
  pub _updateCompleted: QNetworkConfigurationManager_updateCompleted_signal,
  pub _configurationChanged: QNetworkConfigurationManager_configurationChanged_signal,
}

impl /*struct*/ QNetworkConfigurationManager {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkConfigurationManager {
    return QNetworkConfigurationManager{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNetworkConfigurationManager {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QNetworkConfigurationManager {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QNetworkConfigurationManager::isOnline();
impl /*struct*/ QNetworkConfigurationManager {
  pub fn isOnline<RetType, T: QNetworkConfigurationManager_isOnline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOnline(self);
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_isOnline<RetType> {
  fn isOnline(self , rsthis: & QNetworkConfigurationManager) -> RetType;
}

  // proto:  bool QNetworkConfigurationManager::isOnline();
impl<'a> /*trait*/ QNetworkConfigurationManager_isOnline<i8> for () {
  fn isOnline(self , rsthis: & QNetworkConfigurationManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK28QNetworkConfigurationManager8isOnlineEv()};
    let mut ret = unsafe {_ZNK28QNetworkConfigurationManager8isOnlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QNetworkConfiguration QNetworkConfigurationManager::configurationFromIdentifier(const QString & identifier);
impl /*struct*/ QNetworkConfigurationManager {
  pub fn configurationFromIdentifier<RetType, T: QNetworkConfigurationManager_configurationFromIdentifier<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.configurationFromIdentifier(self);
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_configurationFromIdentifier<RetType> {
  fn configurationFromIdentifier(self , rsthis: & QNetworkConfigurationManager) -> RetType;
}

  // proto:  QNetworkConfiguration QNetworkConfigurationManager::configurationFromIdentifier(const QString & identifier);
impl<'a> /*trait*/ QNetworkConfigurationManager_configurationFromIdentifier<QNetworkConfiguration> for (&'a QString) {
  fn configurationFromIdentifier(self , rsthis: & QNetworkConfigurationManager) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK28QNetworkConfigurationManager27configurationFromIdentifierERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK28QNetworkConfigurationManager27configurationFromIdentifierERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkConfigurationManager::updateConfigurations();
impl /*struct*/ QNetworkConfigurationManager {
  pub fn updateConfigurations<RetType, T: QNetworkConfigurationManager_updateConfigurations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateConfigurations(self);
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_updateConfigurations<RetType> {
  fn updateConfigurations(self , rsthis: & QNetworkConfigurationManager) -> RetType;
}

  // proto:  void QNetworkConfigurationManager::updateConfigurations();
impl<'a> /*trait*/ QNetworkConfigurationManager_updateConfigurations<()> for () {
  fn updateConfigurations(self , rsthis: & QNetworkConfigurationManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN28QNetworkConfigurationManager20updateConfigurationsEv()};
     unsafe {_ZN28QNetworkConfigurationManager20updateConfigurationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkConfigurationManager::QNetworkConfigurationManager(const QNetworkConfigurationManager & );
impl /*struct*/ QNetworkConfigurationManager {
  pub fn new<T: QNetworkConfigurationManager_new>(value: T) -> QNetworkConfigurationManager {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_new {
  fn new(self) -> QNetworkConfigurationManager;
}

  // proto:  void QNetworkConfigurationManager::QNetworkConfigurationManager(const QNetworkConfigurationManager & );
impl<'a> /*trait*/ QNetworkConfigurationManager_new for (&'a QNetworkConfigurationManager) {
  fn new(self) -> QNetworkConfigurationManager {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN28QNetworkConfigurationManagerC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkConfigurationManager_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN28QNetworkConfigurationManagerC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkConfigurationManager{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QNetworkConfiguration QNetworkConfigurationManager::defaultConfiguration();
impl /*struct*/ QNetworkConfigurationManager {
  pub fn defaultConfiguration<RetType, T: QNetworkConfigurationManager_defaultConfiguration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultConfiguration(self);
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_defaultConfiguration<RetType> {
  fn defaultConfiguration(self , rsthis: & QNetworkConfigurationManager) -> RetType;
}

  // proto:  QNetworkConfiguration QNetworkConfigurationManager::defaultConfiguration();
impl<'a> /*trait*/ QNetworkConfigurationManager_defaultConfiguration<QNetworkConfiguration> for () {
  fn defaultConfiguration(self , rsthis: & QNetworkConfigurationManager) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK28QNetworkConfigurationManager20defaultConfigurationEv()};
    let mut ret = unsafe {_ZNK28QNetworkConfigurationManager20defaultConfigurationEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkConfigurationManager::QNetworkConfigurationManager(QObject * parent);
impl<'a> /*trait*/ QNetworkConfigurationManager_new for (&'a QObject) {
  fn new(self) -> QNetworkConfigurationManager {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN28QNetworkConfigurationManagerC2EP7QObject()};
    let ctysz: c_int = unsafe{QNetworkConfigurationManager_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN28QNetworkConfigurationManagerC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkConfigurationManager{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QNetworkConfigurationManager::metaObject();
impl /*struct*/ QNetworkConfigurationManager {
  pub fn metaObject<RetType, T: QNetworkConfigurationManager_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_metaObject<RetType> {
  fn metaObject(self , rsthis: & QNetworkConfigurationManager) -> RetType;
}

  // proto:  const QMetaObject * QNetworkConfigurationManager::metaObject();
impl<'a> /*trait*/ QNetworkConfigurationManager_metaObject<()> for () {
  fn metaObject(self , rsthis: & QNetworkConfigurationManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK28QNetworkConfigurationManager10metaObjectEv()};
     unsafe {_ZNK28QNetworkConfigurationManager10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkConfigurationManager::~QNetworkConfigurationManager();
impl /*struct*/ QNetworkConfigurationManager {
  pub fn free<RetType, T: QNetworkConfigurationManager_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkConfigurationManager_free<RetType> {
  fn free(self , rsthis: & QNetworkConfigurationManager) -> RetType;
}

  // proto:  void QNetworkConfigurationManager::~QNetworkConfigurationManager();
impl<'a> /*trait*/ QNetworkConfigurationManager_free<()> for () {
  fn free(self , rsthis: & QNetworkConfigurationManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN28QNetworkConfigurationManagerD2Ev()};
     unsafe {_ZN28QNetworkConfigurationManagerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QNetworkConfigurationManager_configurationAdded
pub struct QNetworkConfigurationManager_configurationAdded_signal{poi:u64}
impl /* struct */ QNetworkConfigurationManager {
  pub fn configurationAdded(&self) -> QNetworkConfigurationManager_configurationAdded_signal {
     return QNetworkConfigurationManager_configurationAdded_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkConfigurationManager_configurationAdded_signal {
  pub fn connect<T: QNetworkConfigurationManager_configurationAdded_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkConfigurationManager_configurationAdded_signal_connect {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationAdded_signal);
}

#[derive(Default)] // for QNetworkConfigurationManager_onlineStateChanged
pub struct QNetworkConfigurationManager_onlineStateChanged_signal{poi:u64}
impl /* struct */ QNetworkConfigurationManager {
  pub fn onlineStateChanged(&self) -> QNetworkConfigurationManager_onlineStateChanged_signal {
     return QNetworkConfigurationManager_onlineStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkConfigurationManager_onlineStateChanged_signal {
  pub fn connect<T: QNetworkConfigurationManager_onlineStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkConfigurationManager_onlineStateChanged_signal_connect {
  fn connect(self, sigthis: QNetworkConfigurationManager_onlineStateChanged_signal);
}

#[derive(Default)] // for QNetworkConfigurationManager_configurationRemoved
pub struct QNetworkConfigurationManager_configurationRemoved_signal{poi:u64}
impl /* struct */ QNetworkConfigurationManager {
  pub fn configurationRemoved(&self) -> QNetworkConfigurationManager_configurationRemoved_signal {
     return QNetworkConfigurationManager_configurationRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkConfigurationManager_configurationRemoved_signal {
  pub fn connect<T: QNetworkConfigurationManager_configurationRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkConfigurationManager_configurationRemoved_signal_connect {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationRemoved_signal);
}

#[derive(Default)] // for QNetworkConfigurationManager_updateCompleted
pub struct QNetworkConfigurationManager_updateCompleted_signal{poi:u64}
impl /* struct */ QNetworkConfigurationManager {
  pub fn updateCompleted(&self) -> QNetworkConfigurationManager_updateCompleted_signal {
     return QNetworkConfigurationManager_updateCompleted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkConfigurationManager_updateCompleted_signal {
  pub fn connect<T: QNetworkConfigurationManager_updateCompleted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkConfigurationManager_updateCompleted_signal_connect {
  fn connect(self, sigthis: QNetworkConfigurationManager_updateCompleted_signal);
}

#[derive(Default)] // for QNetworkConfigurationManager_configurationChanged
pub struct QNetworkConfigurationManager_configurationChanged_signal{poi:u64}
impl /* struct */ QNetworkConfigurationManager {
  pub fn configurationChanged(&self) -> QNetworkConfigurationManager_configurationChanged_signal {
     return QNetworkConfigurationManager_configurationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkConfigurationManager_configurationChanged_signal {
  pub fn connect<T: QNetworkConfigurationManager_configurationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkConfigurationManager_configurationChanged_signal_connect {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationChanged_signal);
}

// onlineStateChanged(_Bool)
extern fn QNetworkConfigurationManager_onlineStateChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QNetworkConfigurationManager_onlineStateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkConfigurationManager_onlineStateChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QNetworkConfigurationManager_onlineStateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_onlineStateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager18onlineStateChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkConfigurationManager_onlineStateChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QNetworkConfigurationManager_onlineStateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_onlineStateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager18onlineStateChangedEb(arg0, arg1, arg2)};
  }
}
// updateCompleted()
extern fn QNetworkConfigurationManager_updateCompleted_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkConfigurationManager_updateCompleted_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkConfigurationManager_updateCompleted_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkConfigurationManager_updateCompleted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_updateCompleted_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager15updateCompletedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkConfigurationManager_updateCompleted_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkConfigurationManager_updateCompleted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_updateCompleted_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager15updateCompletedEv(arg0, arg1, arg2)};
  }
}
// configurationChanged(const class QNetworkConfiguration &)
extern fn QNetworkConfigurationManager_configurationChanged_signal_connect_cb_2(rsfptr:fn(QNetworkConfiguration), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QNetworkConfigurationManager_configurationChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QNetworkConfiguration)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkConfigurationManager_configurationChanged_signal_connect for fn(QNetworkConfiguration) {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_configurationChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager20configurationChangedERK21QNetworkConfiguration(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkConfigurationManager_configurationChanged_signal_connect for Box<Fn(QNetworkConfiguration)> {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_configurationChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager20configurationChangedERK21QNetworkConfiguration(arg0, arg1, arg2)};
  }
}
// configurationRemoved(const class QNetworkConfiguration &)
extern fn QNetworkConfigurationManager_configurationRemoved_signal_connect_cb_3(rsfptr:fn(QNetworkConfiguration), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QNetworkConfigurationManager_configurationRemoved_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QNetworkConfiguration)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkConfigurationManager_configurationRemoved_signal_connect for fn(QNetworkConfiguration) {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationRemoved_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_configurationRemoved_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager20configurationRemovedERK21QNetworkConfiguration(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkConfigurationManager_configurationRemoved_signal_connect for Box<Fn(QNetworkConfiguration)> {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationRemoved_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_configurationRemoved_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager20configurationRemovedERK21QNetworkConfiguration(arg0, arg1, arg2)};
  }
}
// configurationAdded(const class QNetworkConfiguration &)
extern fn QNetworkConfigurationManager_configurationAdded_signal_connect_cb_4(rsfptr:fn(QNetworkConfiguration), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QNetworkConfigurationManager_configurationAdded_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QNetworkConfiguration)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkConfigurationManager_configurationAdded_signal_connect for fn(QNetworkConfiguration) {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationAdded_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_configurationAdded_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager18configurationAddedERK21QNetworkConfiguration(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkConfigurationManager_configurationAdded_signal_connect for Box<Fn(QNetworkConfiguration)> {
  fn connect(self, sigthis: QNetworkConfigurationManager_configurationAdded_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkConfigurationManager_configurationAdded_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkConfigurationManager_SlotProxy_connect__ZN28QNetworkConfigurationManager18configurationAddedERK21QNetworkConfiguration(arg0, arg1, arg2)};
  }
}
// <= body block end

