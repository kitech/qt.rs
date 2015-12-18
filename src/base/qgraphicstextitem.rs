// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcursor::QTextCursor;
use super::qfont::QFont;
use super::qstring::QString;
use super::qgraphicsitem::QGraphicsItem;
use super::qtextdocument::QTextDocument;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qcolor::QColor;
use super::qpainterpath::QPainterPath;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QGraphicsTextItem::openExternalLinks();
  fn _ZNK17QGraphicsTextItem17openExternalLinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QGraphicsTextItem::textWidth();
  fn _ZNK17QGraphicsTextItem9textWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsTextItem::setTextWidth(qreal width);
  fn _ZN17QGraphicsTextItem12setTextWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
  fn _ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGraphicsTextItem::type_();
  fn _ZNK17QGraphicsTextItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QFont QGraphicsTextItem::font();
  fn _ZNK17QGraphicsTextItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsTextItem::NewQGraphicsTextItem(const QString & text, QGraphicsItem * parent);
  fn _ZN17QGraphicsTextItemC1ERK7QStringP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsTextItem::metaObject();
  fn _ZNK17QGraphicsTextItem10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsTextItem::setOpenExternalLinks(bool open);
  fn _ZN17QGraphicsTextItem20setOpenExternalLinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsTextItem::setTabChangesFocus(bool b);
  fn _ZN17QGraphicsTextItem18setTabChangesFocusEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QGraphicsTextItem::toHtml();
  fn _ZNK17QGraphicsTextItem6toHtmlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsTextItem::setDocument(QTextDocument * document);
  fn _ZN17QGraphicsTextItem11setDocumentEP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsTextItem::setPlainText(const QString & text);
  fn _ZN17QGraphicsTextItem12setPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsTextItem::linkHovered(const QString & );
  fn _ZN17QGraphicsTextItem11linkHoveredERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsTextItem::setFont(const QFont & font);
  fn _ZN17QGraphicsTextItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
  fn _ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QGraphicsTextItem::defaultTextColor();
  fn _ZNK17QGraphicsTextItem16defaultTextColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsTextItem::FreeQGraphicsTextItem();
  fn _ZN17QGraphicsTextItemD0Ev(qthis: *mut c_void) ;
  // proto:  QPainterPath QGraphicsTextItem::shape();
  fn _ZNK17QGraphicsTextItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsTextItem::linkActivated(const QString & );
  fn _ZN17QGraphicsTextItem13linkActivatedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextCursor QGraphicsTextItem::textCursor();
  fn _ZNK17QGraphicsTextItem10textCursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QGraphicsTextItem::toPlainText();
  fn _ZNK17QGraphicsTextItem11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsTextItem::setHtml(const QString & html);
  fn _ZN17QGraphicsTextItem7setHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsTextItem::tabChangesFocus();
  fn _ZNK17QGraphicsTextItem15tabChangesFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsTextItem::NewQGraphicsTextItem(const QGraphicsTextItem & );
  fn _ZN17QGraphicsTextItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsTextItem::NewQGraphicsTextItem(QGraphicsItem * parent);
  fn _ZN17QGraphicsTextItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextDocument * QGraphicsTextItem::document();
  fn _ZNK17QGraphicsTextItem8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QPainterPath QGraphicsTextItem::opaqueArea();
  fn _ZNK17QGraphicsTextItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsTextItem::contains(const QPointF & point);
  fn _ZNK17QGraphicsTextItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsTextItem::adjustSize();
  fn _ZN17QGraphicsTextItem10adjustSizeEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsTextItem)=1
