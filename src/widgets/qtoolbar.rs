// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qtoolbar.h
// dst-file: /src/widgets/qtoolbar.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qobject::QObject; // 771
use super::qaction::QAction; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QToolBar_Class_Size() -> c_int;
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_char) -> *mut c_void;
  // proto:  bool QToolBar::isFloatable();
  fn _ZNK8QToolBar11isFloatableEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QToolBar::iconSize();
  fn _ZNK8QToolBar8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QToolBar::actionGeometry(QAction * action);
  fn _ZNK8QToolBar14actionGeometryEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QToolBar::widgetForAction(QAction * action);
  fn _ZNK8QToolBar15widgetForActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::visibilityChanged(bool visible);
  fn _ZN8QToolBar17visibilityChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QToolBar::clear();
  fn _ZN8QToolBar5clearEv(qthis: *mut c_void);
  // proto:  void QToolBar::QToolBar(const QString & title, QWidget * parent);
  fn dector_ZN8QToolBarC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN8QToolBarC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QToolBar::setMovable(bool movable);
  fn _ZN8QToolBar10setMovableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QToolBar::isMovable();
  fn _ZNK8QToolBar9isMovableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QToolBar::setIconSize(const QSize & iconSize);
  fn _ZN8QToolBar11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAction * QToolBar::addSeparator();
  fn _ZN8QToolBar12addSeparatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::setFloatable(bool floatable);
  fn _ZN8QToolBar12setFloatableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QAction * QToolBar::addAction(const QString & text);
  fn _ZN8QToolBar9addActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::topLevelChanged(bool topLevel);
  fn _ZN8QToolBar15topLevelChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QToolBar::actionTriggered(QAction * action);
  fn _ZN8QToolBar15actionTriggeredEP7QAction(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
  fn _ZN8QToolBar9addActionERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::QToolBar(QWidget * parent);
  fn dector_ZN8QToolBarC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QToolBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAction * QToolBar::actionAt(const QPoint & p);
  fn _ZNK8QToolBar8actionAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::movableChanged(bool movable);
  fn _ZN8QToolBar14movableChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QAction * QToolBar::actionAt(int x, int y);
  fn _ZNK8QToolBar8actionAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QToolBar::iconSizeChanged(const QSize & iconSize);
  fn _ZN8QToolBar15iconSizeChangedERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QToolBar::isFloating();
  fn _ZNK8QToolBar10isFloatingEv(qthis: *mut c_void) -> c_char;
  // proto:  QAction * QToolBar::toggleViewAction();
  fn _ZNK8QToolBar16toggleViewActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::QToolBar(const QToolBar & );
  fn dector_ZN8QToolBarC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QToolBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QToolBar::~QToolBar();
  fn _ZN8QToolBarD0Ev(qthis: *mut c_void);
  // proto:  QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char) -> *mut c_void;
  // proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
  fn _ZN8QToolBar12insertWidgetEP7QActionP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QAction * QToolBar::addWidget(QWidget * widget);
  fn _ZN8QToolBar9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QToolBar::metaObject();
  fn _ZNK8QToolBar10metaObjectEv(qthis: *mut c_void);
  // proto:  QAction * QToolBar::insertSeparator(QAction * before);
  fn _ZN8QToolBar15insertSeparatorEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QToolBar)=1
