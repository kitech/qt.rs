// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qcoreapplication.h
// dst-file: /src/core/qcoreapplication.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qcoreevent::QEvent; // 773
use super::qtranslator::QTranslator; // 773
use super::qstringlist::QStringList; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCoreApplication_Class_Size() -> c_int;
  // proto: static void QCoreApplication::sendPostedEvents(QObject * receiver, int event_type);
  fn _ZN16QCoreApplication16sendPostedEventsEP7QObjecti(arg0: *mut c_void, arg1: c_int);
  // proto: static void QCoreApplication::addLibraryPath(const QString & );
  fn _ZN16QCoreApplication14addLibraryPathERK7QString(arg0: *mut c_void);
  // proto: static qint64 QCoreApplication::applicationPid();
  fn _ZN16QCoreApplication14applicationPidEv() -> c_longlong;
  // proto: static void QCoreApplication::setApplicationName(const QString & application);
  fn _ZN16QCoreApplication18setApplicationNameERK7QString(arg0: *mut c_void);
  // proto: static QString QCoreApplication::organizationName();
  fn _ZN16QCoreApplication16organizationNameEv() -> *mut c_void;
  // proto: static void QCoreApplication::postEvent(QObject * receiver, QEvent * event, int priority);
  fn _ZN16QCoreApplication9postEventEP7QObjectP6QEventi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto: static bool QCoreApplication::isSetuidAllowed();
  fn _ZN16QCoreApplication15isSetuidAllowedEv() -> c_char;
  // proto: static QString QCoreApplication::applicationName();
  fn _ZN16QCoreApplication15applicationNameEv() -> *mut c_void;
  // proto:  void QCoreApplication::QCoreApplication(const QCoreApplication & );
  fn dector_ZN16QCoreApplicationC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QCoreApplicationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QCoreApplication::setSetuidAllowed(bool allow);
  fn _ZN16QCoreApplication16setSetuidAllowedEb(arg0: c_char);
  // proto: static QStringList QCoreApplication::libraryPaths();
  fn _ZN16QCoreApplication12libraryPathsEv();
  // proto:  void QCoreApplication::applicationNameChanged();
  fn _ZN16QCoreApplication22applicationNameChangedEv(qthis: *mut c_void);
  // proto:  void QCoreApplication::applicationVersionChanged();
  fn _ZN16QCoreApplication25applicationVersionChangedEv(qthis: *mut c_void);
  // proto: static QString QCoreApplication::translate(const char * context, const char * key, const char * disambiguation, int n);
  fn _ZN16QCoreApplication9translateEPKcS1_S1_i(arg0: *mut c_char, arg1: *mut c_char, arg2: *mut c_char, arg3: c_int) -> *mut c_void;
  // proto: static QString QCoreApplication::applicationFilePath();
  fn _ZN16QCoreApplication19applicationFilePathEv() -> *mut c_void;
  // proto: static bool QCoreApplication::removeTranslator(QTranslator * messageFile);
  fn _ZN16QCoreApplication16removeTranslatorEP11QTranslator(arg0: *mut c_void) -> c_char;
  // proto: static void QCoreApplication::setOrganizationName(const QString & orgName);
  fn _ZN16QCoreApplication19setOrganizationNameERK7QString(arg0: *mut c_void);
  // proto: static void QCoreApplication::exit(int retcode);
  fn _ZN16QCoreApplication4exitEi(arg0: c_int);
  // proto: static QString QCoreApplication::applicationVersion();
  fn _ZN16QCoreApplication18applicationVersionEv() -> *mut c_void;
  // proto: static void QCoreApplication::quit();
  fn _ZN16QCoreApplication4quitEv();
  // proto: static bool QCoreApplication::closingDown();
  fn _ZN16QCoreApplication11closingDownEv() -> c_char;
  // proto: static void QCoreApplication::setQuitLockEnabled(bool enabled);
  fn _ZN16QCoreApplication18setQuitLockEnabledEb(arg0: c_char);
  // proto: static bool QCoreApplication::hasPendingEvents();
  fn _ZN16QCoreApplication16hasPendingEventsEv() -> c_char;
  // proto: static void QCoreApplication::removeLibraryPath(const QString & );
  fn _ZN16QCoreApplication17removeLibraryPathERK7QString(arg0: *mut c_void);
  // proto: static void QCoreApplication::setOrganizationDomain(const QString & orgDomain);
  fn _ZN16QCoreApplication21setOrganizationDomainERK7QString(arg0: *mut c_void);
  // proto:  void QCoreApplication::~QCoreApplication();
  fn _ZN16QCoreApplicationD0Ev(qthis: *mut c_void);
  // proto:  void QCoreApplication::organizationDomainChanged();
  fn _ZN16QCoreApplication25organizationDomainChangedEv(qthis: *mut c_void);
  // proto: static QString QCoreApplication::organizationDomain();
  fn _ZN16QCoreApplication18organizationDomainEv() -> *mut c_void;
  // proto: static bool QCoreApplication::installTranslator(QTranslator * messageFile);
  fn _ZN16QCoreApplication17installTranslatorEP11QTranslator(arg0: *mut c_void) -> c_char;
  // proto: static QString QCoreApplication::applicationDirPath();
  fn _ZN16QCoreApplication18applicationDirPathEv() -> *mut c_void;
  // proto: static void QCoreApplication::flush();
  fn _ZN16QCoreApplication5flushEv();
  // proto: static int QCoreApplication::exec();
  fn _ZN16QCoreApplication4execEv() -> c_int;
  // proto: static QStringList QCoreApplication::arguments();
  fn _ZN16QCoreApplication9argumentsEv();
  // proto: static void QCoreApplication::setLibraryPaths(const QStringList & );
  fn _ZN16QCoreApplication15setLibraryPathsERK11QStringList(arg0: *mut c_void);
  // proto: static QAbstractEventDispatcher * QCoreApplication::eventDispatcher();
  fn _ZN16QCoreApplication15eventDispatcherEv();
  // proto: static bool QCoreApplication::startingUp();
  fn _ZN16QCoreApplication10startingUpEv() -> c_char;
  // proto: static bool QCoreApplication::sendEvent(QObject * receiver, QEvent * event);
  fn _ZN16QCoreApplication9sendEventEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QCoreApplication::notify(QObject * , QEvent * );
  fn _ZN16QCoreApplication6notifyEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static bool QCoreApplication::isQuitLockEnabled();
  fn _ZN16QCoreApplication17isQuitLockEnabledEv() -> c_char;
  // proto:  void QCoreApplication::organizationNameChanged();
  fn _ZN16QCoreApplication23organizationNameChangedEv(qthis: *mut c_void);
  // proto: static void QCoreApplication::removePostedEvents(QObject * receiver, int eventType);
  fn _ZN16QCoreApplication18removePostedEventsEP7QObjecti(arg0: *mut c_void, arg1: c_int);
  // proto:  const QMetaObject * QCoreApplication::metaObject();
  fn _ZNK16QCoreApplication10metaObjectEv(qthis: *mut c_void);
  // proto: static void QCoreApplication::setApplicationVersion(const QString & version);
  fn _ZN16QCoreApplication21setApplicationVersionERK7QString(arg0: *mut c_void);
  // proto:  void QCoreApplication::QCoreApplication(int & argc, char ** argv, int );
  fn dector_ZN16QCoreApplicationC1ERiPPci(arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> *mut c_void;
  fn _ZN16QCoreApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QCoreApplication)=1
