// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsitem::QGraphicsItem;
use super::qpixmap::QPixmap;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(QGraphicsItem * parent);
  fn _ZN19QGraphicsPixmapItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QPixmap & pixmap, QGraphicsItem * parent);
  fn _ZN19QGraphicsPixmapItemC1ERK7QPixmapP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsPixmapItem::FreeQGraphicsPixmapItem();
  fn _ZN19QGraphicsPixmapItemD0Ev() -> i32;
  // proto: QPainterPath QGraphicsPixmapItem::opaqueArea();
  fn _ZNK19QGraphicsPixmapItem10opaqueAreaEv() -> i32;
  // proto: bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: int QGraphicsPixmapItem::type_();
  fn _ZNK19QGraphicsPixmapItem4typeEv() -> i32;
  // proto: QPainterPath QGraphicsPixmapItem::shape();
  fn _ZNK19QGraphicsPixmapItem5shapeEv() -> i32;
  // proto: QPixmap QGraphicsPixmapItem::pixmap();
  fn _ZNK19QGraphicsPixmapItem6pixmapEv() -> i32;
  // proto: void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
  fn _ZN19QGraphicsPixmapItem9setOffsetEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QGraphicsPixmapItem & );
  fn _ZN19QGraphicsPixmapItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsPixmapItem::offset();
  fn _ZNK19QGraphicsPixmapItem6offsetEv() -> i32;
  // proto: QRectF QGraphicsPixmapItem::boundingRect();
  fn _ZNK19QGraphicsPixmapItem12boundingRectEv() -> i32;
  // proto: bool QGraphicsPixmapItem::contains(const QPointF & point);
  fn _ZNK19QGraphicsPixmapItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
  fn _ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap(arg0: *const c_void) -> i32;
  // proto: void QGraphicsPixmapItem::setOffset(const QPointF & offset);
  fn _ZN19QGraphicsPixmapItem9setOffsetERK7QPointF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsPixmapItem)=1
pub struct QGraphicsPixmapItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn NewQGraphicsPixmapItem<T: QGraphicsPixmapItem_NewQGraphicsPixmapItem>(value: T) -> QGraphicsPixmapItem {
    let rsthis = value.NewQGraphicsPixmapItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_NewQGraphicsPixmapItem {
  fn NewQGraphicsPixmapItem(self) -> QGraphicsPixmapItem;
}

// proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPixmapItem_NewQGraphicsPixmapItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsPixmapItem(self) -> QGraphicsPixmapItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsPixmapItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsPixmapItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QPixmap & pixmap, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPixmapItem_NewQGraphicsPixmapItem for (&'a  QPixmap, &'a mut QGraphicsItem) {
  fn NewQGraphicsPixmapItem(self) -> QGraphicsPixmapItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemC1ERK7QPixmapP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsPixmapItemC1ERK7QPixmapP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsPixmapItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn FreeQGraphicsPixmapItem<T: QGraphicsPixmapItem_FreeQGraphicsPixmapItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsPixmapItem(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_FreeQGraphicsPixmapItem {
  fn FreeQGraphicsPixmapItem(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: void QGraphicsPixmapItem::FreeQGraphicsPixmapItem();
impl<'a> /*trait*/ QGraphicsPixmapItem_FreeQGraphicsPixmapItem for () {
  fn FreeQGraphicsPixmapItem(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemD0Ev()};
    unsafe {_ZN19QGraphicsPixmapItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn opaqueArea<T: QGraphicsPixmapItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: QPainterPath QGraphicsPixmapItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPixmapItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem10opaqueAreaEv()};
    unsafe {_ZNK19QGraphicsPixmapItem10opaqueAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn isObscuredBy<T: QGraphicsPixmapItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPixmapItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn type_<T: QGraphicsPixmapItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_type_ {
  fn type_(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: int QGraphicsPixmapItem::type_();
impl<'a> /*trait*/ QGraphicsPixmapItem_type_ for () {
  fn type_(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem4typeEv()};
    unsafe {_ZNK19QGraphicsPixmapItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn shape<T: QGraphicsPixmapItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_shape {
  fn shape(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: QPainterPath QGraphicsPixmapItem::shape();
impl<'a> /*trait*/ QGraphicsPixmapItem_shape for () {
  fn shape(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem5shapeEv()};
    unsafe {_ZNK19QGraphicsPixmapItem5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn pixmap<T: QGraphicsPixmapItem_pixmap>(&mut self, value: T) -> i32 {
    value.pixmap(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_pixmap {
  fn pixmap(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: QPixmap QGraphicsPixmapItem::pixmap();
impl<'a> /*trait*/ QGraphicsPixmapItem_pixmap for () {
  fn pixmap(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem6pixmapEv()};
    unsafe {_ZNK19QGraphicsPixmapItem6pixmapEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn setOffset<T: QGraphicsPixmapItem_setOffset>(&mut self, value: T) -> i32 {
    value.setOffset(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_setOffset {
  fn setOffset(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsPixmapItem_setOffset for (f64, f64) {
  fn setOffset(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsPixmapItem9setOffsetEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn paint<T: QGraphicsPixmapItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_paint {
  fn paint(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPixmapItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QGraphicsPixmapItem & );
impl<'a> /*trait*/ QGraphicsPixmapItem_NewQGraphicsPixmapItem for (&'a  QGraphicsPixmapItem) {
  fn NewQGraphicsPixmapItem(self) -> QGraphicsPixmapItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsPixmapItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsPixmapItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn offset<T: QGraphicsPixmapItem_offset>(&mut self, value: T) -> i32 {
    value.offset(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_offset {
  fn offset(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: QPointF QGraphicsPixmapItem::offset();
impl<'a> /*trait*/ QGraphicsPixmapItem_offset for () {
  fn offset(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem6offsetEv()};
    unsafe {_ZNK19QGraphicsPixmapItem6offsetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn boundingRect<T: QGraphicsPixmapItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: QRectF QGraphicsPixmapItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPixmapItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem12boundingRectEv()};
    unsafe {_ZNK19QGraphicsPixmapItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn contains<T: QGraphicsPixmapItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_contains {
  fn contains(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: bool QGraphicsPixmapItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPixmapItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QGraphicsPixmapItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn setPixmap<T: QGraphicsPixmapItem_setPixmap>(&mut self, value: T) -> i32 {
    value.setPixmap(self);
    return 1;
  }
}

pub trait QGraphicsPixmapItem_setPixmap {
  fn setPixmap(self, this: &mut QGraphicsPixmapItem) -> i32;
}

// proto: void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QGraphicsPixmapItem_setPixmap for (&'a  QPixmap) {
  fn setPixmap(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap(arg0)};
    return 1;
  }
}

// proto: void QGraphicsPixmapItem::setOffset(const QPointF & offset);
impl<'a> /*trait*/ QGraphicsPixmapItem_setOffset for (&'a  QPointF) {
  fn setOffset(self, this: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF(arg0)};
    return 1;
  }
}

