// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::qaction::*; // 773
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDockWidget_Class_Size() -> c_int;
  // proto:  QWidget * QDockWidget::widget();
  fn C_ZNK11QDockWidget6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDockWidget::setFloating(bool floating);
  fn C_ZN11QDockWidget11setFloatingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QWidget * QDockWidget::titleBarWidget();
  fn C_ZNK11QDockWidget14titleBarWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDockWidget::~QDockWidget();
  fn C_ZN11QDockWidgetD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDockWidget::setWidget(QWidget * widget);
  fn C_ZN11QDockWidget9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QDockWidget::isFloating();
  fn C_ZNK11QDockWidget10isFloatingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAction * QDockWidget::toggleViewAction();
  fn C_ZNK11QDockWidget16toggleViewActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDockWidget::setTitleBarWidget(QWidget * widget);
  fn C_ZN11QDockWidget17setTitleBarWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QDockWidget::metaObject();
  fn C_ZNK11QDockWidget10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget17visibilityChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDockWidget_SlotProxy_connect__ZN11QDockWidget15topLevelChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDockWidget)=1
#[derive(Default)]
pub struct QDockWidget {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _featuresChanged: QDockWidget_featuresChanged_signal,
  pub _visibilityChanged: QDockWidget_visibilityChanged_signal,
  pub _topLevelChanged: QDockWidget_topLevelChanged_signal,
  pub _allowedAreasChanged: QDockWidget_allowedAreasChanged_signal,
  pub _dockLocationChanged: QDockWidget_dockLocationChanged_signal,
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
    let mut ret = unsafe {C_ZNK11QDockWidget6widgetEv(rsthis.qclsinst)};
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
     unsafe {C_ZN11QDockWidget11setFloatingEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK11QDockWidget14titleBarWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDockWidget::~QDockWidget();
impl /*struct*/ QDockWidget {
  pub fn free<RetType, T: QDockWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDockWidget_free<RetType> {
  fn free(self , rsthis: & QDockWidget) -> RetType;
}

  // proto:  void QDockWidget::~QDockWidget();
impl<'a> /*trait*/ QDockWidget_free<()> for () {
  fn free(self , rsthis: & QDockWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDockWidgetD2Ev()};
     unsafe {C_ZN11QDockWidgetD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN11QDockWidget9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK11QDockWidget10isFloatingEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK11QDockWidget16toggleViewActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN11QDockWidget17setTitleBarWidgetEP7QWidget(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QDockWidget_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QDockWidget) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDockWidget10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QDockWidget10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QDockWidget_featuresChanged
pub struct QDockWidget_featuresChanged_signal{poi:u64}
impl /* struct */ QDockWidget {
  pub fn featuresChanged(&self) -> QDockWidget_featuresChanged_signal {
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
  pub fn visibilityChanged(&self) -> QDockWidget_visibilityChanged_signal {
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
  pub fn topLevelChanged(&self) -> QDockWidget_topLevelChanged_signal {
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
  pub fn allowedAreasChanged(&self) -> QDockWidget_allowedAreasChanged_signal {
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
  pub fn dockLocationChanged(&self) -> QDockWidget_dockLocationChanged_signal {
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

// visibilityChanged(_Bool)
extern fn QDockWidget_visibilityChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QDockWidget_visibilityChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDockWidget_visibilityChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QDockWidget_visibilityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDockWidget_visibilityChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget17visibilityChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDockWidget_visibilityChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QDockWidget_visibilityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDockWidget_visibilityChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget17visibilityChangedEb(arg0, arg1, arg2)};
  }
}
// topLevelChanged(_Bool)
extern fn QDockWidget_topLevelChanged_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QDockWidget_topLevelChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDockWidget_topLevelChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QDockWidget_topLevelChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDockWidget_topLevelChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget15topLevelChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDockWidget_topLevelChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QDockWidget_topLevelChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDockWidget_topLevelChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDockWidget_SlotProxy_connect__ZN11QDockWidget15topLevelChangedEb(arg0, arg1, arg2)};
  }
}
// <= body block end

