// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QAccessibleWidget::childCount();
  fn _ZNK17QAccessibleWidget10childCountEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleWidget::child(int index);
  fn _ZNK17QAccessibleWidget5childEi(arg0: c_int) -> i32;
  // proto: QWindow * QAccessibleWidget::window();
  fn _ZNK17QAccessibleWidget6windowEv() -> i32;
  // proto: QRect QAccessibleWidget::rect();
  fn _ZNK17QAccessibleWidget4rectEv() -> i32;
  // proto: QColor QAccessibleWidget::foregroundColor();
  fn _ZNK17QAccessibleWidget15foregroundColorEv() -> i32;
  // proto: bool QAccessibleWidget::isValid();
  fn _ZNK17QAccessibleWidget7isValidEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleWidget::focusChild();
  fn _ZNK17QAccessibleWidget10focusChildEv() -> i32;
  // proto: void QAccessibleWidget::NewQAccessibleWidget(const QAccessibleWidget & );
  fn _ZN17QAccessibleWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QColor QAccessibleWidget::backgroundColor();
  fn _ZNK17QAccessibleWidget15backgroundColorEv() -> i32;
  // proto: void QAccessibleWidget::FreeQAccessibleWidget();
  fn _ZN17QAccessibleWidgetD0Ev() -> i32;
  // proto: QStringList QAccessibleWidget::actionNames();
  fn _ZNK17QAccessibleWidget11actionNamesEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleWidget::parent();
  fn _ZNK17QAccessibleWidget6parentEv() -> i32;
  // proto: void QAccessibleWidget::doAction(const QString & actionName);
  fn _ZN17QAccessibleWidget8doActionERK7QString(arg0: *const c_void) -> i32;
  // proto: QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
  fn _ZNK17QAccessibleWidget20keyBindingsForActionERK7QString(arg0: *const c_void) -> i32;
  // proto: int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
  fn _ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QAccessibleWidget)=32
pub struct QAccessibleWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleWidget {
  pub fn childCount<T: QAccessibleWidget_childCount>(&mut self, value: T) -> i32 {
    value.childCount(self);
    return 1;
  }
}

