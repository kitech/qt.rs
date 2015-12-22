// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QScrollBar * QAbstractScrollArea::horizontalScrollBar();
  fn _ZNK19QAbstractScrollArea19horizontalScrollBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QAbstractScrollArea::maximumViewportSize();
  fn _ZNK19QAbstractScrollArea19maximumViewportSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractScrollArea::QAbstractScrollArea(QWidget * parent);
  fn _ZN19QAbstractScrollAreaC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractScrollArea::setViewport(QWidget * widget);
  fn _ZN19QAbstractScrollArea11setViewportEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QAbstractScrollArea::minimumSizeHint();
  fn _ZNK19QAbstractScrollArea15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractScrollArea::setCornerWidget(QWidget * widget);
  fn _ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractScrollArea::metaObject();
  fn _ZNK19QAbstractScrollArea10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAbstractScrollArea::setupViewport(QWidget * viewport);
  fn _ZN19QAbstractScrollArea13setupViewportEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QAbstractScrollArea::cornerWidget();
  fn _ZNK19QAbstractScrollArea12cornerWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QScrollBar * QAbstractScrollArea::verticalScrollBar();
  fn _ZNK19QAbstractScrollArea17verticalScrollBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QAbstractScrollArea::viewport();
  fn _ZNK19QAbstractScrollArea8viewportEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractScrollArea::~QAbstractScrollArea();
  fn _ZN19QAbstractScrollAreaD0Ev(qthis: *mut c_void);
  // proto:  QSize QAbstractScrollArea::sizeHint();
  fn _ZNK19QAbstractScrollArea8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar * scrollbar);
  fn _ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractScrollArea::setVerticalScrollBar(QScrollBar * scrollbar);
  fn _ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractScrollArea)=1
