// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsitem::QGraphicsItem;
use super::qrectf::QRectF;
use super::qpointf::QPointF;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QGraphicsRectItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: QRectF QGraphicsRectItem::boundingRect();
  fn _ZNK17QGraphicsRectItem12boundingRectEv() -> i32;
  // proto: void QGraphicsRectItem::NewQGraphicsRectItem(const QGraphicsRectItem & );
  fn _ZN17QGraphicsRectItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QGraphicsRectItem::type_();
  fn _ZNK17QGraphicsRectItem4typeEv() -> i32;
  // proto: QRectF QGraphicsRectItem::rect();
  fn _ZNK17QGraphicsRectItem4rectEv() -> i32;
  // proto: QPainterPath QGraphicsRectItem::shape();
  fn _ZNK17QGraphicsRectItem5shapeEv() -> i32;
  // proto: void QGraphicsRectItem::FreeQGraphicsRectItem();
  fn _ZN17QGraphicsRectItemD0Ev() -> i32;
  // proto: void QGraphicsRectItem::NewQGraphicsRectItem(const QRectF & rect, QGraphicsItem * parent);
  fn _ZN17QGraphicsRectItemC1ERK6QRectFP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QPainterPath QGraphicsRectItem::opaqueArea();
  fn _ZNK17QGraphicsRectItem10opaqueAreaEv() -> i32;
  // proto: void QGraphicsRectItem::setRect(const QRectF & rect);
  fn _ZN17QGraphicsRectItem7setRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsRectItem::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN17QGraphicsRectItem7setRectEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsRectItem::NewQGraphicsRectItem(QGraphicsItem * parent);
  fn _ZN17QGraphicsRectItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: bool QGraphicsRectItem::contains(const QPointF & point);
  fn _ZNK17QGraphicsRectItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsRectItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsRectItem::NewQGraphicsRectItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
  fn _ZN17QGraphicsRectItemC1EddddP13QGraphicsItem(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsRectItem)=1
pub struct QGraphicsRectItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsRectItem {
  pub fn isObscuredBy<T: QGraphicsRectItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: bool QGraphicsRectItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsRectItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn boundingRect<T: QGraphicsRectItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: QRectF QGraphicsRectItem::boundingRect();
impl<'a> /*trait*/ QGraphicsRectItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem12boundingRectEv()};
    unsafe {_ZNK17QGraphicsRectItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn NewQGraphicsRectItem<T: QGraphicsRectItem_NewQGraphicsRectItem>(value: T) -> QGraphicsRectItem {
    let rsthis = value.NewQGraphicsRectItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRectItem_NewQGraphicsRectItem {
  fn NewQGraphicsRectItem(self) -> QGraphicsRectItem;
}

// proto: void QGraphicsRectItem::NewQGraphicsRectItem(const QGraphicsRectItem & );
impl<'a> /*trait*/ QGraphicsRectItem_NewQGraphicsRectItem for (&'a  QGraphicsRectItem) {
  fn NewQGraphicsRectItem(self) -> QGraphicsRectItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsRectItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsRectItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn type_<T: QGraphicsRectItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_type_ {
  fn type_(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: int QGraphicsRectItem::type_();
impl<'a> /*trait*/ QGraphicsRectItem_type_ for () {
  fn type_(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem4typeEv()};
    unsafe {_ZNK17QGraphicsRectItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn rect<T: QGraphicsRectItem_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_rect {
  fn rect(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: QRectF QGraphicsRectItem::rect();
impl<'a> /*trait*/ QGraphicsRectItem_rect for () {
  fn rect(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem4rectEv()};
    unsafe {_ZNK17QGraphicsRectItem4rectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn shape<T: QGraphicsRectItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_shape {
  fn shape(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: QPainterPath QGraphicsRectItem::shape();
impl<'a> /*trait*/ QGraphicsRectItem_shape for () {
  fn shape(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem5shapeEv()};
    unsafe {_ZNK17QGraphicsRectItem5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn FreeQGraphicsRectItem<T: QGraphicsRectItem_FreeQGraphicsRectItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsRectItem(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_FreeQGraphicsRectItem {
  fn FreeQGraphicsRectItem(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: void QGraphicsRectItem::FreeQGraphicsRectItem();
impl<'a> /*trait*/ QGraphicsRectItem_FreeQGraphicsRectItem for () {
  fn FreeQGraphicsRectItem(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemD0Ev()};
    unsafe {_ZN17QGraphicsRectItemD0Ev()};
    return 1;
  }
}

// proto: void QGraphicsRectItem::NewQGraphicsRectItem(const QRectF & rect, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsRectItem_NewQGraphicsRectItem for (&'a  QRectF, &'a mut QGraphicsItem) {
  fn NewQGraphicsRectItem(self) -> QGraphicsRectItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC1ERK6QRectFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsRectItemC1ERK6QRectFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsRectItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn opaqueArea<T: QGraphicsRectItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: QPainterPath QGraphicsRectItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsRectItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem10opaqueAreaEv()};
    unsafe {_ZNK17QGraphicsRectItem10opaqueAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn setRect<T: QGraphicsRectItem_setRect>(&mut self, value: T) -> i32 {
    value.setRect(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_setRect {
  fn setRect(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: void QGraphicsRectItem::setRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsRectItem_setRect for (&'a  QRectF) {
  fn setRect(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItem7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsRectItem7setRectERK6QRectF(arg0)};
    return 1;
  }
}

// proto: void QGraphicsRectItem::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsRectItem_setRect for (f64, f64, f64, f64) {
  fn setRect(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItem7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN17QGraphicsRectItem7setRectEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QGraphicsRectItem::NewQGraphicsRectItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsRectItem_NewQGraphicsRectItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsRectItem(self) -> QGraphicsRectItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsRectItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsRectItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn contains<T: QGraphicsRectItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_contains {
  fn contains(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: bool QGraphicsRectItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsRectItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QGraphicsRectItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn paint<T: QGraphicsRectItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsRectItem_paint {
  fn paint(self, this: &mut QGraphicsRectItem) -> i32;
}

// proto: void QGraphicsRectItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsRectItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QGraphicsRectItem::NewQGraphicsRectItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsRectItem_NewQGraphicsRectItem for (f64, f64, f64, f64, &'a mut QGraphicsItem) {
  fn NewQGraphicsRectItem(self) -> QGraphicsRectItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC1EddddP13QGraphicsItem()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsRectItemC1EddddP13QGraphicsItem(qthis, arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsRectItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

