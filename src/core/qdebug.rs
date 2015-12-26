// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNoDebug_Class_Size() -> c_int;
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
  fn QDebugStateSaver_Class_Size() -> c_int;
  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
  fn dector_ZN16QDebugStateSaverC1ER6QDebug(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QDebugStateSaverC1ER6QDebug(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebugStateSaver::QDebugStateSaver(const QDebugStateSaver & );
  fn dector_ZN16QDebugStateSaverC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QDebugStateSaverC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebugStateSaver::~QDebugStateSaver();
  fn _ZN16QDebugStateSaverD0Ev(qthis: *mut c_void);
  fn QDebug_Class_Size() -> c_int;
  // proto:  QDebug & QDebug::noquote();
  fn _ZN6QDebug7noquoteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::~QDebug();
  fn _ZN6QDebugD0Ev(qthis: *mut c_void);
  // proto:  void QDebug::QDebug(const QDebug & o);
  fn dector_ZN6QDebugC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QDebugC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDebug & QDebug::space();
  fn _ZN6QDebug5spaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::QDebug(QtMsgType t);
  fn dector_ZN6QDebugC1E9QtMsgType(arg0: c_int) -> *mut c_void;
  fn _ZN6QDebugC1E9QtMsgType(qthis: *mut c_void, arg0: c_int);
  // proto:  QDebug & QDebug::maybeSpace();
  fn _ZN6QDebug10maybeSpaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDebug & QDebug::resetFormat();
  fn _ZN6QDebug11resetFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDebug::setAutoInsertSpaces(bool b);
  fn _ZN6QDebug19setAutoInsertSpacesEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QDebug::QDebug(QString * string);
  fn dector_ZN6QDebugC1EP7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QDebugC1EP7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDebug::swap(QDebug & other);
  fn _ZN6QDebug4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDebug & QDebug::nospace();
  fn _ZN6QDebug7nospaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDebug::autoInsertSpaces();
  fn _ZNK6QDebug16autoInsertSpacesEv(qthis: *mut c_void) -> c_char;
  // proto:  void QDebug::QDebug(QIODevice * device);
  fn dector_ZN6QDebugC1EP9QIODevice(arg0: *mut c_void) -> *mut c_void;
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
  pub fn maybeQuote<RetType, T: QNoDebug_maybeQuote<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maybeQuote(self);
    // return 1;
  }
}

pub trait QNoDebug_maybeQuote<RetType> {
  fn maybeQuote(self , rsthis: & QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::maybeQuote(const char );
impl<'a> /*trait*/ QNoDebug_maybeQuote<QNoDebug> for (i8) {
  fn maybeQuote(self , rsthis: & QNoDebug) -> QNoDebug {
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
  pub fn quote<RetType, T: QNoDebug_quote<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quote(self);
    // return 1;
  }
}

pub trait QNoDebug_quote<RetType> {
  fn quote(self , rsthis: & QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::quote();
impl<'a> /*trait*/ QNoDebug_quote<QNoDebug> for () {
  fn quote(self , rsthis: & QNoDebug) -> QNoDebug {
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
  pub fn space<RetType, T: QNoDebug_space<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.space(self);
    // return 1;
  }
}

pub trait QNoDebug_space<RetType> {
  fn space(self , rsthis: & QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::space();
impl<'a> /*trait*/ QNoDebug_space<QNoDebug> for () {
  fn space(self , rsthis: & QNoDebug) -> QNoDebug {
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
  pub fn nospace<RetType, T: QNoDebug_nospace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nospace(self);
    // return 1;
  }
}

pub trait QNoDebug_nospace<RetType> {
  fn nospace(self , rsthis: & QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::nospace();
impl<'a> /*trait*/ QNoDebug_nospace<QNoDebug> for () {
  fn nospace(self , rsthis: & QNoDebug) -> QNoDebug {
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
  pub fn noquote<RetType, T: QNoDebug_noquote<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.noquote(self);
    // return 1;
  }
}

pub trait QNoDebug_noquote<RetType> {
  fn noquote(self , rsthis: & QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::noquote();
impl<'a> /*trait*/ QNoDebug_noquote<QNoDebug> for () {
  fn noquote(self , rsthis: & QNoDebug) -> QNoDebug {
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
  pub fn maybeSpace<RetType, T: QNoDebug_maybeSpace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maybeSpace(self);
    // return 1;
  }
}

pub trait QNoDebug_maybeSpace<RetType> {
  fn maybeSpace(self , rsthis: & QNoDebug) -> RetType;
}

