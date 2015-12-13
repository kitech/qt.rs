// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QColor::getHsvF(qreal * h, qreal * s, qreal * v, qreal * a);
  fn _ZNK6QColor7getHsvFEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: int QColor::alpha();
  fn _ZNK6QColor5alphaEv() -> i32;
  // proto: double QColor::hslSaturationF();
  fn _ZNK6QColor14hslSaturationFEv() -> i32;
  // proto: void QColor::setAlphaF(qreal alpha);
  fn _ZN6QColor9setAlphaFEd(arg0: c_double) -> i32;
  // proto: void QColor::getRgb(int * r, int * g, int * b, int * a);
  fn _ZNK6QColor6getRgbEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: int QColor::hslHue();
  fn _ZNK6QColor6hslHueEv() -> i32;
  // proto: int QColor::lightness();
  fn _ZNK6QColor9lightnessEv() -> i32;
  // proto: void QColor::setAlpha(int alpha);
  fn _ZN6QColor8setAlphaEi(arg0: c_int) -> i32;
  // proto: QColor QColor::fromHslF(qreal h, qreal s, qreal l, qreal a);
  fn _ZN6QColor8fromHslFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QColor::getCmyk(int * c, int * m, int * y, int * k, int * a);
  fn _ZN6QColor7getCmykEPiS0_S0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int) -> i32;
  // proto: int QColor::green();
  fn _ZNK6QColor5greenEv() -> i32;
  // proto: int QColor::hsvSaturation();
  fn _ZNK6QColor13hsvSaturationEv() -> i32;
  // proto: QColor QColor::toHsl();
  fn _ZNK6QColor5toHslEv() -> i32;
  // proto: void QColor::NewQColor();
  fn _ZN6QColorC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QColor::NewQColor(const char * name);
  fn _ZN6QColorC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  // proto: void QColor::setBlue(int blue);
  fn _ZN6QColor7setBlueEi(arg0: c_int) -> i32;
  // proto: int QColor::cyan();
  fn _ZNK6QColor4cyanEv() -> i32;
  // proto: void QColor::setCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
  fn _ZN6QColor8setCmykFEddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: QColor QColor::fromCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
  fn _ZN6QColor9fromCmykFEddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: QColor QColor::light(int f);
  fn _ZNK6QColor5lightEi(arg0: c_int) -> i32;
  // proto: void QColor::getHslF(qreal * h, qreal * s, qreal * l, qreal * a);
  fn _ZNK6QColor7getHslFEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: QColor QColor::fromRgb(QRgb rgb);
  fn _ZN6QColor7fromRgbEj(arg0: c_uint) -> i32;
  // proto: int QColor::yellow();
  fn _ZNK6QColor6yellowEv() -> i32;
  // proto: void QColor::getRgbF(qreal * r, qreal * g, qreal * b, qreal * a);
  fn _ZNK6QColor7getRgbFEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: void QColor::setRgb(int r, int g, int b, int a);
  fn _ZN6QColor6setRgbEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: QString QColor::name();
  fn _ZNK6QColor4nameEv() -> i32;
  // proto: double QColor::redF();
  fn _ZNK6QColor4redFEv() -> i32;
  // proto: double QColor::blackF();
  fn _ZNK6QColor6blackFEv() -> i32;
  // proto: void QColor::setHsvF(qreal h, qreal s, qreal v, qreal a);
  fn _ZN6QColor7setHsvFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QColor::setRgb(QRgb rgb);
  fn _ZN6QColor6setRgbEj(arg0: c_uint) -> i32;
  // proto: QColor QColor::fromRgb(int r, int g, int b, int a);
  fn _ZN6QColor7fromRgbEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: double QColor::hsvHueF();
  fn _ZNK6QColor7hsvHueFEv() -> i32;
  // proto: double QColor::hsvSaturationF();
  fn _ZNK6QColor14hsvSaturationFEv() -> i32;
  // proto: double QColor::yellowF();
  fn _ZNK6QColor7yellowFEv() -> i32;
  // proto: int QColor::black();
  fn _ZNK6QColor5blackEv() -> i32;
  // proto: void QColor::setGreenF(qreal green);
  fn _ZN6QColor9setGreenFEd(arg0: c_double) -> i32;
  // proto: unsigned int QColor::rgba();
  fn _ZNK6QColor4rgbaEv() -> i32;
  // proto: QColor QColor::toCmyk();
  fn _ZNK6QColor6toCmykEv() -> i32;
  // proto: double QColor::greenF();
  fn _ZNK6QColor6greenFEv() -> i32;
  // proto: int QColor::red();
  fn _ZNK6QColor3redEv() -> i32;
  // proto: void QColor::setRgbF(qreal r, qreal g, qreal b, qreal a);
  fn _ZN6QColor7setRgbFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: double QColor::lightnessF();
  fn _ZNK6QColor10lightnessFEv() -> i32;
  // proto: QColor QColor::toHsv();
  fn _ZNK6QColor5toHsvEv() -> i32;
  // proto: void QColor::NewQColor(const QColor & color);
  fn _ZN6QColorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QColor QColor::fromHsv(int h, int s, int v, int a);
  fn _ZN6QColor7fromHsvEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: double QColor::hueF();
  fn _ZNK6QColor4hueFEv() -> i32;
  // proto: void QColor::setBlueF(qreal blue);
  fn _ZN6QColor8setBlueFEd(arg0: c_double) -> i32;
  // proto: double QColor::saturationF();
  fn _ZNK6QColor11saturationFEv() -> i32;
  // proto: bool QColor::isValid();
  fn _ZNK6QColor7isValidEv() -> i32;
  // proto: QColor QColor::darker(int f);
  fn _ZNK6QColor6darkerEi(arg0: c_int) -> i32;
  // proto: double QColor::blueF();
  fn _ZNK6QColor5blueFEv() -> i32;
  // proto: int QColor::hue();
  fn _ZNK6QColor3hueEv() -> i32;
  // proto: void QColor::setRgba(QRgb rgba);
  fn _ZN6QColor7setRgbaEj(arg0: c_uint) -> i32;
  // proto: void QColor::setNamedColor(const QString & name);
  fn _ZN6QColor13setNamedColorERK7QString(arg0: *const c_void) -> i32;
  // proto: int QColor::magenta();
  fn _ZNK6QColor7magentaEv() -> i32;
  // proto: QColor QColor::lighter(int f);
  fn _ZNK6QColor7lighterEi(arg0: c_int) -> i32;
  // proto: QColor QColor::toRgb();
  fn _ZNK6QColor5toRgbEv() -> i32;
  // proto: double QColor::magentaF();
  fn _ZNK6QColor8magentaFEv() -> i32;
  // proto: double QColor::hslHueF();
  fn _ZNK6QColor7hslHueFEv() -> i32;
  // proto: QColor QColor::fromCmyk(int c, int m, int y, int k, int a);
  fn _ZN6QColor8fromCmykEiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> i32;
  // proto: void QColor::setCmyk(int c, int m, int y, int k, int a);
  fn _ZN6QColor7setCmykEiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> i32;
  // proto: QStringList QColor::colorNames();
  fn _ZN6QColor10colorNamesEv() -> i32;
  // proto: void QColor::getHsv(int * h, int * s, int * v, int * a);
  fn _ZNK6QColor6getHsvEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: void QColor::getCmykF(qreal * c, qreal * m, qreal * y, qreal * k, qreal * a);
  fn _ZN6QColor8getCmykFEPdS0_S0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double, arg4: *mut c_double) -> i32;
  // proto: void QColor::setRed(int red);
  fn _ZN6QColor6setRedEi(arg0: c_int) -> i32;
  // proto: void QColor::NewQColor(const QString & name);
  fn _ZN6QColorC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QColor QColor::fromRgba(QRgb rgba);
  fn _ZN6QColor8fromRgbaEj(arg0: c_uint) -> i32;
  // proto: void QColor::setHsv(int h, int s, int v, int a);
  fn _ZN6QColor6setHsvEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: unsigned int QColor::rgb();
  fn _ZNK6QColor3rgbEv() -> i32;
  // proto: void QColor::setHslF(qreal h, qreal s, qreal l, qreal a);
  fn _ZN6QColor7setHslFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: int QColor::saturation();
  fn _ZNK6QColor10saturationEv() -> i32;
  // proto: void QColor::NewQColor(int r, int g, int b, int a);
  fn _ZN6QColorC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: double QColor::alphaF();
  fn _ZNK6QColor6alphaFEv() -> i32;
  // proto: int QColor::value();
  fn _ZNK6QColor5valueEv() -> i32;
  // proto: QColor QColor::fromHsvF(qreal h, qreal s, qreal v, qreal a);
  fn _ZN6QColor8fromHsvFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: QColor QColor::dark(int f);
  fn _ZNK6QColor4darkEi(arg0: c_int) -> i32;
  // proto: void QColor::setRedF(qreal red);
  fn _ZN6QColor7setRedFEd(arg0: c_double) -> i32;
  // proto: QColor QColor::fromHsl(int h, int s, int l, int a);
  fn _ZN6QColor7fromHslEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QColor::setHsl(int h, int s, int l, int a);
  fn _ZN6QColor6setHslEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QColor::NewQColor(QRgb rgb);
  fn _ZN6QColorC1Ej(qthis: *mut c_void, arg0: c_uint) -> i32;
  // proto: void QColor::setGreen(int green);
  fn _ZN6QColor8setGreenEi(arg0: c_int) -> i32;
  // proto: void QColor::getHsl(int * h, int * s, int * l, int * a);
  fn _ZNK6QColor6getHslEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: bool QColor::isValidColor(const QString & name);
  fn _ZN6QColor12isValidColorERK7QString(arg0: *const c_void) -> i32;
  // proto: int QColor::hslSaturation();
  fn _ZNK6QColor13hslSaturationEv() -> i32;
  // proto: QColor QColor::fromRgbF(qreal r, qreal g, qreal b, qreal a);
  fn _ZN6QColor8fromRgbFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: int QColor::blue();
  fn _ZNK6QColor4blueEv() -> i32;
  // proto: int QColor::hsvHue();
  fn _ZNK6QColor6hsvHueEv() -> i32;
  // proto: double QColor::valueF();
  fn _ZNK6QColor6valueFEv() -> i32;
  // proto: double QColor::cyanF();
  fn _ZNK6QColor5cyanFEv() -> i32;
}

