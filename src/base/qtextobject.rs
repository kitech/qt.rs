// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QTextDocumentPrivate * QTextObject::docHandle();
  fn _ZNK11QTextObject9docHandleEv() -> i32;
  // proto: void QTextObject::FreeQTextObject();
  fn _ZN11QTextObjectD0Ev() -> i32;
  // proto: void QTextObject::NewQTextObject(const QTextObject & );
  fn _ZN11QTextObjectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QTextFormat QTextObject::format();
  fn _ZNK11QTextObject6formatEv() -> i32;
  // proto: int QTextObject::formatIndex();
  fn _ZNK11QTextObject11formatIndexEv() -> i32;
  // proto: QTextDocument * QTextObject::document();
  fn _ZNK11QTextObject8documentEv() -> i32;
  // proto: int QTextObject::objectIndex();
  fn _ZNK11QTextObject11objectIndexEv() -> i32;
  // proto: void QTextObject::NewQTextObject(QTextDocument * doc);
  fn _ZN11QTextObjectC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QTextObject::metaObject();
  fn _ZNK11QTextObject10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QTextObject)=1
pub struct QTextObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextObject {
  pub fn docHandle<T: QTextObject_docHandle>(&mut self, value: T) -> i32 {
    value.docHandle(self);
    return 1;
  }
}

pub trait QTextObject_docHandle {
  fn docHandle(self, this: &mut QTextObject) -> i32;
}

// proto: QTextDocumentPrivate * QTextObject::docHandle();
impl<'a> /*trait*/ QTextObject_docHandle for () {
  fn docHandle(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject9docHandleEv()};
    unsafe {_ZNK11QTextObject9docHandleEv()};
    return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn FreeQTextObject<T: QTextObject_FreeQTextObject>(&mut self, value: T) -> i32 {
    value.FreeQTextObject(self);
    return 1;
  }
}

pub trait QTextObject_FreeQTextObject {
  fn FreeQTextObject(self, this: &mut QTextObject) -> i32;
}

// proto: void QTextObject::FreeQTextObject();
impl<'a> /*trait*/ QTextObject_FreeQTextObject for () {
  fn FreeQTextObject(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectD0Ev()};
    unsafe {_ZN11QTextObjectD0Ev()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextObjectC1ERKS_(qthis, arg0)};
    let rsthis = QTextObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn format<T: QTextObject_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QTextObject_format {
  fn format(self, this: &mut QTextObject) -> i32;
}

// proto: QTextFormat QTextObject::format();
impl<'a> /*trait*/ QTextObject_format for () {
  fn format(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject6formatEv()};
    unsafe {_ZNK11QTextObject6formatEv()};
    return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn formatIndex<T: QTextObject_formatIndex>(&mut self, value: T) -> i32 {
    value.formatIndex(self);
    return 1;
  }
}

pub trait QTextObject_formatIndex {
  fn formatIndex(self, this: &mut QTextObject) -> i32;
}

// proto: int QTextObject::formatIndex();
impl<'a> /*trait*/ QTextObject_formatIndex for () {
  fn formatIndex(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject11formatIndexEv()};
    unsafe {_ZNK11QTextObject11formatIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn document<T: QTextObject_document>(&mut self, value: T) -> i32 {
    value.document(self);
    return 1;
  }
}

pub trait QTextObject_document {
  fn document(self, this: &mut QTextObject) -> i32;
}

// proto: QTextDocument * QTextObject::document();
impl<'a> /*trait*/ QTextObject_document for () {
  fn document(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject8documentEv()};
    unsafe {_ZNK11QTextObject8documentEv()};
    return 1;
  }
}

impl /*struct*/ QTextObject {
  pub fn objectIndex<T: QTextObject_objectIndex>(&mut self, value: T) -> i32 {
    value.objectIndex(self);
    return 1;
  }
}

pub trait QTextObject_objectIndex {
  fn objectIndex(self, this: &mut QTextObject) -> i32;
}

// proto: int QTextObject::objectIndex();
impl<'a> /*trait*/ QTextObject_objectIndex for () {
  fn objectIndex(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject11objectIndexEv()};
    unsafe {_ZNK11QTextObject11objectIndexEv()};
    return 1;
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
  pub fn metaObject<T: QTextObject_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextObject_metaObject {
  fn metaObject(self, this: &mut QTextObject) -> i32;
}

// proto: const QMetaObject * QTextObject::metaObject();
impl<'a> /*trait*/ QTextObject_metaObject for () {
  fn metaObject(self, this: &mut QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject10metaObjectEv()};
    unsafe {_ZNK11QTextObject10metaObjectEv()};
    return 1;
  }
}

