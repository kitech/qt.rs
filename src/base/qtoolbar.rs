// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qicon::QIcon;
use super::qstring::QString;
use super::qobject::QObject;
use super::qaction::QAction;
use super::qsize::QSize;
use super::qrect::QRect;
use super::qwidget::QWidget;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *const c_char) -> *mut c_void;
  // proto:  bool QToolBar::isFloatable();
  fn _ZNK8QToolBar11isFloatableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QToolBar::iconSize();
  fn _ZNK8QToolBar8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QToolBar::actionGeometry(QAction * action);
  fn _ZNK8QToolBar14actionGeometryEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QToolBar::widgetForAction(QAction * action);
  fn _ZNK8QToolBar15widgetForActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::visibilityChanged(bool visible);
  fn _ZN8QToolBar17visibilityChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QToolBar::clear();
  fn _ZN8QToolBar5clearEv(qthis: *mut c_void) ;
  // proto:  void QToolBar::NewQToolBar(const QString & title, QWidget * parent);
  fn _ZN8QToolBarC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QToolBar::setMovable(bool movable);
  fn _ZN8QToolBar10setMovableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QToolBar::isMovable();
  fn _ZNK8QToolBar9isMovableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QToolBar::setIconSize(const QSize & iconSize);
  fn _ZN8QToolBar11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QToolBar::addSeparator();
  fn _ZN8QToolBar12addSeparatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::setFloatable(bool floatable);
  fn _ZN8QToolBar12setFloatableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QAction * QToolBar::addAction(const QString & text);
  fn _ZN8QToolBar9addActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::topLevelChanged(bool topLevel);
  fn _ZN8QToolBar15topLevelChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QToolBar::actionTriggered(QAction * action);
  fn _ZN8QToolBar15actionTriggeredEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
  fn _ZN8QToolBar9addActionERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::NewQToolBar(QWidget * parent);
  fn _ZN8QToolBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QToolBar::actionAt(const QPoint & p);
  fn _ZNK8QToolBar8actionAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::movableChanged(bool movable);
  fn _ZN8QToolBar14movableChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QAction * QToolBar::actionAt(int x, int y);
  fn _ZNK8QToolBar8actionAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QToolBar::iconSizeChanged(const QSize & iconSize);
  fn _ZN8QToolBar15iconSizeChangedERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QToolBar::isFloating();
  fn _ZNK8QToolBar10isFloatingEv(qthis: *mut c_void) -> int8_t;
  // proto:  QAction * QToolBar::toggleViewAction();
  fn _ZNK8QToolBar16toggleViewActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolBar::NewQToolBar(const QToolBar & );
  fn _ZN8QToolBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QToolBar::FreeQToolBar();
  fn _ZN8QToolBarD0Ev(qthis: *mut c_void) ;
  // proto:  QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *const c_char) -> *mut c_void;
  // proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
  fn _ZN8QToolBar12insertWidgetEP7QActionP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QAction * QToolBar::addWidget(QWidget * widget);
  fn _ZN8QToolBar9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QToolBar::metaObject();
  fn _ZNK8QToolBar10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QAction * QToolBar::insertSeparator(QAction * before);
  fn _ZN8QToolBar15insertSeparatorEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QToolBar)=1
pub struct QToolBar {
  pub qclsinst: *mut c_void,
}

// proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
impl /*struct*/ QToolBar {
  pub fn addAction<RetType, T: QToolBar_addAction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QToolBar_addAction<RetType> {
  fn addAction(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a  QIcon, &'a  QString, &'a  QObject, &'a  String) {
  fn addAction(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QToolBar::isFloatable();
impl /*struct*/ QToolBar {
  pub fn isFloatable<RetType, T: QToolBar_isFloatable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isFloatable(self);
    // return 1;
  }
}

pub trait QToolBar_isFloatable<RetType> {
  fn isFloatable(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  bool QToolBar::isFloatable();
impl<'a> /*trait*/ QToolBar_isFloatable<i8> for () {
  fn isFloatable(self , rsthis: &mut QToolBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar11isFloatableEv()};
    let mut ret = unsafe {_ZNK8QToolBar11isFloatableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QSize QToolBar::iconSize();
impl /*struct*/ QToolBar {
  pub fn iconSize<RetType, T: QToolBar_iconSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QToolBar_iconSize<RetType> {
  fn iconSize(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QSize QToolBar::iconSize();
impl<'a> /*trait*/ QToolBar_iconSize<QSize> for () {
  fn iconSize(self , rsthis: &mut QToolBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8iconSizeEv()};
    let mut ret = unsafe {_ZNK8QToolBar8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRect QToolBar::actionGeometry(QAction * action);
impl /*struct*/ QToolBar {
  pub fn actionGeometry<RetType, T: QToolBar_actionGeometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.actionGeometry(self);
    // return 1;
  }
}

pub trait QToolBar_actionGeometry<RetType> {
  fn actionGeometry(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QRect QToolBar::actionGeometry(QAction * action);
impl<'a> /*trait*/ QToolBar_actionGeometry<QRect> for (&'a mut QAction) {
  fn actionGeometry(self , rsthis: &mut QToolBar) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBar14actionGeometryEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QWidget * QToolBar::widgetForAction(QAction * action);
impl /*struct*/ QToolBar {
  pub fn widgetForAction<RetType, T: QToolBar_widgetForAction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.widgetForAction(self);
    // return 1;
  }
}

pub trait QToolBar_widgetForAction<RetType> {
  fn widgetForAction(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QWidget * QToolBar::widgetForAction(QAction * action);
impl<'a> /*trait*/ QToolBar_widgetForAction<QWidget> for (&'a mut QAction) {
  fn widgetForAction(self , rsthis: &mut QToolBar) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar15widgetForActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBar15widgetForActionEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QToolBar::visibilityChanged(bool visible);
impl /*struct*/ QToolBar {
  pub fn visibilityChanged<RetType, T: QToolBar_visibilityChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.visibilityChanged(self);
    // return 1;
  }
}

pub trait QToolBar_visibilityChanged<RetType> {
  fn visibilityChanged(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::visibilityChanged(bool visible);
impl<'a> /*trait*/ QToolBar_visibilityChanged<()> for (i8) {
  fn visibilityChanged(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar17visibilityChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QToolBar17visibilityChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QToolBar::clear();
impl /*struct*/ QToolBar {
  pub fn clear<RetType, T: QToolBar_clear<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QToolBar_clear<RetType> {
  fn clear(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::clear();
impl<'a> /*trait*/ QToolBar_clear<()> for () {
  fn clear(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar5clearEv()};
     unsafe {_ZN8QToolBar5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn NewQToolBar<T: QToolBar_NewQToolBar>(value: T) -> QToolBar {
    let rsthis = value.NewQToolBar();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBar_NewQToolBar {
  fn NewQToolBar(self) -> QToolBar;
}

// proto: void QToolBar::NewQToolBar(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QToolBar_NewQToolBar for (&'a  QString, &'a mut QWidget) {
  fn NewQToolBar(self) -> QToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBarC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QToolBar::setMovable(bool movable);
impl /*struct*/ QToolBar {
  pub fn setMovable<RetType, T: QToolBar_setMovable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMovable(self);
    // return 1;
  }
}

pub trait QToolBar_setMovable<RetType> {
  fn setMovable(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::setMovable(bool movable);
impl<'a> /*trait*/ QToolBar_setMovable<()> for (i8) {
  fn setMovable(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar10setMovableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QToolBar10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QToolBar::isMovable();
impl /*struct*/ QToolBar {
  pub fn isMovable<RetType, T: QToolBar_isMovable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isMovable(self);
    // return 1;
  }
}

pub trait QToolBar_isMovable<RetType> {
  fn isMovable(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  bool QToolBar::isMovable();
impl<'a> /*trait*/ QToolBar_isMovable<i8> for () {
  fn isMovable(self , rsthis: &mut QToolBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar9isMovableEv()};
    let mut ret = unsafe {_ZNK8QToolBar9isMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QToolBar::setIconSize(const QSize & iconSize);
impl /*struct*/ QToolBar {
  pub fn setIconSize<RetType, T: QToolBar_setIconSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QToolBar_setIconSize<RetType> {
  fn setIconSize(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QToolBar_setIconSize<()> for (&'a  QSize) {
  fn setIconSize(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBar11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QAction * QToolBar::addSeparator();
impl /*struct*/ QToolBar {
  pub fn addSeparator<RetType, T: QToolBar_addSeparator<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addSeparator(self);
    // return 1;
  }
}

pub trait QToolBar_addSeparator<RetType> {
  fn addSeparator(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::addSeparator();
impl<'a> /*trait*/ QToolBar_addSeparator<QAction> for () {
  fn addSeparator(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12addSeparatorEv()};
    let mut ret = unsafe {_ZN8QToolBar12addSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QToolBar::setFloatable(bool floatable);
impl /*struct*/ QToolBar {
  pub fn setFloatable<RetType, T: QToolBar_setFloatable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFloatable(self);
    // return 1;
  }
}

pub trait QToolBar_setFloatable<RetType> {
  fn setFloatable(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::setFloatable(bool floatable);
impl<'a> /*trait*/ QToolBar_setFloatable<()> for (i8) {
  fn setFloatable(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12setFloatableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QToolBar12setFloatableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QAction * QToolBar::addAction(const QString & text);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a  QString) {
  fn addAction(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QToolBar::topLevelChanged(bool topLevel);
impl /*struct*/ QToolBar {
  pub fn topLevelChanged<RetType, T: QToolBar_topLevelChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.topLevelChanged(self);
    // return 1;
  }
}

pub trait QToolBar_topLevelChanged<RetType> {
  fn topLevelChanged(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::topLevelChanged(bool topLevel);
impl<'a> /*trait*/ QToolBar_topLevelChanged<()> for (i8) {
  fn topLevelChanged(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15topLevelChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QToolBar15topLevelChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QToolBar::actionTriggered(QAction * action);
impl /*struct*/ QToolBar {
  pub fn actionTriggered<RetType, T: QToolBar_actionTriggered<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.actionTriggered(self);
    // return 1;
  }
}

pub trait QToolBar_actionTriggered<RetType> {
  fn actionTriggered(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::actionTriggered(QAction * action);
impl<'a> /*trait*/ QToolBar_actionTriggered<()> for (&'a mut QAction) {
  fn actionTriggered(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15actionTriggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBar15actionTriggeredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a  QIcon, &'a  QString) {
  fn addAction(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QToolBar::NewQToolBar(QWidget * parent);
impl<'a> /*trait*/ QToolBar_NewQToolBar for (&'a mut QWidget) {
  fn NewQToolBar(self) -> QToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QAction * QToolBar::actionAt(const QPoint & p);
impl /*struct*/ QToolBar {
  pub fn actionAt<RetType, T: QToolBar_actionAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.actionAt(self);
    // return 1;
  }
}

pub trait QToolBar_actionAt<RetType> {
  fn actionAt(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::actionAt(const QPoint & p);
impl<'a> /*trait*/ QToolBar_actionAt<QAction> for (&'a  QPoint) {
  fn actionAt(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBar8actionAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QToolBar::movableChanged(bool movable);
impl /*struct*/ QToolBar {
  pub fn movableChanged<RetType, T: QToolBar_movableChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.movableChanged(self);
    // return 1;
  }
}

pub trait QToolBar_movableChanged<RetType> {
  fn movableChanged(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::movableChanged(bool movable);
impl<'a> /*trait*/ QToolBar_movableChanged<()> for (i8) {
  fn movableChanged(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar14movableChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QToolBar14movableChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QAction * QToolBar::actionAt(int x, int y);
impl<'a> /*trait*/ QToolBar_actionAt<QAction> for (i32, i32) {
  fn actionAt(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8actionAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK8QToolBar8actionAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QToolBar::iconSizeChanged(const QSize & iconSize);
impl /*struct*/ QToolBar {
  pub fn iconSizeChanged<RetType, T: QToolBar_iconSizeChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.iconSizeChanged(self);
    // return 1;
  }
}

pub trait QToolBar_iconSizeChanged<RetType> {
  fn iconSizeChanged(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QToolBar_iconSizeChanged<()> for (&'a  QSize) {
  fn iconSizeChanged(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBar15iconSizeChangedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QToolBar::isFloating();
impl /*struct*/ QToolBar {
  pub fn isFloating<RetType, T: QToolBar_isFloating<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isFloating(self);
    // return 1;
  }
}

pub trait QToolBar_isFloating<RetType> {
  fn isFloating(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  bool QToolBar::isFloating();
impl<'a> /*trait*/ QToolBar_isFloating<i8> for () {
  fn isFloating(self , rsthis: &mut QToolBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10isFloatingEv()};
    let mut ret = unsafe {_ZNK8QToolBar10isFloatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QAction * QToolBar::toggleViewAction();
impl /*struct*/ QToolBar {
  pub fn toggleViewAction<RetType, T: QToolBar_toggleViewAction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toggleViewAction(self);
    // return 1;
  }
}

pub trait QToolBar_toggleViewAction<RetType> {
  fn toggleViewAction(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::toggleViewAction();
impl<'a> /*trait*/ QToolBar_toggleViewAction<QAction> for () {
  fn toggleViewAction(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar16toggleViewActionEv()};
    let mut ret = unsafe {_ZNK8QToolBar16toggleViewActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QToolBar::NewQToolBar(const QToolBar & );
impl<'a> /*trait*/ QToolBar_NewQToolBar for (&'a  QToolBar) {
  fn NewQToolBar(self) -> QToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBarC1ERKS_(qthis, arg0)};
    let rsthis = QToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QToolBar::FreeQToolBar();
impl /*struct*/ QToolBar {
  pub fn FreeQToolBar<RetType, T: QToolBar_FreeQToolBar<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQToolBar(self);
    // return 1;
  }
}

pub trait QToolBar_FreeQToolBar<RetType> {
  fn FreeQToolBar(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  void QToolBar::FreeQToolBar();
impl<'a> /*trait*/ QToolBar_FreeQToolBar<()> for () {
  fn FreeQToolBar(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarD0Ev()};
     unsafe {_ZN8QToolBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QToolBar_addAction<QAction> for (&'a  QString, &'a  QObject, &'a  String) {
  fn addAction(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
impl /*struct*/ QToolBar {
  pub fn insertWidget<RetType, T: QToolBar_insertWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QToolBar_insertWidget<RetType> {
  fn insertWidget(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
impl<'a> /*trait*/ QToolBar_insertWidget<QAction> for (&'a mut QAction, &'a mut QWidget) {
  fn insertWidget(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12insertWidgetEP7QActionP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar12insertWidgetEP7QActionP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAction * QToolBar::addWidget(QWidget * widget);
impl /*struct*/ QToolBar {
  pub fn addWidget<RetType, T: QToolBar_addWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QToolBar_addWidget<RetType> {
  fn addWidget(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::addWidget(QWidget * widget);
impl<'a> /*trait*/ QToolBar_addWidget<QAction> for (&'a mut QWidget) {
  fn addWidget(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QMetaObject * QToolBar::metaObject();
impl /*struct*/ QToolBar {
  pub fn metaObject<RetType, T: QToolBar_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QToolBar_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  const QMetaObject * QToolBar::metaObject();
impl<'a> /*trait*/ QToolBar_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QToolBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10metaObjectEv()};
     unsafe {_ZNK8QToolBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QAction * QToolBar::insertSeparator(QAction * before);
impl /*struct*/ QToolBar {
  pub fn insertSeparator<RetType, T: QToolBar_insertSeparator<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertSeparator(self);
    // return 1;
  }
}

pub trait QToolBar_insertSeparator<RetType> {
  fn insertSeparator(self , rsthis: &mut QToolBar) -> RetType;
}

// proto:  QAction * QToolBar::insertSeparator(QAction * before);
impl<'a> /*trait*/ QToolBar_insertSeparator<QAction> for (&'a mut QAction) {
  fn insertSeparator(self , rsthis: &mut QToolBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBar15insertSeparatorEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