// body block begin
// class sizeof(QColor)=16
pub struct QColor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColor {
  pub fn getHsvF<T: QColor_getHsvF>(&mut self, value: T) -> i32 {
    value.getHsvF(self);
    return 1;
  }
}

pub trait QColor_getHsvF {
  fn getHsvF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getHsvF(qreal * h, qreal * s, qreal * v, qreal * a);
impl<'a> /*trait*/ QColor_getHsvF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getHsvF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7getHsvFEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK6QColor7getHsvFEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn alpha<T: QColor_alpha>(&mut self, value: T) -> i32 {
    value.alpha(self);
    return 1;
  }
}

pub trait QColor_alpha {
  fn alpha(self, this: &mut QColor) -> i32;
}

// proto: int QColor::alpha();
impl<'a> /*trait*/ QColor_alpha for () {
  fn alpha(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5alphaEv()};
    unsafe {_ZNK6QColor5alphaEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslSaturationF<T: QColor_hslSaturationF>(&mut self, value: T) -> i32 {
    value.hslSaturationF(self);
    return 1;
  }
}

pub trait QColor_hslSaturationF {
  fn hslSaturationF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::hslSaturationF();
impl<'a> /*trait*/ QColor_hslSaturationF for () {
  fn hslSaturationF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor14hslSaturationFEv()};
    unsafe {_ZNK6QColor14hslSaturationFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setAlphaF<T: QColor_setAlphaF>(&mut self, value: T) -> i32 {
    value.setAlphaF(self);
    return 1;
  }
}

