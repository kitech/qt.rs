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
  // proto: QString QAccessibleActionInterface::scrollUpAction();
  fn _ZN26QAccessibleActionInterface14scrollUpActionEv() -> i32;
  // proto: QStringList QAccessibleActionInterface::actionNames();
  fn _ZNK26QAccessibleActionInterface11actionNamesEv() -> i32;
  // proto: const QString & QAccessibleActionInterface::decreaseAction();
  fn _ZN26QAccessibleActionInterface14decreaseActionEv() -> i32;
  // proto: const QString & QAccessibleActionInterface::toggleAction();
  fn _ZN26QAccessibleActionInterface12toggleActionEv() -> i32;
  // proto: QString QAccessibleActionInterface::localizedActionName(const QString & name);
  fn _ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
  fn _ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QAccessibleActionInterface::scrollLeftAction();
  fn _ZN26QAccessibleActionInterface16scrollLeftActionEv() -> i32;
  // proto: QString QAccessibleActionInterface::previousPageAction();
  fn _ZN26QAccessibleActionInterface18previousPageActionEv() -> i32;
  // proto: const QString & QAccessibleActionInterface::showMenuAction();
  fn _ZN26QAccessibleActionInterface14showMenuActionEv() -> i32;
  // proto: QString QAccessibleActionInterface::scrollRightAction();
  fn _ZN26QAccessibleActionInterface17scrollRightActionEv() -> i32;
  // proto: const QString & QAccessibleActionInterface::setFocusAction();
  fn _ZN26QAccessibleActionInterface14setFocusActionEv() -> i32;
  // proto: QString QAccessibleActionInterface::nextPageAction();
  fn _ZN26QAccessibleActionInterface14nextPageActionEv() -> i32;
  // proto: void QAccessibleActionInterface::FreeQAccessibleActionInterface();
  fn _ZN26QAccessibleActionInterfaceD0Ev() -> i32;
  // proto: const QString & QAccessibleActionInterface::pressAction();
  fn _ZN26QAccessibleActionInterface11pressActionEv() -> i32;
  // proto: void QAccessibleActionInterface::doAction(const QString & actionName);
  fn _ZN26QAccessibleActionInterface8doActionERK7QString(arg0: *const c_void) -> i32;
  // proto: const QString & QAccessibleActionInterface::increaseAction();
  fn _ZN26QAccessibleActionInterface14increaseActionEv() -> i32;
  // proto: QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
  fn _ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QAccessibleActionInterface::scrollDownAction();
  fn _ZN26QAccessibleActionInterface16scrollDownActionEv() -> i32;
}

