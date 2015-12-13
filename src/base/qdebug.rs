// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN6QDebug7noquoteEv() -> i32;
  fn _ZN6QDebugD0Ev() -> i32;
  fn _ZN6QDebugC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN6QDebug5spaceEv() -> i32;
  fn _ZN6QDebugC1E9QtMsgType(qthis: *mut c_void, arg0: c_int) -> i32;
  fn _ZN6QDebug10maybeSpaceEv() -> i32;
  fn _ZN6QDebug11resetFormatEv() -> i32;
  fn _ZN6QDebug19setAutoInsertSpacesEb(arg0: int8_t) -> i32;
  fn _ZN6QDebugC1EP7QString(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN6QDebug4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN6QDebug7nospaceEv() -> i32;
  fn _ZNK6QDebug16autoInsertSpacesEv() -> i32;
  fn _ZN6QDebugC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN6QDebug5quoteEv() -> i32;
  fn _ZN6QDebug10maybeQuoteEc(arg0: c_char) -> i32;
}

// body block begin
// class sizeof(QDebug)=8
pub struct QDebug {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDebug {
  pub fn noquote<T: QDebug_noquote>(&mut self, value: T) -> i32 {
    value.noquote(self);
    return 1;
  }
}

pub trait QDebug_noquote {
  fn noquote(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::noquote();
impl<'a> /*trait*/ QDebug_noquote for () {
  fn noquote(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug7noquoteEv()};
    unsafe {_ZN6QDebug7noquoteEv()};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn FreeQDebug<T: QDebug_FreeQDebug>(&mut self, value: T) -> i32 {
    value.FreeQDebug(self);
    return 1;
  }
}

pub trait QDebug_FreeQDebug {
  fn FreeQDebug(self, this: &mut QDebug) -> i32;
}

// proto: void QDebug::FreeQDebug();
impl<'a> /*trait*/ QDebug_FreeQDebug for () {
  fn FreeQDebug(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugD0Ev()};
    unsafe {_ZN6QDebugD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn NewQDebug<T: QDebug_NewQDebug>(value: T) -> QDebug {
    let rsthis = value.NewQDebug();
    return rsthis;
    // return 1;
  }
}

pub trait QDebug_NewQDebug {
  fn NewQDebug(self) -> QDebug;
}

// proto: void QDebug::NewQDebug(const QDebug & o);
impl<'a> /*trait*/ QDebug_NewQDebug for (&'a  QDebug) {
  fn NewQDebug(self) -> QDebug {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QDebugC1ERKS_(qthis, arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn space<T: QDebug_space>(&mut self, value: T) -> i32 {
    value.space(self);
    return 1;
  }
}

pub trait QDebug_space {
  fn space(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::space();
impl<'a> /*trait*/ QDebug_space for () {
  fn space(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5spaceEv()};
    unsafe {_ZN6QDebug5spaceEv()};
    return 1;
  }
}

// proto: void QDebug::NewQDebug(QtMsgType t);
impl<'a> /*trait*/ QDebug_NewQDebug for (i32) {
  fn NewQDebug(self) -> QDebug {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1E9QtMsgType()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QDebugC1E9QtMsgType(qthis, arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn maybeSpace<T: QDebug_maybeSpace>(&mut self, value: T) -> i32 {
    value.maybeSpace(self);
    return 1;
  }
}

pub trait QDebug_maybeSpace {
  fn maybeSpace(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::maybeSpace();
impl<'a> /*trait*/ QDebug_maybeSpace for () {
  fn maybeSpace(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug10maybeSpaceEv()};
    unsafe {_ZN6QDebug10maybeSpaceEv()};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn resetFormat<T: QDebug_resetFormat>(&mut self, value: T) -> i32 {
    value.resetFormat(self);
    return 1;
  }
}

pub trait QDebug_resetFormat {
  fn resetFormat(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::resetFormat();
impl<'a> /*trait*/ QDebug_resetFormat for () {
  fn resetFormat(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug11resetFormatEv()};
    unsafe {_ZN6QDebug11resetFormatEv()};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn setAutoInsertSpaces<T: QDebug_setAutoInsertSpaces>(&mut self, value: T) -> i32 {
    value.setAutoInsertSpaces(self);
    return 1;
  }
}

pub trait QDebug_setAutoInsertSpaces {
  fn setAutoInsertSpaces(self, this: &mut QDebug) -> i32;
}

// proto: void QDebug::setAutoInsertSpaces(bool b);
impl<'a> /*trait*/ QDebug_setAutoInsertSpaces for (i8) {
  fn setAutoInsertSpaces(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug19setAutoInsertSpacesEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QDebug19setAutoInsertSpacesEb(arg0)};
    return 1;
  }
}

// proto: void QDebug::NewQDebug(QString * string);
impl<'a> /*trait*/ QDebug_NewQDebug for (&'a mut QString) {
  fn NewQDebug(self) -> QDebug {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1EP7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QDebugC1EP7QString(qthis, arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn swap<T: QDebug_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QDebug_swap {
  fn swap(self, this: &mut QDebug) -> i32;
}

// proto: void QDebug::swap(QDebug & other);
impl<'a> /*trait*/ QDebug_swap for (&'a mut QDebug) {
  fn swap(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QDebug4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn nospace<T: QDebug_nospace>(&mut self, value: T) -> i32 {
    value.nospace(self);
    return 1;
  }
}

pub trait QDebug_nospace {
  fn nospace(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::nospace();
impl<'a> /*trait*/ QDebug_nospace for () {
  fn nospace(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug7nospaceEv()};
    unsafe {_ZN6QDebug7nospaceEv()};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn autoInsertSpaces<T: QDebug_autoInsertSpaces>(&mut self, value: T) -> i32 {
    value.autoInsertSpaces(self);
    return 1;
  }
}

pub trait QDebug_autoInsertSpaces {
  fn autoInsertSpaces(self, this: &mut QDebug) -> i32;
}

// proto: bool QDebug::autoInsertSpaces();
impl<'a> /*trait*/ QDebug_autoInsertSpaces for () {
  fn autoInsertSpaces(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QDebug16autoInsertSpacesEv()};
    unsafe {_ZNK6QDebug16autoInsertSpacesEv()};
    return 1;
  }
}

// proto: void QDebug::NewQDebug(QIODevice * device);
impl<'a> /*trait*/ QDebug_NewQDebug for (&'a mut QIODevice) {
  fn NewQDebug(self) -> QDebug {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1EP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QDebugC1EP9QIODevice(qthis, arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn quote<T: QDebug_quote>(&mut self, value: T) -> i32 {
    value.quote(self);
    return 1;
  }
}

pub trait QDebug_quote {
  fn quote(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::quote();
impl<'a> /*trait*/ QDebug_quote for () {
  fn quote(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5quoteEv()};
    unsafe {_ZN6QDebug5quoteEv()};
    return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn maybeQuote<T: QDebug_maybeQuote>(&mut self, value: T) -> i32 {
    value.maybeQuote(self);
    return 1;
  }
}

pub trait QDebug_maybeQuote {
  fn maybeQuote(self, this: &mut QDebug) -> i32;
}

// proto: QDebug & QDebug::maybeQuote(char c);
impl<'a> /*trait*/ QDebug_maybeQuote for (i8) {
  fn maybeQuote(self, this: &mut QDebug) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug10maybeQuoteEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN6QDebug10maybeQuoteEc(arg0)};
    return 1;
  }
}

