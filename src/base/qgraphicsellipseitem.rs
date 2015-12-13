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
use super::qpainterpath::QPainterPath;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsEllipseItem::setStartAngle(int angle);
  fn _ZN20QGraphicsEllipseItem13setStartAngleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QGraphicsEllipseItem & );
  fn _ZN20QGraphicsEllipseItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsEllipseItem::contains(const QPointF & point);
  fn _ZNK20QGraphicsEllipseItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
  fn _ZN20QGraphicsEllipseItemC1ERK6QRectFP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsEllipseItem::setRect(const QRectF & rect);
  fn _ZN20QGraphicsEllipseItem7setRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QRectF QGraphicsEllipseItem::rect();
  fn _ZNK20QGraphicsEllipseItem4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QGraphicsEllipseItem::spanAngle();
  fn _ZNK20QGraphicsEllipseItem9spanAngleEv(qthis: *mut c_void) -> c_int;
  // proto:  int QGraphicsEllipseItem::startAngle();
  fn _ZNK20QGraphicsEllipseItem10startAngleEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsEllipseItem::NewQGraphicsEllipseItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
  fn _ZN20QGraphicsEllipseItemC1EddddP13QGraphicsItem(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) ;
  // proto:  void QGraphicsEllipseItem::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN20QGraphicsEllipseItem7setRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QGraphicsEllipseItem::setSpanAngle(int angle);
  fn _ZN20QGraphicsEllipseItem12setSpanAngleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QGraphicsEllipseItem::type_();
  fn _ZNK20QGraphicsEllipseItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QRectF QGraphicsEllipseItem::boundingRect();
  fn _ZNK20QGraphicsEllipseItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsEllipseItem::shape();
  fn _ZNK20QGraphicsEllipseItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsEllipseItem::FreeQGraphicsEllipseItem();
  fn _ZN20QGraphicsEllipseItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsEllipseItem::NewQGraphicsEllipseItem(QGraphicsItem * parent);
  fn _ZN20QGraphicsEllipseItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QGraphicsEllipseItem::opaqueArea();
  fn _ZNK20QGraphicsEllipseItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGraphicsEllipseItem)=1
