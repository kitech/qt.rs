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
use super::qobject::QObject;
use super::qurl::QUrl;
use super::qvariant::QVariant;
use super::qtextobject::QTextObject;
use super::qtextframe::QTextFrame;
use super::qtextblock::QTextBlock;
use super::qregexp::QRegExp;
use super::qregularexpression::QRegularExpression;
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
  fn _ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
  fn _ZN13QTextDocument20setDefaultStyleSheetERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
  fn _ZNK13QTextDocument14documentLayoutEv(qthis: *mut c_void);
  // proto:  bool QTextDocument::isModified();
  fn _ZNK13QTextDocument10isModifiedEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTextDocument::revision();
  fn _ZNK13QTextDocument8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  QSizeF QTextDocument::pageSize();
  fn _ZNK13QTextDocument8pageSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::redo(QTextCursor * cursor);
  fn _ZN13QTextDocument4redoEP11QTextCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextDocument::lineCount();
  fn _ZNK13QTextDocument9lineCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
  fn _ZNK13QTextDocument5printEP17QPagedPaintDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
  fn _ZNK13QTextDocument9docHandleEv(qthis: *mut c_void);
  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
  fn _ZNK13QTextDocument6toHtmlERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::availableUndoSteps();
  fn _ZNK13QTextDocument18availableUndoStepsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::undoAvailable(bool );
  fn _ZN13QTextDocument13undoAvailableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
  fn _ZN13QTextDocument18setUndoRedoEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextDocument::undo(QTextCursor * cursor);
  fn _ZN13QTextDocument4undoEP11QTextCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTextDocument::toPlainText();
  fn _ZNK13QTextDocument11toPlainTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
  fn _ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QSizeF QTextDocument::size();
  fn _ZNK13QTextDocument4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextObject * QTextDocument::object(int objectIndex);
  fn _ZNK13QTextDocument6objectEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
  fn _ZNK13QTextDocument5cloneEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::markContentsDirty(int from, int length);
  fn _ZN13QTextDocument17markContentsDirtyEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QTextDocument::QTextDocument(QObject * parent);
  fn _ZN13QTextDocumentC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::modificationChanged(bool m);
  fn _ZN13QTextDocument19modificationChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QTextDocument::characterCount();
  fn _ZNK13QTextDocument14characterCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextFrame * QTextDocument::rootFrame();
  fn _ZNK13QTextDocument9rootFrameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::firstBlock();
  fn _ZNK13QTextDocument10firstBlockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::blockCount();
  fn _ZNK13QTextDocument10blockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  qreal QTextDocument::idealWidth();
  fn _ZNK13QTextDocument10idealWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextDocument::adjustSize();
  fn _ZN13QTextDocument10adjustSizeEv(qthis: *mut c_void);
  // proto:  bool QTextDocument::isRedoAvailable();
  fn _ZNK13QTextDocument15isRedoAvailableEv(qthis: *mut c_void) -> c_char;
  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
  fn _ZNK13QTextDocument10allFormatsEv(qthis: *mut c_void);
  // proto:  void QTextDocument::blockCountChanged(int newBlockCount);
  fn _ZN13QTextDocument17blockCountChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QTextDocument::defaultStyleSheet();
  fn _ZNK13QTextDocument17defaultStyleSheetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::lastBlock();
  fn _ZNK13QTextDocument9lastBlockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::QTextDocument(const QString & text, QObject * parent);
  fn _ZN13QTextDocumentC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QTextDocument::useDesignMetrics();
  fn _ZNK13QTextDocument16useDesignMetricsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextDocument::documentLayoutChanged();
  fn _ZN13QTextDocument21documentLayoutChangedEv(qthis: *mut c_void);
  // proto:  int QTextDocument::pageCount();
  fn _ZNK13QTextDocument9pageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::baseUrlChanged(const QUrl & url);
  fn _ZN13QTextDocument14baseUrlChangedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::setTextWidth(qreal width);
  fn _ZN13QTextDocument12setTextWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
  fn _ZN13QTextDocument17setDocumentMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QTextDocument::isUndoAvailable();
  fn _ZNK13QTextDocument15isUndoAvailableEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QTextDocument::indentWidth();
  fn _ZNK13QTextDocument11indentWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextDocument::undoCommandAdded();
  fn _ZN13QTextDocument16undoCommandAddedEv(qthis: *mut c_void);
  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
  fn _ZN13QTextDocument19setUseDesignMetricsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextDocument::setIndentWidth(qreal width);
  fn _ZN13QTextDocument14setIndentWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QUrl QTextDocument::baseUrl();
  fn _ZNK13QTextDocument7baseUrlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
  fn _ZNK13QTextDocument7frameAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::QTextDocument(const QTextDocument & );
  fn _ZN13QTextDocumentC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
  fn _ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QFont QTextDocument::defaultFont();
  fn _ZNK13QTextDocument11defaultFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
  fn _ZNK13QTextDocument17findBlockByNumberEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTextOption QTextDocument::defaultTextOption();
  fn _ZNK13QTextDocument17defaultTextOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::findBlock(int pos);
  fn _ZNK13QTextDocument9findBlockEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
  fn _ZN13QTextDocument10setBaseUrlERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::redoAvailable(bool );
  fn _ZN13QTextDocument13redoAvailableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextDocument::redo();
  fn _ZN13QTextDocument4redoEv(qthis: *mut c_void);
  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
  fn _ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
  fn _ZNK13QTextDocument21findBlockByLineNumberEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::undo();
  fn _ZN13QTextDocument4undoEv(qthis: *mut c_void);
  // proto:  qreal QTextDocument::textWidth();
  fn _ZNK13QTextDocument9textWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  const QMetaObject * QTextDocument::metaObject();
  fn _ZNK13QTextDocument10metaObjectEv(qthis: *mut c_void);
  // proto:  int QTextDocument::availableRedoSteps();
  fn _ZNK13QTextDocument18availableRedoStepsEv(qthis: *mut c_void) -> c_int;
  // proto:  QChar QTextDocument::characterAt(int pos);
  fn _ZNK13QTextDocument11characterAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
  fn _ZN13QTextDocument14setDefaultFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
  fn _ZNK13QTextDocument15objectForFormatERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTextDocument::isEmpty();
  fn _ZNK13QTextDocument7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTextDocument::isUndoRedoEnabled();
  fn _ZNK13QTextDocument17isUndoRedoEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
  fn _ZN13QTextDocument14contentsChangeEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QTextDocument::~QTextDocument();
  fn _ZN13QTextDocumentD0Ev(qthis: *mut c_void);
  // proto:  void QTextDocument::contentsChanged();
  fn _ZN13QTextDocument15contentsChangedEv(qthis: *mut c_void);
  // proto:  qreal QTextDocument::documentMargin();
  fn _ZNK13QTextDocument14documentMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
  fn _ZN13QTextDocument11setPageSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::setHtml(const QString & html);
  fn _ZN13QTextDocument7setHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextBlock QTextDocument::end();
  fn _ZNK13QTextDocument3endEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::maximumBlockCount();
  fn _ZNK13QTextDocument17maximumBlockCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextDocument::setPlainText(const QString & text);
  fn _ZN13QTextDocument12setPlainTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDocument::clear();
  fn _ZN13QTextDocument5clearEv(qthis: *mut c_void);
  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
  fn _ZNK13QTextDocument8resourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::begin();
  fn _ZNK13QTextDocument5beginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
  fn _ZN13QTextDocument20setMaximumBlockCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextDocument::setModified(bool m);
  fn _ZN13QTextDocument11setModifiedEb(qthis: *mut c_void, arg0: c_char);
}

