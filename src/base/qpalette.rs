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
  // proto: void QPalette::NewQPalette(const QColor & windowText, const QColor & window, const QColor & light, const QColor & dark, const QColor & mid, const QColor & text, const QColor & base);
  fn _ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void, arg6: *const c_void) -> i32;
  // proto: void QPalette::FreeQPalette();
  fn _ZN8QPaletteD0Ev() -> i32;
  // proto: const QBrush & QPalette::button();
  fn _ZNK8QPalette6buttonEv() -> i32;
  // proto: const QBrush & QPalette::foreground();
  fn _ZNK8QPalette10foregroundEv() -> i32;
  // proto: const QBrush & QPalette::background();
  fn _ZNK8QPalette10backgroundEv() -> i32;
  // proto: void QPalette::resolve(uint mask);
  fn _ZN8QPalette7resolveEj(arg0: c_uint) -> i32;
  // proto: void QPalette::NewQPalette();
  fn _ZN8QPaletteC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPalette::NewQPalette(const QColor & button);
  fn _ZN8QPaletteC1ERK6QColor(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPalette::isCopyOf(const QPalette & p);
  fn _ZNK8QPalette8isCopyOfERKS_(arg0: *const c_void) -> i32;
  // proto: void QPalette::swap(QPalette & other);
  fn _ZN8QPalette4swapERS_(arg0: *mut c_void) -> i32;
  // proto: unsigned int QPalette::resolve();
  fn _ZNK8QPalette7resolveEv() -> i32;
  // proto: const QBrush & QPalette::window();
  fn _ZNK8QPalette6windowEv() -> i32;
  // proto: const QBrush & QPalette::highlightedText();
  fn _ZNK8QPalette15highlightedTextEv() -> i32;
  // proto: void QPalette::NewQPalette(const QColor & button, const QColor & window);
  fn _ZN8QPaletteC1ERK6QColorS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: const QBrush & QPalette::text();
  fn _ZNK8QPalette4textEv() -> i32;
  // proto: const QBrush & QPalette::light();
  fn _ZNK8QPalette5lightEv() -> i32;
  // proto: QPalette QPalette::resolve(const QPalette & );
  fn _ZNK8QPalette7resolveERKS_(arg0: *const c_void) -> i32;
  // proto: const QBrush & QPalette::link();
  fn _ZNK8QPalette4linkEv() -> i32;
  // proto: long long QPalette::cacheKey();
  fn _ZNK8QPalette8cacheKeyEv() -> i32;
  // proto: const QBrush & QPalette::base();
  fn _ZNK8QPalette4baseEv() -> i32;
  // proto: const QBrush & QPalette::dark();
  fn _ZNK8QPalette4darkEv() -> i32;
  // proto: const QBrush & QPalette::highlight();
  fn _ZNK8QPalette9highlightEv() -> i32;
  // proto: const QBrush & QPalette::mid();
  fn _ZNK8QPalette3midEv() -> i32;
  // proto: void QPalette::NewQPalette(const QPalette & palette);
  fn _ZN8QPaletteC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QBrush & QPalette::shadow();
  fn _ZNK8QPalette6shadowEv() -> i32;
  // proto: const QBrush & QPalette::buttonText();
  fn _ZNK8QPalette10buttonTextEv() -> i32;
  // proto: const QBrush & QPalette::toolTipBase();
  fn _ZNK8QPalette11toolTipBaseEv() -> i32;
  // proto: const QBrush & QPalette::midlight();
  fn _ZNK8QPalette8midlightEv() -> i32;
  // proto: const QBrush & QPalette::brightText();
  fn _ZNK8QPalette10brightTextEv() -> i32;
  // proto: const QBrush & QPalette::linkVisited();
  fn _ZNK8QPalette11linkVisitedEv() -> i32;
  // proto: const QBrush & QPalette::alternateBase();
  fn _ZNK8QPalette13alternateBaseEv() -> i32;
  // proto: void QPalette::NewQPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
  fn _ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void, arg6: *const c_void, arg7: *const c_void, arg8: *const c_void) -> i32;
  // proto: const QBrush & QPalette::windowText();
  fn _ZNK8QPalette10windowTextEv() -> i32;
  // proto: const QBrush & QPalette::toolTipText();
  fn _ZNK8QPalette11toolTipTextEv() -> i32;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6.qclsinst  as *const c_void;
    unsafe {_ZN8QPaletteC1ERK6QColorS2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn FreeQPalette<T: QPalette_FreeQPalette>(&mut self, value: T) -> i32 {
    value.FreeQPalette(self);
    return 1;
  }
}

