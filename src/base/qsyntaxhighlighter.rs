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
  // proto:  void QSyntaxHighlighter::FreeQSyntaxHighlighter();
  fn _ZN18QSyntaxHighlighterD0Ev(qthis: *mut c_void) ;
  // proto:  void QSyntaxHighlighter::rehighlight();
  fn _ZN18QSyntaxHighlighter11rehighlightEv(qthis: *mut c_void) ;
  // proto:  void QSyntaxHighlighter::NewQSyntaxHighlighter(QObject * parent);
  fn _ZN18QSyntaxHighlighterC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
  fn _ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSyntaxHighlighter::setDocument(QTextDocument * doc);
  fn _ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSyntaxHighlighter::NewQSyntaxHighlighter(QTextDocument * parent);
  fn _ZN18QSyntaxHighlighterC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSyntaxHighlighter::NewQSyntaxHighlighter(const QSyntaxHighlighter & );
  fn _ZN18QSyntaxHighlighterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QSyntaxHighlighter::metaObject();
  fn _ZNK18QSyntaxHighlighter10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QTextDocument * QSyntaxHighlighter::document();
  fn _ZNK18QSyntaxHighlighter8documentEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSyntaxHighlighter)=1
pub struct QSyntaxHighlighter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn FreeQSyntaxHighlighter<RetType, T: QSyntaxHighlighter_FreeQSyntaxHighlighter<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSyntaxHighlighter(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_FreeQSyntaxHighlighter<RetType> {
  fn FreeQSyntaxHighlighter(self, rsthis: &mut QSyntaxHighlighter) -> RetType;
}

// proto:  void QSyntaxHighlighter::FreeQSyntaxHighlighter();
impl<'a> /*trait*/ QSyntaxHighlighter_FreeQSyntaxHighlighter<()> for () {
  fn FreeQSyntaxHighlighter(self, rsthis: &mut QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighterD0Ev()};
     unsafe {_ZN18QSyntaxHighlighterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn rehighlight<RetType, T: QSyntaxHighlighter_rehighlight<RetType>>(&mut self, value: T) -> RetType {
    return value.rehighlight(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_rehighlight<RetType> {
  fn rehighlight(self, rsthis: &mut QSyntaxHighlighter) -> RetType;
}

// proto:  void QSyntaxHighlighter::rehighlight();
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlight<()> for () {
  fn rehighlight(self, rsthis: &mut QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter11rehighlightEv()};
     unsafe {_ZN18QSyntaxHighlighter11rehighlightEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn rehighlightBlock<RetType, T: QSyntaxHighlighter_rehighlightBlock<RetType>>(&mut self, value: T) -> RetType {
    return value.rehighlightBlock(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_rehighlightBlock<RetType> {
  fn rehighlightBlock(self, rsthis: &mut QSyntaxHighlighter) -> RetType;
}

// proto:  void QSyntaxHighlighter::rehighlightBlock(const QTextBlock & block);
impl<'a> /*trait*/ QSyntaxHighlighter_rehighlightBlock<()> for (&'a  QTextBlock) {
  fn rehighlightBlock(self, rsthis: &mut QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QSyntaxHighlighter16rehighlightBlockERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn setDocument<RetType, T: QSyntaxHighlighter_setDocument<RetType>>(&mut self, value: T) -> RetType {
    return value.setDocument(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_setDocument<RetType> {
  fn setDocument(self, rsthis: &mut QSyntaxHighlighter) -> RetType;
}

// proto:  void QSyntaxHighlighter::setDocument(QTextDocument * doc);
impl<'a> /*trait*/ QSyntaxHighlighter_setDocument<()> for (&'a mut QTextDocument) {
  fn setDocument(self, rsthis: &mut QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QSyntaxHighlighter11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QSyntaxHighlighterC1ERKS_(qthis, arg0)};
    let rsthis = QSyntaxHighlighter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn metaObject<RetType, T: QSyntaxHighlighter_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSyntaxHighlighter) -> RetType;
}

// proto:  const QMetaObject * QSyntaxHighlighter::metaObject();
impl<'a> /*trait*/ QSyntaxHighlighter_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSyntaxHighlighter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSyntaxHighlighter10metaObjectEv()};
     unsafe {_ZNK18QSyntaxHighlighter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSyntaxHighlighter {
  pub fn document<RetType, T: QSyntaxHighlighter_document<RetType>>(&mut self, value: T) -> RetType {
    return value.document(self);
    // return 1;
  }
}

pub trait QSyntaxHighlighter_document<RetType> {
  fn document(self, rsthis: &mut QSyntaxHighlighter) -> RetType;
}

// proto:  QTextDocument * QSyntaxHighlighter::document();
impl<'a> /*trait*/ QSyntaxHighlighter_document<QTextDocument> for () {
  fn document(self, rsthis: &mut QSyntaxHighlighter) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSyntaxHighlighter8documentEv()};
    let mut ret = unsafe {_ZNK18QSyntaxHighlighter8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