pub trait QColor_setAlphaF {
  fn setAlphaF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setAlphaF(qreal alpha);
impl<'a> /*trait*/ QColor_setAlphaF for (f64) {
  fn setAlphaF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9setAlphaFEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QColor9setAlphaFEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getRgb<T: QColor_getRgb>(&mut self, value: T) -> i32 {
    value.getRgb(self);
    return 1;
  }
}

pub trait QColor_getRgb {
  fn getRgb(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getRgb(int * r, int * g, int * b, int * a);
impl<'a> /*trait*/ QColor_getRgb for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getRgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6getRgbEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK6QColor6getRgbEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslHue<T: QColor_hslHue>(&mut self, value: T) -> i32 {
    value.hslHue(self);
    return 1;
  }
}

pub trait QColor_hslHue {
  fn hslHue(self, this: &mut QColor) -> i32;
}

// proto: int QColor::hslHue();
impl<'a> /*trait*/ QColor_hslHue for () {
  fn hslHue(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6hslHueEv()};
    unsafe {_ZNK6QColor6hslHueEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn lightness<T: QColor_lightness>(&mut self, value: T) -> i32 {
    value.lightness(self);
    return 1;
  }
}

pub trait QColor_lightness {
  fn lightness(self, this: &mut QColor) -> i32;
}

// proto: int QColor::lightness();
impl<'a> /*trait*/ QColor_lightness for () {
  fn lightness(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor9lightnessEv()};
    unsafe {_ZNK6QColor9lightnessEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setAlpha<T: QColor_setAlpha>(&mut self, value: T) -> i32 {
    value.setAlpha(self);
    return 1;
  }
}

