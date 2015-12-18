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
  fn _ZNK15QSessionManager14restartCommandEv(qthis: *mut c_void) ;
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
  fn _ZNK15QSessionManager14discardCommandEv(qthis: *mut c_void) ;
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
  pub fn sessionId<RetType, T: QSessionManager_sessionId<RetType>>(&mut self, value: T) -> RetType {
    return value.sessionId(self);
    // return 1;
  }
}

pub trait QSessionManager_sessionId<RetType> {
  fn sessionId(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  QString QSessionManager::sessionId();
impl<'a> /*trait*/ QSessionManager_sessionId<QString> for () {
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
  pub fn sessionKey<RetType, T: QSessionManager_sessionKey<RetType>>(&mut self, value: T) -> RetType {
    return value.sessionKey(self);
    // return 1;
  }
}

pub trait QSessionManager_sessionKey<RetType> {
  fn sessionKey(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  QString QSessionManager::sessionKey();
impl<'a> /*trait*/ QSessionManager_sessionKey<QString> for () {
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
  pub fn setRestartCommand<RetType, T: QSessionManager_setRestartCommand<RetType>>(&mut self, value: T) -> RetType {
    return value.setRestartCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_setRestartCommand<RetType> {
  fn setRestartCommand(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::setRestartCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setRestartCommand<()> for (&'a  QStringList) {
  fn setRestartCommand(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setRestartCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager17setRestartCommandERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn metaObject<RetType, T: QSessionManager_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSessionManager_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  const QMetaObject * QSessionManager::metaObject();
impl<'a> /*trait*/ QSessionManager_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10metaObjectEv()};
     unsafe {_ZNK15QSessionManager10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn allowsErrorInteraction<RetType, T: QSessionManager_allowsErrorInteraction<RetType>>(&mut self, value: T) -> RetType {
    return value.allowsErrorInteraction(self);
    // return 1;
  }
}

pub trait QSessionManager_allowsErrorInteraction<RetType> {
  fn allowsErrorInteraction(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  bool QSessionManager::allowsErrorInteraction();
impl<'a> /*trait*/ QSessionManager_allowsErrorInteraction<i8> for () {
  fn allowsErrorInteraction(self, rsthis: &mut QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager22allowsErrorInteractionEv()};
    let mut ret = unsafe {_ZN15QSessionManager22allowsErrorInteractionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn FreeQSessionManager<RetType, T: QSessionManager_FreeQSessionManager<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSessionManager(self);
    // return 1;
  }
}

pub trait QSessionManager_FreeQSessionManager<RetType> {
  fn FreeQSessionManager(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::FreeQSessionManager();
impl<'a> /*trait*/ QSessionManager_FreeQSessionManager<()> for () {
  fn FreeQSessionManager(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManagerD0Ev()};
     unsafe {_ZN15QSessionManagerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn restartCommand<RetType, T: QSessionManager_restartCommand<RetType>>(&mut self, value: T) -> RetType {
    return value.restartCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_restartCommand<RetType> {
  fn restartCommand(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  QStringList QSessionManager::restartCommand();
impl<'a> /*trait*/ QSessionManager_restartCommand<()> for () {
  fn restartCommand(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14restartCommandEv()};
     unsafe {_ZNK15QSessionManager14restartCommandEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn requestPhase2<RetType, T: QSessionManager_requestPhase2<RetType>>(&mut self, value: T) -> RetType {
    return value.requestPhase2(self);
    // return 1;
  }
}

pub trait QSessionManager_requestPhase2<RetType> {
  fn requestPhase2(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::requestPhase2();
impl<'a> /*trait*/ QSessionManager_requestPhase2<()> for () {
  fn requestPhase2(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager13requestPhase2Ev()};
     unsafe {_ZN15QSessionManager13requestPhase2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn isPhase2<RetType, T: QSessionManager_isPhase2<RetType>>(&mut self, value: T) -> RetType {
    return value.isPhase2(self);
    // return 1;
  }
}

pub trait QSessionManager_isPhase2<RetType> {
  fn isPhase2(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  bool QSessionManager::isPhase2();
impl<'a> /*trait*/ QSessionManager_isPhase2<i8> for () {
  fn isPhase2(self, rsthis: &mut QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager8isPhase2Ev()};
    let mut ret = unsafe {_ZNK15QSessionManager8isPhase2Ev(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn release<RetType, T: QSessionManager_release<RetType>>(&mut self, value: T) -> RetType {
    return value.release(self);
    // return 1;
  }
}

pub trait QSessionManager_release<RetType> {
  fn release(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::release();
impl<'a> /*trait*/ QSessionManager_release<()> for () {
  fn release(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager7releaseEv()};
     unsafe {_ZN15QSessionManager7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setManagerProperty<RetType, T: QSessionManager_setManagerProperty<RetType>>(&mut self, value: T) -> RetType {
    return value.setManagerProperty(self);
    // return 1;
  }
}

pub trait QSessionManager_setManagerProperty<RetType> {
  fn setManagerProperty(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::setManagerProperty(const QString & name, const QString & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty<()> for (&'a  QString, &'a  QString) {
  fn setManagerProperty(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager18setManagerPropertyERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty<()> for (&'a  QString, &'a  QStringList) {
  fn setManagerProperty(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn discardCommand<RetType, T: QSessionManager_discardCommand<RetType>>(&mut self, value: T) -> RetType {
    return value.discardCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_discardCommand<RetType> {
  fn discardCommand(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  QStringList QSessionManager::discardCommand();
impl<'a> /*trait*/ QSessionManager_discardCommand<()> for () {
  fn discardCommand(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14discardCommandEv()};
     unsafe {_ZNK15QSessionManager14discardCommandEv(rsthis.qclsinst)};
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
  pub fn cancel<RetType, T: QSessionManager_cancel<RetType>>(&mut self, value: T) -> RetType {
    return value.cancel(self);
    // return 1;
  }
}

pub trait QSessionManager_cancel<RetType> {
  fn cancel(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::cancel();
impl<'a> /*trait*/ QSessionManager_cancel<()> for () {
  fn cancel(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager6cancelEv()};
     unsafe {_ZN15QSessionManager6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn setDiscardCommand<RetType, T: QSessionManager_setDiscardCommand<RetType>>(&mut self, value: T) -> RetType {
    return value.setDiscardCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_setDiscardCommand<RetType> {
  fn setDiscardCommand(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  void QSessionManager::setDiscardCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setDiscardCommand<()> for (&'a  QStringList) {
  fn setDiscardCommand(self, rsthis: &mut QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setDiscardCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSessionManager17setDiscardCommandERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSessionManager {
  pub fn allowsInteraction<RetType, T: QSessionManager_allowsInteraction<RetType>>(&mut self, value: T) -> RetType {
    return value.allowsInteraction(self);
    // return 1;
  }
}

pub trait QSessionManager_allowsInteraction<RetType> {
  fn allowsInteraction(self, rsthis: &mut QSessionManager) -> RetType;
}

// proto:  bool QSessionManager::allowsInteraction();
impl<'a> /*trait*/ QSessionManager_allowsInteraction<i8> for () {
  fn allowsInteraction(self, rsthis: &mut QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17allowsInteractionEv()};
    let mut ret = unsafe {_ZN15QSessionManager17allowsInteractionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

