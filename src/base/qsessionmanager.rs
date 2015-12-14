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
use super::qguiapplication::QGuiApplication;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QSessionManager::sessionId();
  fn _ZNK15QSessionManager9sessionIdEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QSessionManager::sessionKey();
  fn _ZNK15QSessionManager10sessionKeyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSessionManager::setRestartCommand(const QStringList & );
  fn _ZN15QSessionManager17setRestartCommandERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QSessionManager::metaObject();
  fn _ZNK15QSessionManager10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QSessionManager::allowsErrorInteraction();
  fn _ZN15QSessionManager22allowsErrorInteractionEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSessionManager::FreeQSessionManager();
  fn _ZN15QSessionManagerD0Ev(qthis: *mut c_void) ;
  // proto:  QStringList QSessionManager::restartCommand();
  fn _ZNK15QSessionManager14restartCommandEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSessionManager::requestPhase2();
  fn _ZN15QSessionManager13requestPhase2Ev(qthis: *mut c_void) ;
  // proto:  bool QSessionManager::isPhase2();
  fn _ZNK15QSessionManager8isPhase2Ev(qthis: *mut c_void) -> int8_t;
  // proto:  void QSessionManager::release();
  fn _ZN15QSessionManager7releaseEv(qthis: *mut c_void) ;
  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QString & value);
  fn _ZN15QSessionManager18setManagerPropertyERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
  fn _ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QStringList QSessionManager::discardCommand();
  fn _ZNK15QSessionManager14discardCommandEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSessionManager::NewQSessionManager(QGuiApplication * app, QString & id, QString & key);
  fn _ZN15QSessionManagerC1EP15QGuiApplicationR7QStringS3_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QSessionManager::cancel();
  fn _ZN15QSessionManager6cancelEv(qthis: *mut c_void) ;
  // proto:  void QSessionManager::setDiscardCommand(const QStringList & );
  fn _ZN15QSessionManager17setDiscardCommandERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSessionManager::allowsInteraction();
  fn _ZN15QSessionManager17allowsInteractionEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QSessionManager)=1
pub struct QSessionManager {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSessionManager {
  pub fn sessionId<T: QSessionManager_sessionId>(&mut self, value: T) -> QString {
    return value.sessionId(self);
    // return 1;
  }
}

pub trait QSessionManager_sessionId {
  fn sessionId(self, rsthis: &mut QSessionManager) -> QString;
}

// proto:  QString QSessionManager::sessionId();
impl<'a> /*trait*/ QSessionManager_sessionId for () {
  fn sessionId(self, rsthis: &mut QSessionManager) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager9sessionIdEv()};
    let mut ret = unsafe {_ZNK15QSessionManager9sessionIdEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn sessionKey<T: QSessionManager_sessionKey>(&mut self, value: T) -> QString {
    return value.sessionKey(self);
    // return 1;
  }
}

pub trait QSessionManager_sessionKey {
  fn sessionKey(self, rsthis: &mut QSessionManager) -> QString;
}

