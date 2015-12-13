// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;
use super::qwindow::QWindow;
use super::qrect::QRect;
use super::qcolor::QColor;
use super::qstringlist::QStringList;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QAccessibleWidget::childCount();
  fn _ZNK17QAccessibleWidget10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleInterface * QAccessibleWidget::child(int index);
  fn _ZNK17QAccessibleWidget5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWindow * QAccessibleWidget::window();
  fn _ZNK17QAccessibleWidget6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QAccessibleWidget::rect();
  fn _ZNK17QAccessibleWidget4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QAccessibleWidget::foregroundColor();
  fn _ZNK17QAccessibleWidget15foregroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleWidget::isValid();
  fn _ZNK17QAccessibleWidget7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QAccessibleInterface * QAccessibleWidget::focusChild();
  fn _ZNK17QAccessibleWidget10focusChildEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleWidget::NewQAccessibleWidget(const QAccessibleWidget & );
  fn _ZN17QAccessibleWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QAccessibleWidget::backgroundColor();
  fn _ZNK17QAccessibleWidget15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleWidget::FreeQAccessibleWidget();
  fn _ZN17QAccessibleWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  QStringList QAccessibleWidget::actionNames();
  fn _ZNK17QAccessibleWidget11actionNamesEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleWidget::parent();
  fn _ZNK17QAccessibleWidget6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleWidget::doAction(const QString & actionName);
  fn _ZN17QAccessibleWidget8doActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
  fn _ZNK17QAccessibleWidget20keyBindingsForActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
  fn _ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QAccessibleWidget)=32
