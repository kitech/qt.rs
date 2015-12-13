// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qmenu::QMenu;
use super::qtextdocument::QTextDocument;
use super::qrect::QRect;
use super::qstring::QString;
use super::qurl::QUrl;
use super::qvariant::QVariant;
use super::qtextcursor::QTextCursor;
use super::qtextcharformat::QTextCharFormat;
use super::qpagedpaintdevice::QPagedPaintDevice;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
  fn _ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::ensureCursorVisible();
  fn _ZN14QPlainTextEdit19ensureCursorVisibleEv(qthis: *mut c_void) ;
  // proto:  QTextDocument * QPlainTextEdit::document();
  fn _ZNK14QPlainTextEdit8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QPlainTextEdit::cursorRect();
  fn _ZNK14QPlainTextEdit10cursorRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
  fn _ZN14QPlainTextEdit18setTabChangesFocusEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QPlainTextEdit::toPlainText();
  fn _ZNK14QPlainTextEdit11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
  fn _ZN14QPlainTextEdit12loadResourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextEdit::tabStopWidth();
  fn _ZNK14QPlainTextEdit12tabStopWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QPlainTextEdit::isReadOnly();
  fn _ZNK14QPlainTextEdit10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
  fn _ZN14QPlainTextEdit11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QTextCursor QPlainTextEdit::textCursor();
  fn _ZNK14QPlainTextEdit10textCursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
  fn _ZN14QPlainTextEdit17setCenterOnScrollEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QPlainTextEdit::placeholderText();
  fn _ZNK14QPlainTextEdit15placeholderTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextEdit::blockCount();
  fn _ZNK14QPlainTextEdit10blockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn _ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::redoAvailable(bool b);
  fn _ZN14QPlainTextEdit13redoAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
  fn _ZN14QPlainTextEdit11setDocumentEP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
  fn _ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
  fn _ZN14QPlainTextEdit15setTabStopWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QPlainTextEdit::backgroundVisible();
  fn _ZNK14QPlainTextEdit17backgroundVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::redo();
  fn _ZN14QPlainTextEdit4redoEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::NewQPlainTextEdit(const QString & text, QWidget * parent);
  fn _ZN14QPlainTextEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
  fn _ZN14QPlainTextEdit16setOverwriteModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPlainTextEdit::undoAvailable(bool b);
  fn _ZN14QPlainTextEdit13undoAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QPlainTextEdit::tabChangesFocus();
  fn _ZNK14QPlainTextEdit15tabChangesFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::copy();
  fn _ZN14QPlainTextEdit4copyEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn _ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QPlainTextEdit::maximumBlockCount();
  fn _ZNK14QPlainTextEdit17maximumBlockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
  fn _ZN14QPlainTextEdit15insertPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
  fn _ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::paste();
  fn _ZN14QPlainTextEdit5pasteEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::zoomIn(int range);
  fn _ZN14QPlainTextEdit6zoomInEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
  fn _ZN14QPlainTextEdit20setMaximumBlockCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
  fn _ZNK14QPlainTextEdit17currentCharFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::selectionChanged();
  fn _ZN14QPlainTextEdit16selectionChangedEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::setCursorWidth(int width);
  fn _ZN14QPlainTextEdit14setCursorWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QPlainTextEdit::documentTitle();
  fn _ZNK14QPlainTextEdit13documentTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::selectAll();
  fn _ZN14QPlainTextEdit9selectAllEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::cursorPositionChanged();
  fn _ZN14QPlainTextEdit21cursorPositionChangedEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::NewQPlainTextEdit(const QPlainTextEdit & );
  fn _ZN14QPlainTextEditC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
  fn _ZN14QPlainTextEdit12setPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
  fn _ZN14QPlainTextEdit20setBackgroundVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPlainTextEdit::blockCountChanged(int newBlockCount);
  fn _ZN14QPlainTextEdit17blockCountChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
  fn _ZN14QPlainTextEdit18setUndoRedoEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QPlainTextEdit::overwriteMode();
  fn _ZNK14QPlainTextEdit13overwriteModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::centerCursor();
  fn _ZN14QPlainTextEdit12centerCursorEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
  fn _ZNK14QPlainTextEdit10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::textChanged();
  fn _ZN14QPlainTextEdit11textChangedEv(qthis: *mut c_void) ;
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu();
  fn _ZN14QPlainTextEdit25createStandardContextMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
  fn _ZN14QPlainTextEdit16setDocumentTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::FreeQPlainTextEdit();
  fn _ZN14QPlainTextEditD0Ev(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::clear();
  fn _ZN14QPlainTextEdit5clearEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
  fn _ZN14QPlainTextEdit13updateRequestERK5QRecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
  fn _ZNK14QPlainTextEdit8anchorAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPlainTextEdit::canPaste();
  fn _ZNK14QPlainTextEdit8canPasteEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::NewQPlainTextEdit(QWidget * parent);
  fn _ZN14QPlainTextEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::cut();
  fn _ZN14QPlainTextEdit3cutEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
  fn _ZN14QPlainTextEdit10appendHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
  fn _ZNK14QPlainTextEdit17isUndoRedoEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::zoomOut(int range);
  fn _ZN14QPlainTextEdit7zoomOutEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
  fn _ZN14QPlainTextEdit18setPlaceholderTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextEdit::undo();
  fn _ZN14QPlainTextEdit4undoEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextEdit::modificationChanged(bool );
  fn _ZN14QPlainTextEdit19modificationChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
  fn _ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPlainTextEdit::centerOnScroll();
  fn _ZNK14QPlainTextEdit14centerOnScrollEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
  fn _ZN14QPlainTextEdit15appendPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QPlainTextEdit::cursorWidth();
  fn _ZNK14QPlainTextEdit11cursorWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
  fn _ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::copyAvailable(bool b);
  fn _ZN14QPlainTextEdit13copyAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QPlainTextEdit)=1