pub trait QAccessibleWidget_childCount {
  fn childCount(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: int QAccessibleWidget::childCount();
impl<'a> /*trait*/ QAccessibleWidget_childCount for () {
  fn childCount(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget10childCountEv()};
    unsafe {_ZNK17QAccessibleWidget10childCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn child<T: QAccessibleWidget_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QAccessibleWidget_child {
  fn child(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QAccessibleInterface * QAccessibleWidget::child(int index);
impl<'a> /*trait*/ QAccessibleWidget_child for (i32) {
  fn child(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget5childEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK17QAccessibleWidget5childEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn window<T: QAccessibleWidget_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QAccessibleWidget_window {
  fn window(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QWindow * QAccessibleWidget::window();
impl<'a> /*trait*/ QAccessibleWidget_window for () {
  fn window(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget6windowEv()};
    unsafe {_ZNK17QAccessibleWidget6windowEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn rect<T: QAccessibleWidget_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QAccessibleWidget_rect {
  fn rect(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QRect QAccessibleWidget::rect();
impl<'a> /*trait*/ QAccessibleWidget_rect for () {
  fn rect(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget4rectEv()};
    unsafe {_ZNK17QAccessibleWidget4rectEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn foregroundColor<T: QAccessibleWidget_foregroundColor>(&mut self, value: T) -> i32 {
    value.foregroundColor(self);
    return 1;
  }
}

pub trait QAccessibleWidget_foregroundColor {
  fn foregroundColor(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QColor QAccessibleWidget::foregroundColor();
impl<'a> /*trait*/ QAccessibleWidget_foregroundColor for () {
  fn foregroundColor(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget15foregroundColorEv()};
    unsafe {_ZNK17QAccessibleWidget15foregroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn isValid<T: QAccessibleWidget_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QAccessibleWidget_isValid {
  fn isValid(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: bool QAccessibleWidget::isValid();
impl<'a> /*trait*/ QAccessibleWidget_isValid for () {
  fn isValid(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget7isValidEv()};
    unsafe {_ZNK17QAccessibleWidget7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn focusChild<T: QAccessibleWidget_focusChild>(&mut self, value: T) -> i32 {
    value.focusChild(self);
    return 1;
  }
}

pub trait QAccessibleWidget_focusChild {
  fn focusChild(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QAccessibleInterface * QAccessibleWidget::focusChild();
impl<'a> /*trait*/ QAccessibleWidget_focusChild for () {
  fn focusChild(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget10focusChildEv()};
    unsafe {_ZNK17QAccessibleWidget10focusChildEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn NewQAccessibleWidget<T: QAccessibleWidget_NewQAccessibleWidget>(value: T) -> QAccessibleWidget {
    let rsthis = value.NewQAccessibleWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleWidget_NewQAccessibleWidget {
  fn NewQAccessibleWidget(self) -> QAccessibleWidget;
}

// proto: void QAccessibleWidget::NewQAccessibleWidget(const QAccessibleWidget & );
impl<'a> /*trait*/ QAccessibleWidget_NewQAccessibleWidget for (&'a  QAccessibleWidget) {
  fn NewQAccessibleWidget(self) -> QAccessibleWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QAccessibleWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn backgroundColor<T: QAccessibleWidget_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QAccessibleWidget_backgroundColor {
  fn backgroundColor(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QColor QAccessibleWidget::backgroundColor();
impl<'a> /*trait*/ QAccessibleWidget_backgroundColor for () {
  fn backgroundColor(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget15backgroundColorEv()};
    unsafe {_ZNK17QAccessibleWidget15backgroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn FreeQAccessibleWidget<T: QAccessibleWidget_FreeQAccessibleWidget>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleWidget(self);
    return 1;
  }
}

pub trait QAccessibleWidget_FreeQAccessibleWidget {
  fn FreeQAccessibleWidget(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: void QAccessibleWidget::FreeQAccessibleWidget();
impl<'a> /*trait*/ QAccessibleWidget_FreeQAccessibleWidget for () {
  fn FreeQAccessibleWidget(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidgetD0Ev()};
    unsafe {_ZN17QAccessibleWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn actionNames<T: QAccessibleWidget_actionNames>(&mut self, value: T) -> i32 {
    value.actionNames(self);
    return 1;
  }
}

pub trait QAccessibleWidget_actionNames {
  fn actionNames(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QStringList QAccessibleWidget::actionNames();
impl<'a> /*trait*/ QAccessibleWidget_actionNames for () {
  fn actionNames(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget11actionNamesEv()};
    unsafe {_ZNK17QAccessibleWidget11actionNamesEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn parent<T: QAccessibleWidget_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QAccessibleWidget_parent {
  fn parent(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QAccessibleInterface * QAccessibleWidget::parent();
impl<'a> /*trait*/ QAccessibleWidget_parent for () {
  fn parent(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget6parentEv()};
    unsafe {_ZNK17QAccessibleWidget6parentEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn doAction<T: QAccessibleWidget_doAction>(&mut self, value: T) -> i32 {
    value.doAction(self);
    return 1;
  }
}

pub trait QAccessibleWidget_doAction {
  fn doAction(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: void QAccessibleWidget::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleWidget_doAction for (&'a  QString) {
  fn doAction(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidget8doActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QAccessibleWidget8doActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn keyBindingsForAction<T: QAccessibleWidget_keyBindingsForAction>(&mut self, value: T) -> i32 {
    value.keyBindingsForAction(self);
    return 1;
  }
}

pub trait QAccessibleWidget_keyBindingsForAction {
  fn keyBindingsForAction(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleWidget_keyBindingsForAction for (&'a  QString) {
  fn keyBindingsForAction(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn indexOfChild<T: QAccessibleWidget_indexOfChild>(&mut self, value: T) -> i32 {
    value.indexOfChild(self);
    return 1;
  }
}

pub trait QAccessibleWidget_indexOfChild {
  fn indexOfChild(self, this: &mut QAccessibleWidget) -> i32;
}

// proto: int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
impl<'a> /*trait*/ QAccessibleWidget_indexOfChild for (&'a  QAccessibleInterface) {
  fn indexOfChild(self, this: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface(arg0)};
    return 1;
  }
}

