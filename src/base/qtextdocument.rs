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
use super::qsizef::QSizeF;
use super::qpagedpaintdevice::QPagedPaintDevice;
use super::qbytearray::QByteArray;
use super::qurl::QUrl;
use super::qvariant::QVariant;
use super::qtextobject::QTextObject;
use super::qobject::QObject;
use super::qtextframe::QTextFrame;
use super::qtextblock::QTextBlock;
use super::qtextoption::QTextOption;
use super::qfont::QFont;
use super::qpainter::QPainter;
use super::qrectf::QRectF;
use super::qchar::QChar;
use super::qtextformat::QTextFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextDocument::cursorPositionChanged(const QTextCursor & cursor);
  fn _ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
  fn _ZN13QTextDocument20setDefaultStyleSheetERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
  fn _ZNK13QTextDocument14documentLayoutEv(qthis: *mut c_void) ;
  // proto:  bool QTextDocument::isModified();
  fn _ZNK13QTextDocument10isModifiedEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTextDocument::revision();
  fn _ZNK13QTextDocument8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  QSizeF QTextDocument::pageSize();
  fn _ZNK13QTextDocument8pageSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::redo(QTextCursor * cursor);
  fn _ZN13QTextDocument4redoEP11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextDocument::lineCount();
  fn _ZNK13QTextDocument9lineCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
  fn _ZNK13QTextDocument5printEP17QPagedPaintDevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
  fn _ZNK13QTextDocument9docHandleEv(qthis: *mut c_void) ;
  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
  fn _ZNK13QTextDocument6toHtmlERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::availableUndoSteps();
  fn _ZNK13QTextDocument18availableUndoStepsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::undoAvailable(bool );
  fn _ZN13QTextDocument13undoAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
  fn _ZN13QTextDocument18setUndoRedoEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextDocument::undo(QTextCursor * cursor);
  fn _ZN13QTextDocument4undoEP11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextDocument::toPlainText();
  fn _ZNK13QTextDocument11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
  fn _ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QSizeF QTextDocument::size();
  fn _ZNK13QTextDocument4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextObject * QTextDocument::object(int objectIndex);
  fn _ZNK13QTextDocument6objectEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
  fn _ZNK13QTextDocument5cloneEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::markContentsDirty(int from, int length);
  fn _ZN13QTextDocument17markContentsDirtyEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTextDocument::NewQTextDocument(QObject * parent);
  fn _ZN13QTextDocumentC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::modificationChanged(bool m);
  fn _ZN13QTextDocument19modificationChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTextDocument::characterCount();
  fn _ZNK13QTextDocument14characterCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextFrame * QTextDocument::rootFrame();
  fn _ZNK13QTextDocument9rootFrameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::firstBlock();
  fn _ZNK13QTextDocument10firstBlockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::blockCount();
  fn _ZNK13QTextDocument10blockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  double QTextDocument::idealWidth();
  fn _ZNK13QTextDocument10idealWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextDocument::adjustSize();
  fn _ZN13QTextDocument10adjustSizeEv(qthis: *mut c_void) ;
  // proto:  bool QTextDocument::isRedoAvailable();
  fn _ZNK13QTextDocument15isRedoAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
  fn _ZNK13QTextDocument10allFormatsEv(qthis: *mut c_void) ;
  // proto:  void QTextDocument::blockCountChanged(int newBlockCount);
  fn _ZN13QTextDocument17blockCountChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QTextDocument::defaultStyleSheet();
  fn _ZNK13QTextDocument17defaultStyleSheetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::lastBlock();
  fn _ZNK13QTextDocument9lastBlockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::NewQTextDocument(const QString & text, QObject * parent);
  fn _ZN13QTextDocumentC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QTextDocument::useDesignMetrics();
  fn _ZNK13QTextDocument16useDesignMetricsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextDocument::documentLayoutChanged();
  fn _ZN13QTextDocument21documentLayoutChangedEv(qthis: *mut c_void) ;
  // proto:  int QTextDocument::pageCount();
  fn _ZNK13QTextDocument9pageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::baseUrlChanged(const QUrl & url);
  fn _ZN13QTextDocument14baseUrlChangedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::setTextWidth(qreal width);
  fn _ZN13QTextDocument12setTextWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
  fn _ZN13QTextDocument17setDocumentMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextDocument::isUndoAvailable();
  fn _ZNK13QTextDocument15isUndoAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QTextDocument::indentWidth();
  fn _ZNK13QTextDocument11indentWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextDocument::undoCommandAdded();
  fn _ZN13QTextDocument16undoCommandAddedEv(qthis: *mut c_void) ;
  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
  fn _ZN13QTextDocument19setUseDesignMetricsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextDocument::setIndentWidth(qreal width);
  fn _ZN13QTextDocument14setIndentWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QUrl QTextDocument::baseUrl();
  fn _ZNK13QTextDocument7baseUrlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
  fn _ZNK13QTextDocument7frameAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::NewQTextDocument(const QTextDocument & );
  fn _ZN13QTextDocumentC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
  fn _ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QFont QTextDocument::defaultFont();
  fn _ZNK13QTextDocument11defaultFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
  fn _ZNK13QTextDocument17findBlockByNumberEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTextOption QTextDocument::defaultTextOption();
  fn _ZNK13QTextDocument17defaultTextOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::findBlock(int pos);
  fn _ZNK13QTextDocument9findBlockEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
  fn _ZN13QTextDocument10setBaseUrlERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::redoAvailable(bool );
  fn _ZN13QTextDocument13redoAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextDocument::redo();
  fn _ZN13QTextDocument4redoEv(qthis: *mut c_void) ;
  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
  fn _ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
  fn _ZNK13QTextDocument21findBlockByLineNumberEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::undo();
  fn _ZN13QTextDocument4undoEv(qthis: *mut c_void) ;
  // proto:  double QTextDocument::textWidth();
  fn _ZNK13QTextDocument9textWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  const QMetaObject * QTextDocument::metaObject();
  fn _ZNK13QTextDocument10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QTextDocument::availableRedoSteps();
  fn _ZNK13QTextDocument18availableRedoStepsEv(qthis: *mut c_void) -> c_int;
  // proto:  QChar QTextDocument::characterAt(int pos);
  fn _ZNK13QTextDocument11characterAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
  fn _ZN13QTextDocument14setDefaultFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
  fn _ZNK13QTextDocument15objectForFormatERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTextDocument::isEmpty();
  fn _ZNK13QTextDocument7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextDocument::isUndoRedoEnabled();
  fn _ZNK13QTextDocument17isUndoRedoEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
  fn _ZN13QTextDocument14contentsChangeEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  void QTextDocument::FreeQTextDocument();
  fn _ZN13QTextDocumentD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextDocument::contentsChanged();
  fn _ZN13QTextDocument15contentsChangedEv(qthis: *mut c_void) ;
  // proto:  double QTextDocument::documentMargin();
  fn _ZNK13QTextDocument14documentMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
  fn _ZN13QTextDocument11setPageSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::setHtml(const QString & html);
  fn _ZN13QTextDocument7setHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextBlock QTextDocument::end();
  fn _ZNK13QTextDocument3endEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::maximumBlockCount();
  fn _ZNK13QTextDocument17maximumBlockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::setPlainText(const QString & text);
  fn _ZN13QTextDocument12setPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextDocument::clear();
  fn _ZN13QTextDocument5clearEv(qthis: *mut c_void) ;
  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
  fn _ZNK13QTextDocument8resourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::begin();
  fn _ZNK13QTextDocument5beginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
  fn _ZN13QTextDocument20setMaximumBlockCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextDocument::setModified(bool m);
  fn _ZN13QTextDocument11setModifiedEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QTextDocument)=1
