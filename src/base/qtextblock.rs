// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextblockuserdata::QTextBlockUserData;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QTextDocument * QTextBlock::document();
  fn _ZNK10QTextBlock8documentEv() -> i32;
  // proto: QTextBlock QTextBlock::previous();
  fn _ZNK10QTextBlock8previousEv() -> i32;
  // proto: int QTextBlock::length();
  fn _ZNK10QTextBlock6lengthEv() -> i32;
  // proto: QTextBlockUserData * QTextBlock::userData();
  fn _ZNK10QTextBlock8userDataEv() -> i32;
  // proto: void QTextBlock::NewQTextBlock(const QTextBlock & o);
  fn _ZN10QTextBlockC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QString QTextBlock::text();
  fn _ZNK10QTextBlock4textEv() -> i32;
  // proto: int QTextBlock::lineCount();
  fn _ZNK10QTextBlock9lineCountEv() -> i32;
  // proto: bool QTextBlock::contains(int position);
  fn _ZNK10QTextBlock8containsEi(arg0: c_int) -> i32;
  // proto: int QTextBlock::blockNumber();
  fn _ZNK10QTextBlock11blockNumberEv() -> i32;
  // proto: void QTextBlock::setRevision(int rev);
  fn _ZN10QTextBlock11setRevisionEi(arg0: c_int) -> i32;
  // proto: void QTextBlock::setVisible(bool visible);
  fn _ZN10QTextBlock10setVisibleEb(arg0: int8_t) -> i32;
  // proto: void QTextBlock::clearLayout();
  fn _ZN10QTextBlock11clearLayoutEv() -> i32;
  // proto: QTextDocumentPrivate * QTextBlock::docHandle();
  fn _ZNK10QTextBlock9docHandleEv() -> i32;
  // proto: int QTextBlock::userState();
  fn _ZNK10QTextBlock9userStateEv() -> i32;
  // proto: int QTextBlock::charFormatIndex();
  fn _ZNK10QTextBlock15charFormatIndexEv() -> i32;
  // proto: int QTextBlock::revision();
  fn _ZNK10QTextBlock8revisionEv() -> i32;
  // proto: int QTextBlock::position();
  fn _ZNK10QTextBlock8positionEv() -> i32;
  // proto: bool QTextBlock::isValid();
  fn _ZNK10QTextBlock7isValidEv() -> i32;
  // proto: QTextList * QTextBlock::textList();
  fn _ZNK10QTextBlock8textListEv() -> i32;
  // proto: QTextLayout * QTextBlock::layout();
  fn _ZNK10QTextBlock6layoutEv() -> i32;
  // proto: void QTextBlock::setUserData(QTextBlockUserData * data);
  fn _ZN10QTextBlock11setUserDataEP18QTextBlockUserData(arg0: *mut c_void) -> i32;
  // proto: int QTextBlock::blockFormatIndex();
  fn _ZNK10QTextBlock16blockFormatIndexEv() -> i32;
  // proto: void QTextBlock::setUserState(int state);
  fn _ZN10QTextBlock12setUserStateEi(arg0: c_int) -> i32;
  // proto: int QTextBlock::fragmentIndex();
  fn _ZNK10QTextBlock13fragmentIndexEv() -> i32;
  // proto: bool QTextBlock::isVisible();
  fn _ZNK10QTextBlock9isVisibleEv() -> i32;
  // proto: void QTextBlock::setLineCount(int count);
  fn _ZN10QTextBlock12setLineCountEi(arg0: c_int) -> i32;
  // proto: QTextBlock QTextBlock::next();
  fn _ZNK10QTextBlock4nextEv() -> i32;
  // proto: QTextBlockFormat QTextBlock::blockFormat();
  fn _ZNK10QTextBlock11blockFormatEv() -> i32;
  // proto: void QTextBlock::NewQTextBlock();
  fn _ZN10QTextBlockC1Ev(qthis: *mut c_void) -> i32;
  // proto: int QTextBlock::firstLineNumber();
  fn _ZNK10QTextBlock15firstLineNumberEv() -> i32;
  // proto: QTextCharFormat QTextBlock::charFormat();
  fn _ZNK10QTextBlock10charFormatEv() -> i32;
}

