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
use super::qregexp::QRegExp;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
  fn _ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::ensureCursorVisible();
  fn _ZN14QPlainTextEdit19ensureCursorVisibleEv(qthis: *mut c_void);
  // proto:  QTextDocument * QPlainTextEdit::document();
  fn _ZNK14QPlainTextEdit8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QPlainTextEdit::cursorRect();
  fn _ZNK14QPlainTextEdit10cursorRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
  fn _ZN14QPlainTextEdit18setTabChangesFocusEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QPlainTextEdit::toPlainText();
  fn _ZNK14QPlainTextEdit11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
  fn _ZN14QPlainTextEdit12loadResourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextEdit::tabStopWidth();
  fn _ZNK14QPlainTextEdit12tabStopWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QPlainTextEdit::isReadOnly();
  fn _ZNK14QPlainTextEdit10isReadOnlyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
  fn _ZN14QPlainTextEdit11setReadOnlyEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QTextCursor QPlainTextEdit::textCursor();
  fn _ZNK14QPlainTextEdit10textCursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
  fn _ZN14QPlainTextEdit17setCenterOnScrollEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QPlainTextEdit::placeholderText();
  fn _ZNK14QPlainTextEdit15placeholderTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextEdit::blockCount();
  fn _ZNK14QPlainTextEdit10blockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn _ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::redoAvailable(bool b);
  fn _ZN14QPlainTextEdit13redoAvailableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
  fn _ZN14QPlainTextEdit11setDocumentEP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
  fn _ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
  fn _ZN14QPlainTextEdit15setTabStopWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QPlainTextEdit::backgroundVisible();
  fn _ZNK14QPlainTextEdit17backgroundVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::redo();
  fn _ZN14QPlainTextEdit4redoEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::QPlainTextEdit(const QString & text, QWidget * parent);
  fn _ZN14QPlainTextEditC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
  fn _ZN14QPlainTextEdit16setOverwriteModeEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPlainTextEdit::undoAvailable(bool b);
  fn _ZN14QPlainTextEdit13undoAvailableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QPlainTextEdit::tabChangesFocus();
  fn _ZNK14QPlainTextEdit15tabChangesFocusEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::copy();
  fn _ZN14QPlainTextEdit4copyEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn _ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QPlainTextEdit::maximumBlockCount();
  fn _ZNK14QPlainTextEdit17maximumBlockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
  fn _ZN14QPlainTextEdit15insertPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
  fn _ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::paste();
  fn _ZN14QPlainTextEdit5pasteEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::zoomIn(int range);
  fn _ZN14QPlainTextEdit6zoomInEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
  fn _ZN14QPlainTextEdit20setMaximumBlockCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
  fn _ZNK14QPlainTextEdit17currentCharFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::selectionChanged();
  fn _ZN14QPlainTextEdit16selectionChangedEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::setCursorWidth(int width);
  fn _ZN14QPlainTextEdit14setCursorWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QPlainTextEdit::documentTitle();
  fn _ZNK14QPlainTextEdit13documentTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::selectAll();
  fn _ZN14QPlainTextEdit9selectAllEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::cursorPositionChanged();
  fn _ZN14QPlainTextEdit21cursorPositionChangedEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::QPlainTextEdit(const QPlainTextEdit & );
  fn _ZN14QPlainTextEditC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
  fn _ZN14QPlainTextEdit12setPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
  fn _ZN14QPlainTextEdit20setBackgroundVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPlainTextEdit::blockCountChanged(int newBlockCount);
  fn _ZN14QPlainTextEdit17blockCountChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
  fn _ZN14QPlainTextEdit18setUndoRedoEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QPlainTextEdit::overwriteMode();
  fn _ZNK14QPlainTextEdit13overwriteModeEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::centerCursor();
  fn _ZN14QPlainTextEdit12centerCursorEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
  fn _ZNK14QPlainTextEdit10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::textChanged();
  fn _ZN14QPlainTextEdit11textChangedEv(qthis: *mut c_void);
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu();
  fn _ZN14QPlainTextEdit25createStandardContextMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
  fn _ZN14QPlainTextEdit16setDocumentTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::~QPlainTextEdit();
  fn _ZN14QPlainTextEditD0Ev(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::clear();
  fn _ZN14QPlainTextEdit5clearEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
  fn _ZN14QPlainTextEdit13updateRequestERK5QRecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
  fn _ZNK14QPlainTextEdit8anchorAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPlainTextEdit::canPaste();
  fn _ZNK14QPlainTextEdit8canPasteEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::QPlainTextEdit(QWidget * parent);
  fn _ZN14QPlainTextEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::cut();
  fn _ZN14QPlainTextEdit3cutEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
  fn _ZN14QPlainTextEdit10appendHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
  fn _ZNK14QPlainTextEdit17isUndoRedoEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::zoomOut(int range);
  fn _ZN14QPlainTextEdit7zoomOutEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
  fn _ZN14QPlainTextEdit18setPlaceholderTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::undo();
  fn _ZN14QPlainTextEdit4undoEv(qthis: *mut c_void);
  // proto:  void QPlainTextEdit::modificationChanged(bool );
  fn _ZN14QPlainTextEdit19modificationChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
  fn _ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPlainTextEdit::centerOnScroll();
  fn _ZNK14QPlainTextEdit14centerOnScrollEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
  fn _ZN14QPlainTextEdit15appendPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QPlainTextEdit::cursorWidth();
  fn _ZNK14QPlainTextEdit11cursorWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
  fn _ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::copyAvailable(bool b);
  fn _ZN14QPlainTextEdit13copyAvailableEb(qthis: *mut c_void, arg0: c_char);
}

