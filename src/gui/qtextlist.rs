// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtGui/qtextlist.h
// dst-file: /src/gui/qtextlist.rs
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
use super::qtextobject::QTextBlockGroup; // 773
use std::ops::Deref;
use super::qtextobject::QTextBlock; // 773
use super::qtextformat::QTextListFormat; // 773
use super::qtextdocument::QTextDocument; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextList_Class_Size() -> c_int;
  // proto:  QTextBlock QTextList::item(int i);
  fn _ZNK9QTextList4itemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTextList::remove(const QTextBlock & );
  fn _ZN9QTextList6removeERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextList::setFormat(const QTextListFormat & format);
  fn _ZN9QTextList9setFormatERK15QTextListFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextList::QTextList(QTextDocument * doc);
  fn _ZN9QTextListC2EP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextList::QTextList(const QTextList & );
  fn _ZN9QTextListC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextList::add(const QTextBlock & block);
  fn _ZN9QTextList3addERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextList::itemText(const QTextBlock & );
  fn _ZNK9QTextList8itemTextERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextList::removeItem(int i);
  fn _ZN9QTextList10removeItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTextList::itemNumber(const QTextBlock & );
  fn _ZNK9QTextList10itemNumberERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QTextList::count();
  fn _ZNK9QTextList5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextListFormat QTextList::format();
  fn _ZNK9QTextList6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextList::~QTextList();
  fn _ZN9QTextListD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextList::isEmpty();
  fn _ZNK9QTextList7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QTextList::metaObject();
  fn _ZNK9QTextList10metaObjectEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QTextList)=1
