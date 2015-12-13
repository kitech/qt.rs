// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextframe::QTextFrame;
use super::qtextblock::QTextBlock;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPlainTextDocumentLayout::requestUpdate();
  fn _ZN24QPlainTextDocumentLayout13requestUpdateEv() -> i32;
  // proto: void QPlainTextDocumentLayout::setCursorWidth(int width);
  fn _ZN24QPlainTextDocumentLayout14setCursorWidthEi(arg0: c_int) -> i32;
  // proto: QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
  fn _ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame(arg0: *mut c_void) -> i32;
  // proto: int QPlainTextDocumentLayout::pageCount();
  fn _ZNK24QPlainTextDocumentLayout9pageCountEv() -> i32;
  // proto: const QMetaObject * QPlainTextDocumentLayout::metaObject();
  fn _ZNK24QPlainTextDocumentLayout10metaObjectEv() -> i32;
  // proto: void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
  fn _ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: void QPlainTextDocumentLayout::FreeQPlainTextDocumentLayout();
  fn _ZN24QPlainTextDocumentLayoutD0Ev() -> i32;
  // proto: QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
  fn _ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock(arg0: *const c_void) -> i32;
  // proto: int QPlainTextDocumentLayout::cursorWidth();
  fn _ZNK24QPlainTextDocumentLayout11cursorWidthEv() -> i32;
  // proto: void QPlainTextDocumentLayout::NewQPlainTextDocumentLayout(QTextDocument * document);
  fn _ZN24QPlainTextDocumentLayoutC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QSizeF QPlainTextDocumentLayout::documentSize();
  fn _ZNK24QPlainTextDocumentLayout12documentSizeEv() -> i32;
}

// body block begin
// class sizeof(QPlainTextDocumentLayout)=1
pub struct QPlainTextDocumentLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn requestUpdate<T: QPlainTextDocumentLayout_requestUpdate>(&mut self, value: T) -> i32 {
    value.requestUpdate(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_requestUpdate {
  fn requestUpdate(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: void QPlainTextDocumentLayout::requestUpdate();
impl<'a> /*trait*/ QPlainTextDocumentLayout_requestUpdate for () {
  fn requestUpdate(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayout13requestUpdateEv()};
    unsafe {_ZN24QPlainTextDocumentLayout13requestUpdateEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn setCursorWidth<T: QPlainTextDocumentLayout_setCursorWidth>(&mut self, value: T) -> i32 {
    value.setCursorWidth(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_setCursorWidth {
  fn setCursorWidth(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: void QPlainTextDocumentLayout::setCursorWidth(int width);
impl<'a> /*trait*/ QPlainTextDocumentLayout_setCursorWidth for (i32) {
  fn setCursorWidth(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayout14setCursorWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN24QPlainTextDocumentLayout14setCursorWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn frameBoundingRect<T: QPlainTextDocumentLayout_frameBoundingRect>(&mut self, value: T) -> i32 {
    value.frameBoundingRect(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_frameBoundingRect {
  fn frameBoundingRect(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame * );
impl<'a> /*trait*/ QPlainTextDocumentLayout_frameBoundingRect for (&'a mut QTextFrame) {
  fn frameBoundingRect(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn pageCount<T: QPlainTextDocumentLayout_pageCount>(&mut self, value: T) -> i32 {
    value.pageCount(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_pageCount {
  fn pageCount(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: int QPlainTextDocumentLayout::pageCount();
impl<'a> /*trait*/ QPlainTextDocumentLayout_pageCount for () {
  fn pageCount(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout9pageCountEv()};
    unsafe {_ZNK24QPlainTextDocumentLayout9pageCountEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn metaObject<T: QPlainTextDocumentLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_metaObject {
  fn metaObject(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: const QMetaObject * QPlainTextDocumentLayout::metaObject();
impl<'a> /*trait*/ QPlainTextDocumentLayout_metaObject for () {
  fn metaObject(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout10metaObjectEv()};
    unsafe {_ZNK24QPlainTextDocumentLayout10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn ensureBlockLayout<T: QPlainTextDocumentLayout_ensureBlockLayout>(&mut self, value: T) -> i32 {
    value.ensureBlockLayout(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_ensureBlockLayout {
  fn ensureBlockLayout(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock & block);
impl<'a> /*trait*/ QPlainTextDocumentLayout_ensureBlockLayout for (&'a  QTextBlock) {
  fn ensureBlockLayout(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn FreeQPlainTextDocumentLayout<T: QPlainTextDocumentLayout_FreeQPlainTextDocumentLayout>(&mut self, value: T) -> i32 {
    value.FreeQPlainTextDocumentLayout(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_FreeQPlainTextDocumentLayout {
  fn FreeQPlainTextDocumentLayout(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: void QPlainTextDocumentLayout::FreeQPlainTextDocumentLayout();
impl<'a> /*trait*/ QPlainTextDocumentLayout_FreeQPlainTextDocumentLayout for () {
  fn FreeQPlainTextDocumentLayout(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QPlainTextDocumentLayoutD0Ev()};
    unsafe {_ZN24QPlainTextDocumentLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn blockBoundingRect<T: QPlainTextDocumentLayout_blockBoundingRect>(&mut self, value: T) -> i32 {
    value.blockBoundingRect(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_blockBoundingRect {
  fn blockBoundingRect(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl<'a> /*trait*/ QPlainTextDocumentLayout_blockBoundingRect for (&'a  QTextBlock) {
  fn blockBoundingRect(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock(arg0)};
    return 1;
  }
}

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn cursorWidth<T: QPlainTextDocumentLayout_cursorWidth>(&mut self, value: T) -> i32 {
    value.cursorWidth(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_cursorWidth {
  fn cursorWidth(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: int QPlainTextDocumentLayout::cursorWidth();
impl<'a> /*trait*/ QPlainTextDocumentLayout_cursorWidth for () {
  fn cursorWidth(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout11cursorWidthEv()};
    unsafe {_ZNK24QPlainTextDocumentLayout11cursorWidthEv()};
    return 1;
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

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn documentSize<T: QPlainTextDocumentLayout_documentSize>(&mut self, value: T) -> i32 {
    value.documentSize(self);
    return 1;
  }
}

pub trait QPlainTextDocumentLayout_documentSize {
  fn documentSize(self, this: &mut QPlainTextDocumentLayout) -> i32;
}

// proto: QSizeF QPlainTextDocumentLayout::documentSize();
impl<'a> /*trait*/ QPlainTextDocumentLayout_documentSize for () {
  fn documentSize(self, this: &mut QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QPlainTextDocumentLayout12documentSizeEv()};
    unsafe {_ZNK24QPlainTextDocumentLayout12documentSizeEv()};
    return 1;
  }
}

