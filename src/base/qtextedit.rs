// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qmenu::QMenu;
use super::qtextdocument::QTextDocument;
use super::qrect::QRect;
use super::qcolor::QColor;
use super::qtextcharformat::QTextCharFormat;
use super::qpoint::QPoint;
use super::qtextcursor::QTextCursor;
use super::qfont::QFont;
use super::qurl::QUrl;
use super::qvariant::QVariant;
use super::qwidget::QWidget;
use super::qpagedpaintdevice::QPagedPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QTextEdit::lineWrapColumnOrWidth();
  fn _ZNK9QTextEdit21lineWrapColumnOrWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextEdit::setFontFamily(const QString & fontFamily);
  fn _ZN9QTextEdit13setFontFamilyERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextEdit::toPlainText();
  fn _ZNK9QTextEdit11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setCursorWidth(int width);
  fn _ZN9QTextEdit14setCursorWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QMenu * QTextEdit::createStandardContextMenu();
  fn _ZN9QTextEdit25createStandardContextMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextDocument * QTextEdit::document();
  fn _ZNK9QTextEdit8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QTextEdit::cursorRect();
  fn _ZNK9QTextEdit10cursorRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setTextColor(const QColor & c);
  fn _ZN9QTextEdit12setTextColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextEdit::acceptRichText();
  fn _ZNK9QTextEdit14acceptRichTextEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextEdit::clear();
  fn _ZN9QTextEdit5clearEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::insertHtml(const QString & text);
  fn _ZN9QTextEdit10insertHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextEdit::fontFamily();
  fn _ZNK9QTextEdit10fontFamilyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setFontUnderline(bool b);
  fn _ZN9QTextEdit16setFontUnderlineEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextEdit::cut();
  fn _ZN9QTextEdit3cutEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
  fn _ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextEdit::anchorAt(const QPoint & pos);
  fn _ZNK9QTextEdit8anchorAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTextEdit::cursorWidth();
  fn _ZNK9QTextEdit11cursorWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextEdit::setTextBackgroundColor(const QColor & c);
  fn _ZN9QTextEdit22setTextBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextEdit::tabStopWidth();
  fn _ZNK9QTextEdit12tabStopWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextEdit::setFontWeight(int w);
  fn _ZN9QTextEdit13setFontWeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextEdit::selectAll();
  fn _ZN9QTextEdit9selectAllEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::zoomOut(int range);
  fn _ZN9QTextEdit7zoomOutEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextEdit::redo();
  fn _ZN9QTextEdit4redoEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::setFontPointSize(qreal s);
  fn _ZN9QTextEdit16setFontPointSizeEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextEdit::overwriteMode();
  fn _ZNK9QTextEdit13overwriteModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTextCursor QTextEdit::textCursor();
  fn _ZNK9QTextEdit10textCursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn _ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextEdit::setPlainText(const QString & text);
  fn _ZN9QTextEdit12setPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextEdit::placeholderText();
  fn _ZNK9QTextEdit15placeholderTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::FreeQTextEdit();
  fn _ZN9QTextEditD0Ev(qthis: *mut c_void) ;
  // proto:  bool QTextEdit::fontItalic();
  fn _ZNK9QTextEdit10fontItalicEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextEdit::copy();
  fn _ZN9QTextEdit4copyEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::textChanged();
  fn _ZN9QTextEdit11textChangedEv(qthis: *mut c_void) ;
  // proto:  double QTextEdit::fontPointSize();
  fn _ZNK9QTextEdit13fontPointSizeEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextEdit::setDocument(QTextDocument * document);
  fn _ZN9QTextEdit11setDocumentEP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextEdit::setOverwriteMode(bool overwrite);
  fn _ZN9QTextEdit16setOverwriteModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextEdit::undo();
  fn _ZN9QTextEdit4undoEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::zoomIn(int range);
  fn _ZN9QTextEdit6zoomInEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextEdit::setDocumentTitle(const QString & title);
  fn _ZN9QTextEdit16setDocumentTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextEdit::canPaste();
  fn _ZNK9QTextEdit8canPasteEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QTextEdit::toHtml();
  fn _ZNK9QTextEdit6toHtmlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMenu * QTextEdit::createStandardContextMenu(const QPoint & position);
  fn _ZN9QTextEdit25createStandardContextMenuERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::cursorPositionChanged();
  fn _ZN9QTextEdit21cursorPositionChangedEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::setTabStopWidth(int width);
  fn _ZN9QTextEdit15setTabStopWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextEdit::undoAvailable(bool b);
  fn _ZN9QTextEdit13undoAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QTextEdit::documentTitle();
  fn _ZNK9QTextEdit13documentTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextEdit::isUndoRedoEnabled();
  fn _ZNK9QTextEdit17isUndoRedoEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextEdit::setText(const QString & text);
  fn _ZN9QTextEdit7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextEdit::ensureCursorVisible();
  fn _ZN9QTextEdit19ensureCursorVisibleEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::setAcceptRichText(bool accept);
  fn _ZN9QTextEdit17setAcceptRichTextEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextEdit::setPlaceholderText(const QString & placeholderText);
  fn _ZN9QTextEdit18setPlaceholderTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextEdit::isReadOnly();
  fn _ZNK9QTextEdit10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextEdit::setUndoRedoEnabled(bool enable);
  fn _ZN9QTextEdit18setUndoRedoEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextEdit::NewQTextEdit(const QTextEdit & );
  fn _ZN9QTextEditC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextCharFormat QTextEdit::currentCharFormat();
  fn _ZNK9QTextEdit17currentCharFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
  fn _ZNK9QTextEdit17cursorForPositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::scrollToAnchor(const QString & name);
  fn _ZN9QTextEdit14scrollToAnchorERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QFont QTextEdit::currentFont();
  fn _ZNK9QTextEdit11currentFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::paste();
  fn _ZN9QTextEdit5pasteEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::setTextCursor(const QTextCursor & cursor);
  fn _ZN9QTextEdit13setTextCursorERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn _ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVariant QTextEdit::loadResource(int type, const QUrl & name);
  fn _ZN9QTextEdit12loadResourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setTabChangesFocus(bool b);
  fn _ZN9QTextEdit18setTabChangesFocusEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextEdit::selectionChanged();
  fn _ZN9QTextEdit16selectionChangedEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::setHtml(const QString & text);
  fn _ZN9QTextEdit7setHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QTextEdit::cursorRect(const QTextCursor & cursor);
  fn _ZNK9QTextEdit10cursorRectERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setLineWrapColumnOrWidth(int w);
  fn _ZN9QTextEdit24setLineWrapColumnOrWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextEdit::setFontItalic(bool b);
  fn _ZN9QTextEdit13setFontItalicEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QTextEdit::metaObject();
  fn _ZNK9QTextEdit10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTextEdit::setCurrentFont(const QFont & f);
  fn _ZN9QTextEdit14setCurrentFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextEdit::tabChangesFocus();
  fn _ZNK9QTextEdit15tabChangesFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  QColor QTextEdit::textBackgroundColor();
  fn _ZNK9QTextEdit19textBackgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::NewQTextEdit(const QString & text, QWidget * parent);
  fn _ZN9QTextEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTextEdit::print(QPagedPaintDevice * printer);
  fn _ZNK9QTextEdit5printEP17QPagedPaintDevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextEdit::fontUnderline();
  fn _ZNK9QTextEdit13fontUnderlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextEdit::insertPlainText(const QString & text);
  fn _ZN9QTextEdit15insertPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextEdit::fontWeight();
  fn _ZNK9QTextEdit10fontWeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextEdit::copyAvailable(bool b);
  fn _ZN9QTextEdit13copyAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QColor QTextEdit::textColor();
  fn _ZNK9QTextEdit9textColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::append(const QString & text);
  fn _ZN9QTextEdit6appendERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextEdit::redoAvailable(bool b);
  fn _ZN9QTextEdit13redoAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextEdit::NewQTextEdit(QWidget * parent);
  fn _ZN9QTextEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextEdit::setReadOnly(bool ro);
  fn _ZN9QTextEdit11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QTextEdit)=1
