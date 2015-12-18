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
use super::qpainterpath::QPainterPath;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpointf::QPointF;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsPixmapItem::NewQGraphicsPixmapItem(QGraphicsItem * parent);
  fn _ZN19QGraphicsPixmapItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QPixmap & pixmap, QGraphicsItem * parent);
  fn _ZN19QGraphicsPixmapItemC1ERK7QPixmapP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsPixmapItem::FreeQGraphicsPixmapItem();
  fn _ZN19QGraphicsPixmapItemD0Ev(qthis: *mut c_void) ;
  // proto:  QPainterPath QGraphicsPixmapItem::opaqueArea();
  fn _ZNK19QGraphicsPixmapItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QGraphicsPixmapItem::type_();
  fn _ZNK19QGraphicsPixmapItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QPainterPath QGraphicsPixmapItem::shape();
  fn _ZNK19QGraphicsPixmapItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPixmap QGraphicsPixmapItem::pixmap();
  fn _ZNK19QGraphicsPixmapItem6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
  fn _ZN19QGraphicsPixmapItem9setOffsetEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QGraphicsPixmapItem & );
  fn _ZN19QGraphicsPixmapItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsPixmapItem::offset();
  fn _ZNK19QGraphicsPixmapItem6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsPixmapItem::boundingRect();
  fn _ZNK19QGraphicsPixmapItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsPixmapItem::contains(const QPointF & point);
  fn _ZNK19QGraphicsPixmapItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
  fn _ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsPixmapItem::setOffset(const QPointF & offset);
  fn _ZN19QGraphicsPixmapItem9setOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsPixmapItemC1ERK7QPixmapP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsPixmapItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn FreeQGraphicsPixmapItem<RetType, T: QGraphicsPixmapItem_FreeQGraphicsPixmapItem<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsPixmapItem(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_FreeQGraphicsPixmapItem<RetType> {
  fn FreeQGraphicsPixmapItem(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  void QGraphicsPixmapItem::FreeQGraphicsPixmapItem();
impl<'a> /*trait*/ QGraphicsPixmapItem_FreeQGraphicsPixmapItem<()> for () {
  fn FreeQGraphicsPixmapItem(self, rsthis: &mut QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemD0Ev()};
     unsafe {_ZN19QGraphicsPixmapItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn opaqueArea<RetType, T: QGraphicsPixmapItem_opaqueArea<RetType>>(&mut self, value: T) -> RetType {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_opaqueArea<RetType> {
  fn opaqueArea(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  QPainterPath QGraphicsPixmapItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPixmapItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsPixmapItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn isObscuredBy<RetType, T: QGraphicsPixmapItem_isObscuredBy<RetType>>(&mut self, value: T) -> RetType {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_isObscuredBy<RetType> {
  fn isObscuredBy(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPixmapItem_isObscuredBy<i8> for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsPixmapItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn type_<RetType, T: QGraphicsPixmapItem_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_type_<RetType> {
  fn type_(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  int QGraphicsPixmapItem::type_();
impl<'a> /*trait*/ QGraphicsPixmapItem_type_<i32> for () {
  fn type_(self, rsthis: &mut QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem4typeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn shape<RetType, T: QGraphicsPixmapItem_shape<RetType>>(&mut self, value: T) -> RetType {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_shape<RetType> {
  fn shape(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  QPainterPath QGraphicsPixmapItem::shape();
impl<'a> /*trait*/ QGraphicsPixmapItem_shape<QPainterPath> for () {
  fn shape(self, rsthis: &mut QGraphicsPixmapItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem5shapeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn pixmap<RetType, T: QGraphicsPixmapItem_pixmap<RetType>>(&mut self, value: T) -> RetType {
    return value.pixmap(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_pixmap<RetType> {
  fn pixmap(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  QPixmap QGraphicsPixmapItem::pixmap();
impl<'a> /*trait*/ QGraphicsPixmapItem_pixmap<QPixmap> for () {
  fn pixmap(self, rsthis: &mut QGraphicsPixmapItem) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem6pixmapEv()};
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn setOffset<RetType, T: QGraphicsPixmapItem_setOffset<RetType>>(&mut self, value: T) -> RetType {
    return value.setOffset(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_setOffset<RetType> {
  fn setOffset(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsPixmapItem_setOffset<()> for (f64, f64) {
  fn setOffset(self, rsthis: &mut QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsPixmapItem9setOffsetEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn paint<RetType, T: QGraphicsPixmapItem_paint<RetType>>(&mut self, value: T) -> RetType {
    return value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_paint<RetType> {
  fn paint(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPixmapItem_paint<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto: void QGraphicsPixmapItem::NewQGraphicsPixmapItem(const QGraphicsPixmapItem & );
impl<'a> /*trait*/ QGraphicsPixmapItem_NewQGraphicsPixmapItem for (&'a  QGraphicsPixmapItem) {
  fn NewQGraphicsPixmapItem(self) -> QGraphicsPixmapItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsPixmapItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsPixmapItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn offset<RetType, T: QGraphicsPixmapItem_offset<RetType>>(&mut self, value: T) -> RetType {
    return value.offset(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_offset<RetType> {
  fn offset(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  QPointF QGraphicsPixmapItem::offset();
impl<'a> /*trait*/ QGraphicsPixmapItem_offset<QPointF> for () {
  fn offset(self, rsthis: &mut QGraphicsPixmapItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem6offsetEv()};
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn boundingRect<RetType, T: QGraphicsPixmapItem_boundingRect<RetType>>(&mut self, value: T) -> RetType {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_boundingRect<RetType> {
  fn boundingRect(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  QRectF QGraphicsPixmapItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPixmapItem_boundingRect<QRectF> for () {
  fn boundingRect(self, rsthis: &mut QGraphicsPixmapItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn contains<RetType, T: QGraphicsPixmapItem_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_contains<RetType> {
  fn contains(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  bool QGraphicsPixmapItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPixmapItem_contains<i8> for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsPixmapItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QGraphicsPixmapItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn setPixmap<RetType, T: QGraphicsPixmapItem_setPixmap<RetType>>(&mut self, value: T) -> RetType {
    return value.setPixmap(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_setPixmap<RetType> {
  fn setPixmap(self, rsthis: &mut QGraphicsPixmapItem) -> RetType;
}

// proto:  void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QGraphicsPixmapItem_setPixmap<()> for (&'a  QPixmap) {
  fn setPixmap(self, rsthis: &mut QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsPixmapItem::setOffset(const QPointF & offset);
impl<'a> /*trait*/ QGraphicsPixmapItem_setOffset<()> for (&'a  QPointF) {
  fn setOffset(self, rsthis: &mut QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

