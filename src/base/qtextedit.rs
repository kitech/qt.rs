// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qcolor::QColor;
use super::qtextcharformat::QTextCharFormat;
use super::qpoint::QPoint;
use super::qtextdocument::QTextDocument;
use super::qtextcursor::QTextCursor;
use super::qurl::QUrl;
use super::qfont::QFont;
use super::qwidget::QWidget;
use super::qpagedpaintdevice::QPagedPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QTextEdit::lineWrapColumnOrWidth();
  fn _ZNK9QTextEdit21lineWrapColumnOrWidthEv() -> i32;
  // proto: void QTextEdit::setFontFamily(const QString & fontFamily);
  fn _ZN9QTextEdit13setFontFamilyERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTextEdit::toPlainText();
  fn _ZNK9QTextEdit11toPlainTextEv() -> i32;
  // proto: void QTextEdit::setCursorWidth(int width);
  fn _ZN9QTextEdit14setCursorWidthEi(arg0: c_int) -> i32;
  // proto: QMenu * QTextEdit::createStandardContextMenu();
  fn _ZN9QTextEdit25createStandardContextMenuEv() -> i32;
  // proto: QTextDocument * QTextEdit::document();
  fn _ZNK9QTextEdit8documentEv() -> i32;
  // proto: QRect QTextEdit::cursorRect();
  fn _ZNK9QTextEdit10cursorRectEv() -> i32;
  // proto: void QTextEdit::setTextColor(const QColor & c);
  fn _ZN9QTextEdit12setTextColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: bool QTextEdit::acceptRichText();
  fn _ZNK9QTextEdit14acceptRichTextEv() -> i32;
  // proto: void QTextEdit::clear();
  fn _ZN9QTextEdit5clearEv() -> i32;
  // proto: void QTextEdit::insertHtml(const QString & text);
  fn _ZN9QTextEdit10insertHtmlERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTextEdit::fontFamily();
  fn _ZNK9QTextEdit10fontFamilyEv() -> i32;
  // proto: void QTextEdit::setFontUnderline(bool b);
  fn _ZN9QTextEdit16setFontUnderlineEb(arg0: int8_t) -> i32;
  // proto: void QTextEdit::cut();
  fn _ZN9QTextEdit3cutEv() -> i32;
  // proto: void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
  fn _ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(arg0: *const c_void) -> i32;
  // proto: QString QTextEdit::anchorAt(const QPoint & pos);
  fn _ZNK9QTextEdit8anchorAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: int QTextEdit::cursorWidth();
  fn _ZNK9QTextEdit11cursorWidthEv() -> i32;
  // proto: void QTextEdit::setTextBackgroundColor(const QColor & c);
  fn _ZN9QTextEdit22setTextBackgroundColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: int QTextEdit::tabStopWidth();
  fn _ZNK9QTextEdit12tabStopWidthEv() -> i32;
  // proto: void QTextEdit::setFontWeight(int w);
  fn _ZN9QTextEdit13setFontWeightEi(arg0: c_int) -> i32;
  // proto: void QTextEdit::selectAll();
  fn _ZN9QTextEdit9selectAllEv() -> i32;
  // proto: void QTextEdit::zoomOut(int range);
  fn _ZN9QTextEdit7zoomOutEi(arg0: c_int) -> i32;
  // proto: void QTextEdit::redo();
  fn _ZN9QTextEdit4redoEv() -> i32;
  // proto: void QTextEdit::setFontPointSize(qreal s);
  fn _ZN9QTextEdit16setFontPointSizeEd(arg0: c_double) -> i32;
  // proto: bool QTextEdit::overwriteMode();
  fn _ZNK9QTextEdit13overwriteModeEv() -> i32;
  // proto: QTextCursor QTextEdit::textCursor();
  fn _ZNK9QTextEdit10textCursorEv() -> i32;
  // proto: void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn _ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::setPlainText(const QString & text);
  fn _ZN9QTextEdit12setPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTextEdit::placeholderText();
  fn _ZNK9QTextEdit15placeholderTextEv() -> i32;
  // proto: void QTextEdit::FreeQTextEdit();
  fn _ZN9QTextEditD0Ev() -> i32;
  // proto: bool QTextEdit::fontItalic();
  fn _ZNK9QTextEdit10fontItalicEv() -> i32;
  // proto: void QTextEdit::copy();
  fn _ZN9QTextEdit4copyEv() -> i32;
  // proto: void QTextEdit::textChanged();
  fn _ZN9QTextEdit11textChangedEv() -> i32;
  // proto: double QTextEdit::fontPointSize();
  fn _ZNK9QTextEdit13fontPointSizeEv() -> i32;
  // proto: void QTextEdit::setDocument(QTextDocument * document);
  fn _ZN9QTextEdit11setDocumentEP13QTextDocument(arg0: *mut c_void) -> i32;
  // proto: void QTextEdit::setOverwriteMode(bool overwrite);
  fn _ZN9QTextEdit16setOverwriteModeEb(arg0: int8_t) -> i32;
  // proto: void QTextEdit::undo();
  fn _ZN9QTextEdit4undoEv() -> i32;
  // proto: void QTextEdit::zoomIn(int range);
  fn _ZN9QTextEdit6zoomInEi(arg0: c_int) -> i32;
  // proto: void QTextEdit::setDocumentTitle(const QString & title);
  fn _ZN9QTextEdit16setDocumentTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QTextEdit::canPaste();
  fn _ZNK9QTextEdit8canPasteEv() -> i32;
  // proto: QString QTextEdit::toHtml();
  fn _ZNK9QTextEdit6toHtmlEv() -> i32;
  // proto: QMenu * QTextEdit::createStandardContextMenu(const QPoint & position);
  fn _ZN9QTextEdit25createStandardContextMenuERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::cursorPositionChanged();
  fn _ZN9QTextEdit21cursorPositionChangedEv() -> i32;
  // proto: void QTextEdit::setTabStopWidth(int width);
  fn _ZN9QTextEdit15setTabStopWidthEi(arg0: c_int) -> i32;
  // proto: void QTextEdit::undoAvailable(bool b);
  fn _ZN9QTextEdit13undoAvailableEb(arg0: int8_t) -> i32;
  // proto: QString QTextEdit::documentTitle();
  fn _ZNK9QTextEdit13documentTitleEv() -> i32;
  // proto: bool QTextEdit::isUndoRedoEnabled();
  fn _ZNK9QTextEdit17isUndoRedoEnabledEv() -> i32;
  // proto: void QTextEdit::setText(const QString & text);
  fn _ZN9QTextEdit7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::ensureCursorVisible();
  fn _ZN9QTextEdit19ensureCursorVisibleEv() -> i32;
  // proto: void QTextEdit::setAcceptRichText(bool accept);
  fn _ZN9QTextEdit17setAcceptRichTextEb(arg0: int8_t) -> i32;
  // proto: void QTextEdit::setPlaceholderText(const QString & placeholderText);
  fn _ZN9QTextEdit18setPlaceholderTextERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QTextEdit::isReadOnly();
  fn _ZNK9QTextEdit10isReadOnlyEv() -> i32;
  // proto: void QTextEdit::setUndoRedoEnabled(bool enable);
  fn _ZN9QTextEdit18setUndoRedoEnabledEb(arg0: int8_t) -> i32;
  // proto: void QTextEdit::NewQTextEdit(const QTextEdit & );
  fn _ZN9QTextEditC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QTextCharFormat QTextEdit::currentCharFormat();
  fn _ZNK9QTextEdit17currentCharFormatEv() -> i32;
  // proto: QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
  fn _ZNK9QTextEdit17cursorForPositionERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::scrollToAnchor(const QString & name);
  fn _ZN9QTextEdit14scrollToAnchorERK7QString(arg0: *const c_void) -> i32;
  // proto: QFont QTextEdit::currentFont();
  fn _ZNK9QTextEdit11currentFontEv() -> i32;
  // proto: void QTextEdit::paste();
  fn _ZN9QTextEdit5pasteEv() -> i32;
  // proto: void QTextEdit::setTextCursor(const QTextCursor & cursor);
  fn _ZN9QTextEdit13setTextCursorERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn _ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat(arg0: *const c_void) -> i32;
  // proto: QVariant QTextEdit::loadResource(int type, const QUrl & name);
  fn _ZN9QTextEdit12loadResourceEiRK4QUrl(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTextEdit::setTabChangesFocus(bool b);
  fn _ZN9QTextEdit18setTabChangesFocusEb(arg0: int8_t) -> i32;
  // proto: void QTextEdit::selectionChanged();
  fn _ZN9QTextEdit16selectionChangedEv() -> i32;
  // proto: void QTextEdit::setHtml(const QString & text);
  fn _ZN9QTextEdit7setHtmlERK7QString(arg0: *const c_void) -> i32;
  // proto: QRect QTextEdit::cursorRect(const QTextCursor & cursor);
  fn _ZNK9QTextEdit10cursorRectERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::setLineWrapColumnOrWidth(int w);
  fn _ZN9QTextEdit24setLineWrapColumnOrWidthEi(arg0: c_int) -> i32;
  // proto: void QTextEdit::setFontItalic(bool b);
  fn _ZN9QTextEdit13setFontItalicEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QTextEdit::metaObject();
  fn _ZNK9QTextEdit10metaObjectEv() -> i32;
  // proto: void QTextEdit::setCurrentFont(const QFont & f);
  fn _ZN9QTextEdit14setCurrentFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: bool QTextEdit::tabChangesFocus();
  fn _ZNK9QTextEdit15tabChangesFocusEv() -> i32;
  // proto: QColor QTextEdit::textBackgroundColor();
  fn _ZNK9QTextEdit19textBackgroundColorEv() -> i32;
  // proto: void QTextEdit::NewQTextEdit(const QString & text, QWidget * parent);
  fn _ZN9QTextEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QTextEdit::print(QPagedPaintDevice * printer);
  fn _ZNK9QTextEdit5printEP17QPagedPaintDevice(arg0: *mut c_void) -> i32;
  // proto: bool QTextEdit::fontUnderline();
  fn _ZNK9QTextEdit13fontUnderlineEv() -> i32;
  // proto: void QTextEdit::insertPlainText(const QString & text);
  fn _ZN9QTextEdit15insertPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: int QTextEdit::fontWeight();
  fn _ZNK9QTextEdit10fontWeightEv() -> i32;
  // proto: void QTextEdit::copyAvailable(bool b);
  fn _ZN9QTextEdit13copyAvailableEb(arg0: int8_t) -> i32;
  // proto: QColor QTextEdit::textColor();
  fn _ZNK9QTextEdit9textColorEv() -> i32;
  // proto: void QTextEdit::append(const QString & text);
  fn _ZN9QTextEdit6appendERK7QString(arg0: *const c_void) -> i32;
  // proto: void QTextEdit::redoAvailable(bool b);
  fn _ZN9QTextEdit13redoAvailableEb(arg0: int8_t) -> i32;
  // proto: void QTextEdit::NewQTextEdit(QWidget * parent);
  fn _ZN9QTextEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QTextEdit::setReadOnly(bool ro);
  fn _ZN9QTextEdit11setReadOnlyEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QTextEdit)=1
