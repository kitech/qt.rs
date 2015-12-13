// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextframelayoutdata::QTextFrameLayoutData;
use super::qtextframeformat::QTextFrameFormat;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QTextFrameFormat QTextFrame::frameFormat();
  fn _ZNK10QTextFrame11frameFormatEv() -> i32;
  // proto: QTextFrameLayoutData * QTextFrame::layoutData();
  fn _ZNK10QTextFrame10layoutDataEv() -> i32;
  // proto: void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
  fn _ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData(arg0: *mut c_void) -> i32;
  // proto: void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
  fn _ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat(arg0: *const c_void) -> i32;
  // proto: void QTextFrame::NewQTextFrame(const QTextFrame & );
  fn _ZN10QTextFrameC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QTextFrame::metaObject();
  fn _ZNK10QTextFrame10metaObjectEv() -> i32;
  // proto: QTextFrame * QTextFrame::parentFrame();
  fn _ZNK10QTextFrame11parentFrameEv() -> i32;
  // proto: int QTextFrame::firstPosition();
  fn _ZNK10QTextFrame13firstPositionEv() -> i32;
  // proto: QList<QTextFrame *> QTextFrame::childFrames();
  fn _ZNK10QTextFrame11childFramesEv() -> i32;
  // proto: void QTextFrame::FreeQTextFrame();
  fn _ZN10QTextFrameD0Ev() -> i32;
  // proto: QTextCursor QTextFrame::lastCursorPosition();
  fn _ZNK10QTextFrame18lastCursorPositionEv() -> i32;
  // proto: void QTextFrame::NewQTextFrame(QTextDocument * doc);
  fn _ZN10QTextFrameC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QTextFrame::lastPosition();
  fn _ZNK10QTextFrame12lastPositionEv() -> i32;
  // proto: QTextCursor QTextFrame::firstCursorPosition();
  fn _ZNK10QTextFrame19firstCursorPositionEv() -> i32;
}

