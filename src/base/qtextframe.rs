// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextframeformat::QTextFrameFormat;
use super::qtextframelayoutdata::QTextFrameLayoutData;
use super::qtextcursor::QTextCursor;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTextFrameFormat QTextFrame::frameFormat();
  fn _ZNK10QTextFrame11frameFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextFrameLayoutData * QTextFrame::layoutData();
  fn _ZNK10QTextFrame10layoutDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
  fn _ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
  fn _ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextFrame::NewQTextFrame(const QTextFrame & );
  fn _ZN10QTextFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QTextFrame::metaObject();
  fn _ZNK10QTextFrame10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QTextFrame * QTextFrame::parentFrame();
  fn _ZNK10QTextFrame11parentFrameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextFrame::firstPosition();
  fn _ZNK10QTextFrame13firstPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QTextFrame *> QTextFrame::childFrames();
  fn _ZNK10QTextFrame11childFramesEv(qthis: *mut c_void) ;
  // proto:  void QTextFrame::FreeQTextFrame();
  fn _ZN10QTextFrameD0Ev(qthis: *mut c_void) ;
  // proto:  QTextCursor QTextFrame::lastCursorPosition();
  fn _ZNK10QTextFrame18lastCursorPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrame::NewQTextFrame(QTextDocument * doc);
  fn _ZN10QTextFrameC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextFrame::lastPosition();
  fn _ZNK10QTextFrame12lastPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCursor QTextFrame::firstCursorPosition();
  fn _ZNK10QTextFrame19firstCursorPositionEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QTextFrame)=1
pub struct QTextFrame {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFrame {
  pub fn frameFormat<T: QTextFrame_frameFormat>(&mut self, value: T) -> QTextFrameFormat {
    return value.frameFormat(self);
    // return 1;
  }
}

pub trait QTextFrame_frameFormat {
  fn frameFormat(self, rsthis: &mut QTextFrame) -> QTextFrameFormat;
}

// proto:  QTextFrameFormat QTextFrame::frameFormat();
impl<'a> /*trait*/ QTextFrame_frameFormat for () {
  fn frameFormat(self, rsthis: &mut QTextFrame) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11frameFormatEv()};
    let mut ret = unsafe {_ZNK10QTextFrame11frameFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn layoutData<T: QTextFrame_layoutData>(&mut self, value: T) -> QTextFrameLayoutData {
    return value.layoutData(self);
    // return 1;
  }
}

pub trait QTextFrame_layoutData {
  fn layoutData(self, rsthis: &mut QTextFrame) -> QTextFrameLayoutData;
}

// proto:  QTextFrameLayoutData * QTextFrame::layoutData();
impl<'a> /*trait*/ QTextFrame_layoutData for () {
  fn layoutData(self, rsthis: &mut QTextFrame) -> QTextFrameLayoutData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame10layoutDataEv()};
    let mut ret = unsafe {_ZNK10QTextFrame10layoutDataEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameLayoutData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn setLayoutData<T: QTextFrame_setLayoutData>(&mut self, value: T)  {
     value.setLayoutData(self);
    // return 1;
  }
}

pub trait QTextFrame_setLayoutData {
  fn setLayoutData(self, rsthis: &mut QTextFrame) ;
}