pub struct QTextEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextEdit {
  pub fn lineWrapColumnOrWidth<T: QTextEdit_lineWrapColumnOrWidth>(&mut self, value: T) -> i32 {
    value.lineWrapColumnOrWidth(self);
    return 1;
  }
}

pub trait QTextEdit_lineWrapColumnOrWidth {
  fn lineWrapColumnOrWidth(self, this: &mut QTextEdit) -> i32;
}

// proto: int QTextEdit::lineWrapColumnOrWidth();
impl<'a> /*trait*/ QTextEdit_lineWrapColumnOrWidth for () {
  fn lineWrapColumnOrWidth(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit21lineWrapColumnOrWidthEv()};
    unsafe {_ZNK9QTextEdit21lineWrapColumnOrWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontFamily<T: QTextEdit_setFontFamily>(&mut self, value: T) -> i32 {
    value.setFontFamily(self);
    return 1;
  }
}

pub trait QTextEdit_setFontFamily {
  fn setFontFamily(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setFontFamily(const QString & fontFamily);
impl<'a> /*trait*/ QTextEdit_setFontFamily for (&'a  QString) {
  fn setFontFamily(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit13setFontFamilyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn toPlainText<T: QTextEdit_toPlainText>(&mut self, value: T) -> i32 {
    value.toPlainText(self);
    return 1;
  }
}

pub trait QTextEdit_toPlainText {
  fn toPlainText(self, this: &mut QTextEdit) -> i32;
}

// proto: QString QTextEdit::toPlainText();
impl<'a> /*trait*/ QTextEdit_toPlainText for () {
  fn toPlainText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11toPlainTextEv()};
    unsafe {_ZNK9QTextEdit11toPlainTextEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setCursorWidth<T: QTextEdit_setCursorWidth>(&mut self, value: T) -> i32 {
    value.setCursorWidth(self);
    return 1;
  }
}

pub trait QTextEdit_setCursorWidth {
  fn setCursorWidth(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QTextEdit_setCursorWidth for (i32) {
  fn setCursorWidth(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextEdit14setCursorWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn createStandardContextMenu<T: QTextEdit_createStandardContextMenu>(&mut self, value: T) -> i32 {
    value.createStandardContextMenu(self);
    return 1;
  }
}

pub trait QTextEdit_createStandardContextMenu {
  fn createStandardContextMenu(self, this: &mut QTextEdit) -> i32;
}

// proto: QMenu * QTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu for () {
  fn createStandardContextMenu(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit25createStandardContextMenuEv()};
    unsafe {_ZN9QTextEdit25createStandardContextMenuEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn document<T: QTextEdit_document>(&mut self, value: T) -> i32 {
    value.document(self);
    return 1;
  }
}

pub trait QTextEdit_document {
  fn document(self, this: &mut QTextEdit) -> i32;
}

// proto: QTextDocument * QTextEdit::document();
impl<'a> /*trait*/ QTextEdit_document for () {
  fn document(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8documentEv()};
    unsafe {_ZNK9QTextEdit8documentEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorRect<T: QTextEdit_cursorRect>(&mut self, value: T) -> i32 {
    value.cursorRect(self);
    return 1;
  }
}

pub trait QTextEdit_cursorRect {
  fn cursorRect(self, this: &mut QTextEdit) -> i32;
}

// proto: QRect QTextEdit::cursorRect();
impl<'a> /*trait*/ QTextEdit_cursorRect for () {
  fn cursorRect(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10cursorRectEv()};
    unsafe {_ZNK9QTextEdit10cursorRectEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTextColor<T: QTextEdit_setTextColor>(&mut self, value: T) -> i32 {
    value.setTextColor(self);
    return 1;
  }
}

pub trait QTextEdit_setTextColor {
  fn setTextColor(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setTextColor(const QColor & c);
impl<'a> /*trait*/ QTextEdit_setTextColor for (&'a  QColor) {
  fn setTextColor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit12setTextColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn acceptRichText<T: QTextEdit_acceptRichText>(&mut self, value: T) -> i32 {
    value.acceptRichText(self);
    return 1;
  }
}

pub trait QTextEdit_acceptRichText {
  fn acceptRichText(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::acceptRichText();
impl<'a> /*trait*/ QTextEdit_acceptRichText for () {
  fn acceptRichText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit14acceptRichTextEv()};
    unsafe {_ZNK9QTextEdit14acceptRichTextEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn clear<T: QTextEdit_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QTextEdit_clear {
  fn clear(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::clear();
impl<'a> /*trait*/ QTextEdit_clear for () {
  fn clear(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit5clearEv()};
    unsafe {_ZN9QTextEdit5clearEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn insertHtml<T: QTextEdit_insertHtml>(&mut self, value: T) -> i32 {
    value.insertHtml(self);
    return 1;
  }
}

pub trait QTextEdit_insertHtml {
  fn insertHtml(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::insertHtml(const QString & text);
impl<'a> /*trait*/ QTextEdit_insertHtml for (&'a  QString) {
  fn insertHtml(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit10insertHtmlERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit10insertHtmlERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontFamily<T: QTextEdit_fontFamily>(&mut self, value: T) -> i32 {
    value.fontFamily(self);
    return 1;
  }
}

pub trait QTextEdit_fontFamily {
  fn fontFamily(self, this: &mut QTextEdit) -> i32;
}

// proto: QString QTextEdit::fontFamily();
impl<'a> /*trait*/ QTextEdit_fontFamily for () {
  fn fontFamily(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontFamilyEv()};
    unsafe {_ZNK9QTextEdit10fontFamilyEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontUnderline<T: QTextEdit_setFontUnderline>(&mut self, value: T) -> i32 {
    value.setFontUnderline(self);
    return 1;
  }
}

pub trait QTextEdit_setFontUnderline {
  fn setFontUnderline(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setFontUnderline(bool b);
impl<'a> /*trait*/ QTextEdit_setFontUnderline for (i8) {
  fn setFontUnderline(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setFontUnderlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit16setFontUnderlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cut<T: QTextEdit_cut>(&mut self, value: T) -> i32 {
    value.cut(self);
    return 1;
  }
}

pub trait QTextEdit_cut {
  fn cut(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::cut();
impl<'a> /*trait*/ QTextEdit_cut for () {
  fn cut(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit3cutEv()};
    unsafe {_ZN9QTextEdit3cutEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn currentCharFormatChanged<T: QTextEdit_currentCharFormatChanged>(&mut self, value: T) -> i32 {
    value.currentCharFormatChanged(self);
    return 1;
  }
}

pub trait QTextEdit_currentCharFormatChanged {
  fn currentCharFormatChanged(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextEdit_currentCharFormatChanged for (&'a  QTextCharFormat) {
  fn currentCharFormatChanged(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn anchorAt<T: QTextEdit_anchorAt>(&mut self, value: T) -> i32 {
    value.anchorAt(self);
    return 1;
  }
}

pub trait QTextEdit_anchorAt {
  fn anchorAt(self, this: &mut QTextEdit) -> i32;
}

// proto: QString QTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QTextEdit_anchorAt for (&'a  QPoint) {
  fn anchorAt(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTextEdit8anchorAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorWidth<T: QTextEdit_cursorWidth>(&mut self, value: T) -> i32 {
    value.cursorWidth(self);
    return 1;
  }
}

pub trait QTextEdit_cursorWidth {
  fn cursorWidth(self, this: &mut QTextEdit) -> i32;
}

// proto: int QTextEdit::cursorWidth();
impl<'a> /*trait*/ QTextEdit_cursorWidth for () {
  fn cursorWidth(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11cursorWidthEv()};
    unsafe {_ZNK9QTextEdit11cursorWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTextBackgroundColor<T: QTextEdit_setTextBackgroundColor>(&mut self, value: T) -> i32 {
    value.setTextBackgroundColor(self);
    return 1;
  }
}

pub trait QTextEdit_setTextBackgroundColor {
  fn setTextBackgroundColor(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setTextBackgroundColor(const QColor & c);
impl<'a> /*trait*/ QTextEdit_setTextBackgroundColor for (&'a  QColor) {
  fn setTextBackgroundColor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit22setTextBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit22setTextBackgroundColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn tabStopWidth<T: QTextEdit_tabStopWidth>(&mut self, value: T) -> i32 {
    value.tabStopWidth(self);
    return 1;
  }
}

pub trait QTextEdit_tabStopWidth {
  fn tabStopWidth(self, this: &mut QTextEdit) -> i32;
}

// proto: int QTextEdit::tabStopWidth();
impl<'a> /*trait*/ QTextEdit_tabStopWidth for () {
  fn tabStopWidth(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit12tabStopWidthEv()};
    unsafe {_ZNK9QTextEdit12tabStopWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontWeight<T: QTextEdit_setFontWeight>(&mut self, value: T) -> i32 {
    value.setFontWeight(self);
    return 1;
  }
}

pub trait QTextEdit_setFontWeight {
  fn setFontWeight(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setFontWeight(int w);
impl<'a> /*trait*/ QTextEdit_setFontWeight for (i32) {
  fn setFontWeight(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontWeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextEdit13setFontWeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn selectAll<T: QTextEdit_selectAll>(&mut self, value: T) -> i32 {
    value.selectAll(self);
    return 1;
  }
}

pub trait QTextEdit_selectAll {
  fn selectAll(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::selectAll();
impl<'a> /*trait*/ QTextEdit_selectAll for () {
  fn selectAll(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit9selectAllEv()};
    unsafe {_ZN9QTextEdit9selectAllEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn zoomOut<T: QTextEdit_zoomOut>(&mut self, value: T) -> i32 {
    value.zoomOut(self);
    return 1;
  }
}

pub trait QTextEdit_zoomOut {
  fn zoomOut(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QTextEdit_zoomOut for (i32) {
  fn zoomOut(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextEdit7zoomOutEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn redo<T: QTextEdit_redo>(&mut self, value: T) -> i32 {
    value.redo(self);
    return 1;
  }
}

pub trait QTextEdit_redo {
  fn redo(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::redo();
impl<'a> /*trait*/ QTextEdit_redo for () {
  fn redo(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4redoEv()};
    unsafe {_ZN9QTextEdit4redoEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontPointSize<T: QTextEdit_setFontPointSize>(&mut self, value: T) -> i32 {
    value.setFontPointSize(self);
    return 1;
  }
}

pub trait QTextEdit_setFontPointSize {
  fn setFontPointSize(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setFontPointSize(qreal s);
impl<'a> /*trait*/ QTextEdit_setFontPointSize for (f64) {
  fn setFontPointSize(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setFontPointSizeEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN9QTextEdit16setFontPointSizeEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn overwriteMode<T: QTextEdit_overwriteMode>(&mut self, value: T) -> i32 {
    value.overwriteMode(self);
    return 1;
  }
}

pub trait QTextEdit_overwriteMode {
  fn overwriteMode(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::overwriteMode();
impl<'a> /*trait*/ QTextEdit_overwriteMode for () {
  fn overwriteMode(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13overwriteModeEv()};
    unsafe {_ZNK9QTextEdit13overwriteModeEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textCursor<T: QTextEdit_textCursor>(&mut self, value: T) -> i32 {
    value.textCursor(self);
    return 1;
  }
}

pub trait QTextEdit_textCursor {
  fn textCursor(self, this: &mut QTextEdit) -> i32;
}

// proto: QTextCursor QTextEdit::textCursor();
impl<'a> /*trait*/ QTextEdit_textCursor for () {
  fn textCursor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10textCursorEv()};
    unsafe {_ZNK9QTextEdit10textCursorEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn mergeCurrentCharFormat<T: QTextEdit_mergeCurrentCharFormat>(&mut self, value: T) -> i32 {
    value.mergeCurrentCharFormat(self);
    return 1;
  }
}

pub trait QTextEdit_mergeCurrentCharFormat {
  fn mergeCurrentCharFormat(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextEdit_mergeCurrentCharFormat for (&'a  QTextCharFormat) {
  fn mergeCurrentCharFormat(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setPlainText<T: QTextEdit_setPlainText>(&mut self, value: T) -> i32 {
    value.setPlainText(self);
    return 1;
  }
}

pub trait QTextEdit_setPlainText {
  fn setPlainText(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextEdit_setPlainText for (&'a  QString) {
  fn setPlainText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit12setPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn placeholderText<T: QTextEdit_placeholderText>(&mut self, value: T) -> i32 {
    value.placeholderText(self);
    return 1;
  }
}

pub trait QTextEdit_placeholderText {
  fn placeholderText(self, this: &mut QTextEdit) -> i32;
}

// proto: QString QTextEdit::placeholderText();
impl<'a> /*trait*/ QTextEdit_placeholderText for () {
  fn placeholderText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit15placeholderTextEv()};
    unsafe {_ZNK9QTextEdit15placeholderTextEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn FreeQTextEdit<T: QTextEdit_FreeQTextEdit>(&mut self, value: T) -> i32 {
    value.FreeQTextEdit(self);
    return 1;
  }
}

pub trait QTextEdit_FreeQTextEdit {
  fn FreeQTextEdit(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::FreeQTextEdit();
impl<'a> /*trait*/ QTextEdit_FreeQTextEdit for () {
  fn FreeQTextEdit(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditD0Ev()};
    unsafe {_ZN9QTextEditD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontItalic<T: QTextEdit_fontItalic>(&mut self, value: T) -> i32 {
    value.fontItalic(self);
    return 1;
  }
}

pub trait QTextEdit_fontItalic {
  fn fontItalic(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::fontItalic();
impl<'a> /*trait*/ QTextEdit_fontItalic for () {
  fn fontItalic(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontItalicEv()};
    unsafe {_ZNK9QTextEdit10fontItalicEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn copy<T: QTextEdit_copy>(&mut self, value: T) -> i32 {
    value.copy(self);
    return 1;
  }
}

pub trait QTextEdit_copy {
  fn copy(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::copy();
impl<'a> /*trait*/ QTextEdit_copy for () {
  fn copy(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4copyEv()};
    unsafe {_ZN9QTextEdit4copyEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textChanged<T: QTextEdit_textChanged>(&mut self, value: T) -> i32 {
    value.textChanged(self);
    return 1;
  }
}

pub trait QTextEdit_textChanged {
  fn textChanged(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::textChanged();
impl<'a> /*trait*/ QTextEdit_textChanged for () {
  fn textChanged(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11textChangedEv()};
    unsafe {_ZN9QTextEdit11textChangedEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontPointSize<T: QTextEdit_fontPointSize>(&mut self, value: T) -> i32 {
    value.fontPointSize(self);
    return 1;
  }
}

pub trait QTextEdit_fontPointSize {
  fn fontPointSize(self, this: &mut QTextEdit) -> i32;
}

// proto: double QTextEdit::fontPointSize();
impl<'a> /*trait*/ QTextEdit_fontPointSize for () {
  fn fontPointSize(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13fontPointSizeEv()};
    unsafe {_ZNK9QTextEdit13fontPointSizeEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setDocument<T: QTextEdit_setDocument>(&mut self, value: T) -> i32 {
    value.setDocument(self);
    return 1;
  }
}

pub trait QTextEdit_setDocument {
  fn setDocument(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QTextEdit_setDocument for (&'a mut QTextDocument) {
  fn setDocument(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextEdit11setDocumentEP13QTextDocument(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setOverwriteMode<T: QTextEdit_setOverwriteMode>(&mut self, value: T) -> i32 {
    value.setOverwriteMode(self);
    return 1;
  }
}

pub trait QTextEdit_setOverwriteMode {
  fn setOverwriteMode(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QTextEdit_setOverwriteMode for (i8) {
  fn setOverwriteMode(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setOverwriteModeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit16setOverwriteModeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn undo<T: QTextEdit_undo>(&mut self, value: T) -> i32 {
    value.undo(self);
    return 1;
  }
}

pub trait QTextEdit_undo {
  fn undo(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::undo();
impl<'a> /*trait*/ QTextEdit_undo for () {
  fn undo(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4undoEv()};
    unsafe {_ZN9QTextEdit4undoEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn zoomIn<T: QTextEdit_zoomIn>(&mut self, value: T) -> i32 {
    value.zoomIn(self);
    return 1;
  }
}

pub trait QTextEdit_zoomIn {
  fn zoomIn(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QTextEdit_zoomIn for (i32) {
  fn zoomIn(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextEdit6zoomInEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setDocumentTitle<T: QTextEdit_setDocumentTitle>(&mut self, value: T) -> i32 {
    value.setDocumentTitle(self);
    return 1;
  }
}

pub trait QTextEdit_setDocumentTitle {
  fn setDocumentTitle(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QTextEdit_setDocumentTitle for (&'a  QString) {
  fn setDocumentTitle(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit16setDocumentTitleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn canPaste<T: QTextEdit_canPaste>(&mut self, value: T) -> i32 {
    value.canPaste(self);
    return 1;
  }
}

pub trait QTextEdit_canPaste {
  fn canPaste(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::canPaste();
impl<'a> /*trait*/ QTextEdit_canPaste for () {
  fn canPaste(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8canPasteEv()};
    unsafe {_ZNK9QTextEdit8canPasteEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn toHtml<T: QTextEdit_toHtml>(&mut self, value: T) -> i32 {
    value.toHtml(self);
    return 1;
  }
}

pub trait QTextEdit_toHtml {
  fn toHtml(self, this: &mut QTextEdit) -> i32;
}

// proto: QString QTextEdit::toHtml();
impl<'a> /*trait*/ QTextEdit_toHtml for () {
  fn toHtml(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit6toHtmlEv()};
    unsafe {_ZNK9QTextEdit6toHtmlEv()};
    return 1;
  }
}

// proto: QMenu * QTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu for (&'a  QPoint) {
  fn createStandardContextMenu(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit25createStandardContextMenuERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorPositionChanged<T: QTextEdit_cursorPositionChanged>(&mut self, value: T) -> i32 {
    value.cursorPositionChanged(self);
    return 1;
  }
}

pub trait QTextEdit_cursorPositionChanged {
  fn cursorPositionChanged(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::cursorPositionChanged();
impl<'a> /*trait*/ QTextEdit_cursorPositionChanged for () {
  fn cursorPositionChanged(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit21cursorPositionChangedEv()};
    unsafe {_ZN9QTextEdit21cursorPositionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTabStopWidth<T: QTextEdit_setTabStopWidth>(&mut self, value: T) -> i32 {
    value.setTabStopWidth(self);
    return 1;
  }
}

pub trait QTextEdit_setTabStopWidth {
  fn setTabStopWidth(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QTextEdit_setTabStopWidth for (i32) {
  fn setTabStopWidth(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextEdit15setTabStopWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn undoAvailable<T: QTextEdit_undoAvailable>(&mut self, value: T) -> i32 {
    value.undoAvailable(self);
    return 1;
  }
}

pub trait QTextEdit_undoAvailable {
  fn undoAvailable(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::undoAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_undoAvailable for (i8) {
  fn undoAvailable(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13undoAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit13undoAvailableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn documentTitle<T: QTextEdit_documentTitle>(&mut self, value: T) -> i32 {
    value.documentTitle(self);
    return 1;
  }
}

pub trait QTextEdit_documentTitle {
  fn documentTitle(self, this: &mut QTextEdit) -> i32;
}

// proto: QString QTextEdit::documentTitle();
impl<'a> /*trait*/ QTextEdit_documentTitle for () {
  fn documentTitle(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13documentTitleEv()};
    unsafe {_ZNK9QTextEdit13documentTitleEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn isUndoRedoEnabled<T: QTextEdit_isUndoRedoEnabled>(&mut self, value: T) -> i32 {
    value.isUndoRedoEnabled(self);
    return 1;
  }
}

pub trait QTextEdit_isUndoRedoEnabled {
  fn isUndoRedoEnabled(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextEdit_isUndoRedoEnabled for () {
  fn isUndoRedoEnabled(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17isUndoRedoEnabledEv()};
    unsafe {_ZNK9QTextEdit17isUndoRedoEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setText<T: QTextEdit_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QTextEdit_setText {
  fn setText(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setText(const QString & text);
impl<'a> /*trait*/ QTextEdit_setText for (&'a  QString) {
  fn setText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn ensureCursorVisible<T: QTextEdit_ensureCursorVisible>(&mut self, value: T) -> i32 {
    value.ensureCursorVisible(self);
    return 1;
  }
}

pub trait QTextEdit_ensureCursorVisible {
  fn ensureCursorVisible(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QTextEdit_ensureCursorVisible for () {
  fn ensureCursorVisible(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit19ensureCursorVisibleEv()};
    unsafe {_ZN9QTextEdit19ensureCursorVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setAcceptRichText<T: QTextEdit_setAcceptRichText>(&mut self, value: T) -> i32 {
    value.setAcceptRichText(self);
    return 1;
  }
}

pub trait QTextEdit_setAcceptRichText {
  fn setAcceptRichText(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setAcceptRichText(bool accept);
impl<'a> /*trait*/ QTextEdit_setAcceptRichText for (i8) {
  fn setAcceptRichText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit17setAcceptRichTextEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit17setAcceptRichTextEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setPlaceholderText<T: QTextEdit_setPlaceholderText>(&mut self, value: T) -> i32 {
    value.setPlaceholderText(self);
    return 1;
  }
}

pub trait QTextEdit_setPlaceholderText {
  fn setPlaceholderText(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QTextEdit_setPlaceholderText for (&'a  QString) {
  fn setPlaceholderText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit18setPlaceholderTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn isReadOnly<T: QTextEdit_isReadOnly>(&mut self, value: T) -> i32 {
    value.isReadOnly(self);
    return 1;
  }
}

pub trait QTextEdit_isReadOnly {
  fn isReadOnly(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::isReadOnly();
impl<'a> /*trait*/ QTextEdit_isReadOnly for () {
  fn isReadOnly(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10isReadOnlyEv()};
    unsafe {_ZNK9QTextEdit10isReadOnlyEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setUndoRedoEnabled<T: QTextEdit_setUndoRedoEnabled>(&mut self, value: T) -> i32 {
    value.setUndoRedoEnabled(self);
    return 1;
  }
}

pub trait QTextEdit_setUndoRedoEnabled {
  fn setUndoRedoEnabled(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextEdit_setUndoRedoEnabled for (i8) {
  fn setUndoRedoEnabled(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit18setUndoRedoEnabledEb(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEditC1ERKS_(qthis, arg0)};
    let rsthis = QTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn currentCharFormat<T: QTextEdit_currentCharFormat>(&mut self, value: T) -> i32 {
    value.currentCharFormat(self);
    return 1;
  }
}

pub trait QTextEdit_currentCharFormat {
  fn currentCharFormat(self, this: &mut QTextEdit) -> i32;
}

// proto: QTextCharFormat QTextEdit::currentCharFormat();
impl<'a> /*trait*/ QTextEdit_currentCharFormat for () {
  fn currentCharFormat(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17currentCharFormatEv()};
    unsafe {_ZNK9QTextEdit17currentCharFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn cursorForPosition<T: QTextEdit_cursorForPosition>(&mut self, value: T) -> i32 {
    value.cursorForPosition(self);
    return 1;
  }
}

pub trait QTextEdit_cursorForPosition {
  fn cursorForPosition(self, this: &mut QTextEdit) -> i32;
}

// proto: QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QTextEdit_cursorForPosition for (&'a  QPoint) {
  fn cursorForPosition(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTextEdit17cursorForPositionERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn scrollToAnchor<T: QTextEdit_scrollToAnchor>(&mut self, value: T) -> i32 {
    value.scrollToAnchor(self);
    return 1;
  }
}

pub trait QTextEdit_scrollToAnchor {
  fn scrollToAnchor(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::scrollToAnchor(const QString & name);
impl<'a> /*trait*/ QTextEdit_scrollToAnchor for (&'a  QString) {
  fn scrollToAnchor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14scrollToAnchorERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit14scrollToAnchorERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn currentFont<T: QTextEdit_currentFont>(&mut self, value: T) -> i32 {
    value.currentFont(self);
    return 1;
  }
}

pub trait QTextEdit_currentFont {
  fn currentFont(self, this: &mut QTextEdit) -> i32;
}

// proto: QFont QTextEdit::currentFont();
impl<'a> /*trait*/ QTextEdit_currentFont for () {
  fn currentFont(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11currentFontEv()};
    unsafe {_ZNK9QTextEdit11currentFontEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn paste<T: QTextEdit_paste>(&mut self, value: T) -> i32 {
    value.paste(self);
    return 1;
  }
}

pub trait QTextEdit_paste {
  fn paste(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::paste();
impl<'a> /*trait*/ QTextEdit_paste for () {
  fn paste(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit5pasteEv()};
    unsafe {_ZN9QTextEdit5pasteEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTextCursor<T: QTextEdit_setTextCursor>(&mut self, value: T) -> i32 {
    value.setTextCursor(self);
    return 1;
  }
}

pub trait QTextEdit_setTextCursor {
  fn setTextCursor(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextEdit_setTextCursor for (&'a  QTextCursor) {
  fn setTextCursor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit13setTextCursorERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setCurrentCharFormat<T: QTextEdit_setCurrentCharFormat>(&mut self, value: T) -> i32 {
    value.setCurrentCharFormat(self);
    return 1;
  }
}

pub trait QTextEdit_setCurrentCharFormat {
  fn setCurrentCharFormat(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextEdit_setCurrentCharFormat for (&'a  QTextCharFormat) {
  fn setCurrentCharFormat(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn loadResource<T: QTextEdit_loadResource>(&mut self, value: T) -> i32 {
    value.loadResource(self);
    return 1;
  }
}

pub trait QTextEdit_loadResource {
  fn loadResource(self, this: &mut QTextEdit) -> i32;
}

// proto: QVariant QTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextEdit_loadResource for (i32, &'a  QUrl) {
  fn loadResource(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit12loadResourceEiRK4QUrl(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setTabChangesFocus<T: QTextEdit_setTabChangesFocus>(&mut self, value: T) -> i32 {
    value.setTabChangesFocus(self);
    return 1;
  }
}

pub trait QTextEdit_setTabChangesFocus {
  fn setTabChangesFocus(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QTextEdit_setTabChangesFocus for (i8) {
  fn setTabChangesFocus(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit18setTabChangesFocusEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn selectionChanged<T: QTextEdit_selectionChanged>(&mut self, value: T) -> i32 {
    value.selectionChanged(self);
    return 1;
  }
}

pub trait QTextEdit_selectionChanged {
  fn selectionChanged(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::selectionChanged();
impl<'a> /*trait*/ QTextEdit_selectionChanged for () {
  fn selectionChanged(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16selectionChangedEv()};
    unsafe {_ZN9QTextEdit16selectionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setHtml<T: QTextEdit_setHtml>(&mut self, value: T) -> i32 {
    value.setHtml(self);
    return 1;
  }
}

pub trait QTextEdit_setHtml {
  fn setHtml(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setHtml(const QString & text);
impl<'a> /*trait*/ QTextEdit_setHtml for (&'a  QString) {
  fn setHtml(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit7setHtmlERK7QString(arg0)};
    return 1;
  }
}

// proto: QRect QTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextEdit_cursorRect for (&'a  QTextCursor) {
  fn cursorRect(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTextEdit10cursorRectERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setLineWrapColumnOrWidth<T: QTextEdit_setLineWrapColumnOrWidth>(&mut self, value: T) -> i32 {
    value.setLineWrapColumnOrWidth(self);
    return 1;
  }
}

pub trait QTextEdit_setLineWrapColumnOrWidth {
  fn setLineWrapColumnOrWidth(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setLineWrapColumnOrWidth(int w);
impl<'a> /*trait*/ QTextEdit_setLineWrapColumnOrWidth for (i32) {
  fn setLineWrapColumnOrWidth(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit24setLineWrapColumnOrWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextEdit24setLineWrapColumnOrWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setFontItalic<T: QTextEdit_setFontItalic>(&mut self, value: T) -> i32 {
    value.setFontItalic(self);
    return 1;
  }
}

pub trait QTextEdit_setFontItalic {
  fn setFontItalic(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setFontItalic(bool b);
impl<'a> /*trait*/ QTextEdit_setFontItalic for (i8) {
  fn setFontItalic(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontItalicEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit13setFontItalicEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn metaObject<T: QTextEdit_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextEdit_metaObject {
  fn metaObject(self, this: &mut QTextEdit) -> i32;
}

// proto: const QMetaObject * QTextEdit::metaObject();
impl<'a> /*trait*/ QTextEdit_metaObject for () {
  fn metaObject(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10metaObjectEv()};
    unsafe {_ZNK9QTextEdit10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn setCurrentFont<T: QTextEdit_setCurrentFont>(&mut self, value: T) -> i32 {
    value.setCurrentFont(self);
    return 1;
  }
}

pub trait QTextEdit_setCurrentFont {
  fn setCurrentFont(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QTextEdit_setCurrentFont for (&'a  QFont) {
  fn setCurrentFont(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit14setCurrentFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn tabChangesFocus<T: QTextEdit_tabChangesFocus>(&mut self, value: T) -> i32 {
    value.tabChangesFocus(self);
    return 1;
  }
}

pub trait QTextEdit_tabChangesFocus {
  fn tabChangesFocus(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QTextEdit_tabChangesFocus for () {
  fn tabChangesFocus(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit15tabChangesFocusEv()};
    unsafe {_ZNK9QTextEdit15tabChangesFocusEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textBackgroundColor<T: QTextEdit_textBackgroundColor>(&mut self, value: T) -> i32 {
    value.textBackgroundColor(self);
    return 1;
  }
}

pub trait QTextEdit_textBackgroundColor {
  fn textBackgroundColor(self, this: &mut QTextEdit) -> i32;
}

// proto: QColor QTextEdit::textBackgroundColor();
impl<'a> /*trait*/ QTextEdit_textBackgroundColor for () {
  fn textBackgroundColor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit19textBackgroundColorEv()};
    unsafe {_ZNK9QTextEdit19textBackgroundColorEv()};
    return 1;
  }
}

// proto: void QTextEdit::NewQTextEdit(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QTextEdit_NewQTextEdit for (&'a  QString, &'a mut QWidget) {
  fn NewQTextEdit(self) -> QTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn print<T: QTextEdit_print>(&mut self, value: T) -> i32 {
    value.print(self);
    return 1;
  }
}

pub trait QTextEdit_print {
  fn print(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextEdit_print for (&'a mut QPagedPaintDevice) {
  fn print(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK9QTextEdit5printEP17QPagedPaintDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontUnderline<T: QTextEdit_fontUnderline>(&mut self, value: T) -> i32 {
    value.fontUnderline(self);
    return 1;
  }
}

pub trait QTextEdit_fontUnderline {
  fn fontUnderline(self, this: &mut QTextEdit) -> i32;
}

// proto: bool QTextEdit::fontUnderline();
impl<'a> /*trait*/ QTextEdit_fontUnderline for () {
  fn fontUnderline(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13fontUnderlineEv()};
    unsafe {_ZNK9QTextEdit13fontUnderlineEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn insertPlainText<T: QTextEdit_insertPlainText>(&mut self, value: T) -> i32 {
    value.insertPlainText(self);
    return 1;
  }
}

pub trait QTextEdit_insertPlainText {
  fn insertPlainText(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QTextEdit_insertPlainText for (&'a  QString) {
  fn insertPlainText(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit15insertPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn fontWeight<T: QTextEdit_fontWeight>(&mut self, value: T) -> i32 {
    value.fontWeight(self);
    return 1;
  }
}

pub trait QTextEdit_fontWeight {
  fn fontWeight(self, this: &mut QTextEdit) -> i32;
}

// proto: int QTextEdit::fontWeight();
impl<'a> /*trait*/ QTextEdit_fontWeight for () {
  fn fontWeight(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontWeightEv()};
    unsafe {_ZNK9QTextEdit10fontWeightEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn copyAvailable<T: QTextEdit_copyAvailable>(&mut self, value: T) -> i32 {
    value.copyAvailable(self);
    return 1;
  }
}

pub trait QTextEdit_copyAvailable {
  fn copyAvailable(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::copyAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_copyAvailable for (i8) {
  fn copyAvailable(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13copyAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit13copyAvailableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn textColor<T: QTextEdit_textColor>(&mut self, value: T) -> i32 {
    value.textColor(self);
    return 1;
  }
}

pub trait QTextEdit_textColor {
  fn textColor(self, this: &mut QTextEdit) -> i32;
}

// proto: QColor QTextEdit::textColor();
impl<'a> /*trait*/ QTextEdit_textColor for () {
  fn textColor(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit9textColorEv()};
    unsafe {_ZNK9QTextEdit9textColorEv()};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn append<T: QTextEdit_append>(&mut self, value: T) -> i32 {
    value.append(self);
    return 1;
  }
}

pub trait QTextEdit_append {
  fn append(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::append(const QString & text);
impl<'a> /*trait*/ QTextEdit_append for (&'a  QString) {
  fn append(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit6appendERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextEdit6appendERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEdit {
  pub fn redoAvailable<T: QTextEdit_redoAvailable>(&mut self, value: T) -> i32 {
    value.redoAvailable(self);
    return 1;
  }
}

pub trait QTextEdit_redoAvailable {
  fn redoAvailable(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::redoAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_redoAvailable for (i8) {
  fn redoAvailable(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13redoAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit13redoAvailableEb(arg0)};
    return 1;
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
  pub fn setReadOnly<T: QTextEdit_setReadOnly>(&mut self, value: T) -> i32 {
    value.setReadOnly(self);
    return 1;
  }
}

pub trait QTextEdit_setReadOnly {
  fn setReadOnly(self, this: &mut QTextEdit) -> i32;
}

// proto: void QTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QTextEdit_setReadOnly for (i8) {
  fn setReadOnly(self, this: &mut QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11setReadOnlyEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextEdit11setReadOnlyEb(arg0)};
    return 1;
  }
}