pub struct QGraphicsEllipseItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn setStartAngle<T: QGraphicsEllipseItem_setStartAngle>(&mut self, value: T)  {
     value.setStartAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setStartAngle {
  fn setStartAngle(self, rsthis: &mut QGraphicsEllipseItem) ;
}

// proto:  void QGraphicsEllipseItem::setStartAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setStartAngle for (i32) {
  fn setStartAngle(self, rsthis: &mut QGraphicsEllipseItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem13setStartAngleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QGraphicsEllipseItem13setStartAngleEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsEllipseItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsEllipseItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn contains<T: QGraphicsEllipseItem_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_contains {
  fn contains(self, rsthis: &mut QGraphicsEllipseItem) -> i8;
}

// proto:  bool QGraphicsEllipseItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsEllipseItem_contains for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsEllipseItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QGraphicsEllipseItem::NewQGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_NewQGraphicsEllipseItem for (&'a  QRectF, &'a mut QGraphicsItem) {
  fn NewQGraphicsEllipseItem(self) -> QGraphicsEllipseItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC1ERK6QRectFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsEllipseItemC1ERK6QRectFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsEllipseItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn setRect<T: QGraphicsEllipseItem_setRect>(&mut self, value: T)  {
     value.setRect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setRect {
  fn setRect(self, rsthis: &mut QGraphicsEllipseItem) ;
}

// proto:  void QGraphicsEllipseItem::setRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect for (&'a  QRectF) {
  fn setRect(self, rsthis: &mut QGraphicsEllipseItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsEllipseItem7setRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn paint<T: QGraphicsEllipseItem_paint>(&mut self, value: T)  {
     value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_paint {
  fn paint(self, rsthis: &mut QGraphicsEllipseItem) ;
}

// proto:  void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsEllipseItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsEllipseItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn isObscuredBy<T: QGraphicsEllipseItem_isObscuredBy>(&mut self, value: T) -> i8 {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_isObscuredBy {
  fn isObscuredBy(self, rsthis: &mut QGraphicsEllipseItem) -> i8;
}

// proto:  bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsEllipseItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsEllipseItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn rect<T: QGraphicsEllipseItem_rect>(&mut self, value: T) -> QRectF {
    return value.rect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_rect {
  fn rect(self, rsthis: &mut QGraphicsEllipseItem) -> QRectF;
}

// proto:  QRectF QGraphicsEllipseItem::rect();
impl<'a> /*trait*/ QGraphicsEllipseItem_rect for () {
  fn rect(self, rsthis: &mut QGraphicsEllipseItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4rectEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn spanAngle<T: QGraphicsEllipseItem_spanAngle>(&mut self, value: T) -> i32 {
    return value.spanAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_spanAngle {
  fn spanAngle(self, rsthis: &mut QGraphicsEllipseItem) -> i32;
}

// proto:  int QGraphicsEllipseItem::spanAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_spanAngle for () {
  fn spanAngle(self, rsthis: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem9spanAngleEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem9spanAngleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn startAngle<T: QGraphicsEllipseItem_startAngle>(&mut self, value: T) -> i32 {
    return value.startAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_startAngle {
  fn startAngle(self, rsthis: &mut QGraphicsEllipseItem) -> i32;
}

// proto:  int QGraphicsEllipseItem::startAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_startAngle for () {
  fn startAngle(self, rsthis: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10startAngleEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem10startAngleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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

// proto:  void QGraphicsEllipseItem::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect for (f64, f64, f64, f64) {
  fn setRect(self, rsthis: &mut QGraphicsEllipseItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN20QGraphicsEllipseItem7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn setSpanAngle<T: QGraphicsEllipseItem_setSpanAngle>(&mut self, value: T)  {
     value.setSpanAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setSpanAngle {
  fn setSpanAngle(self, rsthis: &mut QGraphicsEllipseItem) ;
}

// proto:  void QGraphicsEllipseItem::setSpanAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setSpanAngle for (i32) {
  fn setSpanAngle(self, rsthis: &mut QGraphicsEllipseItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem12setSpanAngleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QGraphicsEllipseItem12setSpanAngleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn type_<T: QGraphicsEllipseItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_type_ {
  fn type_(self, rsthis: &mut QGraphicsEllipseItem) -> i32;
}

// proto:  int QGraphicsEllipseItem::type_();
impl<'a> /*trait*/ QGraphicsEllipseItem_type_ for () {
  fn type_(self, rsthis: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4typeEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn boundingRect<T: QGraphicsEllipseItem_boundingRect>(&mut self, value: T) -> QRectF {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_boundingRect {
  fn boundingRect(self, rsthis: &mut QGraphicsEllipseItem) -> QRectF;
}

// proto:  QRectF QGraphicsEllipseItem::boundingRect();
impl<'a> /*trait*/ QGraphicsEllipseItem_boundingRect for () {
  fn boundingRect(self, rsthis: &mut QGraphicsEllipseItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn shape<T: QGraphicsEllipseItem_shape>(&mut self, value: T) -> QPainterPath {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_shape {
  fn shape(self, rsthis: &mut QGraphicsEllipseItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsEllipseItem::shape();
impl<'a> /*trait*/ QGraphicsEllipseItem_shape for () {
  fn shape(self, rsthis: &mut QGraphicsEllipseItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem5shapeEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn FreeQGraphicsEllipseItem<T: QGraphicsEllipseItem_FreeQGraphicsEllipseItem>(&mut self, value: T)  {
     value.FreeQGraphicsEllipseItem(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_FreeQGraphicsEllipseItem {
  fn FreeQGraphicsEllipseItem(self, rsthis: &mut QGraphicsEllipseItem) ;
}

// proto:  void QGraphicsEllipseItem::FreeQGraphicsEllipseItem();
impl<'a> /*trait*/ QGraphicsEllipseItem_FreeQGraphicsEllipseItem for () {
  fn FreeQGraphicsEllipseItem(self, rsthis: &mut QGraphicsEllipseItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemD0Ev()};
     unsafe {_ZN20QGraphicsEllipseItemD0Ev(rsthis.qclsinst)};
    // return 1;
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
  pub fn opaqueArea<T: QGraphicsEllipseItem_opaqueArea>(&mut self, value: T) -> QPainterPath {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_opaqueArea {
  fn opaqueArea(self, rsthis: &mut QGraphicsEllipseItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsEllipseItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsEllipseItem_opaqueArea for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsEllipseItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

