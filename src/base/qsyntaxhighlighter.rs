// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qtextblock::QTextBlock;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QSyntaxHighlighter::FreeQSyntaxHighlighter();
  fn _ZN18QSyntaxHighlighterD0Ev() -> i32;
  // proto: void QSyntaxHighlighter::rehighlight();
  fn _ZN18QSyntaxHighlighter11rehighlightEv() -> i32;
  // proto: void QSyntaxHighlighter::NewQSyntaxHighlighter(QObject * parent);
  fn _ZN18QSyntaxHighlighterC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
  fn _ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: void QSyntaxHighlighter::setDocument(QTextDocument * doc);
  fn _ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument(arg0: *mut c_void) -> i32;
  // proto: void QSyntaxHighlighter::NewQSyntaxHighlighter(QTextDocument * parent);
  fn _ZN18QSyntaxHighlighterC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QSyntaxHighlighter::NewQSyntaxHighlighter(const QSyntaxHighlighter & );
  fn _ZN18QSyntaxHighlighterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QSyntaxHighlighter::metaObject();
  fn _ZNK18QSyntaxHighlighter10metaObjectEv() -> i32;
  // proto: QTextDocument * QSyntaxHighlighter::document();
  fn _ZNK18QSyntaxHighlighter8documentEv() -> i32;
}

// body block begin
// class sizeof(QSyntaxHighlighter)=1
pub struct QSyntaxHighlighter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn FreeQSyntaxHighlighter<T: QSyntaxHighlighter_FreeQSyntaxHighlighter>(&mut self, value: T) -> i32 {
    value.FreeQSyntaxHighlighter(self);
    return 1;
  }
}

pub trait QSyntaxHighlighter_FreeQSyntaxHighlighter {
  fn FreeQSyntaxHighlighter(self, this: &mut QSyntaxHighlighter) -> i32;
}

// proto: void QSyntaxHighlighter::FreeQSyntaxHighlighter();
impl<'a> /*trait*/ QSyntaxHighlighter_FreeQSyntaxHighlighter for () {
  fn FreeQSyntaxHighlighter(self, this: &mut QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterD0Ev()};
    unsafe {_ZN18QSyntaxHighlighterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlight<T: QSyntaxHighlighter_rehighlight>(&mut self, value: T) -> i32 {
    value.rehighlight(self);
    return 1;
  }
}

pub trait QSyntaxHighlighter_rehighlight {
  fn rehighlight(self, this: &mut QSyntaxHighlighter) -> i32;
}

// proto: void QSyntaxHighlighter::rehighlight();
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlight for () {
  fn rehighlight(self, this: &mut QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter11rehighlightEv()};
    unsafe {_ZN18QSyntaxHighlighter11rehighlightEv()};
    return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn NewQSyntaxHighlighter<T: QSyntaxHighlighter_NewQSyntaxHighlighter>(value: T) -> QSyntaxHighlighter {
    let rsthis = value.NewQSyntaxHighlighter();
    return rsthis;
    // return 1;
  }
}

pub trait QSyntaxHighlighter_NewQSyntaxHighlighter {
  fn NewQSyntaxHighlighter(self) -> QSyntaxHighlighter;
}

// proto: void QSyntaxHighlighter::NewQSyntaxHighlighter(QObject * parent);
impl<'a> /*trait*/ QSyntaxHighlighter_NewQSyntaxHighlighter for (&'a mut QObject) {
  fn NewQSyntaxHighlighter(self) -> QSyntaxHighlighter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighterC1EP7QObject(qthis, arg0)};
    let rsthis = QSyntaxHighlighter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlightBlock<T: QSyntaxHighlighter_rehighlightBlock>(&mut self, value: T) -> i32 {
    value.rehighlightBlock(self);
    return 1;
  }
}

pub trait QSyntaxHighlighter_rehighlightBlock {
  fn rehighlightBlock(self, this: &mut QSyntaxHighlighter) -> i32;
}

// proto: void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlightBlock for (&'a  QTextBlock) {
  fn rehighlightBlock(self, this: &mut QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn setDocument<T: QSyntaxHighlighter_setDocument>(&mut self, value: T) -> i32 {
    value.setDocument(self);
    return 1;
  }
}

pub trait QSyntaxHighlighter_setDocument {
  fn setDocument(self, this: &mut QSyntaxHighlighter) -> i32;
}

// proto: void QSyntaxHighlighter::setDocument(QTextDocument * doc);
impl<'a> /*trait*/ QSyntaxHighlighter_setDocument for (&'a mut QTextDocument) {
  fn setDocument(self, this: &mut QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument(arg0)};
    return 1;
  }
}

// proto: void QSyntaxHighlighter::NewQSyntaxHighlighter(QTextDocument * parent);
impl<'a> /*trait*/ QSyntaxHighlighter_NewQSyntaxHighlighter for (&'a mut QTextDocument) {
  fn NewQSyntaxHighlighter(self) -> QSyntaxHighlighter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighterC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QSyntaxHighlighter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSyntaxHighlighter::NewQSyntaxHighlighter(const QSyntaxHighlighter & );
impl<'a> /*trait*/ QSyntaxHighlighter_NewQSyntaxHighlighter for (&'a  QSyntaxHighlighter) {
  fn NewQSyntaxHighlighter(self) -> QSyntaxHighlighter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QSyntaxHighlighterC1ERKS_(qthis, arg0)};
    let rsthis = QSyntaxHighlighter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn metaObject<T: QSyntaxHighlighter_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSyntaxHighlighter_metaObject {
  fn metaObject(self, this: &mut QSyntaxHighlighter) -> i32;
}

// proto: const QMetaObject * QSyntaxHighlighter::metaObject();
impl<'a> /*trait*/ QSyntaxHighlighter_metaObject for () {
  fn metaObject(self, this: &mut QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSyntaxHighlighter10metaObjectEv()};
    unsafe {_ZNK18QSyntaxHighlighter10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn document<T: QSyntaxHighlighter_document>(&mut self, value: T) -> i32 {
    value.document(self);
    return 1;
  }
}

pub trait QSyntaxHighlighter_document {
  fn document(self, this: &mut QSyntaxHighlighter) -> i32;
}

// proto: QTextDocument * QSyntaxHighlighter::document();
impl<'a> /*trait*/ QSyntaxHighlighter_document for () {
  fn document(self, this: &mut QSyntaxHighlighter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSyntaxHighlighter8documentEv()};
    unsafe {_ZNK18QSyntaxHighlighter8documentEv()};
    return 1;
  }
}

