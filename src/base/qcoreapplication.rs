// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qevent::QEvent;
use super::qtranslator::QTranslator;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QCoreApplication::sendPostedEvents(QObject * receiver, int event_type);
  fn _ZN16QCoreApplication16sendPostedEventsEP7QObjecti(arg0: *mut c_void, arg1: c_int) ;
  // proto: static void QCoreApplication::addLibraryPath(const QString & );
  fn _ZN16QCoreApplication14addLibraryPathERK7QString(arg0: *mut c_void) ;
  // proto: static long long QCoreApplication::applicationPid();
  fn _ZN16QCoreApplication14applicationPidEv() -> c_longlong;
  // proto: static void QCoreApplication::setApplicationName(const QString & application);
  fn _ZN16QCoreApplication18setApplicationNameERK7QString(arg0: *mut c_void) ;
  // proto: static QString QCoreApplication::organizationName();
  fn _ZN16QCoreApplication16organizationNameEv() -> *mut c_void;
  // proto: static QCoreApplication * QCoreApplication::instance();
  fn _ZN16QCoreApplication8instanceEv() -> *mut c_void;
  // proto: static bool QCoreApplication::isSetuidAllowed();
  fn _ZN16QCoreApplication15isSetuidAllowedEv() -> int8_t;
  // proto:  void QCoreApplication::applicationVersionChanged();
  fn _ZN16QCoreApplication25applicationVersionChangedEv(qthis: *mut c_void) ;
  // proto: static QString QCoreApplication::applicationName();
  fn _ZN16QCoreApplication15applicationNameEv() -> *mut c_void;
  // proto:  void QCoreApplication::NewQCoreApplication(const QCoreApplication & );
  fn _ZN16QCoreApplicationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QCoreApplication::setSetuidAllowed(bool allow);
  fn _ZN16QCoreApplication16setSetuidAllowedEb(arg0: int8_t) ;
  // proto: static void QCoreApplication::postEvent(QObject * receiver, QEvent * event, int priority);
  fn _ZN16QCoreApplication9postEventEP7QObjectP6QEventi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto: static QStringList QCoreApplication::libraryPaths();
  fn _ZN16QCoreApplication12libraryPathsEv() ;
  // proto:  void QCoreApplication::applicationNameChanged();
  fn _ZN16QCoreApplication22applicationNameChangedEv(qthis: *mut c_void) ;
  // proto: static void QCoreApplication::removeLibraryPath(const QString & );
  fn _ZN16QCoreApplication17removeLibraryPathERK7QString(arg0: *mut c_void) ;
  // proto: static QString QCoreApplication::translate(const char * context, const char * key, const char * disambiguation, int n);
  fn _ZN16QCoreApplication9translateEPKcS1_S1_i(arg0: *const c_char, arg1: *const c_char, arg2: *const c_char, arg3: c_int) -> *mut c_void;
  // proto: static QString QCoreApplication::applicationFilePath();
  fn _ZN16QCoreApplication19applicationFilePathEv() -> *mut c_void;
  // proto: static bool QCoreApplication::removeTranslator(QTranslator * messageFile);
  fn _ZN16QCoreApplication16removeTranslatorEP11QTranslator(arg0: *mut c_void) -> int8_t;
  // proto: static void QCoreApplication::setOrganizationName(const QString & orgName);
  fn _ZN16QCoreApplication19setOrganizationNameERK7QString(arg0: *mut c_void) ;
  // proto: static void QCoreApplication::exit(int retcode);
  fn _ZN16QCoreApplication4exitEi(arg0: c_int) ;
  // proto: static QString QCoreApplication::applicationVersion();
  fn _ZN16QCoreApplication18applicationVersionEv() -> *mut c_void;
  // proto: static void QCoreApplication::quit();
  fn _ZN16QCoreApplication4quitEv() ;
  // proto: static bool QCoreApplication::closingDown();
  fn _ZN16QCoreApplication11closingDownEv() -> int8_t;
  // proto: static void QCoreApplication::setQuitLockEnabled(bool enabled);
  fn _ZN16QCoreApplication18setQuitLockEnabledEb(arg0: int8_t) ;
  // proto: static bool QCoreApplication::hasPendingEvents();
  fn _ZN16QCoreApplication16hasPendingEventsEv() -> int8_t;
  // proto: static void QCoreApplication::setOrganizationDomain(const QString & orgDomain);
  fn _ZN16QCoreApplication21setOrganizationDomainERK7QString(arg0: *mut c_void) ;
  // proto:  void QCoreApplication::FreeQCoreApplication();
  fn _ZN16QCoreApplicationD0Ev(qthis: *mut c_void) ;
  // proto:  void QCoreApplication::organizationDomainChanged();
  fn _ZN16QCoreApplication25organizationDomainChangedEv(qthis: *mut c_void) ;
  // proto: static QString QCoreApplication::organizationDomain();
  fn _ZN16QCoreApplication18organizationDomainEv() -> *mut c_void;
  // proto: static bool QCoreApplication::installTranslator(QTranslator * messageFile);
  fn _ZN16QCoreApplication17installTranslatorEP11QTranslator(arg0: *mut c_void) -> int8_t;
  // proto: static QString QCoreApplication::applicationDirPath();
  fn _ZN16QCoreApplication18applicationDirPathEv() -> *mut c_void;
  // proto: static void QCoreApplication::flush();
  fn _ZN16QCoreApplication5flushEv() ;
  // proto: static int QCoreApplication::exec();
  fn _ZN16QCoreApplication4execEv() -> c_int;
  // proto: static QStringList QCoreApplication::arguments();
  fn _ZN16QCoreApplication9argumentsEv() ;
  // proto: static void QCoreApplication::setLibraryPaths(const QStringList & );
  fn _ZN16QCoreApplication15setLibraryPathsERK11QStringList(arg0: *mut c_void) ;
  // proto: static QAbstractEventDispatcher * QCoreApplication::eventDispatcher();
  fn _ZN16QCoreApplication15eventDispatcherEv() ;
  // proto: static bool QCoreApplication::startingUp();
  fn _ZN16QCoreApplication10startingUpEv() -> int8_t;
  // proto: static bool QCoreApplication::sendEvent(QObject * receiver, QEvent * event);
  fn _ZN16QCoreApplication9sendEventEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  bool QCoreApplication::notify(QObject * , QEvent * );
  fn _ZN16QCoreApplication6notifyEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static bool QCoreApplication::isQuitLockEnabled();
  fn _ZN16QCoreApplication17isQuitLockEnabledEv() -> int8_t;
  // proto:  void QCoreApplication::organizationNameChanged();
  fn _ZN16QCoreApplication23organizationNameChangedEv(qthis: *mut c_void) ;
  // proto: static void QCoreApplication::removePostedEvents(QObject * receiver, int eventType);
  fn _ZN16QCoreApplication18removePostedEventsEP7QObjecti(arg0: *mut c_void, arg1: c_int) ;
  // proto:  const QMetaObject * QCoreApplication::metaObject();
  fn _ZNK16QCoreApplication10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QCoreApplication::NewQCoreApplication(int & argc, char ** argv, int );
  fn _ZN16QCoreApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) ;
  // proto: static void QCoreApplication::setApplicationVersion(const QString & version);
  fn _ZN16QCoreApplication21setApplicationVersionERK7QString(arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QCoreApplication)=1
