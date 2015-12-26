// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFocusFrame_Class_Size() -> c_int;
  // proto:  void QFocusFrame::~QFocusFrame();
  fn _ZN11QFocusFrameD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QFocusFrame::metaObject();
  fn _ZNK11QFocusFrame10metaObjectEv(qthis: *mut c_void);
  // proto:  void QFocusFrame::QFocusFrame(const QFocusFrame & );
  fn dector_ZN11QFocusFrameC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFocusFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QFocusFrame::widget();
  fn _ZNK11QFocusFrame6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
  fn dector_ZN11QFocusFrameC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFocusFrameC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFocusFrame::setWidget(QWidget * widget);
  fn _ZN11QFocusFrame9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFocusFrame)=1
pub struct QFocusFrame {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFocusFrame {
  pub fn inheritFrom(qthis: *mut c_void) -> QFocusFrame {
    return QFocusFrame{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
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
  pub fn Free<RetType, T: QFocusFrame_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFocusFrame_Free<RetType> {
  fn Free(self , rsthis: & QFocusFrame) -> RetType;
}

  // proto:  void QFocusFrame::~QFocusFrame();
impl<'a> /*trait*/ QFocusFrame_Free<()> for () {
  fn Free(self , rsthis: & QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameD0Ev()};
     unsafe {_ZN11QFocusFrameD0Ev(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QFocusFrame_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame10metaObjectEv()};
     unsafe {_ZNK11QFocusFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFocusFrame::QFocusFrame(const QFocusFrame & );
impl /*struct*/ QFocusFrame {
  pub fn New<T: QFocusFrame_New>(value: T) -> QFocusFrame {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFocusFrame_New {
  fn New(self) -> QFocusFrame;
}

  // proto:  void QFocusFrame::QFocusFrame(const QFocusFrame & );
impl<'a> /*trait*/ QFocusFrame_New for (&'a QFocusFrame) {
  fn New(self) -> QFocusFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameC1ERKS_()};
    let ctysz: c_int = unsafe{QFocusFrame_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFocusFrameC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFocusFrameC1ERKS_(arg0)};
    let rsthis = QFocusFrame{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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
    let mut ret = unsafe {_ZNK11QFocusFrame6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
impl<'a> /*trait*/ QFocusFrame_New for (&'a QWidget) {
  fn New(self) -> QFocusFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameC1EP7QWidget()};
    let ctysz: c_int = unsafe{QFocusFrame_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFocusFrameC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFocusFrameC1EP7QWidget(arg0)};
    let rsthis = QFocusFrame{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
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
     unsafe {_ZN11QFocusFrame9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

