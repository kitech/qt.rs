// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextframe::QTextFrame;
use super::qrectf::QRectF;
use super::qtextblock::QTextBlock;
use super::qtextdocument::QTextDocument;
use super::qsizef::QSizeF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPlainTextDocumentLayout::requestUpdate();
  fn _ZN24QPlainTextDocumentLayout13requestUpdateEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextDocumentLayout::setCursorWidth(int width);
  fn _ZN24QPlainTextDocumentLayout14setCursorWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
  fn _ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextDocumentLayout::pageCount();
  fn _ZNK24QPlainTextDocumentLayout9pageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QPlainTextDocumentLayout::metaObject();
  fn _ZNK24QPlainTextDocumentLayout10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
  fn _ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPlainTextDocumentLayout::FreeQPlainTextDocumentLayout();
  fn _ZN24QPlainTextDocumentLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
  fn _ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QPlainTextDocumentLayout::cursorWidth();
  fn _ZNK24QPlainTextDocumentLayout11cursorWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPlainTextDocumentLayout::NewQPlainTextDocumentLayout(QTextDocument * document);
  fn _ZN24QPlainTextDocumentLayoutC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSizeF QPlainTextDocumentLayout::documentSize();
  fn _ZNK24QPlainTextDocumentLayout12documentSizeEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPlainTextDocumentLayout)=1
pub struct QPlainTextDocumentLayout {
  pub qclsinst: *mut c_void,
}

// proto:  void QPlainTextDocumentLayout::requestUpdate();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn requestUpdate<RetType, T: QPlainTextDocumentLayout_requestUpdate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.requestUpdate(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_requestUpdate<RetType> {
  fn requestUpdate(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  void QPlainTextDocumentLayout::requestUpdate();
impl<'a> /*trait*/ QPlainTextDocumentLayout_requestUpdate<()> for () {
  fn requestUpdate(self , rsthis: &mut QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayout13requestUpdateEv()};
     unsafe {_ZN24QPlainTextDocumentLayout13requestUpdateEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPlainTextDocumentLayout::setCursorWidth(int width);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn setCursorWidth<RetType, T: QPlainTextDocumentLayout_setCursorWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_setCursorWidth<RetType> {
  fn setCursorWidth(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  void QPlainTextDocumentLayout::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextDocumentLayout_setCursorWidth<()> for (i32) {
  fn setCursorWidth(self , rsthis: &mut QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayout14setCursorWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QPlainTextDocumentLayout14setCursorWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn frameBoundingRect<RetType, T: QPlainTextDocumentLayout_frameBoundingRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.frameBoundingRect(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_frameBoundingRect<RetType> {
  fn frameBoundingRect(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
impl<'a> /*trait*/ QPlainTextDocumentLayout_frameBoundingRect<QRectF> for (&'a mut QTextFrame) {
  fn frameBoundingRect(self , rsthis: &mut QPlainTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QPlainTextDocumentLayout::pageCount();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn pageCount<RetType, T: QPlainTextDocumentLayout_pageCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pageCount(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_pageCount<RetType> {
  fn pageCount(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  int QPlainTextDocumentLayout::pageCount();
impl<'a> /*trait*/ QPlainTextDocumentLayout_pageCount<i32> for () {
  fn pageCount(self , rsthis: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout9pageCountEv()};
    let mut ret = unsafe {_ZNK24QPlainTextDocumentLayout9pageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  const QMetaObject * QPlainTextDocumentLayout::metaObject();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn metaObject<RetType, T: QPlainTextDocumentLayout_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  const QMetaObject * QPlainTextDocumentLayout::metaObject();
impl<'a> /*trait*/ QPlainTextDocumentLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout10metaObjectEv()};
     unsafe {_ZNK24QPlainTextDocumentLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn ensureBlockLayout<RetType, T: QPlainTextDocumentLayout_ensureBlockLayout<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.ensureBlockLayout(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_ensureBlockLayout<RetType> {
  fn ensureBlockLayout(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
impl<'a> /*trait*/ QPlainTextDocumentLayout_ensureBlockLayout<()> for (&'a  QTextBlock) {
  fn ensureBlockLayout(self , rsthis: &mut QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPlainTextDocumentLayout::FreeQPlainTextDocumentLayout();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn FreeQPlainTextDocumentLayout<RetType, T: QPlainTextDocumentLayout_FreeQPlainTextDocumentLayout<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQPlainTextDocumentLayout(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_FreeQPlainTextDocumentLayout<RetType> {
  fn FreeQPlainTextDocumentLayout(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  void QPlainTextDocumentLayout::FreeQPlainTextDocumentLayout();
impl<'a> /*trait*/ QPlainTextDocumentLayout_FreeQPlainTextDocumentLayout<()> for () {
  fn FreeQPlainTextDocumentLayout(self , rsthis: &mut QPlainTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayoutD0Ev()};
     unsafe {_ZN24QPlainTextDocumentLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn blockBoundingRect<RetType, T: QPlainTextDocumentLayout_blockBoundingRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.blockBoundingRect(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_blockBoundingRect<RetType> {
  fn blockBoundingRect(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl<'a> /*trait*/ QPlainTextDocumentLayout_blockBoundingRect<QRectF> for (&'a  QTextBlock) {
  fn blockBoundingRect(self , rsthis: &mut QPlainTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QPlainTextDocumentLayout::cursorWidth();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn cursorWidth<RetType, T: QPlainTextDocumentLayout_cursorWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cursorWidth(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_cursorWidth<RetType> {
  fn cursorWidth(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  int QPlainTextDocumentLayout::cursorWidth();
impl<'a> /*trait*/ QPlainTextDocumentLayout_cursorWidth<i32> for () {
  fn cursorWidth(self , rsthis: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout11cursorWidthEv()};
    let mut ret = unsafe {_ZNK24QPlainTextDocumentLayout11cursorWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn NewQPlainTextDocumentLayout<T: QPlainTextDocumentLayout_NewQPlainTextDocumentLayout>(value: T) -> QPlainTextDocumentLayout {
    let rsthis = value.NewQPlainTextDocumentLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_NewQPlainTextDocumentLayout {
  fn NewQPlainTextDocumentLayout(self) -> QPlainTextDocumentLayout;
}

// proto: void QPlainTextDocumentLayout::NewQPlainTextDocumentLayout(QTextDocument * document);
impl<'a> /*trait*/ QPlainTextDocumentLayout_NewQPlainTextDocumentLayout for (&'a mut QTextDocument) {
  fn NewQPlainTextDocumentLayout(self) -> QPlainTextDocumentLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayoutC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QPlainTextDocumentLayoutC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QPlainTextDocumentLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QSizeF QPlainTextDocumentLayout::documentSize();
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn documentSize<RetType, T: QPlainTextDocumentLayout_documentSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.documentSize(self);
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_documentSize<RetType> {
  fn documentSize(self , rsthis: &mut QPlainTextDocumentLayout) -> RetType;
}

// proto:  QSizeF QPlainTextDocumentLayout::documentSize();
impl<'a> /*trait*/ QPlainTextDocumentLayout_documentSize<QSizeF> for () {
  fn documentSize(self , rsthis: &mut QPlainTextDocumentLayout) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout12documentSizeEv()};
    let mut ret = unsafe {_ZNK24QPlainTextDocumentLayout12documentSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

