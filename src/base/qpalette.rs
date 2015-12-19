// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;
use super::qbrush::QBrush;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPalette::NewQPalette(const QColor & windowText, const QColor & window, const QColor & light, const QColor & dark, const QColor & mid, const QColor & text, const QColor & base);
  fn _ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void) ;
  // proto:  void QPalette::FreeQPalette();
  fn _ZN8QPaletteD0Ev(qthis: *mut c_void) ;
  // proto:  const QBrush & QPalette::button();
  fn _ZNK8QPalette6buttonEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::foreground();
  fn _ZNK8QPalette10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::background();
  fn _ZNK8QPalette10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPalette::resolve(uint mask);
  fn _ZN8QPalette7resolveEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QPalette::NewQPalette();
  fn _ZN8QPaletteC1Ev(qthis: *mut c_void) ;
  // proto:  void QPalette::NewQPalette(const QColor & button);
  fn _ZN8QPaletteC1ERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPalette::isCopyOf(const QPalette & p);
  fn _ZNK8QPalette8isCopyOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QPalette::swap(QPalette & other);
  fn _ZN8QPalette4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  unsigned int QPalette::resolve();
  fn _ZNK8QPalette7resolveEv(qthis: *mut c_void) -> c_uint;
  // proto:  const QBrush & QPalette::window();
  fn _ZNK8QPalette6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::highlightedText();
  fn _ZNK8QPalette15highlightedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPalette::NewQPalette(const QColor & button, const QColor & window);
  fn _ZN8QPaletteC1ERK6QColorS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QBrush & QPalette::text();
  fn _ZNK8QPalette4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::light();
  fn _ZNK8QPalette5lightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPalette QPalette::resolve(const QPalette & );
  fn _ZNK8QPalette7resolveERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::link();
  fn _ZNK8QPalette4linkEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QPalette::cacheKey();
  fn _ZNK8QPalette8cacheKeyEv(qthis: *mut c_void) -> c_longlong;
  // proto:  const QBrush & QPalette::base();
  fn _ZNK8QPalette4baseEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::dark();
  fn _ZNK8QPalette4darkEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::highlight();
  fn _ZNK8QPalette9highlightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::mid();
  fn _ZNK8QPalette3midEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPalette::NewQPalette(const QPalette & palette);
  fn _ZN8QPaletteC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QBrush & QPalette::shadow();
  fn _ZNK8QPalette6shadowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::buttonText();
  fn _ZNK8QPalette10buttonTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::toolTipBase();
  fn _ZNK8QPalette11toolTipBaseEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::midlight();
  fn _ZNK8QPalette8midlightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::brightText();
  fn _ZNK8QPalette10brightTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::linkVisited();
  fn _ZNK8QPalette11linkVisitedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::alternateBase();
  fn _ZNK8QPalette13alternateBaseEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPalette::NewQPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
  fn _ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void) ;
  // proto:  const QBrush & QPalette::windowText();
  fn _ZNK8QPalette10windowTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QBrush & QPalette::toolTipText();
  fn _ZNK8QPalette11toolTipTextEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPalette)=16
