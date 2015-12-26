// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qpalette.h
// dst-file: /src/gui/qpalette.rs
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
use super::qcolor::QColor; // 773
use super::qbrush::QBrush; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPalette_Class_Size() -> c_int;
  // proto:  void QPalette::QPalette(const QColor & windowText, const QColor & window, const QColor & light, const QColor & dark, const QColor & mid, const QColor & text, const QColor & base);
  fn dector_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void);
  // proto:  void QPalette::~QPalette();
  fn _ZN8QPaletteD0Ev(qthis: *mut c_void);
  // proto:  void QPalette::QPalette();
  fn dector_ZN8QPaletteC1Ev() -> *mut c_void;
  fn _ZN8QPaletteC1Ev(qthis: *mut c_void);
  // proto:  void QPalette::QPalette(const QColor & button);
  fn dector_ZN8QPaletteC1ERK6QColor(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPalette::isCopyOf(const QPalette & p);
  fn _ZNK8QPalette8isCopyOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPalette::QPalette(const QColor & button, const QColor & window);
  fn dector_ZN8QPaletteC1ERK6QColorS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QColorS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QPalette QPalette::resolve(const QPalette & );
  fn _ZNK8QPalette7resolveERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qint64 QPalette::cacheKey();
  fn _ZNK8QPalette8cacheKeyEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QPalette::QPalette(const QPalette & palette);
  fn dector_ZN8QPaletteC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPalette::QPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
  fn dector_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPalette)=16
pub struct QPalette {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPalette {
  pub fn inheritFrom(qthis: *mut c_void) -> QPalette {
    return QPalette{qclsinst: qthis};
  }
}
  // proto:  void QPalette::QPalette(const QColor & windowText, const QColor & window, const QColor & light, const QColor & dark, const QColor & mid, const QColor & text, const QColor & base);
impl /*struct*/ QPalette {
  pub fn New<T: QPalette_New>(value: T) -> QPalette {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_New {
  fn New(self) -> QPalette;
}

  // proto:  void QPalette::QPalette(const QColor & windowText, const QColor & window, const QColor & light, const QColor & dark, const QColor & mid, const QColor & text, const QColor & base);
impl<'a> /*trait*/ QPalette_New for (&'a QColor, &'a QColor, &'a QColor, &'a QColor, &'a QColor, &'a QColor, &'a QColor) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPalette::~QPalette();
impl /*struct*/ QPalette {
  pub fn Free<RetType, T: QPalette_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPalette_Free<RetType> {
  fn Free(self , rsthis: & QPalette) -> RetType;
}

  // proto:  void QPalette::~QPalette();
impl<'a> /*trait*/ QPalette_Free<()> for () {
  fn Free(self , rsthis: & QPalette) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteD0Ev()};
     unsafe {_ZN8QPaletteD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPalette::QPalette();
impl<'a> /*trait*/ QPalette_New for () {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1Ev()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN8QPaletteC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPaletteC1Ev()};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QColor & button);
impl<'a> /*trait*/ QPalette_New for (&'a QColor) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColor()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QColor(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPaletteC1ERK6QColor(arg0)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPalette::isCopyOf(const QPalette & p);
impl /*struct*/ QPalette {
  pub fn isCopyOf<RetType, T: QPalette_isCopyOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCopyOf(self);
    // return 1;
  }
}

pub trait QPalette_isCopyOf<RetType> {
  fn isCopyOf(self , rsthis: & QPalette) -> RetType;
}

  // proto:  bool QPalette::isCopyOf(const QPalette & p);
impl<'a> /*trait*/ QPalette_isCopyOf<i8> for (&'a QPalette) {
  fn isCopyOf(self , rsthis: & QPalette) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPalette8isCopyOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QColor & button, const QColor & window);
impl<'a> /*trait*/ QPalette_New for (&'a QColor, &'a QColor) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColorS2_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QColorS2_(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPaletteC1ERK6QColorS2_(arg0, arg1)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPalette QPalette::resolve(const QPalette & );
impl /*struct*/ QPalette {
  pub fn resolve<RetType, T: QPalette_resolve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolve(self);
    // return 1;
  }
}

pub trait QPalette_resolve<RetType> {
  fn resolve(self , rsthis: & QPalette) -> RetType;
}

  // proto:  QPalette QPalette::resolve(const QPalette & );
impl<'a> /*trait*/ QPalette_resolve<QPalette> for (&'a QPalette) {
  fn resolve(self , rsthis: & QPalette) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPalette7resolveERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPalette::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QPalette::cacheKey();
impl /*struct*/ QPalette {
  pub fn cacheKey<RetType, T: QPalette_cacheKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QPalette_cacheKey<RetType> {
  fn cacheKey(self , rsthis: & QPalette) -> RetType;
}

  // proto:  qint64 QPalette::cacheKey();
impl<'a> /*trait*/ QPalette_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: & QPalette) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8cacheKeyEv()};
    let mut ret = unsafe {_ZNK8QPalette8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QPalette & palette);
impl<'a> /*trait*/ QPalette_New for (&'a QPalette) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERKS_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPaletteC1ERKS_(arg0)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
impl<'a> /*trait*/ QPalette_New for (&'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    let arg7 = self.7.qclsinst  as *mut c_void;
    let arg8 = self.8.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