pub struct QAbstractScrollArea {
  qbase: QFrame,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractScrollArea {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractScrollArea {
    return QAbstractScrollArea{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAbstractScrollArea {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return &self.qbase;
  }
}
impl AsRef<QFrame> for QAbstractScrollArea {
  fn as_ref(&self) -> &QFrame {
    return &self.qbase;
  }
}
  // proto:  QScrollBar * QAbstractScrollArea::horizontalScrollBar();
impl /*struct*/ QAbstractScrollArea {
  pub fn horizontalScrollBar<RetType, T: QAbstractScrollArea_horizontalScrollBar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_horizontalScrollBar<RetType> {
  fn horizontalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QScrollBar * QAbstractScrollArea::horizontalScrollBar();
impl<'a> /*trait*/ QAbstractScrollArea_horizontalScrollBar<QScrollBar> for () {
  fn horizontalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> QScrollBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea19horizontalScrollBarEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea19horizontalScrollBarEv(rsthis.qclsinst)};
    let mut ret1 = QScrollBar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QAbstractScrollArea::maximumViewportSize();
impl /*struct*/ QAbstractScrollArea {
  pub fn maximumViewportSize<RetType, T: QAbstractScrollArea_maximumViewportSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumViewportSize(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_maximumViewportSize<RetType> {
  fn maximumViewportSize(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QSize QAbstractScrollArea::maximumViewportSize();
impl<'a> /*trait*/ QAbstractScrollArea_maximumViewportSize<QSize> for () {
  fn maximumViewportSize(self , rsthis: &mut QAbstractScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea19maximumViewportSizeEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea19maximumViewportSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::QAbstractScrollArea(QWidget * parent);
impl /*struct*/ QAbstractScrollArea {
  pub fn NewQAbstractScrollArea<T: QAbstractScrollArea_NewQAbstractScrollArea>(value: T) -> QAbstractScrollArea {
    let rsthis = value.NewQAbstractScrollArea();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractScrollArea_NewQAbstractScrollArea {
  fn NewQAbstractScrollArea(self) -> QAbstractScrollArea;
}

  // proto:  void QAbstractScrollArea::QAbstractScrollArea(QWidget * parent);
impl<'a> /*trait*/ QAbstractScrollArea_NewQAbstractScrollArea for (QWidget) {
  fn NewQAbstractScrollArea(self) -> QAbstractScrollArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollAreaC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QAbstractScrollAreaC1EP7QWidget(qthis, arg0)};
    let rsthis = QAbstractScrollArea{/**/qbase: QFrame::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setViewport(QWidget * widget);
impl /*struct*/ QAbstractScrollArea {
  pub fn setViewport<RetType, T: QAbstractScrollArea_setViewport<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setViewport(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setViewport<RetType> {
  fn setViewport(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setViewport(QWidget * widget);
impl<'a> /*trait*/ QAbstractScrollArea_setViewport<()> for (QWidget) {
  fn setViewport(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea11setViewportEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea11setViewportEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QAbstractScrollArea::minimumSizeHint();
impl /*struct*/ QAbstractScrollArea {
  pub fn minimumSizeHint<RetType, T: QAbstractScrollArea_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QSize QAbstractScrollArea::minimumSizeHint();
impl<'a> /*trait*/ QAbstractScrollArea_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QAbstractScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setCornerWidget(QWidget * widget);
impl /*struct*/ QAbstractScrollArea {
  pub fn setCornerWidget<RetType, T: QAbstractScrollArea_setCornerWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCornerWidget(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setCornerWidget<RetType> {
  fn setCornerWidget(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setCornerWidget(QWidget * widget);
impl<'a> /*trait*/ QAbstractScrollArea_setCornerWidget<()> for (QWidget) {
  fn setCornerWidget(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractScrollArea::metaObject();
impl /*struct*/ QAbstractScrollArea {
  pub fn metaObject<RetType, T: QAbstractScrollArea_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  const QMetaObject * QAbstractScrollArea::metaObject();
impl<'a> /*trait*/ QAbstractScrollArea_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea10metaObjectEv()};
     unsafe {_ZNK19QAbstractScrollArea10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setupViewport(QWidget * viewport);
impl /*struct*/ QAbstractScrollArea {
  pub fn setupViewport<RetType, T: QAbstractScrollArea_setupViewport<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setupViewport(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setupViewport<RetType> {
  fn setupViewport(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setupViewport(QWidget * viewport);
impl<'a> /*trait*/ QAbstractScrollArea_setupViewport<()> for (QWidget) {
  fn setupViewport(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea13setupViewportEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea13setupViewportEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QAbstractScrollArea::cornerWidget();
impl /*struct*/ QAbstractScrollArea {
  pub fn cornerWidget<RetType, T: QAbstractScrollArea_cornerWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cornerWidget(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_cornerWidget<RetType> {
  fn cornerWidget(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QWidget * QAbstractScrollArea::cornerWidget();
impl<'a> /*trait*/ QAbstractScrollArea_cornerWidget<QWidget> for () {
  fn cornerWidget(self , rsthis: &mut QAbstractScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea12cornerWidgetEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea12cornerWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QScrollBar * QAbstractScrollArea::verticalScrollBar();
impl /*struct*/ QAbstractScrollArea {
  pub fn verticalScrollBar<RetType, T: QAbstractScrollArea_verticalScrollBar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_verticalScrollBar<RetType> {
  fn verticalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QScrollBar * QAbstractScrollArea::verticalScrollBar();
impl<'a> /*trait*/ QAbstractScrollArea_verticalScrollBar<QScrollBar> for () {
  fn verticalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> QScrollBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea17verticalScrollBarEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea17verticalScrollBarEv(rsthis.qclsinst)};
    let mut ret1 = QScrollBar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QAbstractScrollArea::viewport();
impl /*struct*/ QAbstractScrollArea {
  pub fn viewport<RetType, T: QAbstractScrollArea_viewport<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.viewport(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_viewport<RetType> {
  fn viewport(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QWidget * QAbstractScrollArea::viewport();
impl<'a> /*trait*/ QAbstractScrollArea_viewport<QWidget> for () {
  fn viewport(self , rsthis: &mut QAbstractScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea8viewportEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea8viewportEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::~QAbstractScrollArea();
impl /*struct*/ QAbstractScrollArea {
  pub fn FreeQAbstractScrollArea<RetType, T: QAbstractScrollArea_FreeQAbstractScrollArea<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractScrollArea(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_FreeQAbstractScrollArea<RetType> {
  fn FreeQAbstractScrollArea(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::~QAbstractScrollArea();
impl<'a> /*trait*/ QAbstractScrollArea_FreeQAbstractScrollArea<()> for () {
  fn FreeQAbstractScrollArea(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollAreaD0Ev()};
     unsafe {_ZN19QAbstractScrollAreaD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QAbstractScrollArea::sizeHint();
impl /*struct*/ QAbstractScrollArea {
  pub fn sizeHint<RetType, T: QAbstractScrollArea_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  QSize QAbstractScrollArea::sizeHint();
impl<'a> /*trait*/ QAbstractScrollArea_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QAbstractScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractScrollArea8sizeHintEv()};
    let mut ret = unsafe {_ZNK19QAbstractScrollArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar * scrollbar);
impl /*struct*/ QAbstractScrollArea {
  pub fn setHorizontalScrollBar<RetType, T: QAbstractScrollArea_setHorizontalScrollBar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setHorizontalScrollBar<RetType> {
  fn setHorizontalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setHorizontalScrollBar(QScrollBar * scrollbar);
impl<'a> /*trait*/ QAbstractScrollArea_setHorizontalScrollBar<()> for (QScrollBar) {
  fn setHorizontalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractScrollArea::setVerticalScrollBar(QScrollBar * scrollbar);
impl /*struct*/ QAbstractScrollArea {
  pub fn setVerticalScrollBar<RetType, T: QAbstractScrollArea_setVerticalScrollBar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVerticalScrollBar(self);
    // return 1;
  }
}

pub trait QAbstractScrollArea_setVerticalScrollBar<RetType> {
  fn setVerticalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> RetType;
}

  // proto:  void QAbstractScrollArea::setVerticalScrollBar(QScrollBar * scrollbar);
impl<'a> /*trait*/ QAbstractScrollArea_setVerticalScrollBar<()> for (QScrollBar) {
  fn setVerticalScrollBar(self , rsthis: &mut QAbstractScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