// proto:  QString QSessionManager::sessionKey();
impl<'a> /*trait*/ QSessionManager_sessionKey for () {
  fn sessionKey(self, rsthis: &mut QSessionManager) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10sessionKeyEv()};
    let mut ret = unsafe {_ZNK15QSessionManager10sessionKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setRestartCommand<T: QSessionManager_setRestartCommand>(&mut self, value: T)  {
     value.setRestartCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_setRestartCommand {
  fn setRestartCommand(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::setRestartCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setRestartCommand for (&'a  QStringList) {
  fn setRestartCommand(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setRestartCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager17setRestartCommandERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn metaObject<T: QSessionManager_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSessionManager_metaObject {
  fn metaObject(self, rsthis: &mut QSessionManager) ;
}

// proto:  const QMetaObject * QSessionManager::metaObject();
impl<'a> /*trait*/ QSessionManager_metaObject for () {
  fn metaObject(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10metaObjectEv()};
     unsafe {_ZNK15QSessionManager10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn allowsErrorInteraction<T: QSessionManager_allowsErrorInteraction>(&mut self, value: T) -> i8 {
    return value.allowsErrorInteraction(self);
    // return 1;
  }
}

pub trait QSessionManager_allowsErrorInteraction {
  fn allowsErrorInteraction(self, rsthis: &mut QSessionManager) -> i8;
}

// proto:  bool QSessionManager::allowsErrorInteraction();
impl<'a> /*trait*/ QSessionManager_allowsErrorInteraction for () {
  fn allowsErrorInteraction(self, rsthis: &mut QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager22allowsErrorInteractionEv()};
    let mut ret = unsafe {_ZN15QSessionManager22allowsErrorInteractionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn FreeQSessionManager<T: QSessionManager_FreeQSessionManager>(&mut self, value: T)  {
     value.FreeQSessionManager(self);
    // return 1;
  }
}

pub trait QSessionManager_FreeQSessionManager {
  fn FreeQSessionManager(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::FreeQSessionManager();
impl<'a> /*trait*/ QSessionManager_FreeQSessionManager for () {
  fn FreeQSessionManager(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManagerD0Ev()};
     unsafe {_ZN15QSessionManagerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn restartCommand<T: QSessionManager_restartCommand>(&mut self, value: T) -> QStringList {
    return value.restartCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_restartCommand {
  fn restartCommand(self, rsthis: &mut QSessionManager) -> QStringList;
}

// proto:  QStringList QSessionManager::restartCommand();
impl<'a> /*trait*/ QSessionManager_restartCommand for () {
  fn restartCommand(self, rsthis: &mut QSessionManager) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14restartCommandEv()};
    let mut ret = unsafe {_ZNK15QSessionManager14restartCommandEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn requestPhase2<T: QSessionManager_requestPhase2>(&mut self, value: T)  {
     value.requestPhase2(self);
    // return 1;
  }
}

pub trait QSessionManager_requestPhase2 {
  fn requestPhase2(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::requestPhase2();
impl<'a> /*trait*/ QSessionManager_requestPhase2 for () {
  fn requestPhase2(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager13requestPhase2Ev()};
     unsafe {_ZN15QSessionManager13requestPhase2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn isPhase2<T: QSessionManager_isPhase2>(&mut self, value: T) -> i8 {
    return value.isPhase2(self);
    // return 1;
  }
}

pub trait QSessionManager_isPhase2 {
  fn isPhase2(self, rsthis: &mut QSessionManager) -> i8;
}

// proto:  bool QSessionManager::isPhase2();
impl<'a> /*trait*/ QSessionManager_isPhase2 for () {
  fn isPhase2(self, rsthis: &mut QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager8isPhase2Ev()};
    let mut ret = unsafe {_ZNK15QSessionManager8isPhase2Ev(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn release<T: QSessionManager_release>(&mut self, value: T)  {
     value.release(self);
    // return 1;
  }
}

pub trait QSessionManager_release {
  fn release(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::release();
impl<'a> /*trait*/ QSessionManager_release for () {
  fn release(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager7releaseEv()};
     unsafe {_ZN15QSessionManager7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setManagerProperty<T: QSessionManager_setManagerProperty>(&mut self, value: T)  {
     value.setManagerProperty(self);
    // return 1;
  }
}

pub trait QSessionManager_setManagerProperty {
  fn setManagerProperty(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::setManagerProperty(const QString & name, const QString & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty for (&'a  QString, &'a  QString) {
  fn setManagerProperty(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager18setManagerPropertyERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty for (&'a  QString, &'a  QStringList) {
  fn setManagerProperty(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn discardCommand<T: QSessionManager_discardCommand>(&mut self, value: T) -> QStringList {
    return value.discardCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_discardCommand {
  fn discardCommand(self, rsthis: &mut QSessionManager) -> QStringList;
}

// proto:  QStringList QSessionManager::discardCommand();
impl<'a> /*trait*/ QSessionManager_discardCommand for () {
  fn discardCommand(self, rsthis: &mut QSessionManager) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14discardCommandEv()};
    let mut ret = unsafe {_ZNK15QSessionManager14discardCommandEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn cancel<T: QSessionManager_cancel>(&mut self, value: T)  {
     value.cancel(self);
    // return 1;
  }
}

pub trait QSessionManager_cancel {
  fn cancel(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::cancel();
impl<'a> /*trait*/ QSessionManager_cancel for () {
  fn cancel(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager6cancelEv()};
     unsafe {_ZN15QSessionManager6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setDiscardCommand<T: QSessionManager_setDiscardCommand>(&mut self, value: T)  {
     value.setDiscardCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_setDiscardCommand {
  fn setDiscardCommand(self, rsthis: &mut QSessionManager) ;
}

// proto:  void QSessionManager::setDiscardCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setDiscardCommand for (&'a  QStringList) {
  fn setDiscardCommand(self, rsthis: &mut QSessionManager)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setDiscardCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager17setDiscardCommandERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn allowsInteraction<T: QSessionManager_allowsInteraction>(&mut self, value: T) -> i8 {
    return value.allowsInteraction(self);
    // return 1;
  }
}

pub trait QSessionManager_allowsInteraction {
  fn allowsInteraction(self, rsthis: &mut QSessionManager) -> i8;
}

// proto:  bool QSessionManager::allowsInteraction();
impl<'a> /*trait*/ QSessionManager_allowsInteraction for () {
  fn allowsInteraction(self, rsthis: &mut QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17allowsInteractionEv()};
    let mut ret = unsafe {_ZN15QSessionManager17allowsInteractionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