pub struct QTextDocument {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextDocument {
  pub fn cursorPositionChanged<T: QTextDocument_cursorPositionChanged>(&mut self, value: T)  {
     value.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_cursorPositionChanged {
  fn cursorPositionChanged(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::cursorPositionChanged(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextDocument_cursorPositionChanged for (&'a  QTextCursor) {
  fn cursorPositionChanged(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDefaultStyleSheet<T: QTextDocument_setDefaultStyleSheet>(&mut self, value: T)  {
     value.setDefaultStyleSheet(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultStyleSheet {
  fn setDefaultStyleSheet(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QTextDocument_setDefaultStyleSheet for (&'a  QString) {
  fn setDefaultStyleSheet(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument20setDefaultStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn documentLayout<T: QTextDocument_documentLayout>(&mut self, value: T)  {
     value.documentLayout(self);
    // return 1;
  }
}

pub trait QTextDocument_documentLayout {
  fn documentLayout(self, rsthis: &mut QTextDocument) ;
}

// proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
impl<'a> /*trait*/ QTextDocument_documentLayout for () {
  fn documentLayout(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentLayoutEv()};
     unsafe {_ZNK13QTextDocument14documentLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isModified<T: QTextDocument_isModified>(&mut self, value: T) -> i8 {
    return value.isModified(self);
    // return 1;
  }
}

pub trait QTextDocument_isModified {
  fn isModified(self, rsthis: &mut QTextDocument) -> i8;
}

// proto:  bool QTextDocument::isModified();
impl<'a> /*trait*/ QTextDocument_isModified for () {
  fn isModified(self, rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10isModifiedEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10isModifiedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn revision<T: QTextDocument_revision>(&mut self, value: T) -> i32 {
    return value.revision(self);
    // return 1;
  }
}

pub trait QTextDocument_revision {
  fn revision(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::revision();
impl<'a> /*trait*/ QTextDocument_revision for () {
  fn revision(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8revisionEv()};
    let mut ret = unsafe {_ZNK13QTextDocument8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn pageSize<T: QTextDocument_pageSize>(&mut self, value: T) -> QSizeF {
    return value.pageSize(self);
    // return 1;
  }
}

pub trait QTextDocument_pageSize {
  fn pageSize(self, rsthis: &mut QTextDocument) -> QSizeF;
}

// proto:  QSizeF QTextDocument::pageSize();
impl<'a> /*trait*/ QTextDocument_pageSize for () {
  fn pageSize(self, rsthis: &mut QTextDocument) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8pageSizeEv()};
    let mut ret = unsafe {_ZNK13QTextDocument8pageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn redo<T: QTextDocument_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QTextDocument_redo {
  fn redo(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::redo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_redo for (&'a mut QTextCursor) {
  fn redo(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument4redoEP11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn lineCount<T: QTextDocument_lineCount>(&mut self, value: T) -> i32 {
    return value.lineCount(self);
    // return 1;
  }
}

pub trait QTextDocument_lineCount {
  fn lineCount(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::lineCount();
impl<'a> /*trait*/ QTextDocument_lineCount for () {
  fn lineCount(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lineCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn print<T: QTextDocument_print>(&mut self, value: T)  {
     value.print(self);
    // return 1;
  }
}

pub trait QTextDocument_print {
  fn print(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextDocument_print for (&'a mut QPagedPaintDevice) {
  fn print(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QTextDocument5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn docHandle<T: QTextDocument_docHandle>(&mut self, value: T)  {
     value.docHandle(self);
    // return 1;
  }
}

pub trait QTextDocument_docHandle {
  fn docHandle(self, rsthis: &mut QTextDocument) ;
}

// proto:  QTextDocumentPrivate * QTextDocument::docHandle();
impl<'a> /*trait*/ QTextDocument_docHandle for () {
  fn docHandle(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9docHandleEv()};
     unsafe {_ZNK13QTextDocument9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn toHtml<T: QTextDocument_toHtml>(&mut self, value: T) -> QString {
    return value.toHtml(self);
    // return 1;
  }
}

pub trait QTextDocument_toHtml {
  fn toHtml(self, rsthis: &mut QTextDocument) -> QString;
}

// proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocument_toHtml for (&'a  QByteArray) {
  fn toHtml(self, rsthis: &mut QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument6toHtmlERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn availableUndoSteps<T: QTextDocument_availableUndoSteps>(&mut self, value: T) -> i32 {
    return value.availableUndoSteps(self);
    // return 1;
  }
}

pub trait QTextDocument_availableUndoSteps {
  fn availableUndoSteps(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::availableUndoSteps();
impl<'a> /*trait*/ QTextDocument_availableUndoSteps for () {
  fn availableUndoSteps(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableUndoStepsEv()};
    let mut ret = unsafe {_ZNK13QTextDocument18availableUndoStepsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn undoAvailable<T: QTextDocument_undoAvailable>(&mut self, value: T)  {
     value.undoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_undoAvailable {
  fn undoAvailable(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::undoAvailable(bool );
impl<'a> /*trait*/ QTextDocument_undoAvailable for (i8) {
  fn undoAvailable(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument13undoAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTextDocument13undoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setUndoRedoEnabled<T: QTextDocument_setUndoRedoEnabled>(&mut self, value: T)  {
     value.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextDocument_setUndoRedoEnabled {
  fn setUndoRedoEnabled(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextDocument_setUndoRedoEnabled for (i8) {
  fn setUndoRedoEnabled(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument18setUndoRedoEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTextDocument18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn undo<T: QTextDocument_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QTextDocument_undo {
  fn undo(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::undo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_undo for (&'a mut QTextCursor) {
  fn undo(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument4undoEP11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn toPlainText<T: QTextDocument_toPlainText>(&mut self, value: T) -> QString {
    return value.toPlainText(self);
    // return 1;
  }
}

pub trait QTextDocument_toPlainText {
  fn toPlainText(self, rsthis: &mut QTextDocument) -> QString;
}

// proto:  QString QTextDocument::toPlainText();
impl<'a> /*trait*/ QTextDocument_toPlainText for () {
  fn toPlainText(self, rsthis: &mut QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11toPlainTextEv()};
    let mut ret = unsafe {_ZNK13QTextDocument11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn addResource<T: QTextDocument_addResource>(&mut self, value: T)  {
     value.addResource(self);
    // return 1;
  }
}

pub trait QTextDocument_addResource {
  fn addResource(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
impl<'a> /*trait*/ QTextDocument_addResource for (i32, &'a  QUrl, &'a  QVariant) {
  fn addResource(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn size<T: QTextDocument_size>(&mut self, value: T) -> QSizeF {
    return value.size(self);
    // return 1;
  }
}

pub trait QTextDocument_size {
  fn size(self, rsthis: &mut QTextDocument) -> QSizeF;
}

// proto:  QSizeF QTextDocument::size();
impl<'a> /*trait*/ QTextDocument_size for () {
  fn size(self, rsthis: &mut QTextDocument) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument4sizeEv()};
    let mut ret = unsafe {_ZNK13QTextDocument4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn object<T: QTextDocument_object>(&mut self, value: T) -> QTextObject {
    return value.object(self);
    // return 1;
  }
}

pub trait QTextDocument_object {
  fn object(self, rsthis: &mut QTextDocument) -> QTextObject;
}

// proto:  QTextObject * QTextDocument::object(int objectIndex);
impl<'a> /*trait*/ QTextDocument_object for (i32) {
  fn object(self, rsthis: &mut QTextDocument) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6objectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument6objectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn clone<T: QTextDocument_clone>(&mut self, value: T) -> QTextDocument {
    return value.clone(self);
    // return 1;
  }
}

pub trait QTextDocument_clone {
  fn clone(self, rsthis: &mut QTextDocument) -> QTextDocument;
}

// proto:  QTextDocument * QTextDocument::clone(QObject * parent);
impl<'a> /*trait*/ QTextDocument_clone for (&'a mut QObject) {
  fn clone(self, rsthis: &mut QTextDocument) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5cloneEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument5cloneEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn markContentsDirty<T: QTextDocument_markContentsDirty>(&mut self, value: T)  {
     value.markContentsDirty(self);
    // return 1;
  }
}

pub trait QTextDocument_markContentsDirty {
  fn markContentsDirty(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::markContentsDirty(int from, int length);
impl<'a> /*trait*/ QTextDocument_markContentsDirty for (i32, i32) {
  fn markContentsDirty(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17markContentsDirtyEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QTextDocument17markContentsDirtyEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
  pub fn modificationChanged<T: QTextDocument_modificationChanged>(&mut self, value: T)  {
     value.modificationChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_modificationChanged {
  fn modificationChanged(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::modificationChanged(bool m);
impl<'a> /*trait*/ QTextDocument_modificationChanged for (i8) {
  fn modificationChanged(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19modificationChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTextDocument19modificationChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn characterCount<T: QTextDocument_characterCount>(&mut self, value: T) -> i32 {
    return value.characterCount(self);
    // return 1;
  }
}

pub trait QTextDocument_characterCount {
  fn characterCount(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::characterCount();
impl<'a> /*trait*/ QTextDocument_characterCount for () {
  fn characterCount(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14characterCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument14characterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn rootFrame<T: QTextDocument_rootFrame>(&mut self, value: T) -> QTextFrame {
    return value.rootFrame(self);
    // return 1;
  }
}

pub trait QTextDocument_rootFrame {
  fn rootFrame(self, rsthis: &mut QTextDocument) -> QTextFrame;
}

// proto:  QTextFrame * QTextDocument::rootFrame();
impl<'a> /*trait*/ QTextDocument_rootFrame for () {
  fn rootFrame(self, rsthis: &mut QTextDocument) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9rootFrameEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9rootFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn firstBlock<T: QTextDocument_firstBlock>(&mut self, value: T) -> QTextBlock {
    return value.firstBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_firstBlock {
  fn firstBlock(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::firstBlock();
impl<'a> /*trait*/ QTextDocument_firstBlock for () {
  fn firstBlock(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10firstBlockEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10firstBlockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn blockCount<T: QTextDocument_blockCount>(&mut self, value: T) -> i32 {
    return value.blockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_blockCount {
  fn blockCount(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::blockCount();
impl<'a> /*trait*/ QTextDocument_blockCount for () {
  fn blockCount(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10blockCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10blockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn idealWidth<T: QTextDocument_idealWidth>(&mut self, value: T) -> f64 {
    return value.idealWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_idealWidth {
  fn idealWidth(self, rsthis: &mut QTextDocument) -> f64;
}

// proto:  double QTextDocument::idealWidth();
impl<'a> /*trait*/ QTextDocument_idealWidth for () {
  fn idealWidth(self, rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10idealWidthEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10idealWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn adjustSize<T: QTextDocument_adjustSize>(&mut self, value: T)  {
     value.adjustSize(self);
    // return 1;
  }
}

pub trait QTextDocument_adjustSize {
  fn adjustSize(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::adjustSize();
impl<'a> /*trait*/ QTextDocument_adjustSize for () {
  fn adjustSize(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10adjustSizeEv()};
     unsafe {_ZN13QTextDocument10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isRedoAvailable<T: QTextDocument_isRedoAvailable>(&mut self, value: T) -> i8 {
    return value.isRedoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_isRedoAvailable {
  fn isRedoAvailable(self, rsthis: &mut QTextDocument) -> i8;
}

// proto:  bool QTextDocument::isRedoAvailable();
impl<'a> /*trait*/ QTextDocument_isRedoAvailable for () {
  fn isRedoAvailable(self, rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isRedoAvailableEv()};
    let mut ret = unsafe {_ZNK13QTextDocument15isRedoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn allFormats<T: QTextDocument_allFormats>(&mut self, value: T)  {
     value.allFormats(self);
    // return 1;
  }
}

pub trait QTextDocument_allFormats {
  fn allFormats(self, rsthis: &mut QTextDocument) ;
}

// proto:  QVector<QTextFormat> QTextDocument::allFormats();
impl<'a> /*trait*/ QTextDocument_allFormats for () {
  fn allFormats(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10allFormatsEv()};
     unsafe {_ZNK13QTextDocument10allFormatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn blockCountChanged<T: QTextDocument_blockCountChanged>(&mut self, value: T)  {
     value.blockCountChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_blockCountChanged {
  fn blockCountChanged(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::blockCountChanged(int newBlockCount);
impl<'a> /*trait*/ QTextDocument_blockCountChanged for (i32) {
  fn blockCountChanged(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17blockCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QTextDocument17blockCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn defaultStyleSheet<T: QTextDocument_defaultStyleSheet>(&mut self, value: T) -> QString {
    return value.defaultStyleSheet(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultStyleSheet {
  fn defaultStyleSheet(self, rsthis: &mut QTextDocument) -> QString;
}

// proto:  QString QTextDocument::defaultStyleSheet();
impl<'a> /*trait*/ QTextDocument_defaultStyleSheet for () {
  fn defaultStyleSheet(self, rsthis: &mut QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultStyleSheetEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17defaultStyleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn lastBlock<T: QTextDocument_lastBlock>(&mut self, value: T) -> QTextBlock {
    return value.lastBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_lastBlock {
  fn lastBlock(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::lastBlock();
impl<'a> /*trait*/ QTextDocument_lastBlock for () {
  fn lastBlock(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lastBlockEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9lastBlockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextDocument::NewQTextDocument(const QString & text, QObject * parent);
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (&'a  QString, &'a mut QObject) {
  fn NewQTextDocument(self) -> QTextDocument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QTextDocumentC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QTextDocument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn useDesignMetrics<T: QTextDocument_useDesignMetrics>(&mut self, value: T) -> i8 {
    return value.useDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextDocument_useDesignMetrics {
  fn useDesignMetrics(self, rsthis: &mut QTextDocument) -> i8;
}

// proto:  bool QTextDocument::useDesignMetrics();
impl<'a> /*trait*/ QTextDocument_useDesignMetrics for () {
  fn useDesignMetrics(self, rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument16useDesignMetricsEv()};
    let mut ret = unsafe {_ZNK13QTextDocument16useDesignMetricsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn documentLayoutChanged<T: QTextDocument_documentLayoutChanged>(&mut self, value: T)  {
     value.documentLayoutChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_documentLayoutChanged {
  fn documentLayoutChanged(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::documentLayoutChanged();
impl<'a> /*trait*/ QTextDocument_documentLayoutChanged for () {
  fn documentLayoutChanged(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument21documentLayoutChangedEv()};
     unsafe {_ZN13QTextDocument21documentLayoutChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn pageCount<T: QTextDocument_pageCount>(&mut self, value: T) -> i32 {
    return value.pageCount(self);
    // return 1;
  }
}

pub trait QTextDocument_pageCount {
  fn pageCount(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::pageCount();
impl<'a> /*trait*/ QTextDocument_pageCount for () {
  fn pageCount(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9pageCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9pageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn baseUrlChanged<T: QTextDocument_baseUrlChanged>(&mut self, value: T)  {
     value.baseUrlChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_baseUrlChanged {
  fn baseUrlChanged(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::baseUrlChanged(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_baseUrlChanged for (&'a  QUrl) {
  fn baseUrlChanged(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14baseUrlChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument14baseUrlChangedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setTextWidth<T: QTextDocument_setTextWidth>(&mut self, value: T)  {
     value.setTextWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_setTextWidth {
  fn setTextWidth(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setTextWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setTextWidth for (f64) {
  fn setTextWidth(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QTextDocument12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDocumentMargin<T: QTextDocument_setDocumentMargin>(&mut self, value: T)  {
     value.setDocumentMargin(self);
    // return 1;
  }
}

pub trait QTextDocument_setDocumentMargin {
  fn setDocumentMargin(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setDocumentMargin(qreal margin);
impl<'a> /*trait*/ QTextDocument_setDocumentMargin for (f64) {
  fn setDocumentMargin(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17setDocumentMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QTextDocument17setDocumentMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isUndoAvailable<T: QTextDocument_isUndoAvailable>(&mut self, value: T) -> i8 {
    return value.isUndoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_isUndoAvailable {
  fn isUndoAvailable(self, rsthis: &mut QTextDocument) -> i8;
}

// proto:  bool QTextDocument::isUndoAvailable();
impl<'a> /*trait*/ QTextDocument_isUndoAvailable for () {
  fn isUndoAvailable(self, rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isUndoAvailableEv()};
    let mut ret = unsafe {_ZNK13QTextDocument15isUndoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn indentWidth<T: QTextDocument_indentWidth>(&mut self, value: T) -> f64 {
    return value.indentWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_indentWidth {
  fn indentWidth(self, rsthis: &mut QTextDocument) -> f64;
}

// proto:  double QTextDocument::indentWidth();
impl<'a> /*trait*/ QTextDocument_indentWidth for () {
  fn indentWidth(self, rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11indentWidthEv()};
    let mut ret = unsafe {_ZNK13QTextDocument11indentWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn undoCommandAdded<T: QTextDocument_undoCommandAdded>(&mut self, value: T)  {
     value.undoCommandAdded(self);
    // return 1;
  }
}

pub trait QTextDocument_undoCommandAdded {
  fn undoCommandAdded(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::undoCommandAdded();
impl<'a> /*trait*/ QTextDocument_undoCommandAdded for () {
  fn undoCommandAdded(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument16undoCommandAddedEv()};
     unsafe {_ZN13QTextDocument16undoCommandAddedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setUseDesignMetrics<T: QTextDocument_setUseDesignMetrics>(&mut self, value: T)  {
     value.setUseDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextDocument_setUseDesignMetrics {
  fn setUseDesignMetrics(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextDocument_setUseDesignMetrics for (i8) {
  fn setUseDesignMetrics(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19setUseDesignMetricsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTextDocument19setUseDesignMetricsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setIndentWidth<T: QTextDocument_setIndentWidth>(&mut self, value: T)  {
     value.setIndentWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_setIndentWidth {
  fn setIndentWidth(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setIndentWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setIndentWidth for (f64) {
  fn setIndentWidth(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setIndentWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QTextDocument14setIndentWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn baseUrl<T: QTextDocument_baseUrl>(&mut self, value: T) -> QUrl {
    return value.baseUrl(self);
    // return 1;
  }
}

pub trait QTextDocument_baseUrl {
  fn baseUrl(self, rsthis: &mut QTextDocument) -> QUrl;
}

// proto:  QUrl QTextDocument::baseUrl();
impl<'a> /*trait*/ QTextDocument_baseUrl for () {
  fn baseUrl(self, rsthis: &mut QTextDocument) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7baseUrlEv()};
    let mut ret = unsafe {_ZNK13QTextDocument7baseUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn frameAt<T: QTextDocument_frameAt>(&mut self, value: T) -> QTextFrame {
    return value.frameAt(self);
    // return 1;
  }
}

pub trait QTextDocument_frameAt {
  fn frameAt(self, rsthis: &mut QTextDocument) -> QTextFrame;
}

// proto:  QTextFrame * QTextDocument::frameAt(int pos);
impl<'a> /*trait*/ QTextDocument_frameAt for (i32) {
  fn frameAt(self, rsthis: &mut QTextDocument) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7frameAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument7frameAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextDocument::NewQTextDocument(const QTextDocument & );
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (&'a  QTextDocument) {
  fn NewQTextDocument(self) -> QTextDocument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTextDocumentC1ERKS_(qthis, arg0)};
    let rsthis = QTextDocument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDefaultTextOption<T: QTextDocument_setDefaultTextOption>(&mut self, value: T)  {
     value.setDefaultTextOption(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultTextOption {
  fn setDefaultTextOption(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextDocument_setDefaultTextOption for (&'a  QTextOption) {
  fn setDefaultTextOption(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn defaultFont<T: QTextDocument_defaultFont>(&mut self, value: T) -> QFont {
    return value.defaultFont(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultFont {
  fn defaultFont(self, rsthis: &mut QTextDocument) -> QFont;
}

// proto:  QFont QTextDocument::defaultFont();
impl<'a> /*trait*/ QTextDocument_defaultFont for () {
  fn defaultFont(self, rsthis: &mut QTextDocument) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11defaultFontEv()};
    let mut ret = unsafe {_ZNK13QTextDocument11defaultFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn findBlockByNumber<T: QTextDocument_findBlockByNumber>(&mut self, value: T) -> QTextBlock {
    return value.findBlockByNumber(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlockByNumber {
  fn findBlockByNumber(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByNumber for (i32) {
  fn findBlockByNumber(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17findBlockByNumberEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument17findBlockByNumberEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn defaultTextOption<T: QTextDocument_defaultTextOption>(&mut self, value: T) -> QTextOption {
    return value.defaultTextOption(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultTextOption {
  fn defaultTextOption(self, rsthis: &mut QTextDocument) -> QTextOption;
}

// proto:  QTextOption QTextDocument::defaultTextOption();
impl<'a> /*trait*/ QTextDocument_defaultTextOption for () {
  fn defaultTextOption(self, rsthis: &mut QTextDocument) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultTextOptionEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17defaultTextOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn findBlock<T: QTextDocument_findBlock>(&mut self, value: T) -> QTextBlock {
    return value.findBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlock {
  fn findBlock(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::findBlock(int pos);
impl<'a> /*trait*/ QTextDocument_findBlock for (i32) {
  fn findBlock(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9findBlockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument9findBlockEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setBaseUrl<T: QTextDocument_setBaseUrl>(&mut self, value: T)  {
     value.setBaseUrl(self);
    // return 1;
  }
}

pub trait QTextDocument_setBaseUrl {
  fn setBaseUrl(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setBaseUrl(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_setBaseUrl for (&'a  QUrl) {
  fn setBaseUrl(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10setBaseUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument10setBaseUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn redoAvailable<T: QTextDocument_redoAvailable>(&mut self, value: T)  {
     value.redoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_redoAvailable {
  fn redoAvailable(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::redoAvailable(bool );
impl<'a> /*trait*/ QTextDocument_redoAvailable for (i8) {
  fn redoAvailable(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument13redoAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTextDocument13redoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextDocument::redo();
impl<'a> /*trait*/ QTextDocument_redo for () {
  fn redo(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEv()};
     unsafe {_ZN13QTextDocument4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn drawContents<T: QTextDocument_drawContents>(&mut self, value: T)  {
     value.drawContents(self);
    // return 1;
  }
}

pub trait QTextDocument_drawContents {
  fn drawContents(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
impl<'a> /*trait*/ QTextDocument_drawContents for (&'a mut QPainter, &'a  QRectF) {
  fn drawContents(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn findBlockByLineNumber<T: QTextDocument_findBlockByLineNumber>(&mut self, value: T) -> QTextBlock {
    return value.findBlockByLineNumber(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlockByLineNumber {
  fn findBlockByLineNumber(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByLineNumber for (i32) {
  fn findBlockByLineNumber(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument21findBlockByLineNumberEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument21findBlockByLineNumberEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextDocument::undo();
impl<'a> /*trait*/ QTextDocument_undo for () {
  fn undo(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEv()};
     unsafe {_ZN13QTextDocument4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn textWidth<T: QTextDocument_textWidth>(&mut self, value: T) -> f64 {
    return value.textWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_textWidth {
  fn textWidth(self, rsthis: &mut QTextDocument) -> f64;
}

// proto:  double QTextDocument::textWidth();
impl<'a> /*trait*/ QTextDocument_textWidth for () {
  fn textWidth(self, rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9textWidthEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn metaObject<T: QTextDocument_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTextDocument_metaObject {
  fn metaObject(self, rsthis: &mut QTextDocument) ;
}

// proto:  const QMetaObject * QTextDocument::metaObject();
impl<'a> /*trait*/ QTextDocument_metaObject for () {
  fn metaObject(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10metaObjectEv()};
     unsafe {_ZNK13QTextDocument10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn availableRedoSteps<T: QTextDocument_availableRedoSteps>(&mut self, value: T) -> i32 {
    return value.availableRedoSteps(self);
    // return 1;
  }
}

pub trait QTextDocument_availableRedoSteps {
  fn availableRedoSteps(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::availableRedoSteps();
impl<'a> /*trait*/ QTextDocument_availableRedoSteps for () {
  fn availableRedoSteps(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableRedoStepsEv()};
    let mut ret = unsafe {_ZNK13QTextDocument18availableRedoStepsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn characterAt<T: QTextDocument_characterAt>(&mut self, value: T) -> QChar {
    return value.characterAt(self);
    // return 1;
  }
}

pub trait QTextDocument_characterAt {
  fn characterAt(self, rsthis: &mut QTextDocument) -> QChar;
}

// proto:  QChar QTextDocument::characterAt(int pos);
impl<'a> /*trait*/ QTextDocument_characterAt for (i32) {
  fn characterAt(self, rsthis: &mut QTextDocument) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11characterAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument11characterAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setDefaultFont<T: QTextDocument_setDefaultFont>(&mut self, value: T)  {
     value.setDefaultFont(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultFont {
  fn setDefaultFont(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setDefaultFont(const QFont & font);
impl<'a> /*trait*/ QTextDocument_setDefaultFont for (&'a  QFont) {
  fn setDefaultFont(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setDefaultFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument14setDefaultFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn objectForFormat<T: QTextDocument_objectForFormat>(&mut self, value: T) -> QTextObject {
    return value.objectForFormat(self);
    // return 1;
  }
}

pub trait QTextDocument_objectForFormat {
  fn objectForFormat(self, rsthis: &mut QTextDocument) -> QTextObject;
}

// proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
impl<'a> /*trait*/ QTextDocument_objectForFormat for (&'a  QTextFormat) {
  fn objectForFormat(self, rsthis: &mut QTextDocument) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15objectForFormatERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument15objectForFormatERK11QTextFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isEmpty<T: QTextDocument_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QTextDocument_isEmpty {
  fn isEmpty(self, rsthis: &mut QTextDocument) -> i8;
}

// proto:  bool QTextDocument::isEmpty();
impl<'a> /*trait*/ QTextDocument_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7isEmptyEv()};
    let mut ret = unsafe {_ZNK13QTextDocument7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn isUndoRedoEnabled<T: QTextDocument_isUndoRedoEnabled>(&mut self, value: T) -> i8 {
    return value.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextDocument_isUndoRedoEnabled {
  fn isUndoRedoEnabled(self, rsthis: &mut QTextDocument) -> i8;
}

// proto:  bool QTextDocument::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextDocument_isUndoRedoEnabled for () {
  fn isUndoRedoEnabled(self, rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17isUndoRedoEnabledEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn contentsChange<T: QTextDocument_contentsChange>(&mut self, value: T)  {
     value.contentsChange(self);
    // return 1;
  }
}

pub trait QTextDocument_contentsChange {
  fn contentsChange(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
impl<'a> /*trait*/ QTextDocument_contentsChange for (i32, i32, i32) {
  fn contentsChange(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14contentsChangeEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QTextDocument14contentsChangeEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn FreeQTextDocument<T: QTextDocument_FreeQTextDocument>(&mut self, value: T)  {
     value.FreeQTextDocument(self);
    // return 1;
  }
}

pub trait QTextDocument_FreeQTextDocument {
  fn FreeQTextDocument(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::FreeQTextDocument();
impl<'a> /*trait*/ QTextDocument_FreeQTextDocument for () {
  fn FreeQTextDocument(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentD0Ev()};
     unsafe {_ZN13QTextDocumentD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn contentsChanged<T: QTextDocument_contentsChanged>(&mut self, value: T)  {
     value.contentsChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_contentsChanged {
  fn contentsChanged(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::contentsChanged();
impl<'a> /*trait*/ QTextDocument_contentsChanged for () {
  fn contentsChanged(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument15contentsChangedEv()};
     unsafe {_ZN13QTextDocument15contentsChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn documentMargin<T: QTextDocument_documentMargin>(&mut self, value: T) -> f64 {
    return value.documentMargin(self);
    // return 1;
  }
}

pub trait QTextDocument_documentMargin {
  fn documentMargin(self, rsthis: &mut QTextDocument) -> f64;
}

// proto:  double QTextDocument::documentMargin();
impl<'a> /*trait*/ QTextDocument_documentMargin for () {
  fn documentMargin(self, rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentMarginEv()};
    let mut ret = unsafe {_ZNK13QTextDocument14documentMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setPageSize<T: QTextDocument_setPageSize>(&mut self, value: T)  {
     value.setPageSize(self);
    // return 1;
  }
}

pub trait QTextDocument_setPageSize {
  fn setPageSize(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setPageSize(const QSizeF & size);
impl<'a> /*trait*/ QTextDocument_setPageSize for (&'a  QSizeF) {
  fn setPageSize(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setPageSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument11setPageSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setHtml<T: QTextDocument_setHtml>(&mut self, value: T)  {
     value.setHtml(self);
    // return 1;
  }
}

pub trait QTextDocument_setHtml {
  fn setHtml(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setHtml(const QString & html);
impl<'a> /*trait*/ QTextDocument_setHtml for (&'a  QString) {
  fn setHtml(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn end<T: QTextDocument_end>(&mut self, value: T) -> QTextBlock {
    return value.end(self);
    // return 1;
  }
}

pub trait QTextDocument_end {
  fn end(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::end();
impl<'a> /*trait*/ QTextDocument_end for () {
  fn end(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument3endEv()};
    let mut ret = unsafe {_ZNK13QTextDocument3endEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn maximumBlockCount<T: QTextDocument_maximumBlockCount>(&mut self, value: T) -> i32 {
    return value.maximumBlockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_maximumBlockCount {
  fn maximumBlockCount(self, rsthis: &mut QTextDocument) -> i32;
}

// proto:  int QTextDocument::maximumBlockCount();
impl<'a> /*trait*/ QTextDocument_maximumBlockCount for () {
  fn maximumBlockCount(self, rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17maximumBlockCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17maximumBlockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setPlainText<T: QTextDocument_setPlainText>(&mut self, value: T)  {
     value.setPlainText(self);
    // return 1;
  }
}

pub trait QTextDocument_setPlainText {
  fn setPlainText(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextDocument_setPlainText for (&'a  QString) {
  fn setPlainText(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn clear<T: QTextDocument_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QTextDocument_clear {
  fn clear(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::clear();
impl<'a> /*trait*/ QTextDocument_clear for () {
  fn clear(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument5clearEv()};
     unsafe {_ZN13QTextDocument5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn resource<T: QTextDocument_resource>(&mut self, value: T) -> QVariant {
    return value.resource(self);
    // return 1;
  }
}

pub trait QTextDocument_resource {
  fn resource(self, rsthis: &mut QTextDocument) -> QVariant;
}

// proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextDocument_resource for (i32, &'a  QUrl) {
  fn resource(self, rsthis: &mut QTextDocument) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8resourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument8resourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn begin<T: QTextDocument_begin>(&mut self, value: T) -> QTextBlock {
    return value.begin(self);
    // return 1;
  }
}

pub trait QTextDocument_begin {
  fn begin(self, rsthis: &mut QTextDocument) -> QTextBlock;
}

// proto:  QTextBlock QTextDocument::begin();
impl<'a> /*trait*/ QTextDocument_begin for () {
  fn begin(self, rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5beginEv()};
    let mut ret = unsafe {_ZNK13QTextDocument5beginEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setMaximumBlockCount<T: QTextDocument_setMaximumBlockCount>(&mut self, value: T)  {
     value.setMaximumBlockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_setMaximumBlockCount {
  fn setMaximumBlockCount(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QTextDocument_setMaximumBlockCount for (i32) {
  fn setMaximumBlockCount(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QTextDocument20setMaximumBlockCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextDocument {
  pub fn setModified<T: QTextDocument_setModified>(&mut self, value: T)  {
     value.setModified(self);
    // return 1;
  }
}

pub trait QTextDocument_setModified {
  fn setModified(self, rsthis: &mut QTextDocument) ;
}

// proto:  void QTextDocument::setModified(bool m);
impl<'a> /*trait*/ QTextDocument_setModified for (i8) {
  fn setModified(self, rsthis: &mut QTextDocument)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setModifiedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QTextDocument11setModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