// body block begin
// class sizeof(QAccessibleActionInterface)=8
pub struct QAccessibleActionInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollUpAction<T: QAccessibleActionInterface_scrollUpAction>(&mut self, value: T) -> i32 {
    value.scrollUpAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_scrollUpAction {
  fn scrollUpAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::scrollUpAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollUpAction for () {
  fn scrollUpAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    unsafe {_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn actionNames<T: QAccessibleActionInterface_actionNames>(&mut self, value: T) -> i32 {
    value.actionNames(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_actionNames {
  fn actionNames(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QStringList QAccessibleActionInterface::actionNames();
impl<'a> /*trait*/ QAccessibleActionInterface_actionNames for () {
  fn actionNames(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface11actionNamesEv()};
    unsafe {_ZNK26QAccessibleActionInterface11actionNamesEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn decreaseAction<T: QAccessibleActionInterface_decreaseAction>(&mut self, value: T) -> i32 {
    value.decreaseAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_decreaseAction {
  fn decreaseAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: const QString & QAccessibleActionInterface::decreaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_decreaseAction for () {
  fn decreaseAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14decreaseActionEv()};
    unsafe {_ZN26QAccessibleActionInterface14decreaseActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn toggleAction<T: QAccessibleActionInterface_toggleAction>(&mut self, value: T) -> i32 {
    value.toggleAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_toggleAction {
  fn toggleAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: const QString & QAccessibleActionInterface::toggleAction();
impl<'a> /*trait*/ QAccessibleActionInterface_toggleAction for () {
  fn toggleAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface12toggleActionEv()};
    unsafe {_ZN26QAccessibleActionInterface12toggleActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionName<T: QAccessibleActionInterface_localizedActionName>(&mut self, value: T) -> i32 {
    value.localizedActionName(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionName {
  fn localizedActionName(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionName for (&'a  QString) {
  fn localizedActionName(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionDescription<T: QAccessibleActionInterface_localizedActionDescription>(&mut self, value: T) -> i32 {
    value.localizedActionDescription(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionDescription {
  fn localizedActionDescription(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionDescription for (&'a  QString) {
  fn localizedActionDescription(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollLeftAction<T: QAccessibleActionInterface_scrollLeftAction>(&mut self, value: T) -> i32 {
    value.scrollLeftAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_scrollLeftAction {
  fn scrollLeftAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::scrollLeftAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollLeftAction for () {
  fn scrollLeftAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    unsafe {_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn previousPageAction<T: QAccessibleActionInterface_previousPageAction>(&mut self, value: T) -> i32 {
    value.previousPageAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_previousPageAction {
  fn previousPageAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::previousPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_previousPageAction for () {
  fn previousPageAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface18previousPageActionEv()};
    unsafe {_ZN26QAccessibleActionInterface18previousPageActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn showMenuAction<T: QAccessibleActionInterface_showMenuAction>(&mut self, value: T) -> i32 {
    value.showMenuAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_showMenuAction {
  fn showMenuAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: const QString & QAccessibleActionInterface::showMenuAction();
impl<'a> /*trait*/ QAccessibleActionInterface_showMenuAction for () {
  fn showMenuAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14showMenuActionEv()};
    unsafe {_ZN26QAccessibleActionInterface14showMenuActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollRightAction<T: QAccessibleActionInterface_scrollRightAction>(&mut self, value: T) -> i32 {
    value.scrollRightAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_scrollRightAction {
  fn scrollRightAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::scrollRightAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollRightAction for () {
  fn scrollRightAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    unsafe {_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn setFocusAction<T: QAccessibleActionInterface_setFocusAction>(&mut self, value: T) -> i32 {
    value.setFocusAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_setFocusAction {
  fn setFocusAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: const QString & QAccessibleActionInterface::setFocusAction();
impl<'a> /*trait*/ QAccessibleActionInterface_setFocusAction for () {
  fn setFocusAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14setFocusActionEv()};
    unsafe {_ZN26QAccessibleActionInterface14setFocusActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn nextPageAction<T: QAccessibleActionInterface_nextPageAction>(&mut self, value: T) -> i32 {
    value.nextPageAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_nextPageAction {
  fn nextPageAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::nextPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_nextPageAction for () {
  fn nextPageAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14nextPageActionEv()};
    unsafe {_ZN26QAccessibleActionInterface14nextPageActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn FreeQAccessibleActionInterface<T: QAccessibleActionInterface_FreeQAccessibleActionInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleActionInterface(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_FreeQAccessibleActionInterface {
  fn FreeQAccessibleActionInterface(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: void QAccessibleActionInterface::FreeQAccessibleActionInterface();
impl<'a> /*trait*/ QAccessibleActionInterface_FreeQAccessibleActionInterface for () {
  fn FreeQAccessibleActionInterface(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterfaceD0Ev()};
    unsafe {_ZN26QAccessibleActionInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn pressAction<T: QAccessibleActionInterface_pressAction>(&mut self, value: T) -> i32 {
    value.pressAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_pressAction {
  fn pressAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: const QString & QAccessibleActionInterface::pressAction();
impl<'a> /*trait*/ QAccessibleActionInterface_pressAction for () {
  fn pressAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface11pressActionEv()};
    unsafe {_ZN26QAccessibleActionInterface11pressActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn doAction<T: QAccessibleActionInterface_doAction>(&mut self, value: T) -> i32 {
    value.doAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_doAction {
  fn doAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: void QAccessibleActionInterface::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_doAction for (&'a  QString) {
  fn doAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface8doActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleActionInterface8doActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn increaseAction<T: QAccessibleActionInterface_increaseAction>(&mut self, value: T) -> i32 {
    value.increaseAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_increaseAction {
  fn increaseAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: const QString & QAccessibleActionInterface::increaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_increaseAction for () {
  fn increaseAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14increaseActionEv()};
    unsafe {_ZN26QAccessibleActionInterface14increaseActionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn keyBindingsForAction<T: QAccessibleActionInterface_keyBindingsForAction>(&mut self, value: T) -> i32 {
    value.keyBindingsForAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_keyBindingsForAction {
  fn keyBindingsForAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_keyBindingsForAction for (&'a  QString) {
  fn keyBindingsForAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollDownAction<T: QAccessibleActionInterface_scrollDownAction>(&mut self, value: T) -> i32 {
    value.scrollDownAction(self);
    return 1;
  }
}

pub trait QAccessibleActionInterface_scrollDownAction {
  fn scrollDownAction(self, this: &mut QAccessibleActionInterface) -> i32;
}

// proto: QString QAccessibleActionInterface::scrollDownAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollDownAction for () {
  fn scrollDownAction(self, this: &mut QAccessibleActionInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    unsafe {_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    return 1;
  }
}

