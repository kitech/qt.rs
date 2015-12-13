// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcursor::QTextCursor;
use super::qstring::QString;
use super::qgraphicsitem::QGraphicsItem;
use super::qtextdocument::QTextDocument;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qfont::QFont;
use super::qcolor::QColor;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QGraphicsTextItem::openExternalLinks();
  fn _ZNK17QGraphicsTextItem17openExternalLinksEv() -> i32;
  // proto: double QGraphicsTextItem::textWidth();
  fn _ZNK17QGraphicsTextItem9textWidthEv() -> i32;
  // proto: void QGraphicsTextItem::setTextWidth(qreal width);
  fn _ZN17QGraphicsTextItem12setTextWidthEd(arg0: c_double) -> i32;
  // proto: void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
  fn _ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: int QGraphicsTextItem::type_();
  fn _ZNK17QGraphicsTextItem4typeEv() -> i32;
  // proto: QFont QGraphicsTextItem::font();
  fn _ZNK17QGraphicsTextItem4fontEv() -> i32;
  // proto: void QGraphicsTextItem::NewQGraphicsTextItem(const QString & text, QGraphicsItem * parent);
  fn _ZN17QGraphicsTextItemC1ERK7QStringP13QGraphicsItem(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: const QMetaObject * QGraphicsTextItem::metaObject();
  fn _ZNK17QGraphicsTextItem10metaObjectEv() -> i32;
  // proto: void QGraphicsTextItem::setOpenExternalLinks(bool open);
  fn _ZN17QGraphicsTextItem20setOpenExternalLinksEb(arg0: int8_t) -> i32;
  // proto: void QGraphicsTextItem::setTabChangesFocus(bool b);
  fn _ZN17QGraphicsTextItem18setTabChangesFocusEb(arg0: int8_t) -> i32;
  // proto: QString QGraphicsTextItem::toHtml();
  fn _ZNK17QGraphicsTextItem6toHtmlEv() -> i32;
  // proto: void QGraphicsTextItem::setDocument(QTextDocument * document);
  fn _ZN17QGraphicsTextItem11setDocumentEP13QTextDocument(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsTextItem::setPlainText(const QString & text);
  fn _ZN17QGraphicsTextItem12setPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QGraphicsTextItem::linkHovered(const QString & );
  fn _ZN17QGraphicsTextItem11linkHoveredERK7QString(arg0: *const c_void) -> i32;
  // proto: void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsTextItem::setFont(const QFont & font);
  fn _ZN17QGraphicsTextItem7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
  fn _ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: QColor QGraphicsTextItem::defaultTextColor();
  fn _ZNK17QGraphicsTextItem16defaultTextColorEv() -> i32;
  // proto: void QGraphicsTextItem::FreeQGraphicsTextItem();
  fn _ZN17QGraphicsTextItemD0Ev() -> i32;
  // proto: QPainterPath QGraphicsTextItem::shape();
  fn _ZNK17QGraphicsTextItem5shapeEv() -> i32;
  // proto: void QGraphicsTextItem::linkActivated(const QString & );
  fn _ZN17QGraphicsTextItem13linkActivatedERK7QString(arg0: *const c_void) -> i32;
  // proto: QTextCursor QGraphicsTextItem::textCursor();
  fn _ZNK17QGraphicsTextItem10textCursorEv() -> i32;
  // proto: QRectF QGraphicsTextItem::boundingRect();
  fn _ZNK17QGraphicsTextItem12boundingRectEv() -> i32;
  // proto: QString QGraphicsTextItem::toPlainText();
  fn _ZNK17QGraphicsTextItem11toPlainTextEv() -> i32;
  // proto: void QGraphicsTextItem::setHtml(const QString & html);
  fn _ZN17QGraphicsTextItem7setHtmlERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsTextItem::tabChangesFocus();
  fn _ZNK17QGraphicsTextItem15tabChangesFocusEv() -> i32;
  // proto: void QGraphicsTextItem::NewQGraphicsTextItem(const QGraphicsTextItem & );
  fn _ZN17QGraphicsTextItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsTextItem::NewQGraphicsTextItem(QGraphicsItem * parent);
  fn _ZN17QGraphicsTextItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QTextDocument * QGraphicsTextItem::document();
  fn _ZNK17QGraphicsTextItem8documentEv() -> i32;
  // proto: bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsTextItem::opaqueArea();
  fn _ZNK17QGraphicsTextItem10opaqueAreaEv() -> i32;
  // proto: bool QGraphicsTextItem::contains(const QPointF & point);
  fn _ZNK17QGraphicsTextItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsTextItem::adjustSize();
  fn _ZN17QGraphicsTextItem10adjustSizeEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsTextItem)=1
pub struct QGraphicsTextItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsTextItem {
  pub fn openExternalLinks<T: QGraphicsTextItem_openExternalLinks>(&mut self, value: T) -> i32 {
    value.openExternalLinks(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_openExternalLinks {
  fn openExternalLinks(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: bool QGraphicsTextItem::openExternalLinks();
impl<'a> /*trait*/ QGraphicsTextItem_openExternalLinks for () {
  fn openExternalLinks(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem17openExternalLinksEv()};
    unsafe {_ZNK17QGraphicsTextItem17openExternalLinksEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn textWidth<T: QGraphicsTextItem_textWidth>(&mut self, value: T) -> i32 {
    value.textWidth(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_textWidth {
  fn textWidth(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: double QGraphicsTextItem::textWidth();
impl<'a> /*trait*/ QGraphicsTextItem_textWidth for () {
  fn textWidth(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem9textWidthEv()};
    unsafe {_ZNK17QGraphicsTextItem9textWidthEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setTextWidth<T: QGraphicsTextItem_setTextWidth>(&mut self, value: T) -> i32 {
    value.setTextWidth(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setTextWidth {
  fn setTextWidth(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setTextWidth(qreal width);
impl<'a> /*trait*/ QGraphicsTextItem_setTextWidth for (f64) {
  fn setTextWidth(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem12setTextWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN17QGraphicsTextItem12setTextWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setTextCursor<T: QGraphicsTextItem_setTextCursor>(&mut self, value: T) -> i32 {
    value.setTextCursor(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setTextCursor {
  fn setTextCursor(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QGraphicsTextItem_setTextCursor for (&'a  QTextCursor) {
  fn setTextCursor(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn type_<T: QGraphicsTextItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_type_ {
  fn type_(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: int QGraphicsTextItem::type_();
impl<'a> /*trait*/ QGraphicsTextItem_type_ for () {
  fn type_(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem4typeEv()};
    unsafe {_ZNK17QGraphicsTextItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn font<T: QGraphicsTextItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_font {
  fn font(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QFont QGraphicsTextItem::font();
impl<'a> /*trait*/ QGraphicsTextItem_font for () {
  fn font(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem4fontEv()};
    unsafe {_ZNK17QGraphicsTextItem4fontEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn NewQGraphicsTextItem<T: QGraphicsTextItem_NewQGraphicsTextItem>(value: T) -> QGraphicsTextItem {
    let rsthis = value.NewQGraphicsTextItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTextItem_NewQGraphicsTextItem {
  fn NewQGraphicsTextItem(self) -> QGraphicsTextItem;
}

// proto: void QGraphicsTextItem::NewQGraphicsTextItem(const QString & text, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsTextItem_NewQGraphicsTextItem for (&'a  QString, &'a mut QGraphicsItem) {
  fn NewQGraphicsTextItem(self) -> QGraphicsTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemC1ERK7QStringP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsTextItemC1ERK7QStringP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn metaObject<T: QGraphicsTextItem_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_metaObject {
  fn metaObject(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: const QMetaObject * QGraphicsTextItem::metaObject();
impl<'a> /*trait*/ QGraphicsTextItem_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10metaObjectEv()};
    unsafe {_ZNK17QGraphicsTextItem10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setOpenExternalLinks<T: QGraphicsTextItem_setOpenExternalLinks>(&mut self, value: T) -> i32 {
    value.setOpenExternalLinks(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setOpenExternalLinks {
  fn setOpenExternalLinks(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QGraphicsTextItem_setOpenExternalLinks for (i8) {
  fn setOpenExternalLinks(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem20setOpenExternalLinksEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN17QGraphicsTextItem20setOpenExternalLinksEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setTabChangesFocus<T: QGraphicsTextItem_setTabChangesFocus>(&mut self, value: T) -> i32 {
    value.setTabChangesFocus(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setTabChangesFocus {
  fn setTabChangesFocus(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QGraphicsTextItem_setTabChangesFocus for (i8) {
  fn setTabChangesFocus(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem18setTabChangesFocusEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN17QGraphicsTextItem18setTabChangesFocusEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn toHtml<T: QGraphicsTextItem_toHtml>(&mut self, value: T) -> i32 {
    value.toHtml(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_toHtml {
  fn toHtml(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QString QGraphicsTextItem::toHtml();
impl<'a> /*trait*/ QGraphicsTextItem_toHtml for () {
  fn toHtml(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem6toHtmlEv()};
    unsafe {_ZNK17QGraphicsTextItem6toHtmlEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setDocument<T: QGraphicsTextItem_setDocument>(&mut self, value: T) -> i32 {
    value.setDocument(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setDocument {
  fn setDocument(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QGraphicsTextItem_setDocument for (&'a mut QTextDocument) {
  fn setDocument(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setPlainText<T: QGraphicsTextItem_setPlainText>(&mut self, value: T) -> i32 {
    value.setPlainText(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setPlainText {
  fn setPlainText(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setPlainText(const QString & text);
impl<'a> /*trait*/ QGraphicsTextItem_setPlainText for (&'a  QString) {
  fn setPlainText(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem12setPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn linkHovered<T: QGraphicsTextItem_linkHovered>(&mut self, value: T) -> i32 {
    value.linkHovered(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_linkHovered {
  fn linkHovered(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::linkHovered(const QString & );
impl<'a> /*trait*/ QGraphicsTextItem_linkHovered for (&'a  QString) {
  fn linkHovered(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem11linkHoveredERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem11linkHoveredERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn paint<T: QGraphicsTextItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_paint {
  fn paint(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsTextItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setFont<T: QGraphicsTextItem_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setFont {
  fn setFont(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsTextItem_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setDefaultTextColor<T: QGraphicsTextItem_setDefaultTextColor>(&mut self, value: T) -> i32 {
    value.setDefaultTextColor(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setDefaultTextColor {
  fn setDefaultTextColor(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsTextItem_setDefaultTextColor for (&'a  QColor) {
  fn setDefaultTextColor(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn defaultTextColor<T: QGraphicsTextItem_defaultTextColor>(&mut self, value: T) -> i32 {
    value.defaultTextColor(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_defaultTextColor {
  fn defaultTextColor(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QColor QGraphicsTextItem::defaultTextColor();
impl<'a> /*trait*/ QGraphicsTextItem_defaultTextColor for () {
  fn defaultTextColor(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem16defaultTextColorEv()};
    unsafe {_ZNK17QGraphicsTextItem16defaultTextColorEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn FreeQGraphicsTextItem<T: QGraphicsTextItem_FreeQGraphicsTextItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsTextItem(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_FreeQGraphicsTextItem {
  fn FreeQGraphicsTextItem(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::FreeQGraphicsTextItem();
impl<'a> /*trait*/ QGraphicsTextItem_FreeQGraphicsTextItem for () {
  fn FreeQGraphicsTextItem(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemD0Ev()};
    unsafe {_ZN17QGraphicsTextItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn shape<T: QGraphicsTextItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_shape {
  fn shape(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QPainterPath QGraphicsTextItem::shape();
impl<'a> /*trait*/ QGraphicsTextItem_shape for () {
  fn shape(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem5shapeEv()};
    unsafe {_ZNK17QGraphicsTextItem5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn linkActivated<T: QGraphicsTextItem_linkActivated>(&mut self, value: T) -> i32 {
    value.linkActivated(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_linkActivated {
  fn linkActivated(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::linkActivated(const QString & );
impl<'a> /*trait*/ QGraphicsTextItem_linkActivated for (&'a  QString) {
  fn linkActivated(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem13linkActivatedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem13linkActivatedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn textCursor<T: QGraphicsTextItem_textCursor>(&mut self, value: T) -> i32 {
    value.textCursor(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_textCursor {
  fn textCursor(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QTextCursor QGraphicsTextItem::textCursor();
impl<'a> /*trait*/ QGraphicsTextItem_textCursor for () {
  fn textCursor(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10textCursorEv()};
    unsafe {_ZNK17QGraphicsTextItem10textCursorEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn boundingRect<T: QGraphicsTextItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QRectF QGraphicsTextItem::boundingRect();
impl<'a> /*trait*/ QGraphicsTextItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem12boundingRectEv()};
    unsafe {_ZNK17QGraphicsTextItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn toPlainText<T: QGraphicsTextItem_toPlainText>(&mut self, value: T) -> i32 {
    value.toPlainText(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_toPlainText {
  fn toPlainText(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QString QGraphicsTextItem::toPlainText();
impl<'a> /*trait*/ QGraphicsTextItem_toPlainText for () {
  fn toPlainText(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem11toPlainTextEv()};
    unsafe {_ZNK17QGraphicsTextItem11toPlainTextEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setHtml<T: QGraphicsTextItem_setHtml>(&mut self, value: T) -> i32 {
    value.setHtml(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_setHtml {
  fn setHtml(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::setHtml(const QString & html);
impl<'a> /*trait*/ QGraphicsTextItem_setHtml for (&'a  QString) {
  fn setHtml(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItem7setHtmlERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn tabChangesFocus<T: QGraphicsTextItem_tabChangesFocus>(&mut self, value: T) -> i32 {
    value.tabChangesFocus(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_tabChangesFocus {
  fn tabChangesFocus(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: bool QGraphicsTextItem::tabChangesFocus();
impl<'a> /*trait*/ QGraphicsTextItem_tabChangesFocus for () {
  fn tabChangesFocus(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem15tabChangesFocusEv()};
    unsafe {_ZNK17QGraphicsTextItem15tabChangesFocusEv()};
    return 1;
  }
}

// proto: void QGraphicsTextItem::NewQGraphicsTextItem(const QGraphicsTextItem & );
impl<'a> /*trait*/ QGraphicsTextItem_NewQGraphicsTextItem for (&'a  QGraphicsTextItem) {
  fn NewQGraphicsTextItem(self) -> QGraphicsTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QGraphicsTextItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsTextItem::NewQGraphicsTextItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsTextItem_NewQGraphicsTextItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsTextItem(self) -> QGraphicsTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsTextItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn document<T: QGraphicsTextItem_document>(&mut self, value: T) -> i32 {
    value.document(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_document {
  fn document(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QTextDocument * QGraphicsTextItem::document();
impl<'a> /*trait*/ QGraphicsTextItem_document for () {
  fn document(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem8documentEv()};
    unsafe {_ZNK17QGraphicsTextItem8documentEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn isObscuredBy<T: QGraphicsTextItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsTextItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn opaqueArea<T: QGraphicsTextItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: QPainterPath QGraphicsTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsTextItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10opaqueAreaEv()};
    unsafe {_ZNK17QGraphicsTextItem10opaqueAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn contains<T: QGraphicsTextItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_contains {
  fn contains(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: bool QGraphicsTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsTextItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QGraphicsTextItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn adjustSize<T: QGraphicsTextItem_adjustSize>(&mut self, value: T) -> i32 {
    value.adjustSize(self);
    return 1;
  }
}

pub trait QGraphicsTextItem_adjustSize {
  fn adjustSize(self, this: &mut QGraphicsTextItem) -> i32;
}

// proto: void QGraphicsTextItem::adjustSize();
impl<'a> /*trait*/ QGraphicsTextItem_adjustSize for () {
  fn adjustSize(self, this: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem10adjustSizeEv()};
    unsafe {_ZN17QGraphicsTextItem10adjustSizeEv()};
    return 1;
  }
}

