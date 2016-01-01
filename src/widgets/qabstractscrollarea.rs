// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qabstractscrollarea.h
// dst-file: /src/widgets/qabstractscrollarea.rs
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
use super::qscrollbar::QScrollBar; // 773
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractScrollArea_Class_Size() -> c_int;
  // proto:  QScrollBar * QAbstractScrollArea::horizontalScrollBar();
  fn _ZNK19QAbstractScrollArea19horizontalScrollBarEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QAbstractScrollArea::maximumViewportSize();
  fn _ZNK19QAbstractScrollArea19maximumViewportSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractScrollArea::QAbstractScrollArea(QWidget * parent);
  fn dector_ZN19QAbstractScrollAreaC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractScrollAreaC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractScrollArea::setViewport(QWidget * widget);
  fn _ZN19QAbstractScrollArea11setViewportEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QAbstractScrollArea::minimumSizeHint();
  fn _ZNK19QAbstractScrollArea15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractScrollArea::QAbstractScrollArea(const QAbstractScrollArea & );
  fn dector_ZN19QAbstractScrollAreaC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractScrollAreaC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractScrollArea::setCornerWidget(QWidget * widget);
  fn _ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractScrollArea::metaObject();
  fn _ZNK19QAbstractScrollArea10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractScrollArea::setupViewport(QWidget * viewport);
  fn _ZN19QAbstractScrollArea13setupViewportEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QAbstractScrollArea::cornerWidget();
  fn _ZNK19QAbstractScrollArea12cornerWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QScrollBar * QAbstractScrollArea::verticalScrollBar();
  fn _ZNK19QAbstractScrollArea17verticalScrollBarEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QAbstractScrollArea::viewport();
  fn _ZNK19QAbstractScrollArea8viewportEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractScrollArea::~QAbstractScrollArea();
  fn _ZN19QAbstractScrollAreaD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QAbstractScrollArea::sizeHint();
  fn _ZNK19QAbstractScrollArea8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar * scrollbar);
  fn _ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractScrollArea::setVerticalScrollBar(QScrollBar * scrollbar);
  fn _ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractScrollArea)=1
