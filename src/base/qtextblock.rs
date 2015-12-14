// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextdocument::QTextDocument;
use super::qtextblockuserdata::QTextBlockUserData;
use super::qstring::QString;
use super::qtextlist::QTextList;
use super::qtextlayout::QTextLayout;
use super::qtextblockformat::QTextBlockFormat;
use super::qtextcharformat::QTextCharFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QTextDocument * QTextBlock::document();
  fn _ZNK10QTextBlock8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextBlock::previous();
  fn _ZNK10QTextBlock8previousEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextBlock::length();
  fn _ZNK10QTextBlock6lengthEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextBlockUserData * QTextBlock::userData();
  fn _ZNK10QTextBlock8userDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBlock::NewQTextBlock(const QTextBlock & o);
  fn _ZN10QTextBlockC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextBlock::text();
  fn _ZNK10QTextBlock4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextBlock::lineCount();
  fn _ZNK10QTextBlock9lineCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBlock::contains(int position);
  fn _ZNK10QTextBlock8containsEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QTextBlock::blockNumber();
  fn _ZNK10QTextBlock11blockNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlock::setRevision(int rev);
  fn _ZN10QTextBlock11setRevisionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextBlock::setVisible(bool visible);
  fn _ZN10QTextBlock10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextBlock::clearLayout();
  fn _ZN10QTextBlock11clearLayoutEv(qthis: *mut c_void) ;
  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
  fn _ZNK10QTextBlock9docHandleEv(qthis: *mut c_void) ;
  // proto:  int QTextBlock::userState();
  fn _ZNK10QTextBlock9userStateEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextBlock::charFormatIndex();
  fn _ZNK10QTextBlock15charFormatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextBlock::revision();
  fn _ZNK10QTextBlock8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextBlock::position();
  fn _ZNK10QTextBlock8positionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBlock::isValid();
  fn _ZNK10QTextBlock7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTextList * QTextBlock::textList();
  fn _ZNK10QTextBlock8textListEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextLayout * QTextBlock::layout();
  fn _ZNK10QTextBlock6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
  fn _ZN10QTextBlock11setUserDataEP18QTextBlockUserData(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextBlock::blockFormatIndex();
  fn _ZNK10QTextBlock16blockFormatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlock::setUserState(int state);
  fn _ZN10QTextBlock12setUserStateEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTextBlock::fragmentIndex();
  fn _ZNK10QTextBlock13fragmentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBlock::isVisible();
  fn _ZNK10QTextBlock9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextBlock::setLineCount(int count);
  fn _ZN10QTextBlock12setLineCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QTextBlock QTextBlock::next();
  fn _ZNK10QTextBlock4nextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlockFormat QTextBlock::blockFormat();
  fn _ZNK10QTextBlock11blockFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBlock::NewQTextBlock();
  fn _ZN10QTextBlockC1Ev(qthis: *mut c_void) ;
  // proto:  int QTextBlock::firstLineNumber();
  fn _ZNK10QTextBlock15firstLineNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCharFormat QTextBlock::charFormat();
  fn _ZNK10QTextBlock10charFormatEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QTextBlock)=16
pub struct QTextBlock {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBlock {
  pub fn document<T: QTextBlock_document>(&mut self, value: T) -> QTextDocument {
    return value.document(self);
    // return 1;
  }
}

pub trait QTextBlock_document {
  fn document(self, rsthis: &mut QTextBlock) -> QTextDocument;
}

// proto:  const QTextDocument * QTextBlock::document();
impl<'a> /*trait*/ QTextBlock_document for () {
  fn document(self, rsthis: &mut QTextBlock) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8documentEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn previous<T: QTextBlock_previous>(&mut self, value: T) -> QTextBlock {
    return value.previous(self);
    // return 1;
  }
}

pub trait QTextBlock_previous {
  fn previous(self, rsthis: &mut QTextBlock) -> QTextBlock;
}

// proto:  QTextBlock QTextBlock::previous();
impl<'a> /*trait*/ QTextBlock_previous for () {
  fn previous(self, rsthis: &mut QTextBlock) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8previousEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8previousEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn length<T: QTextBlock_length>(&mut self, value: T) -> i32 {
    return value.length(self);
    // return 1;
  }
}

