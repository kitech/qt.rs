// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcharformat::QTextCharFormat;
use super::qtextdocumentfragment::QTextDocumentFragment;
use super::qtextblock::QTextBlock;
use super::qtextlistformat::QTextListFormat;
use super::qtextlist::QTextList;
use super::qtextimageformat::QTextImageFormat;
use super::qstring::QString;
use super::qtextframeformat::QTextFrameFormat;
use super::qtextframe::QTextFrame;
use super::qtexttable::QTextTable;
use super::qtextblockformat::QTextBlockFormat;
use super::qtextdocument::QTextDocument;
use super::qimage::QImage;
use super::qtexttableformat::QTextTableFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QTextCursor::columnNumber();
  fn _ZNK11QTextCursor12columnNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::swap(QTextCursor & other);
  fn _ZN11QTextCursor4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCursor::mergeCharFormat(const QTextCharFormat & modifier);
  fn _ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextDocumentFragment QTextCursor::selection();
  fn _ZNK11QTextCursor9selectionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCursor::hasComplexSelection();
  fn _ZNK11QTextCursor19hasComplexSelectionEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTextBlock QTextCursor::block();
  fn _ZNK11QTextCursor5blockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertFragment(const QTextDocumentFragment & fragment);
  fn _ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextList * QTextCursor::insertList(const QTextListFormat & format);
  fn _ZN11QTextCursor10insertListERK15QTextListFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertImage(const QTextImageFormat & format);
  fn _ZN11QTextCursor11insertImageERK16QTextImageFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCursor::keepPositionOnInsert();
  fn _ZNK11QTextCursor20keepPositionOnInsertEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTextCursor::position();
  fn _ZNK11QTextCursor8positionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextCursor::isNull();
  fn _ZNK11QTextCursor6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCursor::removeSelectedText();
  fn _ZN11QTextCursor18removeSelectedTextEv(qthis: *mut c_void) ;
  // proto:  void QTextCursor::insertHtml(const QString & html);
  fn _ZN11QTextCursor10insertHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCursor::isCopyOf(const QTextCursor & other);
  fn _ZNK11QTextCursor8isCopyOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QTextFrame * QTextCursor::insertFrame(const QTextFrameFormat & format);
  fn _ZN11QTextCursor11insertFrameERK16QTextFrameFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::NewQTextCursor(const QTextCursor & cursor);
  fn _ZN11QTextCursorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCursor::deleteChar();
  fn _ZN11QTextCursor10deleteCharEv(qthis: *mut c_void) ;
  // proto:  QTextFrame * QTextCursor::currentFrame();
  fn _ZNK11QTextCursor12currentFrameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertBlock();
  fn _ZN11QTextCursor11insertBlockEv(qthis: *mut c_void) ;
  // proto:  void QTextCursor::NewQTextCursor(const QTextBlock & block);
  fn _ZN11QTextCursorC1ERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols);
  fn _ZN11QTextCursor11insertTableEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTextCursor::NewQTextCursor();
  fn _ZN11QTextCursorC1Ev(qthis: *mut c_void) ;
  // proto:  bool QTextCursor::atStart();
  fn _ZNK11QTextCursor7atStartEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTextCursor::selectionStart();
  fn _ZNK11QTextCursor14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::selectedTableCells(int * firstRow, int * numRows, int * firstColumn, int * numColumns);
  fn _ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  void QTextCursor::endEditBlock();
  fn _ZN11QTextCursor12endEditBlockEv(qthis: *mut c_void) ;
  // proto:  QString QTextCursor::selectedText();
  fn _ZNK11QTextCursor12selectedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextCursor::positionInBlock();
  fn _ZNK11QTextCursor15positionInBlockEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextCursor::hasSelection();
  fn _ZNK11QTextCursor12hasSelectionEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextCursor::atEnd();
  fn _ZNK11QTextCursor5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCursor::insertImage(const QString & name);
  fn _ZN11QTextCursor11insertImageERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCursor::atBlockStart();
  fn _ZNK11QTextCursor12atBlockStartEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCursor::insertText(const QString & text);
  fn _ZN11QTextCursor10insertTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCursor::visualNavigation();
  fn _ZNK11QTextCursor16visualNavigationEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextCursor::atBlockEnd();
  fn _ZNK11QTextCursor10atBlockEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format);
  fn _ZN11QTextCursor11insertBlockERK16QTextBlockFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextList * QTextCursor::currentList();
  fn _ZNK11QTextCursor11currentListEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format, const QTextCharFormat & charFormat);
  fn _ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTextCursor::mergeBlockCharFormat(const QTextCharFormat & modifier);
  fn _ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCursor::setCharFormat(const QTextCharFormat & format);
  fn _ZN11QTextCursor13setCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextCursor::verticalMovementX();
  fn _ZNK11QTextCursor17verticalMovementXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextCursor::blockNumber();
  fn _ZNK11QTextCursor11blockNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::joinPreviousEditBlock();
  fn _ZN11QTextCursor21joinPreviousEditBlockEv(qthis: *mut c_void) ;
  // proto:  void QTextCursor::NewQTextCursor(QTextDocument * document);
  fn _ZN11QTextCursorC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCursor::insertText(const QString & text, const QTextCharFormat & format);
  fn _ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTextCursor::mergeBlockFormat(const QTextBlockFormat & modifier);
  fn _ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextBlockFormat QTextCursor::blockFormat();
  fn _ZNK11QTextCursor11blockFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::insertImage(const QImage & image, const QString & name);
  fn _ZN11QTextCursor11insertImageERK6QImageRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTextCursor::beginEditBlock();
  fn _ZN11QTextCursor14beginEditBlockEv(qthis: *mut c_void) ;
  // proto:  int QTextCursor::anchor();
  fn _ZNK11QTextCursor6anchorEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCharFormat QTextCursor::charFormat();
  fn _ZNK11QTextCursor10charFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::deletePreviousChar();
  fn _ZN11QTextCursor18deletePreviousCharEv(qthis: *mut c_void) ;
  // proto:  void QTextCursor::FreeQTextCursor();
  fn _ZN11QTextCursorD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextCursor::clearSelection();
  fn _ZN11QTextCursor14clearSelectionEv(qthis: *mut c_void) ;
  // proto:  void QTextCursor::setVisualNavigation(bool b);
  fn _ZN11QTextCursor19setVisualNavigationEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextCursor::setBlockCharFormat(const QTextCharFormat & format);
  fn _ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextTable * QTextCursor::currentTable();
  fn _ZNK11QTextCursor12currentTableEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::setKeepPositionOnInsert(bool b);
  fn _ZN11QTextCursor23setKeepPositionOnInsertEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextCursor::setVerticalMovementX(int x);
  fn _ZN11QTextCursor20setVerticalMovementXEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QTextDocument * QTextCursor::document();
  fn _ZNK11QTextCursor8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextTable * QTextCursor::insertTable(int rows, int cols, const QTextTableFormat & format);
  fn _ZN11QTextCursor11insertTableEiiRK16QTextTableFormat(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QTextCursor::NewQTextCursor(QTextFrame * frame);
  fn _ZN11QTextCursorC1EP10QTextFrame(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextCursor::selectionEnd();
  fn _ZNK11QTextCursor12selectionEndEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCursor::setBlockFormat(const QTextBlockFormat & format);
  fn _ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextList * QTextCursor::createList(const QTextListFormat & format);
  fn _ZN11QTextCursor10createListERK15QTextListFormat(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTextCharFormat QTextCursor::blockCharFormat();
  fn _ZNK11QTextCursor15blockCharFormatEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QTextCursor)=1
pub struct QTextCursor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextCursor {
  pub fn columnNumber<T: QTextCursor_columnNumber>(&mut self, value: T) -> i32 {
    return value.columnNumber(self);
    // return 1;
  }
}

pub trait QTextCursor_columnNumber {
  fn columnNumber(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::columnNumber();
impl<'a> /*trait*/ QTextCursor_columnNumber for () {
  fn columnNumber(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12columnNumberEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12columnNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn swap<T: QTextCursor_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QTextCursor_swap {
  fn swap(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::swap(QTextCursor & other);
impl<'a> /*trait*/ QTextCursor_swap for (&'a mut QTextCursor) {
  fn swap(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn mergeCharFormat<T: QTextCursor_mergeCharFormat>(&mut self, value: T)  {
     value.mergeCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_mergeCharFormat {
  fn mergeCharFormat(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::mergeCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextCursor_mergeCharFormat for (&'a  QTextCharFormat) {
  fn mergeCharFormat(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn selection<T: QTextCursor_selection>(&mut self, value: T) -> QTextDocumentFragment {
    return value.selection(self);
    // return 1;
  }
}

pub trait QTextCursor_selection {
  fn selection(self, rsthis: &mut QTextCursor) -> QTextDocumentFragment;
}

// proto:  QTextDocumentFragment QTextCursor::selection();
impl<'a> /*trait*/ QTextCursor_selection for () {
  fn selection(self, rsthis: &mut QTextCursor) -> QTextDocumentFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor9selectionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor9selectionEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocumentFragment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn hasComplexSelection<T: QTextCursor_hasComplexSelection>(&mut self, value: T) -> i8 {
    return value.hasComplexSelection(self);
    // return 1;
  }
}

pub trait QTextCursor_hasComplexSelection {
  fn hasComplexSelection(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::hasComplexSelection();
impl<'a> /*trait*/ QTextCursor_hasComplexSelection for () {
  fn hasComplexSelection(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor19hasComplexSelectionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor19hasComplexSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn block<T: QTextCursor_block>(&mut self, value: T) -> QTextBlock {
    return value.block(self);
    // return 1;
  }
}

pub trait QTextCursor_block {
  fn block(self, rsthis: &mut QTextCursor) -> QTextBlock;
}

// proto:  QTextBlock QTextCursor::block();
impl<'a> /*trait*/ QTextCursor_block for () {
  fn block(self, rsthis: &mut QTextCursor) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor5blockEv()};
    let mut ret = unsafe {_ZNK11QTextCursor5blockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertFragment<T: QTextCursor_insertFragment>(&mut self, value: T)  {
     value.insertFragment(self);
    // return 1;
  }
}

pub trait QTextCursor_insertFragment {
  fn insertFragment(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::insertFragment(const QTextDocumentFragment & fragment);
impl<'a> /*trait*/ QTextCursor_insertFragment for (&'a  QTextDocumentFragment) {
  fn insertFragment(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertList<T: QTextCursor_insertList>(&mut self, value: T) -> QTextList {
    return value.insertList(self);
    // return 1;
  }
}

pub trait QTextCursor_insertList {
  fn insertList(self, rsthis: &mut QTextCursor) -> QTextList;
}

// proto:  QTextList * QTextCursor::insertList(const QTextListFormat & format);
impl<'a> /*trait*/ QTextCursor_insertList for (&'a  QTextListFormat) {
  fn insertList(self, rsthis: &mut QTextCursor) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertListERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor10insertListERK15QTextListFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertImage<T: QTextCursor_insertImage>(&mut self, value: T)  {
     value.insertImage(self);
    // return 1;
  }
}

pub trait QTextCursor_insertImage {
  fn insertImage(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::insertImage(const QTextImageFormat & format);
impl<'a> /*trait*/ QTextCursor_insertImage for (&'a  QTextImageFormat) {
  fn insertImage(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertImageERK16QTextImageFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertImageERK16QTextImageFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn keepPositionOnInsert<T: QTextCursor_keepPositionOnInsert>(&mut self, value: T) -> i8 {
    return value.keepPositionOnInsert(self);
    // return 1;
  }
}

pub trait QTextCursor_keepPositionOnInsert {
  fn keepPositionOnInsert(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::keepPositionOnInsert();
impl<'a> /*trait*/ QTextCursor_keepPositionOnInsert for () {
  fn keepPositionOnInsert(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor20keepPositionOnInsertEv()};
    let mut ret = unsafe {_ZNK11QTextCursor20keepPositionOnInsertEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn position<T: QTextCursor_position>(&mut self, value: T) -> i32 {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextCursor_position {
  fn position(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::position();
impl<'a> /*trait*/ QTextCursor_position for () {
  fn position(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor8positionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn isNull<T: QTextCursor_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QTextCursor_isNull {
  fn isNull(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::isNull();
impl<'a> /*trait*/ QTextCursor_isNull for () {
  fn isNull(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor6isNullEv()};
    let mut ret = unsafe {_ZNK11QTextCursor6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn removeSelectedText<T: QTextCursor_removeSelectedText>(&mut self, value: T)  {
     value.removeSelectedText(self);
    // return 1;
  }
}

pub trait QTextCursor_removeSelectedText {
  fn removeSelectedText(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::removeSelectedText();
impl<'a> /*trait*/ QTextCursor_removeSelectedText for () {
  fn removeSelectedText(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor18removeSelectedTextEv()};
     unsafe {_ZN11QTextCursor18removeSelectedTextEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertHtml<T: QTextCursor_insertHtml>(&mut self, value: T)  {
     value.insertHtml(self);
    // return 1;
  }
}

pub trait QTextCursor_insertHtml {
  fn insertHtml(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::insertHtml(const QString & html);
impl<'a> /*trait*/ QTextCursor_insertHtml for (&'a  QString) {
  fn insertHtml(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor10insertHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn isCopyOf<T: QTextCursor_isCopyOf>(&mut self, value: T) -> i8 {
    return value.isCopyOf(self);
    // return 1;
  }
}

pub trait QTextCursor_isCopyOf {
  fn isCopyOf(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::isCopyOf(const QTextCursor & other);
impl<'a> /*trait*/ QTextCursor_isCopyOf for (&'a  QTextCursor) {
  fn isCopyOf(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTextCursor8isCopyOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertFrame<T: QTextCursor_insertFrame>(&mut self, value: T) -> QTextFrame {
    return value.insertFrame(self);
    // return 1;
  }
}

pub trait QTextCursor_insertFrame {
  fn insertFrame(self, rsthis: &mut QTextCursor) -> QTextFrame;
}

// proto:  QTextFrame * QTextCursor::insertFrame(const QTextFrameFormat & format);
impl<'a> /*trait*/ QTextCursor_insertFrame for (&'a  QTextFrameFormat) {
  fn insertFrame(self, rsthis: &mut QTextCursor) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertFrameERK16QTextFrameFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor11insertFrameERK16QTextFrameFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn NewQTextCursor<T: QTextCursor_NewQTextCursor>(value: T) -> QTextCursor {
    let rsthis = value.NewQTextCursor();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCursor_NewQTextCursor {
  fn NewQTextCursor(self) -> QTextCursor;
}

// proto: void QTextCursor::NewQTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextCursor_NewQTextCursor for (&'a  QTextCursor) {
  fn NewQTextCursor(self) -> QTextCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextCursorC1ERKS_(qthis, arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn deleteChar<T: QTextCursor_deleteChar>(&mut self, value: T)  {
     value.deleteChar(self);
    // return 1;
  }
}

pub trait QTextCursor_deleteChar {
  fn deleteChar(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::deleteChar();
impl<'a> /*trait*/ QTextCursor_deleteChar for () {
  fn deleteChar(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10deleteCharEv()};
     unsafe {_ZN11QTextCursor10deleteCharEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn currentFrame<T: QTextCursor_currentFrame>(&mut self, value: T) -> QTextFrame {
    return value.currentFrame(self);
    // return 1;
  }
}

pub trait QTextCursor_currentFrame {
  fn currentFrame(self, rsthis: &mut QTextCursor) -> QTextFrame;
}

// proto:  QTextFrame * QTextCursor::currentFrame();
impl<'a> /*trait*/ QTextCursor_currentFrame for () {
  fn currentFrame(self, rsthis: &mut QTextCursor) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12currentFrameEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12currentFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertBlock<T: QTextCursor_insertBlock>(&mut self, value: T)  {
     value.insertBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_insertBlock {
  fn insertBlock(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::insertBlock();
impl<'a> /*trait*/ QTextCursor_insertBlock for () {
  fn insertBlock(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertBlockEv()};
     unsafe {_ZN11QTextCursor11insertBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTextCursor::NewQTextCursor(const QTextBlock & block);
impl<'a> /*trait*/ QTextCursor_NewQTextCursor for (&'a  QTextBlock) {
  fn NewQTextCursor(self) -> QTextCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1ERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextCursorC1ERK10QTextBlock(qthis, arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertTable<T: QTextCursor_insertTable>(&mut self, value: T) -> QTextTable {
    return value.insertTable(self);
    // return 1;
  }
}

pub trait QTextCursor_insertTable {
  fn insertTable(self, rsthis: &mut QTextCursor) -> QTextTable;
}

// proto:  QTextTable * QTextCursor::insertTable(int rows, int cols);
impl<'a> /*trait*/ QTextCursor_insertTable for (i32, i32) {
  fn insertTable(self, rsthis: &mut QTextCursor) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertTableEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QTextCursor11insertTableEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTextTable{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextCursor::NewQTextCursor();
impl<'a> /*trait*/ QTextCursor_NewQTextCursor for () {
  fn NewQTextCursor(self) -> QTextCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1Ev()};
    unsafe {_ZN11QTextCursorC1Ev(qthis)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn atStart<T: QTextCursor_atStart>(&mut self, value: T) -> i8 {
    return value.atStart(self);
    // return 1;
  }
}

pub trait QTextCursor_atStart {
  fn atStart(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::atStart();
impl<'a> /*trait*/ QTextCursor_atStart for () {
  fn atStart(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor7atStartEv()};
    let mut ret = unsafe {_ZNK11QTextCursor7atStartEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn selectionStart<T: QTextCursor_selectionStart>(&mut self, value: T) -> i32 {
    return value.selectionStart(self);
    // return 1;
  }
}

pub trait QTextCursor_selectionStart {
  fn selectionStart(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::selectionStart();
impl<'a> /*trait*/ QTextCursor_selectionStart for () {
  fn selectionStart(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor14selectionStartEv()};
    let mut ret = unsafe {_ZNK11QTextCursor14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn selectedTableCells<T: QTextCursor_selectedTableCells>(&mut self, value: T)  {
     value.selectedTableCells(self);
    // return 1;
  }
}

pub trait QTextCursor_selectedTableCells {
  fn selectedTableCells(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::selectedTableCells(int * firstRow, int * numRows, int * firstColumn, int * numColumns);
impl<'a> /*trait*/ QTextCursor_selectedTableCells for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn selectedTableCells(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn endEditBlock<T: QTextCursor_endEditBlock>(&mut self, value: T)  {
     value.endEditBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_endEditBlock {
  fn endEditBlock(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::endEditBlock();
impl<'a> /*trait*/ QTextCursor_endEditBlock for () {
  fn endEditBlock(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor12endEditBlockEv()};
     unsafe {_ZN11QTextCursor12endEditBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn selectedText<T: QTextCursor_selectedText>(&mut self, value: T) -> QString {
    return value.selectedText(self);
    // return 1;
  }
}

pub trait QTextCursor_selectedText {
  fn selectedText(self, rsthis: &mut QTextCursor) -> QString;
}

// proto:  QString QTextCursor::selectedText();
impl<'a> /*trait*/ QTextCursor_selectedText for () {
  fn selectedText(self, rsthis: &mut QTextCursor) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12selectedTextEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12selectedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn positionInBlock<T: QTextCursor_positionInBlock>(&mut self, value: T) -> i32 {
    return value.positionInBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_positionInBlock {
  fn positionInBlock(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::positionInBlock();
impl<'a> /*trait*/ QTextCursor_positionInBlock for () {
  fn positionInBlock(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor15positionInBlockEv()};
    let mut ret = unsafe {_ZNK11QTextCursor15positionInBlockEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn hasSelection<T: QTextCursor_hasSelection>(&mut self, value: T) -> i8 {
    return value.hasSelection(self);
    // return 1;
  }
}

pub trait QTextCursor_hasSelection {
  fn hasSelection(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::hasSelection();
impl<'a> /*trait*/ QTextCursor_hasSelection for () {
  fn hasSelection(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12hasSelectionEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12hasSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn atEnd<T: QTextCursor_atEnd>(&mut self, value: T) -> i8 {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QTextCursor_atEnd {
  fn atEnd(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::atEnd();
impl<'a> /*trait*/ QTextCursor_atEnd for () {
  fn atEnd(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor5atEndEv()};
    let mut ret = unsafe {_ZNK11QTextCursor5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCursor::insertImage(const QString & name);
impl<'a> /*trait*/ QTextCursor_insertImage for (&'a  QString) {
  fn insertImage(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertImageERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertImageERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn atBlockStart<T: QTextCursor_atBlockStart>(&mut self, value: T) -> i8 {
    return value.atBlockStart(self);
    // return 1;
  }
}

pub trait QTextCursor_atBlockStart {
  fn atBlockStart(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::atBlockStart();
impl<'a> /*trait*/ QTextCursor_atBlockStart for () {
  fn atBlockStart(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12atBlockStartEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12atBlockStartEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn insertText<T: QTextCursor_insertText>(&mut self, value: T)  {
     value.insertText(self);
    // return 1;
  }
}

pub trait QTextCursor_insertText {
  fn insertText(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::insertText(const QString & text);
impl<'a> /*trait*/ QTextCursor_insertText for (&'a  QString) {
  fn insertText(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor10insertTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn visualNavigation<T: QTextCursor_visualNavigation>(&mut self, value: T) -> i8 {
    return value.visualNavigation(self);
    // return 1;
  }
}

pub trait QTextCursor_visualNavigation {
  fn visualNavigation(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::visualNavigation();
impl<'a> /*trait*/ QTextCursor_visualNavigation for () {
  fn visualNavigation(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor16visualNavigationEv()};
    let mut ret = unsafe {_ZNK11QTextCursor16visualNavigationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn atBlockEnd<T: QTextCursor_atBlockEnd>(&mut self, value: T) -> i8 {
    return value.atBlockEnd(self);
    // return 1;
  }
}

pub trait QTextCursor_atBlockEnd {
  fn atBlockEnd(self, rsthis: &mut QTextCursor) -> i8;
}

// proto:  bool QTextCursor::atBlockEnd();
impl<'a> /*trait*/ QTextCursor_atBlockEnd for () {
  fn atBlockEnd(self, rsthis: &mut QTextCursor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor10atBlockEndEv()};
    let mut ret = unsafe {_ZNK11QTextCursor10atBlockEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format);
impl<'a> /*trait*/ QTextCursor_insertBlock for (&'a  QTextBlockFormat) {
  fn insertBlock(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertBlockERK16QTextBlockFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertBlockERK16QTextBlockFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn currentList<T: QTextCursor_currentList>(&mut self, value: T) -> QTextList {
    return value.currentList(self);
    // return 1;
  }
}

pub trait QTextCursor_currentList {
  fn currentList(self, rsthis: &mut QTextCursor) -> QTextList;
}

// proto:  QTextList * QTextCursor::currentList();
impl<'a> /*trait*/ QTextCursor_currentList for () {
  fn currentList(self, rsthis: &mut QTextCursor) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor11currentListEv()};
    let mut ret = unsafe {_ZNK11QTextCursor11currentListEv(rsthis.qclsinst)};
    let mut ret1 = QTextList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextCursor::insertBlock(const QTextBlockFormat & format, const QTextCharFormat & charFormat);
impl<'a> /*trait*/ QTextCursor_insertBlock for (&'a  QTextBlockFormat, &'a  QTextCharFormat) {
  fn insertBlock(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn mergeBlockCharFormat<T: QTextCursor_mergeBlockCharFormat>(&mut self, value: T)  {
     value.mergeBlockCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_mergeBlockCharFormat {
  fn mergeBlockCharFormat(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::mergeBlockCharFormat(const QTextCharFormat & modifier);
impl<'a> /*trait*/ QTextCursor_mergeBlockCharFormat for (&'a  QTextCharFormat) {
  fn mergeBlockCharFormat(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn setCharFormat<T: QTextCursor_setCharFormat>(&mut self, value: T)  {
     value.setCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_setCharFormat {
  fn setCharFormat(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::setCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextCursor_setCharFormat for (&'a  QTextCharFormat) {
  fn setCharFormat(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor13setCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor13setCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn verticalMovementX<T: QTextCursor_verticalMovementX>(&mut self, value: T) -> i32 {
    return value.verticalMovementX(self);
    // return 1;
  }
}

pub trait QTextCursor_verticalMovementX {
  fn verticalMovementX(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::verticalMovementX();
impl<'a> /*trait*/ QTextCursor_verticalMovementX for () {
  fn verticalMovementX(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor17verticalMovementXEv()};
    let mut ret = unsafe {_ZNK11QTextCursor17verticalMovementXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn blockNumber<T: QTextCursor_blockNumber>(&mut self, value: T) -> i32 {
    return value.blockNumber(self);
    // return 1;
  }
}

pub trait QTextCursor_blockNumber {
  fn blockNumber(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::blockNumber();
impl<'a> /*trait*/ QTextCursor_blockNumber for () {
  fn blockNumber(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor11blockNumberEv()};
    let mut ret = unsafe {_ZNK11QTextCursor11blockNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn joinPreviousEditBlock<T: QTextCursor_joinPreviousEditBlock>(&mut self, value: T)  {
     value.joinPreviousEditBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_joinPreviousEditBlock {
  fn joinPreviousEditBlock(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::joinPreviousEditBlock();
impl<'a> /*trait*/ QTextCursor_joinPreviousEditBlock for () {
  fn joinPreviousEditBlock(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor21joinPreviousEditBlockEv()};
     unsafe {_ZN11QTextCursor21joinPreviousEditBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTextCursor::NewQTextCursor(QTextDocument * document);
impl<'a> /*trait*/ QTextCursor_NewQTextCursor for (&'a mut QTextDocument) {
  fn NewQTextCursor(self) -> QTextCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextCursorC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTextCursor::insertText(const QString & text, const QTextCharFormat & format);
impl<'a> /*trait*/ QTextCursor_insertText for (&'a  QString, &'a  QTextCharFormat) {
  fn insertText(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn mergeBlockFormat<T: QTextCursor_mergeBlockFormat>(&mut self, value: T)  {
     value.mergeBlockFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_mergeBlockFormat {
  fn mergeBlockFormat(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::mergeBlockFormat(const QTextBlockFormat & modifier);
impl<'a> /*trait*/ QTextCursor_mergeBlockFormat for (&'a  QTextBlockFormat) {
  fn mergeBlockFormat(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn blockFormat<T: QTextCursor_blockFormat>(&mut self, value: T) -> QTextBlockFormat {
    return value.blockFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_blockFormat {
  fn blockFormat(self, rsthis: &mut QTextCursor) -> QTextBlockFormat;
}

// proto:  QTextBlockFormat QTextCursor::blockFormat();
impl<'a> /*trait*/ QTextCursor_blockFormat for () {
  fn blockFormat(self, rsthis: &mut QTextCursor) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor11blockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextCursor11blockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextCursor::insertImage(const QImage & image, const QString & name);
impl<'a> /*trait*/ QTextCursor_insertImage for (&'a  QImage, &'a  QString) {
  fn insertImage(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertImageERK6QImageRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor11insertImageERK6QImageRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn beginEditBlock<T: QTextCursor_beginEditBlock>(&mut self, value: T)  {
     value.beginEditBlock(self);
    // return 1;
  }
}

pub trait QTextCursor_beginEditBlock {
  fn beginEditBlock(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::beginEditBlock();
impl<'a> /*trait*/ QTextCursor_beginEditBlock for () {
  fn beginEditBlock(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14beginEditBlockEv()};
     unsafe {_ZN11QTextCursor14beginEditBlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn anchor<T: QTextCursor_anchor>(&mut self, value: T) -> i32 {
    return value.anchor(self);
    // return 1;
  }
}

pub trait QTextCursor_anchor {
  fn anchor(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::anchor();
impl<'a> /*trait*/ QTextCursor_anchor for () {
  fn anchor(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor6anchorEv()};
    let mut ret = unsafe {_ZNK11QTextCursor6anchorEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn charFormat<T: QTextCursor_charFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.charFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_charFormat {
  fn charFormat(self, rsthis: &mut QTextCursor) -> QTextCharFormat;
}

// proto:  QTextCharFormat QTextCursor::charFormat();
impl<'a> /*trait*/ QTextCursor_charFormat for () {
  fn charFormat(self, rsthis: &mut QTextCursor) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor10charFormatEv()};
    let mut ret = unsafe {_ZNK11QTextCursor10charFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn deletePreviousChar<T: QTextCursor_deletePreviousChar>(&mut self, value: T)  {
     value.deletePreviousChar(self);
    // return 1;
  }
}

pub trait QTextCursor_deletePreviousChar {
  fn deletePreviousChar(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::deletePreviousChar();
impl<'a> /*trait*/ QTextCursor_deletePreviousChar for () {
  fn deletePreviousChar(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor18deletePreviousCharEv()};
     unsafe {_ZN11QTextCursor18deletePreviousCharEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn FreeQTextCursor<T: QTextCursor_FreeQTextCursor>(&mut self, value: T)  {
     value.FreeQTextCursor(self);
    // return 1;
  }
}

pub trait QTextCursor_FreeQTextCursor {
  fn FreeQTextCursor(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::FreeQTextCursor();
impl<'a> /*trait*/ QTextCursor_FreeQTextCursor for () {
  fn FreeQTextCursor(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorD0Ev()};
     unsafe {_ZN11QTextCursorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn clearSelection<T: QTextCursor_clearSelection>(&mut self, value: T)  {
     value.clearSelection(self);
    // return 1;
  }
}

pub trait QTextCursor_clearSelection {
  fn clearSelection(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::clearSelection();
impl<'a> /*trait*/ QTextCursor_clearSelection for () {
  fn clearSelection(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14clearSelectionEv()};
     unsafe {_ZN11QTextCursor14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn setVisualNavigation<T: QTextCursor_setVisualNavigation>(&mut self, value: T)  {
     value.setVisualNavigation(self);
    // return 1;
  }
}

pub trait QTextCursor_setVisualNavigation {
  fn setVisualNavigation(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::setVisualNavigation(bool b);
impl<'a> /*trait*/ QTextCursor_setVisualNavigation for (i8) {
  fn setVisualNavigation(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor19setVisualNavigationEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QTextCursor19setVisualNavigationEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn setBlockCharFormat<T: QTextCursor_setBlockCharFormat>(&mut self, value: T)  {
     value.setBlockCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_setBlockCharFormat {
  fn setBlockCharFormat(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::setBlockCharFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextCursor_setBlockCharFormat for (&'a  QTextCharFormat) {
  fn setBlockCharFormat(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn currentTable<T: QTextCursor_currentTable>(&mut self, value: T) -> QTextTable {
    return value.currentTable(self);
    // return 1;
  }
}

pub trait QTextCursor_currentTable {
  fn currentTable(self, rsthis: &mut QTextCursor) -> QTextTable;
}

// proto:  QTextTable * QTextCursor::currentTable();
impl<'a> /*trait*/ QTextCursor_currentTable for () {
  fn currentTable(self, rsthis: &mut QTextCursor) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12currentTableEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12currentTableEv(rsthis.qclsinst)};
    let mut ret1 = QTextTable{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn setKeepPositionOnInsert<T: QTextCursor_setKeepPositionOnInsert>(&mut self, value: T)  {
     value.setKeepPositionOnInsert(self);
    // return 1;
  }
}

pub trait QTextCursor_setKeepPositionOnInsert {
  fn setKeepPositionOnInsert(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::setKeepPositionOnInsert(bool b);
impl<'a> /*trait*/ QTextCursor_setKeepPositionOnInsert for (i8) {
  fn setKeepPositionOnInsert(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor23setKeepPositionOnInsertEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QTextCursor23setKeepPositionOnInsertEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn setVerticalMovementX<T: QTextCursor_setVerticalMovementX>(&mut self, value: T)  {
     value.setVerticalMovementX(self);
    // return 1;
  }
}

pub trait QTextCursor_setVerticalMovementX {
  fn setVerticalMovementX(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::setVerticalMovementX(int x);
impl<'a> /*trait*/ QTextCursor_setVerticalMovementX for (i32) {
  fn setVerticalMovementX(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor20setVerticalMovementXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextCursor20setVerticalMovementXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn document<T: QTextCursor_document>(&mut self, value: T) -> QTextDocument {
    return value.document(self);
    // return 1;
  }
}

pub trait QTextCursor_document {
  fn document(self, rsthis: &mut QTextCursor) -> QTextDocument;
}

// proto:  QTextDocument * QTextCursor::document();
impl<'a> /*trait*/ QTextCursor_document for () {
  fn document(self, rsthis: &mut QTextCursor) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor8documentEv()};
    let mut ret = unsafe {_ZNK11QTextCursor8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTextTable * QTextCursor::insertTable(int rows, int cols, const QTextTableFormat & format);
impl<'a> /*trait*/ QTextCursor_insertTable for (i32, i32, &'a  QTextTableFormat) {
  fn insertTable(self, rsthis: &mut QTextCursor) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor11insertTableEiiRK16QTextTableFormat()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor11insertTableEiiRK16QTextTableFormat(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QTextTable{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextCursor::NewQTextCursor(QTextFrame * frame);
impl<'a> /*trait*/ QTextCursor_NewQTextCursor for (&'a mut QTextFrame) {
  fn NewQTextCursor(self) -> QTextCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursorC1EP10QTextFrame()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextCursorC1EP10QTextFrame(qthis, arg0)};
    let rsthis = QTextCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn selectionEnd<T: QTextCursor_selectionEnd>(&mut self, value: T) -> i32 {
    return value.selectionEnd(self);
    // return 1;
  }
}

pub trait QTextCursor_selectionEnd {
  fn selectionEnd(self, rsthis: &mut QTextCursor) -> i32;
}

// proto:  int QTextCursor::selectionEnd();
impl<'a> /*trait*/ QTextCursor_selectionEnd for () {
  fn selectionEnd(self, rsthis: &mut QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor12selectionEndEv()};
    let mut ret = unsafe {_ZNK11QTextCursor12selectionEndEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn setBlockFormat<T: QTextCursor_setBlockFormat>(&mut self, value: T)  {
     value.setBlockFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_setBlockFormat {
  fn setBlockFormat(self, rsthis: &mut QTextCursor) ;
}

// proto:  void QTextCursor::setBlockFormat(const QTextBlockFormat & format);
impl<'a> /*trait*/ QTextCursor_setBlockFormat for (&'a  QTextBlockFormat) {
  fn setBlockFormat(self, rsthis: &mut QTextCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn createList<T: QTextCursor_createList>(&mut self, value: T) -> QTextList {
    return value.createList(self);
    // return 1;
  }
}

pub trait QTextCursor_createList {
  fn createList(self, rsthis: &mut QTextCursor) -> QTextList;
}

// proto:  QTextList * QTextCursor::createList(const QTextListFormat & format);
impl<'a> /*trait*/ QTextCursor_createList for (&'a  QTextListFormat) {
  fn createList(self, rsthis: &mut QTextCursor) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextCursor10createListERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTextCursor10createListERK15QTextListFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCursor {
  pub fn blockCharFormat<T: QTextCursor_blockCharFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.blockCharFormat(self);
    // return 1;
  }
}

pub trait QTextCursor_blockCharFormat {
  fn blockCharFormat(self, rsthis: &mut QTextCursor) -> QTextCharFormat;
}

// proto:  QTextCharFormat QTextCursor::blockCharFormat();
impl<'a> /*trait*/ QTextCursor_blockCharFormat for () {
  fn blockCharFormat(self, rsthis: &mut QTextCursor) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextCursor15blockCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextCursor15blockCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