#[derive(Default)]
pub struct QAbstractScrollArea {
  qbase: QFrame,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAbstractScrollArea {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractScrollArea {
    return QAbstractScrollArea{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractScrollArea {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return & self.qbase;
  }
}
impl AsRef<QFrame> for QAbstractScrollArea {
  fn as_ref(& self) -> & QFrame {
    return & self.qbase;
  }
}
  // proto:  QScrollBar * QAbstractScrollArea::horizontalScrollBar();
impl /*struct*/ QAbstractScrollArea {
  pub fn horizontalScrollBar<RetType, T: QAbstractScrollArea_horizontalScrollBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_horizontalScrollBar<RetType> {
  fn horizontalScrollBar(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QScrollBar * QAbstractScrollArea::horizontalScrollBar();
impl<'a> /*trait*/ QAbstractScrollArea_horizontalScrollBar<QScrollBar> for () {
  fn horizontalScrollBar(self , rsthis: & QAbstractScrollArea) -> QScrollBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea19horizontalScrollBarEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea19horizontalScrollBarEv(rsthis.qclsinst)};
    let mut ret1 = QScrollBar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QAbstractScrollArea::maximumViewportSize();
impl /*struct*/ QAbstractScrollArea {
  pub fn maximumViewportSize<RetType, T: QAbstractScrollArea_maximumViewportSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumViewportSize(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_maximumViewportSize<RetType> {
  fn maximumViewportSize(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QSize QAbstractScrollArea::maximumViewportSize();
impl<'a> /*trait*/ QAbstractScrollArea_maximumViewportSize<QSize> for () {
  fn maximumViewportSize(self , rsthis: & QAbstractScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea19maximumViewportSizeEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea19maximumViewportSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::QAbstractScrollArea(QWidget * parent);
impl /*struct*/ QAbstractScrollArea {
  pub fn new<T: QAbstractScrollArea_new>(value: T) -> QAbstractScrollArea {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractScrollArea_new {
  fn new(self) -> QAbstractScrollArea;
}

  // proto:  void QAbstractScrollArea::QAbstractScrollArea(QWidget * parent);
impl<'a> /*trait*/ QAbstractScrollArea_new for (&'a QWidget) {
  fn new(self) -> QAbstractScrollArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollAreaC1EP7QWidget()};
    let ctysz: c_int = unsafe{QAbstractScrollArea_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractScrollAreaC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QAbstractScrollAreaC1EP7QWidget(arg0)} as u64;
    let rsthis = QAbstractScrollArea{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setViewport(QWidget * widget);
impl /*struct*/ QAbstractScrollArea {
  pub fn setViewport<RetType, T: QAbstractScrollArea_setViewport<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setViewport(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setViewport<RetType> {
  fn setViewport(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setViewport(QWidget * widget);
impl<'a> /*trait*/ QAbstractScrollArea_setViewport<()> for (&'a QWidget) {
  fn setViewport(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea11setViewportEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea11setViewportEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QAbstractScrollArea::minimumSizeHint();
impl /*struct*/ QAbstractScrollArea {
  pub fn minimumSizeHint<RetType, T: QAbstractScrollArea_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QSize QAbstractScrollArea::minimumSizeHint();
impl<'a> /*trait*/ QAbstractScrollArea_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QAbstractScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::QAbstractScrollArea(const QAbstractScrollArea & );
impl<'a> /*trait*/ QAbstractScrollArea_new for (&'a QAbstractScrollArea) {
  fn new(self) -> QAbstractScrollArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollAreaC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractScrollArea_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractScrollAreaC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QAbstractScrollAreaC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractScrollArea{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setCornerWidget(QWidget * widget);
impl /*struct*/ QAbstractScrollArea {
  pub fn setCornerWidget<RetType, T: QAbstractScrollArea_setCornerWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCornerWidget(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setCornerWidget<RetType> {
  fn setCornerWidget(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setCornerWidget(QWidget * widget);
impl<'a> /*trait*/ QAbstractScrollArea_setCornerWidget<()> for (&'a QWidget) {
  fn setCornerWidget(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractScrollArea::metaObject();
impl /*struct*/ QAbstractScrollArea {
  pub fn metaObject<RetType, T: QAbstractScrollArea_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  const QMetaObject * QAbstractScrollArea::metaObject();
impl<'a> /*trait*/ QAbstractScrollArea_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea10metaObjectEv()};
     unsafe {_ZNK19QAbstractScrollArea10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setupViewport(QWidget * viewport);
impl /*struct*/ QAbstractScrollArea {
  pub fn setupViewport<RetType, T: QAbstractScrollArea_setupViewport<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setupViewport(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setupViewport<RetType> {
  fn setupViewport(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setupViewport(QWidget * viewport);
impl<'a> /*trait*/ QAbstractScrollArea_setupViewport<()> for (&'a QWidget) {
  fn setupViewport(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea13setupViewportEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea13setupViewportEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QAbstractScrollArea::cornerWidget();
impl /*struct*/ QAbstractScrollArea {
  pub fn cornerWidget<RetType, T: QAbstractScrollArea_cornerWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cornerWidget(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_cornerWidget<RetType> {
  fn cornerWidget(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QWidget * QAbstractScrollArea::cornerWidget();
impl<'a> /*trait*/ QAbstractScrollArea_cornerWidget<QWidget> for () {
  fn cornerWidget(self , rsthis: & QAbstractScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea12cornerWidgetEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea12cornerWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QScrollBar * QAbstractScrollArea::verticalScrollBar();
impl /*struct*/ QAbstractScrollArea {
  pub fn verticalScrollBar<RetType, T: QAbstractScrollArea_verticalScrollBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_verticalScrollBar<RetType> {
  fn verticalScrollBar(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QScrollBar * QAbstractScrollArea::verticalScrollBar();
impl<'a> /*trait*/ QAbstractScrollArea_verticalScrollBar<QScrollBar> for () {
  fn verticalScrollBar(self , rsthis: & QAbstractScrollArea) -> QScrollBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea17verticalScrollBarEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea17verticalScrollBarEv(rsthis.qclsinst)};
    let mut ret1 = QScrollBar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QAbstractScrollArea::viewport();
impl /*struct*/ QAbstractScrollArea {
  pub fn viewport<RetType, T: QAbstractScrollArea_viewport<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewport(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_viewport<RetType> {
  fn viewport(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QWidget * QAbstractScrollArea::viewport();
impl<'a> /*trait*/ QAbstractScrollArea_viewport<QWidget> for () {
  fn viewport(self , rsthis: & QAbstractScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea8viewportEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea8viewportEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::~QAbstractScrollArea();
impl /*struct*/ QAbstractScrollArea {
  pub fn free<RetType, T: QAbstractScrollArea_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_free<RetType> {
  fn free(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::~QAbstractScrollArea();
impl<'a> /*trait*/ QAbstractScrollArea_free<()> for () {
  fn free(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollAreaD0Ev()};
     unsafe {_ZN19QAbstractScrollAreaD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QAbstractScrollArea::sizeHint();
impl /*struct*/ QAbstractScrollArea {
  pub fn sizeHint<RetType, T: QAbstractScrollArea_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  QSize QAbstractScrollArea::sizeHint();
impl<'a> /*trait*/ QAbstractScrollArea_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QAbstractScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea8sizeHintEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar * scrollbar);
impl /*struct*/ QAbstractScrollArea {
  pub fn setHorizontalScrollBar<RetType, T: QAbstractScrollArea_setHorizontalScrollBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setHorizontalScrollBar<RetType> {
  fn setHorizontalScrollBar(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar * scrollbar);
impl<'a> /*trait*/ QAbstractScrollArea_setHorizontalScrollBar<()> for (&'a QScrollBar) {
  fn setHorizontalScrollBar(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setVerticalScrollBar(QScrollBar * scrollbar);
impl /*struct*/ QAbstractScrollArea {
  pub fn setVerticalScrollBar<RetType, T: QAbstractScrollArea_setVerticalScrollBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setVerticalScrollBar<RetType> {
  fn setVerticalScrollBar(self , rsthis: & QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setVerticalScrollBar(QScrollBar * scrollbar);
impl<'a> /*trait*/ QAbstractScrollArea_setVerticalScrollBar<()> for (&'a QScrollBar) {
  fn setVerticalScrollBar(self , rsthis: & QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

