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
  // proto:  void QPalette::NewQPalette();
  fn _ZN8QPaletteC1Ev(qthis: *mut c_void) ;
  // proto:  void QPalette::NewQPalette(const QColor & button);
  fn _ZN8QPaletteC1ERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPalette::isCopyOf(const QPalette & p);
  fn _ZNK8QPalette8isCopyOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QPalette::swap(QPalette & other);
  fn _ZN8QPalette4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
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

impl /*struct*/ QPalette {
  pub fn FreeQPalette<T: QPalette_FreeQPalette>(&mut self, value: T)  {
     value.FreeQPalette(self);
    // return 1;
  }
}

pub trait QPalette_FreeQPalette {
  fn FreeQPalette(self, rsthis: &mut QPalette) ;
}

// proto:  void QPalette::FreeQPalette();
impl<'a> /*trait*/ QPalette_FreeQPalette for () {
  fn FreeQPalette(self, rsthis: &mut QPalette)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteD0Ev()};
     unsafe {_ZN8QPaletteD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn button<T: QPalette_button>(&mut self, value: T) -> QBrush {
    return value.button(self);
    // return 1;
  }
}

pub trait QPalette_button {
  fn button(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::button();
impl<'a> /*trait*/ QPalette_button for () {
  fn button(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6buttonEv()};
    let mut ret = unsafe {_ZNK8QPalette6buttonEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn foreground<T: QPalette_foreground>(&mut self, value: T) -> QBrush {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QPalette_foreground {
  fn foreground(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::foreground();
impl<'a> /*trait*/ QPalette_foreground for () {
  fn foreground(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10foregroundEv()};
    let mut ret = unsafe {_ZNK8QPalette10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn background<T: QPalette_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QPalette_background {
  fn background(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::background();
impl<'a> /*trait*/ QPalette_background for () {
  fn background(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10backgroundEv()};
    let mut ret = unsafe {_ZNK8QPalette10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
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

impl /*struct*/ QPalette {
  pub fn isCopyOf<T: QPalette_isCopyOf>(&mut self, value: T) -> i8 {
    return value.isCopyOf(self);
    // return 1;
  }
}

pub trait QPalette_isCopyOf {
  fn isCopyOf(self, rsthis: &mut QPalette) -> i8;
}

// proto:  bool QPalette::isCopyOf(const QPalette & p);
impl<'a> /*trait*/ QPalette_isCopyOf for (&'a  QPalette) {
  fn isCopyOf(self, rsthis: &mut QPalette) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPalette8isCopyOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn swap<T: QPalette_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QPalette_swap {
  fn swap(self, rsthis: &mut QPalette) ;
}

// proto:  void QPalette::swap(QPalette & other);
impl<'a> /*trait*/ QPalette_swap for (&'a mut QPalette) {
  fn swap(self, rsthis: &mut QPalette)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPalette4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn window<T: QPalette_window>(&mut self, value: T) -> QBrush {
    return value.window(self);
    // return 1;
  }
}

pub trait QPalette_window {
  fn window(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::window();
impl<'a> /*trait*/ QPalette_window for () {
  fn window(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6windowEv()};
    let mut ret = unsafe {_ZNK8QPalette6windowEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn highlightedText<T: QPalette_highlightedText>(&mut self, value: T) -> QBrush {
    return value.highlightedText(self);
    // return 1;
  }
}

pub trait QPalette_highlightedText {
  fn highlightedText(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::highlightedText();
impl<'a> /*trait*/ QPalette_highlightedText for () {
  fn highlightedText(self, rsthis: &mut QPalette) -> QBrush {
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

impl /*struct*/ QPalette {
  pub fn text<T: QPalette_text>(&mut self, value: T) -> QBrush {
    return value.text(self);
    // return 1;
  }
}

pub trait QPalette_text {
  fn text(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::text();
impl<'a> /*trait*/ QPalette_text for () {
  fn text(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4textEv()};
    let mut ret = unsafe {_ZNK8QPalette4textEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn light<T: QPalette_light>(&mut self, value: T) -> QBrush {
    return value.light(self);
    // return 1;
  }
}

pub trait QPalette_light {
  fn light(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::light();
impl<'a> /*trait*/ QPalette_light for () {
  fn light(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette5lightEv()};
    let mut ret = unsafe {_ZNK8QPalette5lightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn link<T: QPalette_link>(&mut self, value: T) -> QBrush {
    return value.link(self);
    // return 1;
  }
}

pub trait QPalette_link {
  fn link(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::link();
impl<'a> /*trait*/ QPalette_link for () {
  fn link(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4linkEv()};
    let mut ret = unsafe {_ZNK8QPalette4linkEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn cacheKey<T: QPalette_cacheKey>(&mut self, value: T) -> i64 {
    return value.cacheKey(self);
    // return 1;
  }
}

pub trait QPalette_cacheKey {
  fn cacheKey(self, rsthis: &mut QPalette) -> i64;
}

// proto:  long long QPalette::cacheKey();
impl<'a> /*trait*/ QPalette_cacheKey for () {
  fn cacheKey(self, rsthis: &mut QPalette) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8cacheKeyEv()};
    let mut ret = unsafe {_ZNK8QPalette8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn base<T: QPalette_base>(&mut self, value: T) -> QBrush {
    return value.base(self);
    // return 1;
  }
}

pub trait QPalette_base {
  fn base(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::base();
impl<'a> /*trait*/ QPalette_base for () {
  fn base(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4baseEv()};
    let mut ret = unsafe {_ZNK8QPalette4baseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn dark<T: QPalette_dark>(&mut self, value: T) -> QBrush {
    return value.dark(self);
    // return 1;
  }
}

pub trait QPalette_dark {
  fn dark(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::dark();
impl<'a> /*trait*/ QPalette_dark for () {
  fn dark(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4darkEv()};
    let mut ret = unsafe {_ZNK8QPalette4darkEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn highlight<T: QPalette_highlight>(&mut self, value: T) -> QBrush {
    return value.highlight(self);
    // return 1;
  }
}

pub trait QPalette_highlight {
  fn highlight(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::highlight();
impl<'a> /*trait*/ QPalette_highlight for () {
  fn highlight(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette9highlightEv()};
    let mut ret = unsafe {_ZNK8QPalette9highlightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn mid<T: QPalette_mid>(&mut self, value: T) -> QBrush {
    return value.mid(self);
    // return 1;
  }
}

pub trait QPalette_mid {
  fn mid(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::mid();
impl<'a> /*trait*/ QPalette_mid for () {
  fn mid(self, rsthis: &mut QPalette) -> QBrush {
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

impl /*struct*/ QPalette {
  pub fn shadow<T: QPalette_shadow>(&mut self, value: T) -> QBrush {
    return value.shadow(self);
    // return 1;
  }
}

pub trait QPalette_shadow {
  fn shadow(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::shadow();
impl<'a> /*trait*/ QPalette_shadow for () {
  fn shadow(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6shadowEv()};
    let mut ret = unsafe {_ZNK8QPalette6shadowEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn buttonText<T: QPalette_buttonText>(&mut self, value: T) -> QBrush {
    return value.buttonText(self);
    // return 1;
  }
}

pub trait QPalette_buttonText {
  fn buttonText(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::buttonText();
impl<'a> /*trait*/ QPalette_buttonText for () {
  fn buttonText(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10buttonTextEv()};
    let mut ret = unsafe {_ZNK8QPalette10buttonTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn toolTipBase<T: QPalette_toolTipBase>(&mut self, value: T) -> QBrush {
    return value.toolTipBase(self);
    // return 1;
  }
}

pub trait QPalette_toolTipBase {
  fn toolTipBase(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::toolTipBase();
impl<'a> /*trait*/ QPalette_toolTipBase for () {
  fn toolTipBase(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipBaseEv()};
    let mut ret = unsafe {_ZNK8QPalette11toolTipBaseEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn midlight<T: QPalette_midlight>(&mut self, value: T) -> QBrush {
    return value.midlight(self);
    // return 1;
  }
}

pub trait QPalette_midlight {
  fn midlight(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::midlight();
impl<'a> /*trait*/ QPalette_midlight for () {
  fn midlight(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8midlightEv()};
    let mut ret = unsafe {_ZNK8QPalette8midlightEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn brightText<T: QPalette_brightText>(&mut self, value: T) -> QBrush {
    return value.brightText(self);
    // return 1;
  }
}

pub trait QPalette_brightText {
  fn brightText(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::brightText();
impl<'a> /*trait*/ QPalette_brightText for () {
  fn brightText(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10brightTextEv()};
    let mut ret = unsafe {_ZNK8QPalette10brightTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn linkVisited<T: QPalette_linkVisited>(&mut self, value: T) -> QBrush {
    return value.linkVisited(self);
    // return 1;
  }
}

pub trait QPalette_linkVisited {
  fn linkVisited(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::linkVisited();
impl<'a> /*trait*/ QPalette_linkVisited for () {
  fn linkVisited(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11linkVisitedEv()};
    let mut ret = unsafe {_ZNK8QPalette11linkVisitedEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn alternateBase<T: QPalette_alternateBase>(&mut self, value: T) -> QBrush {
    return value.alternateBase(self);
    // return 1;
  }
}

pub trait QPalette_alternateBase {
  fn alternateBase(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::alternateBase();
impl<'a> /*trait*/ QPalette_alternateBase for () {
  fn alternateBase(self, rsthis: &mut QPalette) -> QBrush {
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

impl /*struct*/ QPalette {
  pub fn windowText<T: QPalette_windowText>(&mut self, value: T) -> QBrush {
    return value.windowText(self);
    // return 1;
  }
}

pub trait QPalette_windowText {
  fn windowText(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::windowText();
impl<'a> /*trait*/ QPalette_windowText for () {
  fn windowText(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10windowTextEv()};
    let mut ret = unsafe {_ZNK8QPalette10windowTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn toolTipText<T: QPalette_toolTipText>(&mut self, value: T) -> QBrush {
    return value.toolTipText(self);
    // return 1;
  }
}

pub trait QPalette_toolTipText {
  fn toolTipText(self, rsthis: &mut QPalette) -> QBrush;
}

// proto:  const QBrush & QPalette::toolTipText();
impl<'a> /*trait*/ QPalette_toolTipText for () {
  fn toolTipText(self, rsthis: &mut QPalette) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipTextEv()};
    let mut ret = unsafe {_ZNK8QPalette11toolTipTextEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

