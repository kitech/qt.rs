// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qdebug.h
// dst-file: /src/core/qdebug.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
// use super::qdebug::QDebug; // 773
use super::qstring::QString; // 773
use super::qiodevice::QIODevice; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QNoDebug & QNoDebug::maybeQuote(const char );
  fn _ZN8QNoDebug10maybeQuoteEc(qthis: *mut c_void, arg0: c_char) -> *mut c_void;
  // proto:  QNoDebug & QNoDebug::quote();
  fn _ZN8QNoDebug5quoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QNoDebug & QNoDebug::space();
  fn _ZN8QNoDebug5spaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QNoDebug & QNoDebug::nospace();
  fn _ZN8QNoDebug7nospaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QNoDebug & QNoDebug::noquote();
  fn _ZN8QNoDebug7noquoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QNoDebug & QNoDebug::maybeSpace();
  fn _ZN8QNoDebug10maybeSpaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
  fn _ZN16QDebugStateSaverC1ER6QDebug(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebugStateSaver::QDebugStateSaver(const QDebugStateSaver & );
  fn _ZN16QDebugStateSaverC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebugStateSaver::~QDebugStateSaver();
  fn _ZN16QDebugStateSaverD0Ev(qthis: *mut c_void);
  // proto:  QDebug & QDebug::noquote();
  fn _ZN6QDebug7noquoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::~QDebug();
  fn _ZN6QDebugD0Ev(qthis: *mut c_void);
  // proto:  void QDebug::QDebug(const QDebug & o);
  fn _ZN6QDebugC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDebug & QDebug::space();
  fn _ZN6QDebug5spaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::QDebug(QtMsgType t);
  fn _ZN6QDebugC1E9QtMsgType(qthis: *mut c_void, arg0: c_int);
  // proto:  QDebug & QDebug::maybeSpace();
  fn _ZN6QDebug10maybeSpaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDebug & QDebug::resetFormat();
  fn _ZN6QDebug11resetFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::setAutoInsertSpaces(bool b);
  fn _ZN6QDebug19setAutoInsertSpacesEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QDebug::QDebug(QString * string);
  fn _ZN6QDebugC1EP7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebug::swap(QDebug & other);
  fn _ZN6QDebug4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDebug & QDebug::nospace();
  fn _ZN6QDebug7nospaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDebug::autoInsertSpaces();
  fn _ZNK6QDebug16autoInsertSpacesEv(qthis: *mut c_void) -> c_char;
  // proto:  void QDebug::QDebug(QIODevice * device);
  fn _ZN6QDebugC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDebug & QDebug::quote();
  fn _ZN6QDebug5quoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDebug & QDebug::maybeQuote(char c);
  fn _ZN6QDebug10maybeQuoteEc(qthis: *mut c_void, arg0: c_char) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QNoDebug)=1
pub struct QNoDebug {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDebugStateSaver)=1
pub struct QDebugStateSaver {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDebug)=8
pub struct QDebug {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QNoDebug {
  pub fn inheritFrom(qthis: *mut c_void) -> QNoDebug {
    return QNoDebug{qclsinst: qthis};
  }
}
  // proto:  QNoDebug & QNoDebug::maybeQuote(const char );
impl /*struct*/ QNoDebug {
  pub fn maybeQuote<RetType, T: QNoDebug_maybeQuote<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maybeQuote(self);
    // return 1;
  }
}

