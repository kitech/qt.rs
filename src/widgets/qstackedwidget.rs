// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qstackedwidget.h
// dst-file: /src/widgets/qstackedwidget.rs
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
use super::qframe::QFrame; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStackedWidget_Class_Size() -> c_int;
  // proto:  void QStackedWidget::setCurrentIndex(int index);
  fn _ZN14QStackedWidget15setCurrentIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStackedWidget::QStackedWidget(QWidget * parent);
  fn dector_ZN14QStackedWidgetC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QStackedWidgetC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QStackedWidget::currentWidget();
  fn _ZNK14QStackedWidget13currentWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStackedWidget::widgetRemoved(int index);
  fn _ZN14QStackedWidget13widgetRemovedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QStackedWidget::insertWidget(int index, QWidget * w);
  fn _ZN14QStackedWidget12insertWidgetEiP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_int;
  // proto:  int QStackedWidget::indexOf(QWidget * );
  fn _ZNK14QStackedWidget7indexOfEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QStackedWidget::removeWidget(QWidget * w);
  fn _ZN14QStackedWidget12removeWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QStackedWidget::widget(int );
  fn _ZNK14QStackedWidget6widgetEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QStackedWidget::currentChanged(int );
  fn _ZN14QStackedWidget14currentChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QStackedWidget::addWidget(QWidget * w);
  fn _ZN14QStackedWidget9addWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QStackedWidget::currentIndex();
  fn _ZNK14QStackedWidget12currentIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QStackedWidget::count();
  fn _ZNK14QStackedWidget5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStackedWidget::setCurrentWidget(QWidget * w);
  fn _ZN14QStackedWidget16setCurrentWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStackedWidget::~QStackedWidget();
  fn _ZN14QStackedWidgetD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStackedWidget::QStackedWidget(const QStackedWidget & );
  fn dector_ZN14QStackedWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QStackedWidgetC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QStackedWidget::metaObject();
  fn _ZNK14QStackedWidget10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QStackedWidget_SlotProxy_connect__ZN14QStackedWidget14currentChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QStackedWidget_SlotProxy_connect__ZN14QStackedWidget13widgetRemovedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStackedWidget)=1
#[derive(Default)]
pub struct QStackedWidget {
  qbase: QFrame,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _widgetRemoved_1: QStackedWidget_widgetRemoved_signal,
  pub _currentChanged_1: QStackedWidget_currentChanged_signal,
}

