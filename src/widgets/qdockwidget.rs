// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qdockwidget.h
// dst-file: /src/widgets/qdockwidget.rs
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
use super::super::core::qstring::QString; // 771
use super::qaction::QAction; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QWidget * QDockWidget::widget();
  fn _ZNK11QDockWidget6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDockWidget::setFloating(bool floating);
  fn _ZN11QDockWidget11setFloatingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QWidget * QDockWidget::titleBarWidget();
  fn _ZNK11QDockWidget14titleBarWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDockWidget::topLevelChanged(bool topLevel);
  fn _ZN11QDockWidget15topLevelChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QDockWidget::~QDockWidget();
  fn _ZN11QDockWidgetD0Ev(qthis: *mut c_void);
  // proto:  void QDockWidget::setWidget(QWidget * widget);
  fn _ZN11QDockWidget9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QDockWidget::isFloating();
  fn _ZNK11QDockWidget10isFloatingEv(qthis: *mut c_void) -> c_char;
  // proto:  QAction * QDockWidget::toggleViewAction();
  fn _ZNK11QDockWidget16toggleViewActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDockWidget::QDockWidget(const QDockWidget & );
  fn _ZN11QDockWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
  fn _ZN11QDockWidget17setTitleBarWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QDockWidget::metaObject();
  fn _ZNK11QDockWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDockWidget::visibilityChanged(bool visible);
  fn _ZN11QDockWidget17visibilityChangedEb(qthis: *mut c_void, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QDockWidget)=1
pub struct QDockWidget {
  pub qclsinst: *mut c_void,
}

  // proto:  QWidget * QDockWidget::widget();
impl /*struct*/ QDockWidget {
  pub fn widget<RetType, T: QDockWidget_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QDockWidget_widget<RetType> {
  fn widget(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  QWidget * QDockWidget::widget();
impl<'a> /*trait*/ QDockWidget_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QDockWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget6widgetEv()};
    let mut ret = unsafe {_ZNK11QDockWidget6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::setFloating(bool floating);
impl /*struct*/ QDockWidget {
  pub fn setFloating<RetType, T: QDockWidget_setFloating<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFloating(self);
    // return 1;
  }
}

pub trait QDockWidget_setFloating<RetType> {
  fn setFloating(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::setFloating(bool floating);
impl<'a> /*trait*/ QDockWidget_setFloating<()> for (i8) {
  fn setFloating(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget11setFloatingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QDockWidget11setFloatingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QDockWidget::titleBarWidget();
impl /*struct*/ QDockWidget {
  pub fn titleBarWidget<RetType, T: QDockWidget_titleBarWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.titleBarWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_titleBarWidget<RetType> {
  fn titleBarWidget(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  QWidget * QDockWidget::titleBarWidget();
impl<'a> /*trait*/ QDockWidget_titleBarWidget<QWidget> for () {
  fn titleBarWidget(self , rsthis: &mut QDockWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget14titleBarWidgetEv()};
    let mut ret = unsafe {_ZNK11QDockWidget14titleBarWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::topLevelChanged(bool topLevel);
impl /*struct*/ QDockWidget {
  pub fn topLevelChanged<RetType, T: QDockWidget_topLevelChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLevelChanged(self);
    // return 1;
  }
}

pub trait QDockWidget_topLevelChanged<RetType> {
  fn topLevelChanged(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::topLevelChanged(bool topLevel);
impl<'a> /*trait*/ QDockWidget_topLevelChanged<()> for (i8) {
  fn topLevelChanged(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget15topLevelChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QDockWidget15topLevelChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDockWidget::~QDockWidget();
impl /*struct*/ QDockWidget {
  pub fn FreeQDockWidget<RetType, T: QDockWidget_FreeQDockWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDockWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_FreeQDockWidget<RetType> {
  fn FreeQDockWidget(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::~QDockWidget();
impl<'a> /*trait*/ QDockWidget_FreeQDockWidget<()> for () {
  fn FreeQDockWidget(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetD0Ev()};
     unsafe {_ZN11QDockWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDockWidget::setWidget(QWidget * widget);
impl /*struct*/ QDockWidget {
  pub fn setWidget<RetType, T: QDockWidget_setWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_setWidget<RetType> {
  fn setWidget(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::setWidget(QWidget * widget);
impl<'a> /*trait*/ QDockWidget_setWidget<()> for (QWidget) {
  fn setWidget(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QDockWidget9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QDockWidget::isFloating();
impl /*struct*/ QDockWidget {
  pub fn isFloating<RetType, T: QDockWidget_isFloating<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isFloating(self);
    // return 1;
  }
}

pub trait QDockWidget_isFloating<RetType> {
  fn isFloating(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  bool QDockWidget::isFloating();
impl<'a> /*trait*/ QDockWidget_isFloating<i8> for () {
  fn isFloating(self , rsthis: &mut QDockWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10isFloatingEv()};
    let mut ret = unsafe {_ZNK11QDockWidget10isFloatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAction * QDockWidget::toggleViewAction();
impl /*struct*/ QDockWidget {
  pub fn toggleViewAction<RetType, T: QDockWidget_toggleViewAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toggleViewAction(self);
    // return 1;
  }
}

pub trait QDockWidget_toggleViewAction<RetType> {
  fn toggleViewAction(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  QAction * QDockWidget::toggleViewAction();
impl<'a> /*trait*/ QDockWidget_toggleViewAction<QAction> for () {
  fn toggleViewAction(self , rsthis: &mut QDockWidget) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget16toggleViewActionEv()};
    let mut ret = unsafe {_ZNK11QDockWidget16toggleViewActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::QDockWidget(const QDockWidget & );
impl /*struct*/ QDockWidget {
  pub fn NewQDockWidget<T: QDockWidget_NewQDockWidget>(value: T) -> QDockWidget {
    let rsthis = value.NewQDockWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QDockWidget_NewQDockWidget {
  fn NewQDockWidget(self) -> QDockWidget;
}

  // proto:  void QDockWidget::QDockWidget(const QDockWidget & );
impl<'a> /*trait*/ QDockWidget_NewQDockWidget for (QDockWidget) {
  fn NewQDockWidget(self) -> QDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QDockWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
impl /*struct*/ QDockWidget {
  pub fn setTitleBarWidget<RetType, T: QDockWidget_setTitleBarWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTitleBarWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_setTitleBarWidget<RetType> {
  fn setTitleBarWidget(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
impl<'a> /*trait*/ QDockWidget_setTitleBarWidget<()> for (QWidget) {
  fn setTitleBarWidget(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget17setTitleBarWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QDockWidget17setTitleBarWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDockWidget::metaObject();
impl /*struct*/ QDockWidget {
  pub fn metaObject<RetType, T: QDockWidget_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDockWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  const QMetaObject * QDockWidget::metaObject();
impl<'a> /*trait*/ QDockWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10metaObjectEv()};
     unsafe {_ZNK11QDockWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDockWidget::visibilityChanged(bool visible);
impl /*struct*/ QDockWidget {
  pub fn visibilityChanged<RetType, T: QDockWidget_visibilityChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.visibilityChanged(self);
    // return 1;
  }
}

pub trait QDockWidget_visibilityChanged<RetType> {
  fn visibilityChanged(self , rsthis: &mut QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::visibilityChanged(bool visible);
impl<'a> /*trait*/ QDockWidget_visibilityChanged<()> for (i8) {
  fn visibilityChanged(self , rsthis: &mut QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget17visibilityChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QDockWidget17visibilityChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

