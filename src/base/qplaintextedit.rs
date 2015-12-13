// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qurl::QUrl;
use super::qtextcharformat::QTextCharFormat;
use super::qtextdocument::QTextDocument;
use super::qpagedpaintdevice::QPagedPaintDevice;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qtextcursor::QTextCursor;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
  fn _ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::ensureCursorVisible();
  fn _ZN14QPlainTextEdit19ensureCursorVisibleEv() -> i32;
  // proto: QTextDocument * QPlainTextEdit::document();
  fn _ZNK14QPlainTextEdit8documentEv() -> i32;
  // proto: QRect QPlainTextEdit::cursorRect();
  fn _ZNK14QPlainTextEdit10cursorRectEv() -> i32;
  // proto: void QPlainTextEdit::setTabChangesFocus(bool b);
  fn _ZN14QPlainTextEdit18setTabChangesFocusEb(arg0: int8_t) -> i32;
  // proto: QString QPlainTextEdit::toPlainText();
  fn _ZNK14QPlainTextEdit11toPlainTextEv() -> i32;
  // proto: QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
  fn _ZN14QPlainTextEdit12loadResourceEiRK4QUrl(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: int QPlainTextEdit::tabStopWidth();
  fn _ZNK14QPlainTextEdit12tabStopWidthEv() -> i32;
  // proto: bool QPlainTextEdit::isReadOnly();
  fn _ZNK14QPlainTextEdit10isReadOnlyEv() -> i32;
  // proto: void QPlainTextEdit::setReadOnly(bool ro);
  fn _ZN14QPlainTextEdit11setReadOnlyEb(arg0: int8_t) -> i32;
  // proto: QTextCursor QPlainTextEdit::textCursor();
  fn _ZNK14QPlainTextEdit10textCursorEv() -> i32;
  // proto: void QPlainTextEdit::setCenterOnScroll(bool enabled);
  fn _ZN14QPlainTextEdit17setCenterOnScrollEb(arg0: int8_t) -> i32;
  // proto: QString QPlainTextEdit::placeholderText();
  fn _ZNK14QPlainTextEdit15placeholderTextEv() -> i32;
  // proto: int QPlainTextEdit::blockCount();
  fn _ZNK14QPlainTextEdit10blockCountEv() -> i32;
  // proto: void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn _ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::redoAvailable(bool b);
  fn _ZN14QPlainTextEdit13redoAvailableEb(arg0: int8_t) -> i32;
  // proto: void QPlainTextEdit::setDocument(QTextDocument * document);
  fn _ZN14QPlainTextEdit11setDocumentEP13QTextDocument(arg0: *mut c_void) -> i32;
  // proto: void QPlainTextEdit::print(QPagedPaintDevice * printer);
  fn _ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(arg0: *mut c_void) -> i32;
  // proto: void QPlainTextEdit::setTabStopWidth(int width);
  fn _ZN14QPlainTextEdit15setTabStopWidthEi(arg0: c_int) -> i32;
  // proto: bool QPlainTextEdit::backgroundVisible();
  fn _ZNK14QPlainTextEdit17backgroundVisibleEv() -> i32;
  // proto: void QPlainTextEdit::redo();
  fn _ZN14QPlainTextEdit4redoEv() -> i32;
  // proto: void QPlainTextEdit::NewQPlainTextEdit(const QString & text, QWidget * parent);
  fn _ZN14QPlainTextEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QPlainTextEdit::setOverwriteMode(bool overwrite);
  fn _ZN14QPlainTextEdit16setOverwriteModeEb(arg0: int8_t) -> i32;
  // proto: void QPlainTextEdit::undoAvailable(bool b);
  fn _ZN14QPlainTextEdit13undoAvailableEb(arg0: int8_t) -> i32;
  // proto: bool QPlainTextEdit::tabChangesFocus();
  fn _ZNK14QPlainTextEdit15tabChangesFocusEv() -> i32;
  // proto: void QPlainTextEdit::copy();
  fn _ZN14QPlainTextEdit4copyEv() -> i32;
  // proto: void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn _ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(arg0: *const c_void) -> i32;
  // proto: int QPlainTextEdit::maximumBlockCount();
  fn _ZNK14QPlainTextEdit17maximumBlockCountEv() -> i32;
  // proto: void QPlainTextEdit::insertPlainText(const QString & text);
  fn _ZN14QPlainTextEdit15insertPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
  fn _ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::paste();
  fn _ZN14QPlainTextEdit5pasteEv() -> i32;
  // proto: void QPlainTextEdit::zoomIn(int range);
  fn _ZN14QPlainTextEdit6zoomInEi(arg0: c_int) -> i32;
  // proto: void QPlainTextEdit::setMaximumBlockCount(int maximum);
  fn _ZN14QPlainTextEdit20setMaximumBlockCountEi(arg0: c_int) -> i32;
  // proto: QTextCharFormat QPlainTextEdit::currentCharFormat();
  fn _ZNK14QPlainTextEdit17currentCharFormatEv() -> i32;
  // proto: void QPlainTextEdit::selectionChanged();
  fn _ZN14QPlainTextEdit16selectionChangedEv() -> i32;
  // proto: void QPlainTextEdit::setCursorWidth(int width);
  fn _ZN14QPlainTextEdit14setCursorWidthEi(arg0: c_int) -> i32;
  // proto: QString QPlainTextEdit::documentTitle();
  fn _ZNK14QPlainTextEdit13documentTitleEv() -> i32;
  // proto: void QPlainTextEdit::selectAll();
  fn _ZN14QPlainTextEdit9selectAllEv() -> i32;
  // proto: void QPlainTextEdit::cursorPositionChanged();
  fn _ZN14QPlainTextEdit21cursorPositionChangedEv() -> i32;
  // proto: void QPlainTextEdit::NewQPlainTextEdit(const QPlainTextEdit & );
  fn _ZN14QPlainTextEditC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::setPlainText(const QString & text);
  fn _ZN14QPlainTextEdit12setPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::setBackgroundVisible(bool visible);
  fn _ZN14QPlainTextEdit20setBackgroundVisibleEb(arg0: int8_t) -> i32;
  // proto: void QPlainTextEdit::blockCountChanged(int newBlockCount);
  fn _ZN14QPlainTextEdit17blockCountChangedEi(arg0: c_int) -> i32;
  // proto: void QPlainTextEdit::setUndoRedoEnabled(bool enable);
  fn _ZN14QPlainTextEdit18setUndoRedoEnabledEb(arg0: int8_t) -> i32;
  // proto: bool QPlainTextEdit::overwriteMode();
  fn _ZNK14QPlainTextEdit13overwriteModeEv() -> i32;
  // proto: void QPlainTextEdit::centerCursor();
  fn _ZN14QPlainTextEdit12centerCursorEv() -> i32;
  // proto: const QMetaObject * QPlainTextEdit::metaObject();
  fn _ZNK14QPlainTextEdit10metaObjectEv() -> i32;
  // proto: void QPlainTextEdit::textChanged();
  fn _ZN14QPlainTextEdit11textChangedEv() -> i32;
  // proto: QMenu * QPlainTextEdit::createStandardContextMenu();
  fn _ZN14QPlainTextEdit25createStandardContextMenuEv() -> i32;
  // proto: void QPlainTextEdit::setDocumentTitle(const QString & title);
  fn _ZN14QPlainTextEdit16setDocumentTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::FreeQPlainTextEdit();
  fn _ZN14QPlainTextEditD0Ev() -> i32;
  // proto: void QPlainTextEdit::clear();
  fn _ZN14QPlainTextEdit5clearEv() -> i32;
  // proto: void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
  fn _ZN14QPlainTextEdit13updateRequestERK5QRecti(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QString QPlainTextEdit::anchorAt(const QPoint & pos);
  fn _ZNK14QPlainTextEdit8anchorAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: bool QPlainTextEdit::canPaste();
  fn _ZNK14QPlainTextEdit8canPasteEv() -> i32;
  // proto: void QPlainTextEdit::NewQPlainTextEdit(QWidget * parent);
  fn _ZN14QPlainTextEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QPlainTextEdit::cut();
  fn _ZN14QPlainTextEdit3cutEv() -> i32;
  // proto: void QPlainTextEdit::appendHtml(const QString & html);
  fn _ZN14QPlainTextEdit10appendHtmlERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QPlainTextEdit::isUndoRedoEnabled();
  fn _ZNK14QPlainTextEdit17isUndoRedoEnabledEv() -> i32;
  // proto: void QPlainTextEdit::zoomOut(int range);
  fn _ZN14QPlainTextEdit7zoomOutEi(arg0: c_int) -> i32;
  // proto: void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
  fn _ZN14QPlainTextEdit18setPlaceholderTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::undo();
  fn _ZN14QPlainTextEdit4undoEv() -> i32;
  // proto: void QPlainTextEdit::modificationChanged(bool );
  fn _ZN14QPlainTextEdit19modificationChangedEb(arg0: int8_t) -> i32;
  // proto: QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
  fn _ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(arg0: *const c_void) -> i32;
  // proto: bool QPlainTextEdit::centerOnScroll();
  fn _ZNK14QPlainTextEdit14centerOnScrollEv() -> i32;
  // proto: void QPlainTextEdit::appendPlainText(const QString & text);
  fn _ZN14QPlainTextEdit15appendPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: int QPlainTextEdit::cursorWidth();
  fn _ZNK14QPlainTextEdit11cursorWidthEv() -> i32;
  // proto: QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
  fn _ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: void QPlainTextEdit::copyAvailable(bool b);
  fn _ZN14QPlainTextEdit13copyAvailableEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QPlainTextEdit)=1
pub struct QPlainTextEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPlainTextEdit {
  pub fn createStandardContextMenu<T: QPlainTextEdit_createStandardContextMenu>(&mut self, value: T) -> i32 {
    value.createStandardContextMenu(self);
    return 1;
  }
}

pub trait QPlainTextEdit_createStandardContextMenu {
  fn createStandardContextMenu(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu for (&'a  QPoint) {
  fn createStandardContextMenu(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn ensureCursorVisible<T: QPlainTextEdit_ensureCursorVisible>(&mut self, value: T) -> i32 {
    value.ensureCursorVisible(self);
    return 1;
  }
}

pub trait QPlainTextEdit_ensureCursorVisible {
  fn ensureCursorVisible(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QPlainTextEdit_ensureCursorVisible for () {
  fn ensureCursorVisible(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19ensureCursorVisibleEv()};
    unsafe {_ZN14QPlainTextEdit19ensureCursorVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn document<T: QPlainTextEdit_document>(&mut self, value: T) -> i32 {
    value.document(self);
    return 1;
  }
}

pub trait QPlainTextEdit_document {
  fn document(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QTextDocument * QPlainTextEdit::document();
impl<'a> /*trait*/ QPlainTextEdit_document for () {
  fn document(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8documentEv()};
    unsafe {_ZNK14QPlainTextEdit8documentEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorRect<T: QPlainTextEdit_cursorRect>(&mut self, value: T) -> i32 {
    value.cursorRect(self);
    return 1;
  }
}

pub trait QPlainTextEdit_cursorRect {
  fn cursorRect(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QRect QPlainTextEdit::cursorRect();
impl<'a> /*trait*/ QPlainTextEdit_cursorRect for () {
  fn cursorRect(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectEv()};
    unsafe {_ZNK14QPlainTextEdit10cursorRectEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setTabChangesFocus<T: QPlainTextEdit_setTabChangesFocus>(&mut self, value: T) -> i32 {
    value.setTabChangesFocus(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setTabChangesFocus {
  fn setTabChangesFocus(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QPlainTextEdit_setTabChangesFocus for (i8) {
  fn setTabChangesFocus(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit18setTabChangesFocusEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn toPlainText<T: QPlainTextEdit_toPlainText>(&mut self, value: T) -> i32 {
    value.toPlainText(self);
    return 1;
  }
}

pub trait QPlainTextEdit_toPlainText {
  fn toPlainText(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QString QPlainTextEdit::toPlainText();
impl<'a> /*trait*/ QPlainTextEdit_toPlainText for () {
  fn toPlainText(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11toPlainTextEv()};
    unsafe {_ZNK14QPlainTextEdit11toPlainTextEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn loadResource<T: QPlainTextEdit_loadResource>(&mut self, value: T) -> i32 {
    value.loadResource(self);
    return 1;
  }
}

pub trait QPlainTextEdit_loadResource {
  fn loadResource(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QPlainTextEdit_loadResource for (i32, &'a  QUrl) {
  fn loadResource(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit12loadResourceEiRK4QUrl(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn tabStopWidth<T: QPlainTextEdit_tabStopWidth>(&mut self, value: T) -> i32 {
    value.tabStopWidth(self);
    return 1;
  }
}

pub trait QPlainTextEdit_tabStopWidth {
  fn tabStopWidth(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: int QPlainTextEdit::tabStopWidth();
impl<'a> /*trait*/ QPlainTextEdit_tabStopWidth for () {
  fn tabStopWidth(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit12tabStopWidthEv()};
    unsafe {_ZNK14QPlainTextEdit12tabStopWidthEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn isReadOnly<T: QPlainTextEdit_isReadOnly>(&mut self, value: T) -> i32 {
    value.isReadOnly(self);
    return 1;
  }
}

pub trait QPlainTextEdit_isReadOnly {
  fn isReadOnly(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::isReadOnly();
impl<'a> /*trait*/ QPlainTextEdit_isReadOnly for () {
  fn isReadOnly(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10isReadOnlyEv()};
    unsafe {_ZNK14QPlainTextEdit10isReadOnlyEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setReadOnly<T: QPlainTextEdit_setReadOnly>(&mut self, value: T) -> i32 {
    value.setReadOnly(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setReadOnly {
  fn setReadOnly(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QPlainTextEdit_setReadOnly for (i8) {
  fn setReadOnly(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setReadOnlyEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit11setReadOnlyEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn textCursor<T: QPlainTextEdit_textCursor>(&mut self, value: T) -> i32 {
    value.textCursor(self);
    return 1;
  }
}

pub trait QPlainTextEdit_textCursor {
  fn textCursor(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QTextCursor QPlainTextEdit::textCursor();
impl<'a> /*trait*/ QPlainTextEdit_textCursor for () {
  fn textCursor(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10textCursorEv()};
    unsafe {_ZNK14QPlainTextEdit10textCursorEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setCenterOnScroll<T: QPlainTextEdit_setCenterOnScroll>(&mut self, value: T) -> i32 {
    value.setCenterOnScroll(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setCenterOnScroll {
  fn setCenterOnScroll(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setCenterOnScroll(bool enabled);
impl<'a> /*trait*/ QPlainTextEdit_setCenterOnScroll for (i8) {
  fn setCenterOnScroll(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17setCenterOnScrollEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit17setCenterOnScrollEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn placeholderText<T: QPlainTextEdit_placeholderText>(&mut self, value: T) -> i32 {
    value.placeholderText(self);
    return 1;
  }
}

pub trait QPlainTextEdit_placeholderText {
  fn placeholderText(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QString QPlainTextEdit::placeholderText();
impl<'a> /*trait*/ QPlainTextEdit_placeholderText for () {
  fn placeholderText(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15placeholderTextEv()};
    unsafe {_ZNK14QPlainTextEdit15placeholderTextEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn blockCount<T: QPlainTextEdit_blockCount>(&mut self, value: T) -> i32 {
    value.blockCount(self);
    return 1;
  }
}

pub trait QPlainTextEdit_blockCount {
  fn blockCount(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: int QPlainTextEdit::blockCount();
impl<'a> /*trait*/ QPlainTextEdit_blockCount for () {
  fn blockCount(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10blockCountEv()};
    unsafe {_ZNK14QPlainTextEdit10blockCountEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setCurrentCharFormat<T: QPlainTextEdit_setCurrentCharFormat>(&mut self, value: T) -> i32 {
    value.setCurrentCharFormat(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setCurrentCharFormat {
  fn setCurrentCharFormat(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QPlainTextEdit_setCurrentCharFormat for (&'a  QTextCharFormat) {
  fn setCurrentCharFormat(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn redoAvailable<T: QPlainTextEdit_redoAvailable>(&mut self, value: T) -> i32 {
    value.redoAvailable(self);
    return 1;
  }
}

pub trait QPlainTextEdit_redoAvailable {
  fn redoAvailable(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::redoAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_redoAvailable for (i8) {
  fn redoAvailable(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13redoAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit13redoAvailableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setDocument<T: QPlainTextEdit_setDocument>(&mut self, value: T) -> i32 {
    value.setDocument(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setDocument {
  fn setDocument(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QPlainTextEdit_setDocument for (&'a mut QTextDocument) {
  fn setDocument(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QPlainTextEdit11setDocumentEP13QTextDocument(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn print<T: QPlainTextEdit_print>(&mut self, value: T) -> i32 {
    value.print(self);
    return 1;
  }
}

pub trait QPlainTextEdit_print {
  fn print(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QPlainTextEdit_print for (&'a mut QPagedPaintDevice) {
  fn print(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setTabStopWidth<T: QPlainTextEdit_setTabStopWidth>(&mut self, value: T) -> i32 {
    value.setTabStopWidth(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setTabStopWidth {
  fn setTabStopWidth(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setTabStopWidth for (i32) {
  fn setTabStopWidth(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QPlainTextEdit15setTabStopWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn backgroundVisible<T: QPlainTextEdit_backgroundVisible>(&mut self, value: T) -> i32 {
    value.backgroundVisible(self);
    return 1;
  }
}

pub trait QPlainTextEdit_backgroundVisible {
  fn backgroundVisible(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::backgroundVisible();
impl<'a> /*trait*/ QPlainTextEdit_backgroundVisible for () {
  fn backgroundVisible(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17backgroundVisibleEv()};
    unsafe {_ZNK14QPlainTextEdit17backgroundVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn redo<T: QPlainTextEdit_redo>(&mut self, value: T) -> i32 {
    value.redo(self);
    return 1;
  }
}

pub trait QPlainTextEdit_redo {
  fn redo(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::redo();
impl<'a> /*trait*/ QPlainTextEdit_redo for () {
  fn redo(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4redoEv()};
    unsafe {_ZN14QPlainTextEdit4redoEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn NewQPlainTextEdit<T: QPlainTextEdit_NewQPlainTextEdit>(value: T) -> QPlainTextEdit {
    let rsthis = value.NewQPlainTextEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextEdit_NewQPlainTextEdit {
  fn NewQPlainTextEdit(self) -> QPlainTextEdit;
}

// proto: void QPlainTextEdit::NewQPlainTextEdit(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (&'a  QString, &'a mut QWidget) {
  fn NewQPlainTextEdit(self) -> QPlainTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QPlainTextEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QPlainTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setOverwriteMode<T: QPlainTextEdit_setOverwriteMode>(&mut self, value: T) -> i32 {
    value.setOverwriteMode(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setOverwriteMode {
  fn setOverwriteMode(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QPlainTextEdit_setOverwriteMode for (i8) {
  fn setOverwriteMode(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setOverwriteModeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit16setOverwriteModeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn undoAvailable<T: QPlainTextEdit_undoAvailable>(&mut self, value: T) -> i32 {
    value.undoAvailable(self);
    return 1;
  }
}

pub trait QPlainTextEdit_undoAvailable {
  fn undoAvailable(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::undoAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_undoAvailable for (i8) {
  fn undoAvailable(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13undoAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit13undoAvailableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn tabChangesFocus<T: QPlainTextEdit_tabChangesFocus>(&mut self, value: T) -> i32 {
    value.tabChangesFocus(self);
    return 1;
  }
}

pub trait QPlainTextEdit_tabChangesFocus {
  fn tabChangesFocus(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QPlainTextEdit_tabChangesFocus for () {
  fn tabChangesFocus(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15tabChangesFocusEv()};
    unsafe {_ZNK14QPlainTextEdit15tabChangesFocusEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn copy<T: QPlainTextEdit_copy>(&mut self, value: T) -> i32 {
    value.copy(self);
    return 1;
  }
}

pub trait QPlainTextEdit_copy {
  fn copy(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::copy();
impl<'a> /*trait*/ QPlainTextEdit_copy for () {
  fn copy(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4copyEv()};
    unsafe {_ZN14QPlainTextEdit4copyEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn mergeCurrentCharFormat<T: QPlainTextEdit_mergeCurrentCharFormat>(&mut self, value: T) -> i32 {
    value.mergeCurrentCharFormat(self);
    return 1;
  }
}

pub trait QPlainTextEdit_mergeCurrentCharFormat {
  fn mergeCurrentCharFormat(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QPlainTextEdit_mergeCurrentCharFormat for (&'a  QTextCharFormat) {
  fn mergeCurrentCharFormat(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn maximumBlockCount<T: QPlainTextEdit_maximumBlockCount>(&mut self, value: T) -> i32 {
    value.maximumBlockCount(self);
    return 1;
  }
}

pub trait QPlainTextEdit_maximumBlockCount {
  fn maximumBlockCount(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: int QPlainTextEdit::maximumBlockCount();
impl<'a> /*trait*/ QPlainTextEdit_maximumBlockCount for () {
  fn maximumBlockCount(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17maximumBlockCountEv()};
    unsafe {_ZNK14QPlainTextEdit17maximumBlockCountEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn insertPlainText<T: QPlainTextEdit_insertPlainText>(&mut self, value: T) -> i32 {
    value.insertPlainText(self);
    return 1;
  }
}

pub trait QPlainTextEdit_insertPlainText {
  fn insertPlainText(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_insertPlainText for (&'a  QString) {
  fn insertPlainText(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit15insertPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setTextCursor<T: QPlainTextEdit_setTextCursor>(&mut self, value: T) -> i32 {
    value.setTextCursor(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setTextCursor {
  fn setTextCursor(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_setTextCursor for (&'a  QTextCursor) {
  fn setTextCursor(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn paste<T: QPlainTextEdit_paste>(&mut self, value: T) -> i32 {
    value.paste(self);
    return 1;
  }
}

pub trait QPlainTextEdit_paste {
  fn paste(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::paste();
impl<'a> /*trait*/ QPlainTextEdit_paste for () {
  fn paste(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5pasteEv()};
    unsafe {_ZN14QPlainTextEdit5pasteEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn zoomIn<T: QPlainTextEdit_zoomIn>(&mut self, value: T) -> i32 {
    value.zoomIn(self);
    return 1;
  }
}

pub trait QPlainTextEdit_zoomIn {
  fn zoomIn(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomIn for (i32) {
  fn zoomIn(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QPlainTextEdit6zoomInEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setMaximumBlockCount<T: QPlainTextEdit_setMaximumBlockCount>(&mut self, value: T) -> i32 {
    value.setMaximumBlockCount(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setMaximumBlockCount {
  fn setMaximumBlockCount(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QPlainTextEdit_setMaximumBlockCount for (i32) {
  fn setMaximumBlockCount(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QPlainTextEdit20setMaximumBlockCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn currentCharFormat<T: QPlainTextEdit_currentCharFormat>(&mut self, value: T) -> i32 {
    value.currentCharFormat(self);
    return 1;
  }
}

pub trait QPlainTextEdit_currentCharFormat {
  fn currentCharFormat(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QTextCharFormat QPlainTextEdit::currentCharFormat();
impl<'a> /*trait*/ QPlainTextEdit_currentCharFormat for () {
  fn currentCharFormat(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17currentCharFormatEv()};
    unsafe {_ZNK14QPlainTextEdit17currentCharFormatEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn selectionChanged<T: QPlainTextEdit_selectionChanged>(&mut self, value: T) -> i32 {
    value.selectionChanged(self);
    return 1;
  }
}

pub trait QPlainTextEdit_selectionChanged {
  fn selectionChanged(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::selectionChanged();
impl<'a> /*trait*/ QPlainTextEdit_selectionChanged for () {
  fn selectionChanged(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16selectionChangedEv()};
    unsafe {_ZN14QPlainTextEdit16selectionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setCursorWidth<T: QPlainTextEdit_setCursorWidth>(&mut self, value: T) -> i32 {
    value.setCursorWidth(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setCursorWidth {
  fn setCursorWidth(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setCursorWidth for (i32) {
  fn setCursorWidth(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QPlainTextEdit14setCursorWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn documentTitle<T: QPlainTextEdit_documentTitle>(&mut self, value: T) -> i32 {
    value.documentTitle(self);
    return 1;
  }
}

pub trait QPlainTextEdit_documentTitle {
  fn documentTitle(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QString QPlainTextEdit::documentTitle();
impl<'a> /*trait*/ QPlainTextEdit_documentTitle for () {
  fn documentTitle(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13documentTitleEv()};
    unsafe {_ZNK14QPlainTextEdit13documentTitleEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn selectAll<T: QPlainTextEdit_selectAll>(&mut self, value: T) -> i32 {
    value.selectAll(self);
    return 1;
  }
}

pub trait QPlainTextEdit_selectAll {
  fn selectAll(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::selectAll();
impl<'a> /*trait*/ QPlainTextEdit_selectAll for () {
  fn selectAll(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit9selectAllEv()};
    unsafe {_ZN14QPlainTextEdit9selectAllEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorPositionChanged<T: QPlainTextEdit_cursorPositionChanged>(&mut self, value: T) -> i32 {
    value.cursorPositionChanged(self);
    return 1;
  }
}

pub trait QPlainTextEdit_cursorPositionChanged {
  fn cursorPositionChanged(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::cursorPositionChanged();
impl<'a> /*trait*/ QPlainTextEdit_cursorPositionChanged for () {
  fn cursorPositionChanged(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit21cursorPositionChangedEv()};
    unsafe {_ZN14QPlainTextEdit21cursorPositionChangedEv()};
    return 1;
  }
}

// proto: void QPlainTextEdit::NewQPlainTextEdit(const QPlainTextEdit & );
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (&'a  QPlainTextEdit) {
  fn NewQPlainTextEdit(self) -> QPlainTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEditC1ERKS_(qthis, arg0)};
    let rsthis = QPlainTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setPlainText<T: QPlainTextEdit_setPlainText>(&mut self, value: T) -> i32 {
    value.setPlainText(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setPlainText {
  fn setPlainText(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_setPlainText for (&'a  QString) {
  fn setPlainText(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit12setPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setBackgroundVisible<T: QPlainTextEdit_setBackgroundVisible>(&mut self, value: T) -> i32 {
    value.setBackgroundVisible(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setBackgroundVisible {
  fn setBackgroundVisible(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setBackgroundVisible(bool visible);
impl<'a> /*trait*/ QPlainTextEdit_setBackgroundVisible for (i8) {
  fn setBackgroundVisible(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setBackgroundVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit20setBackgroundVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn blockCountChanged<T: QPlainTextEdit_blockCountChanged>(&mut self, value: T) -> i32 {
    value.blockCountChanged(self);
    return 1;
  }
}

pub trait QPlainTextEdit_blockCountChanged {
  fn blockCountChanged(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::blockCountChanged(int newBlockCount);
impl<'a> /*trait*/ QPlainTextEdit_blockCountChanged for (i32) {
  fn blockCountChanged(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17blockCountChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QPlainTextEdit17blockCountChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setUndoRedoEnabled<T: QPlainTextEdit_setUndoRedoEnabled>(&mut self, value: T) -> i32 {
    value.setUndoRedoEnabled(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setUndoRedoEnabled {
  fn setUndoRedoEnabled(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QPlainTextEdit_setUndoRedoEnabled for (i8) {
  fn setUndoRedoEnabled(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit18setUndoRedoEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn overwriteMode<T: QPlainTextEdit_overwriteMode>(&mut self, value: T) -> i32 {
    value.overwriteMode(self);
    return 1;
  }
}

pub trait QPlainTextEdit_overwriteMode {
  fn overwriteMode(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::overwriteMode();
impl<'a> /*trait*/ QPlainTextEdit_overwriteMode for () {
  fn overwriteMode(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13overwriteModeEv()};
    unsafe {_ZNK14QPlainTextEdit13overwriteModeEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn centerCursor<T: QPlainTextEdit_centerCursor>(&mut self, value: T) -> i32 {
    value.centerCursor(self);
    return 1;
  }
}

pub trait QPlainTextEdit_centerCursor {
  fn centerCursor(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::centerCursor();
impl<'a> /*trait*/ QPlainTextEdit_centerCursor for () {
  fn centerCursor(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12centerCursorEv()};
    unsafe {_ZN14QPlainTextEdit12centerCursorEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn metaObject<T: QPlainTextEdit_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPlainTextEdit_metaObject {
  fn metaObject(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: const QMetaObject * QPlainTextEdit::metaObject();
impl<'a> /*trait*/ QPlainTextEdit_metaObject for () {
  fn metaObject(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10metaObjectEv()};
    unsafe {_ZNK14QPlainTextEdit10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn textChanged<T: QPlainTextEdit_textChanged>(&mut self, value: T) -> i32 {
    value.textChanged(self);
    return 1;
  }
}

pub trait QPlainTextEdit_textChanged {
  fn textChanged(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::textChanged();
impl<'a> /*trait*/ QPlainTextEdit_textChanged for () {
  fn textChanged(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11textChangedEv()};
    unsafe {_ZN14QPlainTextEdit11textChangedEv()};
    return 1;
  }
}

// proto: QMenu * QPlainTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu for () {
  fn createStandardContextMenu(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuEv()};
    unsafe {_ZN14QPlainTextEdit25createStandardContextMenuEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setDocumentTitle<T: QPlainTextEdit_setDocumentTitle>(&mut self, value: T) -> i32 {
    value.setDocumentTitle(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setDocumentTitle {
  fn setDocumentTitle(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QPlainTextEdit_setDocumentTitle for (&'a  QString) {
  fn setDocumentTitle(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit16setDocumentTitleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn FreeQPlainTextEdit<T: QPlainTextEdit_FreeQPlainTextEdit>(&mut self, value: T) -> i32 {
    value.FreeQPlainTextEdit(self);
    return 1;
  }
}

pub trait QPlainTextEdit_FreeQPlainTextEdit {
  fn FreeQPlainTextEdit(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::FreeQPlainTextEdit();
impl<'a> /*trait*/ QPlainTextEdit_FreeQPlainTextEdit for () {
  fn FreeQPlainTextEdit(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditD0Ev()};
    unsafe {_ZN14QPlainTextEditD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn clear<T: QPlainTextEdit_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QPlainTextEdit_clear {
  fn clear(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::clear();
impl<'a> /*trait*/ QPlainTextEdit_clear for () {
  fn clear(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5clearEv()};
    unsafe {_ZN14QPlainTextEdit5clearEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn updateRequest<T: QPlainTextEdit_updateRequest>(&mut self, value: T) -> i32 {
    value.updateRequest(self);
    return 1;
  }
}

pub trait QPlainTextEdit_updateRequest {
  fn updateRequest(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
impl<'a> /*trait*/ QPlainTextEdit_updateRequest for (&'a  QRect, i32) {
  fn updateRequest(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13updateRequestERK5QRecti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN14QPlainTextEdit13updateRequestERK5QRecti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn anchorAt<T: QPlainTextEdit_anchorAt>(&mut self, value: T) -> i32 {
    value.anchorAt(self);
    return 1;
  }
}

pub trait QPlainTextEdit_anchorAt {
  fn anchorAt(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QString QPlainTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_anchorAt for (&'a  QPoint) {
  fn anchorAt(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QPlainTextEdit8anchorAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn canPaste<T: QPlainTextEdit_canPaste>(&mut self, value: T) -> i32 {
    value.canPaste(self);
    return 1;
  }
}

pub trait QPlainTextEdit_canPaste {
  fn canPaste(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::canPaste();
impl<'a> /*trait*/ QPlainTextEdit_canPaste for () {
  fn canPaste(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8canPasteEv()};
    unsafe {_ZNK14QPlainTextEdit8canPasteEv()};
    return 1;
  }
}

// proto: void QPlainTextEdit::NewQPlainTextEdit(QWidget * parent);
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (&'a mut QWidget) {
  fn NewQPlainTextEdit(self) -> QPlainTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QPlainTextEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QPlainTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cut<T: QPlainTextEdit_cut>(&mut self, value: T) -> i32 {
    value.cut(self);
    return 1;
  }
}

pub trait QPlainTextEdit_cut {
  fn cut(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::cut();
impl<'a> /*trait*/ QPlainTextEdit_cut for () {
  fn cut(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit3cutEv()};
    unsafe {_ZN14QPlainTextEdit3cutEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn appendHtml<T: QPlainTextEdit_appendHtml>(&mut self, value: T) -> i32 {
    value.appendHtml(self);
    return 1;
  }
}

pub trait QPlainTextEdit_appendHtml {
  fn appendHtml(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::appendHtml(const QString & html);
impl<'a> /*trait*/ QPlainTextEdit_appendHtml for (&'a  QString) {
  fn appendHtml(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit10appendHtmlERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit10appendHtmlERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn isUndoRedoEnabled<T: QPlainTextEdit_isUndoRedoEnabled>(&mut self, value: T) -> i32 {
    value.isUndoRedoEnabled(self);
    return 1;
  }
}

pub trait QPlainTextEdit_isUndoRedoEnabled {
  fn isUndoRedoEnabled(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QPlainTextEdit_isUndoRedoEnabled for () {
  fn isUndoRedoEnabled(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17isUndoRedoEnabledEv()};
    unsafe {_ZNK14QPlainTextEdit17isUndoRedoEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn zoomOut<T: QPlainTextEdit_zoomOut>(&mut self, value: T) -> i32 {
    value.zoomOut(self);
    return 1;
  }
}

pub trait QPlainTextEdit_zoomOut {
  fn zoomOut(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomOut for (i32) {
  fn zoomOut(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QPlainTextEdit7zoomOutEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setPlaceholderText<T: QPlainTextEdit_setPlaceholderText>(&mut self, value: T) -> i32 {
    value.setPlaceholderText(self);
    return 1;
  }
}

pub trait QPlainTextEdit_setPlaceholderText {
  fn setPlaceholderText(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QPlainTextEdit_setPlaceholderText for (&'a  QString) {
  fn setPlaceholderText(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit18setPlaceholderTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn undo<T: QPlainTextEdit_undo>(&mut self, value: T) -> i32 {
    value.undo(self);
    return 1;
  }
}

pub trait QPlainTextEdit_undo {
  fn undo(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::undo();
impl<'a> /*trait*/ QPlainTextEdit_undo for () {
  fn undo(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4undoEv()};
    unsafe {_ZN14QPlainTextEdit4undoEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn modificationChanged<T: QPlainTextEdit_modificationChanged>(&mut self, value: T) -> i32 {
    value.modificationChanged(self);
    return 1;
  }
}

pub trait QPlainTextEdit_modificationChanged {
  fn modificationChanged(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::modificationChanged(bool );
impl<'a> /*trait*/ QPlainTextEdit_modificationChanged for (i8) {
  fn modificationChanged(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19modificationChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit19modificationChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorForPosition<T: QPlainTextEdit_cursorForPosition>(&mut self, value: T) -> i32 {
    value.cursorForPosition(self);
    return 1;
  }
}

pub trait QPlainTextEdit_cursorForPosition {
  fn cursorForPosition(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_cursorForPosition for (&'a  QPoint) {
  fn cursorForPosition(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn centerOnScroll<T: QPlainTextEdit_centerOnScroll>(&mut self, value: T) -> i32 {
    value.centerOnScroll(self);
    return 1;
  }
}

pub trait QPlainTextEdit_centerOnScroll {
  fn centerOnScroll(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: bool QPlainTextEdit::centerOnScroll();
impl<'a> /*trait*/ QPlainTextEdit_centerOnScroll for () {
  fn centerOnScroll(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit14centerOnScrollEv()};
    unsafe {_ZNK14QPlainTextEdit14centerOnScrollEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn appendPlainText<T: QPlainTextEdit_appendPlainText>(&mut self, value: T) -> i32 {
    value.appendPlainText(self);
    return 1;
  }
}

pub trait QPlainTextEdit_appendPlainText {
  fn appendPlainText(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::appendPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_appendPlainText for (&'a  QString) {
  fn appendPlainText(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15appendPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QPlainTextEdit15appendPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorWidth<T: QPlainTextEdit_cursorWidth>(&mut self, value: T) -> i32 {
    value.cursorWidth(self);
    return 1;
  }
}

pub trait QPlainTextEdit_cursorWidth {
  fn cursorWidth(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: int QPlainTextEdit::cursorWidth();
impl<'a> /*trait*/ QPlainTextEdit_cursorWidth for () {
  fn cursorWidth(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11cursorWidthEv()};
    unsafe {_ZNK14QPlainTextEdit11cursorWidthEv()};
    return 1;
  }
}

// proto: QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_cursorRect for (&'a  QTextCursor) {
  fn cursorRect(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn copyAvailable<T: QPlainTextEdit_copyAvailable>(&mut self, value: T) -> i32 {
    value.copyAvailable(self);
    return 1;
  }
}

pub trait QPlainTextEdit_copyAvailable {
  fn copyAvailable(self, this: &mut QPlainTextEdit) -> i32;
}

// proto: void QPlainTextEdit::copyAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_copyAvailable for (i8) {
  fn copyAvailable(self, this: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13copyAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QPlainTextEdit13copyAvailableEb(arg0)};
    return 1;
  }
}

