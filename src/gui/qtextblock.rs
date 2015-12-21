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
  // proto:  void QTextBlock::QTextBlock(const QTextBlock & o);
  fn _ZN10QTextBlockC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTextBlock::text();
  fn _ZNK10QTextBlock4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextBlock::lineCount();
  fn _ZNK10QTextBlock9lineCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBlock::contains(int position);
  fn _ZNK10QTextBlock8containsEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  int QTextBlock::blockNumber();
  fn _ZNK10QTextBlock11blockNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlock::setRevision(int rev);
  fn _ZN10QTextBlock11setRevisionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextBlock::setVisible(bool visible);
  fn _ZN10QTextBlock10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextBlock::clearLayout();
  fn _ZN10QTextBlock11clearLayoutEv(qthis: *mut c_void);
  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
  fn _ZNK10QTextBlock9docHandleEv(qthis: *mut c_void);
  // proto:  int QTextBlock::userState();
  fn _ZNK10QTextBlock9userStateEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextBlock::charFormatIndex();
  fn _ZNK10QTextBlock15charFormatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextBlock::revision();
  fn _ZNK10QTextBlock8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextBlock::position();
  fn _ZNK10QTextBlock8positionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBlock::isValid();
  fn _ZNK10QTextBlock7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QTextList * QTextBlock::textList();
  fn _ZNK10QTextBlock8textListEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextLayout * QTextBlock::layout();
  fn _ZNK10QTextBlock6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
  fn _ZN10QTextBlock11setUserDataEP18QTextBlockUserData(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextBlock::blockFormatIndex();
  fn _ZNK10QTextBlock16blockFormatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlock::setUserState(int state);
  fn _ZN10QTextBlock12setUserStateEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTextBlock::fragmentIndex();
  fn _ZNK10QTextBlock13fragmentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBlock::isVisible();
  fn _ZNK10QTextBlock9isVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextBlock::setLineCount(int count);
  fn _ZN10QTextBlock12setLineCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextBlock QTextBlock::next();
  fn _ZNK10QTextBlock4nextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextBlockFormat QTextBlock::blockFormat();
  fn _ZNK10QTextBlock11blockFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBlock::QTextBlock();
  fn _ZN10QTextBlockC1Ev(qthis: *mut c_void);
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

  // proto:  const QTextDocument * QTextBlock::document();
impl /*struct*/ QTextBlock {
  pub fn document<RetType, T: QTextBlock_document<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QTextBlock_document<RetType> {
  fn document(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  const QTextDocument * QTextBlock::document();
impl<'a> /*trait*/ QTextBlock_document<QTextDocument> for () {
  fn document(self , rsthis: &mut QTextBlock) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8documentEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextBlock::previous();
impl /*struct*/ QTextBlock {
  pub fn previous<RetType, T: QTextBlock_previous<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.previous(self);
    // return 1;
  }
}

pub trait QTextBlock_previous<RetType> {
  fn previous(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextBlock QTextBlock::previous();
impl<'a> /*trait*/ QTextBlock_previous<QTextBlock> for () {
  fn previous(self , rsthis: &mut QTextBlock) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8previousEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8previousEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextBlock::length();
impl /*struct*/ QTextBlock {
  pub fn length<RetType, T: QTextBlock_length<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QTextBlock_length<RetType> {
  fn length(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::length();
impl<'a> /*trait*/ QTextBlock_length<i32> for () {
  fn length(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6lengthEv()};
    let mut ret = unsafe {_ZNK10QTextBlock6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextBlockUserData * QTextBlock::userData();
impl /*struct*/ QTextBlock {
  pub fn userData<RetType, T: QTextBlock_userData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.userData(self);
    // return 1;
  }
}

pub trait QTextBlock_userData<RetType> {
  fn userData(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextBlockUserData * QTextBlock::userData();
impl<'a> /*trait*/ QTextBlock_userData<QTextBlockUserData> for () {
  fn userData(self , rsthis: &mut QTextBlock) -> QTextBlockUserData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8userDataEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8userDataEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockUserData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBlock::QTextBlock(const QTextBlock & o);
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

  // proto:  void QTextBlock::QTextBlock(const QTextBlock & o);
impl<'a> /*trait*/ QTextBlock_NewQTextBlock for (QTextBlock) {
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

  // proto:  QString QTextBlock::text();
impl /*struct*/ QTextBlock {
  pub fn text<RetType, T: QTextBlock_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextBlock_text<RetType> {
  fn text(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QString QTextBlock::text();
impl<'a> /*trait*/ QTextBlock_text<QString> for () {
  fn text(self , rsthis: &mut QTextBlock) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4textEv()};
    let mut ret = unsafe {_ZNK10QTextBlock4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextBlock::lineCount();
impl /*struct*/ QTextBlock {
  pub fn lineCount<RetType, T: QTextBlock_lineCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineCount(self);
    // return 1;
  }
}

pub trait QTextBlock_lineCount<RetType> {
  fn lineCount(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::lineCount();
impl<'a> /*trait*/ QTextBlock_lineCount<i32> for () {
  fn lineCount(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9lineCountEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBlock::contains(int position);
impl /*struct*/ QTextBlock {
  pub fn contains<RetType, T: QTextBlock_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QTextBlock_contains<RetType> {
  fn contains(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  bool QTextBlock::contains(int position);
impl<'a> /*trait*/ QTextBlock_contains<i8> for (i32) {
  fn contains(self , rsthis: &mut QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8containsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTextBlock8containsEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextBlock::blockNumber();
impl /*struct*/ QTextBlock {
  pub fn blockNumber<RetType, T: QTextBlock_blockNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockNumber(self);
    // return 1;
  }
}

pub trait QTextBlock_blockNumber<RetType> {
  fn blockNumber(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::blockNumber();
impl<'a> /*trait*/ QTextBlock_blockNumber<i32> for () {
  fn blockNumber(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockNumberEv()};
    let mut ret = unsafe {_ZNK10QTextBlock11blockNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBlock::setRevision(int rev);
impl /*struct*/ QTextBlock {
  pub fn setRevision<RetType, T: QTextBlock_setRevision<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRevision(self);
    // return 1;
  }
}

pub trait QTextBlock_setRevision<RetType> {
  fn setRevision(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setRevision(int rev);
impl<'a> /*trait*/ QTextBlock_setRevision<()> for (i32) {
  fn setRevision(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setRevisionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock11setRevisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlock::setVisible(bool visible);
impl /*struct*/ QTextBlock {
  pub fn setVisible<RetType, T: QTextBlock_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QTextBlock_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setVisible(bool visible);
impl<'a> /*trait*/ QTextBlock_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTextBlock10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlock::clearLayout();
impl /*struct*/ QTextBlock {
  pub fn clearLayout<RetType, T: QTextBlock_clearLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearLayout(self);
    // return 1;
  }
}

pub trait QTextBlock_clearLayout<RetType> {
  fn clearLayout(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::clearLayout();
impl<'a> /*trait*/ QTextBlock_clearLayout<()> for () {
  fn clearLayout(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11clearLayoutEv()};
     unsafe {_ZN10QTextBlock11clearLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
impl /*struct*/ QTextBlock {
  pub fn docHandle<RetType, T: QTextBlock_docHandle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.docHandle(self);
    // return 1;
  }
}

pub trait QTextBlock_docHandle<RetType> {
  fn docHandle(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
impl<'a> /*trait*/ QTextBlock_docHandle<()> for () {
  fn docHandle(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9docHandleEv()};
     unsafe {_ZNK10QTextBlock9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextBlock::userState();
impl /*struct*/ QTextBlock {
  pub fn userState<RetType, T: QTextBlock_userState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.userState(self);
    // return 1;
  }
}

pub trait QTextBlock_userState<RetType> {
  fn userState(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::userState();
impl<'a> /*trait*/ QTextBlock_userState<i32> for () {
  fn userState(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9userStateEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9userStateEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextBlock::charFormatIndex();
impl /*struct*/ QTextBlock {
  pub fn charFormatIndex<RetType, T: QTextBlock_charFormatIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.charFormatIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_charFormatIndex<RetType> {
  fn charFormatIndex(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::charFormatIndex();
impl<'a> /*trait*/ QTextBlock_charFormatIndex<i32> for () {
  fn charFormatIndex(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15charFormatIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock15charFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextBlock::revision();
impl /*struct*/ QTextBlock {
  pub fn revision<RetType, T: QTextBlock_revision<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QTextBlock_revision<RetType> {
  fn revision(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::revision();
impl<'a> /*trait*/ QTextBlock_revision<i32> for () {
  fn revision(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8revisionEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextBlock::position();
impl /*struct*/ QTextBlock {
  pub fn position<RetType, T: QTextBlock_position<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextBlock_position<RetType> {
  fn position(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::position();
impl<'a> /*trait*/ QTextBlock_position<i32> for () {
  fn position(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8positionEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBlock::isValid();
impl /*struct*/ QTextBlock {
  pub fn isValid<RetType, T: QTextBlock_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextBlock_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  bool QTextBlock::isValid();
impl<'a> /*trait*/ QTextBlock_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock7isValidEv()};
    let mut ret = unsafe {_ZNK10QTextBlock7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextList * QTextBlock::textList();
impl /*struct*/ QTextBlock {
  pub fn textList<RetType, T: QTextBlock_textList<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textList(self);
    // return 1;
  }
}

pub trait QTextBlock_textList<RetType> {
  fn textList(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextList * QTextBlock::textList();
impl<'a> /*trait*/ QTextBlock_textList<QTextList> for () {
  fn textList(self , rsthis: &mut QTextBlock) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8textListEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8textListEv(rsthis.qclsinst)};
    let mut ret1 = QTextList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextLayout * QTextBlock::layout();
impl /*struct*/ QTextBlock {
  pub fn layout<RetType, T: QTextBlock_layout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QTextBlock_layout<RetType> {
  fn layout(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextLayout * QTextBlock::layout();
impl<'a> /*trait*/ QTextBlock_layout<QTextLayout> for () {
  fn layout(self , rsthis: &mut QTextBlock) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6layoutEv()};
    let mut ret = unsafe {_ZNK10QTextBlock6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QTextLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
impl /*struct*/ QTextBlock {
  pub fn setUserData<RetType, T: QTextBlock_setUserData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUserData(self);
    // return 1;
  }
}

pub trait QTextBlock_setUserData<RetType> {
  fn setUserData(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
impl<'a> /*trait*/ QTextBlock_setUserData<()> for (QTextBlockUserData) {
  fn setUserData(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setUserDataEP18QTextBlockUserData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextBlock11setUserDataEP18QTextBlockUserData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextBlock::blockFormatIndex();
impl /*struct*/ QTextBlock {
  pub fn blockFormatIndex<RetType, T: QTextBlock_blockFormatIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockFormatIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_blockFormatIndex<RetType> {
  fn blockFormatIndex(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::blockFormatIndex();
impl<'a> /*trait*/ QTextBlock_blockFormatIndex<i32> for () {
  fn blockFormatIndex(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock16blockFormatIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock16blockFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBlock::setUserState(int state);
impl /*struct*/ QTextBlock {
  pub fn setUserState<RetType, T: QTextBlock_setUserState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUserState(self);
    // return 1;
  }
}

pub trait QTextBlock_setUserState<RetType> {
  fn setUserState(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setUserState(int state);
impl<'a> /*trait*/ QTextBlock_setUserState<()> for (i32) {
  fn setUserState(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setUserStateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock12setUserStateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextBlock::fragmentIndex();
impl /*struct*/ QTextBlock {
  pub fn fragmentIndex<RetType, T: QTextBlock_fragmentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fragmentIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_fragmentIndex<RetType> {
  fn fragmentIndex(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::fragmentIndex();
impl<'a> /*trait*/ QTextBlock_fragmentIndex<i32> for () {
  fn fragmentIndex(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock13fragmentIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock13fragmentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBlock::isVisible();
impl /*struct*/ QTextBlock {
  pub fn isVisible<RetType, T: QTextBlock_isVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QTextBlock_isVisible<RetType> {
  fn isVisible(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  bool QTextBlock::isVisible();
impl<'a> /*trait*/ QTextBlock_isVisible<i8> for () {
  fn isVisible(self , rsthis: &mut QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9isVisibleEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBlock::setLineCount(int count);
impl /*struct*/ QTextBlock {
  pub fn setLineCount<RetType, T: QTextBlock_setLineCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLineCount(self);
    // return 1;
  }
}

pub trait QTextBlock_setLineCount<RetType> {
  fn setLineCount(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setLineCount(int count);
impl<'a> /*trait*/ QTextBlock_setLineCount<()> for (i32) {
  fn setLineCount(self , rsthis: &mut QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setLineCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock12setLineCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextBlock QTextBlock::next();
impl /*struct*/ QTextBlock {
  pub fn next<RetType, T: QTextBlock_next<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QTextBlock_next<RetType> {
  fn next(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextBlock QTextBlock::next();
impl<'a> /*trait*/ QTextBlock_next<QTextBlock> for () {
  fn next(self , rsthis: &mut QTextBlock) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4nextEv()};
    let mut ret = unsafe {_ZNK10QTextBlock4nextEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlockFormat QTextBlock::blockFormat();
impl /*struct*/ QTextBlock {
  pub fn blockFormat<RetType, T: QTextBlock_blockFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockFormat(self);
    // return 1;
  }
}

pub trait QTextBlock_blockFormat<RetType> {
  fn blockFormat(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextBlockFormat QTextBlock::blockFormat();
impl<'a> /*trait*/ QTextBlock_blockFormat<QTextBlockFormat> for () {
  fn blockFormat(self , rsthis: &mut QTextBlock) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockFormatEv()};
    let mut ret = unsafe {_ZNK10QTextBlock11blockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBlock::QTextBlock();
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

  // proto:  int QTextBlock::firstLineNumber();
impl /*struct*/ QTextBlock {
  pub fn firstLineNumber<RetType, T: QTextBlock_firstLineNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.firstLineNumber(self);
    // return 1;
  }
}

pub trait QTextBlock_firstLineNumber<RetType> {
  fn firstLineNumber(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::firstLineNumber();
impl<'a> /*trait*/ QTextBlock_firstLineNumber<i32> for () {
  fn firstLineNumber(self , rsthis: &mut QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15firstLineNumberEv()};
    let mut ret = unsafe {_ZNK10QTextBlock15firstLineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextBlock::charFormat();
impl /*struct*/ QTextBlock {
  pub fn charFormat<RetType, T: QTextBlock_charFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.charFormat(self);
    // return 1;
  }
}

pub trait QTextBlock_charFormat<RetType> {
  fn charFormat(self , rsthis: &mut QTextBlock) -> RetType;
}

  // proto:  QTextCharFormat QTextBlock::charFormat();
impl<'a> /*trait*/ QTextBlock_charFormat<QTextCharFormat> for () {
  fn charFormat(self , rsthis: &mut QTextBlock) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock10charFormatEv()};
    let mut ret = unsafe {_ZNK10QTextBlock10charFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

