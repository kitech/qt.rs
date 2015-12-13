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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QTextBlock QTextList::item(int i);
  fn _ZNK9QTextList4itemEi(arg0: c_int) -> i32;
  // proto: void QTextList::remove(const QTextBlock & );
  fn _ZN9QTextList6removeERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: void QTextList::setFormat(const QTextListFormat & format);
  fn _ZN9QTextList9setFormatERK15QTextListFormat(arg0: *const c_void) -> i32;
  // proto: void QTextList::NewQTextList(QTextDocument * doc);
  fn _ZN9QTextListC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QTextList::NewQTextList(const QTextList & );
  fn _ZN9QTextListC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextList::add(const QTextBlock & block);
  fn _ZN9QTextList3addERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: QString QTextList::itemText(const QTextBlock & );
  fn _ZNK9QTextList8itemTextERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: void QTextList::removeItem(int i);
  fn _ZN9QTextList10removeItemEi(arg0: c_int) -> i32;
  // proto: int QTextList::itemNumber(const QTextBlock & );
  fn _ZNK9QTextList10itemNumberERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: int QTextList::count();
  fn _ZNK9QTextList5countEv() -> i32;
  // proto: QTextListFormat QTextList::format();
  fn _ZNK9QTextList6formatEv() -> i32;
  // proto: void QTextList::FreeQTextList();
  fn _ZN9QTextListD0Ev() -> i32;
  // proto: bool QTextList::isEmpty();
  fn _ZNK9QTextList7isEmptyEv() -> i32;
  // proto: const QMetaObject * QTextList::metaObject();
  fn _ZNK9QTextList10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QTextList)=1
pub struct QTextList {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextList {
  pub fn item<T: QTextList_item>(&mut self, value: T) -> i32 {
    value.item(self);
    return 1;
  }
}

pub trait QTextList_item {
  fn item(self, this: &mut QTextList) -> i32;
}

// proto: QTextBlock QTextList::item(int i);
impl<'a> /*trait*/ QTextList_item for (i32) {
  fn item(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList4itemEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTextList4itemEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn remove<T: QTextList_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QTextList_remove {
  fn remove(self, this: &mut QTextList) -> i32;
}

// proto: void QTextList::remove(const QTextBlock & );
impl<'a> /*trait*/ QTextList_remove for (&'a  QTextBlock) {
  fn remove(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList6removeERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextList6removeERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn setFormat<T: QTextList_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QTextList_setFormat {
  fn setFormat(self, this: &mut QTextList) -> i32;
}

// proto: void QTextList::setFormat(const QTextListFormat & format);
impl<'a> /*trait*/ QTextList_setFormat for (&'a  QTextListFormat) {
  fn setFormat(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList9setFormatERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextList9setFormatERK15QTextListFormat(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextListC1ERKS_(qthis, arg0)};
    let rsthis = QTextList{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn add<T: QTextList_add>(&mut self, value: T) -> i32 {
    value.add(self);
    return 1;
  }
}

pub trait QTextList_add {
  fn add(self, this: &mut QTextList) -> i32;
}

// proto: void QTextList::add(const QTextBlock & block);
impl<'a> /*trait*/ QTextList_add for (&'a  QTextBlock) {
  fn add(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList3addERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextList3addERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn itemText<T: QTextList_itemText>(&mut self, value: T) -> i32 {
    value.itemText(self);
    return 1;
  }
}

pub trait QTextList_itemText {
  fn itemText(self, this: &mut QTextList) -> i32;
}

// proto: QString QTextList::itemText(const QTextBlock & );
impl<'a> /*trait*/ QTextList_itemText for (&'a  QTextBlock) {
  fn itemText(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList8itemTextERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTextList8itemTextERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn removeItem<T: QTextList_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QTextList_removeItem {
  fn removeItem(self, this: &mut QTextList) -> i32;
}

// proto: void QTextList::removeItem(int i);
impl<'a> /*trait*/ QTextList_removeItem for (i32) {
  fn removeItem(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList10removeItemEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextList10removeItemEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn itemNumber<T: QTextList_itemNumber>(&mut self, value: T) -> i32 {
    value.itemNumber(self);
    return 1;
  }
}

pub trait QTextList_itemNumber {
  fn itemNumber(self, this: &mut QTextList) -> i32;
}

// proto: int QTextList::itemNumber(const QTextBlock & );
impl<'a> /*trait*/ QTextList_itemNumber for (&'a  QTextBlock) {
  fn itemNumber(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList10itemNumberERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTextList10itemNumberERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn count<T: QTextList_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QTextList_count {
  fn count(self, this: &mut QTextList) -> i32;
}

// proto: int QTextList::count();
impl<'a> /*trait*/ QTextList_count for () {
  fn count(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList5countEv()};
    unsafe {_ZNK9QTextList5countEv()};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn format<T: QTextList_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QTextList_format {
  fn format(self, this: &mut QTextList) -> i32;
}

// proto: QTextListFormat QTextList::format();
impl<'a> /*trait*/ QTextList_format for () {
  fn format(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList6formatEv()};
    unsafe {_ZNK9QTextList6formatEv()};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn FreeQTextList<T: QTextList_FreeQTextList>(&mut self, value: T) -> i32 {
    value.FreeQTextList(self);
    return 1;
  }
}

pub trait QTextList_FreeQTextList {
  fn FreeQTextList(self, this: &mut QTextList) -> i32;
}

// proto: void QTextList::FreeQTextList();
impl<'a> /*trait*/ QTextList_FreeQTextList for () {
  fn FreeQTextList(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListD0Ev()};
    unsafe {_ZN9QTextListD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn isEmpty<T: QTextList_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QTextList_isEmpty {
  fn isEmpty(self, this: &mut QTextList) -> i32;
}

// proto: bool QTextList::isEmpty();
impl<'a> /*trait*/ QTextList_isEmpty for () {
  fn isEmpty(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList7isEmptyEv()};
    unsafe {_ZNK9QTextList7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QTextList {
  pub fn metaObject<T: QTextList_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextList_metaObject {
  fn metaObject(self, this: &mut QTextList) -> i32;
}

// proto: const QMetaObject * QTextList::metaObject();
impl<'a> /*trait*/ QTextList_metaObject for () {
  fn metaObject(self, this: &mut QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList10metaObjectEv()};
    unsafe {_ZNK9QTextList10metaObjectEv()};
    return 1;
  }
}