pub struct QCoreApplication {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCoreApplication {
  pub fn inheritFrom(qthis: *mut c_void) -> QCoreApplication {
    return QCoreApplication{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QCoreApplication {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QCoreApplication {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto: static void QCoreApplication::sendPostedEvents(QObject * receiver, int event_type);
impl /*struct*/ QCoreApplication {
  pub fn sendPostedEvents_s<RetType, T: QCoreApplication_sendPostedEvents_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sendPostedEvents_s();
    // return 1;
  }
}

pub trait QCoreApplication_sendPostedEvents_s<RetType> {
  fn sendPostedEvents_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::sendPostedEvents(QObject * receiver, int event_type);
impl<'a> /*trait*/ QCoreApplication_sendPostedEvents_s<()> for (&'a QObject, i32) {
  fn sendPostedEvents_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16sendPostedEventsEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QCoreApplication16sendPostedEventsEP7QObjecti(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QCoreApplication::addLibraryPath(const QString & );
impl /*struct*/ QCoreApplication {
  pub fn addLibraryPath_s<RetType, T: QCoreApplication_addLibraryPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addLibraryPath_s();
    // return 1;
  }
}

pub trait QCoreApplication_addLibraryPath_s<RetType> {
  fn addLibraryPath_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::addLibraryPath(const QString & );
impl<'a> /*trait*/ QCoreApplication_addLibraryPath_s<()> for (&'a QString) {
  fn addLibraryPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication14addLibraryPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication14addLibraryPathERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static qint64 QCoreApplication::applicationPid();
impl /*struct*/ QCoreApplication {
  pub fn applicationPid_s<RetType, T: QCoreApplication_applicationPid_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationPid_s();
    // return 1;
  }
}

pub trait QCoreApplication_applicationPid_s<RetType> {
  fn applicationPid_s(self ) -> RetType;
}

  // proto: static qint64 QCoreApplication::applicationPid();
impl<'a> /*trait*/ QCoreApplication_applicationPid_s<i64> for () {
  fn applicationPid_s(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication14applicationPidEv()};
    let mut ret = unsafe {_ZN16QCoreApplication14applicationPidEv()};
    return ret as i64;
    // return 1;
  }
}

  // proto: static void QCoreApplication::setApplicationName(const QString & application);
impl /*struct*/ QCoreApplication {
  pub fn setApplicationName_s<RetType, T: QCoreApplication_setApplicationName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationName_s();
    // return 1;
  }
}

pub trait QCoreApplication_setApplicationName_s<RetType> {
  fn setApplicationName_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setApplicationName(const QString & application);
impl<'a> /*trait*/ QCoreApplication_setApplicationName_s<()> for (&'a QString) {
  fn setApplicationName_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18setApplicationNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication18setApplicationNameERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static QString QCoreApplication::organizationName();
impl /*struct*/ QCoreApplication {
  pub fn organizationName_s<RetType, T: QCoreApplication_organizationName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.organizationName_s();
    // return 1;
  }
}

pub trait QCoreApplication_organizationName_s<RetType> {
  fn organizationName_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::organizationName();
impl<'a> /*trait*/ QCoreApplication_organizationName_s<QString> for () {
  fn organizationName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16organizationNameEv()};
    let mut ret = unsafe {_ZN16QCoreApplication16organizationNameEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QCoreApplication::postEvent(QObject * receiver, QEvent * event, int priority);
impl /*struct*/ QCoreApplication {
  pub fn postEvent_s<RetType, T: QCoreApplication_postEvent_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.postEvent_s();
    // return 1;
  }
}

pub trait QCoreApplication_postEvent_s<RetType> {
  fn postEvent_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::postEvent(QObject * receiver, QEvent * event, int priority);
impl<'a> /*trait*/ QCoreApplication_postEvent_s<()> for (&'a QObject, &'a QEvent, i32) {
  fn postEvent_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9postEventEP7QObjectP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QCoreApplication9postEventEP7QObjectP6QEventi(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static bool QCoreApplication::isSetuidAllowed();
impl /*struct*/ QCoreApplication {
  pub fn isSetuidAllowed_s<RetType, T: QCoreApplication_isSetuidAllowed_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSetuidAllowed_s();
    // return 1;
  }
}

pub trait QCoreApplication_isSetuidAllowed_s<RetType> {
  fn isSetuidAllowed_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::isSetuidAllowed();
impl<'a> /*trait*/ QCoreApplication_isSetuidAllowed_s<i8> for () {
  fn isSetuidAllowed_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15isSetuidAllowedEv()};
    let mut ret = unsafe {_ZN16QCoreApplication15isSetuidAllowedEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QCoreApplication::applicationName();
impl /*struct*/ QCoreApplication {
  pub fn applicationName_s<RetType, T: QCoreApplication_applicationName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationName_s();
    // return 1;
  }
}

pub trait QCoreApplication_applicationName_s<RetType> {
  fn applicationName_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::applicationName();
impl<'a> /*trait*/ QCoreApplication_applicationName_s<QString> for () {
  fn applicationName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15applicationNameEv()};
    let mut ret = unsafe {_ZN16QCoreApplication15applicationNameEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCoreApplication::QCoreApplication(const QCoreApplication & );
impl /*struct*/ QCoreApplication {
  pub fn New<T: QCoreApplication_New>(value: T) -> QCoreApplication {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCoreApplication_New {
  fn New(self) -> QCoreApplication;
}

  // proto:  void QCoreApplication::QCoreApplication(const QCoreApplication & );
impl<'a> /*trait*/ QCoreApplication_New for (&'a QCoreApplication) {
  fn New(self) -> QCoreApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationC1ERKS_()};
    let ctysz: c_int = unsafe{QCoreApplication_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QCoreApplicationC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QCoreApplicationC1ERKS_(arg0)};
    let rsthis = QCoreApplication{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QCoreApplication::setSetuidAllowed(bool allow);
impl /*struct*/ QCoreApplication {
  pub fn setSetuidAllowed_s<RetType, T: QCoreApplication_setSetuidAllowed_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setSetuidAllowed_s();
    // return 1;
  }
}

pub trait QCoreApplication_setSetuidAllowed_s<RetType> {
  fn setSetuidAllowed_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setSetuidAllowed(bool allow);
impl<'a> /*trait*/ QCoreApplication_setSetuidAllowed_s<()> for (i8) {
  fn setSetuidAllowed_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16setSetuidAllowedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QCoreApplication16setSetuidAllowedEb(arg0)};
    // return 1;
  }
}

  // proto: static QStringList QCoreApplication::libraryPaths();
impl /*struct*/ QCoreApplication {
  pub fn libraryPaths_s<RetType, T: QCoreApplication_libraryPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.libraryPaths_s();
    // return 1;
  }
}

pub trait QCoreApplication_libraryPaths_s<RetType> {
  fn libraryPaths_s(self ) -> RetType;
}

  // proto: static QStringList QCoreApplication::libraryPaths();
impl<'a> /*trait*/ QCoreApplication_libraryPaths_s<()> for () {
  fn libraryPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication12libraryPathsEv()};
     unsafe {_ZN16QCoreApplication12libraryPathsEv()};
    // return 1;
  }
}

  // proto:  void QCoreApplication::applicationNameChanged();