pub struct QToolBar {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolBar {
  pub fn inheritFrom(qthis: *mut c_void) -> QToolBar {
    return QToolBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QToolBar {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QToolBar {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
impl /*struct*/ QToolBar {
  pub fn addAction<RetType, T: QToolBar_addAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QToolBar_addAction<RetType> {
  fn addAction(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a QIcon, &'a QString, &'a QObject, &'a  String) {
  fn addAction(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QToolBar::isFloatable();
impl /*struct*/ QToolBar {
  pub fn isFloatable<RetType, T: QToolBar_isFloatable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFloatable(self);
    // return 1;
  }
}

pub trait QToolBar_isFloatable<RetType> {
  fn isFloatable(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  bool QToolBar::isFloatable();
impl<'a> /*trait*/ QToolBar_isFloatable<i8> for () {
  fn isFloatable(self , rsthis: & QToolBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar11isFloatableEv()};
    let mut ret = unsafe {_ZNK8QToolBar11isFloatableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QToolBar::iconSize();
impl /*struct*/ QToolBar {
  pub fn iconSize<RetType, T: QToolBar_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QToolBar_iconSize<RetType> {
  fn iconSize(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QSize QToolBar::iconSize();
impl<'a> /*trait*/ QToolBar_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QToolBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8iconSizeEv()};
    let mut ret = unsafe {_ZNK8QToolBar8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QToolBar::actionGeometry(QAction * action);
impl /*struct*/ QToolBar {
  pub fn actionGeometry<RetType, T: QToolBar_actionGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionGeometry(self);
    // return 1;
  }
}

pub trait QToolBar_actionGeometry<RetType> {
  fn actionGeometry(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QRect QToolBar::actionGeometry(QAction * action);
impl<'a> /*trait*/ QToolBar_actionGeometry<QRect> for (&'a QAction) {
  fn actionGeometry(self , rsthis: & QToolBar) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBar14actionGeometryEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QToolBar::widgetForAction(QAction * action);
impl /*struct*/ QToolBar {
  pub fn widgetForAction<RetType, T: QToolBar_widgetForAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widgetForAction(self);
    // return 1;
  }
}

pub trait QToolBar_widgetForAction<RetType> {
  fn widgetForAction(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QWidget * QToolBar::widgetForAction(QAction * action);
impl<'a> /*trait*/ QToolBar_widgetForAction<QWidget> for (&'a QAction) {
  fn widgetForAction(self , rsthis: & QToolBar) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar15widgetForActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBar15widgetForActionEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::visibilityChanged(bool visible);
impl /*struct*/ QToolBar {
  pub fn visibilityChanged<RetType, T: QToolBar_visibilityChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visibilityChanged(self);
    // return 1;
  }
}

pub trait QToolBar_visibilityChanged<RetType> {
  fn visibilityChanged(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::visibilityChanged(bool visible);
impl<'a> /*trait*/ QToolBar_visibilityChanged<()> for (i8) {
  fn visibilityChanged(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar17visibilityChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QToolBar17visibilityChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QToolBar::clear();
impl /*struct*/ QToolBar {
  pub fn clear<RetType, T: QToolBar_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QToolBar_clear<RetType> {
  fn clear(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::clear();
impl<'a> /*trait*/ QToolBar_clear<()> for () {
  fn clear(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar5clearEv()};
     unsafe {_ZN8QToolBar5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QToolBar::QToolBar(const QString & title, QWidget * parent);
impl /*struct*/ QToolBar {
  pub fn New<T: QToolBar_New>(value: T) -> QToolBar {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBar_New {
  fn New(self) -> QToolBar;
}

  // proto:  void QToolBar::QToolBar(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QToolBar_New for (&'a QString, &'a QWidget) {
  fn New(self) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QToolBar_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN8QToolBarC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN8QToolBarC1ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QToolBar{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolBar::setMovable(bool movable);
impl /*struct*/ QToolBar {
  pub fn setMovable<RetType, T: QToolBar_setMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMovable(self);
    // return 1;
  }
}

pub trait QToolBar_setMovable<RetType> {
  fn setMovable(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::setMovable(bool movable);
impl<'a> /*trait*/ QToolBar_setMovable<()> for (i8) {
  fn setMovable(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar10setMovableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QToolBar10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QToolBar::isMovable();
impl /*struct*/ QToolBar {
  pub fn isMovable<RetType, T: QToolBar_isMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMovable(self);
    // return 1;
  }
}

pub trait QToolBar_isMovable<RetType> {
  fn isMovable(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  bool QToolBar::isMovable();
impl<'a> /*trait*/ QToolBar_isMovable<i8> for () {
  fn isMovable(self , rsthis: & QToolBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar9isMovableEv()};
    let mut ret = unsafe {_ZNK8QToolBar9isMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QToolBar::setIconSize(const QSize & iconSize);
impl /*struct*/ QToolBar {
  pub fn setIconSize<RetType, T: QToolBar_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QToolBar_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QToolBar_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBar11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::addSeparator();
impl /*struct*/ QToolBar {
  pub fn addSeparator<RetType, T: QToolBar_addSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addSeparator(self);
    // return 1;
  }
}

pub trait QToolBar_addSeparator<RetType> {
  fn addSeparator(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::addSeparator();
impl<'a> /*trait*/ QToolBar_addSeparator<QAction> for () {
  fn addSeparator(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12addSeparatorEv()};
    let mut ret = unsafe {_ZN8QToolBar12addSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::setFloatable(bool floatable);
impl /*struct*/ QToolBar {
  pub fn setFloatable<RetType, T: QToolBar_setFloatable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFloatable(self);
    // return 1;
  }
}

pub trait QToolBar_setFloatable<RetType> {
  fn setFloatable(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::setFloatable(bool floatable);
impl<'a> /*trait*/ QToolBar_setFloatable<()> for (i8) {
  fn setFloatable(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12setFloatableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QToolBar12setFloatableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::addAction(const QString & text);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a QString) {
  fn addAction(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::topLevelChanged(bool topLevel);
impl /*struct*/ QToolBar {
  pub fn topLevelChanged<RetType, T: QToolBar_topLevelChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelChanged(self);
    // return 1;
  }
}

pub trait QToolBar_topLevelChanged<RetType> {
  fn topLevelChanged(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::topLevelChanged(bool topLevel);
impl<'a> /*trait*/ QToolBar_topLevelChanged<()> for (i8) {
  fn topLevelChanged(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15topLevelChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QToolBar15topLevelChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QToolBar::actionTriggered(QAction * action);
impl /*struct*/ QToolBar {
  pub fn actionTriggered<RetType, T: QToolBar_actionTriggered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionTriggered(self);
    // return 1;
  }
}

pub trait QToolBar_actionTriggered<RetType> {
  fn actionTriggered(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::actionTriggered(QAction * action);
impl<'a> /*trait*/ QToolBar_actionTriggered<()> for (&'a QAction) {
  fn actionTriggered(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15actionTriggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBar15actionTriggeredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a QIcon, &'a QString) {
  fn addAction(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::QToolBar(QWidget * parent);
impl<'a> /*trait*/ QToolBar_New for (&'a QWidget) {
  fn New(self) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1EP7QWidget()};
    let ctysz: c_int = unsafe{QToolBar_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QToolBarC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QToolBarC1EP7QWidget(arg0)};
    let rsthis = QToolBar{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAction * QToolBar::actionAt(const QPoint & p);
impl /*struct*/ QToolBar {
  pub fn actionAt<RetType, T: QToolBar_actionAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionAt(self);
    // return 1;
  }
}

pub trait QToolBar_actionAt<RetType> {
  fn actionAt(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::actionAt(const QPoint & p);
impl<'a> /*trait*/ QToolBar_actionAt<QAction> for (&'a QPoint) {
  fn actionAt(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBar8actionAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::movableChanged(bool movable);
impl /*struct*/ QToolBar {
  pub fn movableChanged<RetType, T: QToolBar_movableChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.movableChanged(self);
    // return 1;
  }
}

pub trait QToolBar_movableChanged<RetType> {
  fn movableChanged(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::movableChanged(bool movable);
impl<'a> /*trait*/ QToolBar_movableChanged<()> for (i8) {
  fn movableChanged(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar14movableChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QToolBar14movableChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::actionAt(int x, int y);
impl<'a> /*trait*/ QToolBar_actionAt<QAction> for (i32, i32) {
  fn actionAt(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8actionAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK8QToolBar8actionAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::iconSizeChanged(const QSize & iconSize);
impl /*struct*/ QToolBar {
  pub fn iconSizeChanged<RetType, T: QToolBar_iconSizeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSizeChanged(self);
    // return 1;
  }
}

pub trait QToolBar_iconSizeChanged<RetType> {
  fn iconSizeChanged(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QToolBar_iconSizeChanged<()> for (&'a QSize) {
  fn iconSizeChanged(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBar15iconSizeChangedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QToolBar::isFloating();
impl /*struct*/ QToolBar {
  pub fn isFloating<RetType, T: QToolBar_isFloating<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFloating(self);
    // return 1;
  }
}

pub trait QToolBar_isFloating<RetType> {
  fn isFloating(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  bool QToolBar::isFloating();
impl<'a> /*trait*/ QToolBar_isFloating<i8> for () {
  fn isFloating(self , rsthis: & QToolBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10isFloatingEv()};
    let mut ret = unsafe {_ZNK8QToolBar10isFloatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAction * QToolBar::toggleViewAction();
impl /*struct*/ QToolBar {
  pub fn toggleViewAction<RetType, T: QToolBar_toggleViewAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggleViewAction(self);
    // return 1;
  }
}

pub trait QToolBar_toggleViewAction<RetType> {
  fn toggleViewAction(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::toggleViewAction();
impl<'a> /*trait*/ QToolBar_toggleViewAction<QAction> for () {
  fn toggleViewAction(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar16toggleViewActionEv()};
    let mut ret = unsafe {_ZNK8QToolBar16toggleViewActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::QToolBar(const QToolBar & );
impl<'a> /*trait*/ QToolBar_New for (&'a QToolBar) {
  fn New(self) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1ERKS_()};
    let ctysz: c_int = unsafe{QToolBar_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QToolBarC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QToolBarC1ERKS_(arg0)};
    let rsthis = QToolBar{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolBar::~QToolBar();
impl /*struct*/ QToolBar {
  pub fn Free<RetType, T: QToolBar_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QToolBar_Free<RetType> {
  fn Free(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::~QToolBar();
impl<'a> /*trait*/ QToolBar_Free<()> for () {
  fn Free(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarD0Ev()};
     unsafe {_ZN8QToolBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a QString, &'a QObject, &'a  String) {
  fn addAction(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
impl /*struct*/ QToolBar {
  pub fn insertWidget<RetType, T: QToolBar_insertWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QToolBar_insertWidget<RetType> {
  fn insertWidget(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
impl<'a> /*trait*/ QToolBar_insertWidget<QAction> for (&'a QAction, &'a QWidget) {
  fn insertWidget(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12insertWidgetEP7QActionP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar12insertWidgetEP7QActionP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QToolBar::addWidget(QWidget * widget);
impl /*struct*/ QToolBar {
  pub fn addWidget<RetType, T: QToolBar_addWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QToolBar_addWidget<RetType> {
  fn addWidget(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::addWidget(QWidget * widget);
impl<'a> /*trait*/ QToolBar_addWidget<QAction> for (&'a QWidget) {
  fn addWidget(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QToolBar::metaObject();
impl /*struct*/ QToolBar {
  pub fn metaObject<RetType, T: QToolBar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QToolBar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  const QMetaObject * QToolBar::metaObject();
impl<'a> /*trait*/ QToolBar_metaObject<()> for () {
  fn metaObject(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10metaObjectEv()};
     unsafe {_ZNK8QToolBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::insertSeparator(QAction * before);
impl /*struct*/ QToolBar {
  pub fn insertSeparator<RetType, T: QToolBar_insertSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator(self);
    // return 1;
  }
}

pub trait QToolBar_insertSeparator<RetType> {
  fn insertSeparator(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  QAction * QToolBar::insertSeparator(QAction * before);
impl<'a> /*trait*/ QToolBar_insertSeparator<QAction> for (&'a QAction) {
  fn insertSeparator(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar15insertSeparatorEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

