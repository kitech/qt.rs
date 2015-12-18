// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QString QAccessibleActionInterface::scrollUpAction();
  fn _ZN26QAccessibleActionInterface14scrollUpActionEv() -> *mut c_void;
  // proto:  QStringList QAccessibleActionInterface::actionNames();
  fn _ZNK26QAccessibleActionInterface11actionNamesEv(qthis: *mut c_void) ;
  // proto: static const QString & QAccessibleActionInterface::decreaseAction();
  fn _ZN26QAccessibleActionInterface14decreaseActionEv() -> *mut c_void;
  // proto: static const QString & QAccessibleActionInterface::toggleAction();
  fn _ZN26QAccessibleActionInterface12toggleActionEv() -> *mut c_void;
  // proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
  fn _ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
  fn _ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::scrollLeftAction();
  fn _ZN26QAccessibleActionInterface16scrollLeftActionEv() -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::previousPageAction();
  fn _ZN26QAccessibleActionInterface18previousPageActionEv() -> *mut c_void;
  // proto: static const QString & QAccessibleActionInterface::showMenuAction();
  fn _ZN26QAccessibleActionInterface14showMenuActionEv() -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::scrollRightAction();
  fn _ZN26QAccessibleActionInterface17scrollRightActionEv() -> *mut c_void;
  // proto: static const QString & QAccessibleActionInterface::setFocusAction();
  fn _ZN26QAccessibleActionInterface14setFocusActionEv() -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::nextPageAction();
  fn _ZN26QAccessibleActionInterface14nextPageActionEv() -> *mut c_void;
  // proto:  void QAccessibleActionInterface::FreeQAccessibleActionInterface();
  fn _ZN26QAccessibleActionInterfaceD0Ev(qthis: *mut c_void) ;
  // proto: static const QString & QAccessibleActionInterface::pressAction();
  fn _ZN26QAccessibleActionInterface11pressActionEv() -> *mut c_void;
  // proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
  fn _ZN26QAccessibleActionInterface8doActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static const QString & QAccessibleActionInterface::increaseAction();
  fn _ZN26QAccessibleActionInterface14increaseActionEv() -> *mut c_void;
  // proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
  fn _ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QString QAccessibleActionInterface::scrollDownAction();
  fn _ZN26QAccessibleActionInterface16scrollDownActionEv() -> *mut c_void;
}

