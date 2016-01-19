// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qhostinfo.h
// dst-file: /src/network/qhostinfo.rs
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
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QHostInfo_Class_Size() -> c_int;
  // proto: static QString QHostInfo::localHostName();
  fn _ZN9QHostInfo13localHostNameEv() -> *mut c_void;
  // proto:  void QHostInfo::setHostName(const QString & name);
  fn _ZN9QHostInfo11setHostNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QHostInfo::lookupId();
  fn _ZNK9QHostInfo8lookupIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHostInfo::QHostInfo(const QHostInfo & d);
  fn _ZN9QHostInfoC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static int QHostInfo::lookupHost(const QString & name, QObject * receiver, const char * member);
  fn _ZN9QHostInfo10lookupHostERK7QStringP7QObjectPKc(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char) -> c_int;
  // proto:  QList<QHostAddress> QHostInfo::addresses();
  fn _ZNK9QHostInfo9addressesEv(qthis: u64 /* *mut c_void*/);
  // proto: static void QHostInfo::abortHostLookup(int lookupId);
  fn _ZN9QHostInfo15abortHostLookupEi(arg0: c_int);
  // proto:  QString QHostInfo::errorString();
  fn _ZNK9QHostInfo11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QHostInfo::~QHostInfo();
  fn _ZN9QHostInfoD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QHostInfo::setLookupId(int id);
  fn _ZN9QHostInfo11setLookupIdEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto: static QString QHostInfo::localDomainName();
  fn _ZN9QHostInfo15localDomainNameEv() -> *mut c_void;
  // proto:  void QHostInfo::setErrorString(const QString & errorString);
  fn _ZN9QHostInfo14setErrorStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QHostInfo::hostName();
  fn _ZNK9QHostInfo8hostNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QHostInfo QHostInfo::fromName(const QString & name);
  fn _ZN9QHostInfo8fromNameERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QHostInfo::QHostInfo(int lookupId);
  fn _ZN9QHostInfoC2Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QHostInfo)=1
#[derive(Default)]
pub struct QHostInfo {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QHostInfo {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHostInfo {
    return QHostInfo{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QString QHostInfo::localHostName();
impl /*struct*/ QHostInfo {
  pub fn localHostName_s<RetType, T: QHostInfo_localHostName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.localHostName_s();
    // return 1;
  }
}

pub trait QHostInfo_localHostName_s<RetType> {
  fn localHostName_s(self ) -> RetType;
}

