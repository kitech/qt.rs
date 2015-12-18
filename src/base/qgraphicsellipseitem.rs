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
  pub fn setStartAngle<RetType, T: QGraphicsEllipseItem_setStartAngle<RetType>>(&mut self, value: T) -> RetType {
    return value.setStartAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setStartAngle<RetType> {
  fn setStartAngle(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  void QGraphicsEllipseItem::setStartAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setStartAngle<()> for (i32) {
  fn setStartAngle(self, rsthis: &mut QGraphicsEllipseItem) -> () {
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
  pub fn contains<RetType, T: QGraphicsEllipseItem_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_contains<RetType> {
  fn contains(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  bool QGraphicsEllipseItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsEllipseItem_contains<i8> for (&'a  QPointF) {
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
  pub fn setRect<RetType, T: QGraphicsEllipseItem_setRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setRect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setRect<RetType> {
  fn setRect(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  void QGraphicsEllipseItem::setRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect<()> for (&'a  QRectF) {
  fn setRect(self, rsthis: &mut QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsEllipseItem7setRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn paint<RetType, T: QGraphicsEllipseItem_paint<RetType>>(&mut self, value: T) -> RetType {
    return value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_paint<RetType> {
  fn paint(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsEllipseItem_paint<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsEllipseItem) -> () {
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
  pub fn isObscuredBy<RetType, T: QGraphicsEllipseItem_isObscuredBy<RetType>>(&mut self, value: T) -> RetType {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_isObscuredBy<RetType> {
  fn isObscuredBy(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsEllipseItem_isObscuredBy<i8> for (&'a  QGraphicsItem) {
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
  pub fn rect<RetType, T: QGraphicsEllipseItem_rect<RetType>>(&mut self, value: T) -> RetType {
    return value.rect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_rect<RetType> {
  fn rect(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  QRectF QGraphicsEllipseItem::rect();
impl<'a> /*trait*/ QGraphicsEllipseItem_rect<QRectF> for () {
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
  pub fn spanAngle<RetType, T: QGraphicsEllipseItem_spanAngle<RetType>>(&mut self, value: T) -> RetType {
    return value.spanAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_spanAngle<RetType> {
  fn spanAngle(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  int QGraphicsEllipseItem::spanAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_spanAngle<i32> for () {
  fn spanAngle(self, rsthis: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem9spanAngleEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem9spanAngleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn startAngle<RetType, T: QGraphicsEllipseItem_startAngle<RetType>>(&mut self, value: T) -> RetType {
    return value.startAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_startAngle<RetType> {
  fn startAngle(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  int QGraphicsEllipseItem::startAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_startAngle<i32> for () {
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
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self, rsthis: &mut QGraphicsEllipseItem) -> () {
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
  pub fn setSpanAngle<RetType, T: QGraphicsEllipseItem_setSpanAngle<RetType>>(&mut self, value: T) -> RetType {
    return value.setSpanAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setSpanAngle<RetType> {
  fn setSpanAngle(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  void QGraphicsEllipseItem::setSpanAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setSpanAngle<()> for (i32) {
  fn setSpanAngle(self, rsthis: &mut QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem12setSpanAngleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QGraphicsEllipseItem12setSpanAngleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn type_<RetType, T: QGraphicsEllipseItem_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_type_<RetType> {
  fn type_(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  int QGraphicsEllipseItem::type_();
impl<'a> /*trait*/ QGraphicsEllipseItem_type_<i32> for () {
  fn type_(self, rsthis: &mut QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4typeEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn boundingRect<RetType, T: QGraphicsEllipseItem_boundingRect<RetType>>(&mut self, value: T) -> RetType {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_boundingRect<RetType> {
  fn boundingRect(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  QRectF QGraphicsEllipseItem::boundingRect();
impl<'a> /*trait*/ QGraphicsEllipseItem_boundingRect<QRectF> for () {
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
  pub fn shape<RetType, T: QGraphicsEllipseItem_shape<RetType>>(&mut self, value: T) -> RetType {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_shape<RetType> {
  fn shape(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  QPainterPath QGraphicsEllipseItem::shape();
impl<'a> /*trait*/ QGraphicsEllipseItem_shape<QPainterPath> for () {
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
  pub fn FreeQGraphicsEllipseItem<RetType, T: QGraphicsEllipseItem_FreeQGraphicsEllipseItem<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsEllipseItem(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_FreeQGraphicsEllipseItem<RetType> {
  fn FreeQGraphicsEllipseItem(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  void QGraphicsEllipseItem::FreeQGraphicsEllipseItem();
impl<'a> /*trait*/ QGraphicsEllipseItem_FreeQGraphicsEllipseItem<()> for () {
  fn FreeQGraphicsEllipseItem(self, rsthis: &mut QGraphicsEllipseItem) -> () {
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
  pub fn opaqueArea<RetType, T: QGraphicsEllipseItem_opaqueArea<RetType>>(&mut self, value: T) -> RetType {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_opaqueArea<RetType> {
  fn opaqueArea(self, rsthis: &mut QGraphicsEllipseItem) -> RetType;
}

// proto:  QPainterPath QGraphicsEllipseItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsEllipseItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsEllipseItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK20QGraphicsEllipseItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

