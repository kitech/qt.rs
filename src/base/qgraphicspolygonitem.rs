// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpainterpath::QPainterPath;
use super::qgraphicsitem::QGraphicsItem;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qrectf::QRectF;
use super::qpolygonf::QPolygonF;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPainterPath QGraphicsPolygonItem::shape();
  fn _ZNK20QGraphicsPolygonItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem * parent);
  fn _ZN20QGraphicsPolygonItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QGraphicsPolygonItem::boundingRect();
  fn _ZNK20QGraphicsPolygonItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QGraphicsPolygonItem::type();
  fn _ZNK20QGraphicsPolygonItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsPolygonItem::~QGraphicsPolygonItem();
  fn _ZN20QGraphicsPolygonItemD0Ev(qthis: *mut c_void);
  // proto:  QPolygonF QGraphicsPolygonItem::polygon();
  fn _ZNK20QGraphicsPolygonItem7polygonEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(const QGraphicsPolygonItem & );
  fn _ZN20QGraphicsPolygonItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsPolygonItem::opaqueArea();
  fn _ZNK20QGraphicsPolygonItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(const QPolygonF & polygon, QGraphicsItem * parent);
  fn _ZN20QGraphicsPolygonItemC1ERK9QPolygonFP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QGraphicsPolygonItem::contains(const QPointF & point);
  fn _ZNK20QGraphicsPolygonItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
  fn _ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QGraphicsPolygonItem)=1
pub struct QGraphicsPolygonItem {
  pub qclsinst: *mut c_void,
}

  // proto:  QPainterPath QGraphicsPolygonItem::shape();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn shape<RetType, T: QGraphicsPolygonItem_shape<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_shape<RetType> {
  fn shape(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPolygonItem::shape();
impl<'a> /*trait*/ QGraphicsPolygonItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: &mut QGraphicsPolygonItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem5shapeEv()};
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn isObscuredBy<RetType, T: QGraphicsPolygonItem_isObscuredBy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPolygonItem_isObscuredBy<i8> for (QGraphicsItem) {
  fn isObscuredBy(self , rsthis: &mut QGraphicsPolygonItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn paint<RetType, T: QGraphicsPolygonItem_paint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_paint<RetType> {
  fn paint(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPolygonItem_paint<()> for (QPainter, QStyleOptionGraphicsItem, QWidget) {
  fn paint(self , rsthis: &mut QGraphicsPolygonItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem * parent);
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

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPolygonItem_NewQGraphicsPolygonItem for (QGraphicsItem) {
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

  // proto:  QRectF QGraphicsPolygonItem::boundingRect();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn boundingRect<RetType, T: QGraphicsPolygonItem_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  QRectF QGraphicsPolygonItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPolygonItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QGraphicsPolygonItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QGraphicsPolygonItem::type();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn type_<RetType, T: QGraphicsPolygonItem_type_<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_type_<RetType> {
  fn type_(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  int QGraphicsPolygonItem::type();
impl<'a> /*trait*/ QGraphicsPolygonItem_type_<i32> for () {
  fn type_(self , rsthis: &mut QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem4typeEv()};
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::~QGraphicsPolygonItem();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn FreeQGraphicsPolygonItem<RetType, T: QGraphicsPolygonItem_FreeQGraphicsPolygonItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsPolygonItem(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_FreeQGraphicsPolygonItem<RetType> {
  fn FreeQGraphicsPolygonItem(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  void QGraphicsPolygonItem::~QGraphicsPolygonItem();
impl<'a> /*trait*/ QGraphicsPolygonItem_FreeQGraphicsPolygonItem<()> for () {
  fn FreeQGraphicsPolygonItem(self , rsthis: &mut QGraphicsPolygonItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemD0Ev()};
     unsafe {_ZN20QGraphicsPolygonItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsPolygonItem::polygon();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn polygon<RetType, T: QGraphicsPolygonItem_polygon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.polygon(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_polygon<RetType> {
  fn polygon(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsPolygonItem::polygon();
impl<'a> /*trait*/ QGraphicsPolygonItem_polygon<QPolygonF> for () {
  fn polygon(self , rsthis: &mut QGraphicsPolygonItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem7polygonEv()};
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem7polygonEv(rsthis.qclsinst)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(const QGraphicsPolygonItem & );
impl<'a> /*trait*/ QGraphicsPolygonItem_NewQGraphicsPolygonItem for (QGraphicsPolygonItem) {
  fn NewQGraphicsPolygonItem(self) -> QGraphicsPolygonItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsPolygonItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsPolygonItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPolygonItem::opaqueArea();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn opaqueArea<RetType, T: QGraphicsPolygonItem_opaqueArea<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPolygonItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPolygonItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: &mut QGraphicsPolygonItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(const QPolygonF & polygon, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPolygonItem_NewQGraphicsPolygonItem for (QPolygonF, QGraphicsItem) {
  fn NewQGraphicsPolygonItem(self) -> QGraphicsPolygonItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC1ERK9QPolygonFP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN20QGraphicsPolygonItemC1ERK9QPolygonFP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsPolygonItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsPolygonItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn contains<RetType, T: QGraphicsPolygonItem_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_contains<RetType> {
  fn contains(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  bool QGraphicsPolygonItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPolygonItem_contains<i8> for (QPointF) {
  fn contains(self , rsthis: &mut QGraphicsPolygonItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QGraphicsPolygonItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn setPolygon<RetType, T: QGraphicsPolygonItem_setPolygon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPolygon(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_setPolygon<RetType> {
  fn setPolygon(self , rsthis: &mut QGraphicsPolygonItem) -> RetType;
}

  // proto:  void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsPolygonItem_setPolygon<()> for (QPolygonF) {
  fn setPolygon(self , rsthis: &mut QGraphicsPolygonItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