pub struct QPlainTextEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPlainTextEdit {
  pub fn createStandardContextMenu<T: QPlainTextEdit_createStandardContextMenu>(&mut self, value: T) -> QMenu {
    return value.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_createStandardContextMenu {
  fn createStandardContextMenu(self, rsthis: &mut QPlainTextEdit) -> QMenu;
}

// proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu for (&'a  QPoint) {
  fn createStandardContextMenu(self, rsthis: &mut QPlainTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn ensureCursorVisible<T: QPlainTextEdit_ensureCursorVisible>(&mut self, value: T)  {
     value.ensureCursorVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_ensureCursorVisible {
  fn ensureCursorVisible(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QPlainTextEdit_ensureCursorVisible for () {
  fn ensureCursorVisible(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19ensureCursorVisibleEv()};
     unsafe {_ZN14QPlainTextEdit19ensureCursorVisibleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn document<T: QPlainTextEdit_document>(&mut self, value: T) -> QTextDocument {
    return value.document(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_document {
  fn document(self, rsthis: &mut QPlainTextEdit) -> QTextDocument;
}

// proto:  QTextDocument * QPlainTextEdit::document();
impl<'a> /*trait*/ QPlainTextEdit_document for () {
  fn document(self, rsthis: &mut QPlainTextEdit) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8documentEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorRect<T: QPlainTextEdit_cursorRect>(&mut self, value: T) -> QRect {
    return value.cursorRect(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorRect {
  fn cursorRect(self, rsthis: &mut QPlainTextEdit) -> QRect;
}

// proto:  QRect QPlainTextEdit::cursorRect();
impl<'a> /*trait*/ QPlainTextEdit_cursorRect for () {
  fn cursorRect(self, rsthis: &mut QPlainTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10cursorRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setTabChangesFocus<T: QPlainTextEdit_setTabChangesFocus>(&mut self, value: T)  {
     value.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTabChangesFocus {
  fn setTabChangesFocus(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QPlainTextEdit_setTabChangesFocus for (i8) {
  fn setTabChangesFocus(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn toPlainText<T: QPlainTextEdit_toPlainText>(&mut self, value: T) -> QString {
    return value.toPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_toPlainText {
  fn toPlainText(self, rsthis: &mut QPlainTextEdit) -> QString;
}

// proto:  QString QPlainTextEdit::toPlainText();
impl<'a> /*trait*/ QPlainTextEdit_toPlainText for () {
  fn toPlainText(self, rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11toPlainTextEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn loadResource<T: QPlainTextEdit_loadResource>(&mut self, value: T) -> QVariant {
    return value.loadResource(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_loadResource {
  fn loadResource(self, rsthis: &mut QPlainTextEdit) -> QVariant;
}

// proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QPlainTextEdit_loadResource for (i32, &'a  QUrl) {
  fn loadResource(self, rsthis: &mut QPlainTextEdit) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QPlainTextEdit12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn tabStopWidth<T: QPlainTextEdit_tabStopWidth>(&mut self, value: T) -> i32 {
    return value.tabStopWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_tabStopWidth {
  fn tabStopWidth(self, rsthis: &mut QPlainTextEdit) -> i32;
}

// proto:  int QPlainTextEdit::tabStopWidth();
impl<'a> /*trait*/ QPlainTextEdit_tabStopWidth for () {
  fn tabStopWidth(self, rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit12tabStopWidthEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit12tabStopWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn isReadOnly<T: QPlainTextEdit_isReadOnly>(&mut self, value: T) -> i8 {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_isReadOnly {
  fn isReadOnly(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::isReadOnly();
impl<'a> /*trait*/ QPlainTextEdit_isReadOnly for () {
  fn isReadOnly(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setReadOnly<T: QPlainTextEdit_setReadOnly>(&mut self, value: T)  {
     value.setReadOnly(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setReadOnly {
  fn setReadOnly(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QPlainTextEdit_setReadOnly for (i8) {
  fn setReadOnly(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn textCursor<T: QPlainTextEdit_textCursor>(&mut self, value: T) -> QTextCursor {
    return value.textCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_textCursor {
  fn textCursor(self, rsthis: &mut QPlainTextEdit) -> QTextCursor;
}

// proto:  QTextCursor QPlainTextEdit::textCursor();
impl<'a> /*trait*/ QPlainTextEdit_textCursor for () {
  fn textCursor(self, rsthis: &mut QPlainTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10textCursorEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setCenterOnScroll<T: QPlainTextEdit_setCenterOnScroll>(&mut self, value: T)  {
     value.setCenterOnScroll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCenterOnScroll {
  fn setCenterOnScroll(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
impl<'a> /*trait*/ QPlainTextEdit_setCenterOnScroll for (i8) {
  fn setCenterOnScroll(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17setCenterOnScrollEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit17setCenterOnScrollEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn placeholderText<T: QPlainTextEdit_placeholderText>(&mut self, value: T) -> QString {
    return value.placeholderText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_placeholderText {
  fn placeholderText(self, rsthis: &mut QPlainTextEdit) -> QString;
}

// proto:  QString QPlainTextEdit::placeholderText();
impl<'a> /*trait*/ QPlainTextEdit_placeholderText for () {
  fn placeholderText(self, rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15placeholderTextEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn blockCount<T: QPlainTextEdit_blockCount>(&mut self, value: T) -> i32 {
    return value.blockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_blockCount {
  fn blockCount(self, rsthis: &mut QPlainTextEdit) -> i32;
}

// proto:  int QPlainTextEdit::blockCount();
impl<'a> /*trait*/ QPlainTextEdit_blockCount for () {
  fn blockCount(self, rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10blockCountEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10blockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setCurrentCharFormat<T: QPlainTextEdit_setCurrentCharFormat>(&mut self, value: T)  {
     value.setCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCurrentCharFormat {
  fn setCurrentCharFormat(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QPlainTextEdit_setCurrentCharFormat for (&'a  QTextCharFormat) {
  fn setCurrentCharFormat(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn redoAvailable<T: QPlainTextEdit_redoAvailable>(&mut self, value: T)  {
     value.redoAvailable(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_redoAvailable {
  fn redoAvailable(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::redoAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_redoAvailable for (i8) {
  fn redoAvailable(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13redoAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit13redoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setDocument<T: QPlainTextEdit_setDocument>(&mut self, value: T)  {
     value.setDocument(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setDocument {
  fn setDocument(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QPlainTextEdit_setDocument for (&'a mut QTextDocument) {
  fn setDocument(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn print<T: QPlainTextEdit_print>(&mut self, value: T)  {
     value.print(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_print {
  fn print(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QPlainTextEdit_print for (&'a mut QPagedPaintDevice) {
  fn print(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setTabStopWidth<T: QPlainTextEdit_setTabStopWidth>(&mut self, value: T)  {
     value.setTabStopWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTabStopWidth {
  fn setTabStopWidth(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setTabStopWidth for (i32) {
  fn setTabStopWidth(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit15setTabStopWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn backgroundVisible<T: QPlainTextEdit_backgroundVisible>(&mut self, value: T) -> i8 {
    return value.backgroundVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_backgroundVisible {
  fn backgroundVisible(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::backgroundVisible();
impl<'a> /*trait*/ QPlainTextEdit_backgroundVisible for () {
  fn backgroundVisible(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17backgroundVisibleEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17backgroundVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn redo<T: QPlainTextEdit_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_redo {
  fn redo(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::redo();
impl<'a> /*trait*/ QPlainTextEdit_redo for () {
  fn redo(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4redoEv()};
     unsafe {_ZN14QPlainTextEdit4redoEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QPlainTextEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QPlainTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setOverwriteMode<T: QPlainTextEdit_setOverwriteMode>(&mut self, value: T)  {
     value.setOverwriteMode(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setOverwriteMode {
  fn setOverwriteMode(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QPlainTextEdit_setOverwriteMode for (i8) {
  fn setOverwriteMode(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setOverwriteModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit16setOverwriteModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn undoAvailable<T: QPlainTextEdit_undoAvailable>(&mut self, value: T)  {
     value.undoAvailable(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_undoAvailable {
  fn undoAvailable(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::undoAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_undoAvailable for (i8) {
  fn undoAvailable(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13undoAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit13undoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn tabChangesFocus<T: QPlainTextEdit_tabChangesFocus>(&mut self, value: T) -> i8 {
    return value.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_tabChangesFocus {
  fn tabChangesFocus(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QPlainTextEdit_tabChangesFocus for () {
  fn tabChangesFocus(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15tabChangesFocusEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn copy<T: QPlainTextEdit_copy>(&mut self, value: T)  {
     value.copy(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_copy {
  fn copy(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::copy();
impl<'a> /*trait*/ QPlainTextEdit_copy for () {
  fn copy(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4copyEv()};
     unsafe {_ZN14QPlainTextEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn mergeCurrentCharFormat<T: QPlainTextEdit_mergeCurrentCharFormat>(&mut self, value: T)  {
     value.mergeCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_mergeCurrentCharFormat {
  fn mergeCurrentCharFormat(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QPlainTextEdit_mergeCurrentCharFormat for (&'a  QTextCharFormat) {
  fn mergeCurrentCharFormat(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn maximumBlockCount<T: QPlainTextEdit_maximumBlockCount>(&mut self, value: T) -> i32 {
    return value.maximumBlockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_maximumBlockCount {
  fn maximumBlockCount(self, rsthis: &mut QPlainTextEdit) -> i32;
}

// proto:  int QPlainTextEdit::maximumBlockCount();
impl<'a> /*trait*/ QPlainTextEdit_maximumBlockCount for () {
  fn maximumBlockCount(self, rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17maximumBlockCountEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17maximumBlockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn insertPlainText<T: QPlainTextEdit_insertPlainText>(&mut self, value: T)  {
     value.insertPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_insertPlainText {
  fn insertPlainText(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_insertPlainText for (&'a  QString) {
  fn insertPlainText(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit15insertPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setTextCursor<T: QPlainTextEdit_setTextCursor>(&mut self, value: T)  {
     value.setTextCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTextCursor {
  fn setTextCursor(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_setTextCursor for (&'a  QTextCursor) {
  fn setTextCursor(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn paste<T: QPlainTextEdit_paste>(&mut self, value: T)  {
     value.paste(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_paste {
  fn paste(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::paste();
impl<'a> /*trait*/ QPlainTextEdit_paste for () {
  fn paste(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5pasteEv()};
     unsafe {_ZN14QPlainTextEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn zoomIn<T: QPlainTextEdit_zoomIn>(&mut self, value: T)  {
     value.zoomIn(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_zoomIn {
  fn zoomIn(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomIn for (i32) {
  fn zoomIn(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit6zoomInEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setMaximumBlockCount<T: QPlainTextEdit_setMaximumBlockCount>(&mut self, value: T)  {
     value.setMaximumBlockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setMaximumBlockCount {
  fn setMaximumBlockCount(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QPlainTextEdit_setMaximumBlockCount for (i32) {
  fn setMaximumBlockCount(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit20setMaximumBlockCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn currentCharFormat<T: QPlainTextEdit_currentCharFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.currentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_currentCharFormat {
  fn currentCharFormat(self, rsthis: &mut QPlainTextEdit) -> QTextCharFormat;
}

// proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
impl<'a> /*trait*/ QPlainTextEdit_currentCharFormat for () {
  fn currentCharFormat(self, rsthis: &mut QPlainTextEdit) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17currentCharFormatEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17currentCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn selectionChanged<T: QPlainTextEdit_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::selectionChanged();
impl<'a> /*trait*/ QPlainTextEdit_selectionChanged for () {
  fn selectionChanged(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16selectionChangedEv()};
     unsafe {_ZN14QPlainTextEdit16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setCursorWidth<T: QPlainTextEdit_setCursorWidth>(&mut self, value: T)  {
     value.setCursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCursorWidth {
  fn setCursorWidth(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setCursorWidth for (i32) {
  fn setCursorWidth(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn documentTitle<T: QPlainTextEdit_documentTitle>(&mut self, value: T) -> QString {
    return value.documentTitle(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_documentTitle {
  fn documentTitle(self, rsthis: &mut QPlainTextEdit) -> QString;
}

// proto:  QString QPlainTextEdit::documentTitle();
impl<'a> /*trait*/ QPlainTextEdit_documentTitle for () {
  fn documentTitle(self, rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13documentTitleEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit13documentTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn selectAll<T: QPlainTextEdit_selectAll>(&mut self, value: T)  {
     value.selectAll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_selectAll {
  fn selectAll(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::selectAll();
impl<'a> /*trait*/ QPlainTextEdit_selectAll for () {
  fn selectAll(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit9selectAllEv()};
     unsafe {_ZN14QPlainTextEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorPositionChanged<T: QPlainTextEdit_cursorPositionChanged>(&mut self, value: T)  {
     value.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorPositionChanged {
  fn cursorPositionChanged(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::cursorPositionChanged();
impl<'a> /*trait*/ QPlainTextEdit_cursorPositionChanged for () {
  fn cursorPositionChanged(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit21cursorPositionChangedEv()};
     unsafe {_ZN14QPlainTextEdit21cursorPositionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QPlainTextEdit::NewQPlainTextEdit(const QPlainTextEdit & );
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (&'a  QPlainTextEdit) {
  fn NewQPlainTextEdit(self) -> QPlainTextEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QPlainTextEditC1ERKS_(qthis, arg0)};
    let rsthis = QPlainTextEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setPlainText<T: QPlainTextEdit_setPlainText>(&mut self, value: T)  {
     value.setPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setPlainText {
  fn setPlainText(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_setPlainText for (&'a  QString) {
  fn setPlainText(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setBackgroundVisible<T: QPlainTextEdit_setBackgroundVisible>(&mut self, value: T)  {
     value.setBackgroundVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setBackgroundVisible {
  fn setBackgroundVisible(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
impl<'a> /*trait*/ QPlainTextEdit_setBackgroundVisible for (i8) {
  fn setBackgroundVisible(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setBackgroundVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit20setBackgroundVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn blockCountChanged<T: QPlainTextEdit_blockCountChanged>(&mut self, value: T)  {
     value.blockCountChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_blockCountChanged {
  fn blockCountChanged(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::blockCountChanged(int newBlockCount);
impl<'a> /*trait*/ QPlainTextEdit_blockCountChanged for (i32) {
  fn blockCountChanged(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17blockCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit17blockCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setUndoRedoEnabled<T: QPlainTextEdit_setUndoRedoEnabled>(&mut self, value: T)  {
     value.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setUndoRedoEnabled {
  fn setUndoRedoEnabled(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QPlainTextEdit_setUndoRedoEnabled for (i8) {
  fn setUndoRedoEnabled(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn overwriteMode<T: QPlainTextEdit_overwriteMode>(&mut self, value: T) -> i8 {
    return value.overwriteMode(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_overwriteMode {
  fn overwriteMode(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::overwriteMode();
impl<'a> /*trait*/ QPlainTextEdit_overwriteMode for () {
  fn overwriteMode(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13overwriteModeEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit13overwriteModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn centerCursor<T: QPlainTextEdit_centerCursor>(&mut self, value: T)  {
     value.centerCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_centerCursor {
  fn centerCursor(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::centerCursor();
impl<'a> /*trait*/ QPlainTextEdit_centerCursor for () {
  fn centerCursor(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12centerCursorEv()};
     unsafe {_ZN14QPlainTextEdit12centerCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn metaObject<T: QPlainTextEdit_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_metaObject {
  fn metaObject(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  const QMetaObject * QPlainTextEdit::metaObject();
impl<'a> /*trait*/ QPlainTextEdit_metaObject for () {
  fn metaObject(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10metaObjectEv()};
     unsafe {_ZNK14QPlainTextEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn textChanged<T: QPlainTextEdit_textChanged>(&mut self, value: T)  {
     value.textChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_textChanged {
  fn textChanged(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::textChanged();
impl<'a> /*trait*/ QPlainTextEdit_textChanged for () {
  fn textChanged(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11textChangedEv()};
     unsafe {_ZN14QPlainTextEdit11textChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QMenu * QPlainTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu for () {
  fn createStandardContextMenu(self, rsthis: &mut QPlainTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {_ZN14QPlainTextEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setDocumentTitle<T: QPlainTextEdit_setDocumentTitle>(&mut self, value: T)  {
     value.setDocumentTitle(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setDocumentTitle {
  fn setDocumentTitle(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QPlainTextEdit_setDocumentTitle for (&'a  QString) {
  fn setDocumentTitle(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit16setDocumentTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn FreeQPlainTextEdit<T: QPlainTextEdit_FreeQPlainTextEdit>(&mut self, value: T)  {
     value.FreeQPlainTextEdit(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_FreeQPlainTextEdit {
  fn FreeQPlainTextEdit(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::FreeQPlainTextEdit();
impl<'a> /*trait*/ QPlainTextEdit_FreeQPlainTextEdit for () {
  fn FreeQPlainTextEdit(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditD0Ev()};
     unsafe {_ZN14QPlainTextEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn clear<T: QPlainTextEdit_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_clear {
  fn clear(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::clear();
impl<'a> /*trait*/ QPlainTextEdit_clear for () {
  fn clear(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5clearEv()};
     unsafe {_ZN14QPlainTextEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn updateRequest<T: QPlainTextEdit_updateRequest>(&mut self, value: T)  {
     value.updateRequest(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_updateRequest {
  fn updateRequest(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
impl<'a> /*trait*/ QPlainTextEdit_updateRequest for (&'a  QRect, i32) {
  fn updateRequest(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13updateRequestERK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN14QPlainTextEdit13updateRequestERK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn anchorAt<T: QPlainTextEdit_anchorAt>(&mut self, value: T) -> QString {
    return value.anchorAt(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_anchorAt {
  fn anchorAt(self, rsthis: &mut QPlainTextEdit) -> QString;
}

// proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_anchorAt for (&'a  QPoint) {
  fn anchorAt(self, rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QPlainTextEdit8anchorAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn canPaste<T: QPlainTextEdit_canPaste>(&mut self, value: T) -> i8 {
    return value.canPaste(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_canPaste {
  fn canPaste(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::canPaste();
impl<'a> /*trait*/ QPlainTextEdit_canPaste for () {
  fn canPaste(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8canPasteEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit8canPasteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn cut<T: QPlainTextEdit_cut>(&mut self, value: T)  {
     value.cut(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cut {
  fn cut(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::cut();
impl<'a> /*trait*/ QPlainTextEdit_cut for () {
  fn cut(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit3cutEv()};
     unsafe {_ZN14QPlainTextEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn appendHtml<T: QPlainTextEdit_appendHtml>(&mut self, value: T)  {
     value.appendHtml(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_appendHtml {
  fn appendHtml(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::appendHtml(const QString & html);
impl<'a> /*trait*/ QPlainTextEdit_appendHtml for (&'a  QString) {
  fn appendHtml(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit10appendHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit10appendHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn isUndoRedoEnabled<T: QPlainTextEdit_isUndoRedoEnabled>(&mut self, value: T) -> i8 {
    return value.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_isUndoRedoEnabled {
  fn isUndoRedoEnabled(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QPlainTextEdit_isUndoRedoEnabled for () {
  fn isUndoRedoEnabled(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17isUndoRedoEnabledEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn zoomOut<T: QPlainTextEdit_zoomOut>(&mut self, value: T)  {
     value.zoomOut(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_zoomOut {
  fn zoomOut(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomOut for (i32) {
  fn zoomOut(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit7zoomOutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn setPlaceholderText<T: QPlainTextEdit_setPlaceholderText>(&mut self, value: T)  {
     value.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setPlaceholderText {
  fn setPlaceholderText(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QPlainTextEdit_setPlaceholderText for (&'a  QString) {
  fn setPlaceholderText(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn undo<T: QPlainTextEdit_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_undo {
  fn undo(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::undo();
impl<'a> /*trait*/ QPlainTextEdit_undo for () {
  fn undo(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4undoEv()};
     unsafe {_ZN14QPlainTextEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn modificationChanged<T: QPlainTextEdit_modificationChanged>(&mut self, value: T)  {
     value.modificationChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_modificationChanged {
  fn modificationChanged(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::modificationChanged(bool );
impl<'a> /*trait*/ QPlainTextEdit_modificationChanged for (i8) {
  fn modificationChanged(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19modificationChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit19modificationChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorForPosition<T: QPlainTextEdit_cursorForPosition>(&mut self, value: T) -> QTextCursor {
    return value.cursorForPosition(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorForPosition {
  fn cursorForPosition(self, rsthis: &mut QPlainTextEdit) -> QTextCursor;
}

// proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_cursorForPosition for (&'a  QPoint) {
  fn cursorForPosition(self, rsthis: &mut QPlainTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn centerOnScroll<T: QPlainTextEdit_centerOnScroll>(&mut self, value: T) -> i8 {
    return value.centerOnScroll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_centerOnScroll {
  fn centerOnScroll(self, rsthis: &mut QPlainTextEdit) -> i8;
}

// proto:  bool QPlainTextEdit::centerOnScroll();
impl<'a> /*trait*/ QPlainTextEdit_centerOnScroll for () {
  fn centerOnScroll(self, rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit14centerOnScrollEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit14centerOnScrollEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn appendPlainText<T: QPlainTextEdit_appendPlainText>(&mut self, value: T)  {
     value.appendPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_appendPlainText {
  fn appendPlainText(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::appendPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_appendPlainText for (&'a  QString) {
  fn appendPlainText(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15appendPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit15appendPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn cursorWidth<T: QPlainTextEdit_cursorWidth>(&mut self, value: T) -> i32 {
    return value.cursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorWidth {
  fn cursorWidth(self, rsthis: &mut QPlainTextEdit) -> i32;
}

// proto:  int QPlainTextEdit::cursorWidth();
impl<'a> /*trait*/ QPlainTextEdit_cursorWidth for () {
  fn cursorWidth(self, rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11cursorWidthEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_cursorRect for (&'a  QTextCursor) {
  fn cursorRect(self, rsthis: &mut QPlainTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn copyAvailable<T: QPlainTextEdit_copyAvailable>(&mut self, value: T)  {
     value.copyAvailable(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_copyAvailable {
  fn copyAvailable(self, rsthis: &mut QPlainTextEdit) ;
}

// proto:  void QPlainTextEdit::copyAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_copyAvailable for (i8) {
  fn copyAvailable(self, rsthis: &mut QPlainTextEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13copyAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QPlainTextEdit13copyAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

