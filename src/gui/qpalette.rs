// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
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
  fn _ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void);
  // proto:  void QPalette::~QPalette();
  fn _ZN8QPaletteD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QBrush & QPalette::button();
  fn demth_ZNK8QPalette6buttonEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::foreground();
  fn demth_ZNK8QPalette10foregroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::background();
  fn demth_ZNK8QPalette10backgroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPalette::resolve(uint mask);
  fn demth_ZN8QPalette7resolveEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QPalette::QPalette();
  fn dector_ZN8QPaletteC1Ev() -> *mut c_void;
  fn _ZN8QPaletteC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPalette::QPalette(const QColor & button);
  fn dector_ZN8QPaletteC1ERK6QColor(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPalette::isCopyOf(const QPalette & p);
  fn _ZNK8QPalette8isCopyOfERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QPalette::swap(QPalette & other);
  fn _ZN8QPalette4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  uint QPalette::resolve();
  fn demth_ZNK8QPalette7resolveEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  const QBrush & QPalette::window();
  fn demth_ZNK8QPalette6windowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::highlightedText();
  fn demth_ZNK8QPalette15highlightedTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPalette::QPalette(const QColor & button, const QColor & window);
  fn dector_ZN8QPaletteC1ERK6QColorS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QColorS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QBrush & QPalette::text();
  fn demth_ZNK8QPalette4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::light();
  fn demth_ZNK8QPalette5lightEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPalette QPalette::resolve(const QPalette & );
  fn _ZNK8QPalette7resolveERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::link();
  fn demth_ZNK8QPalette4linkEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QPalette::cacheKey();
  fn _ZNK8QPalette8cacheKeyEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  const QBrush & QPalette::base();
  fn demth_ZNK8QPalette4baseEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::dark();
  fn demth_ZNK8QPalette4darkEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::highlight();
  fn demth_ZNK8QPalette9highlightEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::mid();
  fn demth_ZNK8QPalette3midEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPalette::QPalette(const QPalette & palette);
  fn dector_ZN8QPaletteC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QBrush & QPalette::shadow();
  fn demth_ZNK8QPalette6shadowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::buttonText();
  fn demth_ZNK8QPalette10buttonTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::toolTipBase();
  fn demth_ZNK8QPalette11toolTipBaseEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::midlight();
  fn demth_ZNK8QPalette8midlightEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::brightText();
  fn demth_ZNK8QPalette10brightTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::linkVisited();
  fn demth_ZNK8QPalette11linkVisitedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::alternateBase();
  fn demth_ZNK8QPalette13alternateBaseEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPalette::QPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
  fn dector_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void) -> *mut c_void;
  fn _ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void);
  // proto:  const QBrush & QPalette::windowText();
  fn demth_ZNK8QPalette10windowTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QBrush & QPalette::toolTipText();
  fn demth_ZNK8QPalette11toolTipTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPalette)=16
#[derive(Default)]
pub struct QPalette {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPalette {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPalette {
    return QPalette{qclsinst: qthis, ..Default::default()};
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
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let qthis: u64 = unsafe {dector_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(arg0, arg1, arg2, arg3, arg4, arg5, arg6)} as u64;
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
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