pub struct QGraphicsTextItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsTextItem {
  pub fn openExternalLinks<RetType, T: QGraphicsTextItem_openExternalLinks<RetType>>(&mut self, value: T) -> RetType {
    return value.openExternalLinks(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_openExternalLinks<RetType> {
  fn openExternalLinks(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  bool QGraphicsTextItem::openExternalLinks();
impl<'a> /*trait*/ QGraphicsTextItem_openExternalLinks<i8> for () {
  fn openExternalLinks(self, rsthis: &mut QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem17openExternalLinksEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn textWidth<RetType, T: QGraphicsTextItem_textWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.textWidth(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_textWidth<RetType> {
  fn textWidth(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  double QGraphicsTextItem::textWidth();
impl<'a> /*trait*/ QGraphicsTextItem_textWidth<f64> for () {
  fn textWidth(self, rsthis: &mut QGraphicsTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem9textWidthEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setTextWidth<RetType, T: QGraphicsTextItem_setTextWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextWidth(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setTextWidth<RetType> {
  fn setTextWidth(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setTextWidth(qreal width);
impl<'a> /*trait*/ QGraphicsTextItem_setTextWidth<()> for (f64) {
  fn setTextWidth(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QGraphicsTextItem12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setTextCursor<RetType, T: QGraphicsTextItem_setTextCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextCursor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setTextCursor<RetType> {
  fn setTextCursor(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QGraphicsTextItem_setTextCursor<()> for (&'a  QTextCursor) {
  fn setTextCursor(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn type_<RetType, T: QGraphicsTextItem_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_type_<RetType> {
  fn type_(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  int QGraphicsTextItem::type_();
impl<'a> /*trait*/ QGraphicsTextItem_type_<i32> for () {
  fn type_(self, rsthis: &mut QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem4typeEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn font<RetType, T: QGraphicsTextItem_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_font<RetType> {
  fn font(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QFont QGraphicsTextItem::font();
impl<'a> /*trait*/ QGraphicsTextItem_font<QFont> for () {
  fn font(self, rsthis: &mut QGraphicsTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem4fontEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsTextItemC1ERK7QStringP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsTextItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn metaObject<RetType, T: QGraphicsTextItem_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  const QMetaObject * QGraphicsTextItem::metaObject();
impl<'a> /*trait*/ QGraphicsTextItem_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10metaObjectEv()};
     unsafe {_ZNK17QGraphicsTextItem10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setOpenExternalLinks<RetType, T: QGraphicsTextItem_setOpenExternalLinks<RetType>>(&mut self, value: T) -> RetType {
    return value.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setOpenExternalLinks<RetType> {
  fn setOpenExternalLinks(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QGraphicsTextItem_setOpenExternalLinks<()> for (i8) {
  fn setOpenExternalLinks(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem20setOpenExternalLinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN17QGraphicsTextItem20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setTabChangesFocus<RetType, T: QGraphicsTextItem_setTabChangesFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setTabChangesFocus<RetType> {
  fn setTabChangesFocus(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QGraphicsTextItem_setTabChangesFocus<()> for (i8) {
  fn setTabChangesFocus(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem18setTabChangesFocusEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN17QGraphicsTextItem18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn toHtml<RetType, T: QGraphicsTextItem_toHtml<RetType>>(&mut self, value: T) -> RetType {
    return value.toHtml(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_toHtml<RetType> {
  fn toHtml(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QString QGraphicsTextItem::toHtml();
impl<'a> /*trait*/ QGraphicsTextItem_toHtml<QString> for () {
  fn toHtml(self, rsthis: &mut QGraphicsTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem6toHtmlEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem6toHtmlEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setDocument<RetType, T: QGraphicsTextItem_setDocument<RetType>>(&mut self, value: T) -> RetType {
    return value.setDocument(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setDocument<RetType> {
  fn setDocument(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QGraphicsTextItem_setDocument<()> for (&'a mut QTextDocument) {
  fn setDocument(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setPlainText<RetType, T: QGraphicsTextItem_setPlainText<RetType>>(&mut self, value: T) -> RetType {
    return value.setPlainText(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setPlainText<RetType> {
  fn setPlainText(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setPlainText(const QString & text);
impl<'a> /*trait*/ QGraphicsTextItem_setPlainText<()> for (&'a  QString) {
  fn setPlainText(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn linkHovered<RetType, T: QGraphicsTextItem_linkHovered<RetType>>(&mut self, value: T) -> RetType {
    return value.linkHovered(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_linkHovered<RetType> {
  fn linkHovered(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::linkHovered(const QString & );
impl<'a> /*trait*/ QGraphicsTextItem_linkHovered<()> for (&'a  QString) {
  fn linkHovered(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem11linkHoveredERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem11linkHoveredERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn paint<RetType, T: QGraphicsTextItem_paint<RetType>>(&mut self, value: T) -> RetType {
    return value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_paint<RetType> {
  fn paint(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsTextItem_paint<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setFont<RetType, T: QGraphicsTextItem_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setFont<RetType> {
  fn setFont(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsTextItem_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setDefaultTextColor<RetType, T: QGraphicsTextItem_setDefaultTextColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setDefaultTextColor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setDefaultTextColor<RetType> {
  fn setDefaultTextColor(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsTextItem_setDefaultTextColor<()> for (&'a  QColor) {
  fn setDefaultTextColor(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn defaultTextColor<RetType, T: QGraphicsTextItem_defaultTextColor<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultTextColor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_defaultTextColor<RetType> {
  fn defaultTextColor(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QColor QGraphicsTextItem::defaultTextColor();
impl<'a> /*trait*/ QGraphicsTextItem_defaultTextColor<QColor> for () {
  fn defaultTextColor(self, rsthis: &mut QGraphicsTextItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem16defaultTextColorEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem16defaultTextColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn FreeQGraphicsTextItem<RetType, T: QGraphicsTextItem_FreeQGraphicsTextItem<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsTextItem(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_FreeQGraphicsTextItem<RetType> {
  fn FreeQGraphicsTextItem(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::FreeQGraphicsTextItem();
impl<'a> /*trait*/ QGraphicsTextItem_FreeQGraphicsTextItem<()> for () {
  fn FreeQGraphicsTextItem(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemD0Ev()};
     unsafe {_ZN17QGraphicsTextItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn shape<RetType, T: QGraphicsTextItem_shape<RetType>>(&mut self, value: T) -> RetType {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_shape<RetType> {
  fn shape(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QPainterPath QGraphicsTextItem::shape();
impl<'a> /*trait*/ QGraphicsTextItem_shape<QPainterPath> for () {
  fn shape(self, rsthis: &mut QGraphicsTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem5shapeEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn linkActivated<RetType, T: QGraphicsTextItem_linkActivated<RetType>>(&mut self, value: T) -> RetType {
    return value.linkActivated(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_linkActivated<RetType> {
  fn linkActivated(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::linkActivated(const QString & );
impl<'a> /*trait*/ QGraphicsTextItem_linkActivated<()> for (&'a  QString) {
  fn linkActivated(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem13linkActivatedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem13linkActivatedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn textCursor<RetType, T: QGraphicsTextItem_textCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.textCursor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_textCursor<RetType> {
  fn textCursor(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QTextCursor QGraphicsTextItem::textCursor();
impl<'a> /*trait*/ QGraphicsTextItem_textCursor<QTextCursor> for () {
  fn textCursor(self, rsthis: &mut QGraphicsTextItem) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10textCursorEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn toPlainText<RetType, T: QGraphicsTextItem_toPlainText<RetType>>(&mut self, value: T) -> RetType {
    return value.toPlainText(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_toPlainText<RetType> {
  fn toPlainText(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QString QGraphicsTextItem::toPlainText();
impl<'a> /*trait*/ QGraphicsTextItem_toPlainText<QString> for () {
  fn toPlainText(self, rsthis: &mut QGraphicsTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem11toPlainTextEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn setHtml<RetType, T: QGraphicsTextItem_setHtml<RetType>>(&mut self, value: T) -> RetType {
    return value.setHtml(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setHtml<RetType> {
  fn setHtml(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::setHtml(const QString & html);
impl<'a> /*trait*/ QGraphicsTextItem_setHtml<()> for (&'a  QString) {
  fn setHtml(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsTextItem7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn tabChangesFocus<RetType, T: QGraphicsTextItem_tabChangesFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_tabChangesFocus<RetType> {
  fn tabChangesFocus(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  bool QGraphicsTextItem::tabChangesFocus();
impl<'a> /*trait*/ QGraphicsTextItem_tabChangesFocus<i8> for () {
  fn tabChangesFocus(self, rsthis: &mut QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem15tabChangesFocusEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QGraphicsTextItem::NewQGraphicsTextItem(const QGraphicsTextItem & );
impl<'a> /*trait*/ QGraphicsTextItem_NewQGraphicsTextItem for (&'a  QGraphicsTextItem) {
  fn NewQGraphicsTextItem(self) -> QGraphicsTextItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn document<RetType, T: QGraphicsTextItem_document<RetType>>(&mut self, value: T) -> RetType {
    return value.document(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_document<RetType> {
  fn document(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QTextDocument * QGraphicsTextItem::document();
impl<'a> /*trait*/ QGraphicsTextItem_document<QTextDocument> for () {
  fn document(self, rsthis: &mut QGraphicsTextItem) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem8documentEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn isObscuredBy<RetType, T: QGraphicsTextItem_isObscuredBy<RetType>>(&mut self, value: T) -> RetType {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_isObscuredBy<RetType> {
  fn isObscuredBy(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsTextItem_isObscuredBy<i8> for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn opaqueArea<RetType, T: QGraphicsTextItem_opaqueArea<RetType>>(&mut self, value: T) -> RetType {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_opaqueArea<RetType> {
  fn opaqueArea(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  QPainterPath QGraphicsTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsTextItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK17QGraphicsTextItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn contains<RetType, T: QGraphicsTextItem_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_contains<RetType> {
  fn contains(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  bool QGraphicsTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsTextItem_contains<i8> for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QGraphicsTextItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTextItem {
  pub fn adjustSize<RetType, T: QGraphicsTextItem_adjustSize<RetType>>(&mut self, value: T) -> RetType {
    return value.adjustSize(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_adjustSize<RetType> {
  fn adjustSize(self, rsthis: &mut QGraphicsTextItem) -> RetType;
}

// proto:  void QGraphicsTextItem::adjustSize();
impl<'a> /*trait*/ QGraphicsTextItem_adjustSize<()> for () {
  fn adjustSize(self, rsthis: &mut QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem10adjustSizeEv()};
     unsafe {_ZN17QGraphicsTextItem10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

