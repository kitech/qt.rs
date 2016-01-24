// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qscrollarea.h
// dst-file: /src/widgets/qscrollarea.rs
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
use super::qabstractscrollarea::QAbstractScrollArea; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QScrollArea_Class_Size() -> c_int;
  // proto:  void QScrollArea::QScrollArea(QWidget * parent);
  fn C_ZN11QScrollAreaC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QScrollArea::setWidgetResizable(bool resizable);
  fn C_ZN11QScrollArea18setWidgetResizableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QScrollArea::setWidget(QWidget * widget);
  fn C_ZN11QScrollArea9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QScrollArea::takeWidget();
  fn C_ZN11QScrollArea10takeWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScrollArea::ensureVisible(int x, int y, int xmargin, int ymargin);
  fn C_ZN11QScrollArea13ensureVisibleEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QScrollArea::ensureWidgetVisible(QWidget * childWidget, int xmargin, int ymargin);
  fn C_ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  QWidget * QScrollArea::widget();
  fn C_ZNK11QScrollArea6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QScrollArea::sizeHint();
  fn C_ZNK11QScrollArea8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QScrollArea::widgetResizable();
  fn C_ZNK11QScrollArea15widgetResizableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QScrollArea::~QScrollArea();
  fn C_ZN11QScrollAreaD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QScrollArea::focusNextPrevChild(bool next);
  fn C_ZN11QScrollArea18focusNextPrevChildEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  const QMetaObject * QScrollArea::metaObject();
  fn C_ZNK11QScrollArea10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QScrollArea)=1