  // proto:  const QBrush & QPalette::button();
impl /*struct*/ QPalette {
  pub fn button<RetType, T: QPalette_button<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.button(self);
    // return 1;
  }
}

pub trait QPalette_button<RetType> {
  fn button(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::button();
impl<'a> /*trait*/ QPalette_button<QBrush> for () {
  fn button(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6buttonEv()};
    let mut ret = unsafe {demth_ZNK8QPalette6buttonEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::foreground();
impl /*struct*/ QPalette {
  pub fn foreground<RetType, T: QPalette_foreground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QPalette_foreground<RetType> {
  fn foreground(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::foreground();
impl<'a> /*trait*/ QPalette_foreground<QBrush> for () {
  fn foreground(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10foregroundEv()};
    let mut ret = unsafe {demth_ZNK8QPalette10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::background();
impl /*struct*/ QPalette {
  pub fn background<RetType, T: QPalette_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QPalette_background<RetType> {
  fn background(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::background();
impl<'a> /*trait*/ QPalette_background<QBrush> for () {
  fn background(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10backgroundEv()};
    let mut ret = unsafe {demth_ZNK8QPalette10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPalette::resolve(uint mask);
impl /*struct*/ QPalette {
  pub fn resolve<RetType, T: QPalette_resolve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolve(self);
    // return 1;
  }
}

pub trait QPalette_resolve<RetType> {
  fn resolve(self , rsthis: & QPalette) -> RetType;
}

  // proto:  void QPalette::resolve(uint mask);
impl<'a> /*trait*/ QPalette_resolve<()> for (u32) {
  fn resolve(self , rsthis: & QPalette) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette7resolveEj()};
    let arg0 = self  as c_uint;
     unsafe {demth_ZN8QPalette7resolveEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPalette::QPalette();
impl<'a> /*trait*/ QPalette_New for () {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1Ev()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN8QPaletteC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN8QPaletteC1Ev()} as u64;
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
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
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QColor(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN8QPaletteC1ERK6QColor(arg0)} as u64;
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
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

  // proto:  void QPalette::swap(QPalette & other);
impl /*struct*/ QPalette {
  pub fn swap<RetType, T: QPalette_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPalette_swap<RetType> {
  fn swap(self , rsthis: & QPalette) -> RetType;
}

  // proto:  void QPalette::swap(QPalette & other);
impl<'a> /*trait*/ QPalette_swap<()> for (&'a QPalette) {
  fn swap(self , rsthis: & QPalette) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPalette4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  uint QPalette::resolve();
impl<'a> /*trait*/ QPalette_resolve<u32> for () {
  fn resolve(self , rsthis: & QPalette) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveEv()};
    let mut ret = unsafe {demth_ZNK8QPalette7resolveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::window();
impl /*struct*/ QPalette {
  pub fn window<RetType, T: QPalette_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QPalette_window<RetType> {
  fn window(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::window();
impl<'a> /*trait*/ QPalette_window<QBrush> for () {
  fn window(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6windowEv()};
    let mut ret = unsafe {demth_ZNK8QPalette6windowEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::highlightedText();
impl /*struct*/ QPalette {
  pub fn highlightedText<RetType, T: QPalette_highlightedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.highlightedText(self);
    // return 1;
  }
}

pub trait QPalette_highlightedText<RetType> {
  fn highlightedText(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::highlightedText();
impl<'a> /*trait*/ QPalette_highlightedText<QBrush> for () {
  fn highlightedText(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette15highlightedTextEv()};
    let mut ret = unsafe {demth_ZNK8QPalette15highlightedTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QColor & button, const QColor & window);
impl<'a> /*trait*/ QPalette_New for (&'a QColor, &'a QColor) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColorS2_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERK6QColorS2_(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN8QPaletteC1ERK6QColorS2_(arg0, arg1)} as u64;
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::text();
impl /*struct*/ QPalette {
  pub fn text<RetType, T: QPalette_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QPalette_text<RetType> {
  fn text(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::text();
impl<'a> /*trait*/ QPalette_text<QBrush> for () {
  fn text(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4textEv()};
    let mut ret = unsafe {demth_ZNK8QPalette4textEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::light();
impl /*struct*/ QPalette {
  pub fn light<RetType, T: QPalette_light<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.light(self);
    // return 1;
  }
}

pub trait QPalette_light<RetType> {
  fn light(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::light();
impl<'a> /*trait*/ QPalette_light<QBrush> for () {
  fn light(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette5lightEv()};
    let mut ret = unsafe {demth_ZNK8QPalette5lightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPalette QPalette::resolve(const QPalette & );
impl<'a> /*trait*/ QPalette_resolve<QPalette> for (&'a QPalette) {
  fn resolve(self , rsthis: & QPalette) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPalette7resolveERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::link();
impl /*struct*/ QPalette {
  pub fn link<RetType, T: QPalette_link<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.link(self);
    // return 1;
  }
}

pub trait QPalette_link<RetType> {
  fn link(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::link();
impl<'a> /*trait*/ QPalette_link<QBrush> for () {
  fn link(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4linkEv()};
    let mut ret = unsafe {demth_ZNK8QPalette4linkEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
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

  // proto:  const QBrush & QPalette::base();
impl /*struct*/ QPalette {
  pub fn base<RetType, T: QPalette_base<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.base(self);
    // return 1;
  }
}

pub trait QPalette_base<RetType> {
  fn base(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::base();
impl<'a> /*trait*/ QPalette_base<QBrush> for () {
  fn base(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4baseEv()};
    let mut ret = unsafe {demth_ZNK8QPalette4baseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::dark();
impl /*struct*/ QPalette {
  pub fn dark<RetType, T: QPalette_dark<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dark(self);
    // return 1;
  }
}

pub trait QPalette_dark<RetType> {
  fn dark(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::dark();
impl<'a> /*trait*/ QPalette_dark<QBrush> for () {
  fn dark(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4darkEv()};
    let mut ret = unsafe {demth_ZNK8QPalette4darkEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::highlight();
impl /*struct*/ QPalette {
  pub fn highlight<RetType, T: QPalette_highlight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.highlight(self);
    // return 1;
  }
}

pub trait QPalette_highlight<RetType> {
  fn highlight(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::highlight();
impl<'a> /*trait*/ QPalette_highlight<QBrush> for () {
  fn highlight(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette9highlightEv()};
    let mut ret = unsafe {demth_ZNK8QPalette9highlightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::mid();
impl /*struct*/ QPalette {
  pub fn mid<RetType, T: QPalette_mid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mid(self);
    // return 1;
  }
}

pub trait QPalette_mid<RetType> {
  fn mid(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::mid();
impl<'a> /*trait*/ QPalette_mid<QBrush> for () {
  fn mid(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette3midEv()};
    let mut ret = unsafe {demth_ZNK8QPalette3midEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QPalette & palette);
impl<'a> /*trait*/ QPalette_New for (&'a QPalette) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERKS_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QPaletteC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN8QPaletteC1ERKS_(arg0)} as u64;
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::shadow();
impl /*struct*/ QPalette {
  pub fn shadow<RetType, T: QPalette_shadow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shadow(self);
    // return 1;
  }
}

pub trait QPalette_shadow<RetType> {
  fn shadow(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::shadow();
impl<'a> /*trait*/ QPalette_shadow<QBrush> for () {
  fn shadow(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6shadowEv()};
    let mut ret = unsafe {demth_ZNK8QPalette6shadowEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::buttonText();
impl /*struct*/ QPalette {
  pub fn buttonText<RetType, T: QPalette_buttonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttonText(self);
    // return 1;
  }
}

pub trait QPalette_buttonText<RetType> {
  fn buttonText(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::buttonText();
impl<'a> /*trait*/ QPalette_buttonText<QBrush> for () {
  fn buttonText(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10buttonTextEv()};
    let mut ret = unsafe {demth_ZNK8QPalette10buttonTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::toolTipBase();
impl /*struct*/ QPalette {
  pub fn toolTipBase<RetType, T: QPalette_toolTipBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTipBase(self);
    // return 1;
  }
}

pub trait QPalette_toolTipBase<RetType> {
  fn toolTipBase(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::toolTipBase();
impl<'a> /*trait*/ QPalette_toolTipBase<QBrush> for () {
  fn toolTipBase(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipBaseEv()};
    let mut ret = unsafe {demth_ZNK8QPalette11toolTipBaseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::midlight();
impl /*struct*/ QPalette {
  pub fn midlight<RetType, T: QPalette_midlight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.midlight(self);
    // return 1;
  }
}

pub trait QPalette_midlight<RetType> {
  fn midlight(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::midlight();
impl<'a> /*trait*/ QPalette_midlight<QBrush> for () {
  fn midlight(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8midlightEv()};
    let mut ret = unsafe {demth_ZNK8QPalette8midlightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::brightText();
impl /*struct*/ QPalette {
  pub fn brightText<RetType, T: QPalette_brightText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brightText(self);
    // return 1;
  }
}

pub trait QPalette_brightText<RetType> {
  fn brightText(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::brightText();
impl<'a> /*trait*/ QPalette_brightText<QBrush> for () {
  fn brightText(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10brightTextEv()};
    let mut ret = unsafe {demth_ZNK8QPalette10brightTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::linkVisited();
impl /*struct*/ QPalette {
  pub fn linkVisited<RetType, T: QPalette_linkVisited<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.linkVisited(self);
    // return 1;
  }
}

pub trait QPalette_linkVisited<RetType> {
  fn linkVisited(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::linkVisited();
impl<'a> /*trait*/ QPalette_linkVisited<QBrush> for () {
  fn linkVisited(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11linkVisitedEv()};
    let mut ret = unsafe {demth_ZNK8QPalette11linkVisitedEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::alternateBase();
impl /*struct*/ QPalette {
  pub fn alternateBase<RetType, T: QPalette_alternateBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.alternateBase(self);
    // return 1;
  }
}

pub trait QPalette_alternateBase<RetType> {
  fn alternateBase(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::alternateBase();
impl<'a> /*trait*/ QPalette_alternateBase<QBrush> for () {
  fn alternateBase(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette13alternateBaseEv()};
    let mut ret = unsafe {demth_ZNK8QPalette13alternateBaseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPalette::QPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
impl<'a> /*trait*/ QPalette_New for (&'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush, &'a QBrush) {
  fn New(self) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_()};
    let ctysz: c_int = unsafe{QPalette_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
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
    let qthis: u64 = unsafe {dector_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)} as u64;
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::windowText();
impl /*struct*/ QPalette {
  pub fn windowText<RetType, T: QPalette_windowText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowText(self);
    // return 1;
  }
}

pub trait QPalette_windowText<RetType> {
  fn windowText(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::windowText();
impl<'a> /*trait*/ QPalette_windowText<QBrush> for () {
  fn windowText(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10windowTextEv()};
    let mut ret = unsafe {demth_ZNK8QPalette10windowTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QBrush & QPalette::toolTipText();
impl /*struct*/ QPalette {
  pub fn toolTipText<RetType, T: QPalette_toolTipText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTipText(self);
    // return 1;
  }
}

pub trait QPalette_toolTipText<RetType> {
  fn toolTipText(self , rsthis: & QPalette) -> RetType;
}

  // proto:  const QBrush & QPalette::toolTipText();
impl<'a> /*trait*/ QPalette_toolTipText<QBrush> for () {
  fn toolTipText(self , rsthis: & QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipTextEv()};
    let mut ret = unsafe {demth_ZNK8QPalette11toolTipTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

