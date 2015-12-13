// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QFrame::setFrameRect(const QRect & );
  fn _ZN6QFrame12setFrameRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: int QFrame::lineWidth();
  fn _ZNK6QFrame9lineWidthEv() -> i32;
  // proto: void QFrame::setFrameStyle(int );
  fn _ZN6QFrame13setFrameStyleEi(arg0: c_int) -> i32;
  // proto: QRect QFrame::frameRect();
  fn _ZNK6QFrame9frameRectEv() -> i32;
  // proto: QSize QFrame::sizeHint();
  fn _ZNK6QFrame8sizeHintEv() -> i32;
  // proto: void QFrame::NewQFrame(const QFrame & );
  fn _ZN6QFrameC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QFrame::frameStyle();
  fn _ZNK6QFrame10frameStyleEv() -> i32;
  // proto: int QFrame::midLineWidth();
  fn _ZNK6QFrame12midLineWidthEv() -> i32;
  // proto: void QFrame::setLineWidth(int );
  fn _ZN6QFrame12setLineWidthEi(arg0: c_int) -> i32;
  // proto: void QFrame::setMidLineWidth(int );
  fn _ZN6QFrame15setMidLineWidthEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QFrame::metaObject();
  fn _ZNK6QFrame10metaObjectEv() -> i32;
  // proto: int QFrame::frameWidth();
  fn _ZNK6QFrame10frameWidthEv() -> i32;
  // proto: void QFrame::FreeQFrame();
  fn _ZN6QFrameD0Ev() -> i32;
}

