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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QString QAccessibleActionInterface::scrollUpAction();
  fn _ZN26QAccessibleActionInterface14scrollUpActionEv() -> *mut c_void;
  // proto:  QStringList QAccessibleActionInterface::actionNames();
  fn _ZNK26QAccessibleActionInterface11actionNamesEv(qthis: *mut c_void) -> *mut c_void;
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
  fn _ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::scrollDownAction();
  fn _ZN26QAccessibleActionInterface16scrollDownActionEv() -> *mut c_void;
}

// body block begin
// class sizeof(QAccessibleActionInterface)=8
pub struct QAccessibleActionInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollUpAction<T: QAccessibleActionInterface_scrollUpAction>(&mut self, value: T) -> QString {
    return value.scrollUpAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollUpAction {
  fn scrollUpAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static QString QAccessibleActionInterface::scrollUpAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollUpAction for () {
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
  pub fn actionNames<T: QAccessibleActionInterface_actionNames>(&mut self, value: T) -> QStringList {
    return value.actionNames(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_actionNames {
  fn actionNames(self, rsthis: &mut QAccessibleActionInterface) -> QStringList;
}

// proto:  QStringList QAccessibleActionInterface::actionNames();
impl<'a> /*trait*/ QAccessibleActionInterface_actionNames for () {
  fn actionNames(self, rsthis: &mut QAccessibleActionInterface) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface11actionNamesEv()};
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface11actionNamesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn decreaseAction<T: QAccessibleActionInterface_decreaseAction>(&mut self, value: T) -> QString {
    return value.decreaseAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_decreaseAction {
  fn decreaseAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static const QString & QAccessibleActionInterface::decreaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_decreaseAction for () {
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
  pub fn toggleAction<T: QAccessibleActionInterface_toggleAction>(&mut self, value: T) -> QString {
    return value.toggleAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_toggleAction {
  fn toggleAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static const QString & QAccessibleActionInterface::toggleAction();
impl<'a> /*trait*/ QAccessibleActionInterface_toggleAction for () {
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
  pub fn localizedActionName<T: QAccessibleActionInterface_localizedActionName>(&mut self, value: T) -> QString {
    return value.localizedActionName(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionName {
  fn localizedActionName(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionName for (&'a  QString) {
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
  pub fn localizedActionDescription<T: QAccessibleActionInterface_localizedActionDescription>(&mut self, value: T) -> QString {
    return value.localizedActionDescription(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionDescription {
  fn localizedActionDescription(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionDescription for (&'a  QString) {
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
  pub fn scrollLeftAction<T: QAccessibleActionInterface_scrollLeftAction>(&mut self, value: T) -> QString {
    return value.scrollLeftAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollLeftAction {
  fn scrollLeftAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static QString QAccessibleActionInterface::scrollLeftAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollLeftAction for () {
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
  pub fn previousPageAction<T: QAccessibleActionInterface_previousPageAction>(&mut self, value: T) -> QString {
    return value.previousPageAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_previousPageAction {
  fn previousPageAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static QString QAccessibleActionInterface::previousPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_previousPageAction for () {
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
  pub fn showMenuAction<T: QAccessibleActionInterface_showMenuAction>(&mut self, value: T) -> QString {
    return value.showMenuAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_showMenuAction {
  fn showMenuAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static const QString & QAccessibleActionInterface::showMenuAction();
impl<'a> /*trait*/ QAccessibleActionInterface_showMenuAction for () {
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
  pub fn scrollRightAction<T: QAccessibleActionInterface_scrollRightAction>(&mut self, value: T) -> QString {
    return value.scrollRightAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollRightAction {
  fn scrollRightAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static QString QAccessibleActionInterface::scrollRightAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollRightAction for () {
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
  pub fn setFocusAction<T: QAccessibleActionInterface_setFocusAction>(&mut self, value: T) -> QString {
    return value.setFocusAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_setFocusAction {
  fn setFocusAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static const QString & QAccessibleActionInterface::setFocusAction();
impl<'a> /*trait*/ QAccessibleActionInterface_setFocusAction for () {
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
  pub fn nextPageAction<T: QAccessibleActionInterface_nextPageAction>(&mut self, value: T) -> QString {
    return value.nextPageAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_nextPageAction {
  fn nextPageAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static QString QAccessibleActionInterface::nextPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_nextPageAction for () {
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
  pub fn FreeQAccessibleActionInterface<T: QAccessibleActionInterface_FreeQAccessibleActionInterface>(&mut self, value: T)  {
     value.FreeQAccessibleActionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_FreeQAccessibleActionInterface {
  fn FreeQAccessibleActionInterface(self, rsthis: &mut QAccessibleActionInterface) ;
}

// proto:  void QAccessibleActionInterface::FreeQAccessibleActionInterface();
impl<'a> /*trait*/ QAccessibleActionInterface_FreeQAccessibleActionInterface for () {
  fn FreeQAccessibleActionInterface(self, rsthis: &mut QAccessibleActionInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterfaceD0Ev()};
     unsafe {_ZN26QAccessibleActionInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn pressAction<T: QAccessibleActionInterface_pressAction>(&mut self, value: T) -> QString {
    return value.pressAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_pressAction {
  fn pressAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static const QString & QAccessibleActionInterface::pressAction();
impl<'a> /*trait*/ QAccessibleActionInterface_pressAction for () {
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
  pub fn doAction<T: QAccessibleActionInterface_doAction>(&mut self, value: T)  {
     value.doAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_doAction {
  fn doAction(self, rsthis: &mut QAccessibleActionInterface) ;
}

// proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_doAction for (&'a  QString) {
  fn doAction(self, rsthis: &mut QAccessibleActionInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface8doActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QAccessibleActionInterface8doActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn increaseAction<T: QAccessibleActionInterface_increaseAction>(&mut self, value: T) -> QString {
    return value.increaseAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_increaseAction {
  fn increaseAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static const QString & QAccessibleActionInterface::increaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_increaseAction for () {
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
  pub fn keyBindingsForAction<T: QAccessibleActionInterface_keyBindingsForAction>(&mut self, value: T) -> QStringList {
    return value.keyBindingsForAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_keyBindingsForAction {
  fn keyBindingsForAction(self, rsthis: &mut QAccessibleActionInterface) -> QStringList;
}

// proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_keyBindingsForAction for (&'a  QString) {
  fn keyBindingsForAction(self, rsthis: &mut QAccessibleActionInterface) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollDownAction<T: QAccessibleActionInterface_scrollDownAction>(&mut self, value: T) -> QString {
    return value.scrollDownAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollDownAction {
  fn scrollDownAction(self, rsthis: &mut QAccessibleActionInterface) -> QString;
}

// proto: static QString QAccessibleActionInterface::scrollDownAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollDownAction for () {
  fn scrollDownAction(self, rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

