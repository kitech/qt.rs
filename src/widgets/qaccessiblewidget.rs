// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtWidgets/qaccessiblewidget.h
// dst-file: /src/widgets/qaccessiblewidget.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::gui::qaccessibleobject::QAccessibleObject; // 771
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::gui::qaccessible::QAccessibleInterface; // 771
use super::super::gui::qwindow::QWindow; // 771
use super::super::core::qrect::QRect; // 771
use super::super::gui::qcolor::QColor; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAccessibleWidget_Class_Size() -> c_int;
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
  fn _ZNK17QAccessibleWidget7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QAccessibleInterface * QAccessibleWidget::focusChild();
  fn _ZNK17QAccessibleWidget10focusChildEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleWidget::QAccessibleWidget(const QAccessibleWidget & );
  fn dector_ZN17QAccessibleWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QAccessibleWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QColor QAccessibleWidget::backgroundColor();
  fn _ZNK17QAccessibleWidget15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleWidget::~QAccessibleWidget();
  fn _ZN17QAccessibleWidgetD0Ev(qthis: *mut c_void);
  // proto:  QStringList QAccessibleWidget::actionNames();
  fn _ZNK17QAccessibleWidget11actionNamesEv(qthis: *mut c_void);
  // proto:  QAccessibleInterface * QAccessibleWidget::parent();
  fn _ZNK17QAccessibleWidget6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleWidget::doAction(const QString & actionName);
  fn _ZN17QAccessibleWidget8doActionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
  fn _ZNK17QAccessibleWidget20keyBindingsForActionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
  fn _ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAccessibleWidget)=32