pub struct QCoreApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCoreApplication {
  pub fn sendPostedEvents<T: QCoreApplication_sendPostedEvents>(&mut self, value: T)  {
     value.sendPostedEvents(self);
    // return 1;
  }
}

pub trait QCoreApplication_sendPostedEvents {
  fn sendPostedEvents(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::sendPostedEvents(QObject * receiver, int event_type);
impl<'a> /*trait*/ QCoreApplication_sendPostedEvents for (&'a mut QObject, i32) {
  fn sendPostedEvents(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16sendPostedEventsEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QCoreApplication16sendPostedEventsEP7QObjecti(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn addLibraryPath<T: QCoreApplication_addLibraryPath>(&mut self, value: T)  {
     value.addLibraryPath(self);
    // return 1;
  }
}

pub trait QCoreApplication_addLibraryPath {
  fn addLibraryPath(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::addLibraryPath(const QString & );
impl<'a> /*trait*/ QCoreApplication_addLibraryPath for (&'a  QString) {
  fn addLibraryPath(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication14addLibraryPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication14addLibraryPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationPid<T: QCoreApplication_applicationPid>(&mut self, value: T) -> i64 {
    return value.applicationPid(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationPid {
  fn applicationPid(self, rsthis: &mut QCoreApplication) -> i64;
}

// proto: static long long QCoreApplication::applicationPid();
impl<'a> /*trait*/ QCoreApplication_applicationPid for () {
  fn applicationPid(self, rsthis: &mut QCoreApplication) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication14applicationPidEv()};
    let mut ret = unsafe {_ZN16QCoreApplication14applicationPidEv()};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setApplicationName<T: QCoreApplication_setApplicationName>(&mut self, value: T)  {
     value.setApplicationName(self);
    // return 1;
  }
}

pub trait QCoreApplication_setApplicationName {
  fn setApplicationName(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setApplicationName(const QString & application);
impl<'a> /*trait*/ QCoreApplication_setApplicationName for (&'a  QString) {
  fn setApplicationName(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18setApplicationNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication18setApplicationNameERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationName<T: QCoreApplication_organizationName>(&mut self, value: T) -> QString {
    return value.organizationName(self);
    // return 1;
  }
}

pub trait QCoreApplication_organizationName {
  fn organizationName(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::organizationName();
impl<'a> /*trait*/ QCoreApplication_organizationName for () {
  fn organizationName(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16organizationNameEv()};
    let mut ret = unsafe {_ZN16QCoreApplication16organizationNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn instance<T: QCoreApplication_instance>(&mut self, value: T) -> QCoreApplication {
    return value.instance(self);
    // return 1;
  }
}

pub trait QCoreApplication_instance {
  fn instance(self, rsthis: &mut QCoreApplication) -> QCoreApplication;
}

// proto: static QCoreApplication * QCoreApplication::instance();
impl<'a> /*trait*/ QCoreApplication_instance for () {
  fn instance(self, rsthis: &mut QCoreApplication) -> QCoreApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication8instanceEv()};
    let mut ret = unsafe {_ZN16QCoreApplication8instanceEv()};
    let mut ret1 = QCoreApplication{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn isSetuidAllowed<T: QCoreApplication_isSetuidAllowed>(&mut self, value: T) -> i8 {
    return value.isSetuidAllowed(self);
    // return 1;
  }
}

pub trait QCoreApplication_isSetuidAllowed {
  fn isSetuidAllowed(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::isSetuidAllowed();
impl<'a> /*trait*/ QCoreApplication_isSetuidAllowed for () {
  fn isSetuidAllowed(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15isSetuidAllowedEv()};
    let mut ret = unsafe {_ZN16QCoreApplication15isSetuidAllowedEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationVersionChanged<T: QCoreApplication_applicationVersionChanged>(&mut self, value: T)  {
     value.applicationVersionChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationVersionChanged {
  fn applicationVersionChanged(self, rsthis: &mut QCoreApplication) ;
}

// proto:  void QCoreApplication::applicationVersionChanged();
impl<'a> /*trait*/ QCoreApplication_applicationVersionChanged for () {
  fn applicationVersionChanged(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication25applicationVersionChangedEv()};
     unsafe {_ZN16QCoreApplication25applicationVersionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationName<T: QCoreApplication_applicationName>(&mut self, value: T) -> QString {
    return value.applicationName(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationName {
  fn applicationName(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::applicationName();
impl<'a> /*trait*/ QCoreApplication_applicationName for () {
  fn applicationName(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15applicationNameEv()};
    let mut ret = unsafe {_ZN16QCoreApplication15applicationNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn NewQCoreApplication<T: QCoreApplication_NewQCoreApplication>(value: T) -> QCoreApplication {
    let rsthis = value.NewQCoreApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QCoreApplication_NewQCoreApplication {
  fn NewQCoreApplication(self) -> QCoreApplication;
}

// proto: void QCoreApplication::NewQCoreApplication(const QCoreApplication & );
impl<'a> /*trait*/ QCoreApplication_NewQCoreApplication for (&'a  QCoreApplication) {
  fn NewQCoreApplication(self) -> QCoreApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QCoreApplicationC1ERKS_(qthis, arg0)};
    let rsthis = QCoreApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setSetuidAllowed<T: QCoreApplication_setSetuidAllowed>(&mut self, value: T)  {
     value.setSetuidAllowed(self);
    // return 1;
  }
}

pub trait QCoreApplication_setSetuidAllowed {
  fn setSetuidAllowed(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setSetuidAllowed(bool allow);
impl<'a> /*trait*/ QCoreApplication_setSetuidAllowed for (i8) {
  fn setSetuidAllowed(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16setSetuidAllowedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QCoreApplication16setSetuidAllowedEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn postEvent<T: QCoreApplication_postEvent>(&mut self, value: T)  {
     value.postEvent(self);
    // return 1;
  }
}

pub trait QCoreApplication_postEvent {
  fn postEvent(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::postEvent(QObject * receiver, QEvent * event, int priority);
impl<'a> /*trait*/ QCoreApplication_postEvent for (&'a mut QObject, &'a mut QEvent, i32) {
  fn postEvent(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9postEventEP7QObjectP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QCoreApplication9postEventEP7QObjectP6QEventi(arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn libraryPaths<T: QCoreApplication_libraryPaths>(&mut self, value: T)  {
     value.libraryPaths(self);
    // return 1;
  }
}

pub trait QCoreApplication_libraryPaths {
  fn libraryPaths(self, rsthis: &mut QCoreApplication) ;
}

// proto: static QStringList QCoreApplication::libraryPaths();
impl<'a> /*trait*/ QCoreApplication_libraryPaths for () {
  fn libraryPaths(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication12libraryPathsEv()};
     unsafe {_ZN16QCoreApplication12libraryPathsEv()};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationNameChanged<T: QCoreApplication_applicationNameChanged>(&mut self, value: T)  {
     value.applicationNameChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationNameChanged {
  fn applicationNameChanged(self, rsthis: &mut QCoreApplication) ;
}

// proto:  void QCoreApplication::applicationNameChanged();
impl<'a> /*trait*/ QCoreApplication_applicationNameChanged for () {
  fn applicationNameChanged(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication22applicationNameChangedEv()};
     unsafe {_ZN16QCoreApplication22applicationNameChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn removeLibraryPath<T: QCoreApplication_removeLibraryPath>(&mut self, value: T)  {
     value.removeLibraryPath(self);
    // return 1;
  }
}

pub trait QCoreApplication_removeLibraryPath {
  fn removeLibraryPath(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::removeLibraryPath(const QString & );
impl<'a> /*trait*/ QCoreApplication_removeLibraryPath for (&'a  QString) {
  fn removeLibraryPath(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17removeLibraryPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication17removeLibraryPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn translate<T: QCoreApplication_translate>(&mut self, value: T) -> QString {
    return value.translate(self);
    // return 1;
  }
}

pub trait QCoreApplication_translate {
  fn translate(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::translate(const char * context, const char * key, const char * disambiguation, int n);
impl<'a> /*trait*/ QCoreApplication_translate for (&'a  String, &'a  String, &'a  String, i32) {
  fn translate(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9translateEPKcS1_S1_i()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN16QCoreApplication9translateEPKcS1_S1_i(arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationFilePath<T: QCoreApplication_applicationFilePath>(&mut self, value: T) -> QString {
    return value.applicationFilePath(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationFilePath {
  fn applicationFilePath(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::applicationFilePath();
impl<'a> /*trait*/ QCoreApplication_applicationFilePath for () {
  fn applicationFilePath(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication19applicationFilePathEv()};
    let mut ret = unsafe {_ZN16QCoreApplication19applicationFilePathEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn removeTranslator<T: QCoreApplication_removeTranslator>(&mut self, value: T) -> i8 {
    return value.removeTranslator(self);
    // return 1;
  }
}

pub trait QCoreApplication_removeTranslator {
  fn removeTranslator(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::removeTranslator(QTranslator * messageFile);
impl<'a> /*trait*/ QCoreApplication_removeTranslator for (&'a mut QTranslator) {
  fn removeTranslator(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16removeTranslatorEP11QTranslator()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication16removeTranslatorEP11QTranslator(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setOrganizationName<T: QCoreApplication_setOrganizationName>(&mut self, value: T)  {
     value.setOrganizationName(self);
    // return 1;
  }
}

pub trait QCoreApplication_setOrganizationName {
  fn setOrganizationName(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setOrganizationName(const QString & orgName);
impl<'a> /*trait*/ QCoreApplication_setOrganizationName for (&'a  QString) {
  fn setOrganizationName(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication19setOrganizationNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication19setOrganizationNameERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn exit<T: QCoreApplication_exit>(&mut self, value: T)  {
     value.exit(self);
    // return 1;
  }
}

pub trait QCoreApplication_exit {
  fn exit(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::exit(int retcode);
impl<'a> /*trait*/ QCoreApplication_exit for (i32) {
  fn exit(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QCoreApplication4exitEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationVersion<T: QCoreApplication_applicationVersion>(&mut self, value: T) -> QString {
    return value.applicationVersion(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationVersion {
  fn applicationVersion(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::applicationVersion();
impl<'a> /*trait*/ QCoreApplication_applicationVersion for () {
  fn applicationVersion(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18applicationVersionEv()};
    let mut ret = unsafe {_ZN16QCoreApplication18applicationVersionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn quit<T: QCoreApplication_quit>(&mut self, value: T)  {
     value.quit(self);
    // return 1;
  }
}

pub trait QCoreApplication_quit {
  fn quit(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::quit();
impl<'a> /*trait*/ QCoreApplication_quit for () {
  fn quit(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4quitEv()};
     unsafe {_ZN16QCoreApplication4quitEv()};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn closingDown<T: QCoreApplication_closingDown>(&mut self, value: T) -> i8 {
    return value.closingDown(self);
    // return 1;
  }
}

pub trait QCoreApplication_closingDown {
  fn closingDown(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::closingDown();
impl<'a> /*trait*/ QCoreApplication_closingDown for () {
  fn closingDown(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication11closingDownEv()};
    let mut ret = unsafe {_ZN16QCoreApplication11closingDownEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setQuitLockEnabled<T: QCoreApplication_setQuitLockEnabled>(&mut self, value: T)  {
     value.setQuitLockEnabled(self);
    // return 1;
  }
}

pub trait QCoreApplication_setQuitLockEnabled {
  fn setQuitLockEnabled(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setQuitLockEnabled(bool enabled);
impl<'a> /*trait*/ QCoreApplication_setQuitLockEnabled for (i8) {
  fn setQuitLockEnabled(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18setQuitLockEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QCoreApplication18setQuitLockEnabledEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn hasPendingEvents<T: QCoreApplication_hasPendingEvents>(&mut self, value: T) -> i8 {
    return value.hasPendingEvents(self);
    // return 1;
  }
}

pub trait QCoreApplication_hasPendingEvents {
  fn hasPendingEvents(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::hasPendingEvents();
impl<'a> /*trait*/ QCoreApplication_hasPendingEvents for () {
  fn hasPendingEvents(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16hasPendingEventsEv()};
    let mut ret = unsafe {_ZN16QCoreApplication16hasPendingEventsEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setOrganizationDomain<T: QCoreApplication_setOrganizationDomain>(&mut self, value: T)  {
     value.setOrganizationDomain(self);
    // return 1;
  }
}

pub trait QCoreApplication_setOrganizationDomain {
  fn setOrganizationDomain(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setOrganizationDomain(const QString & orgDomain);
impl<'a> /*trait*/ QCoreApplication_setOrganizationDomain for (&'a  QString) {
  fn setOrganizationDomain(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication21setOrganizationDomainERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication21setOrganizationDomainERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn FreeQCoreApplication<T: QCoreApplication_FreeQCoreApplication>(&mut self, value: T)  {
     value.FreeQCoreApplication(self);
    // return 1;
  }
}

pub trait QCoreApplication_FreeQCoreApplication {
  fn FreeQCoreApplication(self, rsthis: &mut QCoreApplication) ;
}

// proto:  void QCoreApplication::FreeQCoreApplication();
impl<'a> /*trait*/ QCoreApplication_FreeQCoreApplication for () {
  fn FreeQCoreApplication(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationD0Ev()};
     unsafe {_ZN16QCoreApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationDomainChanged<T: QCoreApplication_organizationDomainChanged>(&mut self, value: T)  {
     value.organizationDomainChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_organizationDomainChanged {
  fn organizationDomainChanged(self, rsthis: &mut QCoreApplication) ;
}

// proto:  void QCoreApplication::organizationDomainChanged();
impl<'a> /*trait*/ QCoreApplication_organizationDomainChanged for () {
  fn organizationDomainChanged(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication25organizationDomainChangedEv()};
     unsafe {_ZN16QCoreApplication25organizationDomainChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationDomain<T: QCoreApplication_organizationDomain>(&mut self, value: T) -> QString {
    return value.organizationDomain(self);
    // return 1;
  }
}

pub trait QCoreApplication_organizationDomain {
  fn organizationDomain(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::organizationDomain();
impl<'a> /*trait*/ QCoreApplication_organizationDomain for () {
  fn organizationDomain(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18organizationDomainEv()};
    let mut ret = unsafe {_ZN16QCoreApplication18organizationDomainEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn installTranslator<T: QCoreApplication_installTranslator>(&mut self, value: T) -> i8 {
    return value.installTranslator(self);
    // return 1;
  }
}

pub trait QCoreApplication_installTranslator {
  fn installTranslator(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::installTranslator(QTranslator * messageFile);
impl<'a> /*trait*/ QCoreApplication_installTranslator for (&'a mut QTranslator) {
  fn installTranslator(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17installTranslatorEP11QTranslator()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication17installTranslatorEP11QTranslator(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationDirPath<T: QCoreApplication_applicationDirPath>(&mut self, value: T) -> QString {
    return value.applicationDirPath(self);
    // return 1;
  }
}

pub trait QCoreApplication_applicationDirPath {
  fn applicationDirPath(self, rsthis: &mut QCoreApplication) -> QString;
}

// proto: static QString QCoreApplication::applicationDirPath();
impl<'a> /*trait*/ QCoreApplication_applicationDirPath for () {
  fn applicationDirPath(self, rsthis: &mut QCoreApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18applicationDirPathEv()};
    let mut ret = unsafe {_ZN16QCoreApplication18applicationDirPathEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn flush<T: QCoreApplication_flush>(&mut self, value: T)  {
     value.flush(self);
    // return 1;
  }
}

pub trait QCoreApplication_flush {
  fn flush(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::flush();
impl<'a> /*trait*/ QCoreApplication_flush for () {
  fn flush(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication5flushEv()};
     unsafe {_ZN16QCoreApplication5flushEv()};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn exec<T: QCoreApplication_exec>(&mut self, value: T) -> i32 {
    return value.exec(self);
    // return 1;
  }
}

pub trait QCoreApplication_exec {
  fn exec(self, rsthis: &mut QCoreApplication) -> i32;
}

// proto: static int QCoreApplication::exec();
impl<'a> /*trait*/ QCoreApplication_exec for () {
  fn exec(self, rsthis: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4execEv()};
    let mut ret = unsafe {_ZN16QCoreApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn arguments<T: QCoreApplication_arguments>(&mut self, value: T)  {
     value.arguments(self);
    // return 1;
  }
}

pub trait QCoreApplication_arguments {
  fn arguments(self, rsthis: &mut QCoreApplication) ;
}

// proto: static QStringList QCoreApplication::arguments();
impl<'a> /*trait*/ QCoreApplication_arguments for () {
  fn arguments(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9argumentsEv()};
     unsafe {_ZN16QCoreApplication9argumentsEv()};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setLibraryPaths<T: QCoreApplication_setLibraryPaths>(&mut self, value: T)  {
     value.setLibraryPaths(self);
    // return 1;
  }
}

pub trait QCoreApplication_setLibraryPaths {
  fn setLibraryPaths(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setLibraryPaths(const QStringList & );
impl<'a> /*trait*/ QCoreApplication_setLibraryPaths for (&'a  QStringList) {
  fn setLibraryPaths(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15setLibraryPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication15setLibraryPathsERK11QStringList(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn eventDispatcher<T: QCoreApplication_eventDispatcher>(&mut self, value: T)  {
     value.eventDispatcher(self);
    // return 1;
  }
}

pub trait QCoreApplication_eventDispatcher {
  fn eventDispatcher(self, rsthis: &mut QCoreApplication) ;
}

// proto: static QAbstractEventDispatcher * QCoreApplication::eventDispatcher();
impl<'a> /*trait*/ QCoreApplication_eventDispatcher for () {
  fn eventDispatcher(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15eventDispatcherEv()};
     unsafe {_ZN16QCoreApplication15eventDispatcherEv()};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn startingUp<T: QCoreApplication_startingUp>(&mut self, value: T) -> i8 {
    return value.startingUp(self);
    // return 1;
  }
}

pub trait QCoreApplication_startingUp {
  fn startingUp(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::startingUp();
impl<'a> /*trait*/ QCoreApplication_startingUp for () {
  fn startingUp(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication10startingUpEv()};
    let mut ret = unsafe {_ZN16QCoreApplication10startingUpEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn sendEvent<T: QCoreApplication_sendEvent>(&mut self, value: T) -> i8 {
    return value.sendEvent(self);
    // return 1;
  }
}

pub trait QCoreApplication_sendEvent {
  fn sendEvent(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::sendEvent(QObject * receiver, QEvent * event);
impl<'a> /*trait*/ QCoreApplication_sendEvent for (&'a mut QObject, &'a mut QEvent) {
  fn sendEvent(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn notify<T: QCoreApplication_notify>(&mut self, value: T) -> i8 {
    return value.notify(self);
    // return 1;
  }
}

pub trait QCoreApplication_notify {
  fn notify(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto:  bool QCoreApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QCoreApplication_notify for (&'a mut QObject, &'a mut QEvent) {
  fn notify(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QCoreApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn isQuitLockEnabled<T: QCoreApplication_isQuitLockEnabled>(&mut self, value: T) -> i8 {
    return value.isQuitLockEnabled(self);
    // return 1;
  }
}

pub trait QCoreApplication_isQuitLockEnabled {
  fn isQuitLockEnabled(self, rsthis: &mut QCoreApplication) -> i8;
}

// proto: static bool QCoreApplication::isQuitLockEnabled();
impl<'a> /*trait*/ QCoreApplication_isQuitLockEnabled for () {
  fn isQuitLockEnabled(self, rsthis: &mut QCoreApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17isQuitLockEnabledEv()};
    let mut ret = unsafe {_ZN16QCoreApplication17isQuitLockEnabledEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationNameChanged<T: QCoreApplication_organizationNameChanged>(&mut self, value: T)  {
     value.organizationNameChanged(self);
    // return 1;
  }
}

pub trait QCoreApplication_organizationNameChanged {
  fn organizationNameChanged(self, rsthis: &mut QCoreApplication) ;
}

// proto:  void QCoreApplication::organizationNameChanged();
impl<'a> /*trait*/ QCoreApplication_organizationNameChanged for () {
  fn organizationNameChanged(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication23organizationNameChangedEv()};
     unsafe {_ZN16QCoreApplication23organizationNameChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn removePostedEvents<T: QCoreApplication_removePostedEvents>(&mut self, value: T)  {
     value.removePostedEvents(self);
    // return 1;
  }
}

pub trait QCoreApplication_removePostedEvents {
  fn removePostedEvents(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::removePostedEvents(QObject * receiver, int eventType);
impl<'a> /*trait*/ QCoreApplication_removePostedEvents for (&'a mut QObject, i32) {
  fn removePostedEvents(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18removePostedEventsEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QCoreApplication18removePostedEventsEP7QObjecti(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn metaObject<T: QCoreApplication_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QCoreApplication_metaObject {
  fn metaObject(self, rsthis: &mut QCoreApplication) ;
}

// proto:  const QMetaObject * QCoreApplication::metaObject();
impl<'a> /*trait*/ QCoreApplication_metaObject for () {
  fn metaObject(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QCoreApplication10metaObjectEv()};
     unsafe {_ZNK16QCoreApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QCoreApplication::NewQCoreApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QCoreApplication_NewQCoreApplication for (&'a mut i32, &'a mut String, i32) {
  fn NewQCoreApplication(self) -> QCoreApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationC1ERiPPci()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QCoreApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let rsthis = QCoreApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setApplicationVersion<T: QCoreApplication_setApplicationVersion>(&mut self, value: T)  {
     value.setApplicationVersion(self);
    // return 1;
  }
}

pub trait QCoreApplication_setApplicationVersion {
  fn setApplicationVersion(self, rsthis: &mut QCoreApplication) ;
}

// proto: static void QCoreApplication::setApplicationVersion(const QString & version);
impl<'a> /*trait*/ QCoreApplication_setApplicationVersion for (&'a  QString) {
  fn setApplicationVersion(self, rsthis: &mut QCoreApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication21setApplicationVersionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCoreApplication21setApplicationVersionERK7QString(arg0)};
    // return 1;
  }
}

