// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtGui/qsyntaxhighlighter.h
// dst-file: /src/gui/qsyntaxhighlighter.rs
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
use super::qtextobject::QTextBlock; // 773
use super::qtextdocument::QTextDocument; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSyntaxHighlighter_Class_Size() -> c_int;
  // proto:  void QSyntaxHighlighter::~QSyntaxHighlighter();
  fn _ZN18QSyntaxHighlighterD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSyntaxHighlighter::rehighlight();
  fn _ZN18QSyntaxHighlighter11rehighlightEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(QObject * parent);
  fn _ZN18QSyntaxHighlighterC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
  fn _ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSyntaxHighlighter::setDocument(QTextDocument * doc);
  fn _ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(QTextDocument * parent);
  fn _ZN18QSyntaxHighlighterC2EP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(const QSyntaxHighlighter & );
  fn _ZN18QSyntaxHighlighterC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QSyntaxHighlighter::metaObject();
  fn _ZNK18QSyntaxHighlighter10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTextDocument * QSyntaxHighlighter::document();
  fn _ZNK18QSyntaxHighlighter8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSyntaxHighlighter)=1
#[derive(Default)]
pub struct QSyntaxHighlighter {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSyntaxHighlighter {
    return QSyntaxHighlighter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSyntaxHighlighter {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSyntaxHighlighter {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QSyntaxHighlighter::~QSyntaxHighlighter();
impl /*struct*/ QSyntaxHighlighter {
  pub fn free<RetType, T: QSyntaxHighlighter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_free<RetType> {
  fn free(self , rsthis: & QSyntaxHighlighter) -> RetType;
}

  // proto:  void QSyntaxHighlighter::~QSyntaxHighlighter();
impl<'a> /*trait*/ QSyntaxHighlighter_free<()> for () {
  fn free(self , rsthis: & QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterD2Ev()};
     unsafe {_ZN18QSyntaxHighlighterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSyntaxHighlighter::rehighlight();
impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlight<RetType, T: QSyntaxHighlighter_rehighlight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rehighlight(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_rehighlight<RetType> {
  fn rehighlight(self , rsthis: & QSyntaxHighlighter) -> RetType;
}

  // proto:  void QSyntaxHighlighter::rehighlight();
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlight<()> for () {
  fn rehighlight(self , rsthis: & QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter11rehighlightEv()};
     unsafe {_ZN18QSyntaxHighlighter11rehighlightEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(QObject * parent);
impl /*struct*/ QSyntaxHighlighter {
  pub fn new<T: QSyntaxHighlighter_new>(value: T) -> QSyntaxHighlighter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSyntaxHighlighter_new {
  fn new(self) -> QSyntaxHighlighter;
}

  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(QObject * parent);
impl<'a> /*trait*/ QSyntaxHighlighter_new for (&'a QObject) {
  fn new(self) -> QSyntaxHighlighter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterC2EP7QObject()};
    let ctysz: c_int = unsafe{QSyntaxHighlighter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighterC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSyntaxHighlighter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlightBlock<RetType, T: QSyntaxHighlighter_rehighlightBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rehighlightBlock(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_rehighlightBlock<RetType> {
  fn rehighlightBlock(self , rsthis: & QSyntaxHighlighter) -> RetType;
}

  // proto:  void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlightBlock<()> for (&'a QTextBlock) {
  fn rehighlightBlock(self , rsthis: & QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSyntaxHighlighter::setDocument(QTextDocument * doc);
impl /*struct*/ QSyntaxHighlighter {
  pub fn setDocument<RetType, T: QSyntaxHighlighter_setDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocument(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_setDocument<RetType> {
  fn setDocument(self , rsthis: & QSyntaxHighlighter) -> RetType;
}

  // proto:  void QSyntaxHighlighter::setDocument(QTextDocument * doc);
impl<'a> /*trait*/ QSyntaxHighlighter_setDocument<()> for (&'a QTextDocument) {
  fn setDocument(self , rsthis: & QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(QTextDocument * parent);
impl<'a> /*trait*/ QSyntaxHighlighter_new for (&'a QTextDocument) {
  fn new(self) -> QSyntaxHighlighter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterC2EP13QTextDocument()};
    let ctysz: c_int = unsafe{QSyntaxHighlighter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighterC2EP13QTextDocument(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSyntaxHighlighter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSyntaxHighlighter::QSyntaxHighlighter(const QSyntaxHighlighter & );
impl<'a> /*trait*/ QSyntaxHighlighter_new for (&'a QSyntaxHighlighter) {
  fn new(self) -> QSyntaxHighlighter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterC2ERKS_()};
    let ctysz: c_int = unsafe{QSyntaxHighlighter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighterC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSyntaxHighlighter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSyntaxHighlighter::metaObject();
impl /*struct*/ QSyntaxHighlighter {
  pub fn metaObject<RetType, T: QSyntaxHighlighter_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSyntaxHighlighter) -> RetType;
}

  // proto:  const QMetaObject * QSyntaxHighlighter::metaObject();
impl<'a> /*trait*/ QSyntaxHighlighter_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSyntaxHighlighter10metaObjectEv()};
     unsafe {_ZNK18QSyntaxHighlighter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextDocument * QSyntaxHighlighter::document();
impl /*struct*/ QSyntaxHighlighter {
  pub fn document<RetType, T: QSyntaxHighlighter_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_document<RetType> {
  fn document(self , rsthis: & QSyntaxHighlighter) -> RetType;
}

  // proto:  QTextDocument * QSyntaxHighlighter::document();
impl<'a> /*trait*/ QSyntaxHighlighter_document<QTextDocument> for () {
  fn document(self , rsthis: & QSyntaxHighlighter) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSyntaxHighlighter8documentEv()};
    let mut ret = unsafe {_ZNK18QSyntaxHighlighter8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

