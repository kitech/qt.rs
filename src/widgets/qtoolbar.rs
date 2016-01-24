// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
use super::super::core::qobjectdefs::QMetaObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QToolBar_Class_Size() -> c_int;
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
  fn C_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_char) -> *mut c_void;
  // proto:  bool QToolBar::isFloatable();
  fn C_ZNK8QToolBar11isFloatableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QToolBar::iconSize();
  fn C_ZNK8QToolBar8iconSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QToolBar::actionGeometry(QAction * action);
  fn C_ZNK8QToolBar14actionGeometryEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QToolBar::widgetForAction(QAction * action);
  fn C_ZNK8QToolBar15widgetForActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::clear();
  fn C_ZN8QToolBar5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QToolBar::QToolBar(const QString & title, QWidget * parent);
  fn C_ZN8QToolBarC2ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QToolBar::setMovable(bool movable);
  fn C_ZN8QToolBar10setMovableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QToolBar::isMovable();
  fn C_ZNK8QToolBar9isMovableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QToolBar::setIconSize(const QSize & iconSize);
  fn C_ZN8QToolBar11setIconSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAction * QToolBar::addSeparator();
  fn C_ZN8QToolBar12addSeparatorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QToolBar::setFloatable(bool floatable);
  fn C_ZN8QToolBar12setFloatableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QAction * QToolBar::addAction(const QString & text);
  fn C_ZN8QToolBar9addActionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
  fn C_ZN8QToolBar9addActionERK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::QToolBar(QWidget * parent);
  fn C_ZN8QToolBarC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QAction * QToolBar::actionAt(const QPoint & p);
  fn C_ZNK8QToolBar8actionAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QToolBar::actionAt(int x, int y);
  fn C_ZNK8QToolBar8actionAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QToolBar::isFloating();
  fn C_ZNK8QToolBar10isFloatingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAction * QToolBar::toggleViewAction();
  fn C_ZNK8QToolBar16toggleViewActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QToolBar::~QToolBar();
  fn C_ZN8QToolBarD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
  fn C_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char) -> *mut c_void;
  // proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
  fn C_ZN8QToolBar12insertWidgetEP7QActionP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QAction * QToolBar::addWidget(QWidget * widget);
  fn C_ZN8QToolBar9addWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QToolBar::metaObject();
  fn C_ZNK8QToolBar10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QToolBar::insertSeparator(QAction * before);
  fn C_ZN8QToolBar15insertSeparatorEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  fn QToolBar_SlotProxy_connect__ZN8QToolBar15actionTriggeredEP7QAction(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QToolBar_SlotProxy_connect__ZN8QToolBar14movableChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QToolBar_SlotProxy_connect__ZN8QToolBar17visibilityChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QToolBar_SlotProxy_connect__ZN8QToolBar15iconSizeChangedERK5QSize(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QToolBar_SlotProxy_connect__ZN8QToolBar15topLevelChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QToolBar)=1
#[derive(Default)]
pub struct QToolBar {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _iconSizeChanged: QToolBar_iconSizeChanged_signal,
  pub _allowedAreasChanged: QToolBar_allowedAreasChanged_signal,
  pub _movableChanged: QToolBar_movableChanged_signal,
  pub _toolButtonStyleChanged: QToolBar_toolButtonStyleChanged_signal,
  pub _topLevelChanged: QToolBar_topLevelChanged_signal,
  pub _actionTriggered: QToolBar_actionTriggered_signal,
  pub _orientationChanged: QToolBar_orientationChanged_signal,
  pub _visibilityChanged: QToolBar_visibilityChanged_signal,
}

impl /*struct*/ QToolBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QToolBar {
    return QToolBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK8QToolBar11isFloatableEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK8QToolBar8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK8QToolBar14actionGeometryEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK8QToolBar15widgetForActionEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN8QToolBar5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QToolBar::QToolBar(const QString & title, QWidget * parent);
impl /*struct*/ QToolBar {
  pub fn new<T: QToolBar_new>(value: T) -> QToolBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBar_new {
  fn new(self) -> QToolBar;
}

  // proto:  void QToolBar::QToolBar(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QToolBar_new for (&'a QString, &'a QWidget) {
  fn new(self) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC2ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QToolBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QToolBarC2ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QToolBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN8QToolBar10setMovableEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK8QToolBar9isMovableEv(rsthis.qclsinst)};
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
     unsafe {C_ZN8QToolBar11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZN8QToolBar12addSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
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
     unsafe {C_ZN8QToolBar12setFloatableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QToolBar::addAction(const QString & text);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a QString) {
  fn addAction(self , rsthis: & QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN8QToolBar9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZN8QToolBar9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::QToolBar(QWidget * parent);
impl<'a> /*trait*/ QToolBar_new for (&'a QWidget) {
  fn new(self) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC2EP7QWidget()};
    let ctysz: c_int = unsafe{QToolBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QToolBarC2EP7QWidget(arg0)};
    let rsthis = QToolBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK8QToolBar8actionAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK8QToolBar8actionAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK8QToolBar10isFloatingEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK8QToolBar16toggleViewActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBar::~QToolBar();
impl /*struct*/ QToolBar {
  pub fn free<RetType, T: QToolBar_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QToolBar_free<RetType> {
  fn free(self , rsthis: & QToolBar) -> RetType;
}

  // proto:  void QToolBar::~QToolBar();
impl<'a> /*trait*/ QToolBar_free<()> for () {
  fn free(self , rsthis: & QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarD2Ev()};
     unsafe {C_ZN8QToolBarD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZN8QToolBar12insertWidgetEP7QActionP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZN8QToolBar9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QToolBar_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QToolBar) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10metaObjectEv()};
    let mut ret = unsafe {C_ZNK8QToolBar10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZN8QToolBar15insertSeparatorEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QToolBar_iconSizeChanged
pub struct QToolBar_iconSizeChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn iconSizeChanged(&self) -> QToolBar_iconSizeChanged_signal {
     return QToolBar_iconSizeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_iconSizeChanged_signal {
  pub fn connect<T: QToolBar_iconSizeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_iconSizeChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_iconSizeChanged_signal);
}

#[derive(Default)] // for QToolBar_allowedAreasChanged
pub struct QToolBar_allowedAreasChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn allowedAreasChanged(&self) -> QToolBar_allowedAreasChanged_signal {
     return QToolBar_allowedAreasChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_allowedAreasChanged_signal {
  pub fn connect<T: QToolBar_allowedAreasChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_allowedAreasChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_allowedAreasChanged_signal);
}

#[derive(Default)] // for QToolBar_movableChanged
pub struct QToolBar_movableChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn movableChanged(&self) -> QToolBar_movableChanged_signal {
     return QToolBar_movableChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_movableChanged_signal {
  pub fn connect<T: QToolBar_movableChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_movableChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_movableChanged_signal);
}

#[derive(Default)] // for QToolBar_toolButtonStyleChanged
pub struct QToolBar_toolButtonStyleChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn toolButtonStyleChanged(&self) -> QToolBar_toolButtonStyleChanged_signal {
     return QToolBar_toolButtonStyleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_toolButtonStyleChanged_signal {
  pub fn connect<T: QToolBar_toolButtonStyleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_toolButtonStyleChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_toolButtonStyleChanged_signal);
}

#[derive(Default)] // for QToolBar_topLevelChanged
pub struct QToolBar_topLevelChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn topLevelChanged(&self) -> QToolBar_topLevelChanged_signal {
     return QToolBar_topLevelChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_topLevelChanged_signal {
  pub fn connect<T: QToolBar_topLevelChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_topLevelChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_topLevelChanged_signal);
}

#[derive(Default)] // for QToolBar_actionTriggered
pub struct QToolBar_actionTriggered_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn actionTriggered(&self) -> QToolBar_actionTriggered_signal {
     return QToolBar_actionTriggered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_actionTriggered_signal {
  pub fn connect<T: QToolBar_actionTriggered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_actionTriggered_signal_connect {
  fn connect(self, sigthis: QToolBar_actionTriggered_signal);
}

#[derive(Default)] // for QToolBar_orientationChanged
pub struct QToolBar_orientationChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn orientationChanged(&self) -> QToolBar_orientationChanged_signal {
     return QToolBar_orientationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_orientationChanged_signal {
  pub fn connect<T: QToolBar_orientationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_orientationChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_orientationChanged_signal);
}

#[derive(Default)] // for QToolBar_visibilityChanged
pub struct QToolBar_visibilityChanged_signal{poi:u64}
impl /* struct */ QToolBar {
  pub fn visibilityChanged(&self) -> QToolBar_visibilityChanged_signal {
     return QToolBar_visibilityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolBar_visibilityChanged_signal {
  pub fn connect<T: QToolBar_visibilityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolBar_visibilityChanged_signal_connect {
  fn connect(self, sigthis: QToolBar_visibilityChanged_signal);
}

// actionTriggered(class QAction *)
extern fn QToolBar_actionTriggered_signal_connect_cb_0(rsfptr:fn(QAction), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QToolBar_actionTriggered_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QAction)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QToolBar_actionTriggered_signal_connect for fn(QAction) {
  fn connect(self, sigthis: QToolBar_actionTriggered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_actionTriggered_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar15actionTriggeredEP7QAction(arg0, arg1, arg2)};
  }
}
impl /* trait */ QToolBar_actionTriggered_signal_connect for Box<Fn(QAction)> {
  fn connect(self, sigthis: QToolBar_actionTriggered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_actionTriggered_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar15actionTriggeredEP7QAction(arg0, arg1, arg2)};
  }
}
// movableChanged(_Bool)
extern fn QToolBar_movableChanged_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QToolBar_movableChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QToolBar_movableChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QToolBar_movableChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_movableChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar14movableChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QToolBar_movableChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QToolBar_movableChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_movableChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar14movableChangedEb(arg0, arg1, arg2)};
  }
}
// visibilityChanged(_Bool)
extern fn QToolBar_visibilityChanged_signal_connect_cb_2(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QToolBar_visibilityChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QToolBar_visibilityChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QToolBar_visibilityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_visibilityChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar17visibilityChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QToolBar_visibilityChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QToolBar_visibilityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_visibilityChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar17visibilityChangedEb(arg0, arg1, arg2)};
  }
}
// iconSizeChanged(const class QSize &)
extern fn QToolBar_iconSizeChanged_signal_connect_cb_3(rsfptr:fn(QSize), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QSize::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QToolBar_iconSizeChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QSize)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QSize::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QToolBar_iconSizeChanged_signal_connect for fn(QSize) {
  fn connect(self, sigthis: QToolBar_iconSizeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_iconSizeChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar15iconSizeChangedERK5QSize(arg0, arg1, arg2)};
  }
}
impl /* trait */ QToolBar_iconSizeChanged_signal_connect for Box<Fn(QSize)> {
  fn connect(self, sigthis: QToolBar_iconSizeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_iconSizeChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar15iconSizeChangedERK5QSize(arg0, arg1, arg2)};
  }
}
// topLevelChanged(_Bool)
extern fn QToolBar_topLevelChanged_signal_connect_cb_4(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QToolBar_topLevelChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QToolBar_topLevelChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QToolBar_topLevelChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_topLevelChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar15topLevelChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QToolBar_topLevelChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QToolBar_topLevelChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolBar_topLevelChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QToolBar_SlotProxy_connect__ZN8QToolBar15topLevelChangedEb(arg0, arg1, arg2)};
  }
}
// <= body block end

