// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextformat::QTextFormat;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTextDocumentPrivate * QTextObject::docHandle();
  fn _ZNK11QTextObject9docHandleEv(qthis: *mut c_void) ;
  // proto:  void QTextObject::FreeQTextObject();
  fn _ZN11QTextObjectD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextObject::NewQTextObject(const QTextObject & );
  fn _ZN11QTextObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextFormat QTextObject::format();
  fn _ZNK11QTextObject6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextObject::formatIndex();
  fn _ZNK11QTextObject11formatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextDocument * QTextObject::document();
  fn _ZNK11QTextObject8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextObject::objectIndex();
  fn _ZNK11QTextObject11objectIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextObject::NewQTextObject(QTextDocument * doc);
  fn _ZN11QTextObjectC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QTextObject::metaObject();
  fn _ZNK11QTextObject10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextObject)=1
pub struct QTextObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextObject {
  pub fn docHandle<RetType, T: QTextObject_docHandle<RetType>>(&mut self, value: T) -> RetType {
    return value.docHandle(self);
    // return 1;
  }
}

pub trait QTextObject_docHandle<RetType> {
  fn docHandle(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  QTextDocumentPrivate * QTextObject::docHandle();
impl<'a> /*trait*/ QTextObject_docHandle<()> for () {
  fn docHandle(self, rsthis: &mut QTextObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject9docHandleEv()};
     unsafe {_ZNK11QTextObject9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn FreeQTextObject<RetType, T: QTextObject_FreeQTextObject<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTextObject(self);
    // return 1;
  }
}

pub trait QTextObject_FreeQTextObject<RetType> {
  fn FreeQTextObject(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  void QTextObject::FreeQTextObject();
impl<'a> /*trait*/ QTextObject_FreeQTextObject<()> for () {
  fn FreeQTextObject(self, rsthis: &mut QTextObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectD0Ev()};
     unsafe {_ZN11QTextObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn NewQTextObject<T: QTextObject_NewQTextObject>(value: T) -> QTextObject {
    let rsthis = value.NewQTextObject();
    return rsthis;
    // return 1;
  }
}

pub trait QTextObject_NewQTextObject {
  fn NewQTextObject(self) -> QTextObject;
}

// proto: void QTextObject::NewQTextObject(const QTextObject & );
impl<'a> /*trait*/ QTextObject_NewQTextObject for (&'a  QTextObject) {
  fn NewQTextObject(self) -> QTextObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextObjectC1ERKS_(qthis, arg0)};
    let rsthis = QTextObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn format<RetType, T: QTextObject_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QTextObject_format<RetType> {
  fn format(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  QTextFormat QTextObject::format();
impl<'a> /*trait*/ QTextObject_format<QTextFormat> for () {
  fn format(self, rsthis: &mut QTextObject) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject6formatEv()};
    let mut ret = unsafe {_ZNK11QTextObject6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn formatIndex<RetType, T: QTextObject_formatIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.formatIndex(self);
    // return 1;
  }
}

pub trait QTextObject_formatIndex<RetType> {
  fn formatIndex(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  int QTextObject::formatIndex();
impl<'a> /*trait*/ QTextObject_formatIndex<i32> for () {
  fn formatIndex(self, rsthis: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject11formatIndexEv()};
    let mut ret = unsafe {_ZNK11QTextObject11formatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn document<RetType, T: QTextObject_document<RetType>>(&mut self, value: T) -> RetType {
    return value.document(self);
    // return 1;
  }
}

pub trait QTextObject_document<RetType> {
  fn document(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  QTextDocument * QTextObject::document();
impl<'a> /*trait*/ QTextObject_document<QTextDocument> for () {
  fn document(self, rsthis: &mut QTextObject) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject8documentEv()};
    let mut ret = unsafe {_ZNK11QTextObject8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn objectIndex<RetType, T: QTextObject_objectIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.objectIndex(self);
    // return 1;
  }
}

pub trait QTextObject_objectIndex<RetType> {
  fn objectIndex(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  int QTextObject::objectIndex();
impl<'a> /*trait*/ QTextObject_objectIndex<i32> for () {
  fn objectIndex(self, rsthis: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject11objectIndexEv()};
    let mut ret = unsafe {_ZNK11QTextObject11objectIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QTextObject::NewQTextObject(QTextDocument * doc);
impl<'a> /*trait*/ QTextObject_NewQTextObject for (&'a mut QTextDocument) {
  fn NewQTextObject(self) -> QTextObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextObjectC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn metaObject<RetType, T: QTextObject_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QTextObject_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QTextObject) -> RetType;
}

// proto:  const QMetaObject * QTextObject::metaObject();
impl<'a> /*trait*/ QTextObject_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QTextObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject10metaObjectEv()};
     unsafe {_ZNK11QTextObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