impl /*struct*/ QCoreApplication {
  pub fn applicationNameChanged<RetType, T: QCoreApplication_applicationNameChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applicationNameChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationNameChanged<RetType> {
  fn applicationNameChanged(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  void QCoreApplication::applicationNameChanged();
impl<'a> /*trait*/ QCoreApplication_applicationNameChanged<()> for () {
  fn applicationNameChanged(self , rsthis: & QCoreApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication22applicationNameChangedEv()};
     unsafe {_ZN16QCoreApplication22applicationNameChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCoreApplication::applicationVersionChanged();
impl /*struct*/ QCoreApplication {
  pub fn applicationVersionChanged<RetType, T: QCoreApplication_applicationVersionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applicationVersionChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationVersionChanged<RetType> {
  fn applicationVersionChanged(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  void QCoreApplication::applicationVersionChanged();
impl<'a> /*trait*/ QCoreApplication_applicationVersionChanged<()> for () {
  fn applicationVersionChanged(self , rsthis: & QCoreApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication25applicationVersionChangedEv()};
     unsafe {_ZN16QCoreApplication25applicationVersionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QString QCoreApplication::translate(const char * context, const char * key, const char * disambiguation, int n);
impl /*struct*/ QCoreApplication {
  pub fn translate_s<RetType, T: QCoreApplication_translate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.translate_s();
    // return 1;
  }
}

pub trait QCoreApplication_translate_s<RetType> {
  fn translate_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::translate(const char * context, const char * key, const char * disambiguation, int n);
impl<'a> /*trait*/ QCoreApplication_translate_s<QString> for (&'a  String, &'a  String, &'a  String, i32) {
  fn translate_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9translateEPKcS1_S1_i()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN16QCoreApplication9translateEPKcS1_S1_i(arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QCoreApplication::applicationFilePath();
impl /*struct*/ QCoreApplication {
  pub fn applicationFilePath_s<RetType, T: QCoreApplication_applicationFilePath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationFilePath_s();
    // return 1;
  }
}

pub trait QCoreApplication_applicationFilePath_s<RetType> {
  fn applicationFilePath_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::applicationFilePath();
impl<'a> /*trait*/ QCoreApplication_applicationFilePath_s<QString> for () {
  fn applicationFilePath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication19applicationFilePathEv()};
    let mut ret = unsafe {_ZN16QCoreApplication19applicationFilePathEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QCoreApplication::removeTranslator(QTranslator * messageFile);
impl /*struct*/ QCoreApplication {
  pub fn removeTranslator_s<RetType, T: QCoreApplication_removeTranslator_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeTranslator_s();
    // return 1;
  }
}

pub trait QCoreApplication_removeTranslator_s<RetType> {
  fn removeTranslator_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::removeTranslator(QTranslator * messageFile);
impl<'a> /*trait*/ QCoreApplication_removeTranslator_s<i8> for (&'a QTranslator) {
  fn removeTranslator_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16removeTranslatorEP11QTranslator()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication16removeTranslatorEP11QTranslator(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QCoreApplication::setOrganizationName(const QString & orgName);
impl /*struct*/ QCoreApplication {
  pub fn setOrganizationName_s<RetType, T: QCoreApplication_setOrganizationName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setOrganizationName_s();
    // return 1;
  }
}

pub trait QCoreApplication_setOrganizationName_s<RetType> {
  fn setOrganizationName_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setOrganizationName(const QString & orgName);
impl<'a> /*trait*/ QCoreApplication_setOrganizationName_s<()> for (&'a QString) {
  fn setOrganizationName_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication19setOrganizationNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication19setOrganizationNameERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static void QCoreApplication::exit(int retcode);
impl /*struct*/ QCoreApplication {
  pub fn exit_s<RetType, T: QCoreApplication_exit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exit_s();
    // return 1;
  }
}

pub trait QCoreApplication_exit_s<RetType> {
  fn exit_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::exit(int retcode);
impl<'a> /*trait*/ QCoreApplication_exit_s<()> for (i32) {
  fn exit_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QCoreApplication4exitEi(arg0)};
    // return 1;
  }
}

  // proto: static QString QCoreApplication::applicationVersion();
impl /*struct*/ QCoreApplication {
  pub fn applicationVersion_s<RetType, T: QCoreApplication_applicationVersion_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationVersion_s();
    // return 1;
  }
}

pub trait QCoreApplication_applicationVersion_s<RetType> {
  fn applicationVersion_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::applicationVersion();
impl<'a> /*trait*/ QCoreApplication_applicationVersion_s<QString> for () {
  fn applicationVersion_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18applicationVersionEv()};
    let mut ret = unsafe {_ZN16QCoreApplication18applicationVersionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QCoreApplication::quit();
impl /*struct*/ QCoreApplication {
  pub fn quit_s<RetType, T: QCoreApplication_quit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.quit_s();
    // return 1;
  }
}

pub trait QCoreApplication_quit_s<RetType> {
  fn quit_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::quit();
impl<'a> /*trait*/ QCoreApplication_quit_s<()> for () {
  fn quit_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4quitEv()};
     unsafe {_ZN16QCoreApplication4quitEv()};
    // return 1;
  }
}

  // proto: static bool QCoreApplication::closingDown();
impl /*struct*/ QCoreApplication {
  pub fn closingDown_s<RetType, T: QCoreApplication_closingDown_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.closingDown_s();
    // return 1;
  }
}

pub trait QCoreApplication_closingDown_s<RetType> {
  fn closingDown_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::closingDown();
impl<'a> /*trait*/ QCoreApplication_closingDown_s<i8> for () {
  fn closingDown_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication11closingDownEv()};
    let mut ret = unsafe {_ZN16QCoreApplication11closingDownEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QCoreApplication::setQuitLockEnabled(bool enabled);
impl /*struct*/ QCoreApplication {
  pub fn setQuitLockEnabled_s<RetType, T: QCoreApplication_setQuitLockEnabled_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setQuitLockEnabled_s();
    // return 1;
  }
}

pub trait QCoreApplication_setQuitLockEnabled_s<RetType> {
  fn setQuitLockEnabled_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setQuitLockEnabled(bool enabled);
impl<'a> /*trait*/ QCoreApplication_setQuitLockEnabled_s<()> for (i8) {
  fn setQuitLockEnabled_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18setQuitLockEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QCoreApplication18setQuitLockEnabledEb(arg0)};
    // return 1;
  }
}