  // proto: static QString QHostInfo::localHostName();
impl<'a> /*trait*/ QHostInfo_localHostName_s<QString> for () {
  fn localHostName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo13localHostNameEv()};
    let mut ret = unsafe {_ZN9QHostInfo13localHostNameEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QHostInfo::setHostName(const QString & name);
impl /*struct*/ QHostInfo {
  pub fn setHostName<RetType, T: QHostInfo_setHostName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHostName(self);
    // return 1;
  }
}

pub trait QHostInfo_setHostName<RetType> {
  fn setHostName(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  void QHostInfo::setHostName(const QString & name);
impl<'a> /*trait*/ QHostInfo_setHostName<()> for (&'a QString) {
  fn setHostName(self , rsthis: & QHostInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo11setHostNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QHostInfo11setHostNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHostInfo::lookupId();
impl /*struct*/ QHostInfo {
  pub fn lookupId<RetType, T: QHostInfo_lookupId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lookupId(self);
    // return 1;
  }
}

pub trait QHostInfo_lookupId<RetType> {
  fn lookupId(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  int QHostInfo::lookupId();
impl<'a> /*trait*/ QHostInfo_lookupId<i32> for () {
  fn lookupId(self , rsthis: & QHostInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QHostInfo8lookupIdEv()};
    let mut ret = unsafe {_ZNK9QHostInfo8lookupIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHostInfo::QHostInfo(const QHostInfo & d);
impl /*struct*/ QHostInfo {
  pub fn new<T: QHostInfo_new>(value: T) -> QHostInfo {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QHostInfo_new {
  fn new(self) -> QHostInfo;
}

  // proto:  void QHostInfo::QHostInfo(const QHostInfo & d);
impl<'a> /*trait*/ QHostInfo_new for (&'a QHostInfo) {
  fn new(self) -> QHostInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfoC2ERKS_()};
    let ctysz: c_int = unsafe{QHostInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QHostInfoC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static int QHostInfo::lookupHost(const QString & name, QObject * receiver, const char * member);
impl /*struct*/ QHostInfo {
  pub fn lookupHost_s<RetType, T: QHostInfo_lookupHost_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.lookupHost_s();
    // return 1;
  }
}

pub trait QHostInfo_lookupHost_s<RetType> {
  fn lookupHost_s(self ) -> RetType;
}

  // proto: static int QHostInfo::lookupHost(const QString & name, QObject * receiver, const char * member);
impl<'a> /*trait*/ QHostInfo_lookupHost_s<i32> for (&'a QString, &'a QObject, &'a  String) {
  fn lookupHost_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo10lookupHostERK7QStringP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN9QHostInfo10lookupHostERK7QStringP7QObjectPKc(arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QList<QHostAddress> QHostInfo::addresses();
impl /*struct*/ QHostInfo {
  pub fn addresses<RetType, T: QHostInfo_addresses<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addresses(self);
    // return 1;
  }
}

pub trait QHostInfo_addresses<RetType> {
  fn addresses(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  QList<QHostAddress> QHostInfo::addresses();
impl<'a> /*trait*/ QHostInfo_addresses<()> for () {
  fn addresses(self , rsthis: & QHostInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QHostInfo9addressesEv()};
     unsafe {_ZNK9QHostInfo9addressesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QHostInfo::abortHostLookup(int lookupId);
impl /*struct*/ QHostInfo {
  pub fn abortHostLookup_s<RetType, T: QHostInfo_abortHostLookup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.abortHostLookup_s();
    // return 1;
  }
}

pub trait QHostInfo_abortHostLookup_s<RetType> {
  fn abortHostLookup_s(self ) -> RetType;
}

  // proto: static void QHostInfo::abortHostLookup(int lookupId);
impl<'a> /*trait*/ QHostInfo_abortHostLookup_s<()> for (i32) {
  fn abortHostLookup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo15abortHostLookupEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QHostInfo15abortHostLookupEi(arg0)};
    // return 1;
  }
}

  // proto:  QString QHostInfo::errorString();
impl /*struct*/ QHostInfo {
  pub fn errorString<RetType, T: QHostInfo_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QHostInfo_errorString<RetType> {
  fn errorString(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  QString QHostInfo::errorString();
impl<'a> /*trait*/ QHostInfo_errorString<QString> for () {
  fn errorString(self , rsthis: & QHostInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QHostInfo11errorStringEv()};
    let mut ret = unsafe {_ZNK9QHostInfo11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QHostInfo::~QHostInfo();
impl /*struct*/ QHostInfo {
  pub fn free<RetType, T: QHostInfo_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHostInfo_free<RetType> {
  fn free(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  void QHostInfo::~QHostInfo();
impl<'a> /*trait*/ QHostInfo_free<()> for () {
  fn free(self , rsthis: & QHostInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfoD2Ev()};
     unsafe {_ZN9QHostInfoD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHostInfo::setLookupId(int id);
impl /*struct*/ QHostInfo {
  pub fn setLookupId<RetType, T: QHostInfo_setLookupId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLookupId(self);
    // return 1;
  }
}

pub trait QHostInfo_setLookupId<RetType> {
  fn setLookupId(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  void QHostInfo::setLookupId(int id);
impl<'a> /*trait*/ QHostInfo_setLookupId<()> for (i32) {
  fn setLookupId(self , rsthis: & QHostInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo11setLookupIdEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QHostInfo11setLookupIdEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QString QHostInfo::localDomainName();
impl /*struct*/ QHostInfo {
  pub fn localDomainName_s<RetType, T: QHostInfo_localDomainName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.localDomainName_s();
    // return 1;
  }
}

pub trait QHostInfo_localDomainName_s<RetType> {
  fn localDomainName_s(self ) -> RetType;
}

  // proto: static QString QHostInfo::localDomainName();
impl<'a> /*trait*/ QHostInfo_localDomainName_s<QString> for () {
  fn localDomainName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo15localDomainNameEv()};
    let mut ret = unsafe {_ZN9QHostInfo15localDomainNameEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QHostInfo::setErrorString(const QString & errorString);
impl /*struct*/ QHostInfo {
  pub fn setErrorString<RetType, T: QHostInfo_setErrorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setErrorString(self);
    // return 1;
  }
}

pub trait QHostInfo_setErrorString<RetType> {
  fn setErrorString(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  void QHostInfo::setErrorString(const QString & errorString);
impl<'a> /*trait*/ QHostInfo_setErrorString<()> for (&'a QString) {
  fn setErrorString(self , rsthis: & QHostInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo14setErrorStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QHostInfo14setErrorStringERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QHostInfo::hostName();
impl /*struct*/ QHostInfo {
  pub fn hostName<RetType, T: QHostInfo_hostName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hostName(self);
    // return 1;
  }
}

pub trait QHostInfo_hostName<RetType> {
  fn hostName(self , rsthis: & QHostInfo) -> RetType;
}

  // proto:  QString QHostInfo::hostName();
impl<'a> /*trait*/ QHostInfo_hostName<QString> for () {
  fn hostName(self , rsthis: & QHostInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QHostInfo8hostNameEv()};
    let mut ret = unsafe {_ZNK9QHostInfo8hostNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QHostInfo QHostInfo::fromName(const QString & name);
impl /*struct*/ QHostInfo {
  pub fn fromName_s<RetType, T: QHostInfo_fromName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromName_s();
    // return 1;
  }
}

pub trait QHostInfo_fromName_s<RetType> {
  fn fromName_s(self ) -> RetType;
}

  // proto: static QHostInfo QHostInfo::fromName(const QString & name);
impl<'a> /*trait*/ QHostInfo_fromName_s<QHostInfo> for (&'a QString) {
  fn fromName_s(self ) -> QHostInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfo8fromNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QHostInfo8fromNameERK7QString(arg0)};
    let mut ret1 = QHostInfo::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QHostInfo::QHostInfo(int lookupId);
impl<'a> /*trait*/ QHostInfo_new for (i32) {
  fn new(self) -> QHostInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHostInfoC2Ei()};
    let ctysz: c_int = unsafe{QHostInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    unsafe {_ZN9QHostInfoC2Ei(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHostInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