pub trait QPalette_FreeQPalette {
  fn FreeQPalette(self, this: &mut QPalette) -> i32;
}

// proto: void QPalette::FreeQPalette();
impl<'a> /*trait*/ QPalette_FreeQPalette for () {
  fn FreeQPalette(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteD0Ev()};
    unsafe {_ZN8QPaletteD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn button<T: QPalette_button>(&mut self, value: T) -> i32 {
    value.button(self);
    return 1;
  }
}

pub trait QPalette_button {
  fn button(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::button();
impl<'a> /*trait*/ QPalette_button for () {
  fn button(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6buttonEv()};
    unsafe {_ZNK8QPalette6buttonEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn foreground<T: QPalette_foreground>(&mut self, value: T) -> i32 {
    value.foreground(self);
    return 1;
  }
}

pub trait QPalette_foreground {
  fn foreground(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::foreground();
impl<'a> /*trait*/ QPalette_foreground for () {
  fn foreground(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10foregroundEv()};
    unsafe {_ZNK8QPalette10foregroundEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn background<T: QPalette_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QPalette_background {
  fn background(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::background();
impl<'a> /*trait*/ QPalette_background for () {
  fn background(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10backgroundEv()};
    unsafe {_ZNK8QPalette10backgroundEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn resolve<T: QPalette_resolve>(&mut self, value: T) -> i32 {
    value.resolve(self);
    return 1;
  }
}

pub trait QPalette_resolve {
  fn resolve(self, this: &mut QPalette) -> i32;
}

// proto: void QPalette::resolve(uint mask);
impl<'a> /*trait*/ QPalette_resolve for (u32) {
  fn resolve(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette7resolveEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN8QPalette7resolveEj(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPaletteC1ERK6QColor(qthis, arg0)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn isCopyOf<T: QPalette_isCopyOf>(&mut self, value: T) -> i32 {
    value.isCopyOf(self);
    return 1;
  }
}

pub trait QPalette_isCopyOf {
  fn isCopyOf(self, this: &mut QPalette) -> i32;
}

// proto: bool QPalette::isCopyOf(const QPalette & p);
impl<'a> /*trait*/ QPalette_isCopyOf for (&'a  QPalette) {
  fn isCopyOf(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8isCopyOfERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QPalette8isCopyOfERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn swap<T: QPalette_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPalette_swap {
  fn swap(self, this: &mut QPalette) -> i32;
}

// proto: void QPalette::swap(QPalette & other);
impl<'a> /*trait*/ QPalette_swap for (&'a mut QPalette) {
  fn swap(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPalette4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPalette4swapERS_(arg0)};
    return 1;
  }
}

// proto: unsigned int QPalette::resolve();
impl<'a> /*trait*/ QPalette_resolve for () {
  fn resolve(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveEv()};
    unsafe {_ZNK8QPalette7resolveEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn window<T: QPalette_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QPalette_window {
  fn window(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::window();
impl<'a> /*trait*/ QPalette_window for () {
  fn window(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6windowEv()};
    unsafe {_ZNK8QPalette6windowEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn highlightedText<T: QPalette_highlightedText>(&mut self, value: T) -> i32 {
    value.highlightedText(self);
    return 1;
  }
}

pub trait QPalette_highlightedText {
  fn highlightedText(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::highlightedText();
impl<'a> /*trait*/ QPalette_highlightedText for () {
  fn highlightedText(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette15highlightedTextEv()};
    unsafe {_ZNK8QPalette15highlightedTextEv()};
    return 1;
  }
}

// proto: void QPalette::NewQPalette(const QColor & button, const QColor & window);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QColor, &'a  QColor) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QColorS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPaletteC1ERK6QColorS2_(qthis, arg0, arg1)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn text<T: QPalette_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QPalette_text {
  fn text(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::text();
impl<'a> /*trait*/ QPalette_text for () {
  fn text(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4textEv()};
    unsafe {_ZNK8QPalette4textEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn light<T: QPalette_light>(&mut self, value: T) -> i32 {
    value.light(self);
    return 1;
  }
}

pub trait QPalette_light {
  fn light(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::light();
impl<'a> /*trait*/ QPalette_light for () {
  fn light(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette5lightEv()};
    unsafe {_ZNK8QPalette5lightEv()};
    return 1;
  }
}

// proto: QPalette QPalette::resolve(const QPalette & );
impl<'a> /*trait*/ QPalette_resolve for (&'a  QPalette) {
  fn resolve(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette7resolveERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QPalette7resolveERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn link<T: QPalette_link>(&mut self, value: T) -> i32 {
    value.link(self);
    return 1;
  }
}

pub trait QPalette_link {
  fn link(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::link();
impl<'a> /*trait*/ QPalette_link for () {
  fn link(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4linkEv()};
    unsafe {_ZNK8QPalette4linkEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn cacheKey<T: QPalette_cacheKey>(&mut self, value: T) -> i32 {
    value.cacheKey(self);
    return 1;
  }
}

pub trait QPalette_cacheKey {
  fn cacheKey(self, this: &mut QPalette) -> i32;
}

// proto: long long QPalette::cacheKey();
impl<'a> /*trait*/ QPalette_cacheKey for () {
  fn cacheKey(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8cacheKeyEv()};
    unsafe {_ZNK8QPalette8cacheKeyEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn base<T: QPalette_base>(&mut self, value: T) -> i32 {
    value.base(self);
    return 1;
  }
}

pub trait QPalette_base {
  fn base(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::base();
impl<'a> /*trait*/ QPalette_base for () {
  fn base(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4baseEv()};
    unsafe {_ZNK8QPalette4baseEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn dark<T: QPalette_dark>(&mut self, value: T) -> i32 {
    value.dark(self);
    return 1;
  }
}

pub trait QPalette_dark {
  fn dark(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::dark();
impl<'a> /*trait*/ QPalette_dark for () {
  fn dark(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette4darkEv()};
    unsafe {_ZNK8QPalette4darkEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn highlight<T: QPalette_highlight>(&mut self, value: T) -> i32 {
    value.highlight(self);
    return 1;
  }
}

pub trait QPalette_highlight {
  fn highlight(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::highlight();
impl<'a> /*trait*/ QPalette_highlight for () {
  fn highlight(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette9highlightEv()};
    unsafe {_ZNK8QPalette9highlightEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn mid<T: QPalette_mid>(&mut self, value: T) -> i32 {
    value.mid(self);
    return 1;
  }
}

pub trait QPalette_mid {
  fn mid(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::mid();
impl<'a> /*trait*/ QPalette_mid for () {
  fn mid(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette3midEv()};
    unsafe {_ZNK8QPalette3midEv()};
    return 1;
  }
}

// proto: void QPalette::NewQPalette(const QPalette & palette);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QPalette) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPaletteC1ERKS_(qthis, arg0)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn shadow<T: QPalette_shadow>(&mut self, value: T) -> i32 {
    value.shadow(self);
    return 1;
  }
}

pub trait QPalette_shadow {
  fn shadow(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::shadow();
impl<'a> /*trait*/ QPalette_shadow for () {
  fn shadow(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette6shadowEv()};
    unsafe {_ZNK8QPalette6shadowEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn buttonText<T: QPalette_buttonText>(&mut self, value: T) -> i32 {
    value.buttonText(self);
    return 1;
  }
}

pub trait QPalette_buttonText {
  fn buttonText(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::buttonText();
impl<'a> /*trait*/ QPalette_buttonText for () {
  fn buttonText(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10buttonTextEv()};
    unsafe {_ZNK8QPalette10buttonTextEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn toolTipBase<T: QPalette_toolTipBase>(&mut self, value: T) -> i32 {
    value.toolTipBase(self);
    return 1;
  }
}

pub trait QPalette_toolTipBase {
  fn toolTipBase(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::toolTipBase();
impl<'a> /*trait*/ QPalette_toolTipBase for () {
  fn toolTipBase(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipBaseEv()};
    unsafe {_ZNK8QPalette11toolTipBaseEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn midlight<T: QPalette_midlight>(&mut self, value: T) -> i32 {
    value.midlight(self);
    return 1;
  }
}

pub trait QPalette_midlight {
  fn midlight(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::midlight();
impl<'a> /*trait*/ QPalette_midlight for () {
  fn midlight(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette8midlightEv()};
    unsafe {_ZNK8QPalette8midlightEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn brightText<T: QPalette_brightText>(&mut self, value: T) -> i32 {
    value.brightText(self);
    return 1;
  }
}

pub trait QPalette_brightText {
  fn brightText(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::brightText();
impl<'a> /*trait*/ QPalette_brightText for () {
  fn brightText(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10brightTextEv()};
    unsafe {_ZNK8QPalette10brightTextEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn linkVisited<T: QPalette_linkVisited>(&mut self, value: T) -> i32 {
    value.linkVisited(self);
    return 1;
  }
}

pub trait QPalette_linkVisited {
  fn linkVisited(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::linkVisited();
impl<'a> /*trait*/ QPalette_linkVisited for () {
  fn linkVisited(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11linkVisitedEv()};
    unsafe {_ZNK8QPalette11linkVisitedEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn alternateBase<T: QPalette_alternateBase>(&mut self, value: T) -> i32 {
    value.alternateBase(self);
    return 1;
  }
}

pub trait QPalette_alternateBase {
  fn alternateBase(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::alternateBase();
impl<'a> /*trait*/ QPalette_alternateBase for () {
  fn alternateBase(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette13alternateBaseEv()};
    unsafe {_ZNK8QPalette13alternateBaseEv()};
    return 1;
  }
}

// proto: void QPalette::NewQPalette(const QBrush & windowText, const QBrush & button, const QBrush & light, const QBrush & dark, const QBrush & mid, const QBrush & text, const QBrush & bright_text, const QBrush & base, const QBrush & window);
impl<'a> /*trait*/ QPalette_NewQPalette for (&'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush, &'a  QBrush) {
  fn NewQPalette(self) -> QPalette {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6.qclsinst  as *const c_void;
    let arg7 = self.7.qclsinst  as *const c_void;
    let arg8 = self.8.qclsinst  as *const c_void;
    unsafe {_ZN8QPaletteC1ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let rsthis = QPalette{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn windowText<T: QPalette_windowText>(&mut self, value: T) -> i32 {
    value.windowText(self);
    return 1;
  }
}

pub trait QPalette_windowText {
  fn windowText(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::windowText();
impl<'a> /*trait*/ QPalette_windowText for () {
  fn windowText(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette10windowTextEv()};
    unsafe {_ZNK8QPalette10windowTextEv()};
    return 1;
  }
}

impl /*struct*/ QPalette {
  pub fn toolTipText<T: QPalette_toolTipText>(&mut self, value: T) -> i32 {
    value.toolTipText(self);
    return 1;
  }
}

pub trait QPalette_toolTipText {
  fn toolTipText(self, this: &mut QPalette) -> i32;
}

// proto: const QBrush & QPalette::toolTipText();
impl<'a> /*trait*/ QPalette_toolTipText for () {
  fn toolTipText(self, this: &mut QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPalette11toolTipTextEv()};
    unsafe {_ZNK8QPalette11toolTipTextEv()};
    return 1;
  }
}

