// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qplaintextedit.h
// dst-file: /src/widgets/qplaintextedit.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::gui::qabstracttextdocumentlayout::QAbstractTextDocumentLayout; // 771
use std::ops::Deref;
use super::super::gui::qpainter::QPainter; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::gui::qtextobject::QTextFrame; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::gui::qtextobject::QTextBlock; // 771
use super::super::gui::qtextdocument::QTextDocument; // 771
use super::super::core::qsize::QSizeF; // 771
use super::qabstractscrollarea::QAbstractScrollArea; // 773
use super::super::core::qpoint::QPoint; // 771
use super::qmenu::QMenu; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qurl::QUrl; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::gui::qtextcursor::QTextCursor; // 771
use super::super::gui::qtextformat::QTextCharFormat; // 771
use super::super::gui::qpagedpaintdevice::QPagedPaintDevice; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qregexp::QRegExp; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPlainTextDocumentLayout_Class_Size() -> c_int;
  // proto:  void QPlainTextDocumentLayout::requestUpdate();
  fn C_ZN24QPlainTextDocumentLayout13requestUpdateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextDocumentLayout::setCursorWidth(int width);
  fn C_ZN24QPlainTextDocumentLayout14setCursorWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
  fn C_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextDocumentLayout::pageCount();
  fn C_ZNK24QPlainTextDocumentLayout9pageCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QPlainTextDocumentLayout::metaObject();
  fn C_ZNK24QPlainTextDocumentLayout10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
  fn C_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextDocumentLayout::~QPlainTextDocumentLayout();
  fn C_ZN24QPlainTextDocumentLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
  fn C_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextDocumentLayout::cursorWidth();
  fn C_ZNK24QPlainTextDocumentLayout11cursorWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPlainTextDocumentLayout::QPlainTextDocumentLayout(QTextDocument * document);
  fn C_ZN24QPlainTextDocumentLayoutC2EP13QTextDocument(arg0: *mut c_void) -> u64;
  // proto:  QSizeF QPlainTextDocumentLayout::documentSize();
  fn C_ZNK24QPlainTextDocumentLayout12documentSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QPlainTextEdit_Class_Size() -> c_int;
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
  fn C_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPlainTextEdit::ensureCursorVisible();
  fn C_ZN14QPlainTextEdit19ensureCursorVisibleEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTextDocument * QPlainTextEdit::document();
  fn C_ZNK14QPlainTextEdit8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QPlainTextEdit::cursorRect();
  fn C_ZNK14QPlainTextEdit10cursorRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
  fn C_ZN14QPlainTextEdit18setTabChangesFocusEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QPlainTextEdit::toPlainText();
  fn C_ZNK14QPlainTextEdit11toPlainTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
  fn C_ZN14QPlainTextEdit12loadResourceEiRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextEdit::tabStopWidth();
  fn C_ZNK14QPlainTextEdit12tabStopWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QPlainTextEdit::isReadOnly();
  fn C_ZNK14QPlainTextEdit10isReadOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
  fn C_ZN14QPlainTextEdit11setReadOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QTextCursor QPlainTextEdit::textCursor();
  fn C_ZNK14QPlainTextEdit10textCursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
  fn C_ZN14QPlainTextEdit17setCenterOnScrollEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QPlainTextEdit::placeholderText();
  fn C_ZNK14QPlainTextEdit15placeholderTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QPlainTextEdit::blockCount();
  fn C_ZNK14QPlainTextEdit10blockCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn C_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
  fn C_ZN14QPlainTextEdit11setDocumentEP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
  fn C_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
  fn C_ZN14QPlainTextEdit15setTabStopWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QPlainTextEdit::backgroundVisible();
  fn C_ZNK14QPlainTextEdit17backgroundVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::redo();
  fn C_ZN14QPlainTextEdit4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextEdit::QPlainTextEdit(const QString & text, QWidget * parent);
  fn C_ZN14QPlainTextEditC2ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
  fn C_ZN14QPlainTextEdit16setOverwriteModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QPlainTextEdit::tabChangesFocus();
  fn C_ZNK14QPlainTextEdit15tabChangesFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::copy();
  fn C_ZN14QPlainTextEdit4copyEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn C_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QPlainTextEdit::maximumBlockCount();
  fn C_ZNK14QPlainTextEdit17maximumBlockCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
  fn C_ZN14QPlainTextEdit15insertPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
  fn C_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::paste();
  fn C_ZN14QPlainTextEdit5pasteEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextEdit::zoomIn(int range);
  fn C_ZN14QPlainTextEdit6zoomInEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
  fn C_ZN14QPlainTextEdit20setMaximumBlockCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
  fn C_ZNK14QPlainTextEdit17currentCharFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPlainTextEdit::setCursorWidth(int width);
  fn C_ZN14QPlainTextEdit14setCursorWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QString QPlainTextEdit::documentTitle();
  fn C_ZNK14QPlainTextEdit13documentTitleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPlainTextEdit::selectAll();
  fn C_ZN14QPlainTextEdit9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
  fn C_ZN14QPlainTextEdit12setPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
  fn C_ZN14QPlainTextEdit20setBackgroundVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
  fn C_ZN14QPlainTextEdit18setUndoRedoEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QPlainTextEdit::overwriteMode();
  fn C_ZNK14QPlainTextEdit13overwriteModeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::centerCursor();
  fn C_ZN14QPlainTextEdit12centerCursorEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
  fn C_ZNK14QPlainTextEdit10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu();
  fn C_ZN14QPlainTextEdit25createStandardContextMenuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
  fn C_ZN14QPlainTextEdit16setDocumentTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::~QPlainTextEdit();
  fn C_ZN14QPlainTextEditD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextEdit::clear();
  fn C_ZN14QPlainTextEdit5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
  fn C_ZNK14QPlainTextEdit8anchorAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPlainTextEdit::canPaste();
  fn C_ZNK14QPlainTextEdit8canPasteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::QPlainTextEdit(QWidget * parent);
  fn C_ZN14QPlainTextEditC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QPlainTextEdit::cut();
  fn C_ZN14QPlainTextEdit3cutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
  fn C_ZN14QPlainTextEdit10appendHtmlERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
  fn C_ZNK14QPlainTextEdit17isUndoRedoEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::zoomOut(int range);
  fn C_ZN14QPlainTextEdit7zoomOutEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
  fn C_ZN14QPlainTextEdit18setPlaceholderTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPlainTextEdit::undo();
  fn C_ZN14QPlainTextEdit4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
  fn C_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPlainTextEdit::centerOnScroll();
  fn C_ZNK14QPlainTextEdit14centerOnScrollEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
  fn C_ZN14QPlainTextEdit15appendPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QPlainTextEdit::cursorWidth();
  fn C_ZNK14QPlainTextEdit11cursorWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
  fn C_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit17blockCountChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13undoAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit16selectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13redoAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit19modificationChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13updateRequestERK5QRecti(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit11textChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit21cursorPositionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13copyAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPlainTextDocumentLayout)=1