pub struct QAccessibleWidget {
  qbase: QAccessibleObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleWidget {
    return QAccessibleWidget{qbase: QAccessibleObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleWidget {
  type Target = QAccessibleObject;

  fn deref(&self) -> &QAccessibleObject {
    return & self.qbase;
  }
}
impl AsRef<QAccessibleObject> for QAccessibleWidget {
  fn as_ref(& self) -> & QAccessibleObject {
    return & self.qbase;
  }
}
  // proto:  int QAccessibleWidget::childCount();
impl /*struct*/ QAccessibleWidget {
  pub fn childCount<RetType, T: QAccessibleWidget_childCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_childCount<RetType> {
  fn childCount(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  int QAccessibleWidget::childCount();
impl<'a> /*trait*/ QAccessibleWidget_childCount<i32> for () {
  fn childCount(self , rsthis: & QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget10childCountEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleWidget::child(int index);
impl /*struct*/ QAccessibleWidget {
  pub fn child<RetType, T: QAccessibleWidget_child<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_child<RetType> {
  fn child(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleWidget::child(int index);
impl<'a> /*trait*/ QAccessibleWidget_child<QAccessibleInterface> for (i32) {
  fn child(self , rsthis: & QAccessibleWidget) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QAccessibleWidget5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWindow * QAccessibleWidget::window();
impl /*struct*/ QAccessibleWidget {
  pub fn window<RetType, T: QAccessibleWidget_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_window<RetType> {
  fn window(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QWindow * QAccessibleWidget::window();
impl<'a> /*trait*/ QAccessibleWidget_window<QWindow> for () {
  fn window(self , rsthis: & QAccessibleWidget) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget6windowEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QAccessibleWidget::rect();
impl /*struct*/ QAccessibleWidget {
  pub fn rect<RetType, T: QAccessibleWidget_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_rect<RetType> {
  fn rect(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QRect QAccessibleWidget::rect();
impl<'a> /*trait*/ QAccessibleWidget_rect<QRect> for () {
  fn rect(self , rsthis: & QAccessibleWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget4rectEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QColor QAccessibleWidget::foregroundColor();
impl /*struct*/ QAccessibleWidget {
  pub fn foregroundColor<RetType, T: QAccessibleWidget_foregroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foregroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_foregroundColor<RetType> {
  fn foregroundColor(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QColor QAccessibleWidget::foregroundColor();
impl<'a> /*trait*/ QAccessibleWidget_foregroundColor<QColor> for () {
  fn foregroundColor(self , rsthis: & QAccessibleWidget) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget15foregroundColorEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget15foregroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAccessibleWidget::isValid();
impl /*struct*/ QAccessibleWidget {
  pub fn isValid<RetType, T: QAccessibleWidget_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_isValid<RetType> {
  fn isValid(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  bool QAccessibleWidget::isValid();
impl<'a> /*trait*/ QAccessibleWidget_isValid<i8> for () {
  fn isValid(self , rsthis: & QAccessibleWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget7isValidEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleWidget::focusChild();
impl /*struct*/ QAccessibleWidget {
  pub fn focusChild<RetType, T: QAccessibleWidget_focusChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_focusChild<RetType> {
  fn focusChild(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleWidget::focusChild();
impl<'a> /*trait*/ QAccessibleWidget_focusChild<QAccessibleInterface> for () {
  fn focusChild(self , rsthis: & QAccessibleWidget) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget10focusChildEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleWidget::QAccessibleWidget(const QAccessibleWidget & );
impl /*struct*/ QAccessibleWidget {
  pub fn New<T: QAccessibleWidget_New>(value: T) -> QAccessibleWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleWidget_New {
  fn New(self) -> QAccessibleWidget;
}

  // proto:  void QAccessibleWidget::QAccessibleWidget(const QAccessibleWidget & );
impl<'a> /*trait*/ QAccessibleWidget_New for (&'a QAccessibleWidget) {
  fn New(self) -> QAccessibleWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QAccessibleWidget_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QAccessibleWidgetC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QAccessibleWidgetC1ERKS_(arg0)};
    let rsthis = QAccessibleWidget{/**/qbase: QAccessibleObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QColor QAccessibleWidget::backgroundColor();
impl /*struct*/ QAccessibleWidget {
  pub fn backgroundColor<RetType, T: QAccessibleWidget_backgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QColor QAccessibleWidget::backgroundColor();
impl<'a> /*trait*/ QAccessibleWidget_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: & QAccessibleWidget) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget15backgroundColorEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleWidget::~QAccessibleWidget();
impl /*struct*/ QAccessibleWidget {
  pub fn Free<RetType, T: QAccessibleWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_Free<RetType> {
  fn Free(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  void QAccessibleWidget::~QAccessibleWidget();
impl<'a> /*trait*/ QAccessibleWidget_Free<()> for () {
  fn Free(self , rsthis: & QAccessibleWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidgetD0Ev()};
     unsafe {_ZN17QAccessibleWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QAccessibleWidget::actionNames();
impl /*struct*/ QAccessibleWidget {
  pub fn actionNames<RetType, T: QAccessibleWidget_actionNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionNames(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_actionNames<RetType> {
  fn actionNames(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QStringList QAccessibleWidget::actionNames();
impl<'a> /*trait*/ QAccessibleWidget_actionNames<()> for () {
  fn actionNames(self , rsthis: & QAccessibleWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget11actionNamesEv()};
     unsafe {_ZNK17QAccessibleWidget11actionNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleWidget::parent();
impl /*struct*/ QAccessibleWidget {
  pub fn parent<RetType, T: QAccessibleWidget_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_parent<RetType> {
  fn parent(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleWidget::parent();
impl<'a> /*trait*/ QAccessibleWidget_parent<QAccessibleInterface> for () {
  fn parent(self , rsthis: & QAccessibleWidget) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget6parentEv()};
    let mut ret = unsafe {_ZNK17QAccessibleWidget6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleWidget::doAction(const QString & actionName);
impl /*struct*/ QAccessibleWidget {
  pub fn doAction<RetType, T: QAccessibleWidget_doAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doAction(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_doAction<RetType> {
  fn doAction(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  void QAccessibleWidget::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleWidget_doAction<()> for (&'a QString) {
  fn doAction(self , rsthis: & QAccessibleWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleWidget8doActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAccessibleWidget8doActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
impl /*struct*/ QAccessibleWidget {
  pub fn keyBindingsForAction<RetType, T: QAccessibleWidget_keyBindingsForAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyBindingsForAction(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_keyBindingsForAction<RetType> {
  fn keyBindingsForAction(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  QStringList QAccessibleWidget::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleWidget_keyBindingsForAction<()> for (&'a QString) {
  fn keyBindingsForAction(self , rsthis: & QAccessibleWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
impl /*struct*/ QAccessibleWidget {
  pub fn indexOfChild<RetType, T: QAccessibleWidget_indexOfChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleWidget_indexOfChild<RetType> {
  fn indexOfChild(self , rsthis: & QAccessibleWidget) -> RetType;
}

  // proto:  int QAccessibleWidget::indexOfChild(const QAccessibleInterface * child);
impl<'a> /*trait*/ QAccessibleWidget_indexOfChild<i32> for (&'a QAccessibleInterface) {
  fn indexOfChild(self , rsthis: & QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

