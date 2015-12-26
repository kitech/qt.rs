// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qtextcursor.h
// dst-file: /src/gui/qtextcursor.rs
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
use std::ops::Deref;
use super::qtextformat::QTextCharFormat; // 773
use super::qtextdocumentfragment::QTextDocumentFragment; // 773
use super::qtextobject::QTextBlock; // 773
use super::qtextformat::QTextListFormat; // 773
use super::qtextlist::QTextList; // 773
use super::qtextformat::QTextImageFormat; // 773
use super::super::core::qstring::QString; // 771
use super::qtextformat::QTextFrameFormat; // 773
use super::qtextobject::QTextFrame; // 773
use super::qtexttable::QTextTable; // 773
use super::qtextformat::QTextBlockFormat; // 773
use super::qtextdocument::QTextDocument; // 773
use super::qimage::QImage; // 773
use super::qtextformat::QTextTableFormat; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextCursor_Class_Size() -> c_int;
  // proto:  int QTextCursor::columnNumber();
  fn _ZNK11QTextCursor12columnNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::swap(QTextCursor & other);
  fn _ZN11QTextCursor4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCursor::mergeCharFormat(const QTextCharFormat & modifier);
  fn _ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextDocumentFragment QTextCursor::selection();
  fn _ZNK11QTextCursor9selectionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCursor::hasComplexSelection();
  fn _ZNK11QTextCursor19hasComplexSelectionEv(qthis: *mut c_void) -> c_char;
  // proto:  QTextBlock QTextCursor::block();
  fn _ZNK11QTextCursor5blockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertFragment(const QTextDocumentFragment & fragment);
  fn _ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextList * QTextCursor::insertList(const QTextListFormat & format);
  fn _ZN11QTextCursor10insertListERK15QTextListFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertImage(const QTextImageFormat & format);
  fn _ZN11QTextCursor11insertImageERK16QTextImageFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCursor::keepPositionOnInsert();
  fn _ZNK11QTextCursor20keepPositionOnInsertEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTextCursor::position();
  fn _ZNK11QTextCursor8positionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextCursor::isNull();
  fn _ZNK11QTextCursor6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCursor::removeSelectedText();
  fn _ZN11QTextCursor18removeSelectedTextEv(qthis: *mut c_void);
  // proto:  void QTextCursor::insertHtml(const QString & html);
  fn _ZN11QTextCursor10insertHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCursor::isCopyOf(const QTextCursor & other);
  fn _ZNK11QTextCursor8isCopyOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QTextFrame * QTextCursor::insertFrame(const QTextFrameFormat & format);
  fn _ZN11QTextCursor11insertFrameERK16QTextFrameFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::QTextCursor(const QTextCursor & cursor);
  fn dector_ZN11QTextCursorC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextCursorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCursor::deleteChar();
  fn _ZN11QTextCursor10deleteCharEv(qthis: *mut c_void);
  // proto:  QTextFrame * QTextCursor::currentFrame();
  fn _ZNK11QTextCursor12currentFrameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertBlock();
  fn _ZN11QTextCursor11insertBlockEv(qthis: *mut c_void);
  // proto:  void QTextCursor::QTextCursor(const QTextBlock & block);
  fn dector_ZN11QTextCursorC1ERK10QTextBlock(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextCursorC1ERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols);
  fn _ZN11QTextCursor11insertTableEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTextCursor::QTextCursor();
  fn dector_ZN11QTextCursorC1Ev() -> *mut c_void;
  fn _ZN11QTextCursorC1Ev(qthis: *mut c_void);
  // proto:  bool QTextCursor::atStart();
  fn _ZNK11QTextCursor7atStartEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTextCursor::selectionStart();
  fn _ZNK11QTextCursor14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::selectedTableCells(int * firstRow, int * numRows, int * firstColumn, int * numColumns);
  fn _ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  void QTextCursor::endEditBlock();
  fn _ZN11QTextCursor12endEditBlockEv(qthis: *mut c_void);
  // proto:  QString QTextCursor::selectedText();
  fn _ZNK11QTextCursor12selectedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextCursor::positionInBlock();
  fn _ZNK11QTextCursor15positionInBlockEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextCursor::hasSelection();
  fn _ZNK11QTextCursor12hasSelectionEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTextCursor::atEnd();
  fn _ZNK11QTextCursor5atEndEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCursor::insertImage(const QString & name);
  fn _ZN11QTextCursor11insertImageERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCursor::atBlockStart();
  fn _ZNK11QTextCursor12atBlockStartEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCursor::insertText(const QString & text);
  fn _ZN11QTextCursor10insertTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCursor::visualNavigation();
  fn _ZNK11QTextCursor16visualNavigationEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTextCursor::atBlockEnd();
  fn _ZNK11QTextCursor10atBlockEndEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format);
  fn _ZN11QTextCursor11insertBlockERK16QTextBlockFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextList * QTextCursor::currentList();
  fn _ZNK11QTextCursor11currentListEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format, const QTextCharFormat & charFormat);
  fn _ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QTextCursor::mergeBlockCharFormat(const QTextCharFormat & modifier);
  fn _ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCursor::setCharFormat(const QTextCharFormat & format);
  fn _ZN11QTextCursor13setCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextCursor::verticalMovementX();
  fn _ZNK11QTextCursor17verticalMovementXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextCursor::blockNumber();
  fn _ZNK11QTextCursor11blockNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::joinPreviousEditBlock();
  fn _ZN11QTextCursor21joinPreviousEditBlockEv(qthis: *mut c_void);
  // proto:  void QTextCursor::QTextCursor(QTextDocument * document);
  fn dector_ZN11QTextCursorC1EP13QTextDocument(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextCursorC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCursor::insertText(const QString & text, const QTextCharFormat & format);
  fn _ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QTextCursor::mergeBlockFormat(const QTextBlockFormat & modifier);
  fn _ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextBlockFormat QTextCursor::blockFormat();
  fn _ZNK11QTextCursor11blockFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertImage(const QImage & image, const QString & name);
  fn _ZN11QTextCursor11insertImageERK6QImageRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QTextCursor::beginEditBlock();
  fn _ZN11QTextCursor14beginEditBlockEv(qthis: *mut c_void);
  // proto:  int QTextCursor::anchor();
  fn _ZNK11QTextCursor6anchorEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCharFormat QTextCursor::charFormat();
  fn _ZNK11QTextCursor10charFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::deletePreviousChar();
  fn _ZN11QTextCursor18deletePreviousCharEv(qthis: *mut c_void);
  // proto:  void QTextCursor::~QTextCursor();
  fn _ZN11QTextCursorD0Ev(qthis: *mut c_void);
  // proto:  void QTextCursor::clearSelection();
  fn _ZN11QTextCursor14clearSelectionEv(qthis: *mut c_void);
  // proto:  void QTextCursor::setVisualNavigation(bool b);
  fn _ZN11QTextCursor19setVisualNavigationEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextCursor::setBlockCharFormat(const QTextCharFormat & format);
  fn _ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextTable * QTextCursor::currentTable();
  fn _ZNK11QTextCursor12currentTableEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::setKeepPositionOnInsert(bool b);
  fn _ZN11QTextCursor23setKeepPositionOnInsertEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextCursor::setVerticalMovementX(int x);
  fn _ZN11QTextCursor20setVerticalMovementXEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextDocument * QTextCursor::document();
  fn _ZNK11QTextCursor8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols, const QTextTableFormat & format);
  fn _ZN11QTextCursor11insertTableEiiRK16QTextTableFormat(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::QTextCursor(QTextFrame * frame);
  fn dector_ZN11QTextCursorC1EP10QTextFrame(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextCursorC1EP10QTextFrame(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextCursor::selectionEnd();
  fn _ZNK11QTextCursor12selectionEndEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::setBlockFormat(const QTextBlockFormat & format);
  fn _ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextList * QTextCursor::createList(const QTextListFormat & format);
  fn _ZN11QTextCursor10createListERK15QTextListFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTextCharFormat QTextCursor::blockCharFormat();
  fn _ZNK11QTextCursor15blockCharFormatEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QTextCursor)=1
pub struct QTextCursor {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextCursor {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextCursor {
    return QTextCursor{qclsinst: qthis};
  }
}
  // proto:  int QTextCursor::columnNumber();
impl /*struct*/ QTextCursor {
  pub fn columnNumber<RetType, T: QTextCursor_columnNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnNumber(self);
    // return 1;
  }
}

pub trait QTextCursor_columnNumber<RetType> {
  fn columnNumber(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::columnNumber();
impl<'a> /*trait*/ QTextCursor_columnNumber<i32> for () {
  fn columnNumber(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12columnNumberEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12columnNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCursor::swap(QTextCursor & other);
impl /*struct*/ QTextCursor {
  pub fn swap<RetType, T: QTextCursor_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QTextCursor_swap<RetType> {
  fn swap(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::swap(QTextCursor & other);
impl<'a> /*trait*/ QTextCursor_swap<()> for (&'a QTextCursor) {
  fn swap(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCursor::mergeCharFormat(const QTextCharFormat & modifier);
impl /*struct*/ QTextCursor {
  pub fn mergeCharFormat<RetType, T: QTextCursor_mergeCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_mergeCharFormat<RetType> {
  fn mergeCharFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::mergeCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextCursor_mergeCharFormat<()> for (&'a QTextCharFormat) {
  fn mergeCharFormat(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextDocumentFragment QTextCursor::selection();
impl /*struct*/ QTextCursor {
  pub fn selection<RetType, T: QTextCursor_selection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selection(self);
    // return 1;
  }
}

pub trait QTextCursor_selection<RetType> {
  fn selection(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextDocumentFragment QTextCursor::selection();
impl<'a> /*trait*/ QTextCursor_selection<QTextDocumentFragment> for () {
  fn selection(self , rsthis: & QTextCursor) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor9selectionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor9selectionEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocumentFragment::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextCursor::hasComplexSelection();
impl /*struct*/ QTextCursor {
  pub fn hasComplexSelection<RetType, T: QTextCursor_hasComplexSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasComplexSelection(self);
    // return 1;
  }
}

pub trait QTextCursor_hasComplexSelection<RetType> {
  fn hasComplexSelection(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::hasComplexSelection();
impl<'a> /*trait*/ QTextCursor_hasComplexSelection<i8> for () {
  fn hasComplexSelection(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor19hasComplexSelectionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor19hasComplexSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextBlock QTextCursor::block();
impl /*struct*/ QTextCursor {
  pub fn block<RetType, T: QTextCursor_block<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.block(self);
    // return 1;
  }
}

pub trait QTextCursor_block<RetType> {
  fn block(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextBlock QTextCursor::block();
impl<'a> /*trait*/ QTextCursor_block<QTextBlock> for () {
  fn block(self , rsthis: & QTextCursor) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor5blockEv()};
    let mut ret = unsafe {_ZNK11QTextCursor5blockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertFragment(const QTextDocumentFragment & fragment);
impl /*struct*/ QTextCursor {
  pub fn insertFragment<RetType, T: QTextCursor_insertFragment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertFragment(self);
    // return 1;
  }
}

pub trait QTextCursor_insertFragment<RetType> {
  fn insertFragment(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::insertFragment(const QTextDocumentFragment & fragment);
impl<'a> /*trait*/ QTextCursor_insertFragment<()> for (&'a QTextDocumentFragment) {
  fn insertFragment(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextList * QTextCursor::insertList(const QTextListFormat & format);
impl /*struct*/ QTextCursor {
  pub fn insertList<RetType, T: QTextCursor_insertList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertList(self);
    // return 1;
  }
}

pub trait QTextCursor_insertList<RetType> {
  fn insertList(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextList * QTextCursor::insertList(const QTextListFormat & format);
impl<'a> /*trait*/ QTextCursor_insertList<QTextList> for (&'a QTextListFormat) {
  fn insertList(self , rsthis: & QTextCursor) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertListERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor10insertListERK15QTextListFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextList::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertImage(const QTextImageFormat & format);
impl /*struct*/ QTextCursor {
  pub fn insertImage<RetType, T: QTextCursor_insertImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertImage(self);
    // return 1;
  }
}

pub trait QTextCursor_insertImage<RetType> {
  fn insertImage(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::insertImage(const QTextImageFormat & format);
impl<'a> /*trait*/ QTextCursor_insertImage<()> for (&'a QTextImageFormat) {
  fn insertImage(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertImageERK16QTextImageFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertImageERK16QTextImageFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCursor::keepPositionOnInsert();
impl /*struct*/ QTextCursor {
  pub fn keepPositionOnInsert<RetType, T: QTextCursor_keepPositionOnInsert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keepPositionOnInsert(self);
    // return 1;
  }
}

pub trait QTextCursor_keepPositionOnInsert<RetType> {
  fn keepPositionOnInsert(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::keepPositionOnInsert();
impl<'a> /*trait*/ QTextCursor_keepPositionOnInsert<i8> for () {
  fn keepPositionOnInsert(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor20keepPositionOnInsertEv()};
    let mut ret = unsafe {_ZNK11QTextCursor20keepPositionOnInsertEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextCursor::position();
impl /*struct*/ QTextCursor {
  pub fn position<RetType, T: QTextCursor_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextCursor_position<RetType> {
  fn position(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::position();
impl<'a> /*trait*/ QTextCursor_position<i32> for () {
  fn position(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor8positionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextCursor::isNull();
impl /*struct*/ QTextCursor {
  pub fn isNull<RetType, T: QTextCursor_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QTextCursor_isNull<RetType> {
  fn isNull(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::isNull();
impl<'a> /*trait*/ QTextCursor_isNull<i8> for () {
  fn isNull(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor6isNullEv()};
    let mut ret = unsafe {_ZNK11QTextCursor6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCursor::removeSelectedText();
impl /*struct*/ QTextCursor {
  pub fn removeSelectedText<RetType, T: QTextCursor_removeSelectedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeSelectedText(self);
    // return 1;
  }
}

pub trait QTextCursor_removeSelectedText<RetType> {
  fn removeSelectedText(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::removeSelectedText();
impl<'a> /*trait*/ QTextCursor_removeSelectedText<()> for () {
  fn removeSelectedText(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor18removeSelectedTextEv()};
     unsafe {_ZN11QTextCursor18removeSelectedTextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCursor::insertHtml(const QString & html);
impl /*struct*/ QTextCursor {
  pub fn insertHtml<RetType, T: QTextCursor_insertHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertHtml(self);
    // return 1;
  }
}

pub trait QTextCursor_insertHtml<RetType> {
  fn insertHtml(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::insertHtml(const QString & html);
impl<'a> /*trait*/ QTextCursor_insertHtml<()> for (&'a QString) {
  fn insertHtml(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor10insertHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCursor::isCopyOf(const QTextCursor & other);
impl /*struct*/ QTextCursor {
  pub fn isCopyOf<RetType, T: QTextCursor_isCopyOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCopyOf(self);
    // return 1;
  }
}

pub trait QTextCursor_isCopyOf<RetType> {
  fn isCopyOf(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::isCopyOf(const QTextCursor & other);
impl<'a> /*trait*/ QTextCursor_isCopyOf<i8> for (&'a QTextCursor) {
  fn isCopyOf(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTextCursor8isCopyOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextFrame * QTextCursor::insertFrame(const QTextFrameFormat & format);
impl /*struct*/ QTextCursor {
  pub fn insertFrame<RetType, T: QTextCursor_insertFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertFrame(self);
    // return 1;
  }
}

pub trait QTextCursor_insertFrame<RetType> {
  fn insertFrame(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextFrame * QTextCursor::insertFrame(const QTextFrameFormat & format);
impl<'a> /*trait*/ QTextCursor_insertFrame<QTextFrame> for (&'a QTextFrameFormat) {
  fn insertFrame(self , rsthis: & QTextCursor) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertFrameERK16QTextFrameFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor11insertFrameERK16QTextFrameFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextFrame::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::QTextCursor(const QTextCursor & cursor);
impl /*struct*/ QTextCursor {
  pub fn New<T: QTextCursor_New>(value: T) -> QTextCursor {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCursor_New {
  fn New(self) -> QTextCursor;
}

  // proto:  void QTextCursor::QTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextCursor_New for (&'a QTextCursor) {
  fn New(self) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1ERKS_()};
    let ctysz: c_int = unsafe{QTextCursor_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextCursorC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextCursorC1ERKS_(arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextCursor::deleteChar();
impl /*struct*/ QTextCursor {
  pub fn deleteChar<RetType, T: QTextCursor_deleteChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deleteChar(self);
    // return 1;
  }
}

pub trait QTextCursor_deleteChar<RetType> {
  fn deleteChar(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::deleteChar();
impl<'a> /*trait*/ QTextCursor_deleteChar<()> for () {
  fn deleteChar(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10deleteCharEv()};
     unsafe {_ZN11QTextCursor10deleteCharEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextFrame * QTextCursor::currentFrame();
impl /*struct*/ QTextCursor {
  pub fn currentFrame<RetType, T: QTextCursor_currentFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFrame(self);
    // return 1;
  }
}

pub trait QTextCursor_currentFrame<RetType> {
  fn currentFrame(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextFrame * QTextCursor::currentFrame();
impl<'a> /*trait*/ QTextCursor_currentFrame<QTextFrame> for () {
  fn currentFrame(self , rsthis: & QTextCursor) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12currentFrameEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12currentFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertBlock();
impl /*struct*/ QTextCursor {
  pub fn insertBlock<RetType, T: QTextCursor_insertBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_insertBlock<RetType> {
  fn insertBlock(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::insertBlock();
impl<'a> /*trait*/ QTextCursor_insertBlock<()> for () {
  fn insertBlock(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertBlockEv()};
     unsafe {_ZN11QTextCursor11insertBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCursor::QTextCursor(const QTextBlock & block);
impl<'a> /*trait*/ QTextCursor_New for (&'a QTextBlock) {
  fn New(self) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1ERK10QTextBlock()};
    let ctysz: c_int = unsafe{QTextCursor_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextCursorC1ERK10QTextBlock(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextCursorC1ERK10QTextBlock(arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols);
impl /*struct*/ QTextCursor {
  pub fn insertTable<RetType, T: QTextCursor_insertTable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertTable(self);
    // return 1;
  }
}

pub trait QTextCursor_insertTable<RetType> {
  fn insertTable(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols);
impl<'a> /*trait*/ QTextCursor_insertTable<QTextTable> for (i32, i32) {
  fn insertTable(self , rsthis: & QTextCursor) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertTableEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QTextCursor11insertTableEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTextTable::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::QTextCursor();
impl<'a> /*trait*/ QTextCursor_New for () {
  fn New(self) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1Ev()};
    let ctysz: c_int = unsafe{QTextCursor_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QTextCursorC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextCursorC1Ev()};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextCursor::atStart();
impl /*struct*/ QTextCursor {
  pub fn atStart<RetType, T: QTextCursor_atStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atStart(self);
    // return 1;
  }
}

pub trait QTextCursor_atStart<RetType> {
  fn atStart(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::atStart();
impl<'a> /*trait*/ QTextCursor_atStart<i8> for () {
  fn atStart(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor7atStartEv()};
    let mut ret = unsafe {_ZNK11QTextCursor7atStartEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextCursor::selectionStart();
impl /*struct*/ QTextCursor {
  pub fn selectionStart<RetType, T: QTextCursor_selectionStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionStart(self);
    // return 1;
  }
}

pub trait QTextCursor_selectionStart<RetType> {
  fn selectionStart(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::selectionStart();
impl<'a> /*trait*/ QTextCursor_selectionStart<i32> for () {
  fn selectionStart(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor14selectionStartEv()};
    let mut ret = unsafe {_ZNK11QTextCursor14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCursor::selectedTableCells(int * firstRow, int * numRows, int * firstColumn, int * numColumns);
impl /*struct*/ QTextCursor {
  pub fn selectedTableCells<RetType, T: QTextCursor_selectedTableCells<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedTableCells(self);
    // return 1;
  }
}

pub trait QTextCursor_selectedTableCells<RetType> {
  fn selectedTableCells(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::selectedTableCells(int * firstRow, int * numRows, int * firstColumn, int * numColumns);
impl<'a> /*trait*/ QTextCursor_selectedTableCells<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn selectedTableCells(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QTextCursor::endEditBlock();
impl /*struct*/ QTextCursor {
  pub fn endEditBlock<RetType, T: QTextCursor_endEditBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endEditBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_endEditBlock<RetType> {
  fn endEditBlock(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::endEditBlock();
impl<'a> /*trait*/ QTextCursor_endEditBlock<()> for () {
  fn endEditBlock(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor12endEditBlockEv()};
     unsafe {_ZN11QTextCursor12endEditBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextCursor::selectedText();
impl /*struct*/ QTextCursor {
  pub fn selectedText<RetType, T: QTextCursor_selectedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedText(self);
    // return 1;
  }
}

pub trait QTextCursor_selectedText<RetType> {
  fn selectedText(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QString QTextCursor::selectedText();
impl<'a> /*trait*/ QTextCursor_selectedText<QString> for () {
  fn selectedText(self , rsthis: & QTextCursor) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12selectedTextEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12selectedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextCursor::positionInBlock();
impl /*struct*/ QTextCursor {
  pub fn positionInBlock<RetType, T: QTextCursor_positionInBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.positionInBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_positionInBlock<RetType> {
  fn positionInBlock(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::positionInBlock();
impl<'a> /*trait*/ QTextCursor_positionInBlock<i32> for () {
  fn positionInBlock(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor15positionInBlockEv()};
    let mut ret = unsafe {_ZNK11QTextCursor15positionInBlockEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextCursor::hasSelection();
impl /*struct*/ QTextCursor {
  pub fn hasSelection<RetType, T: QTextCursor_hasSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasSelection(self);
    // return 1;
  }
}

pub trait QTextCursor_hasSelection<RetType> {
  fn hasSelection(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::hasSelection();
impl<'a> /*trait*/ QTextCursor_hasSelection<i8> for () {
  fn hasSelection(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12hasSelectionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12hasSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextCursor::atEnd();
impl /*struct*/ QTextCursor {
  pub fn atEnd<RetType, T: QTextCursor_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QTextCursor_atEnd<RetType> {
  fn atEnd(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::atEnd();
impl<'a> /*trait*/ QTextCursor_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor5atEndEv()};
    let mut ret = unsafe {_ZNK11QTextCursor5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertImage(const QString & name);
impl<'a> /*trait*/ QTextCursor_insertImage<()> for (&'a QString) {
  fn insertImage(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertImageERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertImageERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCursor::atBlockStart();
impl /*struct*/ QTextCursor {
  pub fn atBlockStart<RetType, T: QTextCursor_atBlockStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atBlockStart(self);
    // return 1;
  }
}

pub trait QTextCursor_atBlockStart<RetType> {
  fn atBlockStart(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::atBlockStart();
impl<'a> /*trait*/ QTextCursor_atBlockStart<i8> for () {
  fn atBlockStart(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12atBlockStartEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12atBlockStartEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertText(const QString & text);
impl /*struct*/ QTextCursor {
  pub fn insertText<RetType, T: QTextCursor_insertText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertText(self);
    // return 1;
  }
}

pub trait QTextCursor_insertText<RetType> {
  fn insertText(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::insertText(const QString & text);
impl<'a> /*trait*/ QTextCursor_insertText<()> for (&'a QString) {
  fn insertText(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor10insertTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCursor::visualNavigation();
impl /*struct*/ QTextCursor {
  pub fn visualNavigation<RetType, T: QTextCursor_visualNavigation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualNavigation(self);
    // return 1;
  }
}

pub trait QTextCursor_visualNavigation<RetType> {
  fn visualNavigation(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::visualNavigation();
impl<'a> /*trait*/ QTextCursor_visualNavigation<i8> for () {
  fn visualNavigation(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor16visualNavigationEv()};
    let mut ret = unsafe {_ZNK11QTextCursor16visualNavigationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextCursor::atBlockEnd();
impl /*struct*/ QTextCursor {
  pub fn atBlockEnd<RetType, T: QTextCursor_atBlockEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atBlockEnd(self);
    // return 1;
  }
}

pub trait QTextCursor_atBlockEnd<RetType> {
  fn atBlockEnd(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  bool QTextCursor::atBlockEnd();
impl<'a> /*trait*/ QTextCursor_atBlockEnd<i8> for () {
  fn atBlockEnd(self , rsthis: & QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor10atBlockEndEv()};
    let mut ret = unsafe {_ZNK11QTextCursor10atBlockEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format);
impl<'a> /*trait*/ QTextCursor_insertBlock<()> for (&'a QTextBlockFormat) {
  fn insertBlock(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertBlockERK16QTextBlockFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertBlockERK16QTextBlockFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextList * QTextCursor::currentList();
impl /*struct*/ QTextCursor {
  pub fn currentList<RetType, T: QTextCursor_currentList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentList(self);
    // return 1;
  }
}

pub trait QTextCursor_currentList<RetType> {
  fn currentList(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextList * QTextCursor::currentList();
impl<'a> /*trait*/ QTextCursor_currentList<QTextList> for () {
  fn currentList(self , rsthis: & QTextCursor) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor11currentListEv()};
    let mut ret = unsafe {_ZNK11QTextCursor11currentListEv(rsthis.qclsinst)};
    let mut ret1 = QTextList::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format, const QTextCharFormat & charFormat);
impl<'a> /*trait*/ QTextCursor_insertBlock<()> for (&'a QTextBlockFormat, &'a QTextCharFormat) {
  fn insertBlock(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextCursor::mergeBlockCharFormat(const QTextCharFormat & modifier);
impl /*struct*/ QTextCursor {
  pub fn mergeBlockCharFormat<RetType, T: QTextCursor_mergeBlockCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeBlockCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_mergeBlockCharFormat<RetType> {
  fn mergeBlockCharFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::mergeBlockCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextCursor_mergeBlockCharFormat<()> for (&'a QTextCharFormat) {
  fn mergeBlockCharFormat(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCursor::setCharFormat(const QTextCharFormat & format);
impl /*struct*/ QTextCursor {
  pub fn setCharFormat<RetType, T: QTextCursor_setCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_setCharFormat<RetType> {
  fn setCharFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::setCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextCursor_setCharFormat<()> for (&'a QTextCharFormat) {
  fn setCharFormat(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor13setCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor13setCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextCursor::verticalMovementX();
impl /*struct*/ QTextCursor {
  pub fn verticalMovementX<RetType, T: QTextCursor_verticalMovementX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalMovementX(self);
    // return 1;
  }
}

pub trait QTextCursor_verticalMovementX<RetType> {
  fn verticalMovementX(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::verticalMovementX();
impl<'a> /*trait*/ QTextCursor_verticalMovementX<i32> for () {
  fn verticalMovementX(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor17verticalMovementXEv()};
    let mut ret = unsafe {_ZNK11QTextCursor17verticalMovementXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextCursor::blockNumber();
impl /*struct*/ QTextCursor {
  pub fn blockNumber<RetType, T: QTextCursor_blockNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockNumber(self);
    // return 1;
  }
}

pub trait QTextCursor_blockNumber<RetType> {
  fn blockNumber(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::blockNumber();
impl<'a> /*trait*/ QTextCursor_blockNumber<i32> for () {
  fn blockNumber(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor11blockNumberEv()};
    let mut ret = unsafe {_ZNK11QTextCursor11blockNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCursor::joinPreviousEditBlock();
impl /*struct*/ QTextCursor {
  pub fn joinPreviousEditBlock<RetType, T: QTextCursor_joinPreviousEditBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.joinPreviousEditBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_joinPreviousEditBlock<RetType> {
  fn joinPreviousEditBlock(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::joinPreviousEditBlock();
impl<'a> /*trait*/ QTextCursor_joinPreviousEditBlock<()> for () {
  fn joinPreviousEditBlock(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor21joinPreviousEditBlockEv()};
     unsafe {_ZN11QTextCursor21joinPreviousEditBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCursor::QTextCursor(QTextDocument * document);
impl<'a> /*trait*/ QTextCursor_New for (&'a QTextDocument) {
  fn New(self) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1EP13QTextDocument()};
    let ctysz: c_int = unsafe{QTextCursor_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextCursorC1EP13QTextDocument(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextCursorC1EP13QTextDocument(arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertText(const QString & text, const QTextCharFormat & format);
impl<'a> /*trait*/ QTextCursor_insertText<()> for (&'a QString, &'a QTextCharFormat) {
  fn insertText(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextCursor::mergeBlockFormat(const QTextBlockFormat & modifier);
impl /*struct*/ QTextCursor {
  pub fn mergeBlockFormat<RetType, T: QTextCursor_mergeBlockFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeBlockFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_mergeBlockFormat<RetType> {
  fn mergeBlockFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::mergeBlockFormat(const QTextBlockFormat & modifier);
impl<'a> /*trait*/ QTextCursor_mergeBlockFormat<()> for (&'a QTextBlockFormat) {
  fn mergeBlockFormat(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextBlockFormat QTextCursor::blockFormat();
impl /*struct*/ QTextCursor {
  pub fn blockFormat<RetType, T: QTextCursor_blockFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_blockFormat<RetType> {
  fn blockFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextBlockFormat QTextCursor::blockFormat();
impl<'a> /*trait*/ QTextCursor_blockFormat<QTextBlockFormat> for () {
  fn blockFormat(self , rsthis: & QTextCursor) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor11blockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextCursor11blockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::insertImage(const QImage & image, const QString & name);
impl<'a> /*trait*/ QTextCursor_insertImage<()> for (&'a QImage, &'a QString) {
  fn insertImage(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertImageERK6QImageRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertImageERK6QImageRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextCursor::beginEditBlock();
impl /*struct*/ QTextCursor {
  pub fn beginEditBlock<RetType, T: QTextCursor_beginEditBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginEditBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_beginEditBlock<RetType> {
  fn beginEditBlock(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::beginEditBlock();
impl<'a> /*trait*/ QTextCursor_beginEditBlock<()> for () {
  fn beginEditBlock(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14beginEditBlockEv()};
     unsafe {_ZN11QTextCursor14beginEditBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextCursor::anchor();
impl /*struct*/ QTextCursor {
  pub fn anchor<RetType, T: QTextCursor_anchor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.anchor(self);
    // return 1;
  }
}

pub trait QTextCursor_anchor<RetType> {
  fn anchor(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::anchor();
impl<'a> /*trait*/ QTextCursor_anchor<i32> for () {
  fn anchor(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor6anchorEv()};
    let mut ret = unsafe {_ZNK11QTextCursor6anchorEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextCursor::charFormat();
impl /*struct*/ QTextCursor {
  pub fn charFormat<RetType, T: QTextCursor_charFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.charFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_charFormat<RetType> {
  fn charFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextCharFormat QTextCursor::charFormat();
impl<'a> /*trait*/ QTextCursor_charFormat<QTextCharFormat> for () {
  fn charFormat(self , rsthis: & QTextCursor) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor10charFormatEv()};
    let mut ret = unsafe {_ZNK11QTextCursor10charFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::deletePreviousChar();
impl /*struct*/ QTextCursor {
  pub fn deletePreviousChar<RetType, T: QTextCursor_deletePreviousChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deletePreviousChar(self);
    // return 1;
  }
}

pub trait QTextCursor_deletePreviousChar<RetType> {
  fn deletePreviousChar(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::deletePreviousChar();
impl<'a> /*trait*/ QTextCursor_deletePreviousChar<()> for () {
  fn deletePreviousChar(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor18deletePreviousCharEv()};
     unsafe {_ZN11QTextCursor18deletePreviousCharEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCursor::~QTextCursor();
impl /*struct*/ QTextCursor {
  pub fn Free<RetType, T: QTextCursor_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextCursor_Free<RetType> {
  fn Free(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::~QTextCursor();
impl<'a> /*trait*/ QTextCursor_Free<()> for () {
  fn Free(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorD0Ev()};
     unsafe {_ZN11QTextCursorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCursor::clearSelection();
impl /*struct*/ QTextCursor {
  pub fn clearSelection<RetType, T: QTextCursor_clearSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearSelection(self);
    // return 1;
  }
}

pub trait QTextCursor_clearSelection<RetType> {
  fn clearSelection(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::clearSelection();
impl<'a> /*trait*/ QTextCursor_clearSelection<()> for () {
  fn clearSelection(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14clearSelectionEv()};
     unsafe {_ZN11QTextCursor14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCursor::setVisualNavigation(bool b);
impl /*struct*/ QTextCursor {
  pub fn setVisualNavigation<RetType, T: QTextCursor_setVisualNavigation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisualNavigation(self);
    // return 1;
  }
}

pub trait QTextCursor_setVisualNavigation<RetType> {
  fn setVisualNavigation(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::setVisualNavigation(bool b);
impl<'a> /*trait*/ QTextCursor_setVisualNavigation<()> for (i8) {
  fn setVisualNavigation(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor19setVisualNavigationEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QTextCursor19setVisualNavigationEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCursor::setBlockCharFormat(const QTextCharFormat & format);
impl /*struct*/ QTextCursor {
  pub fn setBlockCharFormat<RetType, T: QTextCursor_setBlockCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBlockCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_setBlockCharFormat<RetType> {
  fn setBlockCharFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::setBlockCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextCursor_setBlockCharFormat<()> for (&'a QTextCharFormat) {
  fn setBlockCharFormat(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextTable * QTextCursor::currentTable();
impl /*struct*/ QTextCursor {
  pub fn currentTable<RetType, T: QTextCursor_currentTable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentTable(self);
    // return 1;
  }
}

pub trait QTextCursor_currentTable<RetType> {
  fn currentTable(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextTable * QTextCursor::currentTable();
impl<'a> /*trait*/ QTextCursor_currentTable<QTextTable> for () {
  fn currentTable(self , rsthis: & QTextCursor) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12currentTableEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12currentTableEv(rsthis.qclsinst)};
    let mut ret1 = QTextTable::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::setKeepPositionOnInsert(bool b);
impl /*struct*/ QTextCursor {
  pub fn setKeepPositionOnInsert<RetType, T: QTextCursor_setKeepPositionOnInsert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeepPositionOnInsert(self);
    // return 1;
  }
}

pub trait QTextCursor_setKeepPositionOnInsert<RetType> {
  fn setKeepPositionOnInsert(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::setKeepPositionOnInsert(bool b);
impl<'a> /*trait*/ QTextCursor_setKeepPositionOnInsert<()> for (i8) {
  fn setKeepPositionOnInsert(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor23setKeepPositionOnInsertEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QTextCursor23setKeepPositionOnInsertEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCursor::setVerticalMovementX(int x);
impl /*struct*/ QTextCursor {
  pub fn setVerticalMovementX<RetType, T: QTextCursor_setVerticalMovementX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalMovementX(self);
    // return 1;
  }
}

pub trait QTextCursor_setVerticalMovementX<RetType> {
  fn setVerticalMovementX(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::setVerticalMovementX(int x);
impl<'a> /*trait*/ QTextCursor_setVerticalMovementX<()> for (i32) {
  fn setVerticalMovementX(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor20setVerticalMovementXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextCursor20setVerticalMovementXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextDocument * QTextCursor::document();
impl /*struct*/ QTextCursor {
  pub fn document<RetType, T: QTextCursor_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QTextCursor_document<RetType> {
  fn document(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextDocument * QTextCursor::document();
impl<'a> /*trait*/ QTextCursor_document<QTextDocument> for () {
  fn document(self , rsthis: & QTextCursor) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor8documentEv()};
    let mut ret = unsafe {_ZNK11QTextCursor8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols, const QTextTableFormat & format);
impl<'a> /*trait*/ QTextCursor_insertTable<QTextTable> for (i32, i32, &'a QTextTableFormat) {
  fn insertTable(self , rsthis: & QTextCursor) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertTableEiiRK16QTextTableFormat()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor11insertTableEiiRK16QTextTableFormat(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QTextTable::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCursor::QTextCursor(QTextFrame * frame);
impl<'a> /*trait*/ QTextCursor_New for (&'a QTextFrame) {
  fn New(self) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1EP10QTextFrame()};
    let ctysz: c_int = unsafe{QTextCursor_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextCursorC1EP10QTextFrame(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextCursorC1EP10QTextFrame(arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextCursor::selectionEnd();
impl /*struct*/ QTextCursor {
  pub fn selectionEnd<RetType, T: QTextCursor_selectionEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionEnd(self);
    // return 1;
  }
}

pub trait QTextCursor_selectionEnd<RetType> {
  fn selectionEnd(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  int QTextCursor::selectionEnd();
impl<'a> /*trait*/ QTextCursor_selectionEnd<i32> for () {
  fn selectionEnd(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12selectionEndEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12selectionEndEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCursor::setBlockFormat(const QTextBlockFormat & format);
impl /*struct*/ QTextCursor {
  pub fn setBlockFormat<RetType, T: QTextCursor_setBlockFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBlockFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_setBlockFormat<RetType> {
  fn setBlockFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  void QTextCursor::setBlockFormat(const QTextBlockFormat & format);
impl<'a> /*trait*/ QTextCursor_setBlockFormat<()> for (&'a QTextBlockFormat) {
  fn setBlockFormat(self , rsthis: & QTextCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextList * QTextCursor::createList(const QTextListFormat & format);
impl /*struct*/ QTextCursor {
  pub fn createList<RetType, T: QTextCursor_createList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createList(self);
    // return 1;
  }
}

pub trait QTextCursor_createList<RetType> {
  fn createList(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextList * QTextCursor::createList(const QTextListFormat & format);
impl<'a> /*trait*/ QTextCursor_createList<QTextList> for (&'a QTextListFormat) {
  fn createList(self , rsthis: & QTextCursor) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10createListERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor10createListERK15QTextListFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextList::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextCursor::blockCharFormat();
impl /*struct*/ QTextCursor {
  pub fn blockCharFormat<RetType, T: QTextCursor_blockCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_blockCharFormat<RetType> {
  fn blockCharFormat(self , rsthis: & QTextCursor) -> RetType;
}

  // proto:  QTextCharFormat QTextCursor::blockCharFormat();
impl<'a> /*trait*/ QTextCursor_blockCharFormat<QTextCharFormat> for () {
  fn blockCharFormat(self , rsthis: & QTextCursor) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor15blockCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextCursor15blockCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

