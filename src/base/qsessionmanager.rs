// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstringlist::QStringList;
use super::qstring::QString;
use super::qguiapplication::QGuiApplication;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QSessionManager::sessionId();
  fn _ZNK15QSessionManager9sessionIdEv() -> i32;
  // proto: QString QSessionManager::sessionKey();
  fn _ZNK15QSessionManager10sessionKeyEv() -> i32;
  // proto: void QSessionManager::setRestartCommand(const QStringList & );
  fn _ZN15QSessionManager17setRestartCommandERK11QStringList(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QSessionManager::metaObject();
  fn _ZNK15QSessionManager10metaObjectEv() -> i32;
  // proto: bool QSessionManager::allowsErrorInteraction();
  fn _ZN15QSessionManager22allowsErrorInteractionEv() -> i32;
  // proto: void QSessionManager::FreeQSessionManager();
  fn _ZN15QSessionManagerD0Ev() -> i32;
  // proto: QStringList QSessionManager::restartCommand();
  fn _ZNK15QSessionManager14restartCommandEv() -> i32;
  // proto: void QSessionManager::requestPhase2();
  fn _ZN15QSessionManager13requestPhase2Ev() -> i32;
  // proto: bool QSessionManager::isPhase2();
  fn _ZNK15QSessionManager8isPhase2Ev() -> i32;
  // proto: void QSessionManager::release();
  fn _ZN15QSessionManager7releaseEv() -> i32;
  // proto: void QSessionManager::setManagerProperty(const QString & name, const QString & value);
  fn _ZN15QSessionManager18setManagerPropertyERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
  fn _ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QStringList QSessionManager::discardCommand();
  fn _ZNK15QSessionManager14discardCommandEv() -> i32;
  // proto: void QSessionManager::NewQSessionManager(QGuiApplication * app, QString & id, QString & key);
  fn _ZN15QSessionManagerC1EP15QGuiApplicationR7QStringS3_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> i32;
  // proto: void QSessionManager::cancel();
  fn _ZN15QSessionManager6cancelEv() -> i32;
  // proto: void QSessionManager::setDiscardCommand(const QStringList & );
  fn _ZN15QSessionManager17setDiscardCommandERK11QStringList(arg0: *const c_void) -> i32;
  // proto: bool QSessionManager::allowsInteraction();
  fn _ZN15QSessionManager17allowsInteractionEv() -> i32;
}

