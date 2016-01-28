// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qgraphicsproxywidget.h
// dst-file: /src/widgets/qgraphicsproxywidget.rs
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
use super::qgraphicswidget::*; // 773
use std::ops::Deref;
use super::qwidget::*; // 773
use super::super::core::qrect::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::gui::qpainter::*; // 771
use super::qstyleoption::*; // 773
use super::qgraphicsitem::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsProxyWidget_Class_Size() -> c_int;
  // proto:  void QGraphicsProxyWidget::~QGraphicsProxyWidget();
  fn C_ZN20QGraphicsProxyWidgetD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsProxyWidget * QGraphicsProxyWidget::createProxyForChildWidget(QWidget * child);
  fn C_ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsProxyWidget::setWidget(QWidget * widget);
  fn C_ZN20QGraphicsProxyWidget9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QGraphicsProxyWidget::subWidgetRect(const QWidget * widget);
  fn C_ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsProxyWidget::metaObject();
  fn C_ZNK20QGraphicsProxyWidget10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsProxyWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsProxyWidget::setGeometry(const QRectF & rect);
  fn C_ZN20QGraphicsProxyWidget11setGeometryERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QGraphicsProxyWidget::widget();
  fn C_ZNK20QGraphicsProxyWidget6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGraphicsProxyWidget::type();
  fn C_ZNK20QGraphicsProxyWidget4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsProxyWidget)=1
#[derive(Default)]
pub struct QGraphicsProxyWidget {
  qbase: QGraphicsWidget,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsProxyWidget {
    return QGraphicsProxyWidget{qbase: QGraphicsWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsProxyWidget {
  type Target = QGraphicsWidget;

  fn deref(&self) -> &QGraphicsWidget {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsWidget> for QGraphicsProxyWidget {
  fn as_ref(& self) -> & QGraphicsWidget {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsProxyWidget::~QGraphicsProxyWidget();
impl /*struct*/ QGraphicsProxyWidget {
  pub fn free<RetType, T: QGraphicsProxyWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_free<RetType> {
  fn free(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  void QGraphicsProxyWidget::~QGraphicsProxyWidget();
impl<'a> /*trait*/ QGraphicsProxyWidget_free<()> for () {
  fn free(self , rsthis: & QGraphicsProxyWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidgetD2Ev()};
     unsafe {C_ZN20QGraphicsProxyWidgetD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsProxyWidget * QGraphicsProxyWidget::createProxyForChildWidget(QWidget * child);
impl /*struct*/ QGraphicsProxyWidget {
  pub fn createProxyForChildWidget<RetType, T: QGraphicsProxyWidget_createProxyForChildWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createProxyForChildWidget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_createProxyForChildWidget<RetType> {
  fn createProxyForChildWidget(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  QGraphicsProxyWidget * QGraphicsProxyWidget::createProxyForChildWidget(QWidget * child);
impl<'a> /*trait*/ QGraphicsProxyWidget_createProxyForChildWidget<QGraphicsProxyWidget> for (&'a QWidget) {
  fn createProxyForChildWidget(self , rsthis: & QGraphicsProxyWidget) -> QGraphicsProxyWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QGraphicsProxyWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsProxyWidget::setWidget(QWidget * widget);
impl /*struct*/ QGraphicsProxyWidget {
  pub fn setWidget<RetType, T: QGraphicsProxyWidget_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_setWidget<RetType> {
  fn setWidget(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  void QGraphicsProxyWidget::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGraphicsProxyWidget_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QGraphicsProxyWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsProxyWidget9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsProxyWidget::subWidgetRect(const QWidget * widget);
impl /*struct*/ QGraphicsProxyWidget {
  pub fn subWidgetRect<RetType, T: QGraphicsProxyWidget_subWidgetRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subWidgetRect(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_subWidgetRect<RetType> {
  fn subWidgetRect(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  QRectF QGraphicsProxyWidget::subWidgetRect(const QWidget * widget);
impl<'a> /*trait*/ QGraphicsProxyWidget_subWidgetRect<QRectF> for (&'a QWidget) {
  fn subWidgetRect(self , rsthis: & QGraphicsProxyWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsProxyWidget::metaObject();
impl /*struct*/ QGraphicsProxyWidget {
  pub fn metaObject<RetType, T: QGraphicsProxyWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsProxyWidget::metaObject();
impl<'a> /*trait*/ QGraphicsProxyWidget_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsProxyWidget) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget10metaObjectEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsProxyWidget10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsProxyWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsProxyWidget {
  pub fn paint<RetType, T: QGraphicsProxyWidget_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  void QGraphicsProxyWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsProxyWidget_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsProxyWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsProxyWidget::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsProxyWidget {
  pub fn setGeometry<RetType, T: QGraphicsProxyWidget_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  void QGraphicsProxyWidget::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsProxyWidget_setGeometry<()> for (&'a QRectF) {
  fn setGeometry(self , rsthis: & QGraphicsProxyWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsProxyWidget11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QGraphicsProxyWidget::widget();
impl /*struct*/ QGraphicsProxyWidget {
  pub fn widget<RetType, T: QGraphicsProxyWidget_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_widget<RetType> {
  fn widget(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  QWidget * QGraphicsProxyWidget::widget();
impl<'a> /*trait*/ QGraphicsProxyWidget_widget<QWidget> for () {
  fn widget(self , rsthis: & QGraphicsProxyWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget6widgetEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsProxyWidget6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGraphicsProxyWidget::type();
impl /*struct*/ QGraphicsProxyWidget {
  pub fn type_<RetType, T: QGraphicsProxyWidget_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}

  // proto:  int QGraphicsProxyWidget::type();
impl<'a> /*trait*/ QGraphicsProxyWidget_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsProxyWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget4typeEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsProxyWidget4typeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

// <= body block end

