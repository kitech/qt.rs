// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qgraphicsitem::QGraphicsItem;
use super::qfont::QFont;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QGraphicsSimpleTextItem::type_();
  fn _ZNK23QGraphicsSimpleTextItem4typeEv() -> i32;
  // proto: QFont QGraphicsSimpleTextItem::font();
  fn _ZNK23QGraphicsSimpleTextItem4fontEv() -> i32;
  // proto: void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsSimpleTextItem::FreeQGraphicsSimpleTextItem();
  fn _ZN23QGraphicsSimpleTextItemD0Ev() -> i32;
  // proto: void QGraphicsSimpleTextItem::setText(const QString & text);
  fn _ZN23QGraphicsSimpleTextItem7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QGraphicsSimpleTextItem::text();
  fn _ZNK23QGraphicsSimpleTextItem4textEv() -> i32;
  // proto: void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(const QString & text, QGraphicsItem * parent);
  fn _ZN23QGraphicsSimpleTextItemC1ERK7QStringP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(const QGraphicsSimpleTextItem & );
  fn _ZN23QGraphicsSimpleTextItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsSimpleTextItem::shape();
  fn _ZNK23QGraphicsSimpleTextItem5shapeEv() -> i32;
  // proto: void QGraphicsSimpleTextItem::NewQGraphicsSimpleTextItem(QGraphicsItem * parent);
  fn _ZN23QGraphicsSimpleTextItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsSimpleTextItem::setFont(const QFont & font);
  fn _ZN23QGraphicsSimpleTextItem7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsSimpleTextItem::opaqueArea();
  fn _ZNK23QGraphicsSimpleTextItem10opaqueAreaEv() -> i32;
  // proto: QRectF QGraphicsSimpleTextItem::boundingRect();
  fn _ZNK23QGraphicsSimpleTextItem12boundingRectEv() -> i32;
  // proto: bool QGraphicsSimpleTextItem::contains(const QPointF & point);
  fn _ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsSimpleTextItem)=1
pub struct QGraphicsSimpleTextItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn type_<T: QGraphicsSimpleTextItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_type_ {
  fn type_(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: int QGraphicsSimpleTextItem::type_();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_type_ for () {
  fn type_(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4typeEv()};
    unsafe {_ZNK23QGraphicsSimpleTextItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn font<T: QGraphicsSimpleTextItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_font {
  fn font(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: QFont QGraphicsSimpleTextItem::font();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_font for () {
  fn font(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4fontEv()};
    unsafe {_ZNK23QGraphicsSimpleTextItem4fontEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn paint<T: QGraphicsSimpleTextItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_paint {
  fn paint(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn FreeQGraphicsSimpleTextItem<T: QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsSimpleTextItem(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem {
  fn FreeQGraphicsSimpleTextItem(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: void QGraphicsSimpleTextItem::FreeQGraphicsSimpleTextItem();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_FreeQGraphicsSimpleTextItem for () {
  fn FreeQGraphicsSimpleTextItem(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemD0Ev()};
    unsafe {_ZN23QGraphicsSimpleTextItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setText<T: QGraphicsSimpleTextItem_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setText {
  fn setText(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: void QGraphicsSimpleTextItem::setText(const QString & text);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setText for (&'a  QString) {
  fn setText(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsSimpleTextItem7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn text<T: QGraphicsSimpleTextItem_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_text {
  fn text(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: QString QGraphicsSimpleTextItem::text();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_text for () {
  fn text(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4textEv()};
    unsafe {_ZNK23QGraphicsSimpleTextItem4textEv()};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsSimpleTextItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSimpleTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn isObscuredBy<T: QGraphicsSimpleTextItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn shape<T: QGraphicsSimpleTextItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_shape {
  fn shape(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: QPainterPath QGraphicsSimpleTextItem::shape();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_shape for () {
  fn shape(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem5shapeEv()};
    unsafe {_ZNK23QGraphicsSimpleTextItem5shapeEv()};
    return 1;
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
  pub fn setFont<T: QGraphicsSimpleTextItem_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setFont {
  fn setFont(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: void QGraphicsSimpleTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsSimpleTextItem7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn opaqueArea<T: QGraphicsSimpleTextItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: QPainterPath QGraphicsSimpleTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv()};
    unsafe {_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn boundingRect<T: QGraphicsSimpleTextItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: QRectF QGraphicsSimpleTextItem::boundingRect();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12boundingRectEv()};
    unsafe {_ZNK23QGraphicsSimpleTextItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn contains<T: QGraphicsSimpleTextItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsSimpleTextItem_contains {
  fn contains(self, this: &mut QGraphicsSimpleTextItem) -> i32;
}

// proto: bool QGraphicsSimpleTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