#[derive(Default)]
pub struct QTextList {
  qbase: QTextBlockGroup,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextList {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextList {
    return QTextList{qbase: QTextBlockGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextList {
  type Target = QTextBlockGroup;

  fn deref(&self) -> &QTextBlockGroup {
    return & self.qbase;
  }
}
impl AsRef<QTextBlockGroup> for QTextList {
  fn as_ref(& self) -> & QTextBlockGroup {
    return & self.qbase;
  }
}
  // proto:  QTextBlock QTextList::item(int i);
impl /*struct*/ QTextList {
  pub fn item<RetType, T: QTextList_item<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QTextList_item<RetType> {
  fn item(self , rsthis: & QTextList) -> RetType;
}

  // proto:  QTextBlock QTextList::item(int i);
impl<'a> /*trait*/ QTextList_item<QTextBlock> for (i32) {
  fn item(self , rsthis: & QTextList) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList4itemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTextList4itemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextList::remove(const QTextBlock & );
impl /*struct*/ QTextList {
  pub fn remove<RetType, T: QTextList_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QTextList_remove<RetType> {
  fn remove(self , rsthis: & QTextList) -> RetType;
}

  // proto:  void QTextList::remove(const QTextBlock & );
impl<'a> /*trait*/ QTextList_remove<()> for (&'a QTextBlock) {
  fn remove(self , rsthis: & QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList6removeERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextList6removeERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextList::setFormat(const QTextListFormat & format);
impl /*struct*/ QTextList {
  pub fn setFormat<RetType, T: QTextList_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextList_setFormat<RetType> {
  fn setFormat(self , rsthis: & QTextList) -> RetType;
}

  // proto:  void QTextList::setFormat(const QTextListFormat & format);
impl<'a> /*trait*/ QTextList_setFormat<()> for (&'a QTextListFormat) {
  fn setFormat(self , rsthis: & QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList9setFormatERK15QTextListFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextList9setFormatERK15QTextListFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextList::QTextList(QTextDocument * doc);
impl /*struct*/ QTextList {
  pub fn new<T: QTextList_new>(value: T) -> QTextList {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextList_new {
  fn new(self) -> QTextList;
}

  // proto:  void QTextList::QTextList(QTextDocument * doc);
impl<'a> /*trait*/ QTextList_new for (&'a QTextDocument) {
  fn new(self) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListC2EP13QTextDocument()};
    let ctysz: c_int = unsafe{QTextList_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextListC2EP13QTextDocument(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTextList{qbase: QTextBlockGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextList::QTextList(const QTextList & );
impl<'a> /*trait*/ QTextList_new for (&'a QTextList) {
  fn new(self) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListC2ERKS_()};
    let ctysz: c_int = unsafe{QTextList_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTextListC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTextList{qbase: QTextBlockGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextList::add(const QTextBlock & block);
impl /*struct*/ QTextList {
  pub fn add<RetType, T: QTextList_add<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.add(self);
    // return 1;
  }
}

pub trait QTextList_add<RetType> {
  fn add(self , rsthis: & QTextList) -> RetType;
}

  // proto:  void QTextList::add(const QTextBlock & block);
impl<'a> /*trait*/ QTextList_add<()> for (&'a QTextBlock) {
  fn add(self , rsthis: & QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList3addERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextList3addERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextList::itemText(const QTextBlock & );
impl /*struct*/ QTextList {
  pub fn itemText<RetType, T: QTextList_itemText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemText(self);
    // return 1;
  }
}

pub trait QTextList_itemText<RetType> {
  fn itemText(self , rsthis: & QTextList) -> RetType;
}

  // proto:  QString QTextList::itemText(const QTextBlock & );
impl<'a> /*trait*/ QTextList_itemText<QString> for (&'a QTextBlock) {
  fn itemText(self , rsthis: & QTextList) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList8itemTextERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTextList8itemTextERK10QTextBlock(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextList::removeItem(int i);
impl /*struct*/ QTextList {
  pub fn removeItem<RetType, T: QTextList_removeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QTextList_removeItem<RetType> {
  fn removeItem(self , rsthis: & QTextList) -> RetType;
}

  // proto:  void QTextList::removeItem(int i);
impl<'a> /*trait*/ QTextList_removeItem<()> for (i32) {
  fn removeItem(self , rsthis: & QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextList10removeItemEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextList10removeItemEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextList::itemNumber(const QTextBlock & );
impl /*struct*/ QTextList {
  pub fn itemNumber<RetType, T: QTextList_itemNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemNumber(self);
    // return 1;
  }
}

pub trait QTextList_itemNumber<RetType> {
  fn itemNumber(self , rsthis: & QTextList) -> RetType;
}

  // proto:  int QTextList::itemNumber(const QTextBlock & );
impl<'a> /*trait*/ QTextList_itemNumber<i32> for (&'a QTextBlock) {
  fn itemNumber(self , rsthis: & QTextList) -> i32 {
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
  pub fn count<RetType, T: QTextList_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QTextList_count<RetType> {
  fn count(self , rsthis: & QTextList) -> RetType;
}

  // proto:  int QTextList::count();
impl<'a> /*trait*/ QTextList_count<i32> for () {
  fn count(self , rsthis: & QTextList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList5countEv()};
    let mut ret = unsafe {_ZNK9QTextList5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextListFormat QTextList::format();
impl /*struct*/ QTextList {
  pub fn format<RetType, T: QTextList_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextList_format<RetType> {
  fn format(self , rsthis: & QTextList) -> RetType;
}

  // proto:  QTextListFormat QTextList::format();
impl<'a> /*trait*/ QTextList_format<QTextListFormat> for () {
  fn format(self , rsthis: & QTextList) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList6formatEv()};
    let mut ret = unsafe {_ZNK9QTextList6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextListFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextList::~QTextList();
impl /*struct*/ QTextList {
  pub fn free<RetType, T: QTextList_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextList_free<RetType> {
  fn free(self , rsthis: & QTextList) -> RetType;
}

  // proto:  void QTextList::~QTextList();
impl<'a> /*trait*/ QTextList_free<()> for () {
  fn free(self , rsthis: & QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextListD2Ev()};
     unsafe {_ZN9QTextListD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextList::isEmpty();
impl /*struct*/ QTextList {
  pub fn isEmpty<RetType, T: QTextList_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextList_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QTextList) -> RetType;
}

  // proto:  bool QTextList::isEmpty();
impl<'a> /*trait*/ QTextList_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QTextList) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QTextList7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextList::metaObject();
impl /*struct*/ QTextList {
  pub fn metaObject<RetType, T: QTextList_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextList_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextList) -> RetType;
}

  // proto:  const QMetaObject * QTextList::metaObject();
impl<'a> /*trait*/ QTextList_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTextList) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextList10metaObjectEv()};
     unsafe {_ZNK9QTextList10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

