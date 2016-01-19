// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworksession.h
// dst-file: /src/network/qnetworksession.rs
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
use super::super::core::qvariant::QVariant; // 771
use super::qnetworkconfiguration::QNetworkConfiguration; // 773
use super::qnetworkinterface::QNetworkInterface; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkSession_Class_Size() -> c_int;
  // proto:  bool QNetworkSession::waitForOpened(int msecs);
  fn _ZN15QNetworkSession13waitForOpenedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QNetworkSession::stop();
  fn _ZN15QNetworkSession4stopEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkSession::reject();
  fn _ZN15QNetworkSession6rejectEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QNetworkSession::metaObject();
  fn _ZNK15QNetworkSession10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkSession::close();
  fn _ZN15QNetworkSession5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QNetworkSession::errorString();
  fn _ZNK15QNetworkSession11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkSession::~QNetworkSession();
  fn _ZN15QNetworkSessionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkSession::open();
  fn _ZN15QNetworkSession4openEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkSession::setSessionProperty(const QString & key, const QVariant & value);
  fn _ZN15QNetworkSession18setSessionPropertyERK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QNetworkSession::QNetworkSession(const QNetworkConfiguration & connConfig, QObject * parent);
  fn _ZN15QNetworkSessionC2ERK21QNetworkConfigurationP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  quint64 QNetworkSession::activeTime();
  fn _ZNK15QNetworkSession10activeTimeEv(qthis: u64 /* *mut c_void*/) -> c_ulonglong;
  // proto:  QNetworkConfiguration QNetworkSession::configuration();
  fn _ZNK15QNetworkSession13configurationEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  quint64 QNetworkSession::bytesWritten();
  fn _ZNK15QNetworkSession12bytesWrittenEv(qthis: u64 /* *mut c_void*/) -> c_ulonglong;
  // proto:  void QNetworkSession::QNetworkSession(const QNetworkSession & );
  fn _ZN15QNetworkSessionC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QNetworkSession::isOpen();
  fn _ZNK15QNetworkSession6isOpenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  quint64 QNetworkSession::bytesReceived();
  fn _ZNK15QNetworkSession13bytesReceivedEv(qthis: u64 /* *mut c_void*/) -> c_ulonglong;
  // proto:  void QNetworkSession::accept();
  fn _ZN15QNetworkSession6acceptEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkSession::migrate();
  fn _ZN15QNetworkSession7migrateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkSession::ignore();
  fn _ZN15QNetworkSession6ignoreEv(qthis: u64 /* *mut c_void*/);
  // proto:  QNetworkInterface QNetworkSession::interface();
  fn _ZNK15QNetworkSession9interfaceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QNetworkSession::sessionProperty(const QString & key);
  fn _ZNK15QNetworkSession15sessionPropertyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession25newConfigurationActivatedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession20usagePoliciesChangedE6QFlagsINS_11UsagePolicyEE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession29preferredConfigurationChangedERK21QNetworkConfigurationb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession6closedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession12stateChangedENS_5StateE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession6openedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QNetworkSession_SlotProxy_connect__ZN15QNetworkSession5errorENS_12SessionErrorE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkSession)=1
#[derive(Default)]
pub struct QNetworkSession {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _preferredConfigurationChanged: QNetworkSession_preferredConfigurationChanged_signal,
  pub _opened: QNetworkSession_opened_signal,
  pub _closed: QNetworkSession_closed_signal,
  pub _error: QNetworkSession_error_signal,
  pub _newConfigurationActivated: QNetworkSession_newConfigurationActivated_signal,
  pub _usagePoliciesChanged: QNetworkSession_usagePoliciesChanged_signal,
  pub _stateChanged: QNetworkSession_stateChanged_signal,
}