  // proto: static bool QCoreApplication::hasPendingEvents();
impl /*struct*/ QCoreApplication {
  pub fn hasPendingEvents_s<RetType, T: QCoreApplication_hasPendingEvents_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasPendingEvents_s();
    // return 1;
  }
}

pub trait QCoreApplication_hasPendingEvents_s<RetType> {
  fn hasPendingEvents_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::hasPendingEvents();
impl<'a> /*trait*/ QCoreApplication_hasPendingEvents_s<i8> for () {
  fn hasPendingEvents_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16hasPendingEventsEv()};
    let mut ret = unsafe {_ZN16QCoreApplication16hasPendingEventsEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QCoreApplication::removeLibraryPath(const QString & );
impl /*struct*/ QCoreApplication {
  pub fn removeLibraryPath_s<RetType, T: QCoreApplication_removeLibraryPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeLibraryPath_s();
    // return 1;
  }
}

pub trait QCoreApplication_removeLibraryPath_s<RetType> {
  fn removeLibraryPath_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::removeLibraryPath(const QString & );
impl<'a> /*trait*/ QCoreApplication_removeLibraryPath_s<()> for (&'a QString) {
  fn removeLibraryPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17removeLibraryPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication17removeLibraryPathERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static void QCoreApplication::setOrganizationDomain(const QString & orgDomain);
impl /*struct*/ QCoreApplication {
  pub fn setOrganizationDomain_s<RetType, T: QCoreApplication_setOrganizationDomain_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setOrganizationDomain_s();
    // return 1;
  }
}