// body block begin
// class sizeof(QTextFrame)=1
pub struct QTextFrame {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFrame {
  pub fn frameFormat<T: QTextFrame_frameFormat>(&mut self, value: T) -> i32 {
    value.frameFormat(self);
    return 1;
  }
}

pub trait QTextFrame_frameFormat {
  fn frameFormat(self, this: &mut QTextFrame) -> i32;
}

// proto: QTextFrameFormat QTextFrame::frameFormat();
impl<'a> /*trait*/ QTextFrame_frameFormat for () {
  fn frameFormat(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11frameFormatEv()};
    unsafe {_ZNK10QTextFrame11frameFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn layoutData<T: QTextFrame_layoutData>(&mut self, value: T) -> i32 {
    value.layoutData(self);
    return 1;
  }
}

pub trait QTextFrame_layoutData {
  fn layoutData(self, this: &mut QTextFrame) -> i32;
}

// proto: QTextFrameLayoutData * QTextFrame::layoutData();
impl<'a> /*trait*/ QTextFrame_layoutData for () {
  fn layoutData(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame10layoutDataEv()};
    unsafe {_ZNK10QTextFrame10layoutDataEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn setLayoutData<T: QTextFrame_setLayoutData>(&mut self, value: T) -> i32 {
    value.setLayoutData(self);
    return 1;
  }
}

pub trait QTextFrame_setLayoutData {
  fn setLayoutData(self, this: &mut QTextFrame) -> i32;
}

// proto: void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
impl<'a> /*trait*/ QTextFrame_setLayoutData for (&'a mut QTextFrameLayoutData) {
  fn setLayoutData(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn setFrameFormat<T: QTextFrame_setFrameFormat>(&mut self, value: T) -> i32 {
    value.setFrameFormat(self);
    return 1;
  }
}

pub trait QTextFrame_setFrameFormat {
  fn setFrameFormat(self, this: &mut QTextFrame) -> i32;
}

// proto: void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
impl<'a> /*trait*/ QTextFrame_setFrameFormat for (&'a  QTextFrameFormat) {
  fn setFrameFormat(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn NewQTextFrame<T: QTextFrame_NewQTextFrame>(value: T) -> QTextFrame {
    let rsthis = value.NewQTextFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrame_NewQTextFrame {
  fn NewQTextFrame(self) -> QTextFrame;
}

// proto: void QTextFrame::NewQTextFrame(const QTextFrame & );
impl<'a> /*trait*/ QTextFrame_NewQTextFrame for (&'a  QTextFrame) {
  fn NewQTextFrame(self) -> QTextFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextFrameC1ERKS_(qthis, arg0)};
    let rsthis = QTextFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn metaObject<T: QTextFrame_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextFrame_metaObject {
  fn metaObject(self, this: &mut QTextFrame) -> i32;
}

// proto: const QMetaObject * QTextFrame::metaObject();
impl<'a> /*trait*/ QTextFrame_metaObject for () {
  fn metaObject(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame10metaObjectEv()};
    unsafe {_ZNK10QTextFrame10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn parentFrame<T: QTextFrame_parentFrame>(&mut self, value: T) -> i32 {
    value.parentFrame(self);
    return 1;
  }
}

pub trait QTextFrame_parentFrame {
  fn parentFrame(self, this: &mut QTextFrame) -> i32;
}

// proto: QTextFrame * QTextFrame::parentFrame();
impl<'a> /*trait*/ QTextFrame_parentFrame for () {
  fn parentFrame(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11parentFrameEv()};
    unsafe {_ZNK10QTextFrame11parentFrameEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn firstPosition<T: QTextFrame_firstPosition>(&mut self, value: T) -> i32 {
    value.firstPosition(self);
    return 1;
  }
}

pub trait QTextFrame_firstPosition {
  fn firstPosition(self, this: &mut QTextFrame) -> i32;
}

// proto: int QTextFrame::firstPosition();
impl<'a> /*trait*/ QTextFrame_firstPosition for () {
  fn firstPosition(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame13firstPositionEv()};
    unsafe {_ZNK10QTextFrame13firstPositionEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn childFrames<T: QTextFrame_childFrames>(&mut self, value: T) -> i32 {
    value.childFrames(self);
    return 1;
  }
}

pub trait QTextFrame_childFrames {
  fn childFrames(self, this: &mut QTextFrame) -> i32;
}

// proto: QList<QTextFrame *> QTextFrame::childFrames();
impl<'a> /*trait*/ QTextFrame_childFrames for () {
  fn childFrames(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11childFramesEv()};
    unsafe {_ZNK10QTextFrame11childFramesEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn FreeQTextFrame<T: QTextFrame_FreeQTextFrame>(&mut self, value: T) -> i32 {
    value.FreeQTextFrame(self);
    return 1;
  }
}

pub trait QTextFrame_FreeQTextFrame {
  fn FreeQTextFrame(self, this: &mut QTextFrame) -> i32;
}

// proto: void QTextFrame::FreeQTextFrame();
impl<'a> /*trait*/ QTextFrame_FreeQTextFrame for () {
  fn FreeQTextFrame(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameD0Ev()};
    unsafe {_ZN10QTextFrameD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn lastCursorPosition<T: QTextFrame_lastCursorPosition>(&mut self, value: T) -> i32 {
    value.lastCursorPosition(self);
    return 1;
  }
}

pub trait QTextFrame_lastCursorPosition {
  fn lastCursorPosition(self, this: &mut QTextFrame) -> i32;
}

// proto: QTextCursor QTextFrame::lastCursorPosition();
impl<'a> /*trait*/ QTextFrame_lastCursorPosition for () {
  fn lastCursorPosition(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame18lastCursorPositionEv()};
    unsafe {_ZNK10QTextFrame18lastCursorPositionEv()};
    return 1;
  }
}

// proto: void QTextFrame::NewQTextFrame(QTextDocument * doc);
impl<'a> /*trait*/ QTextFrame_NewQTextFrame for (&'a mut QTextDocument) {
  fn NewQTextFrame(self) -> QTextFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextFrameC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn lastPosition<T: QTextFrame_lastPosition>(&mut self, value: T) -> i32 {
    value.lastPosition(self);
    return 1;
  }
}

pub trait QTextFrame_lastPosition {
  fn lastPosition(self, this: &mut QTextFrame) -> i32;
}

// proto: int QTextFrame::lastPosition();
impl<'a> /*trait*/ QTextFrame_lastPosition for () {
  fn lastPosition(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame12lastPositionEv()};
    unsafe {_ZNK10QTextFrame12lastPositionEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn firstCursorPosition<T: QTextFrame_firstCursorPosition>(&mut self, value: T) -> i32 {
    value.firstCursorPosition(self);
    return 1;
  }
}

pub trait QTextFrame_firstCursorPosition {
  fn firstCursorPosition(self, this: &mut QTextFrame) -> i32;
}

// proto: QTextCursor QTextFrame::firstCursorPosition();
impl<'a> /*trait*/ QTextFrame_firstCursorPosition for () {
  fn firstCursorPosition(self, this: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame19firstCursorPositionEv()};
    unsafe {_ZNK10QTextFrame19firstCursorPositionEv()};
    return 1;
  }
}

