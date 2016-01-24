// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qfocusframe.h
// dst-file: /src/widgets/qfocusframe.rs
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
use super::super::core::qobjectdefs::QMetaObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFocusFrame_Class_Size() -> c_int;
  // proto:  void QFocusFrame::~QFocusFrame();
  fn C_ZN11QFocusFrameD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QFocusFrame::metaObject();
  fn C_ZNK11QFocusFrame10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QFocusFrame::widget();
  fn C_ZNK11QFocusFrame6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
  fn C_ZN11QFocusFrameC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QFocusFrame::setWidget(QWidget * widget);
  fn C_ZN11QFocusFrame9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFocusFrame)=1
#[derive(Default)]
pub struct QFocusFrame {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFocusFrame {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFocusFrame {
    return QFocusFrame{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFocusFrame {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QFocusFrame {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QFocusFrame::~QFocusFrame();
impl /*struct*/ QFocusFrame {
  pub fn free<RetType, T: QFocusFrame_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFocusFrame_free<RetType> {
  fn free(self , rsthis: & QFocusFrame) -> RetType;
}

  // proto:  void QFocusFrame::~QFocusFrame();
impl<'a> /*trait*/ QFocusFrame_free<()> for () {
  fn free(self , rsthis: & QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameD2Ev()};
     unsafe {C_ZN11QFocusFrameD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFocusFrame::metaObject();
impl /*struct*/ QFocusFrame {
  pub fn metaObject<RetType, T: QFocusFrame_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFocusFrame_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFocusFrame) -> RetType;
}

  // proto:  const QMetaObject * QFocusFrame::metaObject();
impl<'a> /*trait*/ QFocusFrame_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QFocusFrame) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QFocusFrame10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QFocusFrame::widget();
impl /*struct*/ QFocusFrame {
  pub fn widget<RetType, T: QFocusFrame_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QFocusFrame_widget<RetType> {
  fn widget(self , rsthis: & QFocusFrame) -> RetType;
}

  // proto:  QWidget * QFocusFrame::widget();
impl<'a> /*trait*/ QFocusFrame_widget<QWidget> for () {
  fn widget(self , rsthis: & QFocusFrame) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame6widgetEv()};
    let mut ret = unsafe {C_ZNK11QFocusFrame6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
impl /*struct*/ QFocusFrame {
  pub fn new<T: QFocusFrame_new>(value: T) -> QFocusFrame {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFocusFrame_new {
  fn new(self) -> QFocusFrame;
}

  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
impl<'a> /*trait*/ QFocusFrame_new for (&'a QWidget) {
  fn new(self) -> QFocusFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameC2EP7QWidget()};
    let ctysz: c_int = unsafe{QFocusFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QFocusFrameC2EP7QWidget(arg0)};
    let rsthis = QFocusFrame{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFocusFrame::setWidget(QWidget * widget);
impl /*struct*/ QFocusFrame {
  pub fn setWidget<RetType, T: QFocusFrame_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QFocusFrame_setWidget<RetType> {
  fn setWidget(self , rsthis: & QFocusFrame) -> RetType;
}

  // proto:  void QFocusFrame::setWidget(QWidget * widget);
impl<'a> /*trait*/ QFocusFrame_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrame9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QFocusFrame9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