pub trait QNoDebug_maybeQuote<RetType> {
  fn maybeQuote(self , rsthis: &mut QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::maybeQuote(const char );
impl<'a> /*trait*/ QNoDebug_maybeQuote<QNoDebug> for (i8) {
  fn maybeQuote(self , rsthis: &mut QNoDebug) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QNoDebug10maybeQuoteEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN8QNoDebug10maybeQuoteEc(rsthis.qclsinst, arg0)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QNoDebug & QNoDebug::quote();
impl /*struct*/ QNoDebug {
  pub fn quote<RetType, T: QNoDebug_quote<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.quote(self);
    // return 1;
  }
}

pub trait QNoDebug_quote<RetType> {
  fn quote(self , rsthis: &mut QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::quote();
impl<'a> /*trait*/ QNoDebug_quote<QNoDebug> for () {
  fn quote(self , rsthis: &mut QNoDebug) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QNoDebug5quoteEv()};
    let mut ret = unsafe {_ZN8QNoDebug5quoteEv(rsthis.qclsinst)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QNoDebug & QNoDebug::space();
impl /*struct*/ QNoDebug {
  pub fn space<RetType, T: QNoDebug_space<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.space(self);
    // return 1;
  }
}

pub trait QNoDebug_space<RetType> {
  fn space(self , rsthis: &mut QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::space();
impl<'a> /*trait*/ QNoDebug_space<QNoDebug> for () {
  fn space(self , rsthis: &mut QNoDebug) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QNoDebug5spaceEv()};
    let mut ret = unsafe {_ZN8QNoDebug5spaceEv(rsthis.qclsinst)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QNoDebug & QNoDebug::nospace();
impl /*struct*/ QNoDebug {
  pub fn nospace<RetType, T: QNoDebug_nospace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nospace(self);
    // return 1;
  }
}

pub trait QNoDebug_nospace<RetType> {
  fn nospace(self , rsthis: &mut QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::nospace();
impl<'a> /*trait*/ QNoDebug_nospace<QNoDebug> for () {
  fn nospace(self , rsthis: &mut QNoDebug) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QNoDebug7nospaceEv()};
    let mut ret = unsafe {_ZN8QNoDebug7nospaceEv(rsthis.qclsinst)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QNoDebug & QNoDebug::noquote();
impl /*struct*/ QNoDebug {
  pub fn noquote<RetType, T: QNoDebug_noquote<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.noquote(self);
    // return 1;
  }
}

pub trait QNoDebug_noquote<RetType> {
  fn noquote(self , rsthis: &mut QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::noquote();
impl<'a> /*trait*/ QNoDebug_noquote<QNoDebug> for () {
  fn noquote(self , rsthis: &mut QNoDebug) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QNoDebug7noquoteEv()};
    let mut ret = unsafe {_ZN8QNoDebug7noquoteEv(rsthis.qclsinst)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QNoDebug & QNoDebug::maybeSpace();
impl /*struct*/ QNoDebug {
  pub fn maybeSpace<RetType, T: QNoDebug_maybeSpace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maybeSpace(self);
    // return 1;
  }
}

pub trait QNoDebug_maybeSpace<RetType> {
  fn maybeSpace(self , rsthis: &mut QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::maybeSpace();
impl<'a> /*trait*/ QNoDebug_maybeSpace<QNoDebug> for () {
  fn maybeSpace(self , rsthis: &mut QNoDebug) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QNoDebug10maybeSpaceEv()};
    let mut ret = unsafe {_ZN8QNoDebug10maybeSpaceEv(rsthis.qclsinst)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDebugStateSaver {
  pub fn inheritFrom(qthis: *mut c_void) -> QDebugStateSaver {
    return QDebugStateSaver{qclsinst: qthis};
  }
}
  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
impl /*struct*/ QDebugStateSaver {
  pub fn NewQDebugStateSaver<T: QDebugStateSaver_NewQDebugStateSaver>(value: T) -> QDebugStateSaver {
    let rsthis = value.NewQDebugStateSaver();
    return rsthis;
    // return 1;
  }
}

pub trait QDebugStateSaver_NewQDebugStateSaver {
  fn NewQDebugStateSaver(self) -> QDebugStateSaver;
}

  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
impl<'a> /*trait*/ QDebugStateSaver_NewQDebugStateSaver for (QDebug) {
  fn NewQDebugStateSaver(self) -> QDebugStateSaver {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ER6QDebug()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDebugStateSaverC1ER6QDebug(qthis, arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebugStateSaver::QDebugStateSaver(const QDebugStateSaver & );
impl<'a> /*trait*/ QDebugStateSaver_NewQDebugStateSaver for (QDebugStateSaver) {
  fn NewQDebugStateSaver(self) -> QDebugStateSaver {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDebugStateSaverC1ERKS_(qthis, arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebugStateSaver::~QDebugStateSaver();
impl /*struct*/ QDebugStateSaver {
  pub fn FreeQDebugStateSaver<RetType, T: QDebugStateSaver_FreeQDebugStateSaver<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDebugStateSaver(self);
    // return 1;
  }
}

pub trait QDebugStateSaver_FreeQDebugStateSaver<RetType> {
  fn FreeQDebugStateSaver(self , rsthis: &mut QDebugStateSaver) -> RetType;
}

  // proto:  void QDebugStateSaver::~QDebugStateSaver();
impl<'a> /*trait*/ QDebugStateSaver_FreeQDebugStateSaver<()> for () {
  fn FreeQDebugStateSaver(self , rsthis: &mut QDebugStateSaver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverD0Ev()};
     unsafe {_ZN16QDebugStateSaverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDebug {
  pub fn inheritFrom(qthis: *mut c_void) -> QDebug {
    return QDebug{qclsinst: qthis};
  }
}
  // proto:  QDebug & QDebug::noquote();
impl /*struct*/ QDebug {
  pub fn noquote<RetType, T: QDebug_noquote<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.noquote(self);
    // return 1;
  }
}

pub trait QDebug_noquote<RetType> {
  fn noquote(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::noquote();
impl<'a> /*trait*/ QDebug_noquote<QDebug> for () {
  fn noquote(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug7noquoteEv()};
    let mut ret = unsafe {_ZN6QDebug7noquoteEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDebug::~QDebug();
impl /*struct*/ QDebug {
  pub fn FreeQDebug<RetType, T: QDebug_FreeQDebug<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDebug(self);
    // return 1;
  }
}

pub trait QDebug_FreeQDebug<RetType> {
  fn FreeQDebug(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  void QDebug::~QDebug();
impl<'a> /*trait*/ QDebug_FreeQDebug<()> for () {
  fn FreeQDebug(self , rsthis: &mut QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugD0Ev()};
     unsafe {_ZN6QDebugD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(const QDebug & o);
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

  // proto:  void QDebug::QDebug(const QDebug & o);
impl<'a> /*trait*/ QDebug_NewQDebug for (QDebug) {
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

  // proto:  QDebug & QDebug::space();
impl /*struct*/ QDebug {
  pub fn space<RetType, T: QDebug_space<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.space(self);
    // return 1;
  }
}

pub trait QDebug_space<RetType> {
  fn space(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::space();
impl<'a> /*trait*/ QDebug_space<QDebug> for () {
  fn space(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5spaceEv()};
    let mut ret = unsafe {_ZN6QDebug5spaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(QtMsgType t);
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

  // proto:  QDebug & QDebug::maybeSpace();
impl /*struct*/ QDebug {
  pub fn maybeSpace<RetType, T: QDebug_maybeSpace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maybeSpace(self);
    // return 1;
  }
}

pub trait QDebug_maybeSpace<RetType> {
  fn maybeSpace(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::maybeSpace();
impl<'a> /*trait*/ QDebug_maybeSpace<QDebug> for () {
  fn maybeSpace(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug10maybeSpaceEv()};
    let mut ret = unsafe {_ZN6QDebug10maybeSpaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QDebug & QDebug::resetFormat();
impl /*struct*/ QDebug {
  pub fn resetFormat<RetType, T: QDebug_resetFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resetFormat(self);
    // return 1;
  }
}

pub trait QDebug_resetFormat<RetType> {
  fn resetFormat(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::resetFormat();
impl<'a> /*trait*/ QDebug_resetFormat<QDebug> for () {
  fn resetFormat(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug11resetFormatEv()};
    let mut ret = unsafe {_ZN6QDebug11resetFormatEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDebug::setAutoInsertSpaces(bool b);
impl /*struct*/ QDebug {
  pub fn setAutoInsertSpaces<RetType, T: QDebug_setAutoInsertSpaces<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoInsertSpaces(self);
    // return 1;
  }
}

pub trait QDebug_setAutoInsertSpaces<RetType> {
  fn setAutoInsertSpaces(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  void QDebug::setAutoInsertSpaces(bool b);
impl<'a> /*trait*/ QDebug_setAutoInsertSpaces<()> for (i8) {
  fn setAutoInsertSpaces(self , rsthis: &mut QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug19setAutoInsertSpacesEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN6QDebug19setAutoInsertSpacesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(QString * string);
impl<'a> /*trait*/ QDebug_NewQDebug for (QString) {
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

  // proto:  void QDebug::swap(QDebug & other);
impl /*struct*/ QDebug {
  pub fn swap<RetType, T: QDebug_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDebug_swap<RetType> {
  fn swap(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  void QDebug::swap(QDebug & other);
impl<'a> /*trait*/ QDebug_swap<()> for (QDebug) {
  fn swap(self , rsthis: &mut QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QDebug4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDebug & QDebug::nospace();
impl /*struct*/ QDebug {
  pub fn nospace<RetType, T: QDebug_nospace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nospace(self);
    // return 1;
  }
}

pub trait QDebug_nospace<RetType> {
  fn nospace(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::nospace();
impl<'a> /*trait*/ QDebug_nospace<QDebug> for () {
  fn nospace(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug7nospaceEv()};
    let mut ret = unsafe {_ZN6QDebug7nospaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDebug::autoInsertSpaces();
impl /*struct*/ QDebug {
  pub fn autoInsertSpaces<RetType, T: QDebug_autoInsertSpaces<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoInsertSpaces(self);
    // return 1;
  }
}

pub trait QDebug_autoInsertSpaces<RetType> {
  fn autoInsertSpaces(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  bool QDebug::autoInsertSpaces();
impl<'a> /*trait*/ QDebug_autoInsertSpaces<i8> for () {
  fn autoInsertSpaces(self , rsthis: &mut QDebug) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QDebug16autoInsertSpacesEv()};
    let mut ret = unsafe {_ZNK6QDebug16autoInsertSpacesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(QIODevice * device);
impl<'a> /*trait*/ QDebug_NewQDebug for (QIODevice) {
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

  // proto:  QDebug & QDebug::quote();
impl /*struct*/ QDebug {
  pub fn quote<RetType, T: QDebug_quote<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.quote(self);
    // return 1;
  }
}

pub trait QDebug_quote<RetType> {
  fn quote(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::quote();
impl<'a> /*trait*/ QDebug_quote<QDebug> for () {
  fn quote(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5quoteEv()};
    let mut ret = unsafe {_ZN6QDebug5quoteEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QDebug & QDebug::maybeQuote(char c);
impl /*struct*/ QDebug {
  pub fn maybeQuote<RetType, T: QDebug_maybeQuote<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maybeQuote(self);
    // return 1;
  }
}

pub trait QDebug_maybeQuote<RetType> {
  fn maybeQuote(self , rsthis: &mut QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::maybeQuote(char c);
impl<'a> /*trait*/ QDebug_maybeQuote<QDebug> for (i8) {
  fn maybeQuote(self , rsthis: &mut QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug10maybeQuoteEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN6QDebug10maybeQuoteEc(rsthis.qclsinst, arg0)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