pub trait QCoreApplication_setOrganizationDomain_s<RetType> {
  fn setOrganizationDomain_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setOrganizationDomain(const QString & orgDomain);
impl<'a> /*trait*/ QCoreApplication_setOrganizationDomain_s<()> for (&'a QString) {
  fn setOrganizationDomain_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication21setOrganizationDomainERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication21setOrganizationDomainERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  void QCoreApplication::~QCoreApplication();
impl /*struct*/ QCoreApplication {
  pub fn Free<RetType, T: QCoreApplication_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCoreApplication_Free<RetType> {
  fn Free(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  void QCoreApplication::~QCoreApplication();
impl<'a> /*trait*/ QCoreApplication_Free<()> for () {
  fn Free(self , rsthis: & QCoreApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationD0Ev()};
     unsafe {_ZN16QCoreApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCoreApplication::organizationDomainChanged();
impl /*struct*/ QCoreApplication {
  pub fn organizationDomainChanged<RetType, T: QCoreApplication_organizationDomainChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.organizationDomainChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_organizationDomainChanged<RetType> {
  fn organizationDomainChanged(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  void QCoreApplication::organizationDomainChanged();
impl<'a> /*trait*/ QCoreApplication_organizationDomainChanged<()> for () {
  fn organizationDomainChanged(self , rsthis: & QCoreApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication25organizationDomainChangedEv()};
     unsafe {_ZN16QCoreApplication25organizationDomainChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QString QCoreApplication::organizationDomain();
impl /*struct*/ QCoreApplication {
  pub fn organizationDomain_s<RetType, T: QCoreApplication_organizationDomain_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.organizationDomain_s();
    // return 1;
  }
}

pub trait QCoreApplication_organizationDomain_s<RetType> {
  fn organizationDomain_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::organizationDomain();
impl<'a> /*trait*/ QCoreApplication_organizationDomain_s<QString> for () {
  fn organizationDomain_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18organizationDomainEv()};
    let mut ret = unsafe {_ZN16QCoreApplication18organizationDomainEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QCoreApplication::installTranslator(QTranslator * messageFile);
impl /*struct*/ QCoreApplication {
  pub fn installTranslator_s<RetType, T: QCoreApplication_installTranslator_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.installTranslator_s();
    // return 1;
  }
}

pub trait QCoreApplication_installTranslator_s<RetType> {
  fn installTranslator_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::installTranslator(QTranslator * messageFile);
impl<'a> /*trait*/ QCoreApplication_installTranslator_s<i8> for (&'a QTranslator) {
  fn installTranslator_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17installTranslatorEP11QTranslator()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication17installTranslatorEP11QTranslator(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QCoreApplication::applicationDirPath();
impl /*struct*/ QCoreApplication {
  pub fn applicationDirPath_s<RetType, T: QCoreApplication_applicationDirPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationDirPath_s();
    // return 1;
  }
}

pub trait QCoreApplication_applicationDirPath_s<RetType> {
  fn applicationDirPath_s(self ) -> RetType;
}

  // proto: static QString QCoreApplication::applicationDirPath();
impl<'a> /*trait*/ QCoreApplication_applicationDirPath_s<QString> for () {
  fn applicationDirPath_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18applicationDirPathEv()};
    let mut ret = unsafe {_ZN16QCoreApplication18applicationDirPathEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QCoreApplication::flush();
impl /*struct*/ QCoreApplication {
  pub fn flush_s<RetType, T: QCoreApplication_flush_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.flush_s();
    // return 1;
  }
}

pub trait QCoreApplication_flush_s<RetType> {
  fn flush_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::flush();
impl<'a> /*trait*/ QCoreApplication_flush_s<()> for () {
  fn flush_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication5flushEv()};
     unsafe {_ZN16QCoreApplication5flushEv()};
    // return 1;
  }
}

  // proto: static int QCoreApplication::exec();
impl /*struct*/ QCoreApplication {
  pub fn exec_s<RetType, T: QCoreApplication_exec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_s();
    // return 1;
  }
}

pub trait QCoreApplication_exec_s<RetType> {
  fn exec_s(self ) -> RetType;
}

  // proto: static int QCoreApplication::exec();
impl<'a> /*trait*/ QCoreApplication_exec_s<i32> for () {
  fn exec_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4execEv()};
    let mut ret = unsafe {_ZN16QCoreApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QStringList QCoreApplication::arguments();
impl /*struct*/ QCoreApplication {
  pub fn arguments_s<RetType, T: QCoreApplication_arguments_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.arguments_s();
    // return 1;
  }
}

pub trait QCoreApplication_arguments_s<RetType> {
  fn arguments_s(self ) -> RetType;
}

  // proto: static QStringList QCoreApplication::arguments();
impl<'a> /*trait*/ QCoreApplication_arguments_s<()> for () {
  fn arguments_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9argumentsEv()};
     unsafe {_ZN16QCoreApplication9argumentsEv()};
    // return 1;
  }
}

