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
use super::qpagedpaintdevice::QPagedPaintDevice;
use super::qbytearray::QByteArray;
use super::qurl::QUrl;
use super::qvariant::QVariant;
use super::qobject::QObject;
use super::qtextoption::QTextOption;
use super::qpainter::QPainter;
use super::qrectf::QRectF;
use super::qfont::QFont;
use super::qtextformat::QTextFormat;
use super::qsizef::QSizeF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextDocument::cursorPositionChanged(const QTextCursor & cursor);
  fn _ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: void QTextDocument::setDefaultStyleSheet(const QString & sheet);
  fn _ZN13QTextDocument20setDefaultStyleSheetERK7QString(arg0: *const c_void) -> i32;
  // proto: QAbstractTextDocumentLayout * QTextDocument::documentLayout();
  fn _ZNK13QTextDocument14documentLayoutEv() -> i32;
  // proto: bool QTextDocument::isModified();
  fn _ZNK13QTextDocument10isModifiedEv() -> i32;
  // proto: int QTextDocument::revision();
  fn _ZNK13QTextDocument8revisionEv() -> i32;
  // proto: QSizeF QTextDocument::pageSize();
  fn _ZNK13QTextDocument8pageSizeEv() -> i32;
  // proto: void QTextDocument::redo(QTextCursor * cursor);
  fn _ZN13QTextDocument4redoEP11QTextCursor(arg0: *mut c_void) -> i32;
  // proto: int QTextDocument::lineCount();
  fn _ZNK13QTextDocument9lineCountEv() -> i32;
  // proto: void QTextDocument::print(QPagedPaintDevice * printer);
  fn _ZNK13QTextDocument5printEP17QPagedPaintDevice(arg0: *mut c_void) -> i32;
  // proto: QTextDocumentPrivate * QTextDocument::docHandle();
  fn _ZNK13QTextDocument9docHandleEv() -> i32;
  // proto: QString QTextDocument::toHtml(const QByteArray & encoding);
  fn _ZNK13QTextDocument6toHtmlERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: int QTextDocument::availableUndoSteps();
  fn _ZNK13QTextDocument18availableUndoStepsEv() -> i32;
  // proto: void QTextDocument::undoAvailable(bool );
  fn _ZN13QTextDocument13undoAvailableEb(arg0: int8_t) -> i32;
  // proto: void QTextDocument::setUndoRedoEnabled(bool enable);
  fn _ZN13QTextDocument18setUndoRedoEnabledEb(arg0: int8_t) -> i32;
  // proto: void QTextDocument::undo(QTextCursor * cursor);
  fn _ZN13QTextDocument4undoEP11QTextCursor(arg0: *mut c_void) -> i32;
  // proto: QString QTextDocument::toPlainText();
  fn _ZNK13QTextDocument11toPlainTextEv() -> i32;
  // proto: void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
  fn _ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(arg0: c_int, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QSizeF QTextDocument::size();
  fn _ZNK13QTextDocument4sizeEv() -> i32;
  // proto: QTextObject * QTextDocument::object(int objectIndex);
  fn _ZNK13QTextDocument6objectEi(arg0: c_int) -> i32;
  // proto: QTextDocument * QTextDocument::clone(QObject * parent);
  fn _ZNK13QTextDocument5cloneEP7QObject(arg0: *mut c_void) -> i32;
  // proto: void QTextDocument::markContentsDirty(int from, int length);
  fn _ZN13QTextDocument17markContentsDirtyEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QTextDocument::NewQTextDocument(QObject * parent);
  fn _ZN13QTextDocumentC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QTextDocument::modificationChanged(bool m);
  fn _ZN13QTextDocument19modificationChangedEb(arg0: int8_t) -> i32;
  // proto: int QTextDocument::characterCount();
  fn _ZNK13QTextDocument14characterCountEv() -> i32;
  // proto: QTextFrame * QTextDocument::rootFrame();
  fn _ZNK13QTextDocument9rootFrameEv() -> i32;
  // proto: QTextBlock QTextDocument::firstBlock();
  fn _ZNK13QTextDocument10firstBlockEv() -> i32;
  // proto: int QTextDocument::blockCount();
  fn _ZNK13QTextDocument10blockCountEv() -> i32;
  // proto: double QTextDocument::idealWidth();
  fn _ZNK13QTextDocument10idealWidthEv() -> i32;
  // proto: void QTextDocument::adjustSize();
  fn _ZN13QTextDocument10adjustSizeEv() -> i32;
  // proto: bool QTextDocument::isRedoAvailable();
  fn _ZNK13QTextDocument15isRedoAvailableEv() -> i32;
  // proto: QVector<QTextFormat> QTextDocument::allFormats();
  fn _ZNK13QTextDocument10allFormatsEv() -> i32;
  // proto: void QTextDocument::blockCountChanged(int newBlockCount);
  fn _ZN13QTextDocument17blockCountChangedEi(arg0: c_int) -> i32;
  // proto: QString QTextDocument::defaultStyleSheet();
  fn _ZNK13QTextDocument17defaultStyleSheetEv() -> i32;
  // proto: QTextBlock QTextDocument::lastBlock();
  fn _ZNK13QTextDocument9lastBlockEv() -> i32;
  // proto: void QTextDocument::NewQTextDocument(const QString & text, QObject * parent);
  fn _ZN13QTextDocumentC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: bool QTextDocument::useDesignMetrics();
  fn _ZNK13QTextDocument16useDesignMetricsEv() -> i32;
  // proto: void QTextDocument::documentLayoutChanged();
  fn _ZN13QTextDocument21documentLayoutChangedEv() -> i32;
  // proto: int QTextDocument::pageCount();
  fn _ZNK13QTextDocument9pageCountEv() -> i32;
  // proto: void QTextDocument::baseUrlChanged(const QUrl & url);
  fn _ZN13QTextDocument14baseUrlChangedERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QTextDocument::setTextWidth(qreal width);
  fn _ZN13QTextDocument12setTextWidthEd(arg0: c_double) -> i32;
  // proto: void QTextDocument::setDocumentMargin(qreal margin);
  fn _ZN13QTextDocument17setDocumentMarginEd(arg0: c_double) -> i32;
  // proto: bool QTextDocument::isUndoAvailable();
  fn _ZNK13QTextDocument15isUndoAvailableEv() -> i32;
  // proto: double QTextDocument::indentWidth();
  fn _ZNK13QTextDocument11indentWidthEv() -> i32;
  // proto: void QTextDocument::undoCommandAdded();
  fn _ZN13QTextDocument16undoCommandAddedEv() -> i32;
  // proto: void QTextDocument::setUseDesignMetrics(bool b);
  fn _ZN13QTextDocument19setUseDesignMetricsEb(arg0: int8_t) -> i32;
  // proto: void QTextDocument::setIndentWidth(qreal width);
  fn _ZN13QTextDocument14setIndentWidthEd(arg0: c_double) -> i32;
  // proto: QUrl QTextDocument::baseUrl();
  fn _ZNK13QTextDocument7baseUrlEv() -> i32;
  // proto: QTextFrame * QTextDocument::frameAt(int pos);
  fn _ZNK13QTextDocument7frameAtEi(arg0: c_int) -> i32;
  // proto: void QTextDocument::NewQTextDocument(const QTextDocument & );
  fn _ZN13QTextDocumentC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextDocument::setDefaultTextOption(const QTextOption & option);
  fn _ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(arg0: *const c_void) -> i32;
  // proto: QFont QTextDocument::defaultFont();
  fn _ZNK13QTextDocument11defaultFontEv() -> i32;
  // proto: QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
  fn _ZNK13QTextDocument17findBlockByNumberEi(arg0: c_int) -> i32;
  // proto: QTextOption QTextDocument::defaultTextOption();
  fn _ZNK13QTextDocument17defaultTextOptionEv() -> i32;
  // proto: QTextBlock QTextDocument::findBlock(int pos);
  fn _ZNK13QTextDocument9findBlockEi(arg0: c_int) -> i32;
  // proto: void QTextDocument::setBaseUrl(const QUrl & url);
  fn _ZN13QTextDocument10setBaseUrlERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QTextDocument::redoAvailable(bool );
  fn _ZN13QTextDocument13redoAvailableEb(arg0: int8_t) -> i32;
  // proto: void QTextDocument::redo();
  fn _ZN13QTextDocument4redoEv() -> i32;
  // proto: void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
  fn _ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
  fn _ZNK13QTextDocument21findBlockByLineNumberEi(arg0: c_int) -> i32;
  // proto: void QTextDocument::undo();
  fn _ZN13QTextDocument4undoEv() -> i32;
  // proto: double QTextDocument::textWidth();
  fn _ZNK13QTextDocument9textWidthEv() -> i32;
  // proto: const QMetaObject * QTextDocument::metaObject();
  fn _ZNK13QTextDocument10metaObjectEv() -> i32;
  // proto: int QTextDocument::availableRedoSteps();
  fn _ZNK13QTextDocument18availableRedoStepsEv() -> i32;
  // proto: QChar QTextDocument::characterAt(int pos);
  fn _ZNK13QTextDocument11characterAtEi(arg0: c_int) -> i32;
  // proto: void QTextDocument::setDefaultFont(const QFont & font);
  fn _ZN13QTextDocument14setDefaultFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
  fn _ZNK13QTextDocument15objectForFormatERK11QTextFormat(arg0: *const c_void) -> i32;
  // proto: bool QTextDocument::isEmpty();
  fn _ZNK13QTextDocument7isEmptyEv() -> i32;
  // proto: bool QTextDocument::isUndoRedoEnabled();
  fn _ZNK13QTextDocument17isUndoRedoEnabledEv() -> i32;
  // proto: void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
  fn _ZN13QTextDocument14contentsChangeEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QTextDocument::FreeQTextDocument();
  fn _ZN13QTextDocumentD0Ev() -> i32;
  // proto: void QTextDocument::contentsChanged();
  fn _ZN13QTextDocument15contentsChangedEv() -> i32;
  // proto: double QTextDocument::documentMargin();
  fn _ZNK13QTextDocument14documentMarginEv() -> i32;
  // proto: void QTextDocument::setPageSize(const QSizeF & size);
  fn _ZN13QTextDocument11setPageSizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QTextDocument::setHtml(const QString & html);
  fn _ZN13QTextDocument7setHtmlERK7QString(arg0: *const c_void) -> i32;
  // proto: QTextBlock QTextDocument::end();
  fn _ZNK13QTextDocument3endEv() -> i32;
  // proto: int QTextDocument::maximumBlockCount();
  fn _ZNK13QTextDocument17maximumBlockCountEv() -> i32;
  // proto: void QTextDocument::setPlainText(const QString & text);
  fn _ZN13QTextDocument12setPlainTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QTextDocument::clear();
  fn _ZN13QTextDocument5clearEv() -> i32;
  // proto: QVariant QTextDocument::resource(int type, const QUrl & name);
  fn _ZNK13QTextDocument8resourceEiRK4QUrl(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QTextBlock QTextDocument::begin();
  fn _ZNK13QTextDocument5beginEv() -> i32;
  // proto: void QTextDocument::setMaximumBlockCount(int maximum);
  fn _ZN13QTextDocument20setMaximumBlockCountEi(arg0: c_int) -> i32;
  // proto: void QTextDocument::setModified(bool m);
  fn _ZN13QTextDocument11setModifiedEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QTextDocument)=1
pub struct QTextDocument {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextDocument {
  pub fn cursorPositionChanged<T: QTextDocument_cursorPositionChanged>(&mut self, value: T) -> i32 {
    value.cursorPositionChanged(self);
    return 1;
  }
}

pub trait QTextDocument_cursorPositionChanged {
  fn cursorPositionChanged(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::cursorPositionChanged(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextDocument_cursorPositionChanged for (&'a  QTextCursor) {
  fn cursorPositionChanged(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDefaultStyleSheet<T: QTextDocument_setDefaultStyleSheet>(&mut self, value: T) -> i32 {
    value.setDefaultStyleSheet(self);
    return 1;
  }
}

pub trait QTextDocument_setDefaultStyleSheet {
  fn setDefaultStyleSheet(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setDefaultStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QTextDocument_setDefaultStyleSheet for (&'a  QString) {
  fn setDefaultStyleSheet(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument20setDefaultStyleSheetERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn documentLayout<T: QTextDocument_documentLayout>(&mut self, value: T) -> i32 {
    value.documentLayout(self);
    return 1;
  }
}

pub trait QTextDocument_documentLayout {
  fn documentLayout(self, this: &mut QTextDocument) -> i32;
}

// proto: QAbstractTextDocumentLayout * QTextDocument::documentLayout();
impl<'a> /*trait*/ QTextDocument_documentLayout for () {
  fn documentLayout(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentLayoutEv()};
    unsafe {_ZNK13QTextDocument14documentLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isModified<T: QTextDocument_isModified>(&mut self, value: T) -> i32 {
    value.isModified(self);
    return 1;
  }
}

pub trait QTextDocument_isModified {
  fn isModified(self, this: &mut QTextDocument) -> i32;
}

// proto: bool QTextDocument::isModified();
impl<'a> /*trait*/ QTextDocument_isModified for () {
  fn isModified(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10isModifiedEv()};
    unsafe {_ZNK13QTextDocument10isModifiedEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn revision<T: QTextDocument_revision>(&mut self, value: T) -> i32 {
    value.revision(self);
    return 1;
  }
}

pub trait QTextDocument_revision {
  fn revision(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::revision();
impl<'a> /*trait*/ QTextDocument_revision for () {
  fn revision(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8revisionEv()};
    unsafe {_ZNK13QTextDocument8revisionEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn pageSize<T: QTextDocument_pageSize>(&mut self, value: T) -> i32 {
    value.pageSize(self);
    return 1;
  }
}

pub trait QTextDocument_pageSize {
  fn pageSize(self, this: &mut QTextDocument) -> i32;
}

// proto: QSizeF QTextDocument::pageSize();
impl<'a> /*trait*/ QTextDocument_pageSize for () {
  fn pageSize(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8pageSizeEv()};
    unsafe {_ZNK13QTextDocument8pageSizeEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn redo<T: QTextDocument_redo>(&mut self, value: T) -> i32 {
    value.redo(self);
    return 1;
  }
}

pub trait QTextDocument_redo {
  fn redo(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::redo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_redo for (&'a mut QTextCursor) {
  fn redo(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTextDocument4redoEP11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn lineCount<T: QTextDocument_lineCount>(&mut self, value: T) -> i32 {
    value.lineCount(self);
    return 1;
  }
}

pub trait QTextDocument_lineCount {
  fn lineCount(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::lineCount();
impl<'a> /*trait*/ QTextDocument_lineCount for () {
  fn lineCount(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lineCountEv()};
    unsafe {_ZNK13QTextDocument9lineCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn print<T: QTextDocument_print>(&mut self, value: T) -> i32 {
    value.print(self);
    return 1;
  }
}

pub trait QTextDocument_print {
  fn print(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextDocument_print for (&'a mut QPagedPaintDevice) {
  fn print(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QTextDocument5printEP17QPagedPaintDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn docHandle<T: QTextDocument_docHandle>(&mut self, value: T) -> i32 {
    value.docHandle(self);
    return 1;
  }
}

pub trait QTextDocument_docHandle {
  fn docHandle(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextDocumentPrivate * QTextDocument::docHandle();
impl<'a> /*trait*/ QTextDocument_docHandle for () {
  fn docHandle(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9docHandleEv()};
    unsafe {_ZNK13QTextDocument9docHandleEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn toHtml<T: QTextDocument_toHtml>(&mut self, value: T) -> i32 {
    value.toHtml(self);
    return 1;
  }
}

pub trait QTextDocument_toHtml {
  fn toHtml(self, this: &mut QTextDocument) -> i32;
}

// proto: QString QTextDocument::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocument_toHtml for (&'a  QByteArray) {
  fn toHtml(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QTextDocument6toHtmlERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn availableUndoSteps<T: QTextDocument_availableUndoSteps>(&mut self, value: T) -> i32 {
    value.availableUndoSteps(self);
    return 1;
  }
}

pub trait QTextDocument_availableUndoSteps {
  fn availableUndoSteps(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::availableUndoSteps();
impl<'a> /*trait*/ QTextDocument_availableUndoSteps for () {
  fn availableUndoSteps(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableUndoStepsEv()};
    unsafe {_ZNK13QTextDocument18availableUndoStepsEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn undoAvailable<T: QTextDocument_undoAvailable>(&mut self, value: T) -> i32 {
    value.undoAvailable(self);
    return 1;
  }
}

pub trait QTextDocument_undoAvailable {
  fn undoAvailable(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::undoAvailable(bool );
impl<'a> /*trait*/ QTextDocument_undoAvailable for (i8) {
  fn undoAvailable(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument13undoAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTextDocument13undoAvailableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setUndoRedoEnabled<T: QTextDocument_setUndoRedoEnabled>(&mut self, value: T) -> i32 {
    value.setUndoRedoEnabled(self);
    return 1;
  }
}

pub trait QTextDocument_setUndoRedoEnabled {
  fn setUndoRedoEnabled(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextDocument_setUndoRedoEnabled for (i8) {
  fn setUndoRedoEnabled(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument18setUndoRedoEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTextDocument18setUndoRedoEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn undo<T: QTextDocument_undo>(&mut self, value: T) -> i32 {
    value.undo(self);
    return 1;
  }
}

pub trait QTextDocument_undo {
  fn undo(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::undo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_undo for (&'a mut QTextCursor) {
  fn undo(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTextDocument4undoEP11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn toPlainText<T: QTextDocument_toPlainText>(&mut self, value: T) -> i32 {
    value.toPlainText(self);
    return 1;
  }
}

pub trait QTextDocument_toPlainText {
  fn toPlainText(self, this: &mut QTextDocument) -> i32;
}

// proto: QString QTextDocument::toPlainText();
impl<'a> /*trait*/ QTextDocument_toPlainText for () {
  fn toPlainText(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11toPlainTextEv()};
    unsafe {_ZNK13QTextDocument11toPlainTextEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn addResource<T: QTextDocument_addResource>(&mut self, value: T) -> i32 {
    value.addResource(self);
    return 1;
  }
}

pub trait QTextDocument_addResource {
  fn addResource(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
impl<'a> /*trait*/ QTextDocument_addResource for (i32, &'a  QUrl, &'a  QVariant) {
  fn addResource(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn size<T: QTextDocument_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QTextDocument_size {
  fn size(self, this: &mut QTextDocument) -> i32;
}

// proto: QSizeF QTextDocument::size();
impl<'a> /*trait*/ QTextDocument_size for () {
  fn size(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument4sizeEv()};
    unsafe {_ZNK13QTextDocument4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn object<T: QTextDocument_object>(&mut self, value: T) -> i32 {
    value.object(self);
    return 1;
  }
}

pub trait QTextDocument_object {
  fn object(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextObject * QTextDocument::object(int objectIndex);
impl<'a> /*trait*/ QTextDocument_object for (i32) {
  fn object(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6objectEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextDocument6objectEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn clone<T: QTextDocument_clone>(&mut self, value: T) -> i32 {
    value.clone(self);
    return 1;
  }
}

pub trait QTextDocument_clone {
  fn clone(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextDocument * QTextDocument::clone(QObject * parent);
impl<'a> /*trait*/ QTextDocument_clone for (&'a mut QObject) {
  fn clone(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5cloneEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QTextDocument5cloneEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn markContentsDirty<T: QTextDocument_markContentsDirty>(&mut self, value: T) -> i32 {
    value.markContentsDirty(self);
    return 1;
  }
}

pub trait QTextDocument_markContentsDirty {
  fn markContentsDirty(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::markContentsDirty(int from, int length);
impl<'a> /*trait*/ QTextDocument_markContentsDirty for (i32, i32) {
  fn markContentsDirty(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17markContentsDirtyEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QTextDocument17markContentsDirtyEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn NewQTextDocument<T: QTextDocument_NewQTextDocument>(value: T) -> QTextDocument {
    let rsthis = value.NewQTextDocument();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocument_NewQTextDocument {
  fn NewQTextDocument(self) -> QTextDocument;
}

// proto: void QTextDocument::NewQTextDocument(QObject * parent);
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (&'a mut QObject) {
  fn NewQTextDocument(self) -> QTextDocument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTextDocumentC1EP7QObject(qthis, arg0)};
    let rsthis = QTextDocument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn modificationChanged<T: QTextDocument_modificationChanged>(&mut self, value: T) -> i32 {
    value.modificationChanged(self);
    return 1;
  }
}

pub trait QTextDocument_modificationChanged {
  fn modificationChanged(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::modificationChanged(bool m);
impl<'a> /*trait*/ QTextDocument_modificationChanged for (i8) {
  fn modificationChanged(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19modificationChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTextDocument19modificationChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn characterCount<T: QTextDocument_characterCount>(&mut self, value: T) -> i32 {
    value.characterCount(self);
    return 1;
  }
}

pub trait QTextDocument_characterCount {
  fn characterCount(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::characterCount();
impl<'a> /*trait*/ QTextDocument_characterCount for () {
  fn characterCount(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14characterCountEv()};
    unsafe {_ZNK13QTextDocument14characterCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn rootFrame<T: QTextDocument_rootFrame>(&mut self, value: T) -> i32 {
    value.rootFrame(self);
    return 1;
  }
}

pub trait QTextDocument_rootFrame {
  fn rootFrame(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextFrame * QTextDocument::rootFrame();
impl<'a> /*trait*/ QTextDocument_rootFrame for () {
  fn rootFrame(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9rootFrameEv()};
    unsafe {_ZNK13QTextDocument9rootFrameEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn firstBlock<T: QTextDocument_firstBlock>(&mut self, value: T) -> i32 {
    value.firstBlock(self);
    return 1;
  }
}

pub trait QTextDocument_firstBlock {
  fn firstBlock(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::firstBlock();
impl<'a> /*trait*/ QTextDocument_firstBlock for () {
  fn firstBlock(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10firstBlockEv()};
    unsafe {_ZNK13QTextDocument10firstBlockEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn blockCount<T: QTextDocument_blockCount>(&mut self, value: T) -> i32 {
    value.blockCount(self);
    return 1;
  }
}

pub trait QTextDocument_blockCount {
  fn blockCount(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::blockCount();
impl<'a> /*trait*/ QTextDocument_blockCount for () {
  fn blockCount(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10blockCountEv()};
    unsafe {_ZNK13QTextDocument10blockCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn idealWidth<T: QTextDocument_idealWidth>(&mut self, value: T) -> i32 {
    value.idealWidth(self);
    return 1;
  }
}

pub trait QTextDocument_idealWidth {
  fn idealWidth(self, this: &mut QTextDocument) -> i32;
}

// proto: double QTextDocument::idealWidth();
impl<'a> /*trait*/ QTextDocument_idealWidth for () {
  fn idealWidth(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10idealWidthEv()};
    unsafe {_ZNK13QTextDocument10idealWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn adjustSize<T: QTextDocument_adjustSize>(&mut self, value: T) -> i32 {
    value.adjustSize(self);
    return 1;
  }
}

pub trait QTextDocument_adjustSize {
  fn adjustSize(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::adjustSize();
impl<'a> /*trait*/ QTextDocument_adjustSize for () {
  fn adjustSize(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10adjustSizeEv()};
    unsafe {_ZN13QTextDocument10adjustSizeEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isRedoAvailable<T: QTextDocument_isRedoAvailable>(&mut self, value: T) -> i32 {
    value.isRedoAvailable(self);
    return 1;
  }
}

pub trait QTextDocument_isRedoAvailable {
  fn isRedoAvailable(self, this: &mut QTextDocument) -> i32;
}

// proto: bool QTextDocument::isRedoAvailable();
impl<'a> /*trait*/ QTextDocument_isRedoAvailable for () {
  fn isRedoAvailable(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isRedoAvailableEv()};
    unsafe {_ZNK13QTextDocument15isRedoAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn allFormats<T: QTextDocument_allFormats>(&mut self, value: T) -> i32 {
    value.allFormats(self);
    return 1;
  }
}

pub trait QTextDocument_allFormats {
  fn allFormats(self, this: &mut QTextDocument) -> i32;
}

// proto: QVector<QTextFormat> QTextDocument::allFormats();
impl<'a> /*trait*/ QTextDocument_allFormats for () {
  fn allFormats(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10allFormatsEv()};
    unsafe {_ZNK13QTextDocument10allFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn blockCountChanged<T: QTextDocument_blockCountChanged>(&mut self, value: T) -> i32 {
    value.blockCountChanged(self);
    return 1;
  }
}

pub trait QTextDocument_blockCountChanged {
  fn blockCountChanged(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::blockCountChanged(int newBlockCount);
impl<'a> /*trait*/ QTextDocument_blockCountChanged for (i32) {
  fn blockCountChanged(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17blockCountChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QTextDocument17blockCountChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn defaultStyleSheet<T: QTextDocument_defaultStyleSheet>(&mut self, value: T) -> i32 {
    value.defaultStyleSheet(self);
    return 1;
  }
}

pub trait QTextDocument_defaultStyleSheet {
  fn defaultStyleSheet(self, this: &mut QTextDocument) -> i32;
}

// proto: QString QTextDocument::defaultStyleSheet();
impl<'a> /*trait*/ QTextDocument_defaultStyleSheet for () {
  fn defaultStyleSheet(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultStyleSheetEv()};
    unsafe {_ZNK13QTextDocument17defaultStyleSheetEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn lastBlock<T: QTextDocument_lastBlock>(&mut self, value: T) -> i32 {
    value.lastBlock(self);
    return 1;
  }
}

pub trait QTextDocument_lastBlock {
  fn lastBlock(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::lastBlock();
impl<'a> /*trait*/ QTextDocument_lastBlock for () {
  fn lastBlock(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lastBlockEv()};
    unsafe {_ZNK13QTextDocument9lastBlockEv()};
    return 1;
  }
}

// proto: void QTextDocument::NewQTextDocument(const QString & text, QObject * parent);
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (&'a  QString, &'a mut QObject) {
  fn NewQTextDocument(self) -> QTextDocument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QTextDocumentC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QTextDocument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn useDesignMetrics<T: QTextDocument_useDesignMetrics>(&mut self, value: T) -> i32 {
    value.useDesignMetrics(self);
    return 1;
  }
}

pub trait QTextDocument_useDesignMetrics {
  fn useDesignMetrics(self, this: &mut QTextDocument) -> i32;
}

// proto: bool QTextDocument::useDesignMetrics();
impl<'a> /*trait*/ QTextDocument_useDesignMetrics for () {
  fn useDesignMetrics(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument16useDesignMetricsEv()};
    unsafe {_ZNK13QTextDocument16useDesignMetricsEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn documentLayoutChanged<T: QTextDocument_documentLayoutChanged>(&mut self, value: T) -> i32 {
    value.documentLayoutChanged(self);
    return 1;
  }
}

pub trait QTextDocument_documentLayoutChanged {
  fn documentLayoutChanged(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::documentLayoutChanged();
impl<'a> /*trait*/ QTextDocument_documentLayoutChanged for () {
  fn documentLayoutChanged(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument21documentLayoutChangedEv()};
    unsafe {_ZN13QTextDocument21documentLayoutChangedEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn pageCount<T: QTextDocument_pageCount>(&mut self, value: T) -> i32 {
    value.pageCount(self);
    return 1;
  }
}

pub trait QTextDocument_pageCount {
  fn pageCount(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::pageCount();
impl<'a> /*trait*/ QTextDocument_pageCount for () {
  fn pageCount(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9pageCountEv()};
    unsafe {_ZNK13QTextDocument9pageCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn baseUrlChanged<T: QTextDocument_baseUrlChanged>(&mut self, value: T) -> i32 {
    value.baseUrlChanged(self);
    return 1;
  }
}

pub trait QTextDocument_baseUrlChanged {
  fn baseUrlChanged(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::baseUrlChanged(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_baseUrlChanged for (&'a  QUrl) {
  fn baseUrlChanged(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14baseUrlChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument14baseUrlChangedERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setTextWidth<T: QTextDocument_setTextWidth>(&mut self, value: T) -> i32 {
    value.setTextWidth(self);
    return 1;
  }
}

pub trait QTextDocument_setTextWidth {
  fn setTextWidth(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setTextWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setTextWidth for (f64) {
  fn setTextWidth(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setTextWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QTextDocument12setTextWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDocumentMargin<T: QTextDocument_setDocumentMargin>(&mut self, value: T) -> i32 {
    value.setDocumentMargin(self);
    return 1;
  }
}

pub trait QTextDocument_setDocumentMargin {
  fn setDocumentMargin(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setDocumentMargin(qreal margin);
impl<'a> /*trait*/ QTextDocument_setDocumentMargin for (f64) {
  fn setDocumentMargin(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17setDocumentMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QTextDocument17setDocumentMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isUndoAvailable<T: QTextDocument_isUndoAvailable>(&mut self, value: T) -> i32 {
    value.isUndoAvailable(self);
    return 1;
  }
}

pub trait QTextDocument_isUndoAvailable {
  fn isUndoAvailable(self, this: &mut QTextDocument) -> i32;
}

// proto: bool QTextDocument::isUndoAvailable();
impl<'a> /*trait*/ QTextDocument_isUndoAvailable for () {
  fn isUndoAvailable(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isUndoAvailableEv()};
    unsafe {_ZNK13QTextDocument15isUndoAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn indentWidth<T: QTextDocument_indentWidth>(&mut self, value: T) -> i32 {
    value.indentWidth(self);
    return 1;
  }
}

pub trait QTextDocument_indentWidth {
  fn indentWidth(self, this: &mut QTextDocument) -> i32;
}

// proto: double QTextDocument::indentWidth();
impl<'a> /*trait*/ QTextDocument_indentWidth for () {
  fn indentWidth(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11indentWidthEv()};
    unsafe {_ZNK13QTextDocument11indentWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn undoCommandAdded<T: QTextDocument_undoCommandAdded>(&mut self, value: T) -> i32 {
    value.undoCommandAdded(self);
    return 1;
  }
}

pub trait QTextDocument_undoCommandAdded {
  fn undoCommandAdded(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::undoCommandAdded();
impl<'a> /*trait*/ QTextDocument_undoCommandAdded for () {
  fn undoCommandAdded(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument16undoCommandAddedEv()};
    unsafe {_ZN13QTextDocument16undoCommandAddedEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setUseDesignMetrics<T: QTextDocument_setUseDesignMetrics>(&mut self, value: T) -> i32 {
    value.setUseDesignMetrics(self);
    return 1;
  }
}

pub trait QTextDocument_setUseDesignMetrics {
  fn setUseDesignMetrics(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextDocument_setUseDesignMetrics for (i8) {
  fn setUseDesignMetrics(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19setUseDesignMetricsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTextDocument19setUseDesignMetricsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setIndentWidth<T: QTextDocument_setIndentWidth>(&mut self, value: T) -> i32 {
    value.setIndentWidth(self);
    return 1;
  }
}

pub trait QTextDocument_setIndentWidth {
  fn setIndentWidth(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setIndentWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setIndentWidth for (f64) {
  fn setIndentWidth(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setIndentWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QTextDocument14setIndentWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn baseUrl<T: QTextDocument_baseUrl>(&mut self, value: T) -> i32 {
    value.baseUrl(self);
    return 1;
  }
}

pub trait QTextDocument_baseUrl {
  fn baseUrl(self, this: &mut QTextDocument) -> i32;
}

// proto: QUrl QTextDocument::baseUrl();
impl<'a> /*trait*/ QTextDocument_baseUrl for () {
  fn baseUrl(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7baseUrlEv()};
    unsafe {_ZNK13QTextDocument7baseUrlEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn frameAt<T: QTextDocument_frameAt>(&mut self, value: T) -> i32 {
    value.frameAt(self);
    return 1;
  }
}

pub trait QTextDocument_frameAt {
  fn frameAt(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextFrame * QTextDocument::frameAt(int pos);
impl<'a> /*trait*/ QTextDocument_frameAt for (i32) {
  fn frameAt(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7frameAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextDocument7frameAtEi(arg0)};
    return 1;
  }
}

// proto: void QTextDocument::NewQTextDocument(const QTextDocument & );
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (&'a  QTextDocument) {
  fn NewQTextDocument(self) -> QTextDocument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocumentC1ERKS_(qthis, arg0)};
    let rsthis = QTextDocument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDefaultTextOption<T: QTextDocument_setDefaultTextOption>(&mut self, value: T) -> i32 {
    value.setDefaultTextOption(self);
    return 1;
  }
}

pub trait QTextDocument_setDefaultTextOption {
  fn setDefaultTextOption(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setDefaultTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextDocument_setDefaultTextOption for (&'a  QTextOption) {
  fn setDefaultTextOption(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn defaultFont<T: QTextDocument_defaultFont>(&mut self, value: T) -> i32 {
    value.defaultFont(self);
    return 1;
  }
}

pub trait QTextDocument_defaultFont {
  fn defaultFont(self, this: &mut QTextDocument) -> i32;
}

// proto: QFont QTextDocument::defaultFont();
impl<'a> /*trait*/ QTextDocument_defaultFont for () {
  fn defaultFont(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11defaultFontEv()};
    unsafe {_ZNK13QTextDocument11defaultFontEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn findBlockByNumber<T: QTextDocument_findBlockByNumber>(&mut self, value: T) -> i32 {
    value.findBlockByNumber(self);
    return 1;
  }
}

pub trait QTextDocument_findBlockByNumber {
  fn findBlockByNumber(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByNumber for (i32) {
  fn findBlockByNumber(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17findBlockByNumberEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextDocument17findBlockByNumberEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn defaultTextOption<T: QTextDocument_defaultTextOption>(&mut self, value: T) -> i32 {
    value.defaultTextOption(self);
    return 1;
  }
}

pub trait QTextDocument_defaultTextOption {
  fn defaultTextOption(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextOption QTextDocument::defaultTextOption();
impl<'a> /*trait*/ QTextDocument_defaultTextOption for () {
  fn defaultTextOption(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultTextOptionEv()};
    unsafe {_ZNK13QTextDocument17defaultTextOptionEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn findBlock<T: QTextDocument_findBlock>(&mut self, value: T) -> i32 {
    value.findBlock(self);
    return 1;
  }
}

pub trait QTextDocument_findBlock {
  fn findBlock(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::findBlock(int pos);
impl<'a> /*trait*/ QTextDocument_findBlock for (i32) {
  fn findBlock(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9findBlockEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextDocument9findBlockEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setBaseUrl<T: QTextDocument_setBaseUrl>(&mut self, value: T) -> i32 {
    value.setBaseUrl(self);
    return 1;
  }
}

pub trait QTextDocument_setBaseUrl {
  fn setBaseUrl(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setBaseUrl(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_setBaseUrl for (&'a  QUrl) {
  fn setBaseUrl(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10setBaseUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument10setBaseUrlERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn redoAvailable<T: QTextDocument_redoAvailable>(&mut self, value: T) -> i32 {
    value.redoAvailable(self);
    return 1;
  }
}

pub trait QTextDocument_redoAvailable {
  fn redoAvailable(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::redoAvailable(bool );
impl<'a> /*trait*/ QTextDocument_redoAvailable for (i8) {
  fn redoAvailable(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument13redoAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTextDocument13redoAvailableEb(arg0)};
    return 1;
  }
}

// proto: void QTextDocument::redo();
impl<'a> /*trait*/ QTextDocument_redo for () {
  fn redo(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEv()};
    unsafe {_ZN13QTextDocument4redoEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn drawContents<T: QTextDocument_drawContents>(&mut self, value: T) -> i32 {
    value.drawContents(self);
    return 1;
  }
}

pub trait QTextDocument_drawContents {
  fn drawContents(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
impl<'a> /*trait*/ QTextDocument_drawContents for (&'a mut QPainter, &'a  QRectF) {
  fn drawContents(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn findBlockByLineNumber<T: QTextDocument_findBlockByLineNumber>(&mut self, value: T) -> i32 {
    value.findBlockByLineNumber(self);
    return 1;
  }
}

pub trait QTextDocument_findBlockByLineNumber {
  fn findBlockByLineNumber(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByLineNumber for (i32) {
  fn findBlockByLineNumber(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument21findBlockByLineNumberEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextDocument21findBlockByLineNumberEi(arg0)};
    return 1;
  }
}

// proto: void QTextDocument::undo();
impl<'a> /*trait*/ QTextDocument_undo for () {
  fn undo(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEv()};
    unsafe {_ZN13QTextDocument4undoEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn textWidth<T: QTextDocument_textWidth>(&mut self, value: T) -> i32 {
    value.textWidth(self);
    return 1;
  }
}

pub trait QTextDocument_textWidth {
  fn textWidth(self, this: &mut QTextDocument) -> i32;
}

// proto: double QTextDocument::textWidth();
impl<'a> /*trait*/ QTextDocument_textWidth for () {
  fn textWidth(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9textWidthEv()};
    unsafe {_ZNK13QTextDocument9textWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn metaObject<T: QTextDocument_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextDocument_metaObject {
  fn metaObject(self, this: &mut QTextDocument) -> i32;
}

// proto: const QMetaObject * QTextDocument::metaObject();
impl<'a> /*trait*/ QTextDocument_metaObject for () {
  fn metaObject(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10metaObjectEv()};
    unsafe {_ZNK13QTextDocument10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn availableRedoSteps<T: QTextDocument_availableRedoSteps>(&mut self, value: T) -> i32 {
    value.availableRedoSteps(self);
    return 1;
  }
}

pub trait QTextDocument_availableRedoSteps {
  fn availableRedoSteps(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::availableRedoSteps();
impl<'a> /*trait*/ QTextDocument_availableRedoSteps for () {
  fn availableRedoSteps(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableRedoStepsEv()};
    unsafe {_ZNK13QTextDocument18availableRedoStepsEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn characterAt<T: QTextDocument_characterAt>(&mut self, value: T) -> i32 {
    value.characterAt(self);
    return 1;
  }
}

pub trait QTextDocument_characterAt {
  fn characterAt(self, this: &mut QTextDocument) -> i32;
}

// proto: QChar QTextDocument::characterAt(int pos);
impl<'a> /*trait*/ QTextDocument_characterAt for (i32) {
  fn characterAt(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11characterAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextDocument11characterAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDefaultFont<T: QTextDocument_setDefaultFont>(&mut self, value: T) -> i32 {
    value.setDefaultFont(self);
    return 1;
  }
}

pub trait QTextDocument_setDefaultFont {
  fn setDefaultFont(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setDefaultFont(const QFont & font);
impl<'a> /*trait*/ QTextDocument_setDefaultFont for (&'a  QFont) {
  fn setDefaultFont(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setDefaultFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument14setDefaultFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn objectForFormat<T: QTextDocument_objectForFormat>(&mut self, value: T) -> i32 {
    value.objectForFormat(self);
    return 1;
  }
}

pub trait QTextDocument_objectForFormat {
  fn objectForFormat(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
impl<'a> /*trait*/ QTextDocument_objectForFormat for (&'a  QTextFormat) {
  fn objectForFormat(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15objectForFormatERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QTextDocument15objectForFormatERK11QTextFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isEmpty<T: QTextDocument_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QTextDocument_isEmpty {
  fn isEmpty(self, this: &mut QTextDocument) -> i32;
}

// proto: bool QTextDocument::isEmpty();
impl<'a> /*trait*/ QTextDocument_isEmpty for () {
  fn isEmpty(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7isEmptyEv()};
    unsafe {_ZNK13QTextDocument7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isUndoRedoEnabled<T: QTextDocument_isUndoRedoEnabled>(&mut self, value: T) -> i32 {
    value.isUndoRedoEnabled(self);
    return 1;
  }
}

pub trait QTextDocument_isUndoRedoEnabled {
  fn isUndoRedoEnabled(self, this: &mut QTextDocument) -> i32;
}

// proto: bool QTextDocument::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextDocument_isUndoRedoEnabled for () {
  fn isUndoRedoEnabled(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17isUndoRedoEnabledEv()};
    unsafe {_ZNK13QTextDocument17isUndoRedoEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn contentsChange<T: QTextDocument_contentsChange>(&mut self, value: T) -> i32 {
    value.contentsChange(self);
    return 1;
  }
}

pub trait QTextDocument_contentsChange {
  fn contentsChange(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
impl<'a> /*trait*/ QTextDocument_contentsChange for (i32, i32, i32) {
  fn contentsChange(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14contentsChangeEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN13QTextDocument14contentsChangeEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn FreeQTextDocument<T: QTextDocument_FreeQTextDocument>(&mut self, value: T) -> i32 {
    value.FreeQTextDocument(self);
    return 1;
  }
}

pub trait QTextDocument_FreeQTextDocument {
  fn FreeQTextDocument(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::FreeQTextDocument();
impl<'a> /*trait*/ QTextDocument_FreeQTextDocument for () {
  fn FreeQTextDocument(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentD0Ev()};
    unsafe {_ZN13QTextDocumentD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn contentsChanged<T: QTextDocument_contentsChanged>(&mut self, value: T) -> i32 {
    value.contentsChanged(self);
    return 1;
  }
}

pub trait QTextDocument_contentsChanged {
  fn contentsChanged(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::contentsChanged();
impl<'a> /*trait*/ QTextDocument_contentsChanged for () {
  fn contentsChanged(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument15contentsChangedEv()};
    unsafe {_ZN13QTextDocument15contentsChangedEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn documentMargin<T: QTextDocument_documentMargin>(&mut self, value: T) -> i32 {
    value.documentMargin(self);
    return 1;
  }
}

pub trait QTextDocument_documentMargin {
  fn documentMargin(self, this: &mut QTextDocument) -> i32;
}

// proto: double QTextDocument::documentMargin();
impl<'a> /*trait*/ QTextDocument_documentMargin for () {
  fn documentMargin(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentMarginEv()};
    unsafe {_ZNK13QTextDocument14documentMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setPageSize<T: QTextDocument_setPageSize>(&mut self, value: T) -> i32 {
    value.setPageSize(self);
    return 1;
  }
}

pub trait QTextDocument_setPageSize {
  fn setPageSize(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setPageSize(const QSizeF & size);
impl<'a> /*trait*/ QTextDocument_setPageSize for (&'a  QSizeF) {
  fn setPageSize(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setPageSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument11setPageSizeERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setHtml<T: QTextDocument_setHtml>(&mut self, value: T) -> i32 {
    value.setHtml(self);
    return 1;
  }
}

pub trait QTextDocument_setHtml {
  fn setHtml(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setHtml(const QString & html);
impl<'a> /*trait*/ QTextDocument_setHtml for (&'a  QString) {
  fn setHtml(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument7setHtmlERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn end<T: QTextDocument_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QTextDocument_end {
  fn end(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::end();
impl<'a> /*trait*/ QTextDocument_end for () {
  fn end(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument3endEv()};
    unsafe {_ZNK13QTextDocument3endEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn maximumBlockCount<T: QTextDocument_maximumBlockCount>(&mut self, value: T) -> i32 {
    value.maximumBlockCount(self);
    return 1;
  }
}

pub trait QTextDocument_maximumBlockCount {
  fn maximumBlockCount(self, this: &mut QTextDocument) -> i32;
}

// proto: int QTextDocument::maximumBlockCount();
impl<'a> /*trait*/ QTextDocument_maximumBlockCount for () {
  fn maximumBlockCount(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17maximumBlockCountEv()};
    unsafe {_ZNK13QTextDocument17maximumBlockCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setPlainText<T: QTextDocument_setPlainText>(&mut self, value: T) -> i32 {
    value.setPlainText(self);
    return 1;
  }
}

pub trait QTextDocument_setPlainText {
  fn setPlainText(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextDocument_setPlainText for (&'a  QString) {
  fn setPlainText(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextDocument12setPlainTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn clear<T: QTextDocument_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QTextDocument_clear {
  fn clear(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::clear();
impl<'a> /*trait*/ QTextDocument_clear for () {
  fn clear(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument5clearEv()};
    unsafe {_ZN13QTextDocument5clearEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn resource<T: QTextDocument_resource>(&mut self, value: T) -> i32 {
    value.resource(self);
    return 1;
  }
}

pub trait QTextDocument_resource {
  fn resource(self, this: &mut QTextDocument) -> i32;
}

// proto: QVariant QTextDocument::resource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextDocument_resource for (i32, &'a  QUrl) {
  fn resource(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8resourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QTextDocument8resourceEiRK4QUrl(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn begin<T: QTextDocument_begin>(&mut self, value: T) -> i32 {
    value.begin(self);
    return 1;
  }
}

pub trait QTextDocument_begin {
  fn begin(self, this: &mut QTextDocument) -> i32;
}

// proto: QTextBlock QTextDocument::begin();
impl<'a> /*trait*/ QTextDocument_begin for () {
  fn begin(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5beginEv()};
    unsafe {_ZNK13QTextDocument5beginEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setMaximumBlockCount<T: QTextDocument_setMaximumBlockCount>(&mut self, value: T) -> i32 {
    value.setMaximumBlockCount(self);
    return 1;
  }
}

pub trait QTextDocument_setMaximumBlockCount {
  fn setMaximumBlockCount(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QTextDocument_setMaximumBlockCount for (i32) {
  fn setMaximumBlockCount(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QTextDocument20setMaximumBlockCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setModified<T: QTextDocument_setModified>(&mut self, value: T) -> i32 {
    value.setModified(self);
    return 1;
  }
}

pub trait QTextDocument_setModified {
  fn setModified(self, this: &mut QTextDocument) -> i32;
}

// proto: void QTextDocument::setModified(bool m);
impl<'a> /*trait*/ QTextDocument_setModified for (i8) {
  fn setModified(self, this: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setModifiedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTextDocument11setModifiedEb(arg0)};
    return 1;
  }
}

