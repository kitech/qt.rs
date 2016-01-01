// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qtextedit.h
// dst-file: /src/widgets/qtextedit.rs
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
use super::qabstractscrollarea::QAbstractScrollArea; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qmenu::QMenu; // 773
use super::super::gui::qtextdocument::QTextDocument; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qrect::QRect; // 771
use super::super::gui::qcolor::QColor; // 771
use super::super::gui::qtextformat::QTextCharFormat; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::gui::qtextcursor::QTextCursor; // 771
use super::super::core::qregexp::QRegExp; // 771
use super::super::gui::qfont::QFont; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qurl::QUrl; // 771
use super::super::gui::qpagedpaintdevice::QPagedPaintDevice; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextEdit_Class_Size() -> c_int;
  // proto:  int QTextEdit::lineWrapColumnOrWidth();
  fn _ZNK9QTextEdit21lineWrapColumnOrWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextEdit::setFontFamily(const QString & fontFamily);
  fn _ZN9QTextEdit13setFontFamilyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextEdit::toPlainText();
  fn _ZNK9QTextEdit11toPlainTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::setCursorWidth(int width);
  fn _ZN9QTextEdit14setCursorWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QMenu * QTextEdit::createStandardContextMenu();
  fn _ZN9QTextEdit25createStandardContextMenuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextDocument * QTextEdit::document();
  fn _ZNK9QTextEdit8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QTextEdit::cursorRect();
  fn _ZNK9QTextEdit10cursorRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::setTextColor(const QColor & c);
  fn _ZN9QTextEdit12setTextColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextEdit::acceptRichText();
  fn _ZNK9QTextEdit14acceptRichTextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextEdit::clear();
  fn _ZN9QTextEdit5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::insertHtml(const QString & text);
  fn _ZN9QTextEdit10insertHtmlERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextEdit::fontFamily();
  fn _ZNK9QTextEdit10fontFamilyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::setFontUnderline(bool b);
  fn _ZN9QTextEdit16setFontUnderlineEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextEdit::cut();
  fn _ZN9QTextEdit3cutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
  fn _ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextEdit::anchorAt(const QPoint & pos);
  fn _ZNK9QTextEdit8anchorAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTextEdit::cursorWidth();
  fn _ZNK9QTextEdit11cursorWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextEdit::setTextBackgroundColor(const QColor & c);
  fn _ZN9QTextEdit22setTextBackgroundColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextEdit::tabStopWidth();
  fn _ZNK9QTextEdit12tabStopWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextEdit::setFontWeight(int w);
  fn _ZN9QTextEdit13setFontWeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextEdit::selectAll();
  fn _ZN9QTextEdit9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::zoomOut(int range);
  fn _ZN9QTextEdit7zoomOutEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextEdit::redo();
  fn _ZN9QTextEdit4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::setFontPointSize(qreal s);
  fn _ZN9QTextEdit16setFontPointSizeEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QTextEdit::overwriteMode();
  fn _ZNK9QTextEdit13overwriteModeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QTextCursor QTextEdit::textCursor();
  fn _ZNK9QTextEdit10textCursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
  fn _ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextEdit::setPlainText(const QString & text);
  fn _ZN9QTextEdit12setPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextEdit::placeholderText();
  fn _ZNK9QTextEdit15placeholderTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::~QTextEdit();
  fn _ZN9QTextEditD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextEdit::fontItalic();
  fn _ZNK9QTextEdit10fontItalicEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextEdit::copy();
  fn _ZN9QTextEdit4copyEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::textChanged();
  fn _ZN9QTextEdit11textChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QTextEdit::fontPointSize();
  fn _ZNK9QTextEdit13fontPointSizeEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextEdit::setDocument(QTextDocument * document);
  fn _ZN9QTextEdit11setDocumentEP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextEdit::setOverwriteMode(bool overwrite);
  fn _ZN9QTextEdit16setOverwriteModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextEdit::undo();
  fn _ZN9QTextEdit4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::zoomIn(int range);
  fn _ZN9QTextEdit6zoomInEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextEdit::setDocumentTitle(const QString & title);
  fn demth_ZN9QTextEdit16setDocumentTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextEdit::canPaste();
  fn _ZNK9QTextEdit8canPasteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QTextEdit::toHtml();
  fn _ZNK9QTextEdit6toHtmlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMenu * QTextEdit::createStandardContextMenu(const QPoint & position);
  fn _ZN9QTextEdit25createStandardContextMenuERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::cursorPositionChanged();
  fn _ZN9QTextEdit21cursorPositionChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::setTabStopWidth(int width);
  fn _ZN9QTextEdit15setTabStopWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextEdit::undoAvailable(bool b);
  fn _ZN9QTextEdit13undoAvailableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QTextEdit::documentTitle();
  fn demth_ZNK9QTextEdit13documentTitleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QTextEdit::isUndoRedoEnabled();
  fn demth_ZNK9QTextEdit17isUndoRedoEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextEdit::setText(const QString & text);
  fn _ZN9QTextEdit7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextEdit::ensureCursorVisible();
  fn _ZN9QTextEdit19ensureCursorVisibleEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::setAcceptRichText(bool accept);
  fn _ZN9QTextEdit17setAcceptRichTextEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextEdit::setPlaceholderText(const QString & placeholderText);
  fn _ZN9QTextEdit18setPlaceholderTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextEdit::isReadOnly();
  fn _ZNK9QTextEdit10isReadOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextEdit::setUndoRedoEnabled(bool enable);
  fn demth_ZN9QTextEdit18setUndoRedoEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextEdit::QTextEdit(const QTextEdit & );
  fn dector_ZN9QTextEditC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QTextEditC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextCharFormat QTextEdit::currentCharFormat();
  fn _ZNK9QTextEdit17currentCharFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
  fn _ZNK9QTextEdit17cursorForPositionERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::scrollToAnchor(const QString & name);
  fn _ZN9QTextEdit14scrollToAnchorERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QFont QTextEdit::currentFont();
  fn _ZNK9QTextEdit11currentFontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::paste();
  fn _ZN9QTextEdit5pasteEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::setTextCursor(const QTextCursor & cursor);
  fn _ZN9QTextEdit13setTextCursorERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
  fn _ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVariant QTextEdit::loadResource(int type, const QUrl & name);
  fn _ZN9QTextEdit12loadResourceEiRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setTabChangesFocus(bool b);
  fn _ZN9QTextEdit18setTabChangesFocusEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextEdit::selectionChanged();
  fn _ZN9QTextEdit16selectionChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::setHtml(const QString & text);
  fn _ZN9QTextEdit7setHtmlERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QTextEdit::cursorRect(const QTextCursor & cursor);
  fn _ZNK9QTextEdit10cursorRectERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextEdit::setLineWrapColumnOrWidth(int w);
  fn _ZN9QTextEdit24setLineWrapColumnOrWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextEdit::setFontItalic(bool b);
  fn _ZN9QTextEdit13setFontItalicEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QTextEdit::metaObject();
  fn _ZNK9QTextEdit10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextEdit::setCurrentFont(const QFont & f);
  fn _ZN9QTextEdit14setCurrentFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextEdit::tabChangesFocus();
  fn _ZNK9QTextEdit15tabChangesFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QColor QTextEdit::textBackgroundColor();
  fn _ZNK9QTextEdit19textBackgroundColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::QTextEdit(const QString & text, QWidget * parent);
  fn dector_ZN9QTextEditC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QTextEditC1ERK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QTextEdit::print(QPagedPaintDevice * printer);
  fn _ZNK9QTextEdit5printEP17QPagedPaintDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextEdit::fontUnderline();
  fn _ZNK9QTextEdit13fontUnderlineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextEdit::insertPlainText(const QString & text);
  fn _ZN9QTextEdit15insertPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextEdit::fontWeight();
  fn _ZNK9QTextEdit10fontWeightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextEdit::copyAvailable(bool b);
  fn _ZN9QTextEdit13copyAvailableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QColor QTextEdit::textColor();
  fn _ZNK9QTextEdit9textColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextEdit::append(const QString & text);
  fn _ZN9QTextEdit6appendERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextEdit::redoAvailable(bool b);
  fn _ZN9QTextEdit13redoAvailableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextEdit::QTextEdit(QWidget * parent);
  fn dector_ZN9QTextEditC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QTextEditC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextEdit::setReadOnly(bool ro);
  fn _ZN9QTextEdit11setReadOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit13undoAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit13redoAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit13copyAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit21cursorPositionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit16selectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextEdit_SlotProxy_connect__ZN9QTextEdit11textChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTextEdit)=1
