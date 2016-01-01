// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qframe.h
// dst-file: /src/widgets/qframe.rs
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
use super::super::core::qrect::QRect; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFrame_Class_Size() -> c_int;
  // proto:  void QFrame::setFrameRect(const QRect & );
  fn _ZN6QFrame12setFrameRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QFrame::lineWidth();
  fn _ZNK6QFrame9lineWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFrame::setFrameStyle(int );
  fn _ZN6QFrame13setFrameStyleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRect QFrame::frameRect();
  fn _ZNK6QFrame9frameRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QFrame::sizeHint();
  fn _ZNK6QFrame8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFrame::QFrame(const QFrame & );
  fn dector_ZN6QFrameC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QFrameC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QFrame::frameStyle();
  fn _ZNK6QFrame10frameStyleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QFrame::midLineWidth();
  fn _ZNK6QFrame12midLineWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFrame::setLineWidth(int );
  fn _ZN6QFrame12setLineWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFrame::setMidLineWidth(int );
  fn _ZN6QFrame15setMidLineWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QFrame::metaObject();
  fn _ZNK6QFrame10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QFrame::frameWidth();
  fn _ZNK6QFrame10frameWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFrame::~QFrame();
  fn _ZN6QFrameD0Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QFrame)=1
#[derive(Default)]
pub struct QFrame {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFrame {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFrame {
    return QFrame{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFrame {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QFrame {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QFrame::setFrameRect(const QRect & );
impl /*struct*/ QFrame {
  pub fn setFrameRect<RetType, T: QFrame_setFrameRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrameRect(self);
    // return 1;
  }
}

pub trait QFrame_setFrameRect<RetType> {
  fn setFrameRect(self , rsthis: & QFrame) -> RetType;
}

  // proto:  void QFrame::setFrameRect(const QRect & );
impl<'a> /*trait*/ QFrame_setFrameRect<()> for (&'a QRect) {
  fn setFrameRect(self , rsthis: & QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setFrameRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QFrame12setFrameRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QFrame::lineWidth();
impl /*struct*/ QFrame {
  pub fn lineWidth<RetType, T: QFrame_lineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineWidth(self);
    // return 1;
  }
}

pub trait QFrame_lineWidth<RetType> {
  fn lineWidth(self , rsthis: & QFrame) -> RetType;
}

  // proto:  int QFrame::lineWidth();
impl<'a> /*trait*/ QFrame_lineWidth<i32> for () {
  fn lineWidth(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9lineWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame9lineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFrame::setFrameStyle(int );
impl /*struct*/ QFrame {
  pub fn setFrameStyle<RetType, T: QFrame_setFrameStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrameStyle(self);
    // return 1;
  }
}

pub trait QFrame_setFrameStyle<RetType> {
  fn setFrameStyle(self , rsthis: & QFrame) -> RetType;
}

  // proto:  void QFrame::setFrameStyle(int );
impl<'a> /*trait*/ QFrame_setFrameStyle<()> for (i32) {
  fn setFrameStyle(self , rsthis: & QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame13setFrameStyleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame13setFrameStyleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QFrame::frameRect();
impl /*struct*/ QFrame {
  pub fn frameRect<RetType, T: QFrame_frameRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameRect(self);
    // return 1;
  }
}

pub trait QFrame_frameRect<RetType> {
  fn frameRect(self , rsthis: & QFrame) -> RetType;
}

  // proto:  QRect QFrame::frameRect();
impl<'a> /*trait*/ QFrame_frameRect<QRect> for () {
  fn frameRect(self , rsthis: & QFrame) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9frameRectEv()};
    let mut ret = unsafe {_ZNK6QFrame9frameRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QFrame::sizeHint();
impl /*struct*/ QFrame {
  pub fn sizeHint<RetType, T: QFrame_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QFrame_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QFrame) -> RetType;
}

  // proto:  QSize QFrame::sizeHint();
impl<'a> /*trait*/ QFrame_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QFrame) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame8sizeHintEv()};
    let mut ret = unsafe {_ZNK6QFrame8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFrame::QFrame(const QFrame & );
impl /*struct*/ QFrame {
  pub fn new<T: QFrame_new>(value: T) -> QFrame {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFrame_new {
  fn new(self) -> QFrame;
}

  // proto:  void QFrame::QFrame(const QFrame & );
impl<'a> /*trait*/ QFrame_new for (&'a QFrame) {
  fn new(self) -> QFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameC1ERKS_()};
    let ctysz: c_int = unsafe{QFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QFrameC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN6QFrameC1ERKS_(arg0)} as u64;
    let rsthis = QFrame{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QFrame::frameStyle();
impl /*struct*/ QFrame {
  pub fn frameStyle<RetType, T: QFrame_frameStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameStyle(self);
    // return 1;
  }
}

pub trait QFrame_frameStyle<RetType> {
  fn frameStyle(self , rsthis: & QFrame) -> RetType;
}

  // proto:  int QFrame::frameStyle();
impl<'a> /*trait*/ QFrame_frameStyle<i32> for () {
  fn frameStyle(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameStyleEv()};
    let mut ret = unsafe {_ZNK6QFrame10frameStyleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFrame::midLineWidth();
impl /*struct*/ QFrame {
  pub fn midLineWidth<RetType, T: QFrame_midLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.midLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_midLineWidth<RetType> {
  fn midLineWidth(self , rsthis: & QFrame) -> RetType;
}

  // proto:  int QFrame::midLineWidth();
impl<'a> /*trait*/ QFrame_midLineWidth<i32> for () {
  fn midLineWidth(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame12midLineWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame12midLineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFrame::setLineWidth(int );
impl /*struct*/ QFrame {
  pub fn setLineWidth<RetType, T: QFrame_setLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_setLineWidth<RetType> {
  fn setLineWidth(self , rsthis: & QFrame) -> RetType;
}

  // proto:  void QFrame::setLineWidth(int );
impl<'a> /*trait*/ QFrame_setLineWidth<()> for (i32) {
  fn setLineWidth(self , rsthis: & QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setLineWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame12setLineWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFrame::setMidLineWidth(int );
impl /*struct*/ QFrame {
  pub fn setMidLineWidth<RetType, T: QFrame_setMidLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMidLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_setMidLineWidth<RetType> {
  fn setMidLineWidth(self , rsthis: & QFrame) -> RetType;
}

  // proto:  void QFrame::setMidLineWidth(int );
impl<'a> /*trait*/ QFrame_setMidLineWidth<()> for (i32) {
  fn setMidLineWidth(self , rsthis: & QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame15setMidLineWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame15setMidLineWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFrame::metaObject();
impl /*struct*/ QFrame {
  pub fn metaObject<RetType, T: QFrame_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFrame_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFrame) -> RetType;
}

  // proto:  const QMetaObject * QFrame::metaObject();
impl<'a> /*trait*/ QFrame_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10metaObjectEv()};
     unsafe {_ZNK6QFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QFrame::frameWidth();
impl /*struct*/ QFrame {
  pub fn frameWidth<RetType, T: QFrame_frameWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameWidth(self);
    // return 1;
  }
}

pub trait QFrame_frameWidth<RetType> {
  fn frameWidth(self , rsthis: & QFrame) -> RetType;
}

  // proto:  int QFrame::frameWidth();
impl<'a> /*trait*/ QFrame_frameWidth<i32> for () {
  fn frameWidth(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame10frameWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFrame::~QFrame();
impl /*struct*/ QFrame {
  pub fn free<RetType, T: QFrame_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFrame_free<RetType> {
  fn free(self , rsthis: & QFrame) -> RetType;
}

  // proto:  void QFrame::~QFrame();
impl<'a> /*trait*/ QFrame_free<()> for () {
  fn free(self , rsthis: & QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameD0Ev()};
     unsafe {_ZN6QFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

