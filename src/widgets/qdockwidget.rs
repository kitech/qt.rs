// auto generated, do not modify.
// created: Sun Dec 27 22:52:03 2015
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
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qaction::QAction; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDockWidget_Class_Size() -> c_int;
  // proto:  QWidget * QDockWidget::widget();
  fn _ZNK11QDockWidget6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDockWidget::setFloating(bool floating);
  fn _ZN11QDockWidget11setFloatingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QWidget * QDockWidget::titleBarWidget();
  fn _ZNK11QDockWidget14titleBarWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDockWidget::topLevelChanged(bool topLevel);
  fn _ZN11QDockWidget15topLevelChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QDockWidget::~QDockWidget();
  fn _ZN11QDockWidgetD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDockWidget::setWidget(QWidget * widget);
  fn _ZN11QDockWidget9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QDockWidget::isFloating();
  fn demth_ZNK11QDockWidget10isFloatingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAction * QDockWidget::toggleViewAction();
  fn _ZNK11QDockWidget16toggleViewActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDockWidget::QDockWidget(const QDockWidget & );
  fn dector_ZN11QDockWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QDockWidgetC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
  fn _ZN11QDockWidget17setTitleBarWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QDockWidget::metaObject();
  fn _ZNK11QDockWidget10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDockWidget::visibilityChanged(bool visible);
  fn _ZN11QDockWidget17visibilityChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget19dockLocationChangedEN2Qt14DockWidgetAreaE(qthis: *mut c_void, fptr: *mut c_void);
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget15featuresChangedE6QFlagsINS_17DockWidgetFeatureEE(qthis: *mut c_void, fptr: *mut c_void);
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget17visibilityChangedEb(qthis: *mut c_void, fptr: *mut c_void);
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget15topLevelChangedEb(qthis: *mut c_void, fptr: *mut c_void);
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget19allowedAreasChangedE6QFlagsIN2Qt14DockWidgetAreaEE(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDockWidget)=1
#[derive(Default)]
pub struct QDockWidget {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _featuresChanged_1: QDockWidget_featuresChanged_signal,
  pub _visibilityChanged_1: QDockWidget_visibilityChanged_signal,
  pub _topLevelChanged_1: QDockWidget_topLevelChanged_signal,
  pub _allowedAreasChanged_1: QDockWidget_allowedAreasChanged_signal,
  pub _dockLocationChanged_1: QDockWidget_dockLocationChanged_signal,
}