impl /*struct*/ QNetworkSession {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkSession {
    return QNetworkSession{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNetworkSession {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QNetworkSession {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QNetworkSession::waitForOpened(int msecs);
impl /*struct*/ QNetworkSession {
  pub fn waitForOpened<RetType, T: QNetworkSession_waitForOpened<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForOpened(self);
    // return 1;
  }
}

pub trait QNetworkSession_waitForOpened<RetType> {
  fn waitForOpened(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  bool QNetworkSession::waitForOpened(int msecs);
impl<'a> /*trait*/ QNetworkSession_waitForOpened<i8> for (i32) {
  fn waitForOpened(self , rsthis: & QNetworkSession) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession13waitForOpenedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QNetworkSession13waitForOpenedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkSession::stop();
impl /*struct*/ QNetworkSession {
  pub fn stop<RetType, T: QNetworkSession_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QNetworkSession_stop<RetType> {
  fn stop(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::stop();
impl<'a> /*trait*/ QNetworkSession_stop<()> for () {
  fn stop(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession4stopEv()};
     unsafe {_ZN15QNetworkSession4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::reject();
impl /*struct*/ QNetworkSession {
  pub fn reject<RetType, T: QNetworkSession_reject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reject(self);
    // return 1;
  }
}

pub trait QNetworkSession_reject<RetType> {
  fn reject(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::reject();
impl<'a> /*trait*/ QNetworkSession_reject<()> for () {
  fn reject(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession6rejectEv()};
     unsafe {_ZN15QNetworkSession6rejectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QNetworkSession::metaObject();
impl /*struct*/ QNetworkSession {
  pub fn metaObject<RetType, T: QNetworkSession_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QNetworkSession_metaObject<RetType> {
  fn metaObject(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  const QMetaObject * QNetworkSession::metaObject();
impl<'a> /*trait*/ QNetworkSession_metaObject<()> for () {
  fn metaObject(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession10metaObjectEv()};
     unsafe {_ZNK15QNetworkSession10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::close();
impl /*struct*/ QNetworkSession {
  pub fn close<RetType, T: QNetworkSession_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QNetworkSession_close<RetType> {
  fn close(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::close();
impl<'a> /*trait*/ QNetworkSession_close<()> for () {
  fn close(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession5closeEv()};
     unsafe {_ZN15QNetworkSession5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QNetworkSession::errorString();
impl /*struct*/ QNetworkSession {
  pub fn errorString<RetType, T: QNetworkSession_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QNetworkSession_errorString<RetType> {
  fn errorString(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  QString QNetworkSession::errorString();
impl<'a> /*trait*/ QNetworkSession_errorString<QString> for () {
  fn errorString(self , rsthis: & QNetworkSession) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession11errorStringEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkSession::~QNetworkSession();
impl /*struct*/ QNetworkSession {
  pub fn free<RetType, T: QNetworkSession_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkSession_free<RetType> {
  fn free(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::~QNetworkSession();
impl<'a> /*trait*/ QNetworkSession_free<()> for () {
  fn free(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSessionD2Ev()};
     unsafe {_ZN15QNetworkSessionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::open();
impl /*struct*/ QNetworkSession {
  pub fn open<RetType, T: QNetworkSession_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QNetworkSession_open<RetType> {
  fn open(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::open();
impl<'a> /*trait*/ QNetworkSession_open<()> for () {
  fn open(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession4openEv()};
     unsafe {_ZN15QNetworkSession4openEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::setSessionProperty(const QString & key, const QVariant & value);
impl /*struct*/ QNetworkSession {
  pub fn setSessionProperty<RetType, T: QNetworkSession_setSessionProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSessionProperty(self);
    // return 1;
  }
}

pub trait QNetworkSession_setSessionProperty<RetType> {
  fn setSessionProperty(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::setSessionProperty(const QString & key, const QVariant & value);
impl<'a> /*trait*/ QNetworkSession_setSessionProperty<()> for (&'a QString, &'a QVariant) {
  fn setSessionProperty(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession18setSessionPropertyERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QNetworkSession18setSessionPropertyERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::QNetworkSession(const QNetworkConfiguration & connConfig, QObject * parent);
impl /*struct*/ QNetworkSession {
  pub fn new<T: QNetworkSession_new>(value: T) -> QNetworkSession {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkSession_new {
  fn new(self) -> QNetworkSession;
}

  // proto:  void QNetworkSession::QNetworkSession(const QNetworkConfiguration & connConfig, QObject * parent);
impl<'a> /*trait*/ QNetworkSession_new for (&'a QNetworkConfiguration, &'a QObject) {
  fn new(self) -> QNetworkSession {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSessionC2ERK21QNetworkConfigurationP7QObject()};
    let ctysz: c_int = unsafe{QNetworkSession_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QNetworkSessionC2ERK21QNetworkConfigurationP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkSession{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  quint64 QNetworkSession::activeTime();
impl /*struct*/ QNetworkSession {
  pub fn activeTime<RetType, T: QNetworkSession_activeTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeTime(self);
    // return 1;
  }
}

pub trait QNetworkSession_activeTime<RetType> {
  fn activeTime(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  quint64 QNetworkSession::activeTime();
impl<'a> /*trait*/ QNetworkSession_activeTime<u64> for () {
  fn activeTime(self , rsthis: & QNetworkSession) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession10activeTimeEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession10activeTimeEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QNetworkConfiguration QNetworkSession::configuration();
impl /*struct*/ QNetworkSession {
  pub fn configuration<RetType, T: QNetworkSession_configuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.configuration(self);
    // return 1;
  }
}

pub trait QNetworkSession_configuration<RetType> {
  fn configuration(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  QNetworkConfiguration QNetworkSession::configuration();
impl<'a> /*trait*/ QNetworkSession_configuration<QNetworkConfiguration> for () {
  fn configuration(self , rsthis: & QNetworkSession) -> QNetworkConfiguration {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession13configurationEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession13configurationEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkConfiguration::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  quint64 QNetworkSession::bytesWritten();
impl /*struct*/ QNetworkSession {
  pub fn bytesWritten<RetType, T: QNetworkSession_bytesWritten<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesWritten(self);
    // return 1;
  }
}

pub trait QNetworkSession_bytesWritten<RetType> {
  fn bytesWritten(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  quint64 QNetworkSession::bytesWritten();
impl<'a> /*trait*/ QNetworkSession_bytesWritten<u64> for () {
  fn bytesWritten(self , rsthis: & QNetworkSession) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession12bytesWrittenEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession12bytesWrittenEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  void QNetworkSession::QNetworkSession(const QNetworkSession & );
impl<'a> /*trait*/ QNetworkSession_new for (&'a QNetworkSession) {
  fn new(self) -> QNetworkSession {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSessionC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkSession_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QNetworkSessionC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkSession{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QNetworkSession::isOpen();
impl /*struct*/ QNetworkSession {
  pub fn isOpen<RetType, T: QNetworkSession_isOpen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOpen(self);
    // return 1;
  }
}

pub trait QNetworkSession_isOpen<RetType> {
  fn isOpen(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  bool QNetworkSession::isOpen();
impl<'a> /*trait*/ QNetworkSession_isOpen<i8> for () {
  fn isOpen(self , rsthis: & QNetworkSession) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession6isOpenEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession6isOpenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  quint64 QNetworkSession::bytesReceived();
impl /*struct*/ QNetworkSession {
  pub fn bytesReceived<RetType, T: QNetworkSession_bytesReceived<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesReceived(self);
    // return 1;
  }
}

pub trait QNetworkSession_bytesReceived<RetType> {
  fn bytesReceived(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  quint64 QNetworkSession::bytesReceived();
impl<'a> /*trait*/ QNetworkSession_bytesReceived<u64> for () {
  fn bytesReceived(self , rsthis: & QNetworkSession) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession13bytesReceivedEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession13bytesReceivedEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  void QNetworkSession::accept();
impl /*struct*/ QNetworkSession {
  pub fn accept<RetType, T: QNetworkSession_accept<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QNetworkSession_accept<RetType> {
  fn accept(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::accept();
impl<'a> /*trait*/ QNetworkSession_accept<()> for () {
  fn accept(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession6acceptEv()};
     unsafe {_ZN15QNetworkSession6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::migrate();
impl /*struct*/ QNetworkSession {
  pub fn migrate<RetType, T: QNetworkSession_migrate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.migrate(self);
    // return 1;
  }
}

pub trait QNetworkSession_migrate<RetType> {
  fn migrate(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::migrate();
impl<'a> /*trait*/ QNetworkSession_migrate<()> for () {
  fn migrate(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession7migrateEv()};
     unsafe {_ZN15QNetworkSession7migrateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkSession::ignore();
impl /*struct*/ QNetworkSession {
  pub fn ignore<RetType, T: QNetworkSession_ignore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QNetworkSession_ignore<RetType> {
  fn ignore(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  void QNetworkSession::ignore();
impl<'a> /*trait*/ QNetworkSession_ignore<()> for () {
  fn ignore(self , rsthis: & QNetworkSession) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QNetworkSession6ignoreEv()};
     unsafe {_ZN15QNetworkSession6ignoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QNetworkInterface QNetworkSession::interface();
impl /*struct*/ QNetworkSession {
  pub fn interface<RetType, T: QNetworkSession_interface<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.interface(self);
    // return 1;
  }
}

pub trait QNetworkSession_interface<RetType> {
  fn interface(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  QNetworkInterface QNetworkSession::interface();
impl<'a> /*trait*/ QNetworkSession_interface<QNetworkInterface> for () {
  fn interface(self , rsthis: & QNetworkSession) -> QNetworkInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession9interfaceEv()};
    let mut ret = unsafe {_ZNK15QNetworkSession9interfaceEv(rsthis.qclsinst)};
    let mut ret1 = QNetworkInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QNetworkSession::sessionProperty(const QString & key);
impl /*struct*/ QNetworkSession {
  pub fn sessionProperty<RetType, T: QNetworkSession_sessionProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionProperty(self);
    // return 1;
  }
}

pub trait QNetworkSession_sessionProperty<RetType> {
  fn sessionProperty(self , rsthis: & QNetworkSession) -> RetType;
}

  // proto:  QVariant QNetworkSession::sessionProperty(const QString & key);
impl<'a> /*trait*/ QNetworkSession_sessionProperty<QVariant> for (&'a QString) {
  fn sessionProperty(self , rsthis: & QNetworkSession) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QNetworkSession15sessionPropertyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QNetworkSession15sessionPropertyERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QNetworkSession_preferredConfigurationChanged
pub struct QNetworkSession_preferredConfigurationChanged_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn preferredConfigurationChanged(&self) -> QNetworkSession_preferredConfigurationChanged_signal {
     return QNetworkSession_preferredConfigurationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_preferredConfigurationChanged_signal {
  pub fn connect<T: QNetworkSession_preferredConfigurationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_preferredConfigurationChanged_signal_connect {
  fn connect(self, sigthis: QNetworkSession_preferredConfigurationChanged_signal);
}

#[derive(Default)] // for QNetworkSession_opened
pub struct QNetworkSession_opened_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn opened(&self) -> QNetworkSession_opened_signal {
     return QNetworkSession_opened_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_opened_signal {
  pub fn connect<T: QNetworkSession_opened_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_opened_signal_connect {
  fn connect(self, sigthis: QNetworkSession_opened_signal);
}

#[derive(Default)] // for QNetworkSession_closed
pub struct QNetworkSession_closed_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn closed(&self) -> QNetworkSession_closed_signal {
     return QNetworkSession_closed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_closed_signal {
  pub fn connect<T: QNetworkSession_closed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_closed_signal_connect {
  fn connect(self, sigthis: QNetworkSession_closed_signal);
}

#[derive(Default)] // for QNetworkSession_error
pub struct QNetworkSession_error_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn error(&self) -> QNetworkSession_error_signal {
     return QNetworkSession_error_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_error_signal {
  pub fn connect<T: QNetworkSession_error_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_error_signal_connect {
  fn connect(self, sigthis: QNetworkSession_error_signal);
}

#[derive(Default)] // for QNetworkSession_newConfigurationActivated
pub struct QNetworkSession_newConfigurationActivated_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn newConfigurationActivated(&self) -> QNetworkSession_newConfigurationActivated_signal {
     return QNetworkSession_newConfigurationActivated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_newConfigurationActivated_signal {
  pub fn connect<T: QNetworkSession_newConfigurationActivated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_newConfigurationActivated_signal_connect {
  fn connect(self, sigthis: QNetworkSession_newConfigurationActivated_signal);
}

#[derive(Default)] // for QNetworkSession_usagePoliciesChanged
pub struct QNetworkSession_usagePoliciesChanged_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn usagePoliciesChanged(&self) -> QNetworkSession_usagePoliciesChanged_signal {
     return QNetworkSession_usagePoliciesChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_usagePoliciesChanged_signal {
  pub fn connect<T: QNetworkSession_usagePoliciesChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_usagePoliciesChanged_signal_connect {
  fn connect(self, sigthis: QNetworkSession_usagePoliciesChanged_signal);
}

#[derive(Default)] // for QNetworkSession_stateChanged
pub struct QNetworkSession_stateChanged_signal{poi:u64}
impl /* struct */ QNetworkSession {
  pub fn stateChanged(&self) -> QNetworkSession_stateChanged_signal {
     return QNetworkSession_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QNetworkSession_stateChanged_signal {
  pub fn connect<T: QNetworkSession_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QNetworkSession_stateChanged_signal_connect {
  fn connect(self, sigthis: QNetworkSession_stateChanged_signal);
}

// newConfigurationActivated()
extern fn QNetworkSession_newConfigurationActivated_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkSession_newConfigurationActivated_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkSession_newConfigurationActivated_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkSession_newConfigurationActivated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_newConfigurationActivated_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession25newConfigurationActivatedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_newConfigurationActivated_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkSession_newConfigurationActivated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_newConfigurationActivated_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession25newConfigurationActivatedEv(arg0, arg1, arg2)};
  }
}
// usagePoliciesChanged(class QNetworkSession::UsagePolicies)
extern fn QNetworkSession_usagePoliciesChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QNetworkSession_usagePoliciesChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkSession_usagePoliciesChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QNetworkSession_usagePoliciesChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_usagePoliciesChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession20usagePoliciesChangedE6QFlagsINS_11UsagePolicyEE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_usagePoliciesChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QNetworkSession_usagePoliciesChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_usagePoliciesChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession20usagePoliciesChangedE6QFlagsINS_11UsagePolicyEE(arg0, arg1, arg2)};
  }
}
// preferredConfigurationChanged(const class QNetworkConfiguration &, _Bool)
extern fn QNetworkSession_preferredConfigurationChanged_signal_connect_cb_2(rsfptr:fn(QNetworkConfiguration, i8), arg0: *mut c_void, arg1: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  let rsarg1 = arg1 as i8;
  rsfptr(rsarg0,rsarg1);
}
extern fn QNetworkSession_preferredConfigurationChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QNetworkConfiguration, i8)>, arg0: *mut c_void, arg1: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QNetworkConfiguration::inheritFrom(arg0 as u64);
  let rsarg1 = arg1 as i8;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QNetworkSession_preferredConfigurationChanged_signal_connect for fn(QNetworkConfiguration, i8) {
  fn connect(self, sigthis: QNetworkSession_preferredConfigurationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_preferredConfigurationChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession29preferredConfigurationChangedERK21QNetworkConfigurationb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_preferredConfigurationChanged_signal_connect for Box<Fn(QNetworkConfiguration, i8)> {
  fn connect(self, sigthis: QNetworkSession_preferredConfigurationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_preferredConfigurationChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession29preferredConfigurationChangedERK21QNetworkConfigurationb(arg0, arg1, arg2)};
  }
}
// closed()
extern fn QNetworkSession_closed_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkSession_closed_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkSession_closed_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkSession_closed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_closed_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession6closedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_closed_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkSession_closed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_closed_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession6closedEv(arg0, arg1, arg2)};
  }
}
// stateChanged(class QNetworkSession::State)
extern fn QNetworkSession_stateChanged_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QNetworkSession_stateChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkSession_stateChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QNetworkSession_stateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_stateChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession12stateChangedENS_5StateE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_stateChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QNetworkSession_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_stateChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession12stateChangedENS_5StateE(arg0, arg1, arg2)};
  }
}
// opened()
extern fn QNetworkSession_opened_signal_connect_cb_5(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QNetworkSession_opened_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QNetworkSession_opened_signal_connect for fn() {
  fn connect(self, sigthis: QNetworkSession_opened_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_opened_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession6openedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_opened_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QNetworkSession_opened_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_opened_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession6openedEv(arg0, arg1, arg2)};
  }
}
// error(class QNetworkSession::SessionError)
extern fn QNetworkSession_error_signal_connect_cb_6(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QNetworkSession_error_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QNetworkSession_error_signal_connect for fn(i32) {
  fn connect(self, sigthis: QNetworkSession_error_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_error_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession5errorENS_12SessionErrorE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QNetworkSession_error_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QNetworkSession_error_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QNetworkSession_error_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QNetworkSession_SlotProxy_connect__ZN15QNetworkSession5errorENS_12SessionErrorE(arg0, arg1, arg2)};
  }
}
// <= body block end

