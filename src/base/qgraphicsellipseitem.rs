// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qrectf::QRectF;
use super::qgraphicsitem::QGraphicsItem;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsEllipseItem::setStartAngle(int angle);
  fn _ZN20QGraphicsEllipseItem13setStartAngleEi(arg0: c_int) -> i32;
  // proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QGraphicsEllipseItem & );
  fn _ZN20QGraphicsEllipseItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QGraphicsEllipseItem::contains(const QPointF & point);
  fn _ZNK20QGraphicsEllipseItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
  fn _ZN20QGraphicsEllipseItemC1ERK6QRectFP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsEllipseItem::setRect(const QRectF & rect);
  fn _ZN20QGraphicsEllipseItem7setRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: QRectF QGraphicsEllipseItem::rect();
  fn _ZNK20QGraphicsEllipseItem4rectEv() -> i32;
  // proto: int QGraphicsEllipseItem::spanAngle();
  fn _ZNK20QGraphicsEllipseItem9spanAngleEv() -> i32;
  // proto: int QGraphicsEllipseItem::startAngle();
  fn _ZNK20QGraphicsEllipseItem10startAngleEv() -> i32;
  // proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
  fn _ZN20QGraphicsEllipseItemC1EddddP13QGraphicsItem(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> i32;
  // proto: void QGraphicsEllipseItem::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN20QGraphicsEllipseItem7setRectEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsEllipseItem::setSpanAngle(int angle);
  fn _ZN20QGraphicsEllipseItem12setSpanAngleEi(arg0: c_int) -> i32;
  // proto: int QGraphicsEllipseItem::type_();
  fn _ZNK20QGraphicsEllipseItem4typeEv() -> i32;
  // proto: QRectF QGraphicsEllipseItem::boundingRect();
  fn _ZNK20QGraphicsEllipseItem12boundingRectEv() -> i32;
  // proto: QPainterPath QGraphicsEllipseItem::shape();
  fn _ZNK20QGraphicsEllipseItem5shapeEv() -> i32;
  // proto: void QGraphicsEllipseItem::FreeQGraphicsEllipseItem();
  fn _ZN20QGraphicsEllipseItemD0Ev() -> i32;
  // proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(QGraphicsItem * parent);
  fn _ZN20QGraphicsEllipseItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QPainterPath QGraphicsEllipseItem::opaqueArea();
  fn _ZNK20QGraphicsEllipseItem10opaqueAreaEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsEllipseItem)=1