  // proto: static void QCoreApplication::setLibraryPaths(const QStringList & );
impl /*struct*/ QCoreApplication {
  pub fn setLibraryPaths_s<RetType, T: QCoreApplication_setLibraryPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setLibraryPaths_s();
    // return 1;
  }
}

pub trait QCoreApplication_setLibraryPaths_s<RetType> {
  fn setLibraryPaths_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setLibraryPaths(const QStringList & );
impl<'a> /*trait*/ QCoreApplication_setLibraryPaths_s<()> for (&'a QStringList) {
  fn setLibraryPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15setLibraryPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication15setLibraryPathsERK11QStringList(arg0)};
    // return 1;
  }
}

  // proto: static QAbstractEventDispatcher * QCoreApplication::eventDispatcher();
impl /*struct*/ QCoreApplication {
  pub fn eventDispatcher_s<RetType, T: QCoreApplication_eventDispatcher_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.eventDispatcher_s();
    // return 1;
  }
}

pub trait QCoreApplication_eventDispatcher_s<RetType> {
  fn eventDispatcher_s(self ) -> RetType;
}

  // proto: static QAbstractEventDispatcher * QCoreApplication::eventDispatcher();
impl<'a> /*trait*/ QCoreApplication_eventDispatcher_s<()> for () {
  fn eventDispatcher_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15eventDispatcherEv()};
     unsafe {_ZN16QCoreApplication15eventDispatcherEv()};
    // return 1;
  }
}

  // proto: static bool QCoreApplication::startingUp();
