// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextdocument::QTextDocument;
use super::qtextformat::QTextFormat;
use super::qpainter::QPainter;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextObjectInterface::FreeQTextObjectInterface();
  fn _ZN20QTextObjectInterfaceD0Ev() -> i32;
  // proto: QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(arg0: *mut c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void, arg3: c_int, arg4: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTextObjectInterface)=8
pub struct QTextObjectInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextObjectInterface {
  pub fn FreeQTextObjectInterface<T: QTextObjectInterface_FreeQTextObjectInterface>(&mut self, value: T) -> i32 {
    value.FreeQTextObjectInterface(self);
    return 1;
  }
}

pub trait QTextObjectInterface_FreeQTextObjectInterface {
  fn FreeQTextObjectInterface(self, this: &mut QTextObjectInterface) -> i32;
}

// proto: void QTextObjectInterface::FreeQTextObjectInterface();
impl<'a> /*trait*/ QTextObjectInterface_FreeQTextObjectInterface for () {
  fn FreeQTextObjectInterface(self, this: &mut QTextObjectInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterfaceD0Ev()};
    unsafe {_ZN20QTextObjectInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextObjectInterface {
  pub fn intrinsicSize<T: QTextObjectInterface_intrinsicSize>(&mut self, value: T) -> i32 {
    value.intrinsicSize(self);
    return 1;
  }
}

pub trait QTextObjectInterface_intrinsicSize {
  fn intrinsicSize(self, this: &mut QTextObjectInterface) -> i32;
}

// proto: QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_intrinsicSize for (&'a mut QTextDocument, i32, &'a  QTextFormat) {
  fn intrinsicSize(self, this: &mut QTextObjectInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTextObjectInterface {
  pub fn drawObject<T: QTextObjectInterface_drawObject>(&mut self, value: T) -> i32 {
    value.drawObject(self);
    return 1;
  }
}

pub trait QTextObjectInterface_drawObject {
  fn drawObject(self, this: &mut QTextObjectInterface) -> i32;
}

// proto: void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_drawObject for (&'a mut QPainter, &'a  QRectF, &'a mut QTextDocument, i32, &'a  QTextFormat) {
  fn drawObject(self, this: &mut QTextObjectInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