// body block begin
// class sizeof(QAccessibleActionInterface)=8
pub struct QAccessibleActionInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollUpAction<RetType, T: QAccessibleActionInterface_scrollUpAction<RetType>>(&mut self, value: T) -> RetType {
    return value.scrollUpAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollUpAction<RetType> {
  fn scrollUpAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollUpAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollUpAction<QString> for () {
  fn scrollUpAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn actionNames<RetType, T: QAccessibleActionInterface_actionNames<RetType>>(&mut self, value: T) -> RetType {
    return value.actionNames(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_actionNames<RetType> {
  fn actionNames(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QStringList QAccessibleActionInterface::actionNames();
impl<'a> /*trait*/ QAccessibleActionInterface_actionNames<()> for () {
  fn actionNames(self, rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface11actionNamesEv()};
     unsafe {_ZNK26QAccessibleActionInterface11actionNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn decreaseAction<RetType, T: QAccessibleActionInterface_decreaseAction<RetType>>(&mut self, value: T) -> RetType {
    return value.decreaseAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_decreaseAction<RetType> {
  fn decreaseAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::decreaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_decreaseAction<QString> for () {
  fn decreaseAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14decreaseActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14decreaseActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn toggleAction<RetType, T: QAccessibleActionInterface_toggleAction<RetType>>(&mut self, value: T) -> RetType {
    return value.toggleAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_toggleAction<RetType> {
  fn toggleAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::toggleAction();
impl<'a> /*trait*/ QAccessibleActionInterface_toggleAction<QString> for () {
  fn toggleAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface12toggleActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface12toggleActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionName<RetType, T: QAccessibleActionInterface_localizedActionName<RetType>>(&mut self, value: T) -> RetType {
    return value.localizedActionName(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionName<RetType> {
  fn localizedActionName(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionName<QString> for (&'a  QString) {
  fn localizedActionName(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionDescription<RetType, T: QAccessibleActionInterface_localizedActionDescription<RetType>>(&mut self, value: T) -> RetType {
    return value.localizedActionDescription(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionDescription<RetType> {
  fn localizedActionDescription(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionDescription<QString> for (&'a  QString) {
  fn localizedActionDescription(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollLeftAction<RetType, T: QAccessibleActionInterface_scrollLeftAction<RetType>>(&mut self, value: T) -> RetType {
    return value.scrollLeftAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollLeftAction<RetType> {
  fn scrollLeftAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollLeftAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollLeftAction<QString> for () {
  fn scrollLeftAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn previousPageAction<RetType, T: QAccessibleActionInterface_previousPageAction<RetType>>(&mut self, value: T) -> RetType {
    return value.previousPageAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_previousPageAction<RetType> {
  fn previousPageAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static QString QAccessibleActionInterface::previousPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_previousPageAction<QString> for () {
  fn previousPageAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface18previousPageActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface18previousPageActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn showMenuAction<RetType, T: QAccessibleActionInterface_showMenuAction<RetType>>(&mut self, value: T) -> RetType {
    return value.showMenuAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_showMenuAction<RetType> {
  fn showMenuAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::showMenuAction();
impl<'a> /*trait*/ QAccessibleActionInterface_showMenuAction<QString> for () {
  fn showMenuAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14showMenuActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14showMenuActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollRightAction<RetType, T: QAccessibleActionInterface_scrollRightAction<RetType>>(&mut self, value: T) -> RetType {
    return value.scrollRightAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollRightAction<RetType> {
  fn scrollRightAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollRightAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollRightAction<QString> for () {
  fn scrollRightAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn setFocusAction<RetType, T: QAccessibleActionInterface_setFocusAction<RetType>>(&mut self, value: T) -> RetType {
    return value.setFocusAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_setFocusAction<RetType> {
  fn setFocusAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::setFocusAction();
impl<'a> /*trait*/ QAccessibleActionInterface_setFocusAction<QString> for () {
  fn setFocusAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14setFocusActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14setFocusActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn nextPageAction<RetType, T: QAccessibleActionInterface_nextPageAction<RetType>>(&mut self, value: T) -> RetType {
    return value.nextPageAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_nextPageAction<RetType> {
  fn nextPageAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static QString QAccessibleActionInterface::nextPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_nextPageAction<QString> for () {
  fn nextPageAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14nextPageActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14nextPageActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn FreeQAccessibleActionInterface<RetType, T: QAccessibleActionInterface_FreeQAccessibleActionInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQAccessibleActionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_FreeQAccessibleActionInterface<RetType> {
  fn FreeQAccessibleActionInterface(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  void QAccessibleActionInterface::FreeQAccessibleActionInterface();
impl<'a> /*trait*/ QAccessibleActionInterface_FreeQAccessibleActionInterface<()> for () {
  fn FreeQAccessibleActionInterface(self, rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterfaceD0Ev()};
     unsafe {_ZN26QAccessibleActionInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn pressAction<RetType, T: QAccessibleActionInterface_pressAction<RetType>>(&mut self, value: T) -> RetType {
    return value.pressAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_pressAction<RetType> {
  fn pressAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::pressAction();
impl<'a> /*trait*/ QAccessibleActionInterface_pressAction<QString> for () {
  fn pressAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface11pressActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface11pressActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn doAction<RetType, T: QAccessibleActionInterface_doAction<RetType>>(&mut self, value: T) -> RetType {
    return value.doAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_doAction<RetType> {
  fn doAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_doAction<()> for (&'a  QString) {
  fn doAction(self, rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface8doActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QAccessibleActionInterface8doActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn increaseAction<RetType, T: QAccessibleActionInterface_increaseAction<RetType>>(&mut self, value: T) -> RetType {
    return value.increaseAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_increaseAction<RetType> {
  fn increaseAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::increaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_increaseAction<QString> for () {
  fn increaseAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14increaseActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14increaseActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn keyBindingsForAction<RetType, T: QAccessibleActionInterface_keyBindingsForAction<RetType>>(&mut self, value: T) -> RetType {
    return value.keyBindingsForAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_keyBindingsForAction<RetType> {
  fn keyBindingsForAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_keyBindingsForAction<()> for (&'a  QString) {
  fn keyBindingsForAction(self, rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollDownAction<RetType, T: QAccessibleActionInterface_scrollDownAction<RetType>>(&mut self, value: T) -> RetType {
    return value.scrollDownAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollDownAction<RetType> {
  fn scrollDownAction(self, rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollDownAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollDownAction<QString> for () {
  fn scrollDownAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