impl /*struct*/ QCoreApplication {
  pub fn startingUp_s<RetType, T: QCoreApplication_startingUp_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.startingUp_s();
    // return 1;
  }
}

pub trait QCoreApplication_startingUp_s<RetType> {
  fn startingUp_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::startingUp();
impl<'a> /*trait*/ QCoreApplication_startingUp_s<i8> for () {
  fn startingUp_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication10startingUpEv()};
    let mut ret = unsafe {_ZN16QCoreApplication10startingUpEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QCoreApplication::sendEvent(QObject * receiver, QEvent * event);
impl /*struct*/ QCoreApplication {
  pub fn sendEvent_s<RetType, T: QCoreApplication_sendEvent_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sendEvent_s();
    // return 1;
  }
}

pub trait QCoreApplication_sendEvent_s<RetType> {
  fn sendEvent_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::sendEvent(QObject * receiver, QEvent * event);
impl<'a> /*trait*/ QCoreApplication_sendEvent_s<i8> for (&'a QObject, &'a QEvent) {
  fn sendEvent_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QCoreApplication::notify(QObject * , QEvent * );
impl /*struct*/ QCoreApplication {
  pub fn notify<RetType, T: QCoreApplication_notify<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notify(self);
    // return 1;
  }
}

pub trait QCoreApplication_notify<RetType> {
  fn notify(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  bool QCoreApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QCoreApplication_notify<i8> for (&'a QObject, &'a QEvent) {
  fn notify(self , rsthis: & QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QCoreApplication::isQuitLockEnabled();
impl /*struct*/ QCoreApplication {
  pub fn isQuitLockEnabled_s<RetType, T: QCoreApplication_isQuitLockEnabled_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isQuitLockEnabled_s();
    // return 1;
  }
}

pub trait QCoreApplication_isQuitLockEnabled_s<RetType> {
  fn isQuitLockEnabled_s(self ) -> RetType;
}

  // proto: static bool QCoreApplication::isQuitLockEnabled();
impl<'a> /*trait*/ QCoreApplication_isQuitLockEnabled_s<i8> for () {
  fn isQuitLockEnabled_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17isQuitLockEnabledEv()};
    let mut ret = unsafe {_ZN16QCoreApplication17isQuitLockEnabledEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCoreApplication::organizationNameChanged();
impl /*struct*/ QCoreApplication {
  pub fn organizationNameChanged<RetType, T: QCoreApplication_organizationNameChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.organizationNameChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_organizationNameChanged<RetType> {
  fn organizationNameChanged(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  void QCoreApplication::organizationNameChanged();
impl<'a> /*trait*/ QCoreApplication_organizationNameChanged<()> for () {
  fn organizationNameChanged(self , rsthis: & QCoreApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication23organizationNameChangedEv()};
     unsafe {_ZN16QCoreApplication23organizationNameChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QCoreApplication::removePostedEvents(QObject * receiver, int eventType);
impl /*struct*/ QCoreApplication {
  pub fn removePostedEvents_s<RetType, T: QCoreApplication_removePostedEvents_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removePostedEvents_s();
    // return 1;
  }
}

pub trait QCoreApplication_removePostedEvents_s<RetType> {
  fn removePostedEvents_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::removePostedEvents(QObject * receiver, int eventType);
impl<'a> /*trait*/ QCoreApplication_removePostedEvents_s<()> for (&'a QObject, i32) {
  fn removePostedEvents_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18removePostedEventsEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QCoreApplication18removePostedEventsEP7QObjecti(arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QCoreApplication::metaObject();
impl /*struct*/ QCoreApplication {
  pub fn metaObject<RetType, T: QCoreApplication_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCoreApplication_metaObject<RetType> {
  fn metaObject(self , rsthis: & QCoreApplication) -> RetType;
}

  // proto:  const QMetaObject * QCoreApplication::metaObject();
impl<'a> /*trait*/ QCoreApplication_metaObject<()> for () {
  fn metaObject(self , rsthis: & QCoreApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QCoreApplication10metaObjectEv()};
     unsafe {_ZNK16QCoreApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QCoreApplication::setApplicationVersion(const QString & version);
impl /*struct*/ QCoreApplication {
  pub fn setApplicationVersion_s<RetType, T: QCoreApplication_setApplicationVersion_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationVersion_s();
    // return 1;
  }
}

pub trait QCoreApplication_setApplicationVersion_s<RetType> {
  fn setApplicationVersion_s(self ) -> RetType;
}

  // proto: static void QCoreApplication::setApplicationVersion(const QString & version);
impl<'a> /*trait*/ QCoreApplication_setApplicationVersion_s<()> for (&'a QString) {
  fn setApplicationVersion_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication21setApplicationVersionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication21setApplicationVersionERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  void QCoreApplication::QCoreApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QCoreApplication_New for (&'a mut i32, &'a mut String, i32) {
  fn New(self) -> QCoreApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationC1ERiPPci()};
    let ctysz: c_int = unsafe{QCoreApplication_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN16QCoreApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN16QCoreApplicationC1ERiPPci(arg0, arg1, arg2)};
    let rsthis = QCoreApplication{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

