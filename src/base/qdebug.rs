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
  // proto:  QDebug & QDebug::noquote();
  fn _ZN6QDebug7noquoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::FreeQDebug();
  fn _ZN6QDebugD0Ev(qthis: *mut c_void) ;
  // proto:  void QDebug::NewQDebug(const QDebug & o);
  fn _ZN6QDebugC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDebug & QDebug::space();
  fn _ZN6QDebug5spaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::NewQDebug(QtMsgType t);
  fn _ZN6QDebugC1E9QtMsgType(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QDebug & QDebug::maybeSpace();
  fn _ZN6QDebug10maybeSpaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDebug & QDebug::resetFormat();
  fn _ZN6QDebug11resetFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::setAutoInsertSpaces(bool b);
  fn _ZN6QDebug19setAutoInsertSpacesEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QDebug::NewQDebug(QString * string);
  fn _ZN6QDebugC1EP7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDebug::swap(QDebug & other);
  fn _ZN6QDebug4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDebug & QDebug::nospace();
  fn _ZN6QDebug7nospaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDebug::autoInsertSpaces();
  fn _ZNK6QDebug16autoInsertSpacesEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDebug::NewQDebug(QIODevice * device);
  fn _ZN6QDebugC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDebug & QDebug::quote();
  fn _ZN6QDebug5quoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDebug & QDebug::maybeQuote(char c);
  fn _ZN6QDebug10maybeQuoteEc(qthis: *mut c_void, arg0: c_char) -> *mut c_void;
}

// body block begin
// class sizeof(QDebug)=8
pub struct QDebug {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDebug {
  pub fn noquote<T: QDebug_noquote>(&mut self, value: T) -> QDebug {
    return value.noquote(self);
    // return 1;
  }
}

pub trait QDebug_noquote {
  fn noquote(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::noquote();
impl<'a> /*trait*/ QDebug_noquote for () {
  fn noquote(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug7noquoteEv()};
    let mut ret = unsafe {_ZN6QDebug7noquoteEv(rsthis.qclsinst)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn FreeQDebug<T: QDebug_FreeQDebug>(&mut self, value: T)  {
     value.FreeQDebug(self);
    // return 1;
  }
}

pub trait QDebug_FreeQDebug {
  fn FreeQDebug(self, rsthis: &mut QDebug) ;
}

// proto:  void QDebug::FreeQDebug();
impl<'a> /*trait*/ QDebug_FreeQDebug for () {
  fn FreeQDebug(self, rsthis: &mut QDebug)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugD0Ev()};
     unsafe {_ZN6QDebugD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QDebugC1ERKS_(qthis, arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn space<T: QDebug_space>(&mut self, value: T) -> QDebug {
    return value.space(self);
    // return 1;
  }
}

pub trait QDebug_space {
  fn space(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::space();
impl<'a> /*trait*/ QDebug_space for () {
  fn space(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5spaceEv()};
    let mut ret = unsafe {_ZN6QDebug5spaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn maybeSpace<T: QDebug_maybeSpace>(&mut self, value: T) -> QDebug {
    return value.maybeSpace(self);
    // return 1;
  }
}

pub trait QDebug_maybeSpace {
  fn maybeSpace(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::maybeSpace();
impl<'a> /*trait*/ QDebug_maybeSpace for () {
  fn maybeSpace(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug10maybeSpaceEv()};
    let mut ret = unsafe {_ZN6QDebug10maybeSpaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn resetFormat<T: QDebug_resetFormat>(&mut self, value: T) -> QDebug {
    return value.resetFormat(self);
    // return 1;
  }
}

pub trait QDebug_resetFormat {
  fn resetFormat(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::resetFormat();
impl<'a> /*trait*/ QDebug_resetFormat for () {
  fn resetFormat(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug11resetFormatEv()};
    let mut ret = unsafe {_ZN6QDebug11resetFormatEv(rsthis.qclsinst)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn setAutoInsertSpaces<T: QDebug_setAutoInsertSpaces>(&mut self, value: T)  {
     value.setAutoInsertSpaces(self);
    // return 1;
  }
}

pub trait QDebug_setAutoInsertSpaces {
  fn setAutoInsertSpaces(self, rsthis: &mut QDebug) ;
}

// proto:  void QDebug::setAutoInsertSpaces(bool b);
impl<'a> /*trait*/ QDebug_setAutoInsertSpaces for (i8) {
  fn setAutoInsertSpaces(self, rsthis: &mut QDebug)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug19setAutoInsertSpacesEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QDebug19setAutoInsertSpacesEb(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn swap<T: QDebug_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QDebug_swap {
  fn swap(self, rsthis: &mut QDebug) ;
}

// proto:  void QDebug::swap(QDebug & other);
impl<'a> /*trait*/ QDebug_swap for (&'a mut QDebug) {
  fn swap(self, rsthis: &mut QDebug)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QDebug4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn nospace<T: QDebug_nospace>(&mut self, value: T) -> QDebug {
    return value.nospace(self);
    // return 1;
  }
}

pub trait QDebug_nospace {
  fn nospace(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::nospace();
impl<'a> /*trait*/ QDebug_nospace for () {
  fn nospace(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug7nospaceEv()};
    let mut ret = unsafe {_ZN6QDebug7nospaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn autoInsertSpaces<T: QDebug_autoInsertSpaces>(&mut self, value: T) -> i8 {
    return value.autoInsertSpaces(self);
    // return 1;
  }
}

pub trait QDebug_autoInsertSpaces {
  fn autoInsertSpaces(self, rsthis: &mut QDebug) -> i8;
}

// proto:  bool QDebug::autoInsertSpaces();
impl<'a> /*trait*/ QDebug_autoInsertSpaces for () {
  fn autoInsertSpaces(self, rsthis: &mut QDebug) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QDebug16autoInsertSpacesEv()};
    let mut ret = unsafe {_ZNK6QDebug16autoInsertSpacesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn quote<T: QDebug_quote>(&mut self, value: T) -> QDebug {
    return value.quote(self);
    // return 1;
  }
}

pub trait QDebug_quote {
  fn quote(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::quote();
impl<'a> /*trait*/ QDebug_quote for () {
  fn quote(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5quoteEv()};
    let mut ret = unsafe {_ZN6QDebug5quoteEv(rsthis.qclsinst)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn maybeQuote<T: QDebug_maybeQuote>(&mut self, value: T) -> QDebug {
    return value.maybeQuote(self);
    // return 1;
  }
}

pub trait QDebug_maybeQuote {
  fn maybeQuote(self, rsthis: &mut QDebug) -> QDebug;
}

// proto:  QDebug & QDebug::maybeQuote(char c);
impl<'a> /*trait*/ QDebug_maybeQuote for (i8) {
  fn maybeQuote(self, rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug10maybeQuoteEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN6QDebug10maybeQuoteEc(rsthis.qclsinst, arg0)};
    let mut ret1 = QDebug{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