impl /*struct*/ QStackedWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStackedWidget {
    return QStackedWidget{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStackedWidget {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return & self.qbase;
  }
}
impl AsRef<QFrame> for QStackedWidget {
  fn as_ref(& self) -> & QFrame {
    return & self.qbase;
  }
}
  // proto:  void QStackedWidget::setCurrentIndex(int index);
impl /*struct*/ QStackedWidget {
  pub fn setCurrentIndex<RetType, T: QStackedWidget_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QStackedWidget_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QStackedWidget_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedWidget15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStackedWidget::QStackedWidget(QWidget * parent);
impl /*struct*/ QStackedWidget {
  pub fn new<T: QStackedWidget_new>(value: T) -> QStackedWidget {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedWidget_new {
  fn new(self) -> QStackedWidget;
}

  // proto:  void QStackedWidget::QStackedWidget(QWidget * parent);
impl<'a> /*trait*/ QStackedWidget_new for (&'a QWidget) {
  fn new(self) -> QStackedWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetC1EP7QWidget()};
    let ctysz: c_int = unsafe{QStackedWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QStackedWidgetC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QStackedWidgetC1EP7QWidget(arg0)} as u64;
    let rsthis = QStackedWidget{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QStackedWidget::currentWidget();
impl /*struct*/ QStackedWidget {
  pub fn currentWidget<RetType, T: QStackedWidget_currentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_currentWidget<RetType> {
  fn currentWidget(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  QWidget * QStackedWidget::currentWidget();
impl<'a> /*trait*/ QStackedWidget_currentWidget<QWidget> for () {
  fn currentWidget(self , rsthis: & QStackedWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget13currentWidgetEv()};
    let mut ret = unsafe {_ZNK14QStackedWidget13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStackedWidget::widgetRemoved(int index);
impl /*struct*/ QStackedWidget {
  pub fn widgetRemoved<RetType, T: QStackedWidget_widgetRemoved<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widgetRemoved(self);
    // return 1;
  }
}

pub trait QStackedWidget_widgetRemoved<RetType> {
  fn widgetRemoved(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::widgetRemoved(int index);
impl<'a> /*trait*/ QStackedWidget_widgetRemoved<()> for (i32) {
  fn widgetRemoved(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget13widgetRemovedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedWidget13widgetRemovedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStackedWidget::insertWidget(int index, QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn insertWidget<RetType, T: QStackedWidget_insertWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_insertWidget<RetType> {
  fn insertWidget(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::insertWidget(int index, QWidget * w);
impl<'a> /*trait*/ QStackedWidget_insertWidget<i32> for (i32, &'a QWidget) {
  fn insertWidget(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStackedWidget12insertWidgetEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedWidget::indexOf(QWidget * );
impl /*struct*/ QStackedWidget {
  pub fn indexOf<RetType, T: QStackedWidget_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QStackedWidget_indexOf<RetType> {
  fn indexOf(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::indexOf(QWidget * );
impl<'a> /*trait*/ QStackedWidget_indexOf<i32> for (&'a QWidget) {
  fn indexOf(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QStackedWidget7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStackedWidget::removeWidget(QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn removeWidget<RetType, T: QStackedWidget_removeWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_removeWidget<RetType> {
  fn removeWidget(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::removeWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_removeWidget<()> for (&'a QWidget) {
  fn removeWidget(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedWidget12removeWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QStackedWidget::widget(int );
impl /*struct*/ QStackedWidget {
  pub fn widget<RetType, T: QStackedWidget_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QStackedWidget_widget<RetType> {
  fn widget(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  QWidget * QStackedWidget::widget(int );
impl<'a> /*trait*/ QStackedWidget_widget<QWidget> for (i32) {
  fn widget(self , rsthis: & QStackedWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QStackedWidget6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStackedWidget::currentChanged(int );
impl /*struct*/ QStackedWidget {
  pub fn currentChanged<RetType, T: QStackedWidget_currentChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QStackedWidget_currentChanged<RetType> {
  fn currentChanged(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::currentChanged(int );
impl<'a> /*trait*/ QStackedWidget_currentChanged<()> for (i32) {
  fn currentChanged(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedWidget14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStackedWidget::addWidget(QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn addWidget<RetType, T: QStackedWidget_addWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_addWidget<RetType> {
  fn addWidget(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::addWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_addWidget<i32> for (&'a QWidget) {
  fn addWidget(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStackedWidget9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedWidget::currentIndex();
impl /*struct*/ QStackedWidget {
  pub fn currentIndex<RetType, T: QStackedWidget_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QStackedWidget_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::currentIndex();
impl<'a> /*trait*/ QStackedWidget_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget12currentIndexEv()};
    let mut ret = unsafe {_ZNK14QStackedWidget12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedWidget::count();
impl /*struct*/ QStackedWidget {
  pub fn count<RetType, T: QStackedWidget_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QStackedWidget_count<RetType> {
  fn count(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::count();
impl<'a> /*trait*/ QStackedWidget_count<i32> for () {
  fn count(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget5countEv()};
    let mut ret = unsafe {_ZNK14QStackedWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStackedWidget::setCurrentWidget(QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn setCurrentWidget<RetType, T: QStackedWidget_setCurrentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_setCurrentWidget<RetType> {
  fn setCurrentWidget(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::setCurrentWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_setCurrentWidget<()> for (&'a QWidget) {
  fn setCurrentWidget(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedWidget16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStackedWidget::~QStackedWidget();
impl /*struct*/ QStackedWidget {
  pub fn free<RetType, T: QStackedWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStackedWidget_free<RetType> {
  fn free(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::~QStackedWidget();
impl<'a> /*trait*/ QStackedWidget_free<()> for () {
  fn free(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetD0Ev()};
     unsafe {_ZN14QStackedWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStackedWidget::QStackedWidget(const QStackedWidget & );
impl<'a> /*trait*/ QStackedWidget_new for (&'a QStackedWidget) {
  fn new(self) -> QStackedWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QStackedWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QStackedWidgetC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QStackedWidgetC1ERKS_(arg0)} as u64;
    let rsthis = QStackedWidget{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStackedWidget::metaObject();
impl /*struct*/ QStackedWidget {
  pub fn metaObject<RetType, T: QStackedWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStackedWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStackedWidget) -> RetType;
}

  // proto:  const QMetaObject * QStackedWidget::metaObject();
impl<'a> /*trait*/ QStackedWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget10metaObjectEv()};
     unsafe {_ZNK14QStackedWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QStackedWidget_widgetRemoved
pub struct QStackedWidget_widgetRemoved_signal{poi:u64}
impl /* struct */ QStackedWidget {
  pub fn widgetRemoved_1(&self) -> QStackedWidget_widgetRemoved_signal {
     return QStackedWidget_widgetRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStackedWidget_widgetRemoved_signal {
  pub fn connect<T: QStackedWidget_widgetRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStackedWidget_widgetRemoved_signal_connect {
  fn connect(self, sigthis: QStackedWidget_widgetRemoved_signal);
}

#[derive(Default)] // for QStackedWidget_currentChanged
pub struct QStackedWidget_currentChanged_signal{poi:u64}
impl /* struct */ QStackedWidget {
  pub fn currentChanged_1(&self) -> QStackedWidget_currentChanged_signal {
     return QStackedWidget_currentChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStackedWidget_currentChanged_signal {
  pub fn connect<T: QStackedWidget_currentChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStackedWidget_currentChanged_signal_connect {
  fn connect(self, sigthis: QStackedWidget_currentChanged_signal);
}

// currentChanged(int)
extern fn QStackedWidget_currentChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStackedWidget_currentChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStackedWidget_currentChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStackedWidget_currentChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStackedWidget_currentChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStackedWidget_SlotProxy_connect__ZN14QStackedWidget14currentChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStackedWidget_currentChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QStackedWidget_currentChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStackedWidget_currentChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStackedWidget_SlotProxy_connect__ZN14QStackedWidget14currentChangedEi(arg0, arg1, arg2)};
  }
}
// widgetRemoved(int)
extern fn QStackedWidget_widgetRemoved_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStackedWidget_widgetRemoved_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStackedWidget_widgetRemoved_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStackedWidget_widgetRemoved_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStackedWidget_widgetRemoved_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStackedWidget_SlotProxy_connect__ZN14QStackedWidget13widgetRemovedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStackedWidget_widgetRemoved_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QStackedWidget_widgetRemoved_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStackedWidget_widgetRemoved_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStackedWidget_SlotProxy_connect__ZN14QStackedWidget13widgetRemovedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