pub struct QAccessibleWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleWidget {
  pub fn childCount<T: QAccessibleWidget_childCount>(&mut self, value: T) -> i32 {
    return value.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_childCount {
  fn childCount(self, rsthis: &mut QAccessibleWidget) -> i32;
}

// proto:  int QAccessibleWidget::childCount();
impl<'a> /*trait*/ QAccessibleWidget_childCount for () {
  fn childCount(self, rsthis: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget10childCountEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn child<T: QAccessibleWidget_child>(&mut self, value: T) -> QAccessibleInterface {
    return value.child(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_child {
  fn child(self, rsthis: &mut QAccessibleWidget) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleWidget::child(int index);
impl<'a> /*trait*/ QAccessibleWidget_child for (i32) {
  fn child(self, rsthis: &mut QAccessibleWidget) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QAccessibleWidget5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn window<T: QAccessibleWidget_window>(&mut self, value: T) -> QWindow {
    return value.window(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_window {
  fn window(self, rsthis: &mut QAccessibleWidget) -> QWindow;
}

// proto:  QWindow * QAccessibleWidget::window();
impl<'a> /*trait*/ QAccessibleWidget_window for () {
  fn window(self, rsthis: &mut QAccessibleWidget) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget6windowEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn rect<T: QAccessibleWidget_rect>(&mut self, value: T) -> QRect {
    return value.rect(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_rect {
  fn rect(self, rsthis: &mut QAccessibleWidget) -> QRect;
}

// proto:  QRect QAccessibleWidget::rect();
impl<'a> /*trait*/ QAccessibleWidget_rect for () {
  fn rect(self, rsthis: &mut QAccessibleWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget4rectEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn foregroundColor<T: QAccessibleWidget_foregroundColor>(&mut self, value: T) -> QColor {
    return value.foregroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_foregroundColor {
  fn foregroundColor(self, rsthis: &mut QAccessibleWidget) -> QColor;
}

// proto:  QColor QAccessibleWidget::foregroundColor();
impl<'a> /*trait*/ QAccessibleWidget_foregroundColor for () {
  fn foregroundColor(self, rsthis: &mut QAccessibleWidget) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget15foregroundColorEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget15foregroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn isValid<T: QAccessibleWidget_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_isValid {
  fn isValid(self, rsthis: &mut QAccessibleWidget) -> i8;
}

// proto:  bool QAccessibleWidget::isValid();
impl<'a> /*trait*/ QAccessibleWidget_isValid for () {
  fn isValid(self, rsthis: &mut QAccessibleWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget7isValidEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn focusChild<T: QAccessibleWidget_focusChild>(&mut self, value: T) -> QAccessibleInterface {
    return value.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_focusChild {
  fn focusChild(self, rsthis: &mut QAccessibleWidget) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleWidget::focusChild();
impl<'a> /*trait*/ QAccessibleWidget_focusChild for () {
  fn focusChild(self, rsthis: &mut QAccessibleWidget) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget10focusChildEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessibleWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn backgroundColor<T: QAccessibleWidget_backgroundColor>(&mut self, value: T) -> QColor {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_backgroundColor {
  fn backgroundColor(self, rsthis: &mut QAccessibleWidget) -> QColor;
}

// proto:  QColor QAccessibleWidget::backgroundColor();
impl<'a> /*trait*/ QAccessibleWidget_backgroundColor for () {
  fn backgroundColor(self, rsthis: &mut QAccessibleWidget) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget15backgroundColorEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn FreeQAccessibleWidget<T: QAccessibleWidget_FreeQAccessibleWidget>(&mut self, value: T)  {
     value.FreeQAccessibleWidget(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_FreeQAccessibleWidget {
  fn FreeQAccessibleWidget(self, rsthis: &mut QAccessibleWidget) ;
}

// proto:  void QAccessibleWidget::FreeQAccessibleWidget();
impl<'a> /*trait*/ QAccessibleWidget_FreeQAccessibleWidget for () {
  fn FreeQAccessibleWidget(self, rsthis: &mut QAccessibleWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidgetD0Ev()};
     unsafe {_ZN17QAccessibleWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn actionNames<T: QAccessibleWidget_actionNames>(&mut self, value: T) -> QStringList {
    return value.actionNames(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_actionNames {
  fn actionNames(self, rsthis: &mut QAccessibleWidget) -> QStringList;
}

// proto:  QStringList QAccessibleWidget::actionNames();
impl<'a> /*trait*/ QAccessibleWidget_actionNames for () {
  fn actionNames(self, rsthis: &mut QAccessibleWidget) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget11actionNamesEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget11actionNamesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn parent<T: QAccessibleWidget_parent>(&mut self, value: T) -> QAccessibleInterface {
    return value.parent(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_parent {
  fn parent(self, rsthis: &mut QAccessibleWidget) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleWidget::parent();
impl<'a> /*trait*/ QAccessibleWidget_parent for () {
  fn parent(self, rsthis: &mut QAccessibleWidget) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget6parentEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn doAction<T: QAccessibleWidget_doAction>(&mut self, value: T)  {
     value.doAction(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_doAction {
  fn doAction(self, rsthis: &mut QAccessibleWidget) ;
}

// proto:  void QAccessibleWidget::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleWidget_doAction for (&'a  QString) {
  fn doAction(self, rsthis: &mut QAccessibleWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidget8doActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAccessibleWidget8doActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn keyBindingsForAction<T: QAccessibleWidget_keyBindingsForAction>(&mut self, value: T) -> QStringList {
    return value.keyBindingsForAction(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_keyBindingsForAction {
  fn keyBindingsForAction(self, rsthis: &mut QAccessibleWidget) -> QStringList;
}

// proto:  QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleWidget_keyBindingsForAction for (&'a  QString) {
  fn keyBindingsForAction(self, rsthis: &mut QAccessibleWidget) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleWidget {
  pub fn indexOfChild<T: QAccessibleWidget_indexOfChild>(&mut self, value: T) -> i32 {
    return value.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_indexOfChild {
  fn indexOfChild(self, rsthis: &mut QAccessibleWidget) -> i32;
}

// proto:  int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
impl<'a> /*trait*/ QAccessibleWidget_indexOfChild for (&'a  QAccessibleInterface) {
  fn indexOfChild(self, rsthis: &mut QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

