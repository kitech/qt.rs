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
use super::qpainterpath::QPainterPath;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsLineItem::setPen(const QPen & pen);
  fn _ZN17QGraphicsLineItem6setPenERK4QPen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLineItem::NewQGraphicsLineItem(QGraphicsItem * parent);
  fn _ZN17QGraphicsLineItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsLineItem::NewQGraphicsLineItem(const QLineF & line, QGraphicsItem * parent);
  fn _ZN17QGraphicsLineItemC1ERK6QLineFP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QLineF QGraphicsLineItem::line();
  fn _ZNK17QGraphicsLineItem4lineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsLineItem::opaqueArea();
  fn _ZNK17QGraphicsLineItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN17QGraphicsLineItem7setLineEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  QPen QGraphicsLineItem::pen();
  fn _ZNK17QGraphicsLineItem3penEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsLineItem::setLine(const QLineF & line);
  fn _ZN17QGraphicsLineItem7setLineERK6QLineF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QGraphicsLineItem::shape();
  fn _ZNK17QGraphicsLineItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  int QGraphicsLineItem::type_();
  fn _ZNK17QGraphicsLineItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsLineItem::NewQGraphicsLineItem(const QGraphicsLineItem & );
  fn _ZN17QGraphicsLineItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLineItem::NewQGraphicsLineItem(qreal x1, qreal y1, qreal x2, qreal y2, QGraphicsItem * parent);
  fn _ZN17QGraphicsLineItemC1EddddP13QGraphicsItem(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) ;
  // proto:  bool QGraphicsLineItem::contains(const QPointF & point);
  fn _ZNK17QGraphicsLineItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsLineItem::FreeQGraphicsLineItem();
  fn _ZN17QGraphicsLineItemD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsLineItem)=1
pub struct QGraphicsLineItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLineItem {
  pub fn setPen<RetType, T: QGraphicsLineItem_setPen<RetType>>(&mut self, value: T) -> RetType {
    return value.setPen(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_setPen<RetType> {
  fn setPen(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  void QGraphicsLineItem::setPen(const QPen & pen);
impl<'a> /*trait*/ QGraphicsLineItem_setPen<()> for (&'a  QPen) {
  fn setPen(self, rsthis: &mut QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsLineItem6setPenERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn isObscuredBy<RetType, T: QGraphicsLineItem_isObscuredBy<RetType>>(&mut self, value: T) -> RetType {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_isObscuredBy<RetType> {
  fn isObscuredBy(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsLineItem_isObscuredBy<i8> for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsLineItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QGraphicsLineItem::NewQGraphicsLineItem(const QLineF & line, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_NewQGraphicsLineItem for (&'a  QLineF, &'a mut QGraphicsItem) {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC1ERK6QLineFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsLineItemC1ERK6QLineFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsLineItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn line<RetType, T: QGraphicsLineItem_line<RetType>>(&mut self, value: T) -> RetType {
    return value.line(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_line<RetType> {
  fn line(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  QLineF QGraphicsLineItem::line();
impl<'a> /*trait*/ QGraphicsLineItem_line<QLineF> for () {
  fn line(self, rsthis: &mut QGraphicsLineItem) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem4lineEv()};
    let mut ret = unsafe {_ZNK17QGraphicsLineItem4lineEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn opaqueArea<RetType, T: QGraphicsLineItem_opaqueArea<RetType>>(&mut self, value: T) -> RetType {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_opaqueArea<RetType> {
  fn opaqueArea(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  QPainterPath QGraphicsLineItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsLineItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsLineItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK17QGraphicsLineItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn setLine<RetType, T: QGraphicsLineItem_setLine<RetType>>(&mut self, value: T) -> RetType {
    return value.setLine(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_setLine<RetType> {
  fn setLine(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QGraphicsLineItem_setLine<()> for (f64, f64, f64, f64) {
  fn setLine(self, rsthis: &mut QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem7setLineEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN17QGraphicsLineItem7setLineEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn pen<RetType, T: QGraphicsLineItem_pen<RetType>>(&mut self, value: T) -> RetType {
    return value.pen(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_pen<RetType> {
  fn pen(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  QPen QGraphicsLineItem::pen();
impl<'a> /*trait*/ QGraphicsLineItem_pen<QPen> for () {
  fn pen(self, rsthis: &mut QGraphicsLineItem) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem3penEv()};
    let mut ret = unsafe {_ZNK17QGraphicsLineItem3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsLineItem::setLine(const QLineF & line);
impl<'a> /*trait*/ QGraphicsLineItem_setLine<()> for (&'a  QLineF) {
  fn setLine(self, rsthis: &mut QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem7setLineERK6QLineF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsLineItem7setLineERK6QLineF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn shape<RetType, T: QGraphicsLineItem_shape<RetType>>(&mut self, value: T) -> RetType {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_shape<RetType> {
  fn shape(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  QPainterPath QGraphicsLineItem::shape();
impl<'a> /*trait*/ QGraphicsLineItem_shape<QPainterPath> for () {
  fn shape(self, rsthis: &mut QGraphicsLineItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem5shapeEv()};
    let mut ret = unsafe {_ZNK17QGraphicsLineItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn paint<RetType, T: QGraphicsLineItem_paint<RetType>>(&mut self, value: T) -> RetType {
    return value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_paint<RetType> {
  fn paint(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsLineItem_paint<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn type_<RetType, T: QGraphicsLineItem_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_type_<RetType> {
  fn type_(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  int QGraphicsLineItem::type_();
impl<'a> /*trait*/ QGraphicsLineItem_type_<i32> for () {
  fn type_(self, rsthis: &mut QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem4typeEv()};
    let mut ret = unsafe {_ZNK17QGraphicsLineItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QGraphicsLineItem::NewQGraphicsLineItem(const QGraphicsLineItem & );
impl<'a> /*trait*/ QGraphicsLineItem_NewQGraphicsLineItem for (&'a  QGraphicsLineItem) {
  fn NewQGraphicsLineItem(self) -> QGraphicsLineItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn contains<RetType, T: QGraphicsLineItem_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_contains<RetType> {
  fn contains(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  bool QGraphicsLineItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsLineItem_contains<i8> for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsLineItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QGraphicsLineItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn FreeQGraphicsLineItem<RetType, T: QGraphicsLineItem_FreeQGraphicsLineItem<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsLineItem(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_FreeQGraphicsLineItem<RetType> {
  fn FreeQGraphicsLineItem(self, rsthis: &mut QGraphicsLineItem) -> RetType;
}

// proto:  void QGraphicsLineItem::FreeQGraphicsLineItem();
impl<'a> /*trait*/ QGraphicsLineItem_FreeQGraphicsLineItem<()> for () {
  fn FreeQGraphicsLineItem(self, rsthis: &mut QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemD0Ev()};
     unsafe {_ZN17QGraphicsLineItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

