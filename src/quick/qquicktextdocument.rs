// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquicktextdocument.h
// dst-file: /src/quick/qquicktextdocument.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::gui::qtextdocument::QTextDocument; // 771
use super::qquickitem::QQuickItem; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickTextDocument_Class_Size() -> c_int;
  // proto:  const QMetaObject * QQuickTextDocument::metaObject();
  fn _ZNK18QQuickTextDocument10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTextDocument * QQuickTextDocument::textDocument();
  fn _ZNK18QQuickTextDocument12textDocumentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickTextDocument::QQuickTextDocument(const QQuickTextDocument & );
  fn _ZN18QQuickTextDocumentC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickTextDocument::QQuickTextDocument(QQuickItem * parent);
  fn _ZN18QQuickTextDocumentC2EP10QQuickItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickTextDocument)=1
#[derive(Default)]
pub struct QQuickTextDocument {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQuickTextDocument {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickTextDocument {
    return QQuickTextDocument{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickTextDocument {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQuickTextDocument {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QQuickTextDocument::metaObject();
impl /*struct*/ QQuickTextDocument {
  pub fn metaObject<RetType, T: QQuickTextDocument_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickTextDocument_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickTextDocument) -> RetType;
}

  // proto:  const QMetaObject * QQuickTextDocument::metaObject();
impl<'a> /*trait*/ QQuickTextDocument_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QQuickTextDocument10metaObjectEv()};
     unsafe {_ZNK18QQuickTextDocument10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextDocument * QQuickTextDocument::textDocument();
impl /*struct*/ QQuickTextDocument {
  pub fn textDocument<RetType, T: QQuickTextDocument_textDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textDocument(self);
    // return 1;
  }
}

pub trait QQuickTextDocument_textDocument<RetType> {
  fn textDocument(self , rsthis: & QQuickTextDocument) -> RetType;
}

  // proto:  QTextDocument * QQuickTextDocument::textDocument();
impl<'a> /*trait*/ QQuickTextDocument_textDocument<QTextDocument> for () {
  fn textDocument(self , rsthis: & QQuickTextDocument) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QQuickTextDocument12textDocumentEv()};
    let mut ret = unsafe {_ZNK18QQuickTextDocument12textDocumentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickTextDocument::QQuickTextDocument(const QQuickTextDocument & );
impl /*struct*/ QQuickTextDocument {
  pub fn new<T: QQuickTextDocument_new>(value: T) -> QQuickTextDocument {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickTextDocument_new {
  fn new(self) -> QQuickTextDocument;
}

  // proto:  void QQuickTextDocument::QQuickTextDocument(const QQuickTextDocument & );
impl<'a> /*trait*/ QQuickTextDocument_new for (&'a QQuickTextDocument) {
  fn new(self) -> QQuickTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QQuickTextDocumentC2ERKS_()};
    let ctysz: c_int = unsafe{QQuickTextDocument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QQuickTextDocumentC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickTextDocument{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickTextDocument::QQuickTextDocument(QQuickItem * parent);
impl<'a> /*trait*/ QQuickTextDocument_new for (&'a QQuickItem) {
  fn new(self) -> QQuickTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QQuickTextDocumentC2EP10QQuickItem()};
    let ctysz: c_int = unsafe{QQuickTextDocument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QQuickTextDocumentC2EP10QQuickItem(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickTextDocument{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