// body block begin
// class sizeof(QTextBlock)=16
pub struct QTextBlock {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBlock {
  pub fn document<T: QTextBlock_document>(&mut self, value: T) -> i32 {
    value.document(self);
    return 1;
  }
}

pub trait QTextBlock_document {
  fn document(self, this: &mut QTextBlock) -> i32;
}

// proto: const QTextDocument * QTextBlock::document();
impl<'a> /*trait*/ QTextBlock_document for () {
  fn document(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8documentEv()};
    unsafe {_ZNK10QTextBlock8documentEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn previous<T: QTextBlock_previous>(&mut self, value: T) -> i32 {
    value.previous(self);
    return 1;
  }
}

pub trait QTextBlock_previous {
  fn previous(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextBlock QTextBlock::previous();
impl<'a> /*trait*/ QTextBlock_previous for () {
  fn previous(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8previousEv()};
    unsafe {_ZNK10QTextBlock8previousEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn length<T: QTextBlock_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QTextBlock_length {
  fn length(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::length();
impl<'a> /*trait*/ QTextBlock_length for () {
  fn length(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6lengthEv()};
    unsafe {_ZNK10QTextBlock6lengthEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn userData<T: QTextBlock_userData>(&mut self, value: T) -> i32 {
    value.userData(self);
    return 1;
  }
}

pub trait QTextBlock_userData {
  fn userData(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextBlockUserData * QTextBlock::userData();
impl<'a> /*trait*/ QTextBlock_userData for () {
  fn userData(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8userDataEv()};
    unsafe {_ZNK10QTextBlock8userDataEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextBlockC1ERKS_(qthis, arg0)};
    let rsthis = QTextBlock{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn text<T: QTextBlock_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QTextBlock_text {
  fn text(self, this: &mut QTextBlock) -> i32;
}

// proto: QString QTextBlock::text();
impl<'a> /*trait*/ QTextBlock_text for () {
  fn text(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4textEv()};
    unsafe {_ZNK10QTextBlock4textEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn lineCount<T: QTextBlock_lineCount>(&mut self, value: T) -> i32 {
    value.lineCount(self);
    return 1;
  }
}

pub trait QTextBlock_lineCount {
  fn lineCount(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::lineCount();
impl<'a> /*trait*/ QTextBlock_lineCount for () {
  fn lineCount(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9lineCountEv()};
    unsafe {_ZNK10QTextBlock9lineCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn contains<T: QTextBlock_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QTextBlock_contains {
  fn contains(self, this: &mut QTextBlock) -> i32;
}

// proto: bool QTextBlock::contains(int position);
impl<'a> /*trait*/ QTextBlock_contains for (i32) {
  fn contains(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8containsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTextBlock8containsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn blockNumber<T: QTextBlock_blockNumber>(&mut self, value: T) -> i32 {
    value.blockNumber(self);
    return 1;
  }
}

pub trait QTextBlock_blockNumber {
  fn blockNumber(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::blockNumber();
impl<'a> /*trait*/ QTextBlock_blockNumber for () {
  fn blockNumber(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockNumberEv()};
    unsafe {_ZNK10QTextBlock11blockNumberEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setRevision<T: QTextBlock_setRevision>(&mut self, value: T) -> i32 {
    value.setRevision(self);
    return 1;
  }
}

pub trait QTextBlock_setRevision {
  fn setRevision(self, this: &mut QTextBlock) -> i32;
}

// proto: void QTextBlock::setRevision(int rev);
impl<'a> /*trait*/ QTextBlock_setRevision for (i32) {
  fn setRevision(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setRevisionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTextBlock11setRevisionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setVisible<T: QTextBlock_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QTextBlock_setVisible {
  fn setVisible(self, this: &mut QTextBlock) -> i32;
}

// proto: void QTextBlock::setVisible(bool visible);
impl<'a> /*trait*/ QTextBlock_setVisible for (i8) {
  fn setVisible(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTextBlock10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn clearLayout<T: QTextBlock_clearLayout>(&mut self, value: T) -> i32 {
    value.clearLayout(self);
    return 1;
  }
}

pub trait QTextBlock_clearLayout {
  fn clearLayout(self, this: &mut QTextBlock) -> i32;
}

// proto: void QTextBlock::clearLayout();
impl<'a> /*trait*/ QTextBlock_clearLayout for () {
  fn clearLayout(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11clearLayoutEv()};
    unsafe {_ZN10QTextBlock11clearLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn docHandle<T: QTextBlock_docHandle>(&mut self, value: T) -> i32 {
    value.docHandle(self);
    return 1;
  }
}

pub trait QTextBlock_docHandle {
  fn docHandle(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextDocumentPrivate * QTextBlock::docHandle();
impl<'a> /*trait*/ QTextBlock_docHandle for () {
  fn docHandle(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9docHandleEv()};
    unsafe {_ZNK10QTextBlock9docHandleEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn userState<T: QTextBlock_userState>(&mut self, value: T) -> i32 {
    value.userState(self);
    return 1;
  }
}

pub trait QTextBlock_userState {
  fn userState(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::userState();
impl<'a> /*trait*/ QTextBlock_userState for () {
  fn userState(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9userStateEv()};
    unsafe {_ZNK10QTextBlock9userStateEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn charFormatIndex<T: QTextBlock_charFormatIndex>(&mut self, value: T) -> i32 {
    value.charFormatIndex(self);
    return 1;
  }
}

pub trait QTextBlock_charFormatIndex {
  fn charFormatIndex(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::charFormatIndex();
impl<'a> /*trait*/ QTextBlock_charFormatIndex for () {
  fn charFormatIndex(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15charFormatIndexEv()};
    unsafe {_ZNK10QTextBlock15charFormatIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn revision<T: QTextBlock_revision>(&mut self, value: T) -> i32 {
    value.revision(self);
    return 1;
  }
}

pub trait QTextBlock_revision {
  fn revision(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::revision();
impl<'a> /*trait*/ QTextBlock_revision for () {
  fn revision(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8revisionEv()};
    unsafe {_ZNK10QTextBlock8revisionEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn position<T: QTextBlock_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QTextBlock_position {
  fn position(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::position();
impl<'a> /*trait*/ QTextBlock_position for () {
  fn position(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8positionEv()};
    unsafe {_ZNK10QTextBlock8positionEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn isValid<T: QTextBlock_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextBlock_isValid {
  fn isValid(self, this: &mut QTextBlock) -> i32;
}

// proto: bool QTextBlock::isValid();
impl<'a> /*trait*/ QTextBlock_isValid for () {
  fn isValid(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock7isValidEv()};
    unsafe {_ZNK10QTextBlock7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn textList<T: QTextBlock_textList>(&mut self, value: T) -> i32 {
    value.textList(self);
    return 1;
  }
}

pub trait QTextBlock_textList {
  fn textList(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextList * QTextBlock::textList();
impl<'a> /*trait*/ QTextBlock_textList for () {
  fn textList(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8textListEv()};
    unsafe {_ZNK10QTextBlock8textListEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn layout<T: QTextBlock_layout>(&mut self, value: T) -> i32 {
    value.layout(self);
    return 1;
  }
}

pub trait QTextBlock_layout {
  fn layout(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextLayout * QTextBlock::layout();
impl<'a> /*trait*/ QTextBlock_layout for () {
  fn layout(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6layoutEv()};
    unsafe {_ZNK10QTextBlock6layoutEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setUserData<T: QTextBlock_setUserData>(&mut self, value: T) -> i32 {
    value.setUserData(self);
    return 1;
  }
}

pub trait QTextBlock_setUserData {
  fn setUserData(self, this: &mut QTextBlock) -> i32;
}

// proto: void QTextBlock::setUserData(QTextBlockUserData * data);
impl<'a> /*trait*/ QTextBlock_setUserData for (&'a mut QTextBlockUserData) {
  fn setUserData(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setUserDataEP18QTextBlockUserData()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextBlock11setUserDataEP18QTextBlockUserData(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn blockFormatIndex<T: QTextBlock_blockFormatIndex>(&mut self, value: T) -> i32 {
    value.blockFormatIndex(self);
    return 1;
  }
}

pub trait QTextBlock_blockFormatIndex {
  fn blockFormatIndex(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::blockFormatIndex();
impl<'a> /*trait*/ QTextBlock_blockFormatIndex for () {
  fn blockFormatIndex(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock16blockFormatIndexEv()};
    unsafe {_ZNK10QTextBlock16blockFormatIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setUserState<T: QTextBlock_setUserState>(&mut self, value: T) -> i32 {
    value.setUserState(self);
    return 1;
  }
}

pub trait QTextBlock_setUserState {
  fn setUserState(self, this: &mut QTextBlock) -> i32;
}

// proto: void QTextBlock::setUserState(int state);
impl<'a> /*trait*/ QTextBlock_setUserState for (i32) {
  fn setUserState(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setUserStateEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTextBlock12setUserStateEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn fragmentIndex<T: QTextBlock_fragmentIndex>(&mut self, value: T) -> i32 {
    value.fragmentIndex(self);
    return 1;
  }
}

pub trait QTextBlock_fragmentIndex {
  fn fragmentIndex(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::fragmentIndex();
impl<'a> /*trait*/ QTextBlock_fragmentIndex for () {
  fn fragmentIndex(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock13fragmentIndexEv()};
    unsafe {_ZNK10QTextBlock13fragmentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn isVisible<T: QTextBlock_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QTextBlock_isVisible {
  fn isVisible(self, this: &mut QTextBlock) -> i32;
}

// proto: bool QTextBlock::isVisible();
impl<'a> /*trait*/ QTextBlock_isVisible for () {
  fn isVisible(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9isVisibleEv()};
    unsafe {_ZNK10QTextBlock9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn setLineCount<T: QTextBlock_setLineCount>(&mut self, value: T) -> i32 {
    value.setLineCount(self);
    return 1;
  }
}

pub trait QTextBlock_setLineCount {
  fn setLineCount(self, this: &mut QTextBlock) -> i32;
}

// proto: void QTextBlock::setLineCount(int count);
impl<'a> /*trait*/ QTextBlock_setLineCount for (i32) {
  fn setLineCount(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setLineCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTextBlock12setLineCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn next<T: QTextBlock_next>(&mut self, value: T) -> i32 {
    value.next(self);
    return 1;
  }
}

pub trait QTextBlock_next {
  fn next(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextBlock QTextBlock::next();
impl<'a> /*trait*/ QTextBlock_next for () {
  fn next(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4nextEv()};
    unsafe {_ZNK10QTextBlock4nextEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn blockFormat<T: QTextBlock_blockFormat>(&mut self, value: T) -> i32 {
    value.blockFormat(self);
    return 1;
  }
}

pub trait QTextBlock_blockFormat {
  fn blockFormat(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextBlockFormat QTextBlock::blockFormat();
impl<'a> /*trait*/ QTextBlock_blockFormat for () {
  fn blockFormat(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockFormatEv()};
    unsafe {_ZNK10QTextBlock11blockFormatEv()};
    return 1;
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
    value.firstLineNumber(self);
    return 1;
  }
}

pub trait QTextBlock_firstLineNumber {
  fn firstLineNumber(self, this: &mut QTextBlock) -> i32;
}

// proto: int QTextBlock::firstLineNumber();
impl<'a> /*trait*/ QTextBlock_firstLineNumber for () {
  fn firstLineNumber(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15firstLineNumberEv()};
    unsafe {_ZNK10QTextBlock15firstLineNumberEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn charFormat<T: QTextBlock_charFormat>(&mut self, value: T) -> i32 {
    value.charFormat(self);
    return 1;
  }
}

pub trait QTextBlock_charFormat {
  fn charFormat(self, this: &mut QTextBlock) -> i32;
}

// proto: QTextCharFormat QTextBlock::charFormat();
impl<'a> /*trait*/ QTextBlock_charFormat for () {
  fn charFormat(self, this: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock10charFormatEv()};
    unsafe {_ZNK10QTextBlock10charFormatEv()};
    return 1;
  }
}

