// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qrectf::QRectF;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsProxyWidget::FreeQGraphicsProxyWidget();
  fn _ZN20QGraphicsProxyWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  QGraphicsProxyWidget * QGraphicsProxyWidget::createProxyForChildWidget(QWidget * child);
  fn _ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsProxyWidget::setWidget(QWidget * widget);
  fn _ZN20QGraphicsProxyWidget9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QGraphicsProxyWidget::subWidgetRect(const QWidget * widget);
  fn _ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsProxyWidget::NewQGraphicsProxyWidget(const QGraphicsProxyWidget & );
  fn _ZN20QGraphicsProxyWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsProxyWidget::metaObject();
  fn _ZNK20QGraphicsProxyWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsProxyWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsProxyWidget::setGeometry(const QRectF & rect);
  fn _ZN20QGraphicsProxyWidget11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QGraphicsProxyWidget::widget();
  fn _ZNK20QGraphicsProxyWidget6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QGraphicsProxyWidget::type_();
  fn _ZNK20QGraphicsProxyWidget4typeEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QGraphicsProxyWidget)=1
pub struct QGraphicsProxyWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn FreeQGraphicsProxyWidget<T: QGraphicsProxyWidget_FreeQGraphicsProxyWidget>(&mut self, value: T)  {
     value.FreeQGraphicsProxyWidget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_FreeQGraphicsProxyWidget {
  fn FreeQGraphicsProxyWidget(self, rsthis: &mut QGraphicsProxyWidget) ;
}

// proto:  void QGraphicsProxyWidget::FreeQGraphicsProxyWidget();
impl<'a> /*trait*/ QGraphicsProxyWidget_FreeQGraphicsProxyWidget for () {
  fn FreeQGraphicsProxyWidget(self, rsthis: &mut QGraphicsProxyWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidgetD0Ev()};
     unsafe {_ZN20QGraphicsProxyWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn createProxyForChildWidget<T: QGraphicsProxyWidget_createProxyForChildWidget>(&mut self, value: T)  {
     value.createProxyForChildWidget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_createProxyForChildWidget {
  fn createProxyForChildWidget(self, rsthis: &mut QGraphicsProxyWidget) ;
}

// proto:  QGraphicsProxyWidget * QGraphicsProxyWidget::createProxyForChildWidget(QWidget * child);
impl<'a> /*trait*/ QGraphicsProxyWidget_createProxyForChildWidget for (&'a mut QWidget) {
  fn createProxyForChildWidget(self, rsthis: &mut QGraphicsProxyWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn setWidget<T: QGraphicsProxyWidget_setWidget>(&mut self, value: T)  {
     value.setWidget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_setWidget {
  fn setWidget(self, rsthis: &mut QGraphicsProxyWidget) ;
}

// proto:  void QGraphicsProxyWidget::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGraphicsProxyWidget_setWidget for (&'a mut QWidget) {
  fn setWidget(self, rsthis: &mut QGraphicsProxyWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsProxyWidget9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn subWidgetRect<T: QGraphicsProxyWidget_subWidgetRect>(&mut self, value: T) -> QRectF {
    return value.subWidgetRect(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_subWidgetRect {
  fn subWidgetRect(self, rsthis: &mut QGraphicsProxyWidget) -> QRectF;
}

// proto:  QRectF QGraphicsProxyWidget::subWidgetRect(const QWidget * widget);
impl<'a> /*trait*/ QGraphicsProxyWidget_subWidgetRect for (&'a  QWidget) {
  fn subWidgetRect(self, rsthis: &mut QGraphicsProxyWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn NewQGraphicsProxyWidget<T: QGraphicsProxyWidget_NewQGraphicsProxyWidget>(value: T) -> QGraphicsProxyWidget {
    let rsthis = value.NewQGraphicsProxyWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_NewQGraphicsProxyWidget {
  fn NewQGraphicsProxyWidget(self) -> QGraphicsProxyWidget;
}

// proto: void QGraphicsProxyWidget::NewQGraphicsProxyWidget(const QGraphicsProxyWidget & );
impl<'a> /*trait*/ QGraphicsProxyWidget_NewQGraphicsProxyWidget for (&'a  QGraphicsProxyWidget) {
  fn NewQGraphicsProxyWidget(self) -> QGraphicsProxyWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsProxyWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsProxyWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn metaObject<T: QGraphicsProxyWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsProxyWidget) ;
}

// proto:  const QMetaObject * QGraphicsProxyWidget::metaObject();
impl<'a> /*trait*/ QGraphicsProxyWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsProxyWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget10metaObjectEv()};
     unsafe {_ZNK20QGraphicsProxyWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn paint<T: QGraphicsProxyWidget_paint>(&mut self, value: T)  {
     value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_paint {
  fn paint(self, rsthis: &mut QGraphicsProxyWidget) ;
}

// proto:  void QGraphicsProxyWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsProxyWidget_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsProxyWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn setGeometry<T: QGraphicsProxyWidget_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_setGeometry {
  fn setGeometry(self, rsthis: &mut QGraphicsProxyWidget) ;
}

// proto:  void QGraphicsProxyWidget::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsProxyWidget_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, rsthis: &mut QGraphicsProxyWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsProxyWidget11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsProxyWidget11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn widget<T: QGraphicsProxyWidget_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_widget {
  fn widget(self, rsthis: &mut QGraphicsProxyWidget) -> QWidget;
}

// proto:  QWidget * QGraphicsProxyWidget::widget();
impl<'a> /*trait*/ QGraphicsProxyWidget_widget for () {
  fn widget(self, rsthis: &mut QGraphicsProxyWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget6widgetEv()};
    let mut ret = unsafe {_ZNK20QGraphicsProxyWidget6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsProxyWidget {
  pub fn type_<T: QGraphicsProxyWidget_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_type_ {
  fn type_(self, rsthis: &mut QGraphicsProxyWidget) -> i32;
}

// proto:  int QGraphicsProxyWidget::type_();
impl<'a> /*trait*/ QGraphicsProxyWidget_type_ for () {
  fn type_(self, rsthis: &mut QGraphicsProxyWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsProxyWidget4typeEv()};
    let mut ret = unsafe {_ZNK20QGraphicsProxyWidget4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

