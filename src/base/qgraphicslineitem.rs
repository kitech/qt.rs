// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpen::QPen;
use super::qgraphicsitem::QGraphicsItem;
use super::qlinef::QLineF;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsLineItem::setPen(const QPen & pen);
  fn _ZN17QGraphicsLineItem6setPenERK4QPen(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLineItem::NewQGraphicsLineItem(QGraphicsItem * parent);
  fn _ZN17QGraphicsLineItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLineItem::NewQGraphicsLineItem(const QLineF & line, QGraphicsItem * parent);
  fn _ZN17QGraphicsLineItemC1ERK6QLineFP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QLineF QGraphicsLineItem::line();
  fn _ZNK17QGraphicsLineItem4lineEv() -> i32;
  // proto: QPainterPath QGraphicsLineItem::opaqueArea();
  fn _ZNK17QGraphicsLineItem10opaqueAreaEv() -> i32;
  // proto: void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN17QGraphicsLineItem7setLineEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: QRectF QGraphicsLineItem::boundingRect();
  fn _ZNK17QGraphicsLineItem12boundingRectEv() -> i32;
  // proto: QPen QGraphicsLineItem::pen();
  fn _ZNK17QGraphicsLineItem3penEv() -> i32;
  // proto: void QGraphicsLineItem::setLine(const QLineF & line);
  fn _ZN17QGraphicsLineItem7setLineERK6QLineF(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsLineItem::shape();
  fn _ZNK17QGraphicsLineItem5shapeEv() -> i32;
  // proto: void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: int QGraphicsLineItem::type_();
  fn _ZNK17QGraphicsLineItem4typeEv() -> i32;
  // proto: void QGraphicsLineItem::NewQGraphicsLineItem(const QGraphicsLineItem & );
  fn _ZN17QGraphicsLineItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsLineItem::NewQGraphicsLineItem(qreal x1, qreal y1, qreal x2, qreal y2, QGraphicsItem * parent);
  fn _ZN17QGraphicsLineItemC1EddddP13QGraphicsItem(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> i32;
  // proto: bool QGraphicsLineItem::contains(const QPointF & point);
  fn _ZNK17QGraphicsLineItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLineItem::FreeQGraphicsLineItem();
  fn _ZN17QGraphicsLineItemD0Ev() -> i32;
}

// body block begin
// class sizeof(QGraphicsLineItem)=1
pub struct QGraphicsLineItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLineItem {
  pub fn setPen<T: QGraphicsLineItem_setPen>(&mut self, value: T) -> i32 {
    value.setPen(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_setPen {
  fn setPen(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: void QGraphicsLineItem::setPen(const QPen & pen);
impl<'a> /*trait*/ QGraphicsLineItem_setPen for (&'a  QPen) {
  fn setPen(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsLineItem6setPenERK4QPen(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn NewQGraphicsLineItem<T: QGraphicsLineItem_NewQGraphicsLineItem>(value: T) -> QGraphicsLineItem {
    let rsthis = value.NewQGraphicsLineItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLineItem_NewQGraphicsLineItem {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem;
}

// proto: void QGraphicsLineItem::NewQGraphicsLineItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_NewQGraphicsLineItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsLineItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsLineItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn isObscuredBy<T: QGraphicsLineItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsLineItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

// proto: void QGraphicsLineItem::NewQGraphicsLineItem(const QLineF & line, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_NewQGraphicsLineItem for (&'a  QLineF, &'a mut QGraphicsItem) {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC1ERK6QLineFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsLineItemC1ERK6QLineFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsLineItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn line<T: QGraphicsLineItem_line>(&mut self, value: T) -> i32 {
    value.line(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_line {
  fn line(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: QLineF QGraphicsLineItem::line();
impl<'a> /*trait*/ QGraphicsLineItem_line for () {
  fn line(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem4lineEv()};
    unsafe {_ZNK17QGraphicsLineItem4lineEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn opaqueArea<T: QGraphicsLineItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: QPainterPath QGraphicsLineItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsLineItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem10opaqueAreaEv()};
    unsafe {_ZNK17QGraphicsLineItem10opaqueAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn setLine<T: QGraphicsLineItem_setLine>(&mut self, value: T) -> i32 {
    value.setLine(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_setLine {
  fn setLine(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QGraphicsLineItem_setLine for (f64, f64, f64, f64) {
  fn setLine(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem7setLineEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN17QGraphicsLineItem7setLineEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn boundingRect<T: QGraphicsLineItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: QRectF QGraphicsLineItem::boundingRect();
impl<'a> /*trait*/ QGraphicsLineItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem12boundingRectEv()};
    unsafe {_ZNK17QGraphicsLineItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn pen<T: QGraphicsLineItem_pen>(&mut self, value: T) -> i32 {
    value.pen(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_pen {
  fn pen(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: QPen QGraphicsLineItem::pen();
impl<'a> /*trait*/ QGraphicsLineItem_pen for () {
  fn pen(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem3penEv()};
    unsafe {_ZNK17QGraphicsLineItem3penEv()};
    return 1;
  }
}

// proto: void QGraphicsLineItem::setLine(const QLineF & line);
impl<'a> /*trait*/ QGraphicsLineItem_setLine for (&'a  QLineF) {
  fn setLine(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem7setLineERK6QLineF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsLineItem7setLineERK6QLineF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn shape<T: QGraphicsLineItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_shape {
  fn shape(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: QPainterPath QGraphicsLineItem::shape();
impl<'a> /*trait*/ QGraphicsLineItem_shape for () {
  fn shape(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem5shapeEv()};
    unsafe {_ZNK17QGraphicsLineItem5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn paint<T: QGraphicsLineItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_paint {
  fn paint(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsLineItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn type_<T: QGraphicsLineItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_type_ {
  fn type_(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: int QGraphicsLineItem::type_();
impl<'a> /*trait*/ QGraphicsLineItem_type_ for () {
  fn type_(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem4typeEv()};
    unsafe {_ZNK17QGraphicsLineItem4typeEv()};
    return 1;
  }
}

// proto: void QGraphicsLineItem::NewQGraphicsLineItem(const QGraphicsLineItem & );
impl<'a> /*trait*/ QGraphicsLineItem_NewQGraphicsLineItem for (&'a  QGraphicsLineItem) {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsLineItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsLineItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsLineItem::NewQGraphicsLineItem(qreal x1, qreal y1, qreal x2, qreal y2, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_NewQGraphicsLineItem for (f64, f64, f64, f64, &'a mut QGraphicsItem) {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC1EddddP13QGraphicsItem()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsLineItemC1EddddP13QGraphicsItem(qthis, arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsLineItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn contains<T: QGraphicsLineItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_contains {
  fn contains(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: bool QGraphicsLineItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsLineItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QGraphicsLineItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn FreeQGraphicsLineItem<T: QGraphicsLineItem_FreeQGraphicsLineItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsLineItem(self);
    return 1;
  }
}

pub trait QGraphicsLineItem_FreeQGraphicsLineItem {
  fn FreeQGraphicsLineItem(self, this: &mut QGraphicsLineItem) -> i32;
}

// proto: void QGraphicsLineItem::FreeQGraphicsLineItem();
impl<'a> /*trait*/ QGraphicsLineItem_FreeQGraphicsLineItem for () {
  fn FreeQGraphicsLineItem(self, this: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemD0Ev()};
    unsafe {_ZN17QGraphicsLineItemD0Ev()};
    return 1;
  }
}