pub struct QPalette {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPalette {
  pub fn NewQPalette<T: QPalette_NewQPalette>(value: T) -> QPalette {
    let rsthis = value.NewQPalette();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_NewQPalette {
  fn NewQPalette(self) -> QPalette;
}

// proto: void QPalette::NewQPalette(const QColor & windowText, const QColor & window, const QColor & light, const QColor & dark, const QColor & mid, const QColor & text, const QColor & base);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QColor, &'a  QColor, &'a  QColor, &'a  QColor, &'a  QColor, &'a  QColor, &'a  QColor) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    unsafe {_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QPalette::FreeQPalette();
impl /*struct*/ QPalette {
  pub fn FreeQPalette<RetType, T: QPalette_FreeQPalette<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQPalette(self);
    // return 1;
  }
}

pub trait QPalette_FreeQPalette<RetType> {
  fn FreeQPalette(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  void QPalette::FreeQPalette();
impl<'a> /*trait*/ QPalette_FreeQPalette<()> for () {
  fn FreeQPalette(self , rsthis: &mut QPalette) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteD0Ev()};
     unsafe {_ZN8QPaletteD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QBrush & QPalette::button();
impl /*struct*/ QPalette {
  pub fn button<RetType, T: QPalette_button<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.button(self);
    // return 1;
  }
}

pub trait QPalette_button<RetType> {
  fn button(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::button();
impl<'a> /*trait*/ QPalette_button<QBrush> for () {
  fn button(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6buttonEv()};
    let mut ret = unsafe {_ZNK8QPalette6buttonEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::foreground();
impl /*struct*/ QPalette {
  pub fn foreground<RetType, T: QPalette_foreground<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QPalette_foreground<RetType> {
  fn foreground(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::foreground();
impl<'a> /*trait*/ QPalette_foreground<QBrush> for () {
  fn foreground(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10foregroundEv()};
    let mut ret = unsafe {_ZNK8QPalette10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::background();
impl /*struct*/ QPalette {
  pub fn background<RetType, T: QPalette_background<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QPalette_background<RetType> {
  fn background(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::background();
impl<'a> /*trait*/ QPalette_background<QBrush> for () {
  fn background(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10backgroundEv()};
    let mut ret = unsafe {_ZNK8QPalette10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPalette::resolve(uint mask);
impl /*struct*/ QPalette {
  pub fn resolve<RetType, T: QPalette_resolve<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.resolve(self);
    // return 1;
  }
}

pub trait QPalette_resolve<RetType> {
  fn resolve(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  void QPalette::resolve(uint mask);
impl<'a> /*trait*/ QPalette_resolve<()> for (u32) {
  fn resolve(self , rsthis: &mut QPalette) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette7resolveEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN8QPalette7resolveEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QPalette::NewQPalette();
impl<'a> /*trait*/ QPalette_NewQPalette for () {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1Ev()};
    unsafe {_ZN8QPaletteC1Ev(qthis)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPalette::NewQPalette(const QColor & button);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QColor) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPaletteC1ERK6QColor(qthis, arg0)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QPalette::isCopyOf(const QPalette & p);
impl /*struct*/ QPalette {
  pub fn isCopyOf<RetType, T: QPalette_isCopyOf<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isCopyOf(self);
    // return 1;
  }
}

pub trait QPalette_isCopyOf<RetType> {
  fn isCopyOf(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  bool QPalette::isCopyOf(const QPalette & p);
impl<'a> /*trait*/ QPalette_isCopyOf<i8> for (&'a  QPalette) {
  fn isCopyOf(self , rsthis: &mut QPalette) -> i8 {
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
  pub fn swap<RetType, T: QPalette_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPalette_swap<RetType> {
  fn swap(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  void QPalette::swap(QPalette & other);
impl<'a> /*trait*/ QPalette_swap<()> for (&'a mut QPalette) {
  fn swap(self , rsthis: &mut QPalette) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPalette4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  unsigned int QPalette::resolve();
impl<'a> /*trait*/ QPalette_resolve<u32> for () {
  fn resolve(self , rsthis: &mut QPalette) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveEv()};
    let mut ret = unsafe {_ZNK8QPalette7resolveEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::window();
impl /*struct*/ QPalette {
  pub fn window<RetType, T: QPalette_window<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QPalette_window<RetType> {
  fn window(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::window();
impl<'a> /*trait*/ QPalette_window<QBrush> for () {
  fn window(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6windowEv()};
    let mut ret = unsafe {_ZNK8QPalette6windowEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::highlightedText();
impl /*struct*/ QPalette {
  pub fn highlightedText<RetType, T: QPalette_highlightedText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.highlightedText(self);
    // return 1;
  }
}

pub trait QPalette_highlightedText<RetType> {
  fn highlightedText(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::highlightedText();
impl<'a> /*trait*/ QPalette_highlightedText<QBrush> for () {
  fn highlightedText(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette15highlightedTextEv()};
    let mut ret = unsafe {_ZNK8QPalette15highlightedTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPalette::NewQPalette(const QColor & button, const QColor & window);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QColor, &'a  QColor) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColorS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QPaletteC1ERK6QColorS2_(qthis, arg0, arg1)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::text();
impl /*struct*/ QPalette {
  pub fn text<RetType, T: QPalette_text<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QPalette_text<RetType> {
  fn text(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::text();
impl<'a> /*trait*/ QPalette_text<QBrush> for () {
  fn text(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4textEv()};
    let mut ret = unsafe {_ZNK8QPalette4textEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::light();
impl /*struct*/ QPalette {
  pub fn light<RetType, T: QPalette_light<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.light(self);
    // return 1;
  }
}

pub trait QPalette_light<RetType> {
  fn light(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::light();
impl<'a> /*trait*/ QPalette_light<QBrush> for () {
  fn light(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette5lightEv()};
    let mut ret = unsafe {_ZNK8QPalette5lightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPalette QPalette::resolve(const QPalette & );
impl<'a> /*trait*/ QPalette_resolve<QPalette> for (&'a  QPalette) {
  fn resolve(self , rsthis: &mut QPalette) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPalette7resolveERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::link();
impl /*struct*/ QPalette {
  pub fn link<RetType, T: QPalette_link<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.link(self);
    // return 1;
  }
}

pub trait QPalette_link<RetType> {
  fn link(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::link();
impl<'a> /*trait*/ QPalette_link<QBrush> for () {
  fn link(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4linkEv()};
    let mut ret = unsafe {_ZNK8QPalette4linkEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  long long QPalette::cacheKey();
impl /*struct*/ QPalette {
  pub fn cacheKey<RetType, T: QPalette_cacheKey<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QPalette_cacheKey<RetType> {
  fn cacheKey(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  long long QPalette::cacheKey();
impl<'a> /*trait*/ QPalette_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: &mut QPalette) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8cacheKeyEv()};
    let mut ret = unsafe {_ZNK8QPalette8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::base();
impl /*struct*/ QPalette {
  pub fn base<RetType, T: QPalette_base<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.base(self);
    // return 1;
  }
}

pub trait QPalette_base<RetType> {
  fn base(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::base();
impl<'a> /*trait*/ QPalette_base<QBrush> for () {
  fn base(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4baseEv()};
    let mut ret = unsafe {_ZNK8QPalette4baseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::dark();
impl /*struct*/ QPalette {
  pub fn dark<RetType, T: QPalette_dark<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.dark(self);
    // return 1;
  }
}

pub trait QPalette_dark<RetType> {
  fn dark(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::dark();
impl<'a> /*trait*/ QPalette_dark<QBrush> for () {
  fn dark(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4darkEv()};
    let mut ret = unsafe {_ZNK8QPalette4darkEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::highlight();
impl /*struct*/ QPalette {
  pub fn highlight<RetType, T: QPalette_highlight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.highlight(self);
    // return 1;
  }
}

pub trait QPalette_highlight<RetType> {
  fn highlight(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::highlight();
impl<'a> /*trait*/ QPalette_highlight<QBrush> for () {
  fn highlight(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette9highlightEv()};
    let mut ret = unsafe {_ZNK8QPalette9highlightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::mid();
impl /*struct*/ QPalette {
  pub fn mid<RetType, T: QPalette_mid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mid(self);
    // return 1;
  }
}

pub trait QPalette_mid<RetType> {
  fn mid(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::mid();
impl<'a> /*trait*/ QPalette_mid<QBrush> for () {
  fn mid(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette3midEv()};
    let mut ret = unsafe {_ZNK8QPalette3midEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPalette::NewQPalette(const QPalette & palette);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QPalette) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPaletteC1ERKS_(qthis, arg0)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::shadow();
impl /*struct*/ QPalette {
  pub fn shadow<RetType, T: QPalette_shadow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.shadow(self);
    // return 1;
  }
}

pub trait QPalette_shadow<RetType> {
  fn shadow(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::shadow();
impl<'a> /*trait*/ QPalette_shadow<QBrush> for () {
  fn shadow(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6shadowEv()};
    let mut ret = unsafe {_ZNK8QPalette6shadowEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::buttonText();
impl /*struct*/ QPalette {
  pub fn buttonText<RetType, T: QPalette_buttonText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.buttonText(self);
    // return 1;
  }
}

pub trait QPalette_buttonText<RetType> {
  fn buttonText(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::buttonText();
impl<'a> /*trait*/ QPalette_buttonText<QBrush> for () {
  fn buttonText(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10buttonTextEv()};
    let mut ret = unsafe {_ZNK8QPalette10buttonTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::toolTipBase();
impl /*struct*/ QPalette {
  pub fn toolTipBase<RetType, T: QPalette_toolTipBase<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toolTipBase(self);
    // return 1;
  }
}

pub trait QPalette_toolTipBase<RetType> {
  fn toolTipBase(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::toolTipBase();
impl<'a> /*trait*/ QPalette_toolTipBase<QBrush> for () {
  fn toolTipBase(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipBaseEv()};
    let mut ret = unsafe {_ZNK8QPalette11toolTipBaseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::midlight();
impl /*struct*/ QPalette {
  pub fn midlight<RetType, T: QPalette_midlight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.midlight(self);
    // return 1;
  }
}

pub trait QPalette_midlight<RetType> {
  fn midlight(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::midlight();
impl<'a> /*trait*/ QPalette_midlight<QBrush> for () {
  fn midlight(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8midlightEv()};
    let mut ret = unsafe {_ZNK8QPalette8midlightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::brightText();
impl /*struct*/ QPalette {
  pub fn brightText<RetType, T: QPalette_brightText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.brightText(self);
    // return 1;
  }
}

pub trait QPalette_brightText<RetType> {
  fn brightText(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::brightText();
impl<'a> /*trait*/ QPalette_brightText<QBrush> for () {
  fn brightText(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10brightTextEv()};
    let mut ret = unsafe {_ZNK8QPalette10brightTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::linkVisited();
impl /*struct*/ QPalette {
  pub fn linkVisited<RetType, T: QPalette_linkVisited<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.linkVisited(self);
    // return 1;
  }
}

pub trait QPalette_linkVisited<RetType> {
  fn linkVisited(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::linkVisited();
impl<'a> /*trait*/ QPalette_linkVisited<QBrush> for () {
  fn linkVisited(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11linkVisitedEv()};
    let mut ret = unsafe {_ZNK8QPalette11linkVisitedEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::alternateBase();
impl /*struct*/ QPalette {
  pub fn alternateBase<RetType, T: QPalette_alternateBase<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.alternateBase(self);
    // return 1;
  }
}

pub trait QPalette_alternateBase<RetType> {
  fn alternateBase(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::alternateBase();
impl<'a> /*trait*/ QPalette_alternateBase<QBrush> for () {
  fn alternateBase(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette13alternateBaseEv()};
    let mut ret = unsafe {_ZNK8QPalette13alternateBaseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPalette::NewQPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    let arg7 = self.7.qclsinst  as *mut c_void;
    let arg8 = self.8.qclsinst  as *mut c_void;
    unsafe {_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::windowText();
impl /*struct*/ QPalette {
  pub fn windowText<RetType, T: QPalette_windowText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.windowText(self);
    // return 1;
  }
}

pub trait QPalette_windowText<RetType> {
  fn windowText(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::windowText();
impl<'a> /*trait*/ QPalette_windowText<QBrush> for () {
  fn windowText(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10windowTextEv()};
    let mut ret = unsafe {_ZNK8QPalette10windowTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QBrush & QPalette::toolTipText();
impl /*struct*/ QPalette {
  pub fn toolTipText<RetType, T: QPalette_toolTipText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toolTipText(self);
    // return 1;
  }
}

pub trait QPalette_toolTipText<RetType> {
  fn toolTipText(self , rsthis: &mut QPalette) -> RetType;
}

// proto:  const QBrush & QPalette::toolTipText();
impl<'a> /*trait*/ QPalette_toolTipText<QBrush> for () {
  fn toolTipText(self , rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipTextEv()};
    let mut ret = unsafe {_ZNK8QPalette11toolTipTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