pub trait QColor_setAlpha {
  fn setAlpha(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setAlpha(int alpha);
impl<'a> /*trait*/ QColor_setAlpha for (i32) {
  fn setAlpha(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setAlphaEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QColor8setAlphaEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHslF<T: QColor_fromHslF>(&mut self, value: T) -> i32 {
    value.fromHslF(self);
    return 1;
  }
}

pub trait QColor_fromHslF {
  fn fromHslF(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromHslF(qreal h, qreal s, qreal l, qreal a);
impl<'a> /*trait*/ QColor_fromHslF for (f64, f64, f64, f64) {
  fn fromHslF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromHslFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QColor8fromHslFEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getCmyk<T: QColor_getCmyk>(&mut self, value: T) -> i32 {
    value.getCmyk(self);
    return 1;
  }
}

pub trait QColor_getCmyk {
  fn getCmyk(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getCmyk(int * c, int * m, int * y, int * k, int * a);
impl<'a> /*trait*/ QColor_getCmyk for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getCmyk(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7getCmykEPiS0_S0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
    unsafe {_ZN6QColor7getCmykEPiS0_S0_S0_S0_(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn green<T: QColor_green>(&mut self, value: T) -> i32 {
    value.green(self);
    return 1;
  }
}

pub trait QColor_green {
  fn green(self, this: &mut QColor) -> i32;
}

// proto: int QColor::green();
impl<'a> /*trait*/ QColor_green for () {
  fn green(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5greenEv()};
    unsafe {_ZNK6QColor5greenEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvSaturation<T: QColor_hsvSaturation>(&mut self, value: T) -> i32 {
    value.hsvSaturation(self);
    return 1;
  }
}

pub trait QColor_hsvSaturation {
  fn hsvSaturation(self, this: &mut QColor) -> i32;
}

// proto: int QColor::hsvSaturation();
impl<'a> /*trait*/ QColor_hsvSaturation for () {
  fn hsvSaturation(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor13hsvSaturationEv()};
    unsafe {_ZNK6QColor13hsvSaturationEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toHsl<T: QColor_toHsl>(&mut self, value: T) -> i32 {
    value.toHsl(self);
    return 1;
  }
}

pub trait QColor_toHsl {
  fn toHsl(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::toHsl();
impl<'a> /*trait*/ QColor_toHsl for () {
  fn toHsl(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toHslEv()};
    unsafe {_ZNK6QColor5toHslEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn NewQColor<T: QColor_NewQColor>(value: T) -> QColor {
    let rsthis = value.NewQColor();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_NewQColor {
  fn NewQColor(self) -> QColor;
}

// proto: void QColor::NewQColor();
impl<'a> /*trait*/ QColor_NewQColor for () {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1Ev()};
    unsafe {_ZN6QColorC1Ev(qthis)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QColor::NewQColor(const char * name);
impl<'a> /*trait*/ QColor_NewQColor for (&'a  String) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN6QColorC1EPKc(qthis, arg0)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setBlue<T: QColor_setBlue>(&mut self, value: T) -> i32 {
    value.setBlue(self);
    return 1;
  }
}

pub trait QColor_setBlue {
  fn setBlue(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setBlue(int blue);
impl<'a> /*trait*/ QColor_setBlue for (i32) {
  fn setBlue(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setBlueEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QColor7setBlueEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn cyan<T: QColor_cyan>(&mut self, value: T) -> i32 {
    value.cyan(self);
    return 1;
  }
}

pub trait QColor_cyan {
  fn cyan(self, this: &mut QColor) -> i32;
}

// proto: int QColor::cyan();
impl<'a> /*trait*/ QColor_cyan for () {
  fn cyan(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4cyanEv()};
    unsafe {_ZNK6QColor4cyanEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setCmykF<T: QColor_setCmykF>(&mut self, value: T) -> i32 {
    value.setCmykF(self);
    return 1;
  }
}

pub trait QColor_setCmykF {
  fn setCmykF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl<'a> /*trait*/ QColor_setCmykF for (f64, f64, f64, f64, f64) {
  fn setCmykF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setCmykFEddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZN6QColor8setCmykFEddddd(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromCmykF<T: QColor_fromCmykF>(&mut self, value: T) -> i32 {
    value.fromCmykF(self);
    return 1;
  }
}

pub trait QColor_fromCmykF {
  fn fromCmykF(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl<'a> /*trait*/ QColor_fromCmykF for (f64, f64, f64, f64, f64) {
  fn fromCmykF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9fromCmykFEddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZN6QColor9fromCmykFEddddd(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn light<T: QColor_light>(&mut self, value: T) -> i32 {
    value.light(self);
    return 1;
  }
}

pub trait QColor_light {
  fn light(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::light(int f);
impl<'a> /*trait*/ QColor_light for (i32) {
  fn light(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5lightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QColor5lightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getHslF<T: QColor_getHslF>(&mut self, value: T) -> i32 {
    value.getHslF(self);
    return 1;
  }
}

pub trait QColor_getHslF {
  fn getHslF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getHslF(qreal * h, qreal * s, qreal * l, qreal * a);
impl<'a> /*trait*/ QColor_getHslF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getHslF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7getHslFEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK6QColor7getHslFEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromRgb<T: QColor_fromRgb>(&mut self, value: T) -> i32 {
    value.fromRgb(self);
    return 1;
  }
}

pub trait QColor_fromRgb {
  fn fromRgb(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromRgb(QRgb rgb);
impl<'a> /*trait*/ QColor_fromRgb for (u32) {
  fn fromRgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromRgbEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN6QColor7fromRgbEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn yellow<T: QColor_yellow>(&mut self, value: T) -> i32 {
    value.yellow(self);
    return 1;
  }
}

pub trait QColor_yellow {
  fn yellow(self, this: &mut QColor) -> i32;
}

// proto: int QColor::yellow();
impl<'a> /*trait*/ QColor_yellow for () {
  fn yellow(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6yellowEv()};
    unsafe {_ZNK6QColor6yellowEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getRgbF<T: QColor_getRgbF>(&mut self, value: T) -> i32 {
    value.getRgbF(self);
    return 1;
  }
}

pub trait QColor_getRgbF {
  fn getRgbF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getRgbF(qreal * r, qreal * g, qreal * b, qreal * a);
impl<'a> /*trait*/ QColor_getRgbF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getRgbF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7getRgbFEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK6QColor7getRgbFEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRgb<T: QColor_setRgb>(&mut self, value: T) -> i32 {
    value.setRgb(self);
    return 1;
  }
}

pub trait QColor_setRgb {
  fn setRgb(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setRgb(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_setRgb for (i32, i32, i32, i32) {
  fn setRgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRgbEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColor6setRgbEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn name<T: QColor_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QColor_name {
  fn name(self, this: &mut QColor) -> i32;
}

// proto: QString QColor::name();
impl<'a> /*trait*/ QColor_name for () {
  fn name(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4nameEv()};
    unsafe {_ZNK6QColor4nameEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn redF<T: QColor_redF>(&mut self, value: T) -> i32 {
    value.redF(self);
    return 1;
  }
}

pub trait QColor_redF {
  fn redF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::redF();
impl<'a> /*trait*/ QColor_redF for () {
  fn redF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4redFEv()};
    unsafe {_ZNK6QColor4redFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn blackF<T: QColor_blackF>(&mut self, value: T) -> i32 {
    value.blackF(self);
    return 1;
  }
}

pub trait QColor_blackF {
  fn blackF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::blackF();
impl<'a> /*trait*/ QColor_blackF for () {
  fn blackF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6blackFEv()};
    unsafe {_ZNK6QColor6blackFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHsvF<T: QColor_setHsvF>(&mut self, value: T) -> i32 {
    value.setHsvF(self);
    return 1;
  }
}

pub trait QColor_setHsvF {
  fn setHsvF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setHsvF(qreal h, qreal s, qreal v, qreal a);
impl<'a> /*trait*/ QColor_setHsvF for (f64, f64, f64, f64) {
  fn setHsvF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setHsvFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QColor7setHsvFEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QColor::setRgb(QRgb rgb);
impl<'a> /*trait*/ QColor_setRgb for (u32) {
  fn setRgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRgbEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN6QColor6setRgbEj(arg0)};
    return 1;
  }
}

// proto: QColor QColor::fromRgb(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_fromRgb for (i32, i32, i32, i32) {
  fn fromRgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromRgbEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColor7fromRgbEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvHueF<T: QColor_hsvHueF>(&mut self, value: T) -> i32 {
    value.hsvHueF(self);
    return 1;
  }
}

pub trait QColor_hsvHueF {
  fn hsvHueF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::hsvHueF();
impl<'a> /*trait*/ QColor_hsvHueF for () {
  fn hsvHueF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7hsvHueFEv()};
    unsafe {_ZNK6QColor7hsvHueFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvSaturationF<T: QColor_hsvSaturationF>(&mut self, value: T) -> i32 {
    value.hsvSaturationF(self);
    return 1;
  }
}

pub trait QColor_hsvSaturationF {
  fn hsvSaturationF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::hsvSaturationF();
impl<'a> /*trait*/ QColor_hsvSaturationF for () {
  fn hsvSaturationF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor14hsvSaturationFEv()};
    unsafe {_ZNK6QColor14hsvSaturationFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn yellowF<T: QColor_yellowF>(&mut self, value: T) -> i32 {
    value.yellowF(self);
    return 1;
  }
}

pub trait QColor_yellowF {
  fn yellowF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::yellowF();
impl<'a> /*trait*/ QColor_yellowF for () {
  fn yellowF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7yellowFEv()};
    unsafe {_ZNK6QColor7yellowFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn black<T: QColor_black>(&mut self, value: T) -> i32 {
    value.black(self);
    return 1;
  }
}

pub trait QColor_black {
  fn black(self, this: &mut QColor) -> i32;
}

// proto: int QColor::black();
impl<'a> /*trait*/ QColor_black for () {
  fn black(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5blackEv()};
    unsafe {_ZNK6QColor5blackEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setGreenF<T: QColor_setGreenF>(&mut self, value: T) -> i32 {
    value.setGreenF(self);
    return 1;
  }
}

pub trait QColor_setGreenF {
  fn setGreenF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setGreenF(qreal green);
impl<'a> /*trait*/ QColor_setGreenF for (f64) {
  fn setGreenF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9setGreenFEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QColor9setGreenFEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn rgba<T: QColor_rgba>(&mut self, value: T) -> i32 {
    value.rgba(self);
    return 1;
  }
}

pub trait QColor_rgba {
  fn rgba(self, this: &mut QColor) -> i32;
}

// proto: unsigned int QColor::rgba();
impl<'a> /*trait*/ QColor_rgba for () {
  fn rgba(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4rgbaEv()};
    unsafe {_ZNK6QColor4rgbaEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toCmyk<T: QColor_toCmyk>(&mut self, value: T) -> i32 {
    value.toCmyk(self);
    return 1;
  }
}

pub trait QColor_toCmyk {
  fn toCmyk(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::toCmyk();
impl<'a> /*trait*/ QColor_toCmyk for () {
  fn toCmyk(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6toCmykEv()};
    unsafe {_ZNK6QColor6toCmykEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn greenF<T: QColor_greenF>(&mut self, value: T) -> i32 {
    value.greenF(self);
    return 1;
  }
}

pub trait QColor_greenF {
  fn greenF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::greenF();
impl<'a> /*trait*/ QColor_greenF for () {
  fn greenF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6greenFEv()};
    unsafe {_ZNK6QColor6greenFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn red<T: QColor_red>(&mut self, value: T) -> i32 {
    value.red(self);
    return 1;
  }
}

pub trait QColor_red {
  fn red(self, this: &mut QColor) -> i32;
}

// proto: int QColor::red();
impl<'a> /*trait*/ QColor_red for () {
  fn red(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3redEv()};
    unsafe {_ZNK6QColor3redEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRgbF<T: QColor_setRgbF>(&mut self, value: T) -> i32 {
    value.setRgbF(self);
    return 1;
  }
}

pub trait QColor_setRgbF {
  fn setRgbF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setRgbF(qreal r, qreal g, qreal b, qreal a);
impl<'a> /*trait*/ QColor_setRgbF for (f64, f64, f64, f64) {
  fn setRgbF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRgbFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QColor7setRgbFEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn lightnessF<T: QColor_lightnessF>(&mut self, value: T) -> i32 {
    value.lightnessF(self);
    return 1;
  }
}

pub trait QColor_lightnessF {
  fn lightnessF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::lightnessF();
impl<'a> /*trait*/ QColor_lightnessF for () {
  fn lightnessF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor10lightnessFEv()};
    unsafe {_ZNK6QColor10lightnessFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toHsv<T: QColor_toHsv>(&mut self, value: T) -> i32 {
    value.toHsv(self);
    return 1;
  }
}

pub trait QColor_toHsv {
  fn toHsv(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::toHsv();
impl<'a> /*trait*/ QColor_toHsv for () {
  fn toHsv(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toHsvEv()};
    unsafe {_ZNK6QColor5toHsvEv()};
    return 1;
  }
}

// proto: void QColor::NewQColor(const QColor & color);
impl<'a> /*trait*/ QColor_NewQColor for (&'a  QColor) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QColorC1ERKS_(qthis, arg0)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHsv<T: QColor_fromHsv>(&mut self, value: T) -> i32 {
    value.fromHsv(self);
    return 1;
  }
}

pub trait QColor_fromHsv {
  fn fromHsv(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromHsv(int h, int s, int v, int a);
impl<'a> /*trait*/ QColor_fromHsv for (i32, i32, i32, i32) {
  fn fromHsv(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromHsvEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColor7fromHsvEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hueF<T: QColor_hueF>(&mut self, value: T) -> i32 {
    value.hueF(self);
    return 1;
  }
}

pub trait QColor_hueF {
  fn hueF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::hueF();
impl<'a> /*trait*/ QColor_hueF for () {
  fn hueF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4hueFEv()};
    unsafe {_ZNK6QColor4hueFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setBlueF<T: QColor_setBlueF>(&mut self, value: T) -> i32 {
    value.setBlueF(self);
    return 1;
  }
}

pub trait QColor_setBlueF {
  fn setBlueF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setBlueF(qreal blue);
impl<'a> /*trait*/ QColor_setBlueF for (f64) {
  fn setBlueF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setBlueFEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QColor8setBlueFEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn saturationF<T: QColor_saturationF>(&mut self, value: T) -> i32 {
    value.saturationF(self);
    return 1;
  }
}

pub trait QColor_saturationF {
  fn saturationF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::saturationF();
impl<'a> /*trait*/ QColor_saturationF for () {
  fn saturationF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor11saturationFEv()};
    unsafe {_ZNK6QColor11saturationFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn isValid<T: QColor_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QColor_isValid {
  fn isValid(self, this: &mut QColor) -> i32;
}

// proto: bool QColor::isValid();
impl<'a> /*trait*/ QColor_isValid for () {
  fn isValid(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7isValidEv()};
    unsafe {_ZNK6QColor7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn darker<T: QColor_darker>(&mut self, value: T) -> i32 {
    value.darker(self);
    return 1;
  }
}

pub trait QColor_darker {
  fn darker(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::darker(int f);
impl<'a> /*trait*/ QColor_darker for (i32) {
  fn darker(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6darkerEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QColor6darkerEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn blueF<T: QColor_blueF>(&mut self, value: T) -> i32 {
    value.blueF(self);
    return 1;
  }
}

pub trait QColor_blueF {
  fn blueF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::blueF();
impl<'a> /*trait*/ QColor_blueF for () {
  fn blueF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5blueFEv()};
    unsafe {_ZNK6QColor5blueFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hue<T: QColor_hue>(&mut self, value: T) -> i32 {
    value.hue(self);
    return 1;
  }
}

pub trait QColor_hue {
  fn hue(self, this: &mut QColor) -> i32;
}

// proto: int QColor::hue();
impl<'a> /*trait*/ QColor_hue for () {
  fn hue(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3hueEv()};
    unsafe {_ZNK6QColor3hueEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRgba<T: QColor_setRgba>(&mut self, value: T) -> i32 {
    value.setRgba(self);
    return 1;
  }
}

pub trait QColor_setRgba {
  fn setRgba(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setRgba(QRgb rgba);
impl<'a> /*trait*/ QColor_setRgba for (u32) {
  fn setRgba(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRgbaEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN6QColor7setRgbaEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setNamedColor<T: QColor_setNamedColor>(&mut self, value: T) -> i32 {
    value.setNamedColor(self);
    return 1;
  }
}

pub trait QColor_setNamedColor {
  fn setNamedColor(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setNamedColor(const QString & name);
impl<'a> /*trait*/ QColor_setNamedColor for (&'a  QString) {
  fn setNamedColor(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor13setNamedColorERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QColor13setNamedColorERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn magenta<T: QColor_magenta>(&mut self, value: T) -> i32 {
    value.magenta(self);
    return 1;
  }
}

pub trait QColor_magenta {
  fn magenta(self, this: &mut QColor) -> i32;
}

// proto: int QColor::magenta();
impl<'a> /*trait*/ QColor_magenta for () {
  fn magenta(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7magentaEv()};
    unsafe {_ZNK6QColor7magentaEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn lighter<T: QColor_lighter>(&mut self, value: T) -> i32 {
    value.lighter(self);
    return 1;
  }
}

pub trait QColor_lighter {
  fn lighter(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::lighter(int f);
impl<'a> /*trait*/ QColor_lighter for (i32) {
  fn lighter(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7lighterEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QColor7lighterEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toRgb<T: QColor_toRgb>(&mut self, value: T) -> i32 {
    value.toRgb(self);
    return 1;
  }
}

pub trait QColor_toRgb {
  fn toRgb(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::toRgb();
impl<'a> /*trait*/ QColor_toRgb for () {
  fn toRgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toRgbEv()};
    unsafe {_ZNK6QColor5toRgbEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn magentaF<T: QColor_magentaF>(&mut self, value: T) -> i32 {
    value.magentaF(self);
    return 1;
  }
}

pub trait QColor_magentaF {
  fn magentaF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::magentaF();
impl<'a> /*trait*/ QColor_magentaF for () {
  fn magentaF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor8magentaFEv()};
    unsafe {_ZNK6QColor8magentaFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslHueF<T: QColor_hslHueF>(&mut self, value: T) -> i32 {
    value.hslHueF(self);
    return 1;
  }
}

pub trait QColor_hslHueF {
  fn hslHueF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::hslHueF();
impl<'a> /*trait*/ QColor_hslHueF for () {
  fn hslHueF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7hslHueFEv()};
    unsafe {_ZNK6QColor7hslHueFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromCmyk<T: QColor_fromCmyk>(&mut self, value: T) -> i32 {
    value.fromCmyk(self);
    return 1;
  }
}

pub trait QColor_fromCmyk {
  fn fromCmyk(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromCmyk(int c, int m, int y, int k, int a);
impl<'a> /*trait*/ QColor_fromCmyk for (i32, i32, i32, i32, i32) {
  fn fromCmyk(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromCmykEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    unsafe {_ZN6QColor8fromCmykEiiiii(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setCmyk<T: QColor_setCmyk>(&mut self, value: T) -> i32 {
    value.setCmyk(self);
    return 1;
  }
}

pub trait QColor_setCmyk {
  fn setCmyk(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setCmyk(int c, int m, int y, int k, int a);
impl<'a> /*trait*/ QColor_setCmyk for (i32, i32, i32, i32, i32) {
  fn setCmyk(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setCmykEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    unsafe {_ZN6QColor7setCmykEiiiii(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn colorNames<T: QColor_colorNames>(&mut self, value: T) -> i32 {
    value.colorNames(self);
    return 1;
  }
}

pub trait QColor_colorNames {
  fn colorNames(self, this: &mut QColor) -> i32;
}

// proto: QStringList QColor::colorNames();
impl<'a> /*trait*/ QColor_colorNames for () {
  fn colorNames(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor10colorNamesEv()};
    unsafe {_ZN6QColor10colorNamesEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getHsv<T: QColor_getHsv>(&mut self, value: T) -> i32 {
    value.getHsv(self);
    return 1;
  }
}

pub trait QColor_getHsv {
  fn getHsv(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getHsv(int * h, int * s, int * v, int * a);
impl<'a> /*trait*/ QColor_getHsv for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getHsv(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6getHsvEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK6QColor6getHsvEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getCmykF<T: QColor_getCmykF>(&mut self, value: T) -> i32 {
    value.getCmykF(self);
    return 1;
  }
}

pub trait QColor_getCmykF {
  fn getCmykF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getCmykF(qreal * c, qreal * m, qreal * y, qreal * k, qreal * a);
impl<'a> /*trait*/ QColor_getCmykF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getCmykF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8getCmykFEPdS0_S0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    let arg4 = self.4  as *mut c_double;
    unsafe {_ZN6QColor8getCmykFEPdS0_S0_S0_S0_(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRed<T: QColor_setRed>(&mut self, value: T) -> i32 {
    value.setRed(self);
    return 1;
  }
}

pub trait QColor_setRed {
  fn setRed(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setRed(int red);
impl<'a> /*trait*/ QColor_setRed for (i32) {
  fn setRed(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QColor6setRedEi(arg0)};
    return 1;
  }
}

// proto: void QColor::NewQColor(const QString & name);
impl<'a> /*trait*/ QColor_NewQColor for (&'a  QString) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QColorC1ERK7QString(qthis, arg0)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromRgba<T: QColor_fromRgba>(&mut self, value: T) -> i32 {
    value.fromRgba(self);
    return 1;
  }
}

pub trait QColor_fromRgba {
  fn fromRgba(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromRgba(QRgb rgba);
impl<'a> /*trait*/ QColor_fromRgba for (u32) {
  fn fromRgba(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromRgbaEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN6QColor8fromRgbaEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHsv<T: QColor_setHsv>(&mut self, value: T) -> i32 {
    value.setHsv(self);
    return 1;
  }
}

pub trait QColor_setHsv {
  fn setHsv(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setHsv(int h, int s, int v, int a);
impl<'a> /*trait*/ QColor_setHsv for (i32, i32, i32, i32) {
  fn setHsv(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setHsvEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColor6setHsvEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn rgb<T: QColor_rgb>(&mut self, value: T) -> i32 {
    value.rgb(self);
    return 1;
  }
}

pub trait QColor_rgb {
  fn rgb(self, this: &mut QColor) -> i32;
}

// proto: unsigned int QColor::rgb();
impl<'a> /*trait*/ QColor_rgb for () {
  fn rgb(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3rgbEv()};
    unsafe {_ZNK6QColor3rgbEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHslF<T: QColor_setHslF>(&mut self, value: T) -> i32 {
    value.setHslF(self);
    return 1;
  }
}

pub trait QColor_setHslF {
  fn setHslF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setHslF(qreal h, qreal s, qreal l, qreal a);
impl<'a> /*trait*/ QColor_setHslF for (f64, f64, f64, f64) {
  fn setHslF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setHslFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QColor7setHslFEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn saturation<T: QColor_saturation>(&mut self, value: T) -> i32 {
    value.saturation(self);
    return 1;
  }
}

pub trait QColor_saturation {
  fn saturation(self, this: &mut QColor) -> i32;
}

// proto: int QColor::saturation();
impl<'a> /*trait*/ QColor_saturation for () {
  fn saturation(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor10saturationEv()};
    unsafe {_ZNK6QColor10saturationEv()};
    return 1;
  }
}

// proto: void QColor::NewQColor(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_NewQColor for (i32, i32, i32, i32) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColorC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn alphaF<T: QColor_alphaF>(&mut self, value: T) -> i32 {
    value.alphaF(self);
    return 1;
  }
}

pub trait QColor_alphaF {
  fn alphaF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::alphaF();
impl<'a> /*trait*/ QColor_alphaF for () {
  fn alphaF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6alphaFEv()};
    unsafe {_ZNK6QColor6alphaFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn value<T: QColor_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QColor_value {
  fn value(self, this: &mut QColor) -> i32;
}

// proto: int QColor::value();
impl<'a> /*trait*/ QColor_value for () {
  fn value(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5valueEv()};
    unsafe {_ZNK6QColor5valueEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHsvF<T: QColor_fromHsvF>(&mut self, value: T) -> i32 {
    value.fromHsvF(self);
    return 1;
  }
}

pub trait QColor_fromHsvF {
  fn fromHsvF(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromHsvF(qreal h, qreal s, qreal v, qreal a);
impl<'a> /*trait*/ QColor_fromHsvF for (f64, f64, f64, f64) {
  fn fromHsvF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromHsvFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QColor8fromHsvFEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn dark<T: QColor_dark>(&mut self, value: T) -> i32 {
    value.dark(self);
    return 1;
  }
}

pub trait QColor_dark {
  fn dark(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::dark(int f);
impl<'a> /*trait*/ QColor_dark for (i32) {
  fn dark(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4darkEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QColor4darkEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRedF<T: QColor_setRedF>(&mut self, value: T) -> i32 {
    value.setRedF(self);
    return 1;
  }
}

pub trait QColor_setRedF {
  fn setRedF(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setRedF(qreal red);
impl<'a> /*trait*/ QColor_setRedF for (f64) {
  fn setRedF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRedFEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QColor7setRedFEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHsl<T: QColor_fromHsl>(&mut self, value: T) -> i32 {
    value.fromHsl(self);
    return 1;
  }
}

pub trait QColor_fromHsl {
  fn fromHsl(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromHsl(int h, int s, int l, int a);
impl<'a> /*trait*/ QColor_fromHsl for (i32, i32, i32, i32) {
  fn fromHsl(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromHslEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColor7fromHslEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHsl<T: QColor_setHsl>(&mut self, value: T) -> i32 {
    value.setHsl(self);
    return 1;
  }
}

pub trait QColor_setHsl {
  fn setHsl(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setHsl(int h, int s, int l, int a);
impl<'a> /*trait*/ QColor_setHsl for (i32, i32, i32, i32) {
  fn setHsl(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setHslEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN6QColor6setHslEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QColor::NewQColor(QRgb rgb);
impl<'a> /*trait*/ QColor_NewQColor for (u32) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1Ej()};
    let arg0 = self  as c_uint;
    unsafe {_ZN6QColorC1Ej(qthis, arg0)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setGreen<T: QColor_setGreen>(&mut self, value: T) -> i32 {
    value.setGreen(self);
    return 1;
  }
}

pub trait QColor_setGreen {
  fn setGreen(self, this: &mut QColor) -> i32;
}

// proto: void QColor::setGreen(int green);
impl<'a> /*trait*/ QColor_setGreen for (i32) {
  fn setGreen(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setGreenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QColor8setGreenEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getHsl<T: QColor_getHsl>(&mut self, value: T) -> i32 {
    value.getHsl(self);
    return 1;
  }
}

pub trait QColor_getHsl {
  fn getHsl(self, this: &mut QColor) -> i32;
}

// proto: void QColor::getHsl(int * h, int * s, int * l, int * a);
impl<'a> /*trait*/ QColor_getHsl for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getHsl(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6getHslEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK6QColor6getHslEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn isValidColor<T: QColor_isValidColor>(&mut self, value: T) -> i32 {
    value.isValidColor(self);
    return 1;
  }
}

pub trait QColor_isValidColor {
  fn isValidColor(self, this: &mut QColor) -> i32;
}

// proto: bool QColor::isValidColor(const QString & name);
impl<'a> /*trait*/ QColor_isValidColor for (&'a  QString) {
  fn isValidColor(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor12isValidColorERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QColor12isValidColorERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslSaturation<T: QColor_hslSaturation>(&mut self, value: T) -> i32 {
    value.hslSaturation(self);
    return 1;
  }
}

pub trait QColor_hslSaturation {
  fn hslSaturation(self, this: &mut QColor) -> i32;
}

// proto: int QColor::hslSaturation();
impl<'a> /*trait*/ QColor_hslSaturation for () {
  fn hslSaturation(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor13hslSaturationEv()};
    unsafe {_ZNK6QColor13hslSaturationEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromRgbF<T: QColor_fromRgbF>(&mut self, value: T) -> i32 {
    value.fromRgbF(self);
    return 1;
  }
}

pub trait QColor_fromRgbF {
  fn fromRgbF(self, this: &mut QColor) -> i32;
}

// proto: QColor QColor::fromRgbF(qreal r, qreal g, qreal b, qreal a);
impl<'a> /*trait*/ QColor_fromRgbF for (f64, f64, f64, f64) {
  fn fromRgbF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromRgbFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QColor8fromRgbFEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn blue<T: QColor_blue>(&mut self, value: T) -> i32 {
    value.blue(self);
    return 1;
  }
}

pub trait QColor_blue {
  fn blue(self, this: &mut QColor) -> i32;
}

// proto: int QColor::blue();
impl<'a> /*trait*/ QColor_blue for () {
  fn blue(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4blueEv()};
    unsafe {_ZNK6QColor4blueEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvHue<T: QColor_hsvHue>(&mut self, value: T) -> i32 {
    value.hsvHue(self);
    return 1;
  }
}

pub trait QColor_hsvHue {
  fn hsvHue(self, this: &mut QColor) -> i32;
}

// proto: int QColor::hsvHue();
impl<'a> /*trait*/ QColor_hsvHue for () {
  fn hsvHue(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6hsvHueEv()};
    unsafe {_ZNK6QColor6hsvHueEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn valueF<T: QColor_valueF>(&mut self, value: T) -> i32 {
    value.valueF(self);
    return 1;
  }
}

pub trait QColor_valueF {
  fn valueF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::valueF();
impl<'a> /*trait*/ QColor_valueF for () {
  fn valueF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6valueFEv()};
    unsafe {_ZNK6QColor6valueFEv()};
    return 1;
  }
}

impl /*struct*/ QColor {
  pub fn cyanF<T: QColor_cyanF>(&mut self, value: T) -> i32 {
    value.cyanF(self);
    return 1;
  }
}

pub trait QColor_cyanF {
  fn cyanF(self, this: &mut QColor) -> i32;
}

// proto: double QColor::cyanF();
impl<'a> /*trait*/ QColor_cyanF for () {
  fn cyanF(self, this: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5cyanFEv()};
    unsafe {_ZNK6QColor5cyanFEv()};
    return 1;
  }
}