// body block begin
// class sizeof(QSessionManager)=1
pub struct QSessionManager {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSessionManager {
  pub fn sessionId<T: QSessionManager_sessionId>(&mut self, value: T) -> i32 {
    value.sessionId(self);
    return 1;
  }
}

pub trait QSessionManager_sessionId {
  fn sessionId(self, this: &mut QSessionManager) -> i32;
}

// proto: QString QSessionManager::sessionId();
impl<'a> /*trait*/ QSessionManager_sessionId for () {
  fn sessionId(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager9sessionIdEv()};
    unsafe {_ZNK15QSessionManager9sessionIdEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn sessionKey<T: QSessionManager_sessionKey>(&mut self, value: T) -> i32 {
    value.sessionKey(self);
    return 1;
  }
}

pub trait QSessionManager_sessionKey {
  fn sessionKey(self, this: &mut QSessionManager) -> i32;
}

// proto: QString QSessionManager::sessionKey();
impl<'a> /*trait*/ QSessionManager_sessionKey for () {
  fn sessionKey(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10sessionKeyEv()};
    unsafe {_ZNK15QSessionManager10sessionKeyEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setRestartCommand<T: QSessionManager_setRestartCommand>(&mut self, value: T) -> i32 {
    value.setRestartCommand(self);
    return 1;
  }
}

pub trait QSessionManager_setRestartCommand {
  fn setRestartCommand(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::setRestartCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setRestartCommand for (&'a  QStringList) {
  fn setRestartCommand(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setRestartCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSessionManager17setRestartCommandERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn metaObject<T: QSessionManager_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSessionManager_metaObject {
  fn metaObject(self, this: &mut QSessionManager) -> i32;
}

// proto: const QMetaObject * QSessionManager::metaObject();
impl<'a> /*trait*/ QSessionManager_metaObject for () {
  fn metaObject(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10metaObjectEv()};
    unsafe {_ZNK15QSessionManager10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn allowsErrorInteraction<T: QSessionManager_allowsErrorInteraction>(&mut self, value: T) -> i32 {
    value.allowsErrorInteraction(self);
    return 1;
  }
}

pub trait QSessionManager_allowsErrorInteraction {
  fn allowsErrorInteraction(self, this: &mut QSessionManager) -> i32;
}

// proto: bool QSessionManager::allowsErrorInteraction();
impl<'a> /*trait*/ QSessionManager_allowsErrorInteraction for () {
  fn allowsErrorInteraction(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager22allowsErrorInteractionEv()};
    unsafe {_ZN15QSessionManager22allowsErrorInteractionEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn FreeQSessionManager<T: QSessionManager_FreeQSessionManager>(&mut self, value: T) -> i32 {
    value.FreeQSessionManager(self);
    return 1;
  }
}

pub trait QSessionManager_FreeQSessionManager {
  fn FreeQSessionManager(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::FreeQSessionManager();
impl<'a> /*trait*/ QSessionManager_FreeQSessionManager for () {
  fn FreeQSessionManager(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManagerD0Ev()};
    unsafe {_ZN15QSessionManagerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn restartCommand<T: QSessionManager_restartCommand>(&mut self, value: T) -> i32 {
    value.restartCommand(self);
    return 1;
  }
}

pub trait QSessionManager_restartCommand {
  fn restartCommand(self, this: &mut QSessionManager) -> i32;
}

// proto: QStringList QSessionManager::restartCommand();
impl<'a> /*trait*/ QSessionManager_restartCommand for () {
  fn restartCommand(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14restartCommandEv()};
    unsafe {_ZNK15QSessionManager14restartCommandEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn requestPhase2<T: QSessionManager_requestPhase2>(&mut self, value: T) -> i32 {
    value.requestPhase2(self);
    return 1;
  }
}

pub trait QSessionManager_requestPhase2 {
  fn requestPhase2(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::requestPhase2();
impl<'a> /*trait*/ QSessionManager_requestPhase2 for () {
  fn requestPhase2(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager13requestPhase2Ev()};
    unsafe {_ZN15QSessionManager13requestPhase2Ev()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn isPhase2<T: QSessionManager_isPhase2>(&mut self, value: T) -> i32 {
    value.isPhase2(self);
    return 1;
  }
}

pub trait QSessionManager_isPhase2 {
  fn isPhase2(self, this: &mut QSessionManager) -> i32;
}

// proto: bool QSessionManager::isPhase2();
impl<'a> /*trait*/ QSessionManager_isPhase2 for () {
  fn isPhase2(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager8isPhase2Ev()};
    unsafe {_ZNK15QSessionManager8isPhase2Ev()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn release<T: QSessionManager_release>(&mut self, value: T) -> i32 {
    value.release(self);
    return 1;
  }
}

pub trait QSessionManager_release {
  fn release(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::release();
impl<'a> /*trait*/ QSessionManager_release for () {
  fn release(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager7releaseEv()};
    unsafe {_ZN15QSessionManager7releaseEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setManagerProperty<T: QSessionManager_setManagerProperty>(&mut self, value: T) -> i32 {
    value.setManagerProperty(self);
    return 1;
  }
}

pub trait QSessionManager_setManagerProperty {
  fn setManagerProperty(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::setManagerProperty(const QString & name, const QString & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty for (&'a  QString, &'a  QString) {
  fn setManagerProperty(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QSessionManager18setManagerPropertyERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty for (&'a  QString, &'a  QStringList) {
  fn setManagerProperty(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn discardCommand<T: QSessionManager_discardCommand>(&mut self, value: T) -> i32 {
    value.discardCommand(self);
    return 1;
  }
}

pub trait QSessionManager_discardCommand {
  fn discardCommand(self, this: &mut QSessionManager) -> i32;
}

// proto: QStringList QSessionManager::discardCommand();
impl<'a> /*trait*/ QSessionManager_discardCommand for () {
  fn discardCommand(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14discardCommandEv()};
    unsafe {_ZNK15QSessionManager14discardCommandEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn NewQSessionManager<T: QSessionManager_NewQSessionManager>(value: T) -> QSessionManager {
    let rsthis = value.NewQSessionManager();
    return rsthis;
    // return 1;
  }
}

pub trait QSessionManager_NewQSessionManager {
  fn NewQSessionManager(self) -> QSessionManager;
}

// proto: void QSessionManager::NewQSessionManager(QGuiApplication * app, QString & id, QString & key);
impl<'a> /*trait*/ QSessionManager_NewQSessionManager for (&'a mut QGuiApplication, &'a mut QString, &'a mut QString) {
  fn NewQSessionManager(self) -> QSessionManager {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManagerC1EP15QGuiApplicationR7QStringS3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN15QSessionManagerC1EP15QGuiApplicationR7QStringS3_(qthis, arg0, arg1, arg2)};
    let rsthis = QSessionManager{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn cancel<T: QSessionManager_cancel>(&mut self, value: T) -> i32 {
    value.cancel(self);
    return 1;
  }
}

pub trait QSessionManager_cancel {
  fn cancel(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::cancel();
impl<'a> /*trait*/ QSessionManager_cancel for () {
  fn cancel(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager6cancelEv()};
    unsafe {_ZN15QSessionManager6cancelEv()};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setDiscardCommand<T: QSessionManager_setDiscardCommand>(&mut self, value: T) -> i32 {
    value.setDiscardCommand(self);
    return 1;
  }
}

pub trait QSessionManager_setDiscardCommand {
  fn setDiscardCommand(self, this: &mut QSessionManager) -> i32;
}

// proto: void QSessionManager::setDiscardCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setDiscardCommand for (&'a  QStringList) {
  fn setDiscardCommand(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setDiscardCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSessionManager17setDiscardCommandERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn allowsInteraction<T: QSessionManager_allowsInteraction>(&mut self, value: T) -> i32 {
    value.allowsInteraction(self);
    return 1;
  }
}

pub trait QSessionManager_allowsInteraction {
  fn allowsInteraction(self, this: &mut QSessionManager) -> i32;
}

// proto: bool QSessionManager::allowsInteraction();
impl<'a> /*trait*/ QSessionManager_allowsInteraction for () {
  fn allowsInteraction(self, this: &mut QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17allowsInteractionEv()};
    unsafe {_ZN15QSessionManager17allowsInteractionEv()};
    return 1;
  }
}

