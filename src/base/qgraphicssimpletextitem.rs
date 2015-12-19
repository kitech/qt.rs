// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qgraphicsitem::QGraphicsItem;
use super::qpainterpath::QPainterPath;
use super::qrectf::QRectF;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QGraphicsSimpleTextItem::type_();
  fn _ZNK23QGraphicsSimpleTextItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QFont QGraphicsSimpleTextItem::font();
  fn _ZNK23QGraphicsSimpleTextItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsSimpleTextItem::FreeQGraphicsSimpleTextItem();
  fn _ZN23QGraphicsSimpleTextItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
  fn _ZN23QGraphicsSimpleTextItem7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QGraphicsSimpleTextItem::text();
  fn _ZNK23QGraphicsSimpleTextItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(const QString & text, QGraphicsItem * parent);
  fn _ZN23QGraphicsSimpleTextItemC1ERK7QStringP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(const QGraphicsSimpleTextItem & );
  fn _ZN23QGraphicsSimpleTextItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QPainterPath QGraphicsSimpleTextItem::shape();
  fn _ZNK23QGraphicsSimpleTextItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(QGraphicsItem * parent);
  fn _ZN23QGraphicsSimpleTextItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
  fn _ZN23QGraphicsSimpleTextItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
  fn _ZNK23QGraphicsSimpleTextItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
  fn _ZNK23QGraphicsSimpleTextItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
  fn _ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QGraphicsSimpleTextItem)=1
pub struct QGraphicsSimpleTextItem {
  pub qclsinst: *mut c_void,
}

// proto:  int QGraphicsSimpleTextItem::type_();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn type_<RetType, T: QGraphicsSimpleTextItem_type_<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_type_<RetType> {
  fn type_(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  int QGraphicsSimpleTextItem::type_();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_type_<i32> for () {
  fn type_(self , rsthis: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4typeEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QFont QGraphicsSimpleTextItem::font();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn font<RetType, T: QGraphicsSimpleTextItem_font<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_font<RetType> {
  fn font(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  QFont QGraphicsSimpleTextItem::font();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_font<QFont> for () {
  fn font(self , rsthis: &mut QGraphicsSimpleTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4fontEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn paint<RetType, T: QGraphicsSimpleTextItem_paint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_paint<RetType> {
  fn paint(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_paint<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self , rsthis: &mut QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QGraphicsSimpleTextItem::FreeQGraphicsSimpleTextItem();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn FreeQGraphicsSimpleTextItem<RetType, T: QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSimpleTextItem(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem<RetType> {
  fn FreeQGraphicsSimpleTextItem(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  void QGraphicsSimpleTextItem::FreeQGraphicsSimpleTextItem();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem<()> for () {
  fn FreeQGraphicsSimpleTextItem(self , rsthis: &mut QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemD0Ev()};
     unsafe {_ZN23QGraphicsSimpleTextItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setText<RetType, T: QGraphicsSimpleTextItem_setText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setText<RetType> {
  fn setText(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setText<()> for (&'a  QString) {
  fn setText(self , rsthis: &mut QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSimpleTextItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QGraphicsSimpleTextItem::text();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn text<RetType, T: QGraphicsSimpleTextItem_text<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_text<RetType> {
  fn text(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  QString QGraphicsSimpleTextItem::text();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_text<QString> for () {
  fn text(self , rsthis: &mut QGraphicsSimpleTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4textEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn NewQGraphicsSimpleTextItem<T: QGraphicsSimpleTextItem_NewQGraphicsSimpleTextItem>(value: T) -> QGraphicsSimpleTextItem {
    let rsthis = value.NewQGraphicsSimpleTextItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_NewQGraphicsSimpleTextItem {
  fn NewQGraphicsSimpleTextItem(self) -> QGraphicsSimpleTextItem;
}

// proto: void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(const QString & text, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_NewQGraphicsSimpleTextItem for (&'a  QString, &'a mut QGraphicsItem) {
  fn NewQGraphicsSimpleTextItem(self) -> QGraphicsSimpleTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemC1ERK7QStringP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsSimpleTextItemC1ERK7QStringP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsSimpleTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(const QGraphicsSimpleTextItem & );
impl<'a> /*trait*/ QGraphicsSimpleTextItem_NewQGraphicsSimpleTextItem for (&'a  QGraphicsSimpleTextItem) {
  fn NewQGraphicsSimpleTextItem(self) -> QGraphicsSimpleTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsSimpleTextItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSimpleTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn isObscuredBy<RetType, T: QGraphicsSimpleTextItem_isObscuredBy<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_isObscuredBy<i8> for (&'a  QGraphicsItem) {
  fn isObscuredBy(self , rsthis: &mut QGraphicsSimpleTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QPainterPath QGraphicsSimpleTextItem::shape();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn shape<RetType, T: QGraphicsSimpleTextItem_shape<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_shape<RetType> {
  fn shape(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  QPainterPath QGraphicsSimpleTextItem::shape();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: &mut QGraphicsSimpleTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem5shapeEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_NewQGraphicsSimpleTextItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsSimpleTextItem(self) -> QGraphicsSimpleTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsSimpleTextItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsSimpleTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setFont<RetType, T: QGraphicsSimpleTextItem_setFont<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setFont<RetType> {
  fn setFont(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setFont<()> for (&'a  QFont) {
  fn setFont(self , rsthis: &mut QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSimpleTextItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn opaqueArea<RetType, T: QGraphicsSimpleTextItem_opaqueArea<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: &mut QGraphicsSimpleTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn boundingRect<RetType, T: QGraphicsSimpleTextItem_boundingRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QGraphicsSimpleTextItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn contains<RetType, T: QGraphicsSimpleTextItem_contains<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_contains<RetType> {
  fn contains(self , rsthis: &mut QGraphicsSimpleTextItem) -> RetType;
}

// proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_contains<i8> for (&'a  QPointF) {
  fn contains(self , rsthis: &mut QGraphicsSimpleTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

