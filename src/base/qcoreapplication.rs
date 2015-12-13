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
  fn _ZN16QCoreApplication16sendPostedEventsEP7QObjecti(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZN16QCoreApplication14addLibraryPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplication14applicationPidEv() -> i32;
  fn _ZN16QCoreApplication18setApplicationNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplication16organizationNameEv() -> i32;
  fn _ZN16QCoreApplication8instanceEv() -> i32;
  fn _ZN16QCoreApplication15isSetuidAllowedEv() -> i32;
  fn _ZN16QCoreApplication25applicationVersionChangedEv() -> i32;
  fn _ZN16QCoreApplication15applicationNameEv() -> i32;
  fn _ZN16QCoreApplicationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplication16setSetuidAllowedEb(arg0: int8_t) -> i32;
  fn _ZN16QCoreApplication9postEventEP7QObjectP6QEventi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> i32;
  fn _ZN16QCoreApplication12libraryPathsEv() -> i32;
  fn _ZN16QCoreApplication22applicationNameChangedEv() -> i32;
  fn _ZN16QCoreApplication17removeLibraryPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplication9translateEPKcS1_S1_i(arg0: *const c_char, arg1: *const c_char, arg2: *const c_char, arg3: c_int) -> i32;
  fn _ZN16QCoreApplication19applicationFilePathEv() -> i32;
  fn _ZN16QCoreApplication16removeTranslatorEP11QTranslator(arg0: *mut c_void) -> i32;
  fn _ZN16QCoreApplication19setOrganizationNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplication4exitEi(arg0: c_int) -> i32;
  fn _ZN16QCoreApplication18applicationVersionEv() -> i32;
  fn _ZN16QCoreApplication4quitEv() -> i32;
  fn _ZN16QCoreApplication11closingDownEv() -> i32;
  fn _ZN16QCoreApplication18setQuitLockEnabledEb(arg0: int8_t) -> i32;
  fn _ZN16QCoreApplication16hasPendingEventsEv() -> i32;
  fn _ZN16QCoreApplication21setOrganizationDomainERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplicationD0Ev() -> i32;
  fn _ZN16QCoreApplication25organizationDomainChangedEv() -> i32;
  fn _ZN16QCoreApplication18organizationDomainEv() -> i32;
  fn _ZN16QCoreApplication17installTranslatorEP11QTranslator(arg0: *mut c_void) -> i32;
  fn _ZN16QCoreApplication18applicationDirPathEv() -> i32;
  fn _ZN16QCoreApplication5flushEv() -> i32;
  fn _ZN16QCoreApplication4execEv() -> i32;
  fn _ZN16QCoreApplication9argumentsEv() -> i32;
  fn _ZN16QCoreApplication15setLibraryPathsERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN16QCoreApplication15eventDispatcherEv() -> i32;
  fn _ZN16QCoreApplication10startingUpEv() -> i32;
  fn _ZN16QCoreApplication9sendEventEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN16QCoreApplication6notifyEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN16QCoreApplication17isQuitLockEnabledEv() -> i32;
  fn _ZN16QCoreApplication23organizationNameChangedEv() -> i32;
  fn _ZN16QCoreApplication18removePostedEventsEP7QObjecti(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK16QCoreApplication10metaObjectEv() -> i32;
  fn _ZN16QCoreApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> i32;
  fn _ZN16QCoreApplication21setApplicationVersionERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QCoreApplication)=1
pub struct QCoreApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCoreApplication {
  pub fn sendPostedEvents<T: QCoreApplication_sendPostedEvents>(&mut self, value: T) -> i32 {
    value.sendPostedEvents(self);
    return 1;
  }
}

pub trait QCoreApplication_sendPostedEvents {
  fn sendPostedEvents(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::sendPostedEvents(QObject * receiver, int event_type);
impl<'a> /*trait*/ QCoreApplication_sendPostedEvents for (&'a mut QObject, i32) {
  fn sendPostedEvents(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16sendPostedEventsEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QCoreApplication16sendPostedEventsEP7QObjecti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn addLibraryPath<T: QCoreApplication_addLibraryPath>(&mut self, value: T) -> i32 {
    value.addLibraryPath(self);
    return 1;
  }
}

pub trait QCoreApplication_addLibraryPath {
  fn addLibraryPath(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::addLibraryPath(const QString & );
impl<'a> /*trait*/ QCoreApplication_addLibraryPath for (&'a  QString) {
  fn addLibraryPath(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication14addLibraryPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication14addLibraryPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationPid<T: QCoreApplication_applicationPid>(&mut self, value: T) -> i32 {
    value.applicationPid(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationPid {
  fn applicationPid(self, this: &mut QCoreApplication) -> i32;
}

// proto: long long QCoreApplication::applicationPid();
impl<'a> /*trait*/ QCoreApplication_applicationPid for () {
  fn applicationPid(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication14applicationPidEv()};
    unsafe {_ZN16QCoreApplication14applicationPidEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setApplicationName<T: QCoreApplication_setApplicationName>(&mut self, value: T) -> i32 {
    value.setApplicationName(self);
    return 1;
  }
}

pub trait QCoreApplication_setApplicationName {
  fn setApplicationName(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setApplicationName(const QString & application);
impl<'a> /*trait*/ QCoreApplication_setApplicationName for (&'a  QString) {
  fn setApplicationName(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18setApplicationNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication18setApplicationNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationName<T: QCoreApplication_organizationName>(&mut self, value: T) -> i32 {
    value.organizationName(self);
    return 1;
  }
}

pub trait QCoreApplication_organizationName {
  fn organizationName(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::organizationName();
impl<'a> /*trait*/ QCoreApplication_organizationName for () {
  fn organizationName(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16organizationNameEv()};
    unsafe {_ZN16QCoreApplication16organizationNameEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn instance<T: QCoreApplication_instance>(&mut self, value: T) -> i32 {
    value.instance(self);
    return 1;
  }
}

pub trait QCoreApplication_instance {
  fn instance(self, this: &mut QCoreApplication) -> i32;
}

// proto: QCoreApplication * QCoreApplication::instance();
impl<'a> /*trait*/ QCoreApplication_instance for () {
  fn instance(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication8instanceEv()};
    unsafe {_ZN16QCoreApplication8instanceEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn isSetuidAllowed<T: QCoreApplication_isSetuidAllowed>(&mut self, value: T) -> i32 {
    value.isSetuidAllowed(self);
    return 1;
  }
}

pub trait QCoreApplication_isSetuidAllowed {
  fn isSetuidAllowed(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::isSetuidAllowed();
impl<'a> /*trait*/ QCoreApplication_isSetuidAllowed for () {
  fn isSetuidAllowed(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15isSetuidAllowedEv()};
    unsafe {_ZN16QCoreApplication15isSetuidAllowedEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationVersionChanged<T: QCoreApplication_applicationVersionChanged>(&mut self, value: T) -> i32 {
    value.applicationVersionChanged(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationVersionChanged {
  fn applicationVersionChanged(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::applicationVersionChanged();
impl<'a> /*trait*/ QCoreApplication_applicationVersionChanged for () {
  fn applicationVersionChanged(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication25applicationVersionChangedEv()};
    unsafe {_ZN16QCoreApplication25applicationVersionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationName<T: QCoreApplication_applicationName>(&mut self, value: T) -> i32 {
    value.applicationName(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationName {
  fn applicationName(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::applicationName();
impl<'a> /*trait*/ QCoreApplication_applicationName for () {
  fn applicationName(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15applicationNameEv()};
    unsafe {_ZN16QCoreApplication15applicationNameEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplicationC1ERKS_(qthis, arg0)};
    let rsthis = QCoreApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setSetuidAllowed<T: QCoreApplication_setSetuidAllowed>(&mut self, value: T) -> i32 {
    value.setSetuidAllowed(self);
    return 1;
  }
}

pub trait QCoreApplication_setSetuidAllowed {
  fn setSetuidAllowed(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setSetuidAllowed(bool allow);
impl<'a> /*trait*/ QCoreApplication_setSetuidAllowed for (i8) {
  fn setSetuidAllowed(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16setSetuidAllowedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QCoreApplication16setSetuidAllowedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn postEvent<T: QCoreApplication_postEvent>(&mut self, value: T) -> i32 {
    value.postEvent(self);
    return 1;
  }
}

pub trait QCoreApplication_postEvent {
  fn postEvent(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::postEvent(QObject * receiver, QEvent * event, int priority);
impl<'a> /*trait*/ QCoreApplication_postEvent for (&'a mut QObject, &'a mut QEvent, i32) {
  fn postEvent(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9postEventEP7QObjectP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QCoreApplication9postEventEP7QObjectP6QEventi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn libraryPaths<T: QCoreApplication_libraryPaths>(&mut self, value: T) -> i32 {
    value.libraryPaths(self);
    return 1;
  }
}

pub trait QCoreApplication_libraryPaths {
  fn libraryPaths(self, this: &mut QCoreApplication) -> i32;
}

// proto: QStringList QCoreApplication::libraryPaths();
impl<'a> /*trait*/ QCoreApplication_libraryPaths for () {
  fn libraryPaths(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication12libraryPathsEv()};
    unsafe {_ZN16QCoreApplication12libraryPathsEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationNameChanged<T: QCoreApplication_applicationNameChanged>(&mut self, value: T) -> i32 {
    value.applicationNameChanged(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationNameChanged {
  fn applicationNameChanged(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::applicationNameChanged();
impl<'a> /*trait*/ QCoreApplication_applicationNameChanged for () {
  fn applicationNameChanged(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication22applicationNameChangedEv()};
    unsafe {_ZN16QCoreApplication22applicationNameChangedEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn removeLibraryPath<T: QCoreApplication_removeLibraryPath>(&mut self, value: T) -> i32 {
    value.removeLibraryPath(self);
    return 1;
  }
}

pub trait QCoreApplication_removeLibraryPath {
  fn removeLibraryPath(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::removeLibraryPath(const QString & );
impl<'a> /*trait*/ QCoreApplication_removeLibraryPath for (&'a  QString) {
  fn removeLibraryPath(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17removeLibraryPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication17removeLibraryPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn translate<T: QCoreApplication_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QCoreApplication_translate {
  fn translate(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::translate(const char * context, const char * key, const char * disambiguation, int n);
impl<'a> /*trait*/ QCoreApplication_translate for (&'a  String, &'a  String, &'a  String, i32) {
  fn translate(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9translateEPKcS1_S1_i()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as c_int;
    unsafe {_ZN16QCoreApplication9translateEPKcS1_S1_i(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationFilePath<T: QCoreApplication_applicationFilePath>(&mut self, value: T) -> i32 {
    value.applicationFilePath(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationFilePath {
  fn applicationFilePath(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::applicationFilePath();
impl<'a> /*trait*/ QCoreApplication_applicationFilePath for () {
  fn applicationFilePath(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication19applicationFilePathEv()};
    unsafe {_ZN16QCoreApplication19applicationFilePathEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn removeTranslator<T: QCoreApplication_removeTranslator>(&mut self, value: T) -> i32 {
    value.removeTranslator(self);
    return 1;
  }
}

pub trait QCoreApplication_removeTranslator {
  fn removeTranslator(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::removeTranslator(QTranslator * messageFile);
impl<'a> /*trait*/ QCoreApplication_removeTranslator for (&'a mut QTranslator) {
  fn removeTranslator(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16removeTranslatorEP11QTranslator()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QCoreApplication16removeTranslatorEP11QTranslator(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setOrganizationName<T: QCoreApplication_setOrganizationName>(&mut self, value: T) -> i32 {
    value.setOrganizationName(self);
    return 1;
  }
}

pub trait QCoreApplication_setOrganizationName {
  fn setOrganizationName(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setOrganizationName(const QString & orgName);
impl<'a> /*trait*/ QCoreApplication_setOrganizationName for (&'a  QString) {
  fn setOrganizationName(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication19setOrganizationNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication19setOrganizationNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn exit<T: QCoreApplication_exit>(&mut self, value: T) -> i32 {
    value.exit(self);
    return 1;
  }
}

pub trait QCoreApplication_exit {
  fn exit(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::exit(int retcode);
impl<'a> /*trait*/ QCoreApplication_exit for (i32) {
  fn exit(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4exitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QCoreApplication4exitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationVersion<T: QCoreApplication_applicationVersion>(&mut self, value: T) -> i32 {
    value.applicationVersion(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationVersion {
  fn applicationVersion(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::applicationVersion();
impl<'a> /*trait*/ QCoreApplication_applicationVersion for () {
  fn applicationVersion(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18applicationVersionEv()};
    unsafe {_ZN16QCoreApplication18applicationVersionEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn quit<T: QCoreApplication_quit>(&mut self, value: T) -> i32 {
    value.quit(self);
    return 1;
  }
}

pub trait QCoreApplication_quit {
  fn quit(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::quit();
impl<'a> /*trait*/ QCoreApplication_quit for () {
  fn quit(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4quitEv()};
    unsafe {_ZN16QCoreApplication4quitEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn closingDown<T: QCoreApplication_closingDown>(&mut self, value: T) -> i32 {
    value.closingDown(self);
    return 1;
  }
}

pub trait QCoreApplication_closingDown {
  fn closingDown(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::closingDown();
impl<'a> /*trait*/ QCoreApplication_closingDown for () {
  fn closingDown(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication11closingDownEv()};
    unsafe {_ZN16QCoreApplication11closingDownEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setQuitLockEnabled<T: QCoreApplication_setQuitLockEnabled>(&mut self, value: T) -> i32 {
    value.setQuitLockEnabled(self);
    return 1;
  }
}

pub trait QCoreApplication_setQuitLockEnabled {
  fn setQuitLockEnabled(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setQuitLockEnabled(bool enabled);
impl<'a> /*trait*/ QCoreApplication_setQuitLockEnabled for (i8) {
  fn setQuitLockEnabled(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18setQuitLockEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QCoreApplication18setQuitLockEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn hasPendingEvents<T: QCoreApplication_hasPendingEvents>(&mut self, value: T) -> i32 {
    value.hasPendingEvents(self);
    return 1;
  }
}

pub trait QCoreApplication_hasPendingEvents {
  fn hasPendingEvents(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::hasPendingEvents();
impl<'a> /*trait*/ QCoreApplication_hasPendingEvents for () {
  fn hasPendingEvents(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication16hasPendingEventsEv()};
    unsafe {_ZN16QCoreApplication16hasPendingEventsEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setOrganizationDomain<T: QCoreApplication_setOrganizationDomain>(&mut self, value: T) -> i32 {
    value.setOrganizationDomain(self);
    return 1;
  }
}

pub trait QCoreApplication_setOrganizationDomain {
  fn setOrganizationDomain(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setOrganizationDomain(const QString & orgDomain);
impl<'a> /*trait*/ QCoreApplication_setOrganizationDomain for (&'a  QString) {
  fn setOrganizationDomain(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication21setOrganizationDomainERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication21setOrganizationDomainERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn FreeQCoreApplication<T: QCoreApplication_FreeQCoreApplication>(&mut self, value: T) -> i32 {
    value.FreeQCoreApplication(self);
    return 1;
  }
}

pub trait QCoreApplication_FreeQCoreApplication {
  fn FreeQCoreApplication(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::FreeQCoreApplication();
impl<'a> /*trait*/ QCoreApplication_FreeQCoreApplication for () {
  fn FreeQCoreApplication(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplicationD0Ev()};
    unsafe {_ZN16QCoreApplicationD0Ev()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationDomainChanged<T: QCoreApplication_organizationDomainChanged>(&mut self, value: T) -> i32 {
    value.organizationDomainChanged(self);
    return 1;
  }
}

pub trait QCoreApplication_organizationDomainChanged {
  fn organizationDomainChanged(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::organizationDomainChanged();
impl<'a> /*trait*/ QCoreApplication_organizationDomainChanged for () {
  fn organizationDomainChanged(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication25organizationDomainChangedEv()};
    unsafe {_ZN16QCoreApplication25organizationDomainChangedEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationDomain<T: QCoreApplication_organizationDomain>(&mut self, value: T) -> i32 {
    value.organizationDomain(self);
    return 1;
  }
}

pub trait QCoreApplication_organizationDomain {
  fn organizationDomain(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::organizationDomain();
impl<'a> /*trait*/ QCoreApplication_organizationDomain for () {
  fn organizationDomain(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18organizationDomainEv()};
    unsafe {_ZN16QCoreApplication18organizationDomainEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn installTranslator<T: QCoreApplication_installTranslator>(&mut self, value: T) -> i32 {
    value.installTranslator(self);
    return 1;
  }
}

pub trait QCoreApplication_installTranslator {
  fn installTranslator(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::installTranslator(QTranslator * messageFile);
impl<'a> /*trait*/ QCoreApplication_installTranslator for (&'a mut QTranslator) {
  fn installTranslator(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17installTranslatorEP11QTranslator()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QCoreApplication17installTranslatorEP11QTranslator(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn applicationDirPath<T: QCoreApplication_applicationDirPath>(&mut self, value: T) -> i32 {
    value.applicationDirPath(self);
    return 1;
  }
}

pub trait QCoreApplication_applicationDirPath {
  fn applicationDirPath(self, this: &mut QCoreApplication) -> i32;
}

// proto: QString QCoreApplication::applicationDirPath();
impl<'a> /*trait*/ QCoreApplication_applicationDirPath for () {
  fn applicationDirPath(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18applicationDirPathEv()};
    unsafe {_ZN16QCoreApplication18applicationDirPathEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn flush<T: QCoreApplication_flush>(&mut self, value: T) -> i32 {
    value.flush(self);
    return 1;
  }
}

pub trait QCoreApplication_flush {
  fn flush(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::flush();
impl<'a> /*trait*/ QCoreApplication_flush for () {
  fn flush(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication5flushEv()};
    unsafe {_ZN16QCoreApplication5flushEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn exec<T: QCoreApplication_exec>(&mut self, value: T) -> i32 {
    value.exec(self);
    return 1;
  }
}

pub trait QCoreApplication_exec {
  fn exec(self, this: &mut QCoreApplication) -> i32;
}

// proto: int QCoreApplication::exec();
impl<'a> /*trait*/ QCoreApplication_exec for () {
  fn exec(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication4execEv()};
    unsafe {_ZN16QCoreApplication4execEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn arguments<T: QCoreApplication_arguments>(&mut self, value: T) -> i32 {
    value.arguments(self);
    return 1;
  }
}

pub trait QCoreApplication_arguments {
  fn arguments(self, this: &mut QCoreApplication) -> i32;
}

// proto: QStringList QCoreApplication::arguments();
impl<'a> /*trait*/ QCoreApplication_arguments for () {
  fn arguments(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9argumentsEv()};
    unsafe {_ZN16QCoreApplication9argumentsEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn setLibraryPaths<T: QCoreApplication_setLibraryPaths>(&mut self, value: T) -> i32 {
    value.setLibraryPaths(self);
    return 1;
  }
}

pub trait QCoreApplication_setLibraryPaths {
  fn setLibraryPaths(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setLibraryPaths(const QStringList & );
impl<'a> /*trait*/ QCoreApplication_setLibraryPaths for (&'a  QStringList) {
  fn setLibraryPaths(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15setLibraryPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication15setLibraryPathsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn eventDispatcher<T: QCoreApplication_eventDispatcher>(&mut self, value: T) -> i32 {
    value.eventDispatcher(self);
    return 1;
  }
}

pub trait QCoreApplication_eventDispatcher {
  fn eventDispatcher(self, this: &mut QCoreApplication) -> i32;
}

// proto: QAbstractEventDispatcher * QCoreApplication::eventDispatcher();
impl<'a> /*trait*/ QCoreApplication_eventDispatcher for () {
  fn eventDispatcher(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication15eventDispatcherEv()};
    unsafe {_ZN16QCoreApplication15eventDispatcherEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn startingUp<T: QCoreApplication_startingUp>(&mut self, value: T) -> i32 {
    value.startingUp(self);
    return 1;
  }
}

pub trait QCoreApplication_startingUp {
  fn startingUp(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::startingUp();
impl<'a> /*trait*/ QCoreApplication_startingUp for () {
  fn startingUp(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication10startingUpEv()};
    unsafe {_ZN16QCoreApplication10startingUpEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn sendEvent<T: QCoreApplication_sendEvent>(&mut self, value: T) -> i32 {
    value.sendEvent(self);
    return 1;
  }
}

pub trait QCoreApplication_sendEvent {
  fn sendEvent(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::sendEvent(QObject * receiver, QEvent * event);
impl<'a> /*trait*/ QCoreApplication_sendEvent for (&'a mut QObject, &'a mut QEvent) {
  fn sendEvent(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn notify<T: QCoreApplication_notify>(&mut self, value: T) -> i32 {
    value.notify(self);
    return 1;
  }
}

pub trait QCoreApplication_notify {
  fn notify(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QCoreApplication_notify for (&'a mut QObject, &'a mut QEvent) {
  fn notify(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QCoreApplication6notifyEP7QObjectP6QEvent(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn isQuitLockEnabled<T: QCoreApplication_isQuitLockEnabled>(&mut self, value: T) -> i32 {
    value.isQuitLockEnabled(self);
    return 1;
  }
}

pub trait QCoreApplication_isQuitLockEnabled {
  fn isQuitLockEnabled(self, this: &mut QCoreApplication) -> i32;
}

// proto: bool QCoreApplication::isQuitLockEnabled();
impl<'a> /*trait*/ QCoreApplication_isQuitLockEnabled for () {
  fn isQuitLockEnabled(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication17isQuitLockEnabledEv()};
    unsafe {_ZN16QCoreApplication17isQuitLockEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn organizationNameChanged<T: QCoreApplication_organizationNameChanged>(&mut self, value: T) -> i32 {
    value.organizationNameChanged(self);
    return 1;
  }
}

pub trait QCoreApplication_organizationNameChanged {
  fn organizationNameChanged(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::organizationNameChanged();
impl<'a> /*trait*/ QCoreApplication_organizationNameChanged for () {
  fn organizationNameChanged(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication23organizationNameChangedEv()};
    unsafe {_ZN16QCoreApplication23organizationNameChangedEv()};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn removePostedEvents<T: QCoreApplication_removePostedEvents>(&mut self, value: T) -> i32 {
    value.removePostedEvents(self);
    return 1;
  }
}

pub trait QCoreApplication_removePostedEvents {
  fn removePostedEvents(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::removePostedEvents(QObject * receiver, int eventType);
impl<'a> /*trait*/ QCoreApplication_removePostedEvents for (&'a mut QObject, i32) {
  fn removePostedEvents(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication18removePostedEventsEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QCoreApplication18removePostedEventsEP7QObjecti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QCoreApplication {
  pub fn metaObject<T: QCoreApplication_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QCoreApplication_metaObject {
  fn metaObject(self, this: &mut QCoreApplication) -> i32;
}

// proto: const QMetaObject * QCoreApplication::metaObject();
impl<'a> /*trait*/ QCoreApplication_metaObject for () {
  fn metaObject(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QCoreApplication10metaObjectEv()};
    unsafe {_ZNK16QCoreApplication10metaObjectEv()};
    return 1;
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
  pub fn setApplicationVersion<T: QCoreApplication_setApplicationVersion>(&mut self, value: T) -> i32 {
    value.setApplicationVersion(self);
    return 1;
  }
}

pub trait QCoreApplication_setApplicationVersion {
  fn setApplicationVersion(self, this: &mut QCoreApplication) -> i32;
}

// proto: void QCoreApplication::setApplicationVersion(const QString & version);
impl<'a> /*trait*/ QCoreApplication_setApplicationVersion for (&'a  QString) {
  fn setApplicationVersion(self, this: &mut QCoreApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCoreApplication21setApplicationVersionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCoreApplication21setApplicationVersionERK7QString(arg0)};
    return 1;
  }
}