// body block begin
// class sizeof(QFrame)=1
pub struct QFrame {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFrame {
  pub fn setFrameRect<T: QFrame_setFrameRect>(&mut self, value: T) -> i32 {
    value.setFrameRect(self);
    return 1;
  }
}

pub trait QFrame_setFrameRect {
  fn setFrameRect(self, this: &mut QFrame) -> i32;
}

// proto: void QFrame::setFrameRect(const QRect & );
impl<'a> /*trait*/ QFrame_setFrameRect for (&'a  QRect) {
  fn setFrameRect(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setFrameRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QFrame12setFrameRectERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn lineWidth<T: QFrame_lineWidth>(&mut self, value: T) -> i32 {
    value.lineWidth(self);
    return 1;
  }
}

pub trait QFrame_lineWidth {
  fn lineWidth(self, this: &mut QFrame) -> i32;
}

// proto: int QFrame::lineWidth();
impl<'a> /*trait*/ QFrame_lineWidth for () {
  fn lineWidth(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9lineWidthEv()};
    unsafe {_ZNK6QFrame9lineWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setFrameStyle<T: QFrame_setFrameStyle>(&mut self, value: T) -> i32 {
    value.setFrameStyle(self);
    return 1;
  }
}

pub trait QFrame_setFrameStyle {
  fn setFrameStyle(self, this: &mut QFrame) -> i32;
}

// proto: void QFrame::setFrameStyle(int );
impl<'a> /*trait*/ QFrame_setFrameStyle for (i32) {
  fn setFrameStyle(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame13setFrameStyleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QFrame13setFrameStyleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameRect<T: QFrame_frameRect>(&mut self, value: T) -> i32 {
    value.frameRect(self);
    return 1;
  }
}

pub trait QFrame_frameRect {
  fn frameRect(self, this: &mut QFrame) -> i32;
}

// proto: QRect QFrame::frameRect();
impl<'a> /*trait*/ QFrame_frameRect for () {
  fn frameRect(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9frameRectEv()};
    unsafe {_ZNK6QFrame9frameRectEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn sizeHint<T: QFrame_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QFrame_sizeHint {
  fn sizeHint(self, this: &mut QFrame) -> i32;
}

// proto: QSize QFrame::sizeHint();
impl<'a> /*trait*/ QFrame_sizeHint for () {
  fn sizeHint(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame8sizeHintEv()};
    unsafe {_ZNK6QFrame8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn NewQFrame<T: QFrame_NewQFrame>(value: T) -> QFrame {
    let rsthis = value.NewQFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QFrame_NewQFrame {
  fn NewQFrame(self) -> QFrame;
}

// proto: void QFrame::NewQFrame(const QFrame & );
impl<'a> /*trait*/ QFrame_NewQFrame for (&'a  QFrame) {
  fn NewQFrame(self) -> QFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QFrameC1ERKS_(qthis, arg0)};
    let rsthis = QFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameStyle<T: QFrame_frameStyle>(&mut self, value: T) -> i32 {
    value.frameStyle(self);
    return 1;
  }
}

pub trait QFrame_frameStyle {
  fn frameStyle(self, this: &mut QFrame) -> i32;
}

// proto: int QFrame::frameStyle();
impl<'a> /*trait*/ QFrame_frameStyle for () {
  fn frameStyle(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameStyleEv()};
    unsafe {_ZNK6QFrame10frameStyleEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn midLineWidth<T: QFrame_midLineWidth>(&mut self, value: T) -> i32 {
    value.midLineWidth(self);
    return 1;
  }
}

pub trait QFrame_midLineWidth {
  fn midLineWidth(self, this: &mut QFrame) -> i32;
}

// proto: int QFrame::midLineWidth();
impl<'a> /*trait*/ QFrame_midLineWidth for () {
  fn midLineWidth(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame12midLineWidthEv()};
    unsafe {_ZNK6QFrame12midLineWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setLineWidth<T: QFrame_setLineWidth>(&mut self, value: T) -> i32 {
    value.setLineWidth(self);
    return 1;
  }
}

pub trait QFrame_setLineWidth {
  fn setLineWidth(self, this: &mut QFrame) -> i32;
}

// proto: void QFrame::setLineWidth(int );
impl<'a> /*trait*/ QFrame_setLineWidth for (i32) {
  fn setLineWidth(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setLineWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QFrame12setLineWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setMidLineWidth<T: QFrame_setMidLineWidth>(&mut self, value: T) -> i32 {
    value.setMidLineWidth(self);
    return 1;
  }
}

pub trait QFrame_setMidLineWidth {
  fn setMidLineWidth(self, this: &mut QFrame) -> i32;
}

// proto: void QFrame::setMidLineWidth(int );
impl<'a> /*trait*/ QFrame_setMidLineWidth for (i32) {
  fn setMidLineWidth(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame15setMidLineWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QFrame15setMidLineWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn metaObject<T: QFrame_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFrame_metaObject {
  fn metaObject(self, this: &mut QFrame) -> i32;
}

// proto: const QMetaObject * QFrame::metaObject();
impl<'a> /*trait*/ QFrame_metaObject for () {
  fn metaObject(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10metaObjectEv()};
    unsafe {_ZNK6QFrame10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameWidth<T: QFrame_frameWidth>(&mut self, value: T) -> i32 {
    value.frameWidth(self);
    return 1;
  }
}

pub trait QFrame_frameWidth {
  fn frameWidth(self, this: &mut QFrame) -> i32;
}

// proto: int QFrame::frameWidth();
impl<'a> /*trait*/ QFrame_frameWidth for () {
  fn frameWidth(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameWidthEv()};
    unsafe {_ZNK6QFrame10frameWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn FreeQFrame<T: QFrame_FreeQFrame>(&mut self, value: T) -> i32 {
    value.FreeQFrame(self);
    return 1;
  }
}

pub trait QFrame_FreeQFrame {
  fn FreeQFrame(self, this: &mut QFrame) -> i32;
}

// proto: void QFrame::FreeQFrame();
impl<'a> /*trait*/ QFrame_FreeQFrame for () {
  fn FreeQFrame(self, this: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameD0Ev()};
    unsafe {_ZN6QFrameD0Ev()};
    return 1;
  }
}

