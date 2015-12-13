// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsitem::QGraphicsItem;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpolygonf::QPolygonF;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPainterPath QGraphicsPolygonItem::shape();
  fn _ZNK20QGraphicsPolygonItem5shapeEv() -> i32;
  // proto: bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsPolygonItem::NewQGraphicsPolygonItem(QGraphicsItem * parent);
  fn _ZN20QGraphicsPolygonItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QRectF QGraphicsPolygonItem::boundingRect();
  fn _ZNK20QGraphicsPolygonItem12boundingRectEv() -> i32;
  // proto: int QGraphicsPolygonItem::type_();
  fn _ZNK20QGraphicsPolygonItem4typeEv() -> i32;
  // proto: void QGraphicsPolygonItem::FreeQGraphicsPolygonItem();
  fn _ZN20QGraphicsPolygonItemD0Ev() -> i32;
  // proto: QPolygonF QGraphicsPolygonItem::polygon();
  fn _ZNK20QGraphicsPolygonItem7polygonEv() -> i32;
  // proto: void QGraphicsPolygonItem::NewQGraphicsPolygonItem(const QGraphicsPolygonItem & );
  fn _ZN20QGraphicsPolygonItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsPolygonItem::opaqueArea();
  fn _ZNK20QGraphicsPolygonItem10opaqueAreaEv() -> i32;
  // proto: void QGraphicsPolygonItem::NewQGraphicsPolygonItem(const QPolygonF & polygon, QGraphicsItem * parent);
  fn _ZN20QGraphicsPolygonItemC1ERK9QPolygonFP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: bool QGraphicsPolygonItem::contains(const QPointF & point);
  fn _ZNK20QGraphicsPolygonItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
  fn _ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsPolygonItem)=1
pub struct QGraphicsPolygonItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn shape<T: QGraphicsPolygonItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_shape {
  fn shape(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: QPainterPath QGraphicsPolygonItem::shape();
impl<'a> /*trait*/ QGraphicsPolygonItem_shape for () {
  fn shape(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem5shapeEv()};
    unsafe {_ZNK20QGraphicsPolygonItem5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn isObscuredBy<T: QGraphicsPolygonItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPolygonItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn paint<T: QGraphicsPolygonItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_paint {
  fn paint(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPolygonItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn NewQGraphicsPolygonItem<T: QGraphicsPolygonItem_NewQGraphicsPolygonItem>(value: T) -> QGraphicsPolygonItem {
    let rsthis = value.NewQGraphicsPolygonItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_NewQGraphicsPolygonItem {
  fn NewQGraphicsPolygonItem(self) -> QGraphicsPolygonItem;
}

// proto: void QGraphicsPolygonItem::NewQGraphicsPolygonItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPolygonItem_NewQGraphicsPolygonItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsPolygonItem(self) -> QGraphicsPolygonItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsPolygonItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsPolygonItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn boundingRect<T: QGraphicsPolygonItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: QRectF QGraphicsPolygonItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPolygonItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem12boundingRectEv()};
    unsafe {_ZNK20QGraphicsPolygonItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn type_<T: QGraphicsPolygonItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_type_ {
  fn type_(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: int QGraphicsPolygonItem::type_();
impl<'a> /*trait*/ QGraphicsPolygonItem_type_ for () {
  fn type_(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem4typeEv()};
    unsafe {_ZNK20QGraphicsPolygonItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn FreeQGraphicsPolygonItem<T: QGraphicsPolygonItem_FreeQGraphicsPolygonItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsPolygonItem(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_FreeQGraphicsPolygonItem {
  fn FreeQGraphicsPolygonItem(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: void QGraphicsPolygonItem::FreeQGraphicsPolygonItem();
impl<'a> /*trait*/ QGraphicsPolygonItem_FreeQGraphicsPolygonItem for () {
  fn FreeQGraphicsPolygonItem(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemD0Ev()};
    unsafe {_ZN20QGraphicsPolygonItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn polygon<T: QGraphicsPolygonItem_polygon>(&mut self, value: T) -> i32 {
    value.polygon(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_polygon {
  fn polygon(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: QPolygonF QGraphicsPolygonItem::polygon();
impl<'a> /*trait*/ QGraphicsPolygonItem_polygon for () {
  fn polygon(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem7polygonEv()};
    unsafe {_ZNK20QGraphicsPolygonItem7polygonEv()};
    return 1;
  }
}

// proto: void QGraphicsPolygonItem::NewQGraphicsPolygonItem(const QGraphicsPolygonItem & );
impl<'a> /*trait*/ QGraphicsPolygonItem_NewQGraphicsPolygonItem for (&'a  QGraphicsPolygonItem) {
  fn NewQGraphicsPolygonItem(self) -> QGraphicsPolygonItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QGraphicsPolygonItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsPolygonItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn opaqueArea<T: QGraphicsPolygonItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: QPainterPath QGraphicsPolygonItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPolygonItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem10opaqueAreaEv()};
    unsafe {_ZNK20QGraphicsPolygonItem10opaqueAreaEv()};
    return 1;
  }
}

// proto: void QGraphicsPolygonItem::NewQGraphicsPolygonItem(const QPolygonF & polygon, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPolygonItem_NewQGraphicsPolygonItem for (&'a  QPolygonF, &'a mut QGraphicsItem) {
  fn NewQGraphicsPolygonItem(self) -> QGraphicsPolygonItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC1ERK9QPolygonFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsPolygonItemC1ERK9QPolygonFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsPolygonItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn contains<T: QGraphicsPolygonItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_contains {
  fn contains(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: bool QGraphicsPolygonItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPolygonItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QGraphicsPolygonItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn setPolygon<T: QGraphicsPolygonItem_setPolygon>(&mut self, value: T) -> i32 {
    value.setPolygon(self);
    return 1;
  }
}

pub trait QGraphicsPolygonItem_setPolygon {
  fn setPolygon(self, this: &mut QGraphicsPolygonItem) -> i32;
}

// proto: void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsPolygonItem_setPolygon for (&'a  QPolygonF) {
  fn setPolygon(self, this: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF(arg0)};
    return 1;
  }
}