pub struct QTextEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextEdit {
  pub fn lineWrapColumnOrWidth<T: QTextEdit_lineWrapColumnOrWidth>(&mut self, value: T) -> i32 {
    return value.lineWrapColumnOrWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_lineWrapColumnOrWidth {
  fn lineWrapColumnOrWidth(self, rsthis: &mut QTextEdit) -> i32;
}

// proto:  int QTextEdit::lineWrapColumnOrWidth();
impl<'a> /*trait*/ QTextEdit_lineWrapColumnOrWidth for () {
  fn lineWrapColumnOrWidth(self, rsthis: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit21lineWrapColumnOrWidthEv()};
    let mut ret = unsafe {_ZNK9QTextEdit21lineWrapColumnOrWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontFamily<T: QTextEdit_setFontFamily>(&mut self, value: T)  {
     value.setFontFamily(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontFamily {
  fn setFontFamily(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setFontFamily(const QString & fontFamily);
impl<'a> /*trait*/ QTextEdit_setFontFamily for (&'a  QString) {
  fn setFontFamily(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit13setFontFamilyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn toPlainText<T: QTextEdit_toPlainText>(&mut self, value: T) -> QString {
    return value.toPlainText(self);
    // return 1;
  }
}

pub trait QTextEdit_toPlainText {
  fn toPlainText(self, rsthis: &mut QTextEdit) -> QString;
}

// proto:  QString QTextEdit::toPlainText();
impl<'a> /*trait*/ QTextEdit_toPlainText for () {
  fn toPlainText(self, rsthis: &mut QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11toPlainTextEv()};
    let mut ret = unsafe {_ZNK9QTextEdit11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setCursorWidth<T: QTextEdit_setCursorWidth>(&mut self, value: T)  {
     value.setCursorWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_setCursorWidth {
  fn setCursorWidth(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QTextEdit_setCursorWidth for (i32) {
  fn setCursorWidth(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn createStandardContextMenu<T: QTextEdit_createStandardContextMenu>(&mut self, value: T) -> QMenu {
    return value.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QTextEdit_createStandardContextMenu {
  fn createStandardContextMenu(self, rsthis: &mut QTextEdit) -> QMenu;
}

// proto:  QMenu * QTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu for () {
  fn createStandardContextMenu(self, rsthis: &mut QTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {_ZN9QTextEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn document<T: QTextEdit_document>(&mut self, value: T) -> QTextDocument {
    return value.document(self);
    // return 1;
  }
}

pub trait QTextEdit_document {
  fn document(self, rsthis: &mut QTextEdit) -> QTextDocument;
}

// proto:  QTextDocument * QTextEdit::document();
impl<'a> /*trait*/ QTextEdit_document for () {
  fn document(self, rsthis: &mut QTextEdit) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8documentEv()};
    let mut ret = unsafe {_ZNK9QTextEdit8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorRect<T: QTextEdit_cursorRect>(&mut self, value: T) -> QRect {
    return value.cursorRect(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorRect {
  fn cursorRect(self, rsthis: &mut QTextEdit) -> QRect;
}

// proto:  QRect QTextEdit::cursorRect();
impl<'a> /*trait*/ QTextEdit_cursorRect for () {
  fn cursorRect(self, rsthis: &mut QTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10cursorRectEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10cursorRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTextColor<T: QTextEdit_setTextColor>(&mut self, value: T)  {
     value.setTextColor(self);
    // return 1;
  }
}

pub trait QTextEdit_setTextColor {
  fn setTextColor(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setTextColor(const QColor & c);
impl<'a> /*trait*/ QTextEdit_setTextColor for (&'a  QColor) {
  fn setTextColor(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn acceptRichText<T: QTextEdit_acceptRichText>(&mut self, value: T) -> i8 {
    return value.acceptRichText(self);
    // return 1;
  }
}

pub trait QTextEdit_acceptRichText {
  fn acceptRichText(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::acceptRichText();
impl<'a> /*trait*/ QTextEdit_acceptRichText for () {
  fn acceptRichText(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit14acceptRichTextEv()};
    let mut ret = unsafe {_ZNK9QTextEdit14acceptRichTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn clear<T: QTextEdit_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QTextEdit_clear {
  fn clear(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::clear();
impl<'a> /*trait*/ QTextEdit_clear for () {
  fn clear(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit5clearEv()};
     unsafe {_ZN9QTextEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn insertHtml<T: QTextEdit_insertHtml>(&mut self, value: T)  {
     value.insertHtml(self);
    // return 1;
  }
}

pub trait QTextEdit_insertHtml {
  fn insertHtml(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::insertHtml(const QString & text);
impl<'a> /*trait*/ QTextEdit_insertHtml for (&'a  QString) {
  fn insertHtml(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit10insertHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit10insertHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontFamily<T: QTextEdit_fontFamily>(&mut self, value: T) -> QString {
    return value.fontFamily(self);
    // return 1;
  }
}

pub trait QTextEdit_fontFamily {
  fn fontFamily(self, rsthis: &mut QTextEdit) -> QString;
}

// proto:  QString QTextEdit::fontFamily();
impl<'a> /*trait*/ QTextEdit_fontFamily for () {
  fn fontFamily(self, rsthis: &mut QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontFamilyEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10fontFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontUnderline<T: QTextEdit_setFontUnderline>(&mut self, value: T)  {
     value.setFontUnderline(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontUnderline {
  fn setFontUnderline(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setFontUnderline(bool b);
impl<'a> /*trait*/ QTextEdit_setFontUnderline for (i8) {
  fn setFontUnderline(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setFontUnderlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit16setFontUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cut<T: QTextEdit_cut>(&mut self, value: T)  {
     value.cut(self);
    // return 1;
  }
}

pub trait QTextEdit_cut {
  fn cut(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::cut();
impl<'a> /*trait*/ QTextEdit_cut for () {
  fn cut(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit3cutEv()};
     unsafe {_ZN9QTextEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn currentCharFormatChanged<T: QTextEdit_currentCharFormatChanged>(&mut self, value: T)  {
     value.currentCharFormatChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_currentCharFormatChanged {
  fn currentCharFormatChanged(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextEdit_currentCharFormatChanged for (&'a  QTextCharFormat) {
  fn currentCharFormatChanged(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn anchorAt<T: QTextEdit_anchorAt>(&mut self, value: T) -> QString {
    return value.anchorAt(self);
    // return 1;
  }
}

pub trait QTextEdit_anchorAt {
  fn anchorAt(self, rsthis: &mut QTextEdit) -> QString;
}

// proto:  QString QTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QTextEdit_anchorAt for (&'a  QPoint) {
  fn anchorAt(self, rsthis: &mut QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextEdit8anchorAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorWidth<T: QTextEdit_cursorWidth>(&mut self, value: T) -> i32 {
    return value.cursorWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorWidth {
  fn cursorWidth(self, rsthis: &mut QTextEdit) -> i32;
}

// proto:  int QTextEdit::cursorWidth();
impl<'a> /*trait*/ QTextEdit_cursorWidth for () {
  fn cursorWidth(self, rsthis: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11cursorWidthEv()};
    let mut ret = unsafe {_ZNK9QTextEdit11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTextBackgroundColor<T: QTextEdit_setTextBackgroundColor>(&mut self, value: T)  {
     value.setTextBackgroundColor(self);
    // return 1;
  }
}

pub trait QTextEdit_setTextBackgroundColor {
  fn setTextBackgroundColor(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setTextBackgroundColor(const QColor & c);
impl<'a> /*trait*/ QTextEdit_setTextBackgroundColor for (&'a  QColor) {
  fn setTextBackgroundColor(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit22setTextBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit22setTextBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn tabStopWidth<T: QTextEdit_tabStopWidth>(&mut self, value: T) -> i32 {
    return value.tabStopWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_tabStopWidth {
  fn tabStopWidth(self, rsthis: &mut QTextEdit) -> i32;
}

// proto:  int QTextEdit::tabStopWidth();
impl<'a> /*trait*/ QTextEdit_tabStopWidth for () {
  fn tabStopWidth(self, rsthis: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit12tabStopWidthEv()};
    let mut ret = unsafe {_ZNK9QTextEdit12tabStopWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontWeight<T: QTextEdit_setFontWeight>(&mut self, value: T)  {
     value.setFontWeight(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontWeight {
  fn setFontWeight(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setFontWeight(int w);
impl<'a> /*trait*/ QTextEdit_setFontWeight for (i32) {
  fn setFontWeight(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit13setFontWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn selectAll<T: QTextEdit_selectAll>(&mut self, value: T)  {
     value.selectAll(self);
    // return 1;
  }
}

pub trait QTextEdit_selectAll {
  fn selectAll(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::selectAll();
impl<'a> /*trait*/ QTextEdit_selectAll for () {
  fn selectAll(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit9selectAllEv()};
     unsafe {_ZN9QTextEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn zoomOut<T: QTextEdit_zoomOut>(&mut self, value: T)  {
     value.zoomOut(self);
    // return 1;
  }
}

pub trait QTextEdit_zoomOut {
  fn zoomOut(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QTextEdit_zoomOut for (i32) {
  fn zoomOut(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit7zoomOutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn redo<T: QTextEdit_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QTextEdit_redo {
  fn redo(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::redo();
impl<'a> /*trait*/ QTextEdit_redo for () {
  fn redo(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4redoEv()};
     unsafe {_ZN9QTextEdit4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontPointSize<T: QTextEdit_setFontPointSize>(&mut self, value: T)  {
     value.setFontPointSize(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontPointSize {
  fn setFontPointSize(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setFontPointSize(qreal s);
impl<'a> /*trait*/ QTextEdit_setFontPointSize for (f64) {
  fn setFontPointSize(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setFontPointSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QTextEdit16setFontPointSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn overwriteMode<T: QTextEdit_overwriteMode>(&mut self, value: T) -> i8 {
    return value.overwriteMode(self);
    // return 1;
  }
}

pub trait QTextEdit_overwriteMode {
  fn overwriteMode(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::overwriteMode();
impl<'a> /*trait*/ QTextEdit_overwriteMode for () {
  fn overwriteMode(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13overwriteModeEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13overwriteModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textCursor<T: QTextEdit_textCursor>(&mut self, value: T) -> QTextCursor {
    return value.textCursor(self);
    // return 1;
  }
}

pub trait QTextEdit_textCursor {
  fn textCursor(self, rsthis: &mut QTextEdit) -> QTextCursor;
}

// proto:  QTextCursor QTextEdit::textCursor();
impl<'a> /*trait*/ QTextEdit_textCursor for () {
  fn textCursor(self, rsthis: &mut QTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10textCursorEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn mergeCurrentCharFormat<T: QTextEdit_mergeCurrentCharFormat>(&mut self, value: T)  {
     value.mergeCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QTextEdit_mergeCurrentCharFormat {
  fn mergeCurrentCharFormat(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextEdit_mergeCurrentCharFormat for (&'a  QTextCharFormat) {
  fn mergeCurrentCharFormat(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setPlainText<T: QTextEdit_setPlainText>(&mut self, value: T)  {
     value.setPlainText(self);
    // return 1;
  }
}

pub trait QTextEdit_setPlainText {
  fn setPlainText(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextEdit_setPlainText for (&'a  QString) {
  fn setPlainText(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn placeholderText<T: QTextEdit_placeholderText>(&mut self, value: T) -> QString {
    return value.placeholderText(self);
    // return 1;
  }
}

pub trait QTextEdit_placeholderText {
  fn placeholderText(self, rsthis: &mut QTextEdit) -> QString;
}

// proto:  QString QTextEdit::placeholderText();
impl<'a> /*trait*/ QTextEdit_placeholderText for () {
  fn placeholderText(self, rsthis: &mut QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit15placeholderTextEv()};
    let mut ret = unsafe {_ZNK9QTextEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn FreeQTextEdit<T: QTextEdit_FreeQTextEdit>(&mut self, value: T)  {
     value.FreeQTextEdit(self);
    // return 1;
  }
}

pub trait QTextEdit_FreeQTextEdit {
  fn FreeQTextEdit(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::FreeQTextEdit();
impl<'a> /*trait*/ QTextEdit_FreeQTextEdit for () {
  fn FreeQTextEdit(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditD0Ev()};
     unsafe {_ZN9QTextEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontItalic<T: QTextEdit_fontItalic>(&mut self, value: T) -> i8 {
    return value.fontItalic(self);
    // return 1;
  }
}

pub trait QTextEdit_fontItalic {
  fn fontItalic(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::fontItalic();
impl<'a> /*trait*/ QTextEdit_fontItalic for () {
  fn fontItalic(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontItalicEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10fontItalicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn copy<T: QTextEdit_copy>(&mut self, value: T)  {
     value.copy(self);
    // return 1;
  }
}

pub trait QTextEdit_copy {
  fn copy(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::copy();
impl<'a> /*trait*/ QTextEdit_copy for () {
  fn copy(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4copyEv()};
     unsafe {_ZN9QTextEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textChanged<T: QTextEdit_textChanged>(&mut self, value: T)  {
     value.textChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_textChanged {
  fn textChanged(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::textChanged();
impl<'a> /*trait*/ QTextEdit_textChanged for () {
  fn textChanged(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11textChangedEv()};
     unsafe {_ZN9QTextEdit11textChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontPointSize<T: QTextEdit_fontPointSize>(&mut self, value: T) -> f64 {
    return value.fontPointSize(self);
    // return 1;
  }
}

pub trait QTextEdit_fontPointSize {
  fn fontPointSize(self, rsthis: &mut QTextEdit) -> f64;
}

// proto:  double QTextEdit::fontPointSize();
impl<'a> /*trait*/ QTextEdit_fontPointSize for () {
  fn fontPointSize(self, rsthis: &mut QTextEdit) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13fontPointSizeEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13fontPointSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setDocument<T: QTextEdit_setDocument>(&mut self, value: T)  {
     value.setDocument(self);
    // return 1;
  }
}

pub trait QTextEdit_setDocument {
  fn setDocument(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QTextEdit_setDocument for (&'a mut QTextDocument) {
  fn setDocument(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setOverwriteMode<T: QTextEdit_setOverwriteMode>(&mut self, value: T)  {
     value.setOverwriteMode(self);
    // return 1;
  }
}

pub trait QTextEdit_setOverwriteMode {
  fn setOverwriteMode(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QTextEdit_setOverwriteMode for (i8) {
  fn setOverwriteMode(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setOverwriteModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit16setOverwriteModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn undo<T: QTextEdit_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QTextEdit_undo {
  fn undo(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::undo();
impl<'a> /*trait*/ QTextEdit_undo for () {
  fn undo(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4undoEv()};
     unsafe {_ZN9QTextEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn zoomIn<T: QTextEdit_zoomIn>(&mut self, value: T)  {
     value.zoomIn(self);
    // return 1;
  }
}

pub trait QTextEdit_zoomIn {
  fn zoomIn(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QTextEdit_zoomIn for (i32) {
  fn zoomIn(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit6zoomInEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setDocumentTitle<T: QTextEdit_setDocumentTitle>(&mut self, value: T)  {
     value.setDocumentTitle(self);
    // return 1;
  }
}

pub trait QTextEdit_setDocumentTitle {
  fn setDocumentTitle(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QTextEdit_setDocumentTitle for (&'a  QString) {
  fn setDocumentTitle(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit16setDocumentTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn canPaste<T: QTextEdit_canPaste>(&mut self, value: T) -> i8 {
    return value.canPaste(self);
    // return 1;
  }
}

pub trait QTextEdit_canPaste {
  fn canPaste(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::canPaste();
impl<'a> /*trait*/ QTextEdit_canPaste for () {
  fn canPaste(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8canPasteEv()};
    let mut ret = unsafe {_ZNK9QTextEdit8canPasteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn toHtml<T: QTextEdit_toHtml>(&mut self, value: T) -> QString {
    return value.toHtml(self);
    // return 1;
  }
}

pub trait QTextEdit_toHtml {
  fn toHtml(self, rsthis: &mut QTextEdit) -> QString;
}

// proto:  QString QTextEdit::toHtml();
impl<'a> /*trait*/ QTextEdit_toHtml for () {
  fn toHtml(self, rsthis: &mut QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit6toHtmlEv()};
    let mut ret = unsafe {_ZNK9QTextEdit6toHtmlEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMenu * QTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu for (&'a  QPoint) {
  fn createStandardContextMenu(self, rsthis: &mut QTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTextEdit25createStandardContextMenuERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorPositionChanged<T: QTextEdit_cursorPositionChanged>(&mut self, value: T)  {
     value.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorPositionChanged {
  fn cursorPositionChanged(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::cursorPositionChanged();
impl<'a> /*trait*/ QTextEdit_cursorPositionChanged for () {
  fn cursorPositionChanged(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit21cursorPositionChangedEv()};
     unsafe {_ZN9QTextEdit21cursorPositionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTabStopWidth<T: QTextEdit_setTabStopWidth>(&mut self, value: T)  {
     value.setTabStopWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_setTabStopWidth {
  fn setTabStopWidth(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QTextEdit_setTabStopWidth for (i32) {
  fn setTabStopWidth(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit15setTabStopWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn undoAvailable<T: QTextEdit_undoAvailable>(&mut self, value: T)  {
     value.undoAvailable(self);
    // return 1;
  }
}

pub trait QTextEdit_undoAvailable {
  fn undoAvailable(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::undoAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_undoAvailable for (i8) {
  fn undoAvailable(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13undoAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit13undoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn documentTitle<T: QTextEdit_documentTitle>(&mut self, value: T) -> QString {
    return value.documentTitle(self);
    // return 1;
  }
}

pub trait QTextEdit_documentTitle {
  fn documentTitle(self, rsthis: &mut QTextEdit) -> QString;
}

// proto:  QString QTextEdit::documentTitle();
impl<'a> /*trait*/ QTextEdit_documentTitle for () {
  fn documentTitle(self, rsthis: &mut QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13documentTitleEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13documentTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn isUndoRedoEnabled<T: QTextEdit_isUndoRedoEnabled>(&mut self, value: T) -> i8 {
    return value.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextEdit_isUndoRedoEnabled {
  fn isUndoRedoEnabled(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextEdit_isUndoRedoEnabled for () {
  fn isUndoRedoEnabled(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17isUndoRedoEnabledEv()};
    let mut ret = unsafe {_ZNK9QTextEdit17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setText<T: QTextEdit_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QTextEdit_setText {
  fn setText(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setText(const QString & text);
impl<'a> /*trait*/ QTextEdit_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn ensureCursorVisible<T: QTextEdit_ensureCursorVisible>(&mut self, value: T)  {
     value.ensureCursorVisible(self);
    // return 1;
  }
}

pub trait QTextEdit_ensureCursorVisible {
  fn ensureCursorVisible(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QTextEdit_ensureCursorVisible for () {
  fn ensureCursorVisible(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit19ensureCursorVisibleEv()};
     unsafe {_ZN9QTextEdit19ensureCursorVisibleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setAcceptRichText<T: QTextEdit_setAcceptRichText>(&mut self, value: T)  {
     value.setAcceptRichText(self);
    // return 1;
  }
}

pub trait QTextEdit_setAcceptRichText {
  fn setAcceptRichText(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setAcceptRichText(bool accept);
impl<'a> /*trait*/ QTextEdit_setAcceptRichText for (i8) {
  fn setAcceptRichText(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit17setAcceptRichTextEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit17setAcceptRichTextEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setPlaceholderText<T: QTextEdit_setPlaceholderText>(&mut self, value: T)  {
     value.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QTextEdit_setPlaceholderText {
  fn setPlaceholderText(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QTextEdit_setPlaceholderText for (&'a  QString) {
  fn setPlaceholderText(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn isReadOnly<T: QTextEdit_isReadOnly>(&mut self, value: T) -> i8 {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QTextEdit_isReadOnly {
  fn isReadOnly(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::isReadOnly();
impl<'a> /*trait*/ QTextEdit_isReadOnly for () {
  fn isReadOnly(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setUndoRedoEnabled<T: QTextEdit_setUndoRedoEnabled>(&mut self, value: T)  {
     value.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextEdit_setUndoRedoEnabled {
  fn setUndoRedoEnabled(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextEdit_setUndoRedoEnabled for (i8) {
  fn setUndoRedoEnabled(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn NewQTextEdit<T: QTextEdit_NewQTextEdit>(value: T) -> QTextEdit {
    let rsthis = value.NewQTextEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEdit_NewQTextEdit {
  fn NewQTextEdit(self) -> QTextEdit;
}

// proto: void QTextEdit::NewQTextEdit(const QTextEdit & );
impl<'a> /*trait*/ QTextEdit_NewQTextEdit for (&'a  QTextEdit) {
  fn NewQTextEdit(self) -> QTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextEditC1ERKS_(qthis, arg0)};
    let rsthis = QTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn currentCharFormat<T: QTextEdit_currentCharFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.currentCharFormat(self);
    // return 1;
  }
}

pub trait QTextEdit_currentCharFormat {
  fn currentCharFormat(self, rsthis: &mut QTextEdit) -> QTextCharFormat;
}

// proto:  QTextCharFormat QTextEdit::currentCharFormat();
impl<'a> /*trait*/ QTextEdit_currentCharFormat for () {
  fn currentCharFormat(self, rsthis: &mut QTextEdit) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17currentCharFormatEv()};
    let mut ret = unsafe {_ZNK9QTextEdit17currentCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorForPosition<T: QTextEdit_cursorForPosition>(&mut self, value: T) -> QTextCursor {
    return value.cursorForPosition(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorForPosition {
  fn cursorForPosition(self, rsthis: &mut QTextEdit) -> QTextCursor;
}

// proto:  QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QTextEdit_cursorForPosition for (&'a  QPoint) {
  fn cursorForPosition(self, rsthis: &mut QTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextEdit17cursorForPositionERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn scrollToAnchor<T: QTextEdit_scrollToAnchor>(&mut self, value: T)  {
     value.scrollToAnchor(self);
    // return 1;
  }
}

pub trait QTextEdit_scrollToAnchor {
  fn scrollToAnchor(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::scrollToAnchor(const QString & name);
impl<'a> /*trait*/ QTextEdit_scrollToAnchor for (&'a  QString) {
  fn scrollToAnchor(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14scrollToAnchorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit14scrollToAnchorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn currentFont<T: QTextEdit_currentFont>(&mut self, value: T) -> QFont {
    return value.currentFont(self);
    // return 1;
  }
}

pub trait QTextEdit_currentFont {
  fn currentFont(self, rsthis: &mut QTextEdit) -> QFont;
}

// proto:  QFont QTextEdit::currentFont();
impl<'a> /*trait*/ QTextEdit_currentFont for () {
  fn currentFont(self, rsthis: &mut QTextEdit) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11currentFontEv()};
    let mut ret = unsafe {_ZNK9QTextEdit11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn paste<T: QTextEdit_paste>(&mut self, value: T)  {
     value.paste(self);
    // return 1;
  }
}

pub trait QTextEdit_paste {
  fn paste(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::paste();
impl<'a> /*trait*/ QTextEdit_paste for () {
  fn paste(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit5pasteEv()};
     unsafe {_ZN9QTextEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTextCursor<T: QTextEdit_setTextCursor>(&mut self, value: T)  {
     value.setTextCursor(self);
    // return 1;
  }
}

pub trait QTextEdit_setTextCursor {
  fn setTextCursor(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextEdit_setTextCursor for (&'a  QTextCursor) {
  fn setTextCursor(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setCurrentCharFormat<T: QTextEdit_setCurrentCharFormat>(&mut self, value: T)  {
     value.setCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QTextEdit_setCurrentCharFormat {
  fn setCurrentCharFormat(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextEdit_setCurrentCharFormat for (&'a  QTextCharFormat) {
  fn setCurrentCharFormat(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn loadResource<T: QTextEdit_loadResource>(&mut self, value: T) -> QVariant {
    return value.loadResource(self);
    // return 1;
  }
}

pub trait QTextEdit_loadResource {
  fn loadResource(self, rsthis: &mut QTextEdit) -> QVariant;
}

// proto:  QVariant QTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextEdit_loadResource for (i32, &'a  QUrl) {
  fn loadResource(self, rsthis: &mut QTextEdit) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTextEdit12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTabChangesFocus<T: QTextEdit_setTabChangesFocus>(&mut self, value: T)  {
     value.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QTextEdit_setTabChangesFocus {
  fn setTabChangesFocus(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QTextEdit_setTabChangesFocus for (i8) {
  fn setTabChangesFocus(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn selectionChanged<T: QTextEdit_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::selectionChanged();
impl<'a> /*trait*/ QTextEdit_selectionChanged for () {
  fn selectionChanged(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16selectionChangedEv()};
     unsafe {_ZN9QTextEdit16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setHtml<T: QTextEdit_setHtml>(&mut self, value: T)  {
     value.setHtml(self);
    // return 1;
  }
}

pub trait QTextEdit_setHtml {
  fn setHtml(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setHtml(const QString & text);
impl<'a> /*trait*/ QTextEdit_setHtml for (&'a  QString) {
  fn setHtml(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QRect QTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextEdit_cursorRect for (&'a  QTextCursor) {
  fn cursorRect(self, rsthis: &mut QTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextEdit10cursorRectERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setLineWrapColumnOrWidth<T: QTextEdit_setLineWrapColumnOrWidth>(&mut self, value: T)  {
     value.setLineWrapColumnOrWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_setLineWrapColumnOrWidth {
  fn setLineWrapColumnOrWidth(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setLineWrapColumnOrWidth(int w);
impl<'a> /*trait*/ QTextEdit_setLineWrapColumnOrWidth for (i32) {
  fn setLineWrapColumnOrWidth(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit24setLineWrapColumnOrWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit24setLineWrapColumnOrWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontItalic<T: QTextEdit_setFontItalic>(&mut self, value: T)  {
     value.setFontItalic(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontItalic {
  fn setFontItalic(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setFontItalic(bool b);
impl<'a> /*trait*/ QTextEdit_setFontItalic for (i8) {
  fn setFontItalic(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontItalicEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit13setFontItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn metaObject<T: QTextEdit_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTextEdit_metaObject {
  fn metaObject(self, rsthis: &mut QTextEdit) ;
}

// proto:  const QMetaObject * QTextEdit::metaObject();
impl<'a> /*trait*/ QTextEdit_metaObject for () {
  fn metaObject(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10metaObjectEv()};
     unsafe {_ZNK9QTextEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setCurrentFont<T: QTextEdit_setCurrentFont>(&mut self, value: T)  {
     value.setCurrentFont(self);
    // return 1;
  }
}

pub trait QTextEdit_setCurrentFont {
  fn setCurrentFont(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QTextEdit_setCurrentFont for (&'a  QFont) {
  fn setCurrentFont(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn tabChangesFocus<T: QTextEdit_tabChangesFocus>(&mut self, value: T) -> i8 {
    return value.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QTextEdit_tabChangesFocus {
  fn tabChangesFocus(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QTextEdit_tabChangesFocus for () {
  fn tabChangesFocus(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit15tabChangesFocusEv()};
    let mut ret = unsafe {_ZNK9QTextEdit15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textBackgroundColor<T: QTextEdit_textBackgroundColor>(&mut self, value: T) -> QColor {
    return value.textBackgroundColor(self);
    // return 1;
  }
}

pub trait QTextEdit_textBackgroundColor {
  fn textBackgroundColor(self, rsthis: &mut QTextEdit) -> QColor;
}

// proto:  QColor QTextEdit::textBackgroundColor();
impl<'a> /*trait*/ QTextEdit_textBackgroundColor for () {
  fn textBackgroundColor(self, rsthis: &mut QTextEdit) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit19textBackgroundColorEv()};
    let mut ret = unsafe {_ZNK9QTextEdit19textBackgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextEdit::NewQTextEdit(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QTextEdit_NewQTextEdit for (&'a  QString, &'a mut QWidget) {
  fn NewQTextEdit(self) -> QTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn print<T: QTextEdit_print>(&mut self, value: T)  {
     value.print(self);
    // return 1;
  }
}

pub trait QTextEdit_print {
  fn print(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextEdit_print for (&'a mut QPagedPaintDevice) {
  fn print(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK9QTextEdit5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontUnderline<T: QTextEdit_fontUnderline>(&mut self, value: T) -> i8 {
    return value.fontUnderline(self);
    // return 1;
  }
}

pub trait QTextEdit_fontUnderline {
  fn fontUnderline(self, rsthis: &mut QTextEdit) -> i8;
}

// proto:  bool QTextEdit::fontUnderline();
impl<'a> /*trait*/ QTextEdit_fontUnderline for () {
  fn fontUnderline(self, rsthis: &mut QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13fontUnderlineEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13fontUnderlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn insertPlainText<T: QTextEdit_insertPlainText>(&mut self, value: T)  {
     value.insertPlainText(self);
    // return 1;
  }
}

pub trait QTextEdit_insertPlainText {
  fn insertPlainText(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QTextEdit_insertPlainText for (&'a  QString) {
  fn insertPlainText(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit15insertPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontWeight<T: QTextEdit_fontWeight>(&mut self, value: T) -> i32 {
    return value.fontWeight(self);
    // return 1;
  }
}

pub trait QTextEdit_fontWeight {
  fn fontWeight(self, rsthis: &mut QTextEdit) -> i32;
}

// proto:  int QTextEdit::fontWeight();
impl<'a> /*trait*/ QTextEdit_fontWeight for () {
  fn fontWeight(self, rsthis: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontWeightEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10fontWeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn copyAvailable<T: QTextEdit_copyAvailable>(&mut self, value: T)  {
     value.copyAvailable(self);
    // return 1;
  }
}

pub trait QTextEdit_copyAvailable {
  fn copyAvailable(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::copyAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_copyAvailable for (i8) {
  fn copyAvailable(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13copyAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit13copyAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textColor<T: QTextEdit_textColor>(&mut self, value: T) -> QColor {
    return value.textColor(self);
    // return 1;
  }
}

pub trait QTextEdit_textColor {
  fn textColor(self, rsthis: &mut QTextEdit) -> QColor;
}

// proto:  QColor QTextEdit::textColor();
impl<'a> /*trait*/ QTextEdit_textColor for () {
  fn textColor(self, rsthis: &mut QTextEdit) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit9textColorEv()};
    let mut ret = unsafe {_ZNK9QTextEdit9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn append<T: QTextEdit_append>(&mut self, value: T)  {
     value.append(self);
    // return 1;
  }
}

pub trait QTextEdit_append {
  fn append(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::append(const QString & text);
impl<'a> /*trait*/ QTextEdit_append for (&'a  QString) {
  fn append(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit6appendERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit6appendERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn redoAvailable<T: QTextEdit_redoAvailable>(&mut self, value: T)  {
     value.redoAvailable(self);
    // return 1;
  }
}

pub trait QTextEdit_redoAvailable {
  fn redoAvailable(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::redoAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_redoAvailable for (i8) {
  fn redoAvailable(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13redoAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit13redoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTextEdit::NewQTextEdit(QWidget * parent);
impl<'a> /*trait*/ QTextEdit_NewQTextEdit for (&'a mut QWidget) {
  fn NewQTextEdit(self) -> QTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setReadOnly<T: QTextEdit_setReadOnly>(&mut self, value: T)  {
     value.setReadOnly(self);
    // return 1;
  }
}

pub trait QTextEdit_setReadOnly {
  fn setReadOnly(self, rsthis: &mut QTextEdit) ;
}

// proto:  void QTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QTextEdit_setReadOnly for (i8) {
  fn setReadOnly(self, rsthis: &mut QTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