pub struct QGraphicsEllipseItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn setStartAngle<T: QGraphicsEllipseItem_setStartAngle>(&mut self, value: T) -> i32 {
    value.setStartAngle(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_setStartAngle {
  fn setStartAngle(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: void QGraphicsEllipseItem::setStartAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setStartAngle for (i32) {
  fn setStartAngle(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem13setStartAngleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QGraphicsEllipseItem13setStartAngleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn NewQGraphicsEllipseItem<T: QGraphicsEllipseItem_NewQGraphicsEllipseItem>(value: T) -> QGraphicsEllipseItem {
    let rsthis = value.NewQGraphicsEllipseItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_NewQGraphicsEllipseItem {
  fn NewQGraphicsEllipseItem(self) -> QGraphicsEllipseItem;
}

// proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QGraphicsEllipseItem & );
impl<'a> /*trait*/ QGraphicsEllipseItem_NewQGraphicsEllipseItem for (&'a  QGraphicsEllipseItem) {
  fn NewQGraphicsEllipseItem(self) -> QGraphicsEllipseItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QGraphicsEllipseItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsEllipseItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn contains<T: QGraphicsEllipseItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_contains {
  fn contains(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: bool QGraphicsEllipseItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsEllipseItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QGraphicsEllipseItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_NewQGraphicsEllipseItem for (&'a  QRectF, &'a mut QGraphicsItem) {
  fn NewQGraphicsEllipseItem(self) -> QGraphicsEllipseItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC1ERK6QRectFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsEllipseItemC1ERK6QRectFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsEllipseItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn setRect<T: QGraphicsEllipseItem_setRect>(&mut self, value: T) -> i32 {
    value.setRect(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_setRect {
  fn setRect(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: void QGraphicsEllipseItem::setRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect for (&'a  QRectF) {
  fn setRect(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QGraphicsEllipseItem7setRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn paint<T: QGraphicsEllipseItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_paint {
  fn paint(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsEllipseItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn isObscuredBy<T: QGraphicsEllipseItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsEllipseItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn rect<T: QGraphicsEllipseItem_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_rect {
  fn rect(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: QRectF QGraphicsEllipseItem::rect();
impl<'a> /*trait*/ QGraphicsEllipseItem_rect for () {
  fn rect(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4rectEv()};
    unsafe {_ZNK20QGraphicsEllipseItem4rectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn spanAngle<T: QGraphicsEllipseItem_spanAngle>(&mut self, value: T) -> i32 {
    value.spanAngle(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_spanAngle {
  fn spanAngle(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: int QGraphicsEllipseItem::spanAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_spanAngle for () {
  fn spanAngle(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem9spanAngleEv()};
    unsafe {_ZNK20QGraphicsEllipseItem9spanAngleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn startAngle<T: QGraphicsEllipseItem_startAngle>(&mut self, value: T) -> i32 {
    value.startAngle(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_startAngle {
  fn startAngle(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: int QGraphicsEllipseItem::startAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_startAngle for () {
  fn startAngle(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10startAngleEv()};
    unsafe {_ZNK20QGraphicsEllipseItem10startAngleEv()};
    return 1;
  }
}

// proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_NewQGraphicsEllipseItem for (f64, f64, f64, f64, &'a mut QGraphicsItem) {
  fn NewQGraphicsEllipseItem(self) -> QGraphicsEllipseItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC1EddddP13QGraphicsItem()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsEllipseItemC1EddddP13QGraphicsItem(qthis, arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsEllipseItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsEllipseItem::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect for (f64, f64, f64, f64) {
  fn setRect(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN20QGraphicsEllipseItem7setRectEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn setSpanAngle<T: QGraphicsEllipseItem_setSpanAngle>(&mut self, value: T) -> i32 {
    value.setSpanAngle(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_setSpanAngle {
  fn setSpanAngle(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: void QGraphicsEllipseItem::setSpanAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setSpanAngle for (i32) {
  fn setSpanAngle(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem12setSpanAngleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QGraphicsEllipseItem12setSpanAngleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn type_<T: QGraphicsEllipseItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_type_ {
  fn type_(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: int QGraphicsEllipseItem::type_();
impl<'a> /*trait*/ QGraphicsEllipseItem_type_ for () {
  fn type_(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4typeEv()};
    unsafe {_ZNK20QGraphicsEllipseItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn boundingRect<T: QGraphicsEllipseItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: QRectF QGraphicsEllipseItem::boundingRect();
impl<'a> /*trait*/ QGraphicsEllipseItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem12boundingRectEv()};
    unsafe {_ZNK20QGraphicsEllipseItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn shape<T: QGraphicsEllipseItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_shape {
  fn shape(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: QPainterPath QGraphicsEllipseItem::shape();
impl<'a> /*trait*/ QGraphicsEllipseItem_shape for () {
  fn shape(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem5shapeEv()};
    unsafe {_ZNK20QGraphicsEllipseItem5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn FreeQGraphicsEllipseItem<T: QGraphicsEllipseItem_FreeQGraphicsEllipseItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsEllipseItem(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_FreeQGraphicsEllipseItem {
  fn FreeQGraphicsEllipseItem(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: void QGraphicsEllipseItem::FreeQGraphicsEllipseItem();
impl<'a> /*trait*/ QGraphicsEllipseItem_FreeQGraphicsEllipseItem for () {
  fn FreeQGraphicsEllipseItem(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemD0Ev()};
    unsafe {_ZN20QGraphicsEllipseItemD0Ev()};
    return 1;
  }
}

// proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_NewQGraphicsEllipseItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsEllipseItem(self) -> QGraphicsEllipseItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsEllipseItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsEllipseItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn opaqueArea<T: QGraphicsEllipseItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsEllipseItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsEllipseItem) -> i32;
}

// proto: QPainterPath QGraphicsEllipseItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsEllipseItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10opaqueAreaEv()};
    unsafe {_ZNK20QGraphicsEllipseItem10opaqueAreaEv()};
    return 1;
  }
}

