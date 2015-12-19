// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextblock::QTextBlock;
use super::qtextlistformat::QTextListFormat;
use super::qtextdocument::QTextDocument;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTextBlock QTextList::item(int i);
  fn _ZNK9QTextList4itemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextList::remove(const QTextBlock & );
  fn _ZN9QTextList6removeERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextList::setFormat(const QTextListFormat & format);
  fn _ZN9QTextList9setFormatERK15QTextListFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextList::NewQTextList(QTextDocument * doc);
  fn _ZN9QTextListC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextList::NewQTextList(const QTextList & );
  fn _ZN9QTextListC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextList::add(const QTextBlock & block);
  fn _ZN9QTextList3addERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextList::itemText(const QTextBlock & );
  fn _ZNK9QTextList8itemTextERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextList::removeItem(int i);
  fn _ZN9QTextList10removeItemEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTextList::itemNumber(const QTextBlock & );
  fn _ZNK9QTextList10itemNumberERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QTextList::count();
  fn _ZNK9QTextList5countEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextListFormat QTextList::format();
  fn _ZNK9QTextList6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextList::FreeQTextList();
  fn _ZN9QTextListD0Ev(qthis: *mut c_void) ;
  // proto:  bool QTextList::isEmpty();
  fn _ZNK9QTextList7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QTextList::metaObject();
  fn _ZNK9QTextList10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextList)=1
pub struct QTextList {
  pub qclsinst: *mut c_void,
}

// proto:  QTextBlock QTextList::item(int i);
impl /*struct*/ QTextList {
  pub fn item<RetType, T: QTextList_item<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QTextList_item<RetType> {
  fn item(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  QTextBlock QTextList::item(int i);
impl<'a> /*trait*/ QTextList_item<QTextBlock> for (i32) {
  fn item(self , rsthis: &mut QTextList) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList4itemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTextList4itemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextList::remove(const QTextBlock & );
impl /*struct*/ QTextList {
  pub fn remove<RetType, T: QTextList_remove<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QTextList_remove<RetType> {
  fn remove(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  void QTextList::remove(const QTextBlock & );
impl<'a> /*trait*/ QTextList_remove<()> for (&'a  QTextBlock) {
  fn remove(self , rsthis: &mut QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList6removeERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextList6removeERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextList::setFormat(const QTextListFormat & format);
impl /*struct*/ QTextList {
  pub fn setFormat<RetType, T: QTextList_setFormat<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextList_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  void QTextList::setFormat(const QTextListFormat & format);
impl<'a> /*trait*/ QTextList_setFormat<()> for (&'a  QTextListFormat) {
  fn setFormat(self , rsthis: &mut QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList9setFormatERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextList9setFormatERK15QTextListFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn NewQTextList<T: QTextList_NewQTextList>(value: T) -> QTextList {
    let rsthis = value.NewQTextList();
    return rsthis;
    // return 1;
  }
}

pub trait QTextList_NewQTextList {
  fn NewQTextList(self) -> QTextList;
}

// proto: void QTextList::NewQTextList(QTextDocument * doc);
impl<'a> /*trait*/ QTextList_NewQTextList for (&'a mut QTextDocument) {
  fn NewQTextList(self) -> QTextList {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextListC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextList{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTextList::NewQTextList(const QTextList & );
impl<'a> /*trait*/ QTextList_NewQTextList for (&'a  QTextList) {
  fn NewQTextList(self) -> QTextList {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextListC1ERKS_(qthis, arg0)};
    let rsthis = QTextList{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTextList::add(const QTextBlock & block);
impl /*struct*/ QTextList {
  pub fn add<RetType, T: QTextList_add<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.add(self);
    // return 1;
  }
}

pub trait QTextList_add<RetType> {
  fn add(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  void QTextList::add(const QTextBlock & block);
impl<'a> /*trait*/ QTextList_add<()> for (&'a  QTextBlock) {
  fn add(self , rsthis: &mut QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList3addERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextList3addERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QTextList::itemText(const QTextBlock & );
impl /*struct*/ QTextList {
  pub fn itemText<RetType, T: QTextList_itemText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemText(self);
    // return 1;
  }
}

pub trait QTextList_itemText<RetType> {
  fn itemText(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  QString QTextList::itemText(const QTextBlock & );
impl<'a> /*trait*/ QTextList_itemText<QString> for (&'a  QTextBlock) {
  fn itemText(self , rsthis: &mut QTextList) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList8itemTextERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextList8itemTextERK10QTextBlock(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextList::removeItem(int i);
impl /*struct*/ QTextList {
  pub fn removeItem<RetType, T: QTextList_removeItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QTextList_removeItem<RetType> {
  fn removeItem(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  void QTextList::removeItem(int i);
impl<'a> /*trait*/ QTextList_removeItem<()> for (i32) {
  fn removeItem(self , rsthis: &mut QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList10removeItemEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextList10removeItemEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QTextList::itemNumber(const QTextBlock & );
impl /*struct*/ QTextList {
  pub fn itemNumber<RetType, T: QTextList_itemNumber<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemNumber(self);
    // return 1;
  }
}

pub trait QTextList_itemNumber<RetType> {
  fn itemNumber(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  int QTextList::itemNumber(const QTextBlock & );
impl<'a> /*trait*/ QTextList_itemNumber<i32> for (&'a  QTextBlock) {
  fn itemNumber(self , rsthis: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList10itemNumberERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextList10itemNumberERK10QTextBlock(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QTextList::count();
impl /*struct*/ QTextList {
  pub fn count<RetType, T: QTextList_count<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QTextList_count<RetType> {
  fn count(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  int QTextList::count();
impl<'a> /*trait*/ QTextList_count<i32> for () {
  fn count(self , rsthis: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList5countEv()};
    let mut ret = unsafe {_ZNK9QTextList5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QTextListFormat QTextList::format();
impl /*struct*/ QTextList {
  pub fn format<RetType, T: QTextList_format<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextList_format<RetType> {
  fn format(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  QTextListFormat QTextList::format();
impl<'a> /*trait*/ QTextList_format<QTextListFormat> for () {
  fn format(self , rsthis: &mut QTextList) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList6formatEv()};
    let mut ret = unsafe {_ZNK9QTextList6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextListFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextList::FreeQTextList();
impl /*struct*/ QTextList {
  pub fn FreeQTextList<RetType, T: QTextList_FreeQTextList<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTextList(self);
    // return 1;
  }
}

pub trait QTextList_FreeQTextList<RetType> {
  fn FreeQTextList(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  void QTextList::FreeQTextList();
impl<'a> /*trait*/ QTextList_FreeQTextList<()> for () {
  fn FreeQTextList(self , rsthis: &mut QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListD0Ev()};
     unsafe {_ZN9QTextListD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QTextList::isEmpty();
impl /*struct*/ QTextList {
  pub fn isEmpty<RetType, T: QTextList_isEmpty<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextList_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  bool QTextList::isEmpty();
impl<'a> /*trait*/ QTextList_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QTextList) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QTextList7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  const QMetaObject * QTextList::metaObject();
impl /*struct*/ QTextList {
  pub fn metaObject<RetType, T: QTextList_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextList_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTextList) -> RetType;
}

// proto:  const QMetaObject * QTextList::metaObject();
impl<'a> /*trait*/ QTextList_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList10metaObjectEv()};
     unsafe {_ZNK9QTextList10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