pub trait QTextBlock_length {
  fn length(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::length();
impl<'a> /*trait*/ QTextBlock_length for () {
  fn length(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6lengthEv()};
    let mut ret = unsafe {_ZNK10QTextBlock6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn userData<T: QTextBlock_userData>(&mut self, value: T) -> QTextBlockUserData {
    return value.userData(self);
    // return 1;
  }
}

pub trait QTextBlock_userData {
  fn userData(self, rsthis: &mut QTextBlock) -> QTextBlockUserData;
}

// proto:  QTextBlockUserData * QTextBlock::userData();
impl<'a> /*trait*/ QTextBlock_userData for () {
  fn userData(self, rsthis: &mut QTextBlock) -> QTextBlockUserData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8userDataEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8userDataEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockUserData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn NewQTextBlock<T: QTextBlock_NewQTextBlock>(value: T) -> QTextBlock {
    let rsthis = value.NewQTextBlock();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlock_NewQTextBlock {
  fn NewQTextBlock(self) -> QTextBlock;
}

// proto: void QTextBlock::NewQTextBlock(const QTextBlock & o);
impl<'a> /*trait*/ QTextBlock_NewQTextBlock for (&'a  QTextBlock) {
  fn NewQTextBlock(self) -> QTextBlock {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlockC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextBlockC1ERKS_(qthis, arg0)};
    let rsthis = QTextBlock{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn text<T: QTextBlock_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QTextBlock_text {
  fn text(self, rsthis: &mut QTextBlock) -> QString;
}

// proto:  QString QTextBlock::text();
impl<'a> /*trait*/ QTextBlock_text for () {
  fn text(self, rsthis: &mut QTextBlock) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4textEv()};
    let mut ret = unsafe {_ZNK10QTextBlock4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn lineCount<T: QTextBlock_lineCount>(&mut self, value: T) -> i32 {
    return value.lineCount(self);
    // return 1;
  }
}

pub trait QTextBlock_lineCount {
  fn lineCount(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::lineCount();
impl<'a> /*trait*/ QTextBlock_lineCount for () {
  fn lineCount(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9lineCountEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn contains<T: QTextBlock_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QTextBlock_contains {
  fn contains(self, rsthis: &mut QTextBlock) -> i8;
}

// proto:  bool QTextBlock::contains(int position);
impl<'a> /*trait*/ QTextBlock_contains for (i32) {
  fn contains(self, rsthis: &mut QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8containsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTextBlock8containsEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn blockNumber<T: QTextBlock_blockNumber>(&mut self, value: T) -> i32 {
    return value.blockNumber(self);
    // return 1;
  }
}

pub trait QTextBlock_blockNumber {
  fn blockNumber(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::blockNumber();
impl<'a> /*trait*/ QTextBlock_blockNumber for () {
  fn blockNumber(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockNumberEv()};
    let mut ret = unsafe {_ZNK10QTextBlock11blockNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setRevision<T: QTextBlock_setRevision>(&mut self, value: T)  {
     value.setRevision(self);
    // return 1;
  }
}

pub trait QTextBlock_setRevision {
  fn setRevision(self, rsthis: &mut QTextBlock) ;
}

// proto:  void QTextBlock::setRevision(int rev);
impl<'a> /*trait*/ QTextBlock_setRevision for (i32) {
  fn setRevision(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setRevisionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock11setRevisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setVisible<T: QTextBlock_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QTextBlock_setVisible {
  fn setVisible(self, rsthis: &mut QTextBlock) ;
}

// proto:  void QTextBlock::setVisible(bool visible);
impl<'a> /*trait*/ QTextBlock_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTextBlock10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn clearLayout<T: QTextBlock_clearLayout>(&mut self, value: T)  {
     value.clearLayout(self);
    // return 1;
  }
}

pub trait QTextBlock_clearLayout {
  fn clearLayout(self, rsthis: &mut QTextBlock) ;
}

// proto:  void QTextBlock::clearLayout();
impl<'a> /*trait*/ QTextBlock_clearLayout for () {
  fn clearLayout(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11clearLayoutEv()};
     unsafe {_ZN10QTextBlock11clearLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn docHandle<T: QTextBlock_docHandle>(&mut self, value: T)  {
     value.docHandle(self);
    // return 1;
  }
}

pub trait QTextBlock_docHandle {
  fn docHandle(self, rsthis: &mut QTextBlock) ;
}

// proto:  QTextDocumentPrivate * QTextBlock::docHandle();
impl<'a> /*trait*/ QTextBlock_docHandle for () {
  fn docHandle(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9docHandleEv()};
     unsafe {_ZNK10QTextBlock9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn userState<T: QTextBlock_userState>(&mut self, value: T) -> i32 {
    return value.userState(self);
    // return 1;
  }
}

pub trait QTextBlock_userState {
  fn userState(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::userState();
impl<'a> /*trait*/ QTextBlock_userState for () {
  fn userState(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9userStateEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9userStateEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn charFormatIndex<T: QTextBlock_charFormatIndex>(&mut self, value: T) -> i32 {
    return value.charFormatIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_charFormatIndex {
  fn charFormatIndex(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::charFormatIndex();
impl<'a> /*trait*/ QTextBlock_charFormatIndex for () {
  fn charFormatIndex(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15charFormatIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock15charFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn revision<T: QTextBlock_revision>(&mut self, value: T) -> i32 {
    return value.revision(self);
    // return 1;
  }
}

pub trait QTextBlock_revision {
  fn revision(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::revision();
impl<'a> /*trait*/ QTextBlock_revision for () {
  fn revision(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8revisionEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn position<T: QTextBlock_position>(&mut self, value: T) -> i32 {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextBlock_position {
  fn position(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::position();
impl<'a> /*trait*/ QTextBlock_position for () {
  fn position(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8positionEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn isValid<T: QTextBlock_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextBlock_isValid {
  fn isValid(self, rsthis: &mut QTextBlock) -> i8;
}

// proto:  bool QTextBlock::isValid();
impl<'a> /*trait*/ QTextBlock_isValid for () {
  fn isValid(self, rsthis: &mut QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock7isValidEv()};
    let mut ret = unsafe {_ZNK10QTextBlock7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn textList<T: QTextBlock_textList>(&mut self, value: T) -> QTextList {
    return value.textList(self);
    // return 1;
  }
}

pub trait QTextBlock_textList {
  fn textList(self, rsthis: &mut QTextBlock) -> QTextList;
}

// proto:  QTextList * QTextBlock::textList();
impl<'a> /*trait*/ QTextBlock_textList for () {
  fn textList(self, rsthis: &mut QTextBlock) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8textListEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8textListEv(rsthis.qclsinst)};
    let mut ret1 = QTextList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn layout<T: QTextBlock_layout>(&mut self, value: T) -> QTextLayout {
    return value.layout(self);
    // return 1;
  }
}

pub trait QTextBlock_layout {
  fn layout(self, rsthis: &mut QTextBlock) -> QTextLayout;
}

// proto:  QTextLayout * QTextBlock::layout();
impl<'a> /*trait*/ QTextBlock_layout for () {
  fn layout(self, rsthis: &mut QTextBlock) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6layoutEv()};
    let mut ret = unsafe {_ZNK10QTextBlock6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QTextLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setUserData<T: QTextBlock_setUserData>(&mut self, value: T)  {
     value.setUserData(self);
    // return 1;
  }
}

pub trait QTextBlock_setUserData {
  fn setUserData(self, rsthis: &mut QTextBlock) ;
}

// proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
impl<'a> /*trait*/ QTextBlock_setUserData for (&'a mut QTextBlockUserData) {
  fn setUserData(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setUserDataEP18QTextBlockUserData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextBlock11setUserDataEP18QTextBlockUserData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn blockFormatIndex<T: QTextBlock_blockFormatIndex>(&mut self, value: T) -> i32 {
    return value.blockFormatIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_blockFormatIndex {
  fn blockFormatIndex(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::blockFormatIndex();
impl<'a> /*trait*/ QTextBlock_blockFormatIndex for () {
  fn blockFormatIndex(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock16blockFormatIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock16blockFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setUserState<T: QTextBlock_setUserState>(&mut self, value: T)  {
     value.setUserState(self);
    // return 1;
  }
}

pub trait QTextBlock_setUserState {
  fn setUserState(self, rsthis: &mut QTextBlock) ;
}

// proto:  void QTextBlock::setUserState(int state);
impl<'a> /*trait*/ QTextBlock_setUserState for (i32) {
  fn setUserState(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setUserStateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock12setUserStateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn fragmentIndex<T: QTextBlock_fragmentIndex>(&mut self, value: T) -> i32 {
    return value.fragmentIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_fragmentIndex {
  fn fragmentIndex(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::fragmentIndex();
impl<'a> /*trait*/ QTextBlock_fragmentIndex for () {
  fn fragmentIndex(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock13fragmentIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock13fragmentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn isVisible<T: QTextBlock_isVisible>(&mut self, value: T) -> i8 {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QTextBlock_isVisible {
  fn isVisible(self, rsthis: &mut QTextBlock) -> i8;
}

// proto:  bool QTextBlock::isVisible();
impl<'a> /*trait*/ QTextBlock_isVisible for () {
  fn isVisible(self, rsthis: &mut QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9isVisibleEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setLineCount<T: QTextBlock_setLineCount>(&mut self, value: T)  {
     value.setLineCount(self);
    // return 1;
  }
}

pub trait QTextBlock_setLineCount {
  fn setLineCount(self, rsthis: &mut QTextBlock) ;
}

// proto:  void QTextBlock::setLineCount(int count);
impl<'a> /*trait*/ QTextBlock_setLineCount for (i32) {
  fn setLineCount(self, rsthis: &mut QTextBlock)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setLineCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock12setLineCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn next<T: QTextBlock_next>(&mut self, value: T) -> QTextBlock {
    return value.next(self);
    // return 1;
  }
}

pub trait QTextBlock_next {
  fn next(self, rsthis: &mut QTextBlock) -> QTextBlock;
}

// proto:  QTextBlock QTextBlock::next();
impl<'a> /*trait*/ QTextBlock_next for () {
  fn next(self, rsthis: &mut QTextBlock) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4nextEv()};
    let mut ret = unsafe {_ZNK10QTextBlock4nextEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn blockFormat<T: QTextBlock_blockFormat>(&mut self, value: T) -> QTextBlockFormat {
    return value.blockFormat(self);
    // return 1;
  }
}

pub trait QTextBlock_blockFormat {
  fn blockFormat(self, rsthis: &mut QTextBlock) -> QTextBlockFormat;
}

// proto:  QTextBlockFormat QTextBlock::blockFormat();
impl<'a> /*trait*/ QTextBlock_blockFormat for () {
  fn blockFormat(self, rsthis: &mut QTextBlock) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockFormatEv()};
    let mut ret = unsafe {_ZNK10QTextBlock11blockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextBlock::NewQTextBlock();
impl<'a> /*trait*/ QTextBlock_NewQTextBlock for () {
  fn NewQTextBlock(self) -> QTextBlock {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlockC1Ev()};
    unsafe {_ZN10QTextBlockC1Ev(qthis)};
    let rsthis = QTextBlock{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn firstLineNumber<T: QTextBlock_firstLineNumber>(&mut self, value: T) -> i32 {
    return value.firstLineNumber(self);
    // return 1;
  }
}

pub trait QTextBlock_firstLineNumber {
  fn firstLineNumber(self, rsthis: &mut QTextBlock) -> i32;
}

// proto:  int QTextBlock::firstLineNumber();
impl<'a> /*trait*/ QTextBlock_firstLineNumber for () {
  fn firstLineNumber(self, rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15firstLineNumberEv()};
    let mut ret = unsafe {_ZNK10QTextBlock15firstLineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn charFormat<T: QTextBlock_charFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.charFormat(self);
    // return 1;
  }
}

pub trait QTextBlock_charFormat {
  fn charFormat(self, rsthis: &mut QTextBlock) -> QTextCharFormat;
}

// proto:  QTextCharFormat QTextBlock::charFormat();
impl<'a> /*trait*/ QTextBlock_charFormat for () {
  fn charFormat(self, rsthis: &mut QTextBlock) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock10charFormatEv()};
    let mut ret = unsafe {_ZNK10QTextBlock10charFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