// body block begin
// class sizeof(QTextDocument)=1
pub struct QTextDocument {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextDocument::cursorPositionChanged(const QTextCursor & cursor);
impl /*struct*/ QTextDocument {
  pub fn cursorPositionChanged<RetType, T: QTextDocument_cursorPositionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_cursorPositionChanged<RetType> {
  fn cursorPositionChanged(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::cursorPositionChanged(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextDocument_cursorPositionChanged<()> for (QTextCursor) {
  fn cursorPositionChanged(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
impl /*struct*/ QTextDocument {
  pub fn setDefaultStyleSheet<RetType, T: QTextDocument_setDefaultStyleSheet<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultStyleSheet(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultStyleSheet<RetType> {
  fn setDefaultStyleSheet(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QTextDocument_setDefaultStyleSheet<()> for (QString) {
  fn setDefaultStyleSheet(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument20setDefaultStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
impl /*struct*/ QTextDocument {
  pub fn documentLayout<RetType, T: QTextDocument_documentLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentLayout(self);
    // return 1;
  }
}

pub trait QTextDocument_documentLayout<RetType> {
  fn documentLayout(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
impl<'a> /*trait*/ QTextDocument_documentLayout<()> for () {
  fn documentLayout(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentLayoutEv()};
     unsafe {_ZNK13QTextDocument14documentLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextDocument::isModified();
impl /*struct*/ QTextDocument {
  pub fn isModified<RetType, T: QTextDocument_isModified<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isModified(self);
    // return 1;
  }
}

pub trait QTextDocument_isModified<RetType> {
  fn isModified(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isModified();
impl<'a> /*trait*/ QTextDocument_isModified<i8> for () {
  fn isModified(self , rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10isModifiedEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10isModifiedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextDocument::revision();
impl /*struct*/ QTextDocument {
  pub fn revision<RetType, T: QTextDocument_revision<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QTextDocument_revision<RetType> {
  fn revision(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::revision();
impl<'a> /*trait*/ QTextDocument_revision<i32> for () {
  fn revision(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8revisionEv()};
    let mut ret = unsafe {_ZNK13QTextDocument8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSizeF QTextDocument::pageSize();
impl /*struct*/ QTextDocument {
  pub fn pageSize<RetType, T: QTextDocument_pageSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageSize(self);
    // return 1;
  }
}

pub trait QTextDocument_pageSize<RetType> {
  fn pageSize(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QSizeF QTextDocument::pageSize();
impl<'a> /*trait*/ QTextDocument_pageSize<QSizeF> for () {
  fn pageSize(self , rsthis: &mut QTextDocument) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8pageSizeEv()};
    let mut ret = unsafe {_ZNK13QTextDocument8pageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::redo(QTextCursor * cursor);
impl /*struct*/ QTextDocument {
  pub fn redo<RetType, T: QTextDocument_redo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QTextDocument_redo<RetType> {
  fn redo(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::redo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_redo<()> for (QTextCursor) {
  fn redo(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument4redoEP11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextDocument::lineCount();
impl /*struct*/ QTextDocument {
  pub fn lineCount<RetType, T: QTextDocument_lineCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineCount(self);
    // return 1;
  }
}

pub trait QTextDocument_lineCount<RetType> {
  fn lineCount(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::lineCount();
impl<'a> /*trait*/ QTextDocument_lineCount<i32> for () {
  fn lineCount(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lineCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
impl /*struct*/ QTextDocument {
  pub fn print<RetType, T: QTextDocument_print<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.print(self);
    // return 1;
  }
}

pub trait QTextDocument_print<RetType> {
  fn print(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextDocument_print<()> for (QPagedPaintDevice) {
  fn print(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QTextDocument5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
impl /*struct*/ QTextDocument {
  pub fn docHandle<RetType, T: QTextDocument_docHandle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.docHandle(self);
    // return 1;
  }
}

pub trait QTextDocument_docHandle<RetType> {
  fn docHandle(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
impl<'a> /*trait*/ QTextDocument_docHandle<()> for () {
  fn docHandle(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9docHandleEv()};
     unsafe {_ZNK13QTextDocument9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
impl /*struct*/ QTextDocument {
  pub fn toHtml<RetType, T: QTextDocument_toHtml<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toHtml(self);
    // return 1;
  }
}

pub trait QTextDocument_toHtml<RetType> {
  fn toHtml(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocument_toHtml<QString> for (QByteArray) {
  fn toHtml(self , rsthis: &mut QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument6toHtmlERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::availableUndoSteps();
impl /*struct*/ QTextDocument {
  pub fn availableUndoSteps<RetType, T: QTextDocument_availableUndoSteps<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.availableUndoSteps(self);
    // return 1;
  }
}

pub trait QTextDocument_availableUndoSteps<RetType> {
  fn availableUndoSteps(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::availableUndoSteps();
impl<'a> /*trait*/ QTextDocument_availableUndoSteps<i32> for () {
  fn availableUndoSteps(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableUndoStepsEv()};
    let mut ret = unsafe {_ZNK13QTextDocument18availableUndoStepsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextDocument::undoAvailable(bool );
impl /*struct*/ QTextDocument {
  pub fn undoAvailable<RetType, T: QTextDocument_undoAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.undoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_undoAvailable<RetType> {
  fn undoAvailable(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::undoAvailable(bool );
impl<'a> /*trait*/ QTextDocument_undoAvailable<()> for (i8) {
  fn undoAvailable(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument13undoAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTextDocument13undoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
impl /*struct*/ QTextDocument {
  pub fn setUndoRedoEnabled<RetType, T: QTextDocument_setUndoRedoEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextDocument_setUndoRedoEnabled<RetType> {
  fn setUndoRedoEnabled(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextDocument_setUndoRedoEnabled<()> for (i8) {
  fn setUndoRedoEnabled(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument18setUndoRedoEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTextDocument18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::undo(QTextCursor * cursor);
impl /*struct*/ QTextDocument {
  pub fn undo<RetType, T: QTextDocument_undo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QTextDocument_undo<RetType> {
  fn undo(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::undo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_undo<()> for (QTextCursor) {
  fn undo(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument4undoEP11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextDocument::toPlainText();
impl /*struct*/ QTextDocument {
  pub fn toPlainText<RetType, T: QTextDocument_toPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QTextDocument_toPlainText<RetType> {
  fn toPlainText(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QString QTextDocument::toPlainText();
impl<'a> /*trait*/ QTextDocument_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: &mut QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11toPlainTextEv()};
    let mut ret = unsafe {_ZNK13QTextDocument11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
impl /*struct*/ QTextDocument {
  pub fn addResource<RetType, T: QTextDocument_addResource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addResource(self);
    // return 1;
  }
}

pub trait QTextDocument_addResource<RetType> {
  fn addResource(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
impl<'a> /*trait*/ QTextDocument_addResource<()> for (i32, QUrl, QVariant) {
  fn addResource(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QSizeF QTextDocument::size();
impl /*struct*/ QTextDocument {
  pub fn size<RetType, T: QTextDocument_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QTextDocument_size<RetType> {
  fn size(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QSizeF QTextDocument::size();
impl<'a> /*trait*/ QTextDocument_size<QSizeF> for () {
  fn size(self , rsthis: &mut QTextDocument) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument4sizeEv()};
    let mut ret = unsafe {_ZNK13QTextDocument4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextObject * QTextDocument::object(int objectIndex);
impl /*struct*/ QTextDocument {
  pub fn object<RetType, T: QTextDocument_object<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QTextDocument_object<RetType> {
  fn object(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextObject * QTextDocument::object(int objectIndex);
impl<'a> /*trait*/ QTextDocument_object<QTextObject> for (i32) {
  fn object(self , rsthis: &mut QTextDocument) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6objectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument6objectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
impl /*struct*/ QTextDocument {
  pub fn clone<RetType, T: QTextDocument_clone<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QTextDocument_clone<RetType> {
  fn clone(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
impl<'a> /*trait*/ QTextDocument_clone<QTextDocument> for (QObject) {
  fn clone(self , rsthis: &mut QTextDocument) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5cloneEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument5cloneEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::markContentsDirty(int from, int length);
impl /*struct*/ QTextDocument {
  pub fn markContentsDirty<RetType, T: QTextDocument_markContentsDirty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.markContentsDirty(self);
    // return 1;
  }
}

pub trait QTextDocument_markContentsDirty<RetType> {
  fn markContentsDirty(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::markContentsDirty(int from, int length);
impl<'a> /*trait*/ QTextDocument_markContentsDirty<()> for (i32, i32) {
  fn markContentsDirty(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17markContentsDirtyEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QTextDocument17markContentsDirtyEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextDocument::QTextDocument(QObject * parent);
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

  // proto:  void QTextDocument::QTextDocument(QObject * parent);
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (QObject) {
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

  // proto:  void QTextDocument::modificationChanged(bool m);
impl /*struct*/ QTextDocument {
  pub fn modificationChanged<RetType, T: QTextDocument_modificationChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.modificationChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_modificationChanged<RetType> {
  fn modificationChanged(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::modificationChanged(bool m);
impl<'a> /*trait*/ QTextDocument_modificationChanged<()> for (i8) {
  fn modificationChanged(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19modificationChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTextDocument19modificationChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextDocument::characterCount();
impl /*struct*/ QTextDocument {
  pub fn characterCount<RetType, T: QTextDocument_characterCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.characterCount(self);
    // return 1;
  }
}

pub trait QTextDocument_characterCount<RetType> {
  fn characterCount(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::characterCount();
impl<'a> /*trait*/ QTextDocument_characterCount<i32> for () {
  fn characterCount(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14characterCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument14characterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextFrame * QTextDocument::rootFrame();
impl /*struct*/ QTextDocument {
  pub fn rootFrame<RetType, T: QTextDocument_rootFrame<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rootFrame(self);
    // return 1;
  }
}

pub trait QTextDocument_rootFrame<RetType> {
  fn rootFrame(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextFrame * QTextDocument::rootFrame();
impl<'a> /*trait*/ QTextDocument_rootFrame<QTextFrame> for () {
  fn rootFrame(self , rsthis: &mut QTextDocument) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9rootFrameEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9rootFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::firstBlock();
impl /*struct*/ QTextDocument {
  pub fn firstBlock<RetType, T: QTextDocument_firstBlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.firstBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_firstBlock<RetType> {
  fn firstBlock(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::firstBlock();
impl<'a> /*trait*/ QTextDocument_firstBlock<QTextBlock> for () {
  fn firstBlock(self , rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10firstBlockEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10firstBlockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::blockCount();
impl /*struct*/ QTextDocument {
  pub fn blockCount<RetType, T: QTextDocument_blockCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_blockCount<RetType> {
  fn blockCount(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::blockCount();
impl<'a> /*trait*/ QTextDocument_blockCount<i32> for () {
  fn blockCount(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10blockCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10blockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QTextDocument::idealWidth();
impl /*struct*/ QTextDocument {
  pub fn idealWidth<RetType, T: QTextDocument_idealWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.idealWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_idealWidth<RetType> {
  fn idealWidth(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::idealWidth();
impl<'a> /*trait*/ QTextDocument_idealWidth<f64> for () {
  fn idealWidth(self , rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10idealWidthEv()};
    let mut ret = unsafe {_ZNK13QTextDocument10idealWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextDocument::adjustSize();
impl /*struct*/ QTextDocument {
  pub fn adjustSize<RetType, T: QTextDocument_adjustSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.adjustSize(self);
    // return 1;
  }
}

pub trait QTextDocument_adjustSize<RetType> {
  fn adjustSize(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::adjustSize();
impl<'a> /*trait*/ QTextDocument_adjustSize<()> for () {
  fn adjustSize(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10adjustSizeEv()};
     unsafe {_ZN13QTextDocument10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextDocument::isRedoAvailable();
impl /*struct*/ QTextDocument {
  pub fn isRedoAvailable<RetType, T: QTextDocument_isRedoAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRedoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_isRedoAvailable<RetType> {
  fn isRedoAvailable(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isRedoAvailable();
impl<'a> /*trait*/ QTextDocument_isRedoAvailable<i8> for () {
  fn isRedoAvailable(self , rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isRedoAvailableEv()};
    let mut ret = unsafe {_ZNK13QTextDocument15isRedoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
impl /*struct*/ QTextDocument {
  pub fn allFormats<RetType, T: QTextDocument_allFormats<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.allFormats(self);
    // return 1;
  }
}

pub trait QTextDocument_allFormats<RetType> {
  fn allFormats(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
impl<'a> /*trait*/ QTextDocument_allFormats<()> for () {
  fn allFormats(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10allFormatsEv()};
     unsafe {_ZNK13QTextDocument10allFormatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextDocument::blockCountChanged(int newBlockCount);
impl /*struct*/ QTextDocument {
  pub fn blockCountChanged<RetType, T: QTextDocument_blockCountChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockCountChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_blockCountChanged<RetType> {
  fn blockCountChanged(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::blockCountChanged(int newBlockCount);
impl<'a> /*trait*/ QTextDocument_blockCountChanged<()> for (i32) {
  fn blockCountChanged(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17blockCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QTextDocument17blockCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextDocument::defaultStyleSheet();
impl /*struct*/ QTextDocument {
  pub fn defaultStyleSheet<RetType, T: QTextDocument_defaultStyleSheet<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultStyleSheet(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultStyleSheet<RetType> {
  fn defaultStyleSheet(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QString QTextDocument::defaultStyleSheet();
impl<'a> /*trait*/ QTextDocument_defaultStyleSheet<QString> for () {
  fn defaultStyleSheet(self , rsthis: &mut QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultStyleSheetEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17defaultStyleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::lastBlock();
impl /*struct*/ QTextDocument {
  pub fn lastBlock<RetType, T: QTextDocument_lastBlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_lastBlock<RetType> {
  fn lastBlock(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::lastBlock();
impl<'a> /*trait*/ QTextDocument_lastBlock<QTextBlock> for () {
  fn lastBlock(self , rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lastBlockEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9lastBlockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::QTextDocument(const QString & text, QObject * parent);
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (QString, QObject) {
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

  // proto:  bool QTextDocument::useDesignMetrics();
impl /*struct*/ QTextDocument {
  pub fn useDesignMetrics<RetType, T: QTextDocument_useDesignMetrics<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.useDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextDocument_useDesignMetrics<RetType> {
  fn useDesignMetrics(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::useDesignMetrics();
impl<'a> /*trait*/ QTextDocument_useDesignMetrics<i8> for () {
  fn useDesignMetrics(self , rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument16useDesignMetricsEv()};
    let mut ret = unsafe {_ZNK13QTextDocument16useDesignMetricsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextDocument::documentLayoutChanged();
impl /*struct*/ QTextDocument {
  pub fn documentLayoutChanged<RetType, T: QTextDocument_documentLayoutChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentLayoutChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_documentLayoutChanged<RetType> {
  fn documentLayoutChanged(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::documentLayoutChanged();
impl<'a> /*trait*/ QTextDocument_documentLayoutChanged<()> for () {
  fn documentLayoutChanged(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument21documentLayoutChangedEv()};
     unsafe {_ZN13QTextDocument21documentLayoutChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextDocument::pageCount();
impl /*struct*/ QTextDocument {
  pub fn pageCount<RetType, T: QTextDocument_pageCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageCount(self);
    // return 1;
  }
}

pub trait QTextDocument_pageCount<RetType> {
  fn pageCount(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::pageCount();
impl<'a> /*trait*/ QTextDocument_pageCount<i32> for () {
  fn pageCount(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9pageCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9pageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextDocument::baseUrlChanged(const QUrl & url);
impl /*struct*/ QTextDocument {
  pub fn baseUrlChanged<RetType, T: QTextDocument_baseUrlChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.baseUrlChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_baseUrlChanged<RetType> {
  fn baseUrlChanged(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::baseUrlChanged(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_baseUrlChanged<()> for (QUrl) {
  fn baseUrlChanged(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14baseUrlChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument14baseUrlChangedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setTextWidth(qreal width);
impl /*struct*/ QTextDocument {
  pub fn setTextWidth<RetType, T: QTextDocument_setTextWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_setTextWidth<RetType> {
  fn setTextWidth(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setTextWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setTextWidth<()> for (f64) {
  fn setTextWidth(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QTextDocument12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
impl /*struct*/ QTextDocument {
  pub fn setDocumentMargin<RetType, T: QTextDocument_setDocumentMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMargin(self);
    // return 1;
  }
}

pub trait QTextDocument_setDocumentMargin<RetType> {
  fn setDocumentMargin(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
impl<'a> /*trait*/ QTextDocument_setDocumentMargin<()> for (f64) {
  fn setDocumentMargin(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17setDocumentMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QTextDocument17setDocumentMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextDocument::isUndoAvailable();
impl /*struct*/ QTextDocument {
  pub fn isUndoAvailable<RetType, T: QTextDocument_isUndoAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isUndoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_isUndoAvailable<RetType> {
  fn isUndoAvailable(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isUndoAvailable();
impl<'a> /*trait*/ QTextDocument_isUndoAvailable<i8> for () {
  fn isUndoAvailable(self , rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isUndoAvailableEv()};
    let mut ret = unsafe {_ZNK13QTextDocument15isUndoAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTextDocument::indentWidth();
impl /*struct*/ QTextDocument {
  pub fn indentWidth<RetType, T: QTextDocument_indentWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indentWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_indentWidth<RetType> {
  fn indentWidth(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::indentWidth();
impl<'a> /*trait*/ QTextDocument_indentWidth<f64> for () {
  fn indentWidth(self , rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11indentWidthEv()};
    let mut ret = unsafe {_ZNK13QTextDocument11indentWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextDocument::undoCommandAdded();
impl /*struct*/ QTextDocument {
  pub fn undoCommandAdded<RetType, T: QTextDocument_undoCommandAdded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.undoCommandAdded(self);
    // return 1;
  }
}

pub trait QTextDocument_undoCommandAdded<RetType> {
  fn undoCommandAdded(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::undoCommandAdded();
impl<'a> /*trait*/ QTextDocument_undoCommandAdded<()> for () {
  fn undoCommandAdded(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument16undoCommandAddedEv()};
     unsafe {_ZN13QTextDocument16undoCommandAddedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
impl /*struct*/ QTextDocument {
  pub fn setUseDesignMetrics<RetType, T: QTextDocument_setUseDesignMetrics<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUseDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextDocument_setUseDesignMetrics<RetType> {
  fn setUseDesignMetrics(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextDocument_setUseDesignMetrics<()> for (i8) {
  fn setUseDesignMetrics(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19setUseDesignMetricsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTextDocument19setUseDesignMetricsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setIndentWidth(qreal width);
impl /*struct*/ QTextDocument {
  pub fn setIndentWidth<RetType, T: QTextDocument_setIndentWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIndentWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_setIndentWidth<RetType> {
  fn setIndentWidth(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setIndentWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setIndentWidth<()> for (f64) {
  fn setIndentWidth(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setIndentWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QTextDocument14setIndentWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QTextDocument::baseUrl();
impl /*struct*/ QTextDocument {
  pub fn baseUrl<RetType, T: QTextDocument_baseUrl<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.baseUrl(self);
    // return 1;
  }
}

pub trait QTextDocument_baseUrl<RetType> {
  fn baseUrl(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QUrl QTextDocument::baseUrl();
impl<'a> /*trait*/ QTextDocument_baseUrl<QUrl> for () {
  fn baseUrl(self , rsthis: &mut QTextDocument) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7baseUrlEv()};
    let mut ret = unsafe {_ZNK13QTextDocument7baseUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
impl /*struct*/ QTextDocument {
  pub fn frameAt<RetType, T: QTextDocument_frameAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.frameAt(self);
    // return 1;
  }
}

pub trait QTextDocument_frameAt<RetType> {
  fn frameAt(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
impl<'a> /*trait*/ QTextDocument_frameAt<QTextFrame> for (i32) {
  fn frameAt(self , rsthis: &mut QTextDocument) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7frameAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument7frameAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::QTextDocument(const QTextDocument & );
impl<'a> /*trait*/ QTextDocument_NewQTextDocument for (QTextDocument) {
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

  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
impl /*struct*/ QTextDocument {
  pub fn setDefaultTextOption<RetType, T: QTextDocument_setDefaultTextOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultTextOption(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultTextOption<RetType> {
  fn setDefaultTextOption(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextDocument_setDefaultTextOption<()> for (QTextOption) {
  fn setDefaultTextOption(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QTextDocument::defaultFont();
impl /*struct*/ QTextDocument {
  pub fn defaultFont<RetType, T: QTextDocument_defaultFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultFont(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultFont<RetType> {
  fn defaultFont(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QFont QTextDocument::defaultFont();
impl<'a> /*trait*/ QTextDocument_defaultFont<QFont> for () {
  fn defaultFont(self , rsthis: &mut QTextDocument) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11defaultFontEv()};
    let mut ret = unsafe {_ZNK13QTextDocument11defaultFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
impl /*struct*/ QTextDocument {
  pub fn findBlockByNumber<RetType, T: QTextDocument_findBlockByNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.findBlockByNumber(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlockByNumber<RetType> {
  fn findBlockByNumber(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByNumber<QTextBlock> for (i32) {
  fn findBlockByNumber(self , rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17findBlockByNumberEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument17findBlockByNumberEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextOption QTextDocument::defaultTextOption();
impl /*struct*/ QTextDocument {
  pub fn defaultTextOption<RetType, T: QTextDocument_defaultTextOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultTextOption(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultTextOption<RetType> {
  fn defaultTextOption(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextOption QTextDocument::defaultTextOption();
impl<'a> /*trait*/ QTextDocument_defaultTextOption<QTextOption> for () {
  fn defaultTextOption(self , rsthis: &mut QTextDocument) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultTextOptionEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17defaultTextOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::findBlock(int pos);
impl /*struct*/ QTextDocument {
  pub fn findBlock<RetType, T: QTextDocument_findBlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.findBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlock<RetType> {
  fn findBlock(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::findBlock(int pos);
impl<'a> /*trait*/ QTextDocument_findBlock<QTextBlock> for (i32) {
  fn findBlock(self , rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9findBlockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument9findBlockEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
impl /*struct*/ QTextDocument {
  pub fn setBaseUrl<RetType, T: QTextDocument_setBaseUrl<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBaseUrl(self);
    // return 1;
  }
}

pub trait QTextDocument_setBaseUrl<RetType> {
  fn setBaseUrl(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_setBaseUrl<()> for (QUrl) {
  fn setBaseUrl(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10setBaseUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument10setBaseUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::redoAvailable(bool );
impl /*struct*/ QTextDocument {
  pub fn redoAvailable<RetType, T: QTextDocument_redoAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.redoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_redoAvailable<RetType> {
  fn redoAvailable(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::redoAvailable(bool );
impl<'a> /*trait*/ QTextDocument_redoAvailable<()> for (i8) {
  fn redoAvailable(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument13redoAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTextDocument13redoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::redo();
impl<'a> /*trait*/ QTextDocument_redo<()> for () {
  fn redo(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEv()};
     unsafe {_ZN13QTextDocument4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
impl /*struct*/ QTextDocument {
  pub fn drawContents<RetType, T: QTextDocument_drawContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawContents(self);
    // return 1;
  }
}

pub trait QTextDocument_drawContents<RetType> {
  fn drawContents(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
impl<'a> /*trait*/ QTextDocument_drawContents<()> for (QPainter, QRectF) {
  fn drawContents(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
impl /*struct*/ QTextDocument {
  pub fn findBlockByLineNumber<RetType, T: QTextDocument_findBlockByLineNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.findBlockByLineNumber(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlockByLineNumber<RetType> {
  fn findBlockByLineNumber(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByLineNumber<QTextBlock> for (i32) {
  fn findBlockByLineNumber(self , rsthis: &mut QTextDocument) -> QTextBlock {
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
impl<'a> /*trait*/ QTextDocument_undo<()> for () {
  fn undo(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEv()};
     unsafe {_ZN13QTextDocument4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextDocument::textWidth();
impl /*struct*/ QTextDocument {
  pub fn textWidth<RetType, T: QTextDocument_textWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_textWidth<RetType> {
  fn textWidth(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::textWidth();
impl<'a> /*trait*/ QTextDocument_textWidth<f64> for () {
  fn textWidth(self , rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9textWidthEv()};
    let mut ret = unsafe {_ZNK13QTextDocument9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextDocument::metaObject();
impl /*struct*/ QTextDocument {
  pub fn metaObject<RetType, T: QTextDocument_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextDocument_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  const QMetaObject * QTextDocument::metaObject();
impl<'a> /*trait*/ QTextDocument_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10metaObjectEv()};
     unsafe {_ZNK13QTextDocument10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextDocument::availableRedoSteps();
impl /*struct*/ QTextDocument {
  pub fn availableRedoSteps<RetType, T: QTextDocument_availableRedoSteps<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.availableRedoSteps(self);
    // return 1;
  }
}

pub trait QTextDocument_availableRedoSteps<RetType> {
  fn availableRedoSteps(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::availableRedoSteps();
impl<'a> /*trait*/ QTextDocument_availableRedoSteps<i32> for () {
  fn availableRedoSteps(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableRedoStepsEv()};
    let mut ret = unsafe {_ZNK13QTextDocument18availableRedoStepsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QChar QTextDocument::characterAt(int pos);
impl /*struct*/ QTextDocument {
  pub fn characterAt<RetType, T: QTextDocument_characterAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.characterAt(self);
    // return 1;
  }
}

pub trait QTextDocument_characterAt<RetType> {
  fn characterAt(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QChar QTextDocument::characterAt(int pos);
impl<'a> /*trait*/ QTextDocument_characterAt<QChar> for (i32) {
  fn characterAt(self , rsthis: &mut QTextDocument) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11characterAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextDocument11characterAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
impl /*struct*/ QTextDocument {
  pub fn setDefaultFont<RetType, T: QTextDocument_setDefaultFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultFont(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultFont<RetType> {
  fn setDefaultFont(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
impl<'a> /*trait*/ QTextDocument_setDefaultFont<()> for (QFont) {
  fn setDefaultFont(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setDefaultFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument14setDefaultFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
impl /*struct*/ QTextDocument {
  pub fn objectForFormat<RetType, T: QTextDocument_objectForFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.objectForFormat(self);
    // return 1;
  }
}

pub trait QTextDocument_objectForFormat<RetType> {
  fn objectForFormat(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
impl<'a> /*trait*/ QTextDocument_objectForFormat<QTextObject> for (QTextFormat) {
  fn objectForFormat(self , rsthis: &mut QTextDocument) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15objectForFormatERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QTextDocument15objectForFormatERK11QTextFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextDocument::isEmpty();
impl /*struct*/ QTextDocument {
  pub fn isEmpty<RetType, T: QTextDocument_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextDocument_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isEmpty();
impl<'a> /*trait*/ QTextDocument_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7isEmptyEv()};
    let mut ret = unsafe {_ZNK13QTextDocument7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextDocument::isUndoRedoEnabled();
impl /*struct*/ QTextDocument {
  pub fn isUndoRedoEnabled<RetType, T: QTextDocument_isUndoRedoEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextDocument_isUndoRedoEnabled<RetType> {
  fn isUndoRedoEnabled(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextDocument_isUndoRedoEnabled<i8> for () {
  fn isUndoRedoEnabled(self , rsthis: &mut QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17isUndoRedoEnabledEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
impl /*struct*/ QTextDocument {
  pub fn contentsChange<RetType, T: QTextDocument_contentsChange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contentsChange(self);
    // return 1;
  }
}

pub trait QTextDocument_contentsChange<RetType> {
  fn contentsChange(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::contentsChange(int from, int charsRemoved, int charsAdded);
impl<'a> /*trait*/ QTextDocument_contentsChange<()> for (i32, i32, i32) {
  fn contentsChange(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14contentsChangeEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QTextDocument14contentsChangeEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QTextDocument::~QTextDocument();
impl /*struct*/ QTextDocument {
  pub fn FreeQTextDocument<RetType, T: QTextDocument_FreeQTextDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextDocument(self);
    // return 1;
  }
}

pub trait QTextDocument_FreeQTextDocument<RetType> {
  fn FreeQTextDocument(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::~QTextDocument();
impl<'a> /*trait*/ QTextDocument_FreeQTextDocument<()> for () {
  fn FreeQTextDocument(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentD0Ev()};
     unsafe {_ZN13QTextDocumentD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextDocument::contentsChanged();
impl /*struct*/ QTextDocument {
  pub fn contentsChanged<RetType, T: QTextDocument_contentsChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contentsChanged(self);
    // return 1;
  }
}

pub trait QTextDocument_contentsChanged<RetType> {
  fn contentsChanged(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::contentsChanged();
impl<'a> /*trait*/ QTextDocument_contentsChanged<()> for () {
  fn contentsChanged(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument15contentsChangedEv()};
     unsafe {_ZN13QTextDocument15contentsChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextDocument::documentMargin();
impl /*struct*/ QTextDocument {
  pub fn documentMargin<RetType, T: QTextDocument_documentMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentMargin(self);
    // return 1;
  }
}

pub trait QTextDocument_documentMargin<RetType> {
  fn documentMargin(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::documentMargin();
impl<'a> /*trait*/ QTextDocument_documentMargin<f64> for () {
  fn documentMargin(self , rsthis: &mut QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentMarginEv()};
    let mut ret = unsafe {_ZNK13QTextDocument14documentMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
impl /*struct*/ QTextDocument {
  pub fn setPageSize<RetType, T: QTextDocument_setPageSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPageSize(self);
    // return 1;
  }
}

pub trait QTextDocument_setPageSize<RetType> {
  fn setPageSize(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
impl<'a> /*trait*/ QTextDocument_setPageSize<()> for (QSizeF) {
  fn setPageSize(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setPageSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument11setPageSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setHtml(const QString & html);
impl /*struct*/ QTextDocument {
  pub fn setHtml<RetType, T: QTextDocument_setHtml<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHtml(self);
    // return 1;
  }
}

pub trait QTextDocument_setHtml<RetType> {
  fn setHtml(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setHtml(const QString & html);
impl<'a> /*trait*/ QTextDocument_setHtml<()> for (QString) {
  fn setHtml(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::end();
impl /*struct*/ QTextDocument {
  pub fn end<RetType, T: QTextDocument_end<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QTextDocument_end<RetType> {
  fn end(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::end();
impl<'a> /*trait*/ QTextDocument_end<QTextBlock> for () {
  fn end(self , rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument3endEv()};
    let mut ret = unsafe {_ZNK13QTextDocument3endEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::maximumBlockCount();
impl /*struct*/ QTextDocument {
  pub fn maximumBlockCount<RetType, T: QTextDocument_maximumBlockCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumBlockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_maximumBlockCount<RetType> {
  fn maximumBlockCount(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::maximumBlockCount();
impl<'a> /*trait*/ QTextDocument_maximumBlockCount<i32> for () {
  fn maximumBlockCount(self , rsthis: &mut QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17maximumBlockCountEv()};
    let mut ret = unsafe {_ZNK13QTextDocument17maximumBlockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextDocument::setPlainText(const QString & text);
impl /*struct*/ QTextDocument {
  pub fn setPlainText<RetType, T: QTextDocument_setPlainText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPlainText(self);
    // return 1;
  }
}

pub trait QTextDocument_setPlainText<RetType> {
  fn setPlainText(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextDocument_setPlainText<()> for (QString) {
  fn setPlainText(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QTextDocument12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::clear();
impl /*struct*/ QTextDocument {
  pub fn clear<RetType, T: QTextDocument_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTextDocument_clear<RetType> {
  fn clear(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::clear();
impl<'a> /*trait*/ QTextDocument_clear<()> for () {
  fn clear(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument5clearEv()};
     unsafe {_ZN13QTextDocument5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
impl /*struct*/ QTextDocument {
  pub fn resource<RetType, T: QTextDocument_resource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resource(self);
    // return 1;
  }
}

pub trait QTextDocument_resource<RetType> {
  fn resource(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextDocument_resource<QVariant> for (i32, QUrl) {
  fn resource(self , rsthis: &mut QTextDocument) -> QVariant {
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

  // proto:  QTextBlock QTextDocument::begin();
impl /*struct*/ QTextDocument {
  pub fn begin<RetType, T: QTextDocument_begin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QTextDocument_begin<RetType> {
  fn begin(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::begin();
impl<'a> /*trait*/ QTextDocument_begin<QTextBlock> for () {
  fn begin(self , rsthis: &mut QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5beginEv()};
    let mut ret = unsafe {_ZNK13QTextDocument5beginEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
impl /*struct*/ QTextDocument {
  pub fn setMaximumBlockCount<RetType, T: QTextDocument_setMaximumBlockCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumBlockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_setMaximumBlockCount<RetType> {
  fn setMaximumBlockCount(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QTextDocument_setMaximumBlockCount<()> for (i32) {
  fn setMaximumBlockCount(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QTextDocument20setMaximumBlockCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setModified(bool m);
impl /*struct*/ QTextDocument {
  pub fn setModified<RetType, T: QTextDocument_setModified<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setModified(self);
    // return 1;
  }
}

pub trait QTextDocument_setModified<RetType> {
  fn setModified(self , rsthis: &mut QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setModified(bool m);
impl<'a> /*trait*/ QTextDocument_setModified<()> for (i8) {
  fn setModified(self , rsthis: &mut QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setModifiedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTextDocument11setModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

