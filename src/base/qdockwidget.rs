// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QWidget * QDockWidget::widget();
  fn _ZNK11QDockWidget6widgetEv() -> i32;
  // proto: void QDockWidget::setFloating(bool floating);
  fn _ZN11QDockWidget11setFloatingEb(arg0: int8_t) -> i32;
  // proto: QWidget * QDockWidget::titleBarWidget();
  fn _ZNK11QDockWidget14titleBarWidgetEv() -> i32;
  // proto: void QDockWidget::topLevelChanged(bool topLevel);
  fn _ZN11QDockWidget15topLevelChangedEb(arg0: int8_t) -> i32;
  // proto: void QDockWidget::FreeQDockWidget();
  fn _ZN11QDockWidgetD0Ev() -> i32;
  // proto: void QDockWidget::setWidget(QWidget * widget);
  fn _ZN11QDockWidget9setWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: bool QDockWidget::isFloating();
  fn _ZNK11QDockWidget10isFloatingEv() -> i32;
  // proto: QAction * QDockWidget::toggleViewAction();
  fn _ZNK11QDockWidget16toggleViewActionEv() -> i32;
  // proto: void QDockWidget::NewQDockWidget(const QDockWidget & );
  fn _ZN11QDockWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QDockWidget::setTitleBarWidget(QWidget * widget);
  fn _ZN11QDockWidget17setTitleBarWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QDockWidget::metaObject();
  fn _ZNK11QDockWidget10metaObjectEv() -> i32;
  // proto: void QDockWidget::visibilityChanged(bool visible);
  fn _ZN11QDockWidget17visibilityChangedEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QDockWidget)=1
pub struct QDockWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDockWidget {
  pub fn widget<T: QDockWidget_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QDockWidget_widget {
  fn widget(self, this: &mut QDockWidget) -> i32;
}

// proto: QWidget * QDockWidget::widget();
impl<'a> /*trait*/ QDockWidget_widget for () {
  fn widget(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget6widgetEv()};
    unsafe {_ZNK11QDockWidget6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn setFloating<T: QDockWidget_setFloating>(&mut self, value: T) -> i32 {
    value.setFloating(self);
    return 1;
  }
}

pub trait QDockWidget_setFloating {
  fn setFloating(self, this: &mut QDockWidget) -> i32;
}

// proto: void QDockWidget::setFloating(bool floating);
impl<'a> /*trait*/ QDockWidget_setFloating for (i8) {
  fn setFloating(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget11setFloatingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QDockWidget11setFloatingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn titleBarWidget<T: QDockWidget_titleBarWidget>(&mut self, value: T) -> i32 {
    value.titleBarWidget(self);
    return 1;
  }
}

pub trait QDockWidget_titleBarWidget {
  fn titleBarWidget(self, this: &mut QDockWidget) -> i32;
}

// proto: QWidget * QDockWidget::titleBarWidget();
impl<'a> /*trait*/ QDockWidget_titleBarWidget for () {
  fn titleBarWidget(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget14titleBarWidgetEv()};
    unsafe {_ZNK11QDockWidget14titleBarWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn topLevelChanged<T: QDockWidget_topLevelChanged>(&mut self, value: T) -> i32 {
    value.topLevelChanged(self);
    return 1;
  }
}

pub trait QDockWidget_topLevelChanged {
  fn topLevelChanged(self, this: &mut QDockWidget) -> i32;
}

// proto: void QDockWidget::topLevelChanged(bool topLevel);
impl<'a> /*trait*/ QDockWidget_topLevelChanged for (i8) {
  fn topLevelChanged(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget15topLevelChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QDockWidget15topLevelChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn FreeQDockWidget<T: QDockWidget_FreeQDockWidget>(&mut self, value: T) -> i32 {
    value.FreeQDockWidget(self);
    return 1;
  }
}

pub trait QDockWidget_FreeQDockWidget {
  fn FreeQDockWidget(self, this: &mut QDockWidget) -> i32;
}

// proto: void QDockWidget::FreeQDockWidget();
impl<'a> /*trait*/ QDockWidget_FreeQDockWidget for () {
  fn FreeQDockWidget(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetD0Ev()};
    unsafe {_ZN11QDockWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn setWidget<T: QDockWidget_setWidget>(&mut self, value: T) -> i32 {
    value.setWidget(self);
    return 1;
  }
}

pub trait QDockWidget_setWidget {
  fn setWidget(self, this: &mut QDockWidget) -> i32;
}

// proto: void QDockWidget::setWidget(QWidget * widget);
impl<'a> /*trait*/ QDockWidget_setWidget for (&'a mut QWidget) {
  fn setWidget(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QDockWidget9setWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn isFloating<T: QDockWidget_isFloating>(&mut self, value: T) -> i32 {
    value.isFloating(self);
    return 1;
  }
}

pub trait QDockWidget_isFloating {
  fn isFloating(self, this: &mut QDockWidget) -> i32;
}

// proto: bool QDockWidget::isFloating();
impl<'a> /*trait*/ QDockWidget_isFloating for () {
  fn isFloating(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10isFloatingEv()};
    unsafe {_ZNK11QDockWidget10isFloatingEv()};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn toggleViewAction<T: QDockWidget_toggleViewAction>(&mut self, value: T) -> i32 {
    value.toggleViewAction(self);
    return 1;
  }
}

pub trait QDockWidget_toggleViewAction {
  fn toggleViewAction(self, this: &mut QDockWidget) -> i32;
}

// proto: QAction * QDockWidget::toggleViewAction();
impl<'a> /*trait*/ QDockWidget_toggleViewAction for () {
  fn toggleViewAction(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget16toggleViewActionEv()};
    unsafe {_ZNK11QDockWidget16toggleViewActionEv()};
    return 1;
  }
}

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

// proto: void QDockWidget::NewQDockWidget(const QDockWidget & );
impl<'a> /*trait*/ QDockWidget_NewQDockWidget for (&'a  QDockWidget) {
  fn NewQDockWidget(self) -> QDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QDockWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn setTitleBarWidget<T: QDockWidget_setTitleBarWidget>(&mut self, value: T) -> i32 {
    value.setTitleBarWidget(self);
    return 1;
  }
}

pub trait QDockWidget_setTitleBarWidget {
  fn setTitleBarWidget(self, this: &mut QDockWidget) -> i32;
}

// proto: void QDockWidget::setTitleBarWidget(QWidget * widget);
impl<'a> /*trait*/ QDockWidget_setTitleBarWidget for (&'a mut QWidget) {
  fn setTitleBarWidget(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget17setTitleBarWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QDockWidget17setTitleBarWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn metaObject<T: QDockWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDockWidget_metaObject {
  fn metaObject(self, this: &mut QDockWidget) -> i32;
}

// proto: const QMetaObject * QDockWidget::metaObject();
impl<'a> /*trait*/ QDockWidget_metaObject for () {
  fn metaObject(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10metaObjectEv()};
    unsafe {_ZNK11QDockWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDockWidget {
  pub fn visibilityChanged<T: QDockWidget_visibilityChanged>(&mut self, value: T) -> i32 {
    value.visibilityChanged(self);
    return 1;
  }
}

pub trait QDockWidget_visibilityChanged {
  fn visibilityChanged(self, this: &mut QDockWidget) -> i32;
}

// proto: void QDockWidget::visibilityChanged(bool visible);
impl<'a> /*trait*/ QDockWidget_visibilityChanged for (i8) {
  fn visibilityChanged(self, this: &mut QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget17visibilityChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QDockWidget17visibilityChangedEb(arg0)};
    return 1;
  }
}