  // proto:  QNoDebug & QNoDebug::maybeSpace();
impl<'a> /*trait*/ QNoDebug_maybeSpace<QNoDebug> for () {
  fn maybeSpace(self , rsthis: & QNoDebug) -> QNoDebug {
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
  pub fn New<T: QDebugStateSaver_New>(value: T) -> QDebugStateSaver {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDebugStateSaver_New {
  fn New(self) -> QDebugStateSaver;
}

  // proto:  void QDebugStateSaver::QDebugStateSaver(QDebug & dbg);
impl<'a> /*trait*/ QDebugStateSaver_New for (&'a QDebug) {
  fn New(self) -> QDebugStateSaver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ER6QDebug()};
    let ctysz: c_int = unsafe{QDebugStateSaver_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QDebugStateSaverC1ER6QDebug(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QDebugStateSaverC1ER6QDebug(arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebugStateSaver::QDebugStateSaver(const QDebugStateSaver & );
impl<'a> /*trait*/ QDebugStateSaver_New for (&'a QDebugStateSaver) {
  fn New(self) -> QDebugStateSaver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDebugStateSaverC1ERKS_()};
    let ctysz: c_int = unsafe{QDebugStateSaver_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QDebugStateSaverC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QDebugStateSaverC1ERKS_(arg0)};
    let rsthis = QDebugStateSaver{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebugStateSaver::~QDebugStateSaver();
impl /*struct*/ QDebugStateSaver {
  pub fn Free<RetType, T: QDebugStateSaver_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDebugStateSaver_Free<RetType> {
  fn Free(self , rsthis: & QDebugStateSaver) -> RetType;
}

  // proto:  void QDebugStateSaver::~QDebugStateSaver();
impl<'a> /*trait*/ QDebugStateSaver_Free<()> for () {
  fn Free(self , rsthis: & QDebugStateSaver) -> () {
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
  pub fn noquote<RetType, T: QDebug_noquote<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.noquote(self);
    // return 1;
  }
}

pub trait QDebug_noquote<RetType> {
  fn noquote(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::noquote();
impl<'a> /*trait*/ QDebug_noquote<QDebug> for () {
  fn noquote(self , rsthis: & QDebug) -> QDebug {
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
  pub fn Free<RetType, T: QDebug_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDebug_Free<RetType> {
  fn Free(self , rsthis: & QDebug) -> RetType;
}

  // proto:  void QDebug::~QDebug();
impl<'a> /*trait*/ QDebug_Free<()> for () {
  fn Free(self , rsthis: & QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugD0Ev()};
     unsafe {_ZN6QDebugD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(const QDebug & o);
impl /*struct*/ QDebug {
  pub fn New<T: QDebug_New>(value: T) -> QDebug {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDebug_New {
  fn New(self) -> QDebug;
}

  // proto:  void QDebug::QDebug(const QDebug & o);
impl<'a> /*trait*/ QDebug_New for (&'a QDebug) {
  fn New(self) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1ERKS_()};
    let ctysz: c_int = unsafe{QDebug_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QDebugC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QDebugC1ERKS_(arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDebug & QDebug::space();
impl /*struct*/ QDebug {
  pub fn space<RetType, T: QDebug_space<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.space(self);
    // return 1;
  }
}

pub trait QDebug_space<RetType> {
  fn space(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::space();
impl<'a> /*trait*/ QDebug_space<QDebug> for () {
  fn space(self , rsthis: & QDebug) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug5spaceEv()};
    let mut ret = unsafe {_ZN6QDebug5spaceEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(QtMsgType t);
impl<'a> /*trait*/ QDebug_New for (i32) {
  fn New(self) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1E9QtMsgType()};
    let ctysz: c_int = unsafe{QDebug_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN6QDebugC1E9QtMsgType(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QDebugC1E9QtMsgType(arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDebug & QDebug::maybeSpace();
impl /*struct*/ QDebug {
  pub fn maybeSpace<RetType, T: QDebug_maybeSpace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maybeSpace(self);
    // return 1;
  }
}

pub trait QDebug_maybeSpace<RetType> {
  fn maybeSpace(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::maybeSpace();
impl<'a> /*trait*/ QDebug_maybeSpace<QDebug> for () {
  fn maybeSpace(self , rsthis: & QDebug) -> QDebug {
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
  pub fn resetFormat<RetType, T: QDebug_resetFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetFormat(self);
    // return 1;
  }
}

pub trait QDebug_resetFormat<RetType> {
  fn resetFormat(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::resetFormat();
impl<'a> /*trait*/ QDebug_resetFormat<QDebug> for () {
  fn resetFormat(self , rsthis: & QDebug) -> QDebug {
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
  pub fn setAutoInsertSpaces<RetType, T: QDebug_setAutoInsertSpaces<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoInsertSpaces(self);
    // return 1;
  }
}

pub trait QDebug_setAutoInsertSpaces<RetType> {
  fn setAutoInsertSpaces(self , rsthis: & QDebug) -> RetType;
}

  // proto:  void QDebug::setAutoInsertSpaces(bool b);
impl<'a> /*trait*/ QDebug_setAutoInsertSpaces<()> for (i8) {
  fn setAutoInsertSpaces(self , rsthis: & QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug19setAutoInsertSpacesEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN6QDebug19setAutoInsertSpacesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(QString * string);
impl<'a> /*trait*/ QDebug_New for (&'a QString) {
  fn New(self) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1EP7QString()};
    let ctysz: c_int = unsafe{QDebug_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QDebugC1EP7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QDebugC1EP7QString(arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDebug::swap(QDebug & other);
impl /*struct*/ QDebug {
  pub fn swap<RetType, T: QDebug_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QDebug_swap<RetType> {
  fn swap(self , rsthis: & QDebug) -> RetType;
}

  // proto:  void QDebug::swap(QDebug & other);
impl<'a> /*trait*/ QDebug_swap<()> for (&'a QDebug) {
  fn swap(self , rsthis: & QDebug) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebug4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QDebug4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDebug & QDebug::nospace();
impl /*struct*/ QDebug {
  pub fn nospace<RetType, T: QDebug_nospace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nospace(self);
    // return 1;
  }
}

pub trait QDebug_nospace<RetType> {
  fn nospace(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::nospace();
impl<'a> /*trait*/ QDebug_nospace<QDebug> for () {
  fn nospace(self , rsthis: & QDebug) -> QDebug {
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
  pub fn autoInsertSpaces<RetType, T: QDebug_autoInsertSpaces<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoInsertSpaces(self);
    // return 1;
  }
}

pub trait QDebug_autoInsertSpaces<RetType> {
  fn autoInsertSpaces(self , rsthis: & QDebug) -> RetType;
}

  // proto:  bool QDebug::autoInsertSpaces();
impl<'a> /*trait*/ QDebug_autoInsertSpaces<i8> for () {
  fn autoInsertSpaces(self , rsthis: & QDebug) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QDebug16autoInsertSpacesEv()};
    let mut ret = unsafe {_ZNK6QDebug16autoInsertSpacesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDebug::QDebug(QIODevice * device);
impl<'a> /*trait*/ QDebug_New for (&'a QIODevice) {
  fn New(self) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QDebugC1EP9QIODevice()};
    let ctysz: c_int = unsafe{QDebug_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QDebugC1EP9QIODevice(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QDebugC1EP9QIODevice(arg0)};
    let rsthis = QDebug{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDebug & QDebug::quote();
impl /*struct*/ QDebug {
  pub fn quote<RetType, T: QDebug_quote<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quote(self);
    // return 1;
  }
}

pub trait QDebug_quote<RetType> {
  fn quote(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::quote();
impl<'a> /*trait*/ QDebug_quote<QDebug> for () {
  fn quote(self , rsthis: & QDebug) -> QDebug {
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
  pub fn maybeQuote<RetType, T: QDebug_maybeQuote<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maybeQuote(self);
    // return 1;
  }
}

pub trait QDebug_maybeQuote<RetType> {
  fn maybeQuote(self , rsthis: & QDebug) -> RetType;
}

  // proto:  QDebug & QDebug::maybeQuote(char c);
impl<'a> /*trait*/ QDebug_maybeQuote<QDebug> for (i8) {
  fn maybeQuote(self , rsthis: & QDebug) -> QDebug {
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