// body block begin
// class sizeof(QPlainTextEdit)=1
pub struct QPlainTextEdit {
  pub qclsinst: *mut c_void,
}

  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
impl /*struct*/ QPlainTextEdit {
  pub fn createStandardContextMenu<RetType, T: QPlainTextEdit_createStandardContextMenu<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_createStandardContextMenu<RetType> {
  fn createStandardContextMenu(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu<QMenu> for (QPoint) {
  fn createStandardContextMenu(self , rsthis: &mut QPlainTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::ensureCursorVisible();
impl /*struct*/ QPlainTextEdit {
  pub fn ensureCursorVisible<RetType, T: QPlainTextEdit_ensureCursorVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ensureCursorVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_ensureCursorVisible<RetType> {
  fn ensureCursorVisible(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QPlainTextEdit_ensureCursorVisible<()> for () {
  fn ensureCursorVisible(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19ensureCursorVisibleEv()};
     unsafe {_ZN14QPlainTextEdit19ensureCursorVisibleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextDocument * QPlainTextEdit::document();
impl /*struct*/ QPlainTextEdit {
  pub fn document<RetType, T: QPlainTextEdit_document<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_document<RetType> {
  fn document(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QTextDocument * QPlainTextEdit::document();
impl<'a> /*trait*/ QPlainTextEdit_document<QTextDocument> for () {
  fn document(self , rsthis: &mut QPlainTextEdit) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8documentEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QPlainTextEdit::cursorRect();
impl /*struct*/ QPlainTextEdit {
  pub fn cursorRect<RetType, T: QPlainTextEdit_cursorRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorRect(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorRect<RetType> {
  fn cursorRect(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QRect QPlainTextEdit::cursorRect();
impl<'a> /*trait*/ QPlainTextEdit_cursorRect<QRect> for () {
  fn cursorRect(self , rsthis: &mut QPlainTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10cursorRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
impl /*struct*/ QPlainTextEdit {
  pub fn setTabChangesFocus<RetType, T: QPlainTextEdit_setTabChangesFocus<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTabChangesFocus<RetType> {
  fn setTabChangesFocus(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QPlainTextEdit_setTabChangesFocus<()> for (i8) {
  fn setTabChangesFocus(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::toPlainText();
impl /*struct*/ QPlainTextEdit {
  pub fn toPlainText<RetType, T: QPlainTextEdit_toPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_toPlainText<RetType> {
  fn toPlainText(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::toPlainText();
impl<'a> /*trait*/ QPlainTextEdit_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11toPlainTextEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
impl /*struct*/ QPlainTextEdit {
  pub fn loadResource<RetType, T: QPlainTextEdit_loadResource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.loadResource(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_loadResource<RetType> {
  fn loadResource(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QPlainTextEdit_loadResource<QVariant> for (i32, QUrl) {
  fn loadResource(self , rsthis: &mut QPlainTextEdit) -> QVariant {
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

  // proto:  int QPlainTextEdit::tabStopWidth();
impl /*struct*/ QPlainTextEdit {
  pub fn tabStopWidth<RetType, T: QPlainTextEdit_tabStopWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabStopWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_tabStopWidth<RetType> {
  fn tabStopWidth(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::tabStopWidth();
impl<'a> /*trait*/ QPlainTextEdit_tabStopWidth<i32> for () {
  fn tabStopWidth(self , rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit12tabStopWidthEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit12tabStopWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::isReadOnly();
impl /*struct*/ QPlainTextEdit {
  pub fn isReadOnly<RetType, T: QPlainTextEdit_isReadOnly<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::isReadOnly();
impl<'a> /*trait*/ QPlainTextEdit_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
impl /*struct*/ QPlainTextEdit {
  pub fn setReadOnly<RetType, T: QPlainTextEdit_setReadOnly<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QPlainTextEdit_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCursor QPlainTextEdit::textCursor();
impl /*struct*/ QPlainTextEdit {
  pub fn textCursor<RetType, T: QPlainTextEdit_textCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_textCursor<RetType> {
  fn textCursor(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QTextCursor QPlainTextEdit::textCursor();
impl<'a> /*trait*/ QPlainTextEdit_textCursor<QTextCursor> for () {
  fn textCursor(self , rsthis: &mut QPlainTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10textCursorEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
impl /*struct*/ QPlainTextEdit {
  pub fn setCenterOnScroll<RetType, T: QPlainTextEdit_setCenterOnScroll<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenterOnScroll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCenterOnScroll<RetType> {
  fn setCenterOnScroll(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
impl<'a> /*trait*/ QPlainTextEdit_setCenterOnScroll<()> for (i8) {
  fn setCenterOnScroll(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17setCenterOnScrollEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit17setCenterOnScrollEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::placeholderText();
impl /*struct*/ QPlainTextEdit {
  pub fn placeholderText<RetType, T: QPlainTextEdit_placeholderText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.placeholderText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_placeholderText<RetType> {
  fn placeholderText(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::placeholderText();
impl<'a> /*trait*/ QPlainTextEdit_placeholderText<QString> for () {
  fn placeholderText(self , rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15placeholderTextEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::blockCount();
impl /*struct*/ QPlainTextEdit {
  pub fn blockCount<RetType, T: QPlainTextEdit_blockCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_blockCount<RetType> {
  fn blockCount(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::blockCount();
impl<'a> /*trait*/ QPlainTextEdit_blockCount<i32> for () {
  fn blockCount(self , rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10blockCountEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit10blockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl /*struct*/ QPlainTextEdit {
  pub fn setCurrentCharFormat<RetType, T: QPlainTextEdit_setCurrentCharFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCurrentCharFormat<RetType> {
  fn setCurrentCharFormat(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QPlainTextEdit_setCurrentCharFormat<()> for (QTextCharFormat) {
  fn setCurrentCharFormat(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::redoAvailable(bool b);
impl /*struct*/ QPlainTextEdit {
  pub fn redoAvailable<RetType, T: QPlainTextEdit_redoAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.redoAvailable(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_redoAvailable<RetType> {
  fn redoAvailable(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::redoAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_redoAvailable<()> for (i8) {
  fn redoAvailable(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13redoAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit13redoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
impl /*struct*/ QPlainTextEdit {
  pub fn setDocument<RetType, T: QPlainTextEdit_setDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDocument(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setDocument<RetType> {
  fn setDocument(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QPlainTextEdit_setDocument<()> for (QTextDocument) {
  fn setDocument(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
impl /*struct*/ QPlainTextEdit {
  pub fn print<RetType, T: QPlainTextEdit_print<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.print(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_print<RetType> {
  fn print(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QPlainTextEdit_print<()> for (QPagedPaintDevice) {
  fn print(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
impl /*struct*/ QPlainTextEdit {
  pub fn setTabStopWidth<RetType, T: QPlainTextEdit_setTabStopWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabStopWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTabStopWidth<RetType> {
  fn setTabStopWidth(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setTabStopWidth<()> for (i32) {
  fn setTabStopWidth(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit15setTabStopWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::backgroundVisible();
impl /*struct*/ QPlainTextEdit {
  pub fn backgroundVisible<RetType, T: QPlainTextEdit_backgroundVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.backgroundVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_backgroundVisible<RetType> {
  fn backgroundVisible(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::backgroundVisible();
impl<'a> /*trait*/ QPlainTextEdit_backgroundVisible<i8> for () {
  fn backgroundVisible(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17backgroundVisibleEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17backgroundVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::redo();
impl /*struct*/ QPlainTextEdit {
  pub fn redo<RetType, T: QPlainTextEdit_redo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_redo<RetType> {
  fn redo(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::redo();
impl<'a> /*trait*/ QPlainTextEdit_redo<()> for () {
  fn redo(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4redoEv()};
     unsafe {_ZN14QPlainTextEdit4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::QPlainTextEdit(const QString & text, QWidget * parent);
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

  // proto:  void QPlainTextEdit::QPlainTextEdit(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (QString, QWidget) {
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

  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
impl /*struct*/ QPlainTextEdit {
  pub fn setOverwriteMode<RetType, T: QPlainTextEdit_setOverwriteMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOverwriteMode(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setOverwriteMode<RetType> {
  fn setOverwriteMode(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QPlainTextEdit_setOverwriteMode<()> for (i8) {
  fn setOverwriteMode(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setOverwriteModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit16setOverwriteModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::undoAvailable(bool b);
impl /*struct*/ QPlainTextEdit {
  pub fn undoAvailable<RetType, T: QPlainTextEdit_undoAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.undoAvailable(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_undoAvailable<RetType> {
  fn undoAvailable(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::undoAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_undoAvailable<()> for (i8) {
  fn undoAvailable(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13undoAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit13undoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::tabChangesFocus();
impl /*struct*/ QPlainTextEdit {
  pub fn tabChangesFocus<RetType, T: QPlainTextEdit_tabChangesFocus<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_tabChangesFocus<RetType> {
  fn tabChangesFocus(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QPlainTextEdit_tabChangesFocus<i8> for () {
  fn tabChangesFocus(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15tabChangesFocusEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::copy();
impl /*struct*/ QPlainTextEdit {
  pub fn copy<RetType, T: QPlainTextEdit_copy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_copy<RetType> {
  fn copy(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::copy();
impl<'a> /*trait*/ QPlainTextEdit_copy<()> for () {
  fn copy(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4copyEv()};
     unsafe {_ZN14QPlainTextEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl /*struct*/ QPlainTextEdit {
  pub fn mergeCurrentCharFormat<RetType, T: QPlainTextEdit_mergeCurrentCharFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mergeCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_mergeCurrentCharFormat<RetType> {
  fn mergeCurrentCharFormat(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QPlainTextEdit_mergeCurrentCharFormat<()> for (QTextCharFormat) {
  fn mergeCurrentCharFormat(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::maximumBlockCount();
impl /*struct*/ QPlainTextEdit {
  pub fn maximumBlockCount<RetType, T: QPlainTextEdit_maximumBlockCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumBlockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_maximumBlockCount<RetType> {
  fn maximumBlockCount(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::maximumBlockCount();
impl<'a> /*trait*/ QPlainTextEdit_maximumBlockCount<i32> for () {
  fn maximumBlockCount(self , rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17maximumBlockCountEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17maximumBlockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
impl /*struct*/ QPlainTextEdit {
  pub fn insertPlainText<RetType, T: QPlainTextEdit_insertPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_insertPlainText<RetType> {
  fn insertPlainText(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_insertPlainText<()> for (QString) {
  fn insertPlainText(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit15insertPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
impl /*struct*/ QPlainTextEdit {
  pub fn setTextCursor<RetType, T: QPlainTextEdit_setTextCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTextCursor<RetType> {
  fn setTextCursor(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_setTextCursor<()> for (QTextCursor) {
  fn setTextCursor(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::paste();
impl /*struct*/ QPlainTextEdit {
  pub fn paste<RetType, T: QPlainTextEdit_paste<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paste(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_paste<RetType> {
  fn paste(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::paste();
impl<'a> /*trait*/ QPlainTextEdit_paste<()> for () {
  fn paste(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5pasteEv()};
     unsafe {_ZN14QPlainTextEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::zoomIn(int range);
impl /*struct*/ QPlainTextEdit {
  pub fn zoomIn<RetType, T: QPlainTextEdit_zoomIn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.zoomIn(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_zoomIn<RetType> {
  fn zoomIn(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomIn<()> for (i32) {
  fn zoomIn(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit6zoomInEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
impl /*struct*/ QPlainTextEdit {
  pub fn setMaximumBlockCount<RetType, T: QPlainTextEdit_setMaximumBlockCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumBlockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setMaximumBlockCount<RetType> {
  fn setMaximumBlockCount(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QPlainTextEdit_setMaximumBlockCount<()> for (i32) {
  fn setMaximumBlockCount(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit20setMaximumBlockCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
impl /*struct*/ QPlainTextEdit {
  pub fn currentCharFormat<RetType, T: QPlainTextEdit_currentCharFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_currentCharFormat<RetType> {
  fn currentCharFormat(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
impl<'a> /*trait*/ QPlainTextEdit_currentCharFormat<QTextCharFormat> for () {
  fn currentCharFormat(self , rsthis: &mut QPlainTextEdit) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17currentCharFormatEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17currentCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::selectionChanged();
impl /*struct*/ QPlainTextEdit {
  pub fn selectionChanged<RetType, T: QPlainTextEdit_selectionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::selectionChanged();
impl<'a> /*trait*/ QPlainTextEdit_selectionChanged<()> for () {
  fn selectionChanged(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16selectionChangedEv()};
     unsafe {_ZN14QPlainTextEdit16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setCursorWidth(int width);
impl /*struct*/ QPlainTextEdit {
  pub fn setCursorWidth<RetType, T: QPlainTextEdit_setCursorWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCursorWidth<RetType> {
  fn setCursorWidth(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setCursorWidth<()> for (i32) {
  fn setCursorWidth(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::documentTitle();
impl /*struct*/ QPlainTextEdit {
  pub fn documentTitle<RetType, T: QPlainTextEdit_documentTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentTitle(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_documentTitle<RetType> {
  fn documentTitle(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::documentTitle();
impl<'a> /*trait*/ QPlainTextEdit_documentTitle<QString> for () {
  fn documentTitle(self , rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13documentTitleEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit13documentTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::selectAll();
impl /*struct*/ QPlainTextEdit {
  pub fn selectAll<RetType, T: QPlainTextEdit_selectAll<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_selectAll<RetType> {
  fn selectAll(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::selectAll();
impl<'a> /*trait*/ QPlainTextEdit_selectAll<()> for () {
  fn selectAll(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit9selectAllEv()};
     unsafe {_ZN14QPlainTextEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::cursorPositionChanged();
impl /*struct*/ QPlainTextEdit {
  pub fn cursorPositionChanged<RetType, T: QPlainTextEdit_cursorPositionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorPositionChanged<RetType> {
  fn cursorPositionChanged(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::cursorPositionChanged();
impl<'a> /*trait*/ QPlainTextEdit_cursorPositionChanged<()> for () {
  fn cursorPositionChanged(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit21cursorPositionChangedEv()};
     unsafe {_ZN14QPlainTextEdit21cursorPositionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::QPlainTextEdit(const QPlainTextEdit & );
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (QPlainTextEdit) {
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

  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
impl /*struct*/ QPlainTextEdit {
  pub fn setPlainText<RetType, T: QPlainTextEdit_setPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setPlainText<RetType> {
  fn setPlainText(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_setPlainText<()> for (QString) {
  fn setPlainText(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
impl /*struct*/ QPlainTextEdit {
  pub fn setBackgroundVisible<RetType, T: QPlainTextEdit_setBackgroundVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setBackgroundVisible<RetType> {
  fn setBackgroundVisible(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
impl<'a> /*trait*/ QPlainTextEdit_setBackgroundVisible<()> for (i8) {
  fn setBackgroundVisible(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setBackgroundVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit20setBackgroundVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::blockCountChanged(int newBlockCount);
impl /*struct*/ QPlainTextEdit {
  pub fn blockCountChanged<RetType, T: QPlainTextEdit_blockCountChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockCountChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_blockCountChanged<RetType> {
  fn blockCountChanged(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::blockCountChanged(int newBlockCount);
impl<'a> /*trait*/ QPlainTextEdit_blockCountChanged<()> for (i32) {
  fn blockCountChanged(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17blockCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit17blockCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
impl /*struct*/ QPlainTextEdit {
  pub fn setUndoRedoEnabled<RetType, T: QPlainTextEdit_setUndoRedoEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setUndoRedoEnabled<RetType> {
  fn setUndoRedoEnabled(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QPlainTextEdit_setUndoRedoEnabled<()> for (i8) {
  fn setUndoRedoEnabled(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::overwriteMode();
impl /*struct*/ QPlainTextEdit {
  pub fn overwriteMode<RetType, T: QPlainTextEdit_overwriteMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.overwriteMode(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_overwriteMode<RetType> {
  fn overwriteMode(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::overwriteMode();
impl<'a> /*trait*/ QPlainTextEdit_overwriteMode<i8> for () {
  fn overwriteMode(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13overwriteModeEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit13overwriteModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::centerCursor();
impl /*struct*/ QPlainTextEdit {
  pub fn centerCursor<RetType, T: QPlainTextEdit_centerCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.centerCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_centerCursor<RetType> {
  fn centerCursor(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::centerCursor();
impl<'a> /*trait*/ QPlainTextEdit_centerCursor<()> for () {
  fn centerCursor(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12centerCursorEv()};
     unsafe {_ZN14QPlainTextEdit12centerCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
impl /*struct*/ QPlainTextEdit {
  pub fn metaObject<RetType, T: QPlainTextEdit_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
impl<'a> /*trait*/ QPlainTextEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10metaObjectEv()};
     unsafe {_ZNK14QPlainTextEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::textChanged();
impl /*struct*/ QPlainTextEdit {
  pub fn textChanged<RetType, T: QPlainTextEdit_textChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_textChanged<RetType> {
  fn textChanged(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::textChanged();
impl<'a> /*trait*/ QPlainTextEdit_textChanged<()> for () {
  fn textChanged(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11textChangedEv()};
     unsafe {_ZN14QPlainTextEdit11textChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu<QMenu> for () {
  fn createStandardContextMenu(self , rsthis: &mut QPlainTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {_ZN14QPlainTextEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
impl /*struct*/ QPlainTextEdit {
  pub fn setDocumentTitle<RetType, T: QPlainTextEdit_setDocumentTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDocumentTitle(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setDocumentTitle<RetType> {
  fn setDocumentTitle(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QPlainTextEdit_setDocumentTitle<()> for (QString) {
  fn setDocumentTitle(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit16setDocumentTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::~QPlainTextEdit();
impl /*struct*/ QPlainTextEdit {
  pub fn FreeQPlainTextEdit<RetType, T: QPlainTextEdit_FreeQPlainTextEdit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPlainTextEdit(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_FreeQPlainTextEdit<RetType> {
  fn FreeQPlainTextEdit(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::~QPlainTextEdit();
impl<'a> /*trait*/ QPlainTextEdit_FreeQPlainTextEdit<()> for () {
  fn FreeQPlainTextEdit(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditD0Ev()};
     unsafe {_ZN14QPlainTextEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::clear();
impl /*struct*/ QPlainTextEdit {
  pub fn clear<RetType, T: QPlainTextEdit_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_clear<RetType> {
  fn clear(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::clear();
impl<'a> /*trait*/ QPlainTextEdit_clear<()> for () {
  fn clear(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5clearEv()};
     unsafe {_ZN14QPlainTextEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
impl /*struct*/ QPlainTextEdit {
  pub fn updateRequest<RetType, T: QPlainTextEdit_updateRequest<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateRequest(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_updateRequest<RetType> {
  fn updateRequest(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::updateRequest(const QRect & rect, int dy);
impl<'a> /*trait*/ QPlainTextEdit_updateRequest<()> for (QRect, i32) {
  fn updateRequest(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13updateRequestERK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN14QPlainTextEdit13updateRequestERK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
impl /*struct*/ QPlainTextEdit {
  pub fn anchorAt<RetType, T: QPlainTextEdit_anchorAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.anchorAt(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_anchorAt<RetType> {
  fn anchorAt(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_anchorAt<QString> for (QPoint) {
  fn anchorAt(self , rsthis: &mut QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QPlainTextEdit8anchorAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::canPaste();
impl /*struct*/ QPlainTextEdit {
  pub fn canPaste<RetType, T: QPlainTextEdit_canPaste<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canPaste(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_canPaste<RetType> {
  fn canPaste(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::canPaste();
impl<'a> /*trait*/ QPlainTextEdit_canPaste<i8> for () {
  fn canPaste(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8canPasteEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit8canPasteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::QPlainTextEdit(QWidget * parent);
impl<'a> /*trait*/ QPlainTextEdit_NewQPlainTextEdit for (QWidget) {
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

  // proto:  void QPlainTextEdit::cut();
impl /*struct*/ QPlainTextEdit {
  pub fn cut<RetType, T: QPlainTextEdit_cut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cut(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cut<RetType> {
  fn cut(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::cut();
impl<'a> /*trait*/ QPlainTextEdit_cut<()> for () {
  fn cut(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit3cutEv()};
     unsafe {_ZN14QPlainTextEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
impl /*struct*/ QPlainTextEdit {
  pub fn appendHtml<RetType, T: QPlainTextEdit_appendHtml<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.appendHtml(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_appendHtml<RetType> {
  fn appendHtml(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
impl<'a> /*trait*/ QPlainTextEdit_appendHtml<()> for (QString) {
  fn appendHtml(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit10appendHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit10appendHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
impl /*struct*/ QPlainTextEdit {
  pub fn isUndoRedoEnabled<RetType, T: QPlainTextEdit_isUndoRedoEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_isUndoRedoEnabled<RetType> {
  fn isUndoRedoEnabled(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QPlainTextEdit_isUndoRedoEnabled<i8> for () {
  fn isUndoRedoEnabled(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17isUndoRedoEnabledEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::zoomOut(int range);
impl /*struct*/ QPlainTextEdit {
  pub fn zoomOut<RetType, T: QPlainTextEdit_zoomOut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.zoomOut(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_zoomOut<RetType> {
  fn zoomOut(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomOut<()> for (i32) {
  fn zoomOut(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QPlainTextEdit7zoomOutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
impl /*struct*/ QPlainTextEdit {
  pub fn setPlaceholderText<RetType, T: QPlainTextEdit_setPlaceholderText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setPlaceholderText<RetType> {
  fn setPlaceholderText(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QPlainTextEdit_setPlaceholderText<()> for (QString) {
  fn setPlaceholderText(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::undo();
impl /*struct*/ QPlainTextEdit {
  pub fn undo<RetType, T: QPlainTextEdit_undo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_undo<RetType> {
  fn undo(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::undo();
impl<'a> /*trait*/ QPlainTextEdit_undo<()> for () {
  fn undo(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4undoEv()};
     unsafe {_ZN14QPlainTextEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::modificationChanged(bool );
impl /*struct*/ QPlainTextEdit {
  pub fn modificationChanged<RetType, T: QPlainTextEdit_modificationChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.modificationChanged(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_modificationChanged<RetType> {
  fn modificationChanged(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::modificationChanged(bool );
impl<'a> /*trait*/ QPlainTextEdit_modificationChanged<()> for (i8) {
  fn modificationChanged(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19modificationChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit19modificationChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
impl /*struct*/ QPlainTextEdit {
  pub fn cursorForPosition<RetType, T: QPlainTextEdit_cursorForPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorForPosition(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorForPosition<RetType> {
  fn cursorForPosition(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_cursorForPosition<QTextCursor> for (QPoint) {
  fn cursorForPosition(self , rsthis: &mut QPlainTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::centerOnScroll();
impl /*struct*/ QPlainTextEdit {
  pub fn centerOnScroll<RetType, T: QPlainTextEdit_centerOnScroll<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.centerOnScroll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_centerOnScroll<RetType> {
  fn centerOnScroll(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::centerOnScroll();
impl<'a> /*trait*/ QPlainTextEdit_centerOnScroll<i8> for () {
  fn centerOnScroll(self , rsthis: &mut QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit14centerOnScrollEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit14centerOnScrollEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
impl /*struct*/ QPlainTextEdit {
  pub fn appendPlainText<RetType, T: QPlainTextEdit_appendPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.appendPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_appendPlainText<RetType> {
  fn appendPlainText(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_appendPlainText<()> for (QString) {
  fn appendPlainText(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15appendPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QPlainTextEdit15appendPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::cursorWidth();
impl /*struct*/ QPlainTextEdit {
  pub fn cursorWidth<RetType, T: QPlainTextEdit_cursorWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorWidth<RetType> {
  fn cursorWidth(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::cursorWidth();
impl<'a> /*trait*/ QPlainTextEdit_cursorWidth<i32> for () {
  fn cursorWidth(self , rsthis: &mut QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11cursorWidthEv()};
    let mut ret = unsafe {_ZNK14QPlainTextEdit11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_cursorRect<QRect> for (QTextCursor) {
  fn cursorRect(self , rsthis: &mut QPlainTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::copyAvailable(bool b);
impl /*struct*/ QPlainTextEdit {
  pub fn copyAvailable<RetType, T: QPlainTextEdit_copyAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.copyAvailable(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_copyAvailable<RetType> {
  fn copyAvailable(self , rsthis: &mut QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::copyAvailable(bool b);
impl<'a> /*trait*/ QPlainTextEdit_copyAvailable<()> for (i8) {
  fn copyAvailable(self , rsthis: &mut QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13copyAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QPlainTextEdit13copyAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