impl /*struct*/ QDockWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDockWidget {
    return QDockWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDockWidget {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QDockWidget {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QWidget * QDockWidget::widget();
impl /*struct*/ QDockWidget {
  pub fn widget<RetType, T: QDockWidget_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QDockWidget_widget<RetType> {
  fn widget(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  QWidget * QDockWidget::widget();
impl<'a> /*trait*/ QDockWidget_widget<QWidget> for () {
  fn widget(self , rsthis: & QDockWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget6widgetEv()};
    let mut ret = unsafe {_ZNK11QDockWidget6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::setFloating(bool floating);
impl /*struct*/ QDockWidget {
  pub fn setFloating<RetType, T: QDockWidget_setFloating<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFloating(self);
    // return 1;
  }
}

pub trait QDockWidget_setFloating<RetType> {
  fn setFloating(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::setFloating(bool floating);
impl<'a> /*trait*/ QDockWidget_setFloating<()> for (i8) {
  fn setFloating(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget11setFloatingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QDockWidget11setFloatingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QDockWidget::titleBarWidget();
impl /*struct*/ QDockWidget {
  pub fn titleBarWidget<RetType, T: QDockWidget_titleBarWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.titleBarWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_titleBarWidget<RetType> {
  fn titleBarWidget(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  QWidget * QDockWidget::titleBarWidget();
impl<'a> /*trait*/ QDockWidget_titleBarWidget<QWidget> for () {
  fn titleBarWidget(self , rsthis: & QDockWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget14titleBarWidgetEv()};
    let mut ret = unsafe {_ZNK11QDockWidget14titleBarWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::topLevelChanged(bool topLevel);
impl /*struct*/ QDockWidget {
  pub fn topLevelChanged<RetType, T: QDockWidget_topLevelChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelChanged(self);
    // return 1;
  }
}

pub trait QDockWidget_topLevelChanged<RetType> {
  fn topLevelChanged(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::topLevelChanged(bool topLevel);
impl<'a> /*trait*/ QDockWidget_topLevelChanged<()> for (i8) {
  fn topLevelChanged(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget15topLevelChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QDockWidget15topLevelChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDockWidget::~QDockWidget();
impl /*struct*/ QDockWidget {
  pub fn Free<RetType, T: QDockWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDockWidget_Free<RetType> {
  fn Free(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::~QDockWidget();
impl<'a> /*trait*/ QDockWidget_Free<()> for () {
  fn Free(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetD0Ev()};
     unsafe {_ZN11QDockWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDockWidget::setWidget(QWidget * widget);
impl /*struct*/ QDockWidget {
  pub fn setWidget<RetType, T: QDockWidget_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_setWidget<RetType> {
  fn setWidget(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::setWidget(QWidget * widget);
impl<'a> /*trait*/ QDockWidget_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QDockWidget9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QDockWidget::isFloating();
impl /*struct*/ QDockWidget {
  pub fn isFloating<RetType, T: QDockWidget_isFloating<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFloating(self);
    // return 1;
  }
}

pub trait QDockWidget_isFloating<RetType> {
  fn isFloating(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  bool QDockWidget::isFloating();
impl<'a> /*trait*/ QDockWidget_isFloating<i8> for () {
  fn isFloating(self , rsthis: & QDockWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10isFloatingEv()};
    let mut ret = unsafe {demth_ZNK11QDockWidget10isFloatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAction * QDockWidget::toggleViewAction();
impl /*struct*/ QDockWidget {
  pub fn toggleViewAction<RetType, T: QDockWidget_toggleViewAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggleViewAction(self);
    // return 1;
  }
}

pub trait QDockWidget_toggleViewAction<RetType> {
  fn toggleViewAction(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  QAction * QDockWidget::toggleViewAction();
impl<'a> /*trait*/ QDockWidget_toggleViewAction<QAction> for () {
  fn toggleViewAction(self , rsthis: & QDockWidget) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget16toggleViewActionEv()};
    let mut ret = unsafe {_ZNK11QDockWidget16toggleViewActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::QDockWidget(const QDockWidget & );
impl /*struct*/ QDockWidget {
  pub fn New<T: QDockWidget_New>(value: T) -> QDockWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDockWidget_New {
  fn New(self) -> QDockWidget;
}

  // proto:  void QDockWidget::QDockWidget(const QDockWidget & );
impl<'a> /*trait*/ QDockWidget_New for (&'a QDockWidget) {
  fn New(self) -> QDockWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QDockWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QDockWidgetC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QDockWidgetC1ERKS_(arg0)} as u64;
    let rsthis = QDockWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
impl /*struct*/ QDockWidget {
  pub fn setTitleBarWidget<RetType, T: QDockWidget_setTitleBarWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitleBarWidget(self);
    // return 1;
  }
}

pub trait QDockWidget_setTitleBarWidget<RetType> {
  fn setTitleBarWidget(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
impl<'a> /*trait*/ QDockWidget_setTitleBarWidget<()> for (&'a QWidget) {
  fn setTitleBarWidget(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget17setTitleBarWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QDockWidget17setTitleBarWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDockWidget::metaObject();
impl /*struct*/ QDockWidget {
  pub fn metaObject<RetType, T: QDockWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDockWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  const QMetaObject * QDockWidget::metaObject();
impl<'a> /*trait*/ QDockWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10metaObjectEv()};
     unsafe {_ZNK11QDockWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDockWidget::visibilityChanged(bool visible);
impl /*struct*/ QDockWidget {
  pub fn visibilityChanged<RetType, T: QDockWidget_visibilityChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visibilityChanged(self);
    // return 1;
  }
}

pub trait QDockWidget_visibilityChanged<RetType> {
  fn visibilityChanged(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::visibilityChanged(bool visible);
impl<'a> /*trait*/ QDockWidget_visibilityChanged<()> for (i8) {
  fn visibilityChanged(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidget17visibilityChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QDockWidget17visibilityChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QDockWidget_featuresChanged
pub struct QDockWidget_featuresChanged_signal{poi:u64}
impl /* struct */ QDockWidget {
  pub fn featuresChanged_1(self) -> QDockWidget_featuresChanged_signal {
     return QDockWidget_featuresChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDockWidget_featuresChanged_signal {
  pub fn connect<T: QDockWidget_featuresChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDockWidget_featuresChanged_signal_connect {
  fn connect(self, sigthis: QDockWidget_featuresChanged_signal);
}

#[derive(Default)] // for QDockWidget_visibilityChanged
pub struct QDockWidget_visibilityChanged_signal{poi:u64}
impl /* struct */ QDockWidget {
  pub fn visibilityChanged_1(self) -> QDockWidget_visibilityChanged_signal {
     return QDockWidget_visibilityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDockWidget_visibilityChanged_signal {
  pub fn connect<T: QDockWidget_visibilityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDockWidget_visibilityChanged_signal_connect {
  fn connect(self, sigthis: QDockWidget_visibilityChanged_signal);
}

#[derive(Default)] // for QDockWidget_topLevelChanged
pub struct QDockWidget_topLevelChanged_signal{poi:u64}
impl /* struct */ QDockWidget {
  pub fn topLevelChanged_1(self) -> QDockWidget_topLevelChanged_signal {
     return QDockWidget_topLevelChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDockWidget_topLevelChanged_signal {
  pub fn connect<T: QDockWidget_topLevelChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDockWidget_topLevelChanged_signal_connect {
  fn connect(self, sigthis: QDockWidget_topLevelChanged_signal);
}

#[derive(Default)] // for QDockWidget_allowedAreasChanged
pub struct QDockWidget_allowedAreasChanged_signal{poi:u64}
impl /* struct */ QDockWidget {
  pub fn allowedAreasChanged_1(self) -> QDockWidget_allowedAreasChanged_signal {
     return QDockWidget_allowedAreasChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDockWidget_allowedAreasChanged_signal {
  pub fn connect<T: QDockWidget_allowedAreasChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDockWidget_allowedAreasChanged_signal_connect {
  fn connect(self, sigthis: QDockWidget_allowedAreasChanged_signal);
}

#[derive(Default)] // for QDockWidget_dockLocationChanged
pub struct QDockWidget_dockLocationChanged_signal{poi:u64}
impl /* struct */ QDockWidget {
  pub fn dockLocationChanged_1(self) -> QDockWidget_dockLocationChanged_signal {
     return QDockWidget_dockLocationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDockWidget_dockLocationChanged_signal {
  pub fn connect<T: QDockWidget_dockLocationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDockWidget_dockLocationChanged_signal_connect {
  fn connect(self, sigthis: QDockWidget_dockLocationChanged_signal);
}

// dockLocationChanged(Qt::DockWidgetArea)
extern fn QDockWidget_dockLocationChanged_signal_connect_cb_0(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QDockWidget_dockLocationChanged_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QDockWidget_dockLocationChanged_signal) {
    // do smth...
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget19dockLocationChangedEN2Qt14DockWidgetAreaE(sigthis.poi as *mut c_void, QDockWidget_dockLocationChanged_signal_connect_cb_0 as *mut c_void)};
  }
}
// featuresChanged(class QDockWidget::DockWidgetFeatures)
extern fn QDockWidget_featuresChanged_signal_connect_cb_1(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QDockWidget_featuresChanged_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QDockWidget_featuresChanged_signal) {
    // do smth...
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget15featuresChangedE6QFlagsINS_17DockWidgetFeatureEE(sigthis.poi as *mut c_void, QDockWidget_featuresChanged_signal_connect_cb_1 as *mut c_void)};
  }
}
// visibilityChanged(_Bool)
extern fn QDockWidget_visibilityChanged_signal_connect_cb_2(arg0: c_char) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QDockWidget_visibilityChanged_signal_connect for (extern fn(i8)) {
  fn connect(self, sigthis: QDockWidget_visibilityChanged_signal) {
    // do smth...
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget17visibilityChangedEb(sigthis.poi as *mut c_void, QDockWidget_visibilityChanged_signal_connect_cb_2 as *mut c_void)};
  }
}
// topLevelChanged(_Bool)
extern fn QDockWidget_topLevelChanged_signal_connect_cb_3(arg0: c_char) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QDockWidget_topLevelChanged_signal_connect for (extern fn(i8)) {
  fn connect(self, sigthis: QDockWidget_topLevelChanged_signal) {
    // do smth...
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget15topLevelChangedEb(sigthis.poi as *mut c_void, QDockWidget_topLevelChanged_signal_connect_cb_3 as *mut c_void)};
  }
}
// allowedAreasChanged(Qt::DockWidgetAreas)
extern fn QDockWidget_allowedAreasChanged_signal_connect_cb_4(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QDockWidget_allowedAreasChanged_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QDockWidget_allowedAreasChanged_signal) {
    // do smth...
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget19allowedAreasChangedE6QFlagsIN2Qt14DockWidgetAreaEE(sigthis.poi as *mut c_void, QDockWidget_allowedAreasChanged_signal_connect_cb_4 as *mut c_void)};
  }
}
// <= body block end