#[derive(Default)]
pub struct QPlainTextDocumentLayout {
  qbase: QAbstractTextDocumentLayout,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPlainTextEdit)=1
#[derive(Default)]
pub struct QPlainTextEdit {
  qbase: QAbstractScrollArea,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _cursorPositionChanged: QPlainTextEdit_cursorPositionChanged_signal,
  pub _modificationChanged: QPlainTextEdit_modificationChanged_signal,
  pub _redoAvailable: QPlainTextEdit_redoAvailable_signal,
  pub _selectionChanged: QPlainTextEdit_selectionChanged_signal,
  pub _updateRequest: QPlainTextEdit_updateRequest_signal,
  pub _blockCountChanged: QPlainTextEdit_blockCountChanged_signal,
  pub _undoAvailable: QPlainTextEdit_undoAvailable_signal,
  pub _textChanged: QPlainTextEdit_textChanged_signal,
  pub _copyAvailable: QPlainTextEdit_copyAvailable_signal,
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPlainTextDocumentLayout {
    return QPlainTextDocumentLayout{qbase: QAbstractTextDocumentLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPlainTextDocumentLayout {
  type Target = QAbstractTextDocumentLayout;

  fn deref(&self) -> &QAbstractTextDocumentLayout {
    return & self.qbase;
  }
}
impl AsRef<QAbstractTextDocumentLayout> for QPlainTextDocumentLayout {
  fn as_ref(& self) -> & QAbstractTextDocumentLayout {
    return & self.qbase;
  }
}
  // proto:  void QPlainTextDocumentLayout::requestUpdate();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn requestUpdate<RetType, T: QPlainTextDocumentLayout_requestUpdate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestUpdate(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_requestUpdate<RetType> {
  fn requestUpdate(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  void QPlainTextDocumentLayout::requestUpdate();
impl<'a> /*trait*/ QPlainTextDocumentLayout_requestUpdate<()> for () {
  fn requestUpdate(self , rsthis: & QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayout13requestUpdateEv()};
     unsafe {C_ZN24QPlainTextDocumentLayout13requestUpdateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextDocumentLayout::setCursorWidth(int width);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn setCursorWidth<RetType, T: QPlainTextDocumentLayout_setCursorWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_setCursorWidth<RetType> {
  fn setCursorWidth(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  void QPlainTextDocumentLayout::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextDocumentLayout_setCursorWidth<()> for (i32) {
  fn setCursorWidth(self , rsthis: & QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayout14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN24QPlainTextDocumentLayout14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn frameBoundingRect<RetType, T: QPlainTextDocumentLayout_frameBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameBoundingRect(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_frameBoundingRect<RetType> {
  fn frameBoundingRect(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
impl<'a> /*trait*/ QPlainTextDocumentLayout_frameBoundingRect<QRectF> for (&'a QTextFrame) {
  fn frameBoundingRect(self , rsthis: & QPlainTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPlainTextDocumentLayout::pageCount();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn pageCount<RetType, T: QPlainTextDocumentLayout_pageCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pageCount(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_pageCount<RetType> {
  fn pageCount(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  int QPlainTextDocumentLayout::pageCount();
impl<'a> /*trait*/ QPlainTextDocumentLayout_pageCount<i32> for () {
  fn pageCount(self , rsthis: & QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout9pageCountEv()};
    let mut ret = unsafe {C_ZNK24QPlainTextDocumentLayout9pageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPlainTextDocumentLayout::metaObject();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn metaObject<RetType, T: QPlainTextDocumentLayout_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  const QMetaObject * QPlainTextDocumentLayout::metaObject();
impl<'a> /*trait*/ QPlainTextDocumentLayout_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPlainTextDocumentLayout) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout10metaObjectEv()};
    let mut ret = unsafe {C_ZNK24QPlainTextDocumentLayout10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn ensureBlockLayout<RetType, T: QPlainTextDocumentLayout_ensureBlockLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureBlockLayout(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_ensureBlockLayout<RetType> {
  fn ensureBlockLayout(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
impl<'a> /*trait*/ QPlainTextDocumentLayout_ensureBlockLayout<()> for (&'a QTextBlock) {
  fn ensureBlockLayout(self , rsthis: & QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextDocumentLayout::~QPlainTextDocumentLayout();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn free<RetType, T: QPlainTextDocumentLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_free<RetType> {
  fn free(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  void QPlainTextDocumentLayout::~QPlainTextDocumentLayout();
impl<'a> /*trait*/ QPlainTextDocumentLayout_free<()> for () {
  fn free(self , rsthis: & QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayoutD2Ev()};
     unsafe {C_ZN24QPlainTextDocumentLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn blockBoundingRect<RetType, T: QPlainTextDocumentLayout_blockBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingRect(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_blockBoundingRect<RetType> {
  fn blockBoundingRect(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl<'a> /*trait*/ QPlainTextDocumentLayout_blockBoundingRect<QRectF> for (&'a QTextBlock) {
  fn blockBoundingRect(self , rsthis: & QPlainTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPlainTextDocumentLayout::cursorWidth();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn cursorWidth<RetType, T: QPlainTextDocumentLayout_cursorWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_cursorWidth<RetType> {
  fn cursorWidth(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  int QPlainTextDocumentLayout::cursorWidth();
impl<'a> /*trait*/ QPlainTextDocumentLayout_cursorWidth<i32> for () {
  fn cursorWidth(self , rsthis: & QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout11cursorWidthEv()};
    let mut ret = unsafe {C_ZNK24QPlainTextDocumentLayout11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPlainTextDocumentLayout::QPlainTextDocumentLayout(QTextDocument * document);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn new<T: QPlainTextDocumentLayout_new>(value: T) -> QPlainTextDocumentLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_new {
  fn new(self) -> QPlainTextDocumentLayout;
}

  // proto:  void QPlainTextDocumentLayout::QPlainTextDocumentLayout(QTextDocument * document);
impl<'a> /*trait*/ QPlainTextDocumentLayout_new for (&'a QTextDocument) {
  fn new(self) -> QPlainTextDocumentLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayoutC2EP13QTextDocument()};
    let ctysz: c_int = unsafe{QPlainTextDocumentLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN24QPlainTextDocumentLayoutC2EP13QTextDocument(arg0)};
    let rsthis = QPlainTextDocumentLayout{qbase: QAbstractTextDocumentLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSizeF QPlainTextDocumentLayout::documentSize();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn documentSize<RetType, T: QPlainTextDocumentLayout_documentSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentSize(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_documentSize<RetType> {
  fn documentSize(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}

  // proto:  QSizeF QPlainTextDocumentLayout::documentSize();
impl<'a> /*trait*/ QPlainTextDocumentLayout_documentSize<QSizeF> for () {
  fn documentSize(self , rsthis: & QPlainTextDocumentLayout) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout12documentSizeEv()};
    let mut ret = unsafe {C_ZNK24QPlainTextDocumentLayout12documentSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPlainTextEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPlainTextEdit {
    return QPlainTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPlainTextEdit {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QPlainTextEdit {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
impl /*struct*/ QPlainTextEdit {
  pub fn createStandardContextMenu<RetType, T: QPlainTextEdit_createStandardContextMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_createStandardContextMenu<RetType> {
  fn createStandardContextMenu(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu<QMenu> for (&'a QPoint) {
  fn createStandardContextMenu(self , rsthis: & QPlainTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::ensureCursorVisible();
impl /*struct*/ QPlainTextEdit {
  pub fn ensureCursorVisible<RetType, T: QPlainTextEdit_ensureCursorVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureCursorVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_ensureCursorVisible<RetType> {
  fn ensureCursorVisible(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QPlainTextEdit_ensureCursorVisible<()> for () {
  fn ensureCursorVisible(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit19ensureCursorVisibleEv()};
     unsafe {C_ZN14QPlainTextEdit19ensureCursorVisibleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextDocument * QPlainTextEdit::document();
impl /*struct*/ QPlainTextEdit {
  pub fn document<RetType, T: QPlainTextEdit_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_document<RetType> {
  fn document(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QTextDocument * QPlainTextEdit::document();
impl<'a> /*trait*/ QPlainTextEdit_document<QTextDocument> for () {
  fn document(self , rsthis: & QPlainTextEdit) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8documentEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QPlainTextEdit::cursorRect();
impl /*struct*/ QPlainTextEdit {
  pub fn cursorRect<RetType, T: QPlainTextEdit_cursorRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorRect(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorRect<RetType> {
  fn cursorRect(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QRect QPlainTextEdit::cursorRect();
impl<'a> /*trait*/ QPlainTextEdit_cursorRect<QRect> for () {
  fn cursorRect(self , rsthis: & QPlainTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit10cursorRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
impl /*struct*/ QPlainTextEdit {
  pub fn setTabChangesFocus<RetType, T: QPlainTextEdit_setTabChangesFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTabChangesFocus<RetType> {
  fn setTabChangesFocus(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QPlainTextEdit_setTabChangesFocus<()> for (i8) {
  fn setTabChangesFocus(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN14QPlainTextEdit18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::toPlainText();
impl /*struct*/ QPlainTextEdit {
  pub fn toPlainText<RetType, T: QPlainTextEdit_toPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_toPlainText<RetType> {
  fn toPlainText(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::toPlainText();
impl<'a> /*trait*/ QPlainTextEdit_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: & QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11toPlainTextEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
impl /*struct*/ QPlainTextEdit {
  pub fn loadResource<RetType, T: QPlainTextEdit_loadResource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadResource(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_loadResource<RetType> {
  fn loadResource(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QVariant QPlainTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QPlainTextEdit_loadResource<QVariant> for (i32, &'a QUrl) {
  fn loadResource(self , rsthis: & QPlainTextEdit) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN14QPlainTextEdit12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::tabStopWidth();
impl /*struct*/ QPlainTextEdit {
  pub fn tabStopWidth<RetType, T: QPlainTextEdit_tabStopWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabStopWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_tabStopWidth<RetType> {
  fn tabStopWidth(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::tabStopWidth();
impl<'a> /*trait*/ QPlainTextEdit_tabStopWidth<i32> for () {
  fn tabStopWidth(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit12tabStopWidthEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit12tabStopWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::isReadOnly();
impl /*struct*/ QPlainTextEdit {
  pub fn isReadOnly<RetType, T: QPlainTextEdit_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::isReadOnly();
impl<'a> /*trait*/ QPlainTextEdit_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10isReadOnlyEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
impl /*struct*/ QPlainTextEdit {
  pub fn setReadOnly<RetType, T: QPlainTextEdit_setReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QPlainTextEdit_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN14QPlainTextEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCursor QPlainTextEdit::textCursor();
impl /*struct*/ QPlainTextEdit {
  pub fn textCursor<RetType, T: QPlainTextEdit_textCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_textCursor<RetType> {
  fn textCursor(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QTextCursor QPlainTextEdit::textCursor();
impl<'a> /*trait*/ QPlainTextEdit_textCursor<QTextCursor> for () {
  fn textCursor(self , rsthis: & QPlainTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10textCursorEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
impl /*struct*/ QPlainTextEdit {
  pub fn setCenterOnScroll<RetType, T: QPlainTextEdit_setCenterOnScroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCenterOnScroll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCenterOnScroll<RetType> {
  fn setCenterOnScroll(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setCenterOnScroll(bool enabled);
impl<'a> /*trait*/ QPlainTextEdit_setCenterOnScroll<()> for (i8) {
  fn setCenterOnScroll(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit17setCenterOnScrollEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN14QPlainTextEdit17setCenterOnScrollEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::placeholderText();
impl /*struct*/ QPlainTextEdit {
  pub fn placeholderText<RetType, T: QPlainTextEdit_placeholderText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.placeholderText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_placeholderText<RetType> {
  fn placeholderText(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::placeholderText();
impl<'a> /*trait*/ QPlainTextEdit_placeholderText<QString> for () {
  fn placeholderText(self , rsthis: & QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15placeholderTextEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::blockCount();
impl /*struct*/ QPlainTextEdit {
  pub fn blockCount<RetType, T: QPlainTextEdit_blockCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_blockCount<RetType> {
  fn blockCount(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::blockCount();
impl<'a> /*trait*/ QPlainTextEdit_blockCount<i32> for () {
  fn blockCount(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10blockCountEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit10blockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl /*struct*/ QPlainTextEdit {
  pub fn setCurrentCharFormat<RetType, T: QPlainTextEdit_setCurrentCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCurrentCharFormat<RetType> {
  fn setCurrentCharFormat(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QPlainTextEdit_setCurrentCharFormat<()> for (&'a QTextCharFormat) {
  fn setCurrentCharFormat(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
impl /*struct*/ QPlainTextEdit {
  pub fn setDocument<RetType, T: QPlainTextEdit_setDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocument(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setDocument<RetType> {
  fn setDocument(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QPlainTextEdit_setDocument<()> for (&'a QTextDocument) {
  fn setDocument(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
impl /*struct*/ QPlainTextEdit {
  pub fn print<RetType, T: QPlainTextEdit_print<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.print(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_print<RetType> {
  fn print(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QPlainTextEdit_print<()> for (&'a QPagedPaintDevice) {
  fn print(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
impl /*struct*/ QPlainTextEdit {
  pub fn setTabStopWidth<RetType, T: QPlainTextEdit_setTabStopWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabStopWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTabStopWidth<RetType> {
  fn setTabStopWidth(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setTabStopWidth<()> for (i32) {
  fn setTabStopWidth(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN14QPlainTextEdit15setTabStopWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::backgroundVisible();
impl /*struct*/ QPlainTextEdit {
  pub fn backgroundVisible<RetType, T: QPlainTextEdit_backgroundVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_backgroundVisible<RetType> {
  fn backgroundVisible(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::backgroundVisible();
impl<'a> /*trait*/ QPlainTextEdit_backgroundVisible<i8> for () {
  fn backgroundVisible(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17backgroundVisibleEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit17backgroundVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::redo();
impl /*struct*/ QPlainTextEdit {
  pub fn redo<RetType, T: QPlainTextEdit_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_redo<RetType> {
  fn redo(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::redo();
impl<'a> /*trait*/ QPlainTextEdit_redo<()> for () {
  fn redo(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4redoEv()};
     unsafe {C_ZN14QPlainTextEdit4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::QPlainTextEdit(const QString & text, QWidget * parent);
impl /*struct*/ QPlainTextEdit {
  pub fn new<T: QPlainTextEdit_new>(value: T) -> QPlainTextEdit {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextEdit_new {
  fn new(self) -> QPlainTextEdit;
}

  // proto:  void QPlainTextEdit::QPlainTextEdit(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPlainTextEdit_new for (&'a QString, &'a QWidget) {
  fn new(self) -> QPlainTextEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditC2ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QPlainTextEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QPlainTextEditC2ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QPlainTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
impl /*struct*/ QPlainTextEdit {
  pub fn setOverwriteMode<RetType, T: QPlainTextEdit_setOverwriteMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOverwriteMode(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setOverwriteMode<RetType> {
  fn setOverwriteMode(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QPlainTextEdit_setOverwriteMode<()> for (i8) {
  fn setOverwriteMode(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setOverwriteModeEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN14QPlainTextEdit16setOverwriteModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::tabChangesFocus();
impl /*struct*/ QPlainTextEdit {
  pub fn tabChangesFocus<RetType, T: QPlainTextEdit_tabChangesFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_tabChangesFocus<RetType> {
  fn tabChangesFocus(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QPlainTextEdit_tabChangesFocus<i8> for () {
  fn tabChangesFocus(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit15tabChangesFocusEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::copy();
impl /*struct*/ QPlainTextEdit {
  pub fn copy<RetType, T: QPlainTextEdit_copy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_copy<RetType> {
  fn copy(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::copy();
impl<'a> /*trait*/ QPlainTextEdit_copy<()> for () {
  fn copy(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4copyEv()};
     unsafe {C_ZN14QPlainTextEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl /*struct*/ QPlainTextEdit {
  pub fn mergeCurrentCharFormat<RetType, T: QPlainTextEdit_mergeCurrentCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_mergeCurrentCharFormat<RetType> {
  fn mergeCurrentCharFormat(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QPlainTextEdit_mergeCurrentCharFormat<()> for (&'a QTextCharFormat) {
  fn mergeCurrentCharFormat(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::maximumBlockCount();
impl /*struct*/ QPlainTextEdit {
  pub fn maximumBlockCount<RetType, T: QPlainTextEdit_maximumBlockCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumBlockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_maximumBlockCount<RetType> {
  fn maximumBlockCount(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::maximumBlockCount();
impl<'a> /*trait*/ QPlainTextEdit_maximumBlockCount<i32> for () {
  fn maximumBlockCount(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17maximumBlockCountEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit17maximumBlockCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
impl /*struct*/ QPlainTextEdit {
  pub fn insertPlainText<RetType, T: QPlainTextEdit_insertPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_insertPlainText<RetType> {
  fn insertPlainText(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_insertPlainText<()> for (&'a QString) {
  fn insertPlainText(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit15insertPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
impl /*struct*/ QPlainTextEdit {
  pub fn setTextCursor<RetType, T: QPlainTextEdit_setTextCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setTextCursor<RetType> {
  fn setTextCursor(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_setTextCursor<()> for (&'a QTextCursor) {
  fn setTextCursor(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::paste();
impl /*struct*/ QPlainTextEdit {
  pub fn paste<RetType, T: QPlainTextEdit_paste<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paste(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_paste<RetType> {
  fn paste(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::paste();
impl<'a> /*trait*/ QPlainTextEdit_paste<()> for () {
  fn paste(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5pasteEv()};
     unsafe {C_ZN14QPlainTextEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::zoomIn(int range);
impl /*struct*/ QPlainTextEdit {
  pub fn zoomIn<RetType, T: QPlainTextEdit_zoomIn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zoomIn(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_zoomIn<RetType> {
  fn zoomIn(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomIn<()> for (i32) {
  fn zoomIn(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN14QPlainTextEdit6zoomInEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
impl /*struct*/ QPlainTextEdit {
  pub fn setMaximumBlockCount<RetType, T: QPlainTextEdit_setMaximumBlockCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumBlockCount(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setMaximumBlockCount<RetType> {
  fn setMaximumBlockCount(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QPlainTextEdit_setMaximumBlockCount<()> for (i32) {
  fn setMaximumBlockCount(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN14QPlainTextEdit20setMaximumBlockCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
impl /*struct*/ QPlainTextEdit {
  pub fn currentCharFormat<RetType, T: QPlainTextEdit_currentCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormat(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_currentCharFormat<RetType> {
  fn currentCharFormat(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QTextCharFormat QPlainTextEdit::currentCharFormat();
impl<'a> /*trait*/ QPlainTextEdit_currentCharFormat<QTextCharFormat> for () {
  fn currentCharFormat(self , rsthis: & QPlainTextEdit) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17currentCharFormatEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit17currentCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setCursorWidth(int width);
impl /*struct*/ QPlainTextEdit {
  pub fn setCursorWidth<RetType, T: QPlainTextEdit_setCursorWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setCursorWidth<RetType> {
  fn setCursorWidth(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextEdit_setCursorWidth<()> for (i32) {
  fn setCursorWidth(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN14QPlainTextEdit14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::documentTitle();
impl /*struct*/ QPlainTextEdit {
  pub fn documentTitle<RetType, T: QPlainTextEdit_documentTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentTitle(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_documentTitle<RetType> {
  fn documentTitle(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::documentTitle();
impl<'a> /*trait*/ QPlainTextEdit_documentTitle<QString> for () {
  fn documentTitle(self , rsthis: & QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13documentTitleEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit13documentTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::selectAll();
impl /*struct*/ QPlainTextEdit {
  pub fn selectAll<RetType, T: QPlainTextEdit_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_selectAll<RetType> {
  fn selectAll(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::selectAll();
impl<'a> /*trait*/ QPlainTextEdit_selectAll<()> for () {
  fn selectAll(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit9selectAllEv()};
     unsafe {C_ZN14QPlainTextEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
impl /*struct*/ QPlainTextEdit {
  pub fn setPlainText<RetType, T: QPlainTextEdit_setPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setPlainText<RetType> {
  fn setPlainText(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_setPlainText<()> for (&'a QString) {
  fn setPlainText(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
impl /*struct*/ QPlainTextEdit {
  pub fn setBackgroundVisible<RetType, T: QPlainTextEdit_setBackgroundVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundVisible(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setBackgroundVisible<RetType> {
  fn setBackgroundVisible(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setBackgroundVisible(bool visible);
impl<'a> /*trait*/ QPlainTextEdit_setBackgroundVisible<()> for (i8) {
  fn setBackgroundVisible(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit20setBackgroundVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN14QPlainTextEdit20setBackgroundVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
impl /*struct*/ QPlainTextEdit {
  pub fn setUndoRedoEnabled<RetType, T: QPlainTextEdit_setUndoRedoEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setUndoRedoEnabled<RetType> {
  fn setUndoRedoEnabled(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QPlainTextEdit_setUndoRedoEnabled<()> for (i8) {
  fn setUndoRedoEnabled(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN14QPlainTextEdit18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::overwriteMode();
impl /*struct*/ QPlainTextEdit {
  pub fn overwriteMode<RetType, T: QPlainTextEdit_overwriteMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overwriteMode(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_overwriteMode<RetType> {
  fn overwriteMode(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::overwriteMode();
impl<'a> /*trait*/ QPlainTextEdit_overwriteMode<i8> for () {
  fn overwriteMode(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit13overwriteModeEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit13overwriteModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::centerCursor();
impl /*struct*/ QPlainTextEdit {
  pub fn centerCursor<RetType, T: QPlainTextEdit_centerCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centerCursor(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_centerCursor<RetType> {
  fn centerCursor(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::centerCursor();
impl<'a> /*trait*/ QPlainTextEdit_centerCursor<()> for () {
  fn centerCursor(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit12centerCursorEv()};
     unsafe {C_ZN14QPlainTextEdit12centerCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
impl /*struct*/ QPlainTextEdit {
  pub fn metaObject<RetType, T: QPlainTextEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  const QMetaObject * QPlainTextEdit::metaObject();
impl<'a> /*trait*/ QPlainTextEdit_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPlainTextEdit) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10metaObjectEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMenu * QPlainTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu<QMenu> for () {
  fn createStandardContextMenu(self , rsthis: & QPlainTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {C_ZN14QPlainTextEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
impl /*struct*/ QPlainTextEdit {
  pub fn setDocumentTitle<RetType, T: QPlainTextEdit_setDocumentTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentTitle(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setDocumentTitle<RetType> {
  fn setDocumentTitle(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QPlainTextEdit_setDocumentTitle<()> for (&'a QString) {
  fn setDocumentTitle(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit16setDocumentTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::~QPlainTextEdit();
impl /*struct*/ QPlainTextEdit {
  pub fn free<RetType, T: QPlainTextEdit_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_free<RetType> {
  fn free(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::~QPlainTextEdit();
impl<'a> /*trait*/ QPlainTextEdit_free<()> for () {
  fn free(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditD2Ev()};
     unsafe {C_ZN14QPlainTextEditD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::clear();
impl /*struct*/ QPlainTextEdit {
  pub fn clear<RetType, T: QPlainTextEdit_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_clear<RetType> {
  fn clear(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::clear();
impl<'a> /*trait*/ QPlainTextEdit_clear<()> for () {
  fn clear(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit5clearEv()};
     unsafe {C_ZN14QPlainTextEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
impl /*struct*/ QPlainTextEdit {
  pub fn anchorAt<RetType, T: QPlainTextEdit_anchorAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.anchorAt(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_anchorAt<RetType> {
  fn anchorAt(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QString QPlainTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_anchorAt<QString> for (&'a QPoint) {
  fn anchorAt(self , rsthis: & QPlainTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK14QPlainTextEdit8anchorAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::canPaste();
impl /*struct*/ QPlainTextEdit {
  pub fn canPaste<RetType, T: QPlainTextEdit_canPaste<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canPaste(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_canPaste<RetType> {
  fn canPaste(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::canPaste();
impl<'a> /*trait*/ QPlainTextEdit_canPaste<i8> for () {
  fn canPaste(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit8canPasteEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit8canPasteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::QPlainTextEdit(QWidget * parent);
impl<'a> /*trait*/ QPlainTextEdit_new for (&'a QWidget) {
  fn new(self) -> QPlainTextEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEditC2EP7QWidget()};
    let ctysz: c_int = unsafe{QPlainTextEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QPlainTextEditC2EP7QWidget(arg0)};
    let rsthis = QPlainTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::cut();
impl /*struct*/ QPlainTextEdit {
  pub fn cut<RetType, T: QPlainTextEdit_cut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cut(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cut<RetType> {
  fn cut(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::cut();
impl<'a> /*trait*/ QPlainTextEdit_cut<()> for () {
  fn cut(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit3cutEv()};
     unsafe {C_ZN14QPlainTextEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
impl /*struct*/ QPlainTextEdit {
  pub fn appendHtml<RetType, T: QPlainTextEdit_appendHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendHtml(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_appendHtml<RetType> {
  fn appendHtml(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::appendHtml(const QString & html);
impl<'a> /*trait*/ QPlainTextEdit_appendHtml<()> for (&'a QString) {
  fn appendHtml(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit10appendHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit10appendHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
impl /*struct*/ QPlainTextEdit {
  pub fn isUndoRedoEnabled<RetType, T: QPlainTextEdit_isUndoRedoEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_isUndoRedoEnabled<RetType> {
  fn isUndoRedoEnabled(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QPlainTextEdit_isUndoRedoEnabled<i8> for () {
  fn isUndoRedoEnabled(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17isUndoRedoEnabledEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::zoomOut(int range);
impl /*struct*/ QPlainTextEdit {
  pub fn zoomOut<RetType, T: QPlainTextEdit_zoomOut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zoomOut(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_zoomOut<RetType> {
  fn zoomOut(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QPlainTextEdit_zoomOut<()> for (i32) {
  fn zoomOut(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN14QPlainTextEdit7zoomOutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
impl /*struct*/ QPlainTextEdit {
  pub fn setPlaceholderText<RetType, T: QPlainTextEdit_setPlaceholderText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_setPlaceholderText<RetType> {
  fn setPlaceholderText(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QPlainTextEdit_setPlaceholderText<()> for (&'a QString) {
  fn setPlaceholderText(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::undo();
impl /*struct*/ QPlainTextEdit {
  pub fn undo<RetType, T: QPlainTextEdit_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_undo<RetType> {
  fn undo(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::undo();
impl<'a> /*trait*/ QPlainTextEdit_undo<()> for () {
  fn undo(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit4undoEv()};
     unsafe {C_ZN14QPlainTextEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
impl /*struct*/ QPlainTextEdit {
  pub fn cursorForPosition<RetType, T: QPlainTextEdit_cursorForPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorForPosition(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorForPosition<RetType> {
  fn cursorForPosition(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  QTextCursor QPlainTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QPlainTextEdit_cursorForPosition<QTextCursor> for (&'a QPoint) {
  fn cursorForPosition(self , rsthis: & QPlainTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPlainTextEdit::centerOnScroll();
impl /*struct*/ QPlainTextEdit {
  pub fn centerOnScroll<RetType, T: QPlainTextEdit_centerOnScroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centerOnScroll(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_centerOnScroll<RetType> {
  fn centerOnScroll(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  bool QPlainTextEdit::centerOnScroll();
impl<'a> /*trait*/ QPlainTextEdit_centerOnScroll<i8> for () {
  fn centerOnScroll(self , rsthis: & QPlainTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit14centerOnScrollEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit14centerOnScrollEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
impl /*struct*/ QPlainTextEdit {
  pub fn appendPlainText<RetType, T: QPlainTextEdit_appendPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendPlainText(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_appendPlainText<RetType> {
  fn appendPlainText(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  void QPlainTextEdit::appendPlainText(const QString & text);
impl<'a> /*trait*/ QPlainTextEdit_appendPlainText<()> for (&'a QString) {
  fn appendPlainText(self , rsthis: & QPlainTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QPlainTextEdit15appendPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QPlainTextEdit15appendPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPlainTextEdit::cursorWidth();
impl /*struct*/ QPlainTextEdit {
  pub fn cursorWidth<RetType, T: QPlainTextEdit_cursorWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextEdit_cursorWidth<RetType> {
  fn cursorWidth(self , rsthis: & QPlainTextEdit) -> RetType;
}

  // proto:  int QPlainTextEdit::cursorWidth();
impl<'a> /*trait*/ QPlainTextEdit_cursorWidth<i32> for () {
  fn cursorWidth(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit11cursorWidthEv()};
    let mut ret = unsafe {C_ZNK14QPlainTextEdit11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QPlainTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QPlainTextEdit_cursorRect<QRect> for (&'a QTextCursor) {
  fn cursorRect(self , rsthis: & QPlainTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QPlainTextEdit_cursorPositionChanged
pub struct QPlainTextEdit_cursorPositionChanged_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn cursorPositionChanged(&self) -> QPlainTextEdit_cursorPositionChanged_signal {
     return QPlainTextEdit_cursorPositionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_cursorPositionChanged_signal {
  pub fn connect<T: QPlainTextEdit_cursorPositionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_cursorPositionChanged_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_cursorPositionChanged_signal);
}

#[derive(Default)] // for QPlainTextEdit_modificationChanged
pub struct QPlainTextEdit_modificationChanged_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn modificationChanged(&self) -> QPlainTextEdit_modificationChanged_signal {
     return QPlainTextEdit_modificationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_modificationChanged_signal {
  pub fn connect<T: QPlainTextEdit_modificationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_modificationChanged_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_modificationChanged_signal);
}

#[derive(Default)] // for QPlainTextEdit_redoAvailable
pub struct QPlainTextEdit_redoAvailable_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn redoAvailable(&self) -> QPlainTextEdit_redoAvailable_signal {
     return QPlainTextEdit_redoAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_redoAvailable_signal {
  pub fn connect<T: QPlainTextEdit_redoAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_redoAvailable_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_redoAvailable_signal);
}

#[derive(Default)] // for QPlainTextEdit_selectionChanged
pub struct QPlainTextEdit_selectionChanged_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn selectionChanged(&self) -> QPlainTextEdit_selectionChanged_signal {
     return QPlainTextEdit_selectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_selectionChanged_signal {
  pub fn connect<T: QPlainTextEdit_selectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_selectionChanged_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_selectionChanged_signal);
}

#[derive(Default)] // for QPlainTextEdit_updateRequest
pub struct QPlainTextEdit_updateRequest_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn updateRequest(&self) -> QPlainTextEdit_updateRequest_signal {
     return QPlainTextEdit_updateRequest_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_updateRequest_signal {
  pub fn connect<T: QPlainTextEdit_updateRequest_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_updateRequest_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_updateRequest_signal);
}

#[derive(Default)] // for QPlainTextEdit_blockCountChanged
pub struct QPlainTextEdit_blockCountChanged_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn blockCountChanged(&self) -> QPlainTextEdit_blockCountChanged_signal {
     return QPlainTextEdit_blockCountChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_blockCountChanged_signal {
  pub fn connect<T: QPlainTextEdit_blockCountChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_blockCountChanged_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_blockCountChanged_signal);
}

#[derive(Default)] // for QPlainTextEdit_undoAvailable
pub struct QPlainTextEdit_undoAvailable_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn undoAvailable(&self) -> QPlainTextEdit_undoAvailable_signal {
     return QPlainTextEdit_undoAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_undoAvailable_signal {
  pub fn connect<T: QPlainTextEdit_undoAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_undoAvailable_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_undoAvailable_signal);
}

#[derive(Default)] // for QPlainTextEdit_textChanged
pub struct QPlainTextEdit_textChanged_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn textChanged(&self) -> QPlainTextEdit_textChanged_signal {
     return QPlainTextEdit_textChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_textChanged_signal {
  pub fn connect<T: QPlainTextEdit_textChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_textChanged_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_textChanged_signal);
}

#[derive(Default)] // for QPlainTextEdit_copyAvailable
pub struct QPlainTextEdit_copyAvailable_signal{poi:u64}
impl /* struct */ QPlainTextEdit {
  pub fn copyAvailable(&self) -> QPlainTextEdit_copyAvailable_signal {
     return QPlainTextEdit_copyAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QPlainTextEdit_copyAvailable_signal {
  pub fn connect<T: QPlainTextEdit_copyAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QPlainTextEdit_copyAvailable_signal_connect {
  fn connect(self, sigthis: QPlainTextEdit_copyAvailable_signal);
}

// blockCountChanged(int)
extern fn QPlainTextEdit_blockCountChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QPlainTextEdit_blockCountChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QPlainTextEdit_blockCountChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QPlainTextEdit_blockCountChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_blockCountChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit17blockCountChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_blockCountChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QPlainTextEdit_blockCountChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_blockCountChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit17blockCountChangedEi(arg0, arg1, arg2)};
  }
}
// undoAvailable(_Bool)
extern fn QPlainTextEdit_undoAvailable_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QPlainTextEdit_undoAvailable_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QPlainTextEdit_undoAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QPlainTextEdit_undoAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_undoAvailable_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13undoAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_undoAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QPlainTextEdit_undoAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_undoAvailable_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13undoAvailableEb(arg0, arg1, arg2)};
  }
}
// selectionChanged()
extern fn QPlainTextEdit_selectionChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QPlainTextEdit_selectionChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QPlainTextEdit_selectionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QPlainTextEdit_selectionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_selectionChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit16selectionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_selectionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QPlainTextEdit_selectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_selectionChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit16selectionChangedEv(arg0, arg1, arg2)};
  }
}
// redoAvailable(_Bool)
extern fn QPlainTextEdit_redoAvailable_signal_connect_cb_3(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QPlainTextEdit_redoAvailable_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QPlainTextEdit_redoAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QPlainTextEdit_redoAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_redoAvailable_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13redoAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_redoAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QPlainTextEdit_redoAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_redoAvailable_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13redoAvailableEb(arg0, arg1, arg2)};
  }
}
// modificationChanged(_Bool)
extern fn QPlainTextEdit_modificationChanged_signal_connect_cb_4(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QPlainTextEdit_modificationChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QPlainTextEdit_modificationChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QPlainTextEdit_modificationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_modificationChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit19modificationChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_modificationChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QPlainTextEdit_modificationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_modificationChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit19modificationChangedEb(arg0, arg1, arg2)};
  }
}
// updateRequest(const class QRect &, int)
extern fn QPlainTextEdit_updateRequest_signal_connect_cb_5(rsfptr:fn(QRect, i32), arg0: *mut c_void, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QPlainTextEdit_updateRequest_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QRect, i32)>, arg0: *mut c_void, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QPlainTextEdit_updateRequest_signal_connect for fn(QRect, i32) {
  fn connect(self, sigthis: QPlainTextEdit_updateRequest_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_updateRequest_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13updateRequestERK5QRecti(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_updateRequest_signal_connect for Box<Fn(QRect, i32)> {
  fn connect(self, sigthis: QPlainTextEdit_updateRequest_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_updateRequest_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13updateRequestERK5QRecti(arg0, arg1, arg2)};
  }
}
// textChanged()
extern fn QPlainTextEdit_textChanged_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QPlainTextEdit_textChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QPlainTextEdit_textChanged_signal_connect for fn() {
  fn connect(self, sigthis: QPlainTextEdit_textChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_textChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit11textChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_textChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QPlainTextEdit_textChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_textChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit11textChangedEv(arg0, arg1, arg2)};
  }
}
// cursorPositionChanged()
extern fn QPlainTextEdit_cursorPositionChanged_signal_connect_cb_7(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QPlainTextEdit_cursorPositionChanged_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QPlainTextEdit_cursorPositionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QPlainTextEdit_cursorPositionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_cursorPositionChanged_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit21cursorPositionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_cursorPositionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QPlainTextEdit_cursorPositionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_cursorPositionChanged_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit21cursorPositionChangedEv(arg0, arg1, arg2)};
  }
}
// copyAvailable(_Bool)
extern fn QPlainTextEdit_copyAvailable_signal_connect_cb_8(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QPlainTextEdit_copyAvailable_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QPlainTextEdit_copyAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QPlainTextEdit_copyAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_copyAvailable_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13copyAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QPlainTextEdit_copyAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QPlainTextEdit_copyAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QPlainTextEdit_copyAvailable_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QPlainTextEdit_SlotProxy_connect__ZN14QPlainTextEdit13copyAvailableEb(arg0, arg1, arg2)};
  }
}
// <= body block end

