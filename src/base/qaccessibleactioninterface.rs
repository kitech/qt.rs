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

// proto: static QString QAccessibleActionInterface::scrollUpAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollUpAction_s<RetType, T: QAccessibleActionInterface_scrollUpAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollUpAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollUpAction_s<RetType> {
  fn scrollUpAction_s(self ) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollUpAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollUpAction_s<QString> for () {
  fn scrollUpAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QStringList QAccessibleActionInterface::actionNames();
impl /*struct*/ QAccessibleActionInterface {
  pub fn actionNames<RetType, T: QAccessibleActionInterface_actionNames<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.actionNames(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_actionNames<RetType> {
  fn actionNames(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QStringList QAccessibleActionInterface::actionNames();
impl<'a> /*trait*/ QAccessibleActionInterface_actionNames<()> for () {
  fn actionNames(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface11actionNamesEv()};
     unsafe {_ZNK26QAccessibleActionInterface11actionNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static const QString & QAccessibleActionInterface::decreaseAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn decreaseAction_s<RetType, T: QAccessibleActionInterface_decreaseAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.decreaseAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_decreaseAction_s<RetType> {
  fn decreaseAction_s(self ) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::decreaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_decreaseAction_s<QString> for () {
  fn decreaseAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14decreaseActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14decreaseActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static const QString & QAccessibleActionInterface::toggleAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn toggleAction_s<RetType, T: QAccessibleActionInterface_toggleAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toggleAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_toggleAction_s<RetType> {
  fn toggleAction_s(self ) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::toggleAction();
impl<'a> /*trait*/ QAccessibleActionInterface_toggleAction_s<QString> for () {
  fn toggleAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface12toggleActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface12toggleActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionName<RetType, T: QAccessibleActionInterface_localizedActionName<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.localizedActionName(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionName<RetType> {
  fn localizedActionName(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionName<QString> for (&'a  QString) {
  fn localizedActionName(self , rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionDescription<RetType, T: QAccessibleActionInterface_localizedActionDescription<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.localizedActionDescription(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionDescription<RetType> {
  fn localizedActionDescription(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionDescription<QString> for (&'a  QString) {
  fn localizedActionDescription(self , rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QAccessibleActionInterface::scrollLeftAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollLeftAction_s<RetType, T: QAccessibleActionInterface_scrollLeftAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollLeftAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollLeftAction_s<RetType> {
  fn scrollLeftAction_s(self ) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollLeftAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollLeftAction_s<QString> for () {
  fn scrollLeftAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QAccessibleActionInterface::previousPageAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn previousPageAction_s<RetType, T: QAccessibleActionInterface_previousPageAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.previousPageAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_previousPageAction_s<RetType> {
  fn previousPageAction_s(self ) -> RetType;
}

// proto: static QString QAccessibleActionInterface::previousPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_previousPageAction_s<QString> for () {
  fn previousPageAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface18previousPageActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface18previousPageActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static const QString & QAccessibleActionInterface::showMenuAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn showMenuAction_s<RetType, T: QAccessibleActionInterface_showMenuAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.showMenuAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_showMenuAction_s<RetType> {
  fn showMenuAction_s(self ) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::showMenuAction();
impl<'a> /*trait*/ QAccessibleActionInterface_showMenuAction_s<QString> for () {
  fn showMenuAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14showMenuActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14showMenuActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QAccessibleActionInterface::scrollRightAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollRightAction_s<RetType, T: QAccessibleActionInterface_scrollRightAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollRightAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollRightAction_s<RetType> {
  fn scrollRightAction_s(self ) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollRightAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollRightAction_s<QString> for () {
  fn scrollRightAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static const QString & QAccessibleActionInterface::setFocusAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn setFocusAction_s<RetType, T: QAccessibleActionInterface_setFocusAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFocusAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_setFocusAction_s<RetType> {
  fn setFocusAction_s(self ) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::setFocusAction();
impl<'a> /*trait*/ QAccessibleActionInterface_setFocusAction_s<QString> for () {
  fn setFocusAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14setFocusActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14setFocusActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QAccessibleActionInterface::nextPageAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn nextPageAction_s<RetType, T: QAccessibleActionInterface_nextPageAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.nextPageAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_nextPageAction_s<RetType> {
  fn nextPageAction_s(self ) -> RetType;
}

// proto: static QString QAccessibleActionInterface::nextPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_nextPageAction_s<QString> for () {
  fn nextPageAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14nextPageActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14nextPageActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QAccessibleActionInterface::FreeQAccessibleActionInterface();
impl /*struct*/ QAccessibleActionInterface {
  pub fn FreeQAccessibleActionInterface<RetType, T: QAccessibleActionInterface_FreeQAccessibleActionInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleActionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_FreeQAccessibleActionInterface<RetType> {
  fn FreeQAccessibleActionInterface(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  void QAccessibleActionInterface::FreeQAccessibleActionInterface();
impl<'a> /*trait*/ QAccessibleActionInterface_FreeQAccessibleActionInterface<()> for () {
  fn FreeQAccessibleActionInterface(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterfaceD0Ev()};
     unsafe {_ZN26QAccessibleActionInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static const QString & QAccessibleActionInterface::pressAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn pressAction_s<RetType, T: QAccessibleActionInterface_pressAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pressAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_pressAction_s<RetType> {
  fn pressAction_s(self ) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::pressAction();
impl<'a> /*trait*/ QAccessibleActionInterface_pressAction_s<QString> for () {
  fn pressAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface11pressActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface11pressActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
impl /*struct*/ QAccessibleActionInterface {
  pub fn doAction<RetType, T: QAccessibleActionInterface_doAction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.doAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_doAction<RetType> {
  fn doAction(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_doAction<()> for (&'a  QString) {
  fn doAction(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface8doActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QAccessibleActionInterface8doActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static const QString & QAccessibleActionInterface::increaseAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn increaseAction_s<RetType, T: QAccessibleActionInterface_increaseAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.increaseAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_increaseAction_s<RetType> {
  fn increaseAction_s(self ) -> RetType;
}

// proto: static const QString & QAccessibleActionInterface::increaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_increaseAction_s<QString> for () {
  fn increaseAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14increaseActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14increaseActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl /*struct*/ QAccessibleActionInterface {
  pub fn keyBindingsForAction<RetType, T: QAccessibleActionInterface_keyBindingsForAction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.keyBindingsForAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_keyBindingsForAction<RetType> {
  fn keyBindingsForAction(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

// proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_keyBindingsForAction<()> for (&'a  QString) {
  fn keyBindingsForAction(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QString QAccessibleActionInterface::scrollDownAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollDownAction_s<RetType, T: QAccessibleActionInterface_scrollDownAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollDownAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollDownAction_s<RetType> {
  fn scrollDownAction_s(self ) -> RetType;
}

// proto: static QString QAccessibleActionInterface::scrollDownAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollDownAction_s<QString> for () {
  fn scrollDownAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