#[derive(Default)]
pub struct QTextEdit {
  qbase: QAbstractScrollArea,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _cursorPositionChanged_1: QTextEdit_cursorPositionChanged_signal,
  pub _redoAvailable_1: QTextEdit_redoAvailable_signal,
  pub _selectionChanged_1: QTextEdit_selectionChanged_signal,
  pub _currentCharFormatChanged_1: QTextEdit_currentCharFormatChanged_signal,
  pub _undoAvailable_1: QTextEdit_undoAvailable_signal,
  pub _textChanged_1: QTextEdit_textChanged_signal,
  pub _copyAvailable_1: QTextEdit_copyAvailable_signal,
}

impl /*struct*/ QTextEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextEdit {
    return QTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextEdit {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QTextEdit {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  int QTextEdit::lineWrapColumnOrWidth();
impl /*struct*/ QTextEdit {
  pub fn lineWrapColumnOrWidth<RetType, T: QTextEdit_lineWrapColumnOrWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineWrapColumnOrWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_lineWrapColumnOrWidth<RetType> {
  fn lineWrapColumnOrWidth(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  int QTextEdit::lineWrapColumnOrWidth();
impl<'a> /*trait*/ QTextEdit_lineWrapColumnOrWidth<i32> for () {
  fn lineWrapColumnOrWidth(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit21lineWrapColumnOrWidthEv()};
    let mut ret = unsafe {_ZNK9QTextEdit21lineWrapColumnOrWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextEdit::setFontFamily(const QString & fontFamily);
impl /*struct*/ QTextEdit {
  pub fn setFontFamily<RetType, T: QTextEdit_setFontFamily<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFontFamily(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontFamily<RetType> {
  fn setFontFamily(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setFontFamily(const QString & fontFamily);
impl<'a> /*trait*/ QTextEdit_setFontFamily<()> for (&'a QString) {
  fn setFontFamily(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit13setFontFamilyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextEdit::toPlainText();
impl /*struct*/ QTextEdit {
  pub fn toPlainText<RetType, T: QTextEdit_toPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QTextEdit_toPlainText<RetType> {
  fn toPlainText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QString QTextEdit::toPlainText();
impl<'a> /*trait*/ QTextEdit_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: & QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11toPlainTextEv()};
    let mut ret = unsafe {_ZNK9QTextEdit11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::setCursorWidth(int width);
impl /*struct*/ QTextEdit {
  pub fn setCursorWidth<RetType, T: QTextEdit_setCursorWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_setCursorWidth<RetType> {
  fn setCursorWidth(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setCursorWidth(int width);
impl<'a> /*trait*/ QTextEdit_setCursorWidth<()> for (i32) {
  fn setCursorWidth(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMenu * QTextEdit::createStandardContextMenu();
impl /*struct*/ QTextEdit {
  pub fn createStandardContextMenu<RetType, T: QTextEdit_createStandardContextMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu(self);
    // return 1;
  }
}

pub trait QTextEdit_createStandardContextMenu<RetType> {
  fn createStandardContextMenu(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QMenu * QTextEdit::createStandardContextMenu();
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu<QMenu> for () {
  fn createStandardContextMenu(self , rsthis: & QTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit25createStandardContextMenuEv()};
    let mut ret = unsafe {_ZN9QTextEdit25createStandardContextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextDocument * QTextEdit::document();
impl /*struct*/ QTextEdit {
  pub fn document<RetType, T: QTextEdit_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QTextEdit_document<RetType> {
  fn document(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QTextDocument * QTextEdit::document();
impl<'a> /*trait*/ QTextEdit_document<QTextDocument> for () {
  fn document(self , rsthis: & QTextEdit) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8documentEv()};
    let mut ret = unsafe {_ZNK9QTextEdit8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QTextEdit::cursorRect();
impl /*struct*/ QTextEdit {
  pub fn cursorRect<RetType, T: QTextEdit_cursorRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorRect(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorRect<RetType> {
  fn cursorRect(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QRect QTextEdit::cursorRect();
impl<'a> /*trait*/ QTextEdit_cursorRect<QRect> for () {
  fn cursorRect(self , rsthis: & QTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10cursorRectEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10cursorRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::setTextColor(const QColor & c);
impl /*struct*/ QTextEdit {
  pub fn setTextColor<RetType, T: QTextEdit_setTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextColor(self);
    // return 1;
  }
}

pub trait QTextEdit_setTextColor<RetType> {
  fn setTextColor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setTextColor(const QColor & c);
impl<'a> /*trait*/ QTextEdit_setTextColor<()> for (&'a QColor) {
  fn setTextColor(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::acceptRichText();
impl /*struct*/ QTextEdit {
  pub fn acceptRichText<RetType, T: QTextEdit_acceptRichText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptRichText(self);
    // return 1;
  }
}

pub trait QTextEdit_acceptRichText<RetType> {
  fn acceptRichText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::acceptRichText();
impl<'a> /*trait*/ QTextEdit_acceptRichText<i8> for () {
  fn acceptRichText(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit14acceptRichTextEv()};
    let mut ret = unsafe {_ZNK9QTextEdit14acceptRichTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextEdit::clear();
impl /*struct*/ QTextEdit {
  pub fn clear<RetType, T: QTextEdit_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTextEdit_clear<RetType> {
  fn clear(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::clear();
impl<'a> /*trait*/ QTextEdit_clear<()> for () {
  fn clear(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit5clearEv()};
     unsafe {_ZN9QTextEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::insertHtml(const QString & text);
impl /*struct*/ QTextEdit {
  pub fn insertHtml<RetType, T: QTextEdit_insertHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertHtml(self);
    // return 1;
  }
}

pub trait QTextEdit_insertHtml<RetType> {
  fn insertHtml(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::insertHtml(const QString & text);
impl<'a> /*trait*/ QTextEdit_insertHtml<()> for (&'a QString) {
  fn insertHtml(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit10insertHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit10insertHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextEdit::fontFamily();
impl /*struct*/ QTextEdit {
  pub fn fontFamily<RetType, T: QTextEdit_fontFamily<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontFamily(self);
    // return 1;
  }
}

pub trait QTextEdit_fontFamily<RetType> {
  fn fontFamily(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QString QTextEdit::fontFamily();
impl<'a> /*trait*/ QTextEdit_fontFamily<QString> for () {
  fn fontFamily(self , rsthis: & QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontFamilyEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10fontFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::setFontUnderline(bool b);
impl /*struct*/ QTextEdit {
  pub fn setFontUnderline<RetType, T: QTextEdit_setFontUnderline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFontUnderline(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontUnderline<RetType> {
  fn setFontUnderline(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setFontUnderline(bool b);
impl<'a> /*trait*/ QTextEdit_setFontUnderline<()> for (i8) {
  fn setFontUnderline(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setFontUnderlineEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit16setFontUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::cut();
impl /*struct*/ QTextEdit {
  pub fn cut<RetType, T: QTextEdit_cut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cut(self);
    // return 1;
  }
}

pub trait QTextEdit_cut<RetType> {
  fn cut(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::cut();
impl<'a> /*trait*/ QTextEdit_cut<()> for () {
  fn cut(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit3cutEv()};
     unsafe {_ZN9QTextEdit3cutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
impl /*struct*/ QTextEdit {
  pub fn currentCharFormatChanged<RetType, T: QTextEdit_currentCharFormatChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormatChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_currentCharFormatChanged<RetType> {
  fn currentCharFormatChanged(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::currentCharFormatChanged(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextEdit_currentCharFormatChanged<()> for (&'a QTextCharFormat) {
  fn currentCharFormatChanged(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextEdit::anchorAt(const QPoint & pos);
impl /*struct*/ QTextEdit {
  pub fn anchorAt<RetType, T: QTextEdit_anchorAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.anchorAt(self);
    // return 1;
  }
}

pub trait QTextEdit_anchorAt<RetType> {
  fn anchorAt(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QString QTextEdit::anchorAt(const QPoint & pos);
impl<'a> /*trait*/ QTextEdit_anchorAt<QString> for (&'a QPoint) {
  fn anchorAt(self , rsthis: & QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8anchorAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextEdit8anchorAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextEdit::cursorWidth();
impl /*struct*/ QTextEdit {
  pub fn cursorWidth<RetType, T: QTextEdit_cursorWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorWidth<RetType> {
  fn cursorWidth(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  int QTextEdit::cursorWidth();
impl<'a> /*trait*/ QTextEdit_cursorWidth<i32> for () {
  fn cursorWidth(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11cursorWidthEv()};
    let mut ret = unsafe {_ZNK9QTextEdit11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextEdit::setTextBackgroundColor(const QColor & c);
impl /*struct*/ QTextEdit {
  pub fn setTextBackgroundColor<RetType, T: QTextEdit_setTextBackgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextBackgroundColor(self);
    // return 1;
  }
}

pub trait QTextEdit_setTextBackgroundColor<RetType> {
  fn setTextBackgroundColor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setTextBackgroundColor(const QColor & c);
impl<'a> /*trait*/ QTextEdit_setTextBackgroundColor<()> for (&'a QColor) {
  fn setTextBackgroundColor(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit22setTextBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit22setTextBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextEdit::tabStopWidth();
impl /*struct*/ QTextEdit {
  pub fn tabStopWidth<RetType, T: QTextEdit_tabStopWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabStopWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_tabStopWidth<RetType> {
  fn tabStopWidth(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  int QTextEdit::tabStopWidth();
impl<'a> /*trait*/ QTextEdit_tabStopWidth<i32> for () {
  fn tabStopWidth(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit12tabStopWidthEv()};
    let mut ret = unsafe {_ZNK9QTextEdit12tabStopWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextEdit::setFontWeight(int w);
impl /*struct*/ QTextEdit {
  pub fn setFontWeight<RetType, T: QTextEdit_setFontWeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFontWeight(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontWeight<RetType> {
  fn setFontWeight(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setFontWeight(int w);
impl<'a> /*trait*/ QTextEdit_setFontWeight<()> for (i32) {
  fn setFontWeight(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit13setFontWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::selectAll();
impl /*struct*/ QTextEdit {
  pub fn selectAll<RetType, T: QTextEdit_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QTextEdit_selectAll<RetType> {
  fn selectAll(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::selectAll();
impl<'a> /*trait*/ QTextEdit_selectAll<()> for () {
  fn selectAll(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit9selectAllEv()};
     unsafe {_ZN9QTextEdit9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::zoomOut(int range);
impl /*struct*/ QTextEdit {
  pub fn zoomOut<RetType, T: QTextEdit_zoomOut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zoomOut(self);
    // return 1;
  }
}

pub trait QTextEdit_zoomOut<RetType> {
  fn zoomOut(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::zoomOut(int range);
impl<'a> /*trait*/ QTextEdit_zoomOut<()> for (i32) {
  fn zoomOut(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7zoomOutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit7zoomOutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::redo();
impl /*struct*/ QTextEdit {
  pub fn redo<RetType, T: QTextEdit_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QTextEdit_redo<RetType> {
  fn redo(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::redo();
impl<'a> /*trait*/ QTextEdit_redo<()> for () {
  fn redo(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4redoEv()};
     unsafe {_ZN9QTextEdit4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setFontPointSize(qreal s);
impl /*struct*/ QTextEdit {
  pub fn setFontPointSize<RetType, T: QTextEdit_setFontPointSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFontPointSize(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontPointSize<RetType> {
  fn setFontPointSize(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setFontPointSize(qreal s);
impl<'a> /*trait*/ QTextEdit_setFontPointSize<()> for (f64) {
  fn setFontPointSize(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setFontPointSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QTextEdit16setFontPointSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::overwriteMode();
impl /*struct*/ QTextEdit {
  pub fn overwriteMode<RetType, T: QTextEdit_overwriteMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overwriteMode(self);
    // return 1;
  }
}

pub trait QTextEdit_overwriteMode<RetType> {
  fn overwriteMode(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::overwriteMode();
impl<'a> /*trait*/ QTextEdit_overwriteMode<i8> for () {
  fn overwriteMode(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13overwriteModeEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13overwriteModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextCursor QTextEdit::textCursor();
impl /*struct*/ QTextEdit {
  pub fn textCursor<RetType, T: QTextEdit_textCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textCursor(self);
    // return 1;
  }
}

pub trait QTextEdit_textCursor<RetType> {
  fn textCursor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QTextCursor QTextEdit::textCursor();
impl<'a> /*trait*/ QTextEdit_textCursor<QTextCursor> for () {
  fn textCursor(self , rsthis: & QTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10textCursorEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl /*struct*/ QTextEdit {
  pub fn mergeCurrentCharFormat<RetType, T: QTextEdit_mergeCurrentCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QTextEdit_mergeCurrentCharFormat<RetType> {
  fn mergeCurrentCharFormat(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextEdit_mergeCurrentCharFormat<()> for (&'a QTextCharFormat) {
  fn mergeCurrentCharFormat(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setPlainText(const QString & text);
impl /*struct*/ QTextEdit {
  pub fn setPlainText<RetType, T: QTextEdit_setPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlainText(self);
    // return 1;
  }
}

pub trait QTextEdit_setPlainText<RetType> {
  fn setPlainText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextEdit_setPlainText<()> for (&'a QString) {
  fn setPlainText(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextEdit::placeholderText();
impl /*struct*/ QTextEdit {
  pub fn placeholderText<RetType, T: QTextEdit_placeholderText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.placeholderText(self);
    // return 1;
  }
}

pub trait QTextEdit_placeholderText<RetType> {
  fn placeholderText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QString QTextEdit::placeholderText();
impl<'a> /*trait*/ QTextEdit_placeholderText<QString> for () {
  fn placeholderText(self , rsthis: & QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit15placeholderTextEv()};
    let mut ret = unsafe {_ZNK9QTextEdit15placeholderTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::~QTextEdit();
impl /*struct*/ QTextEdit {
  pub fn free<RetType, T: QTextEdit_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextEdit_free<RetType> {
  fn free(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::~QTextEdit();
impl<'a> /*trait*/ QTextEdit_free<()> for () {
  fn free(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditD0Ev()};
     unsafe {_ZN9QTextEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::fontItalic();
impl /*struct*/ QTextEdit {
  pub fn fontItalic<RetType, T: QTextEdit_fontItalic<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontItalic(self);
    // return 1;
  }
}

pub trait QTextEdit_fontItalic<RetType> {
  fn fontItalic(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::fontItalic();
impl<'a> /*trait*/ QTextEdit_fontItalic<i8> for () {
  fn fontItalic(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontItalicEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10fontItalicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextEdit::copy();
impl /*struct*/ QTextEdit {
  pub fn copy<RetType, T: QTextEdit_copy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QTextEdit_copy<RetType> {
  fn copy(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::copy();
impl<'a> /*trait*/ QTextEdit_copy<()> for () {
  fn copy(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4copyEv()};
     unsafe {_ZN9QTextEdit4copyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::textChanged();
impl /*struct*/ QTextEdit {
  pub fn textChanged<RetType, T: QTextEdit_textChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_textChanged<RetType> {
  fn textChanged(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::textChanged();
impl<'a> /*trait*/ QTextEdit_textChanged<()> for () {
  fn textChanged(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11textChangedEv()};
     unsafe {_ZN9QTextEdit11textChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextEdit::fontPointSize();
impl /*struct*/ QTextEdit {
  pub fn fontPointSize<RetType, T: QTextEdit_fontPointSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontPointSize(self);
    // return 1;
  }
}

pub trait QTextEdit_fontPointSize<RetType> {
  fn fontPointSize(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  qreal QTextEdit::fontPointSize();
impl<'a> /*trait*/ QTextEdit_fontPointSize<f64> for () {
  fn fontPointSize(self , rsthis: & QTextEdit) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13fontPointSizeEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13fontPointSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextEdit::setDocument(QTextDocument * document);
impl /*struct*/ QTextEdit {
  pub fn setDocument<RetType, T: QTextEdit_setDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocument(self);
    // return 1;
  }
}

pub trait QTextEdit_setDocument<RetType> {
  fn setDocument(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QTextEdit_setDocument<()> for (&'a QTextDocument) {
  fn setDocument(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setOverwriteMode(bool overwrite);
impl /*struct*/ QTextEdit {
  pub fn setOverwriteMode<RetType, T: QTextEdit_setOverwriteMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOverwriteMode(self);
    // return 1;
  }
}

pub trait QTextEdit_setOverwriteMode<RetType> {
  fn setOverwriteMode(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QTextEdit_setOverwriteMode<()> for (i8) {
  fn setOverwriteMode(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setOverwriteModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit16setOverwriteModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::undo();
impl /*struct*/ QTextEdit {
  pub fn undo<RetType, T: QTextEdit_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QTextEdit_undo<RetType> {
  fn undo(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::undo();
impl<'a> /*trait*/ QTextEdit_undo<()> for () {
  fn undo(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit4undoEv()};
     unsafe {_ZN9QTextEdit4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::zoomIn(int range);
impl /*struct*/ QTextEdit {
  pub fn zoomIn<RetType, T: QTextEdit_zoomIn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zoomIn(self);
    // return 1;
  }
}

pub trait QTextEdit_zoomIn<RetType> {
  fn zoomIn(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::zoomIn(int range);
impl<'a> /*trait*/ QTextEdit_zoomIn<()> for (i32) {
  fn zoomIn(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit6zoomInEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit6zoomInEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setDocumentTitle(const QString & title);
impl /*struct*/ QTextEdit {
  pub fn setDocumentTitle<RetType, T: QTextEdit_setDocumentTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentTitle(self);
    // return 1;
  }
}

pub trait QTextEdit_setDocumentTitle<RetType> {
  fn setDocumentTitle(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setDocumentTitle(const QString & title);
impl<'a> /*trait*/ QTextEdit_setDocumentTitle<()> for (&'a QString) {
  fn setDocumentTitle(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16setDocumentTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN9QTextEdit16setDocumentTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::canPaste();
impl /*struct*/ QTextEdit {
  pub fn canPaste<RetType, T: QTextEdit_canPaste<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canPaste(self);
    // return 1;
  }
}

pub trait QTextEdit_canPaste<RetType> {
  fn canPaste(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::canPaste();
impl<'a> /*trait*/ QTextEdit_canPaste<i8> for () {
  fn canPaste(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit8canPasteEv()};
    let mut ret = unsafe {_ZNK9QTextEdit8canPasteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QTextEdit::toHtml();
impl /*struct*/ QTextEdit {
  pub fn toHtml<RetType, T: QTextEdit_toHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toHtml(self);
    // return 1;
  }
}

pub trait QTextEdit_toHtml<RetType> {
  fn toHtml(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QString QTextEdit::toHtml();
impl<'a> /*trait*/ QTextEdit_toHtml<QString> for () {
  fn toHtml(self , rsthis: & QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit6toHtmlEv()};
    let mut ret = unsafe {_ZNK9QTextEdit6toHtmlEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMenu * QTextEdit::createStandardContextMenu(const QPoint & position);
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu<QMenu> for (&'a QPoint) {
  fn createStandardContextMenu(self , rsthis: & QTextEdit) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit25createStandardContextMenuERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTextEdit25createStandardContextMenuERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::cursorPositionChanged();
impl /*struct*/ QTextEdit {
  pub fn cursorPositionChanged<RetType, T: QTextEdit_cursorPositionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorPositionChanged<RetType> {
  fn cursorPositionChanged(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::cursorPositionChanged();
impl<'a> /*trait*/ QTextEdit_cursorPositionChanged<()> for () {
  fn cursorPositionChanged(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit21cursorPositionChangedEv()};
     unsafe {_ZN9QTextEdit21cursorPositionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setTabStopWidth(int width);
impl /*struct*/ QTextEdit {
  pub fn setTabStopWidth<RetType, T: QTextEdit_setTabStopWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabStopWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_setTabStopWidth<RetType> {
  fn setTabStopWidth(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setTabStopWidth(int width);
impl<'a> /*trait*/ QTextEdit_setTabStopWidth<()> for (i32) {
  fn setTabStopWidth(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit15setTabStopWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit15setTabStopWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::undoAvailable(bool b);
impl /*struct*/ QTextEdit {
  pub fn undoAvailable<RetType, T: QTextEdit_undoAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undoAvailable(self);
    // return 1;
  }
}

pub trait QTextEdit_undoAvailable<RetType> {
  fn undoAvailable(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::undoAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_undoAvailable<()> for (i8) {
  fn undoAvailable(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13undoAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit13undoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextEdit::documentTitle();
impl /*struct*/ QTextEdit {
  pub fn documentTitle<RetType, T: QTextEdit_documentTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentTitle(self);
    // return 1;
  }
}

pub trait QTextEdit_documentTitle<RetType> {
  fn documentTitle(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QString QTextEdit::documentTitle();
impl<'a> /*trait*/ QTextEdit_documentTitle<QString> for () {
  fn documentTitle(self , rsthis: & QTextEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13documentTitleEv()};
    let mut ret = unsafe {demth_ZNK9QTextEdit13documentTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextEdit::isUndoRedoEnabled();
impl /*struct*/ QTextEdit {
  pub fn isUndoRedoEnabled<RetType, T: QTextEdit_isUndoRedoEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextEdit_isUndoRedoEnabled<RetType> {
  fn isUndoRedoEnabled(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextEdit_isUndoRedoEnabled<i8> for () {
  fn isUndoRedoEnabled(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17isUndoRedoEnabledEv()};
    let mut ret = unsafe {demth_ZNK9QTextEdit17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextEdit::setText(const QString & text);
impl /*struct*/ QTextEdit {
  pub fn setText<RetType, T: QTextEdit_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QTextEdit_setText<RetType> {
  fn setText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setText(const QString & text);
impl<'a> /*trait*/ QTextEdit_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::ensureCursorVisible();
impl /*struct*/ QTextEdit {
  pub fn ensureCursorVisible<RetType, T: QTextEdit_ensureCursorVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureCursorVisible(self);
    // return 1;
  }
}

pub trait QTextEdit_ensureCursorVisible<RetType> {
  fn ensureCursorVisible(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::ensureCursorVisible();
impl<'a> /*trait*/ QTextEdit_ensureCursorVisible<()> for () {
  fn ensureCursorVisible(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit19ensureCursorVisibleEv()};
     unsafe {_ZN9QTextEdit19ensureCursorVisibleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setAcceptRichText(bool accept);
impl /*struct*/ QTextEdit {
  pub fn setAcceptRichText<RetType, T: QTextEdit_setAcceptRichText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceptRichText(self);
    // return 1;
  }
}

pub trait QTextEdit_setAcceptRichText<RetType> {
  fn setAcceptRichText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setAcceptRichText(bool accept);
impl<'a> /*trait*/ QTextEdit_setAcceptRichText<()> for (i8) {
  fn setAcceptRichText(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit17setAcceptRichTextEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit17setAcceptRichTextEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setPlaceholderText(const QString & placeholderText);
impl /*struct*/ QTextEdit {
  pub fn setPlaceholderText<RetType, T: QTextEdit_setPlaceholderText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText(self);
    // return 1;
  }
}

pub trait QTextEdit_setPlaceholderText<RetType> {
  fn setPlaceholderText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setPlaceholderText(const QString & placeholderText);
impl<'a> /*trait*/ QTextEdit_setPlaceholderText<()> for (&'a QString) {
  fn setPlaceholderText(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setPlaceholderTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit18setPlaceholderTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::isReadOnly();
impl /*struct*/ QTextEdit {
  pub fn isReadOnly<RetType, T: QTextEdit_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QTextEdit_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::isReadOnly();
impl<'a> /*trait*/ QTextEdit_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextEdit::setUndoRedoEnabled(bool enable);
impl /*struct*/ QTextEdit {
  pub fn setUndoRedoEnabled<RetType, T: QTextEdit_setUndoRedoEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextEdit_setUndoRedoEnabled<RetType> {
  fn setUndoRedoEnabled(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextEdit_setUndoRedoEnabled<()> for (i8) {
  fn setUndoRedoEnabled(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setUndoRedoEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {demth_ZN9QTextEdit18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::QTextEdit(const QTextEdit & );
impl /*struct*/ QTextEdit {
  pub fn new<T: QTextEdit_new>(value: T) -> QTextEdit {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEdit_new {
  fn new(self) -> QTextEdit;
}

  // proto:  void QTextEdit::QTextEdit(const QTextEdit & );
impl<'a> /*trait*/ QTextEdit_new for (&'a QTextEdit) {
  fn new(self) -> QTextEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1ERKS_()};
    let ctysz: c_int = unsafe{QTextEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTextEditC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QTextEditC1ERKS_(arg0)} as u64;
    let rsthis = QTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextEdit::currentCharFormat();
impl /*struct*/ QTextEdit {
  pub fn currentCharFormat<RetType, T: QTextEdit_currentCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormat(self);
    // return 1;
  }
}

pub trait QTextEdit_currentCharFormat<RetType> {
  fn currentCharFormat(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QTextCharFormat QTextEdit::currentCharFormat();
impl<'a> /*trait*/ QTextEdit_currentCharFormat<QTextCharFormat> for () {
  fn currentCharFormat(self , rsthis: & QTextEdit) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17currentCharFormatEv()};
    let mut ret = unsafe {_ZNK9QTextEdit17currentCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
impl /*struct*/ QTextEdit {
  pub fn cursorForPosition<RetType, T: QTextEdit_cursorForPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorForPosition(self);
    // return 1;
  }
}

pub trait QTextEdit_cursorForPosition<RetType> {
  fn cursorForPosition(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QTextCursor QTextEdit::cursorForPosition(const QPoint & pos);
impl<'a> /*trait*/ QTextEdit_cursorForPosition<QTextCursor> for (&'a QPoint) {
  fn cursorForPosition(self , rsthis: & QTextEdit) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit17cursorForPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextEdit17cursorForPositionERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::scrollToAnchor(const QString & name);
impl /*struct*/ QTextEdit {
  pub fn scrollToAnchor<RetType, T: QTextEdit_scrollToAnchor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scrollToAnchor(self);
    // return 1;
  }
}

pub trait QTextEdit_scrollToAnchor<RetType> {
  fn scrollToAnchor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::scrollToAnchor(const QString & name);
impl<'a> /*trait*/ QTextEdit_scrollToAnchor<()> for (&'a QString) {
  fn scrollToAnchor(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14scrollToAnchorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit14scrollToAnchorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QTextEdit::currentFont();
impl /*struct*/ QTextEdit {
  pub fn currentFont<RetType, T: QTextEdit_currentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFont(self);
    // return 1;
  }
}

pub trait QTextEdit_currentFont<RetType> {
  fn currentFont(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QFont QTextEdit::currentFont();
impl<'a> /*trait*/ QTextEdit_currentFont<QFont> for () {
  fn currentFont(self , rsthis: & QTextEdit) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit11currentFontEv()};
    let mut ret = unsafe {_ZNK9QTextEdit11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::paste();
impl /*struct*/ QTextEdit {
  pub fn paste<RetType, T: QTextEdit_paste<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paste(self);
    // return 1;
  }
}

pub trait QTextEdit_paste<RetType> {
  fn paste(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::paste();
impl<'a> /*trait*/ QTextEdit_paste<()> for () {
  fn paste(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit5pasteEv()};
     unsafe {_ZN9QTextEdit5pasteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setTextCursor(const QTextCursor & cursor);
impl /*struct*/ QTextEdit {
  pub fn setTextCursor<RetType, T: QTextEdit_setTextCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor(self);
    // return 1;
  }
}

pub trait QTextEdit_setTextCursor<RetType> {
  fn setTextCursor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextEdit_setTextCursor<()> for (&'a QTextCursor) {
  fn setTextCursor(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl /*struct*/ QTextEdit {
  pub fn setCurrentCharFormat<RetType, T: QTextEdit_setCurrentCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCharFormat(self);
    // return 1;
  }
}

pub trait QTextEdit_setCurrentCharFormat<RetType> {
  fn setCurrentCharFormat(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setCurrentCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextEdit_setCurrentCharFormat<()> for (&'a QTextCharFormat) {
  fn setCurrentCharFormat(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QTextEdit::loadResource(int type, const QUrl & name);
impl /*struct*/ QTextEdit {
  pub fn loadResource<RetType, T: QTextEdit_loadResource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadResource(self);
    // return 1;
  }
}

pub trait QTextEdit_loadResource<RetType> {
  fn loadResource(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QVariant QTextEdit::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextEdit_loadResource<QVariant> for (i32, &'a QUrl) {
  fn loadResource(self , rsthis: & QTextEdit) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QTextEdit12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::setTabChangesFocus(bool b);
impl /*struct*/ QTextEdit {
  pub fn setTabChangesFocus<RetType, T: QTextEdit_setTabChangesFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QTextEdit_setTabChangesFocus<RetType> {
  fn setTabChangesFocus(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QTextEdit_setTabChangesFocus<()> for (i8) {
  fn setTabChangesFocus(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit18setTabChangesFocusEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::selectionChanged();
impl /*struct*/ QTextEdit {
  pub fn selectionChanged<RetType, T: QTextEdit_selectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QTextEdit_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::selectionChanged();
impl<'a> /*trait*/ QTextEdit_selectionChanged<()> for () {
  fn selectionChanged(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit16selectionChangedEv()};
     unsafe {_ZN9QTextEdit16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setHtml(const QString & text);
impl /*struct*/ QTextEdit {
  pub fn setHtml<RetType, T: QTextEdit_setHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHtml(self);
    // return 1;
  }
}

pub trait QTextEdit_setHtml<RetType> {
  fn setHtml(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setHtml(const QString & text);
impl<'a> /*trait*/ QTextEdit_setHtml<()> for (&'a QString) {
  fn setHtml(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QTextEdit::cursorRect(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextEdit_cursorRect<QRect> for (&'a QTextCursor) {
  fn cursorRect(self , rsthis: & QTextEdit) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10cursorRectERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextEdit10cursorRectERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::setLineWrapColumnOrWidth(int w);
impl /*struct*/ QTextEdit {
  pub fn setLineWrapColumnOrWidth<RetType, T: QTextEdit_setLineWrapColumnOrWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineWrapColumnOrWidth(self);
    // return 1;
  }
}

pub trait QTextEdit_setLineWrapColumnOrWidth<RetType> {
  fn setLineWrapColumnOrWidth(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setLineWrapColumnOrWidth(int w);
impl<'a> /*trait*/ QTextEdit_setLineWrapColumnOrWidth<()> for (i32) {
  fn setLineWrapColumnOrWidth(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit24setLineWrapColumnOrWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextEdit24setLineWrapColumnOrWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setFontItalic(bool b);
impl /*struct*/ QTextEdit {
  pub fn setFontItalic<RetType, T: QTextEdit_setFontItalic<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFontItalic(self);
    // return 1;
  }
}

pub trait QTextEdit_setFontItalic<RetType> {
  fn setFontItalic(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setFontItalic(bool b);
impl<'a> /*trait*/ QTextEdit_setFontItalic<()> for (i8) {
  fn setFontItalic(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13setFontItalicEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit13setFontItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextEdit::metaObject();
impl /*struct*/ QTextEdit {
  pub fn metaObject<RetType, T: QTextEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  const QMetaObject * QTextEdit::metaObject();
impl<'a> /*trait*/ QTextEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10metaObjectEv()};
     unsafe {_ZNK9QTextEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextEdit::setCurrentFont(const QFont & f);
impl /*struct*/ QTextEdit {
  pub fn setCurrentFont<RetType, T: QTextEdit_setCurrentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont(self);
    // return 1;
  }
}

pub trait QTextEdit_setCurrentFont<RetType> {
  fn setCurrentFont(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setCurrentFont(const QFont & f);
impl<'a> /*trait*/ QTextEdit_setCurrentFont<()> for (&'a QFont) {
  fn setCurrentFont(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::tabChangesFocus();
impl /*struct*/ QTextEdit {
  pub fn tabChangesFocus<RetType, T: QTextEdit_tabChangesFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QTextEdit_tabChangesFocus<RetType> {
  fn tabChangesFocus(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::tabChangesFocus();
impl<'a> /*trait*/ QTextEdit_tabChangesFocus<i8> for () {
  fn tabChangesFocus(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit15tabChangesFocusEv()};
    let mut ret = unsafe {_ZNK9QTextEdit15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QColor QTextEdit::textBackgroundColor();
impl /*struct*/ QTextEdit {
  pub fn textBackgroundColor<RetType, T: QTextEdit_textBackgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textBackgroundColor(self);
    // return 1;
  }
}

pub trait QTextEdit_textBackgroundColor<RetType> {
  fn textBackgroundColor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QColor QTextEdit::textBackgroundColor();
impl<'a> /*trait*/ QTextEdit_textBackgroundColor<QColor> for () {
  fn textBackgroundColor(self , rsthis: & QTextEdit) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit19textBackgroundColorEv()};
    let mut ret = unsafe {_ZNK9QTextEdit19textBackgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::QTextEdit(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QTextEdit_new for (&'a QString, &'a QWidget) {
  fn new(self) -> QTextEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QTextEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTextEditC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QTextEditC1ERK7QStringP7QWidget(arg0, arg1)} as u64;
    let rsthis = QTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextEdit::print(QPagedPaintDevice * printer);
impl /*struct*/ QTextEdit {
  pub fn print<RetType, T: QTextEdit_print<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.print(self);
    // return 1;
  }
}

pub trait QTextEdit_print<RetType> {
  fn print(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextEdit_print<()> for (&'a QPagedPaintDevice) {
  fn print(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK9QTextEdit5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextEdit::fontUnderline();
impl /*struct*/ QTextEdit {
  pub fn fontUnderline<RetType, T: QTextEdit_fontUnderline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontUnderline(self);
    // return 1;
  }
}

pub trait QTextEdit_fontUnderline<RetType> {
  fn fontUnderline(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  bool QTextEdit::fontUnderline();
impl<'a> /*trait*/ QTextEdit_fontUnderline<i8> for () {
  fn fontUnderline(self , rsthis: & QTextEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit13fontUnderlineEv()};
    let mut ret = unsafe {_ZNK9QTextEdit13fontUnderlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextEdit::insertPlainText(const QString & text);
impl /*struct*/ QTextEdit {
  pub fn insertPlainText<RetType, T: QTextEdit_insertPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertPlainText(self);
    // return 1;
  }
}

pub trait QTextEdit_insertPlainText<RetType> {
  fn insertPlainText(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::insertPlainText(const QString & text);
impl<'a> /*trait*/ QTextEdit_insertPlainText<()> for (&'a QString) {
  fn insertPlainText(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit15insertPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit15insertPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextEdit::fontWeight();
impl /*struct*/ QTextEdit {
  pub fn fontWeight<RetType, T: QTextEdit_fontWeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontWeight(self);
    // return 1;
  }
}

pub trait QTextEdit_fontWeight<RetType> {
  fn fontWeight(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  int QTextEdit::fontWeight();
impl<'a> /*trait*/ QTextEdit_fontWeight<i32> for () {
  fn fontWeight(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit10fontWeightEv()};
    let mut ret = unsafe {_ZNK9QTextEdit10fontWeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextEdit::copyAvailable(bool b);
impl /*struct*/ QTextEdit {
  pub fn copyAvailable<RetType, T: QTextEdit_copyAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.copyAvailable(self);
    // return 1;
  }
}

pub trait QTextEdit_copyAvailable<RetType> {
  fn copyAvailable(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::copyAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_copyAvailable<()> for (i8) {
  fn copyAvailable(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13copyAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit13copyAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QTextEdit::textColor();
impl /*struct*/ QTextEdit {
  pub fn textColor<RetType, T: QTextEdit_textColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textColor(self);
    // return 1;
  }
}

pub trait QTextEdit_textColor<RetType> {
  fn textColor(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  QColor QTextEdit::textColor();
impl<'a> /*trait*/ QTextEdit_textColor<QColor> for () {
  fn textColor(self , rsthis: & QTextEdit) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextEdit9textColorEv()};
    let mut ret = unsafe {_ZNK9QTextEdit9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextEdit::append(const QString & text);
impl /*struct*/ QTextEdit {
  pub fn append<RetType, T: QTextEdit_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QTextEdit_append<RetType> {
  fn append(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::append(const QString & text);
impl<'a> /*trait*/ QTextEdit_append<()> for (&'a QString) {
  fn append(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit6appendERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextEdit6appendERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::redoAvailable(bool b);
impl /*struct*/ QTextEdit {
  pub fn redoAvailable<RetType, T: QTextEdit_redoAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redoAvailable(self);
    // return 1;
  }
}

pub trait QTextEdit_redoAvailable<RetType> {
  fn redoAvailable(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::redoAvailable(bool b);
impl<'a> /*trait*/ QTextEdit_redoAvailable<()> for (i8) {
  fn redoAvailable(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit13redoAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit13redoAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextEdit::QTextEdit(QWidget * parent);
impl<'a> /*trait*/ QTextEdit_new for (&'a QWidget) {
  fn new(self) -> QTextEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEditC1EP7QWidget()};
    let ctysz: c_int = unsafe{QTextEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTextEditC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QTextEditC1EP7QWidget(arg0)} as u64;
    let rsthis = QTextEdit{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextEdit::setReadOnly(bool ro);
impl /*struct*/ QTextEdit {
  pub fn setReadOnly<RetType, T: QTextEdit_setReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QTextEdit_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: & QTextEdit) -> RetType;
}

  // proto:  void QTextEdit::setReadOnly(bool ro);
impl<'a> /*trait*/ QTextEdit_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: & QTextEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextEdit11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextEdit11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QTextEdit_cursorPositionChanged
pub struct QTextEdit_cursorPositionChanged_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn cursorPositionChanged_1(&self) -> QTextEdit_cursorPositionChanged_signal {
     return QTextEdit_cursorPositionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_cursorPositionChanged_signal {
  pub fn connect<T: QTextEdit_cursorPositionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_cursorPositionChanged_signal_connect {
  fn connect(self, sigthis: QTextEdit_cursorPositionChanged_signal);
}

#[derive(Default)] // for QTextEdit_redoAvailable
pub struct QTextEdit_redoAvailable_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn redoAvailable_1(&self) -> QTextEdit_redoAvailable_signal {
     return QTextEdit_redoAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_redoAvailable_signal {
  pub fn connect<T: QTextEdit_redoAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_redoAvailable_signal_connect {
  fn connect(self, sigthis: QTextEdit_redoAvailable_signal);
}

#[derive(Default)] // for QTextEdit_selectionChanged
pub struct QTextEdit_selectionChanged_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn selectionChanged_1(&self) -> QTextEdit_selectionChanged_signal {
     return QTextEdit_selectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_selectionChanged_signal {
  pub fn connect<T: QTextEdit_selectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_selectionChanged_signal_connect {
  fn connect(self, sigthis: QTextEdit_selectionChanged_signal);
}

#[derive(Default)] // for QTextEdit_currentCharFormatChanged
pub struct QTextEdit_currentCharFormatChanged_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn currentCharFormatChanged_1(&self) -> QTextEdit_currentCharFormatChanged_signal {
     return QTextEdit_currentCharFormatChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_currentCharFormatChanged_signal {
  pub fn connect<T: QTextEdit_currentCharFormatChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_currentCharFormatChanged_signal_connect {
  fn connect(self, sigthis: QTextEdit_currentCharFormatChanged_signal);
}

#[derive(Default)] // for QTextEdit_undoAvailable
pub struct QTextEdit_undoAvailable_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn undoAvailable_1(&self) -> QTextEdit_undoAvailable_signal {
     return QTextEdit_undoAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_undoAvailable_signal {
  pub fn connect<T: QTextEdit_undoAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_undoAvailable_signal_connect {
  fn connect(self, sigthis: QTextEdit_undoAvailable_signal);
}

#[derive(Default)] // for QTextEdit_textChanged
pub struct QTextEdit_textChanged_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn textChanged_1(&self) -> QTextEdit_textChanged_signal {
     return QTextEdit_textChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_textChanged_signal {
  pub fn connect<T: QTextEdit_textChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_textChanged_signal_connect {
  fn connect(self, sigthis: QTextEdit_textChanged_signal);
}

#[derive(Default)] // for QTextEdit_copyAvailable
pub struct QTextEdit_copyAvailable_signal{poi:u64}
impl /* struct */ QTextEdit {
  pub fn copyAvailable_1(&self) -> QTextEdit_copyAvailable_signal {
     return QTextEdit_copyAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextEdit_copyAvailable_signal {
  pub fn connect<T: QTextEdit_copyAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextEdit_copyAvailable_signal_connect {
  fn connect(self, sigthis: QTextEdit_copyAvailable_signal);
}

// undoAvailable(_Bool)
extern fn QTextEdit_undoAvailable_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextEdit_undoAvailable_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
impl /* trait */ QTextEdit_undoAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextEdit_undoAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_undoAvailable_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit13undoAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_undoAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextEdit_undoAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_undoAvailable_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit13undoAvailableEb(arg0, arg1, arg2)};
  }
}
// redoAvailable(_Bool)
extern fn QTextEdit_redoAvailable_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextEdit_redoAvailable_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
impl /* trait */ QTextEdit_redoAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextEdit_redoAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_redoAvailable_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit13redoAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_redoAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextEdit_redoAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_redoAvailable_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit13redoAvailableEb(arg0, arg1, arg2)};
  }
}
// copyAvailable(_Bool)
extern fn QTextEdit_copyAvailable_signal_connect_cb_2(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextEdit_copyAvailable_signal_connect_cb_box_2(rsfptr_raw:*mut Fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
impl /* trait */ QTextEdit_copyAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextEdit_copyAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_copyAvailable_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit13copyAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_copyAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextEdit_copyAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_copyAvailable_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit13copyAvailableEb(arg0, arg1, arg2)};
  }
}
// cursorPositionChanged()
extern fn QTextEdit_cursorPositionChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextEdit_cursorPositionChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QTextEdit_cursorPositionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTextEdit_cursorPositionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_cursorPositionChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit21cursorPositionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_cursorPositionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextEdit_cursorPositionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_cursorPositionChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit21cursorPositionChangedEv(arg0, arg1, arg2)};
  }
}
// selectionChanged()
extern fn QTextEdit_selectionChanged_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextEdit_selectionChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QTextEdit_selectionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTextEdit_selectionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_selectionChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit16selectionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_selectionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextEdit_selectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_selectionChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit16selectionChangedEv(arg0, arg1, arg2)};
  }
}
// currentCharFormatChanged(const class QTextCharFormat &)
extern fn QTextEdit_currentCharFormatChanged_signal_connect_cb_5(rsfptr:fn(QTextCharFormat), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTextCharFormat::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextEdit_currentCharFormatChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Fn(QTextCharFormat), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTextCharFormat::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QTextEdit_currentCharFormatChanged_signal_connect for fn(QTextCharFormat) {
  fn connect(self, sigthis: QTextEdit_currentCharFormatChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_currentCharFormatChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_currentCharFormatChanged_signal_connect for Box<Fn(QTextCharFormat)> {
  fn connect(self, sigthis: QTextEdit_currentCharFormatChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_currentCharFormatChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat(arg0, arg1, arg2)};
  }
}
// textChanged()
extern fn QTextEdit_textChanged_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextEdit_textChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QTextEdit_textChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTextEdit_textChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_textChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit11textChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextEdit_textChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextEdit_textChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextEdit_textChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTextEdit_SlotProxy_connect__ZN9QTextEdit11textChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