// proto:  void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
impl<'a> /*trait*/ QTextFrame_setLayoutData for (&'a mut QTextFrameLayoutData) {
  fn setLayoutData(self, rsthis: &mut QTextFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn setFrameFormat<T: QTextFrame_setFrameFormat>(&mut self, value: T)  {
     value.setFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFrame_setFrameFormat {
  fn setFrameFormat(self, rsthis: &mut QTextFrame) ;
}

// proto:  void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
impl<'a> /*trait*/ QTextFrame_setFrameFormat for (&'a  QTextFrameFormat) {
  fn setFrameFormat(self, rsthis: &mut QTextFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextFrameC1ERKS_(qthis, arg0)};
    let rsthis = QTextFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn metaObject<T: QTextFrame_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTextFrame_metaObject {
  fn metaObject(self, rsthis: &mut QTextFrame) ;
}

// proto:  const QMetaObject * QTextFrame::metaObject();
impl<'a> /*trait*/ QTextFrame_metaObject for () {
  fn metaObject(self, rsthis: &mut QTextFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame10metaObjectEv()};
     unsafe {_ZNK10QTextFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn parentFrame<T: QTextFrame_parentFrame>(&mut self, value: T) -> QTextFrame {
    return value.parentFrame(self);
    // return 1;
  }
}

pub trait QTextFrame_parentFrame {
  fn parentFrame(self, rsthis: &mut QTextFrame) -> QTextFrame;
}

// proto:  QTextFrame * QTextFrame::parentFrame();
impl<'a> /*trait*/ QTextFrame_parentFrame for () {
  fn parentFrame(self, rsthis: &mut QTextFrame) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11parentFrameEv()};
    let mut ret = unsafe {_ZNK10QTextFrame11parentFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn firstPosition<T: QTextFrame_firstPosition>(&mut self, value: T) -> i32 {
    return value.firstPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_firstPosition {
  fn firstPosition(self, rsthis: &mut QTextFrame) -> i32;
}

// proto:  int QTextFrame::firstPosition();
impl<'a> /*trait*/ QTextFrame_firstPosition for () {
  fn firstPosition(self, rsthis: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame13firstPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame13firstPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn childFrames<T: QTextFrame_childFrames>(&mut self, value: T)  {
     value.childFrames(self);
    // return 1;
  }
}

pub trait QTextFrame_childFrames {
  fn childFrames(self, rsthis: &mut QTextFrame) ;
}

// proto:  QList<QTextFrame *> QTextFrame::childFrames();
impl<'a> /*trait*/ QTextFrame_childFrames for () {
  fn childFrames(self, rsthis: &mut QTextFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11childFramesEv()};
     unsafe {_ZNK10QTextFrame11childFramesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn FreeQTextFrame<T: QTextFrame_FreeQTextFrame>(&mut self, value: T)  {
     value.FreeQTextFrame(self);
    // return 1;
  }
}

pub trait QTextFrame_FreeQTextFrame {
  fn FreeQTextFrame(self, rsthis: &mut QTextFrame) ;
}

// proto:  void QTextFrame::FreeQTextFrame();
impl<'a> /*trait*/ QTextFrame_FreeQTextFrame for () {
  fn FreeQTextFrame(self, rsthis: &mut QTextFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameD0Ev()};
     unsafe {_ZN10QTextFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn lastCursorPosition<T: QTextFrame_lastCursorPosition>(&mut self, value: T) -> QTextCursor {
    return value.lastCursorPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_lastCursorPosition {
  fn lastCursorPosition(self, rsthis: &mut QTextFrame) -> QTextCursor;
}

// proto:  QTextCursor QTextFrame::lastCursorPosition();
impl<'a> /*trait*/ QTextFrame_lastCursorPosition for () {
  fn lastCursorPosition(self, rsthis: &mut QTextFrame) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame18lastCursorPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame18lastCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
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
    return value.lastPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_lastPosition {
  fn lastPosition(self, rsthis: &mut QTextFrame) -> i32;
}

// proto:  int QTextFrame::lastPosition();
impl<'a> /*trait*/ QTextFrame_lastPosition for () {
  fn lastPosition(self, rsthis: &mut QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame12lastPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame12lastPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn firstCursorPosition<T: QTextFrame_firstCursorPosition>(&mut self, value: T) -> QTextCursor {
    return value.firstCursorPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_firstCursorPosition {
  fn firstCursorPosition(self, rsthis: &mut QTextFrame) -> QTextCursor;
}

// proto:  QTextCursor QTextFrame::firstCursorPosition();
impl<'a> /*trait*/ QTextFrame_firstCursorPosition for () {
  fn firstCursorPosition(self, rsthis: &mut QTextFrame) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame19firstCursorPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame19firstCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

