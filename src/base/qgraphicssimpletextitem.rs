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

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn type_<T: QGraphicsSimpleTextItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_type_ {
  fn type_(self, rsthis: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto:  int QGraphicsSimpleTextItem::type_();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_type_ for () {
  fn type_(self, rsthis: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4typeEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn font<T: QGraphicsSimpleTextItem_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_font {
  fn font(self, rsthis: &mut QGraphicsSimpleTextItem) -> QFont;
}

// proto:  QFont QGraphicsSimpleTextItem::font();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_font for () {
  fn font(self, rsthis: &mut QGraphicsSimpleTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4fontEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn paint<T: QGraphicsSimpleTextItem_paint>(&mut self, value: T)  {
     value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_paint {
  fn paint(self, rsthis: &mut QGraphicsSimpleTextItem) ;
}

// proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsSimpleTextItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn FreeQGraphicsSimpleTextItem<T: QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem>(&mut self, value: T)  {
     value.FreeQGraphicsSimpleTextItem(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem {
  fn FreeQGraphicsSimpleTextItem(self, rsthis: &mut QGraphicsSimpleTextItem) ;
}

// proto:  void QGraphicsSimpleTextItem::FreeQGraphicsSimpleTextItem();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem for () {
  fn FreeQGraphicsSimpleTextItem(self, rsthis: &mut QGraphicsSimpleTextItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemD0Ev()};
     unsafe {_ZN23QGraphicsSimpleTextItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setText<T: QGraphicsSimpleTextItem_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setText {
  fn setText(self, rsthis: &mut QGraphicsSimpleTextItem) ;
}

// proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QGraphicsSimpleTextItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSimpleTextItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn text<T: QGraphicsSimpleTextItem_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_text {
  fn text(self, rsthis: &mut QGraphicsSimpleTextItem) -> QString;
}

// proto:  QString QGraphicsSimpleTextItem::text();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_text for () {
  fn text(self, rsthis: &mut QGraphicsSimpleTextItem) -> QString {
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

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn isObscuredBy<T: QGraphicsSimpleTextItem_isObscuredBy>(&mut self, value: T) -> i8 {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_isObscuredBy {
  fn isObscuredBy(self, rsthis: &mut QGraphicsSimpleTextItem) -> i8;
}

// proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsSimpleTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn shape<T: QGraphicsSimpleTextItem_shape>(&mut self, value: T) -> QPainterPath {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_shape {
  fn shape(self, rsthis: &mut QGraphicsSimpleTextItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsSimpleTextItem::shape();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_shape for () {
  fn shape(self, rsthis: &mut QGraphicsSimpleTextItem) -> QPainterPath {
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

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setFont<T: QGraphicsSimpleTextItem_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setFont {
  fn setFont(self, rsthis: &mut QGraphicsSimpleTextItem) ;
}

// proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QGraphicsSimpleTextItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSimpleTextItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn opaqueArea<T: QGraphicsSimpleTextItem_opaqueArea>(&mut self, value: T) -> QPainterPath {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_opaqueArea {
  fn opaqueArea(self, rsthis: &mut QGraphicsSimpleTextItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_opaqueArea for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsSimpleTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn boundingRect<T: QGraphicsSimpleTextItem_boundingRect>(&mut self, value: T) -> QRectF {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_boundingRect {
  fn boundingRect(self, rsthis: &mut QGraphicsSimpleTextItem) -> QRectF;
}

// proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_boundingRect for () {
  fn boundingRect(self, rsthis: &mut QGraphicsSimpleTextItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn contains<T: QGraphicsSimpleTextItem_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_contains {
  fn contains(self, rsthis: &mut QGraphicsSimpleTextItem) -> i8;
}

// proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_contains for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsSimpleTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