#[derive(Default)]
pub struct QScrollArea {
  qbase: QAbstractScrollArea,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QScrollArea {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScrollArea {
    return QScrollArea{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QScrollArea {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QScrollArea {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  void QScrollArea::QScrollArea(QWidget * parent);
impl /*struct*/ QScrollArea {
  pub fn new<T: QScrollArea_new>(value: T) -> QScrollArea {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollArea_new {
  fn new(self) -> QScrollArea;
}

  // proto:  void QScrollArea::QScrollArea(QWidget * parent);
impl<'a> /*trait*/ QScrollArea_new for (&'a QWidget) {
  fn new(self) -> QScrollArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollAreaC2EP7QWidget()};
    let ctysz: c_int = unsafe{QScrollArea_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QScrollAreaC2EP7QWidget(arg0)};
    let rsthis = QScrollArea{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QScrollArea::setWidgetResizable(bool resizable);
impl /*struct*/ QScrollArea {
  pub fn setWidgetResizable<RetType, T: QScrollArea_setWidgetResizable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidgetResizable(self);
    // return 1;
  }
}

pub trait QScrollArea_setWidgetResizable<RetType> {
  fn setWidgetResizable(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  void QScrollArea::setWidgetResizable(bool resizable);
impl<'a> /*trait*/ QScrollArea_setWidgetResizable<()> for (i8) {
  fn setWidgetResizable(self , rsthis: & QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea18setWidgetResizableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QScrollArea18setWidgetResizableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScrollArea::setWidget(QWidget * widget);
impl /*struct*/ QScrollArea {
  pub fn setWidget<RetType, T: QScrollArea_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QScrollArea_setWidget<RetType> {
  fn setWidget(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  void QScrollArea::setWidget(QWidget * widget);
impl<'a> /*trait*/ QScrollArea_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QScrollArea9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QScrollArea::takeWidget();
impl /*struct*/ QScrollArea {
  pub fn takeWidget<RetType, T: QScrollArea_takeWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeWidget(self);
    // return 1;
  }
}

pub trait QScrollArea_takeWidget<RetType> {
  fn takeWidget(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  QWidget * QScrollArea::takeWidget();
impl<'a> /*trait*/ QScrollArea_takeWidget<QWidget> for () {
  fn takeWidget(self , rsthis: & QScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea10takeWidgetEv()};
    let mut ret = unsafe {C_ZN11QScrollArea10takeWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollArea::ensureVisible(int x, int y, int xmargin, int ymargin);
impl /*struct*/ QScrollArea {
  pub fn ensureVisible<RetType, T: QScrollArea_ensureVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QScrollArea_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  void QScrollArea::ensureVisible(int x, int y, int xmargin, int ymargin);
impl<'a> /*trait*/ QScrollArea_ensureVisible<()> for (i32, i32, i32, i32) {
  fn ensureVisible(self , rsthis: & QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea13ensureVisibleEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN11QScrollArea13ensureVisibleEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QScrollArea::ensureWidgetVisible(QWidget * childWidget, int xmargin, int ymargin);
impl /*struct*/ QScrollArea {
  pub fn ensureWidgetVisible<RetType, T: QScrollArea_ensureWidgetVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureWidgetVisible(self);
    // return 1;
  }
}

pub trait QScrollArea_ensureWidgetVisible<RetType> {
  fn ensureWidgetVisible(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  void QScrollArea::ensureWidgetVisible(QWidget * childWidget, int xmargin, int ymargin);
impl<'a> /*trait*/ QScrollArea_ensureWidgetVisible<()> for (&'a QWidget, i32, i32) {
  fn ensureWidgetVisible(self , rsthis: & QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QWidget * QScrollArea::widget();
impl /*struct*/ QScrollArea {
  pub fn widget<RetType, T: QScrollArea_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QScrollArea_widget<RetType> {
  fn widget(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  QWidget * QScrollArea::widget();
impl<'a> /*trait*/ QScrollArea_widget<QWidget> for () {
  fn widget(self , rsthis: & QScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea6widgetEv()};
    let mut ret = unsafe {C_ZNK11QScrollArea6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QScrollArea::sizeHint();
impl /*struct*/ QScrollArea {
  pub fn sizeHint<RetType, T: QScrollArea_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QScrollArea_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  QSize QScrollArea::sizeHint();
impl<'a> /*trait*/ QScrollArea_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea8sizeHintEv()};
    let mut ret = unsafe {C_ZNK11QScrollArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QScrollArea::widgetResizable();
impl /*struct*/ QScrollArea {
  pub fn widgetResizable<RetType, T: QScrollArea_widgetResizable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widgetResizable(self);
    // return 1;
  }
}

pub trait QScrollArea_widgetResizable<RetType> {
  fn widgetResizable(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  bool QScrollArea::widgetResizable();
impl<'a> /*trait*/ QScrollArea_widgetResizable<i8> for () {
  fn widgetResizable(self , rsthis: & QScrollArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea15widgetResizableEv()};
    let mut ret = unsafe {C_ZNK11QScrollArea15widgetResizableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QScrollArea::~QScrollArea();
impl /*struct*/ QScrollArea {
  pub fn free<RetType, T: QScrollArea_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScrollArea_free<RetType> {
  fn free(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  void QScrollArea::~QScrollArea();
impl<'a> /*trait*/ QScrollArea_free<()> for () {
  fn free(self , rsthis: & QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollAreaD2Ev()};
     unsafe {C_ZN11QScrollAreaD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QScrollArea::focusNextPrevChild(bool next);
impl /*struct*/ QScrollArea {
  pub fn focusNextPrevChild<RetType, T: QScrollArea_focusNextPrevChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild(self);
    // return 1;
  }
}

pub trait QScrollArea_focusNextPrevChild<RetType> {
  fn focusNextPrevChild(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  bool QScrollArea::focusNextPrevChild(bool next);
impl<'a> /*trait*/ QScrollArea_focusNextPrevChild<i8> for (i8) {
  fn focusNextPrevChild(self , rsthis: & QScrollArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea18focusNextPrevChildEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {C_ZN11QScrollArea18focusNextPrevChildEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QScrollArea::metaObject();
impl /*struct*/ QScrollArea {
  pub fn metaObject<RetType, T: QScrollArea_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScrollArea_metaObject<RetType> {
  fn metaObject(self , rsthis: & QScrollArea) -> RetType;
}

  // proto:  const QMetaObject * QScrollArea::metaObject();
impl<'a> /*trait*/ QScrollArea_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QScrollArea) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QScrollArea10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

