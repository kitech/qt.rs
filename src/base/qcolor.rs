// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QColor::getHsvF(qreal * h, qreal * s, qreal * v, qreal * a);
  fn _ZNK6QColor7getHsvFEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  int QColor::alpha();
  fn _ZNK6QColor5alphaEv(qthis: *mut c_void) -> c_int;
  // proto:  double QColor::hslSaturationF();
  fn _ZNK6QColor14hslSaturationFEv(qthis: *mut c_void) -> c_double;
  // proto:  void QColor::setAlphaF(qreal alpha);
  fn _ZN6QColor9setAlphaFEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QColor::getRgb(int * r, int * g, int * b, int * a);
  fn _ZNK6QColor6getRgbEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  int QColor::hslHue();
  fn _ZNK6QColor6hslHueEv(qthis: *mut c_void) -> c_int;
  // proto:  int QColor::lightness();
  fn _ZNK6QColor9lightnessEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::setAlpha(int alpha);
  fn _ZN6QColor8setAlphaEi(qthis: *mut c_void, arg0: c_int) ;
  // proto: static QColor QColor::fromHslF(qreal h, qreal s, qreal l, qreal a);
  fn _ZN6QColor8fromHslFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QColor::getCmyk(int * c, int * m, int * y, int * k, int * a);
  fn _ZN6QColor7getCmykEPiS0_S0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int) ;
  // proto:  int QColor::green();
  fn _ZNK6QColor5greenEv(qthis: *mut c_void) -> c_int;
  // proto:  int QColor::hsvSaturation();
  fn _ZNK6QColor13hsvSaturationEv(qthis: *mut c_void) -> c_int;
  // proto:  QColor QColor::toHsl();
  fn _ZNK6QColor5toHslEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QColor::NewQColor();
  fn _ZN6QColorC1Ev(qthis: *mut c_void) ;
  // proto:  void QColor::NewQColor(const char * name);
  fn _ZN6QColorC1EPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QColor::setBlue(int blue);
  fn _ZN6QColor7setBlueEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QColor::cyan();
  fn _ZNK6QColor4cyanEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::setCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
  fn _ZN6QColor8setCmykFEddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) ;
  // proto: static QColor QColor::fromCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
  fn _ZN6QColor9fromCmykFEddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QColor QColor::light(int f);
  fn _ZNK6QColor5lightEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QColor::getHslF(qreal * h, qreal * s, qreal * l, qreal * a);
  fn _ZNK6QColor7getHslFEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto: static QColor QColor::fromRgb(QRgb rgb);
  fn _ZN6QColor7fromRgbEj(arg0: c_uint) -> *mut c_void;
  // proto:  int QColor::yellow();
  fn _ZNK6QColor6yellowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::getRgbF(qreal * r, qreal * g, qreal * b, qreal * a);
  fn _ZNK6QColor7getRgbFEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  void QColor::setRgb(int r, int g, int b, int a);
  fn _ZN6QColor6setRgbEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  QString QColor::name();
  fn _ZNK6QColor4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QColor::redF();
  fn _ZNK6QColor4redFEv(qthis: *mut c_void) -> c_double;
  // proto:  double QColor::blackF();
  fn _ZNK6QColor6blackFEv(qthis: *mut c_void) -> c_double;
  // proto:  void QColor::setHsvF(qreal h, qreal s, qreal v, qreal a);
  fn _ZN6QColor7setHsvFEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QColor::setRgb(QRgb rgb);
  fn _ZN6QColor6setRgbEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto: static QColor QColor::fromRgb(int r, int g, int b, int a);
  fn _ZN6QColor7fromRgbEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  double QColor::hsvHueF();
  fn _ZNK6QColor7hsvHueFEv(qthis: *mut c_void) -> c_double;
  // proto:  double QColor::hsvSaturationF();
  fn _ZNK6QColor14hsvSaturationFEv(qthis: *mut c_void) -> c_double;
  // proto:  double QColor::yellowF();
  fn _ZNK6QColor7yellowFEv(qthis: *mut c_void) -> c_double;
  // proto:  int QColor::black();
  fn _ZNK6QColor5blackEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::setGreenF(qreal green);
  fn _ZN6QColor9setGreenFEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  unsigned int QColor::rgba();
  fn _ZNK6QColor4rgbaEv(qthis: *mut c_void) -> c_uint;
  // proto:  QColor QColor::toCmyk();
  fn _ZNK6QColor6toCmykEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QColor::greenF();
  fn _ZNK6QColor6greenFEv(qthis: *mut c_void) -> c_double;
  // proto:  int QColor::red();
  fn _ZNK6QColor3redEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::setRgbF(qreal r, qreal g, qreal b, qreal a);
  fn _ZN6QColor7setRgbFEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  double QColor::lightnessF();
  fn _ZNK6QColor10lightnessFEv(qthis: *mut c_void) -> c_double;
  // proto:  QColor QColor::toHsv();
  fn _ZNK6QColor5toHsvEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QColor::NewQColor(const QColor & color);
  fn _ZN6QColorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QColor QColor::fromHsv(int h, int s, int v, int a);
  fn _ZN6QColor7fromHsvEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  double QColor::hueF();
  fn _ZNK6QColor4hueFEv(qthis: *mut c_void) -> c_double;
  // proto:  void QColor::setBlueF(qreal blue);
  fn _ZN6QColor8setBlueFEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QColor::saturationF();
  fn _ZNK6QColor11saturationFEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QColor::isValid();
  fn _ZNK6QColor7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QColor QColor::darker(int f);
  fn _ZNK6QColor6darkerEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  double QColor::blueF();
  fn _ZNK6QColor5blueFEv(qthis: *mut c_void) -> c_double;
  // proto:  int QColor::hue();
  fn _ZNK6QColor3hueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::setRgba(QRgb rgba);
  fn _ZN6QColor7setRgbaEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QColor::setNamedColor(const QString & name);
  fn _ZN6QColor13setNamedColorERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QColor::magenta();
  fn _ZNK6QColor7magentaEv(qthis: *mut c_void) -> c_int;
  // proto:  QColor QColor::lighter(int f);
  fn _ZNK6QColor7lighterEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QColor QColor::toRgb();
  fn _ZNK6QColor5toRgbEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QColor::magentaF();
  fn _ZNK6QColor8magentaFEv(qthis: *mut c_void) -> c_double;
  // proto:  double QColor::hslHueF();
  fn _ZNK6QColor7hslHueFEv(qthis: *mut c_void) -> c_double;
  // proto: static QColor QColor::fromCmyk(int c, int m, int y, int k, int a);
  fn _ZN6QColor8fromCmykEiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  void QColor::setCmyk(int c, int m, int y, int k, int a);
  fn _ZN6QColor7setCmykEiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) ;
  // proto: static QStringList QColor::colorNames();
  fn _ZN6QColor10colorNamesEv() -> *mut c_void;
  // proto:  void QColor::getHsv(int * h, int * s, int * v, int * a);
  fn _ZNK6QColor6getHsvEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  void QColor::getCmykF(qreal * c, qreal * m, qreal * y, qreal * k, qreal * a);
  fn _ZN6QColor8getCmykFEPdS0_S0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double, arg4: *mut c_double) ;
  // proto:  void QColor::setRed(int red);
  fn _ZN6QColor6setRedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QColor::NewQColor(const QString & name);
  fn _ZN6QColorC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QColor QColor::fromRgba(QRgb rgba);
  fn _ZN6QColor8fromRgbaEj(arg0: c_uint) -> *mut c_void;
  // proto:  void QColor::setHsv(int h, int s, int v, int a);
  fn _ZN6QColor6setHsvEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  unsigned int QColor::rgb();
  fn _ZNK6QColor3rgbEv(qthis: *mut c_void) -> c_uint;
  // proto:  void QColor::setHslF(qreal h, qreal s, qreal l, qreal a);
  fn _ZN6QColor7setHslFEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  int QColor::saturation();
  fn _ZNK6QColor10saturationEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColor::NewQColor(int r, int g, int b, int a);
  fn _ZN6QColorC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  double QColor::alphaF();
  fn _ZNK6QColor6alphaFEv(qthis: *mut c_void) -> c_double;
  // proto:  int QColor::value();
  fn _ZNK6QColor5valueEv(qthis: *mut c_void) -> c_int;
  // proto: static QColor QColor::fromHsvF(qreal h, qreal s, qreal v, qreal a);
  fn _ZN6QColor8fromHsvFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  QColor QColor::dark(int f);
  fn _ZNK6QColor4darkEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QColor::setRedF(qreal red);
  fn _ZN6QColor7setRedFEd(qthis: *mut c_void, arg0: c_double) ;
  // proto: static QColor QColor::fromHsl(int h, int s, int l, int a);
  fn _ZN6QColor7fromHslEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QColor::setHsl(int h, int s, int l, int a);
  fn _ZN6QColor6setHslEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QColor::NewQColor(QRgb rgb);
  fn _ZN6QColorC1Ej(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QColor::setGreen(int green);
  fn _ZN6QColor8setGreenEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QColor::getHsl(int * h, int * s, int * l, int * a);
  fn _ZNK6QColor6getHslEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto: static bool QColor::isValidColor(const QString & name);
  fn _ZN6QColor12isValidColorERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  int QColor::hslSaturation();
  fn _ZNK6QColor13hslSaturationEv(qthis: *mut c_void) -> c_int;
  // proto: static QColor QColor::fromRgbF(qreal r, qreal g, qreal b, qreal a);
  fn _ZN6QColor8fromRgbFEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  int QColor::blue();
  fn _ZNK6QColor4blueEv(qthis: *mut c_void) -> c_int;
  // proto:  int QColor::hsvHue();
  fn _ZNK6QColor6hsvHueEv(qthis: *mut c_void) -> c_int;
  // proto:  double QColor::valueF();
  fn _ZNK6QColor6valueFEv(qthis: *mut c_void) -> c_double;
  // proto:  double QColor::cyanF();
  fn _ZNK6QColor5cyanFEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QColor)=16
pub struct QColor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColor {
  pub fn getHsvF<T: QColor_getHsvF>(&mut self, value: T)  {
     value.getHsvF(self);
    // return 1;
  }
}

pub trait QColor_getHsvF {
  fn getHsvF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getHsvF(qreal * h, qreal * s, qreal * v, qreal * a);
impl<'a> /*trait*/ QColor_getHsvF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getHsvF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7getHsvFEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK6QColor7getHsvFEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn alpha<T: QColor_alpha>(&mut self, value: T) -> i32 {
    return value.alpha(self);
    // return 1;
  }
}

pub trait QColor_alpha {
  fn alpha(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::alpha();
impl<'a> /*trait*/ QColor_alpha for () {
  fn alpha(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5alphaEv()};
    let mut ret = unsafe {_ZNK6QColor5alphaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslSaturationF<T: QColor_hslSaturationF>(&mut self, value: T) -> f64 {
    return value.hslSaturationF(self);
    // return 1;
  }
}

pub trait QColor_hslSaturationF {
  fn hslSaturationF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::hslSaturationF();
impl<'a> /*trait*/ QColor_hslSaturationF for () {
  fn hslSaturationF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor14hslSaturationFEv()};
    let mut ret = unsafe {_ZNK6QColor14hslSaturationFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setAlphaF<T: QColor_setAlphaF>(&mut self, value: T)  {
     value.setAlphaF(self);
    // return 1;
  }
}

pub trait QColor_setAlphaF {
  fn setAlphaF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setAlphaF(qreal alpha);
impl<'a> /*trait*/ QColor_setAlphaF for (f64) {
  fn setAlphaF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9setAlphaFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor9setAlphaFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getRgb<T: QColor_getRgb>(&mut self, value: T)  {
     value.getRgb(self);
    // return 1;
  }
}

pub trait QColor_getRgb {
  fn getRgb(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getRgb(int * r, int * g, int * b, int * a);
impl<'a> /*trait*/ QColor_getRgb for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getRgb(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6getRgbEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK6QColor6getRgbEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslHue<T: QColor_hslHue>(&mut self, value: T) -> i32 {
    return value.hslHue(self);
    // return 1;
  }
}

pub trait QColor_hslHue {
  fn hslHue(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::hslHue();
impl<'a> /*trait*/ QColor_hslHue for () {
  fn hslHue(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6hslHueEv()};
    let mut ret = unsafe {_ZNK6QColor6hslHueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn lightness<T: QColor_lightness>(&mut self, value: T) -> i32 {
    return value.lightness(self);
    // return 1;
  }
}

pub trait QColor_lightness {
  fn lightness(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::lightness();
impl<'a> /*trait*/ QColor_lightness for () {
  fn lightness(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor9lightnessEv()};
    let mut ret = unsafe {_ZNK6QColor9lightnessEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setAlpha<T: QColor_setAlpha>(&mut self, value: T)  {
     value.setAlpha(self);
    // return 1;
  }
}

pub trait QColor_setAlpha {
  fn setAlpha(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setAlpha(int alpha);
impl<'a> /*trait*/ QColor_setAlpha for (i32) {
  fn setAlpha(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setAlphaEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor8setAlphaEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHslF<T: QColor_fromHslF>(&mut self, value: T) -> QColor {
    return value.fromHslF(self);
    // return 1;
  }
}

pub trait QColor_fromHslF {
  fn fromHslF(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromHslF(qreal h, qreal s, qreal l, qreal a);
impl<'a> /*trait*/ QColor_fromHslF for (f64, f64, f64, f64) {
  fn fromHslF(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromHslFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZN6QColor8fromHslFEdddd(arg0, arg1, arg2, arg3)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getCmyk<T: QColor_getCmyk>(&mut self, value: T)  {
     value.getCmyk(self);
    // return 1;
  }
}

pub trait QColor_getCmyk {
  fn getCmyk(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getCmyk(int * c, int * m, int * y, int * k, int * a);
impl<'a> /*trait*/ QColor_getCmyk for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getCmyk(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7getCmykEPiS0_S0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
     unsafe {_ZN6QColor7getCmykEPiS0_S0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn green<T: QColor_green>(&mut self, value: T) -> i32 {
    return value.green(self);
    // return 1;
  }
}

pub trait QColor_green {
  fn green(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::green();
impl<'a> /*trait*/ QColor_green for () {
  fn green(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5greenEv()};
    let mut ret = unsafe {_ZNK6QColor5greenEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvSaturation<T: QColor_hsvSaturation>(&mut self, value: T) -> i32 {
    return value.hsvSaturation(self);
    // return 1;
  }
}

pub trait QColor_hsvSaturation {
  fn hsvSaturation(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::hsvSaturation();
impl<'a> /*trait*/ QColor_hsvSaturation for () {
  fn hsvSaturation(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor13hsvSaturationEv()};
    let mut ret = unsafe {_ZNK6QColor13hsvSaturationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toHsl<T: QColor_toHsl>(&mut self, value: T) -> QColor {
    return value.toHsl(self);
    // return 1;
  }
}

pub trait QColor_toHsl {
  fn toHsl(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::toHsl();
impl<'a> /*trait*/ QColor_toHsl for () {
  fn toHsl(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toHslEv()};
    let mut ret = unsafe {_ZNK6QColor5toHslEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setBlue<T: QColor_setBlue>(&mut self, value: T)  {
     value.setBlue(self);
    // return 1;
  }
}

pub trait QColor_setBlue {
  fn setBlue(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setBlue(int blue);
impl<'a> /*trait*/ QColor_setBlue for (i32) {
  fn setBlue(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setBlueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor7setBlueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn cyan<T: QColor_cyan>(&mut self, value: T) -> i32 {
    return value.cyan(self);
    // return 1;
  }
}

pub trait QColor_cyan {
  fn cyan(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::cyan();
impl<'a> /*trait*/ QColor_cyan for () {
  fn cyan(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4cyanEv()};
    let mut ret = unsafe {_ZNK6QColor4cyanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setCmykF<T: QColor_setCmykF>(&mut self, value: T)  {
     value.setCmykF(self);
    // return 1;
  }
}

pub trait QColor_setCmykF {
  fn setCmykF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl<'a> /*trait*/ QColor_setCmykF for (f64, f64, f64, f64, f64) {
  fn setCmykF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setCmykFEddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
     unsafe {_ZN6QColor8setCmykFEddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromCmykF<T: QColor_fromCmykF>(&mut self, value: T) -> QColor {
    return value.fromCmykF(self);
    // return 1;
  }
}

pub trait QColor_fromCmykF {
  fn fromCmykF(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl<'a> /*trait*/ QColor_fromCmykF for (f64, f64, f64, f64, f64) {
  fn fromCmykF(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9fromCmykFEddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZN6QColor9fromCmykFEddddd(arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn light<T: QColor_light>(&mut self, value: T) -> QColor {
    return value.light(self);
    // return 1;
  }
}

pub trait QColor_light {
  fn light(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::light(int f);
impl<'a> /*trait*/ QColor_light for (i32) {
  fn light(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5lightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor5lightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getHslF<T: QColor_getHslF>(&mut self, value: T)  {
     value.getHslF(self);
    // return 1;
  }
}

pub trait QColor_getHslF {
  fn getHslF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getHslF(qreal * h, qreal * s, qreal * l, qreal * a);
impl<'a> /*trait*/ QColor_getHslF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getHslF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7getHslFEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK6QColor7getHslFEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromRgb<T: QColor_fromRgb>(&mut self, value: T) -> QColor {
    return value.fromRgb(self);
    // return 1;
  }
}

pub trait QColor_fromRgb {
  fn fromRgb(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromRgb(QRgb rgb);
impl<'a> /*trait*/ QColor_fromRgb for (u32) {
  fn fromRgb(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromRgbEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN6QColor7fromRgbEj(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn yellow<T: QColor_yellow>(&mut self, value: T) -> i32 {
    return value.yellow(self);
    // return 1;
  }
}

pub trait QColor_yellow {
  fn yellow(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::yellow();
impl<'a> /*trait*/ QColor_yellow for () {
  fn yellow(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6yellowEv()};
    let mut ret = unsafe {_ZNK6QColor6yellowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getRgbF<T: QColor_getRgbF>(&mut self, value: T)  {
     value.getRgbF(self);
    // return 1;
  }
}

pub trait QColor_getRgbF {
  fn getRgbF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getRgbF(qreal * r, qreal * g, qreal * b, qreal * a);
impl<'a> /*trait*/ QColor_getRgbF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getRgbF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7getRgbFEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK6QColor7getRgbFEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRgb<T: QColor_setRgb>(&mut self, value: T)  {
     value.setRgb(self);
    // return 1;
  }
}

pub trait QColor_setRgb {
  fn setRgb(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setRgb(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_setRgb for (i32, i32, i32, i32) {
  fn setRgb(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRgbEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN6QColor6setRgbEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn name<T: QColor_name>(&mut self, value: T) -> QString {
    return value.name(self);
    // return 1;
  }
}

pub trait QColor_name {
  fn name(self, rsthis: &mut QColor) -> QString;
}

// proto:  QString QColor::name();
impl<'a> /*trait*/ QColor_name for () {
  fn name(self, rsthis: &mut QColor) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4nameEv()};
    let mut ret = unsafe {_ZNK6QColor4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn redF<T: QColor_redF>(&mut self, value: T) -> f64 {
    return value.redF(self);
    // return 1;
  }
}

pub trait QColor_redF {
  fn redF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::redF();
impl<'a> /*trait*/ QColor_redF for () {
  fn redF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4redFEv()};
    let mut ret = unsafe {_ZNK6QColor4redFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn blackF<T: QColor_blackF>(&mut self, value: T) -> f64 {
    return value.blackF(self);
    // return 1;
  }
}

pub trait QColor_blackF {
  fn blackF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::blackF();
impl<'a> /*trait*/ QColor_blackF for () {
  fn blackF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6blackFEv()};
    let mut ret = unsafe {_ZNK6QColor6blackFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHsvF<T: QColor_setHsvF>(&mut self, value: T)  {
     value.setHsvF(self);
    // return 1;
  }
}

pub trait QColor_setHsvF {
  fn setHsvF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setHsvF(qreal h, qreal s, qreal v, qreal a);
impl<'a> /*trait*/ QColor_setHsvF for (f64, f64, f64, f64) {
  fn setHsvF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setHsvFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QColor7setHsvFEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QColor::setRgb(QRgb rgb);
impl<'a> /*trait*/ QColor_setRgb for (u32) {
  fn setRgb(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRgbEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QColor6setRgbEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QColor QColor::fromRgb(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_fromRgb for (i32, i32, i32, i32) {
  fn fromRgb(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromRgbEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN6QColor7fromRgbEiiii(arg0, arg1, arg2, arg3)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvHueF<T: QColor_hsvHueF>(&mut self, value: T) -> f64 {
    return value.hsvHueF(self);
    // return 1;
  }
}

pub trait QColor_hsvHueF {
  fn hsvHueF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::hsvHueF();
impl<'a> /*trait*/ QColor_hsvHueF for () {
  fn hsvHueF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7hsvHueFEv()};
    let mut ret = unsafe {_ZNK6QColor7hsvHueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvSaturationF<T: QColor_hsvSaturationF>(&mut self, value: T) -> f64 {
    return value.hsvSaturationF(self);
    // return 1;
  }
}

pub trait QColor_hsvSaturationF {
  fn hsvSaturationF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::hsvSaturationF();
impl<'a> /*trait*/ QColor_hsvSaturationF for () {
  fn hsvSaturationF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor14hsvSaturationFEv()};
    let mut ret = unsafe {_ZNK6QColor14hsvSaturationFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn yellowF<T: QColor_yellowF>(&mut self, value: T) -> f64 {
    return value.yellowF(self);
    // return 1;
  }
}

pub trait QColor_yellowF {
  fn yellowF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::yellowF();
impl<'a> /*trait*/ QColor_yellowF for () {
  fn yellowF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7yellowFEv()};
    let mut ret = unsafe {_ZNK6QColor7yellowFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn black<T: QColor_black>(&mut self, value: T) -> i32 {
    return value.black(self);
    // return 1;
  }
}

pub trait QColor_black {
  fn black(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::black();
impl<'a> /*trait*/ QColor_black for () {
  fn black(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5blackEv()};
    let mut ret = unsafe {_ZNK6QColor5blackEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setGreenF<T: QColor_setGreenF>(&mut self, value: T)  {
     value.setGreenF(self);
    // return 1;
  }
}

pub trait QColor_setGreenF {
  fn setGreenF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setGreenF(qreal green);
impl<'a> /*trait*/ QColor_setGreenF for (f64) {
  fn setGreenF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9setGreenFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor9setGreenFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn rgba<T: QColor_rgba>(&mut self, value: T) -> u32 {
    return value.rgba(self);
    // return 1;
  }
}

pub trait QColor_rgba {
  fn rgba(self, rsthis: &mut QColor) -> u32;
}

// proto:  unsigned int QColor::rgba();
impl<'a> /*trait*/ QColor_rgba for () {
  fn rgba(self, rsthis: &mut QColor) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4rgbaEv()};
    let mut ret = unsafe {_ZNK6QColor4rgbaEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toCmyk<T: QColor_toCmyk>(&mut self, value: T) -> QColor {
    return value.toCmyk(self);
    // return 1;
  }
}

pub trait QColor_toCmyk {
  fn toCmyk(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::toCmyk();
impl<'a> /*trait*/ QColor_toCmyk for () {
  fn toCmyk(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6toCmykEv()};
    let mut ret = unsafe {_ZNK6QColor6toCmykEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn greenF<T: QColor_greenF>(&mut self, value: T) -> f64 {
    return value.greenF(self);
    // return 1;
  }
}

pub trait QColor_greenF {
  fn greenF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::greenF();
impl<'a> /*trait*/ QColor_greenF for () {
  fn greenF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6greenFEv()};
    let mut ret = unsafe {_ZNK6QColor6greenFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn red<T: QColor_red>(&mut self, value: T) -> i32 {
    return value.red(self);
    // return 1;
  }
}

pub trait QColor_red {
  fn red(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::red();
impl<'a> /*trait*/ QColor_red for () {
  fn red(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3redEv()};
    let mut ret = unsafe {_ZNK6QColor3redEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRgbF<T: QColor_setRgbF>(&mut self, value: T)  {
     value.setRgbF(self);
    // return 1;
  }
}

pub trait QColor_setRgbF {
  fn setRgbF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setRgbF(qreal r, qreal g, qreal b, qreal a);
impl<'a> /*trait*/ QColor_setRgbF for (f64, f64, f64, f64) {
  fn setRgbF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRgbFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QColor7setRgbFEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn lightnessF<T: QColor_lightnessF>(&mut self, value: T) -> f64 {
    return value.lightnessF(self);
    // return 1;
  }
}

pub trait QColor_lightnessF {
  fn lightnessF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::lightnessF();
impl<'a> /*trait*/ QColor_lightnessF for () {
  fn lightnessF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor10lightnessFEv()};
    let mut ret = unsafe {_ZNK6QColor10lightnessFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toHsv<T: QColor_toHsv>(&mut self, value: T) -> QColor {
    return value.toHsv(self);
    // return 1;
  }
}

pub trait QColor_toHsv {
  fn toHsv(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::toHsv();
impl<'a> /*trait*/ QColor_toHsv for () {
  fn toHsv(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toHsvEv()};
    let mut ret = unsafe {_ZNK6QColor5toHsvEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QColor::NewQColor(const QColor & color);
impl<'a> /*trait*/ QColor_NewQColor for (&'a  QColor) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QColorC1ERKS_(qthis, arg0)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHsv<T: QColor_fromHsv>(&mut self, value: T) -> QColor {
    return value.fromHsv(self);
    // return 1;
  }
}

pub trait QColor_fromHsv {
  fn fromHsv(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromHsv(int h, int s, int v, int a);
impl<'a> /*trait*/ QColor_fromHsv for (i32, i32, i32, i32) {
  fn fromHsv(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromHsvEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN6QColor7fromHsvEiiii(arg0, arg1, arg2, arg3)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hueF<T: QColor_hueF>(&mut self, value: T) -> f64 {
    return value.hueF(self);
    // return 1;
  }
}

pub trait QColor_hueF {
  fn hueF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::hueF();
impl<'a> /*trait*/ QColor_hueF for () {
  fn hueF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4hueFEv()};
    let mut ret = unsafe {_ZNK6QColor4hueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setBlueF<T: QColor_setBlueF>(&mut self, value: T)  {
     value.setBlueF(self);
    // return 1;
  }
}

pub trait QColor_setBlueF {
  fn setBlueF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setBlueF(qreal blue);
impl<'a> /*trait*/ QColor_setBlueF for (f64) {
  fn setBlueF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setBlueFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor8setBlueFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn saturationF<T: QColor_saturationF>(&mut self, value: T) -> f64 {
    return value.saturationF(self);
    // return 1;
  }
}

pub trait QColor_saturationF {
  fn saturationF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::saturationF();
impl<'a> /*trait*/ QColor_saturationF for () {
  fn saturationF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor11saturationFEv()};
    let mut ret = unsafe {_ZNK6QColor11saturationFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn isValid<T: QColor_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QColor_isValid {
  fn isValid(self, rsthis: &mut QColor) -> i8;
}

// proto:  bool QColor::isValid();
impl<'a> /*trait*/ QColor_isValid for () {
  fn isValid(self, rsthis: &mut QColor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7isValidEv()};
    let mut ret = unsafe {_ZNK6QColor7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn darker<T: QColor_darker>(&mut self, value: T) -> QColor {
    return value.darker(self);
    // return 1;
  }
}

pub trait QColor_darker {
  fn darker(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::darker(int f);
impl<'a> /*trait*/ QColor_darker for (i32) {
  fn darker(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6darkerEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor6darkerEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn blueF<T: QColor_blueF>(&mut self, value: T) -> f64 {
    return value.blueF(self);
    // return 1;
  }
}

pub trait QColor_blueF {
  fn blueF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::blueF();
impl<'a> /*trait*/ QColor_blueF for () {
  fn blueF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5blueFEv()};
    let mut ret = unsafe {_ZNK6QColor5blueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hue<T: QColor_hue>(&mut self, value: T) -> i32 {
    return value.hue(self);
    // return 1;
  }
}

pub trait QColor_hue {
  fn hue(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::hue();
impl<'a> /*trait*/ QColor_hue for () {
  fn hue(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3hueEv()};
    let mut ret = unsafe {_ZNK6QColor3hueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRgba<T: QColor_setRgba>(&mut self, value: T)  {
     value.setRgba(self);
    // return 1;
  }
}

pub trait QColor_setRgba {
  fn setRgba(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setRgba(QRgb rgba);
impl<'a> /*trait*/ QColor_setRgba for (u32) {
  fn setRgba(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRgbaEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QColor7setRgbaEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setNamedColor<T: QColor_setNamedColor>(&mut self, value: T)  {
     value.setNamedColor(self);
    // return 1;
  }
}

pub trait QColor_setNamedColor {
  fn setNamedColor(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setNamedColor(const QString & name);
impl<'a> /*trait*/ QColor_setNamedColor for (&'a  QString) {
  fn setNamedColor(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor13setNamedColorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QColor13setNamedColorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn magenta<T: QColor_magenta>(&mut self, value: T) -> i32 {
    return value.magenta(self);
    // return 1;
  }
}

pub trait QColor_magenta {
  fn magenta(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::magenta();
impl<'a> /*trait*/ QColor_magenta for () {
  fn magenta(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7magentaEv()};
    let mut ret = unsafe {_ZNK6QColor7magentaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn lighter<T: QColor_lighter>(&mut self, value: T) -> QColor {
    return value.lighter(self);
    // return 1;
  }
}

pub trait QColor_lighter {
  fn lighter(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::lighter(int f);
impl<'a> /*trait*/ QColor_lighter for (i32) {
  fn lighter(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7lighterEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor7lighterEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn toRgb<T: QColor_toRgb>(&mut self, value: T) -> QColor {
    return value.toRgb(self);
    // return 1;
  }
}

pub trait QColor_toRgb {
  fn toRgb(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::toRgb();
impl<'a> /*trait*/ QColor_toRgb for () {
  fn toRgb(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toRgbEv()};
    let mut ret = unsafe {_ZNK6QColor5toRgbEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn magentaF<T: QColor_magentaF>(&mut self, value: T) -> f64 {
    return value.magentaF(self);
    // return 1;
  }
}

pub trait QColor_magentaF {
  fn magentaF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::magentaF();
impl<'a> /*trait*/ QColor_magentaF for () {
  fn magentaF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor8magentaFEv()};
    let mut ret = unsafe {_ZNK6QColor8magentaFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslHueF<T: QColor_hslHueF>(&mut self, value: T) -> f64 {
    return value.hslHueF(self);
    // return 1;
  }
}

pub trait QColor_hslHueF {
  fn hslHueF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::hslHueF();
impl<'a> /*trait*/ QColor_hslHueF for () {
  fn hslHueF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7hslHueFEv()};
    let mut ret = unsafe {_ZNK6QColor7hslHueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromCmyk<T: QColor_fromCmyk>(&mut self, value: T) -> QColor {
    return value.fromCmyk(self);
    // return 1;
  }
}

pub trait QColor_fromCmyk {
  fn fromCmyk(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromCmyk(int c, int m, int y, int k, int a);
impl<'a> /*trait*/ QColor_fromCmyk for (i32, i32, i32, i32, i32) {
  fn fromCmyk(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromCmykEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN6QColor8fromCmykEiiiii(arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setCmyk<T: QColor_setCmyk>(&mut self, value: T)  {
     value.setCmyk(self);
    // return 1;
  }
}

pub trait QColor_setCmyk {
  fn setCmyk(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setCmyk(int c, int m, int y, int k, int a);
impl<'a> /*trait*/ QColor_setCmyk for (i32, i32, i32, i32, i32) {
  fn setCmyk(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setCmykEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN6QColor7setCmykEiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn colorNames<T: QColor_colorNames>(&mut self, value: T) -> QStringList {
    return value.colorNames(self);
    // return 1;
  }
}

pub trait QColor_colorNames {
  fn colorNames(self, rsthis: &mut QColor) -> QStringList;
}

// proto: static QStringList QColor::colorNames();
impl<'a> /*trait*/ QColor_colorNames for () {
  fn colorNames(self, rsthis: &mut QColor) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor10colorNamesEv()};
    let mut ret = unsafe {_ZN6QColor10colorNamesEv()};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getHsv<T: QColor_getHsv>(&mut self, value: T)  {
     value.getHsv(self);
    // return 1;
  }
}

pub trait QColor_getHsv {
  fn getHsv(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getHsv(int * h, int * s, int * v, int * a);
impl<'a> /*trait*/ QColor_getHsv for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getHsv(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6getHsvEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK6QColor6getHsvEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getCmykF<T: QColor_getCmykF>(&mut self, value: T)  {
     value.getCmykF(self);
    // return 1;
  }
}

pub trait QColor_getCmykF {
  fn getCmykF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getCmykF(qreal * c, qreal * m, qreal * y, qreal * k, qreal * a);
impl<'a> /*trait*/ QColor_getCmykF for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getCmykF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8getCmykFEPdS0_S0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    let arg4 = self.4  as *mut c_double;
     unsafe {_ZN6QColor8getCmykFEPdS0_S0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRed<T: QColor_setRed>(&mut self, value: T)  {
     value.setRed(self);
    // return 1;
  }
}

pub trait QColor_setRed {
  fn setRed(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setRed(int red);
impl<'a> /*trait*/ QColor_setRed for (i32) {
  fn setRed(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor6setRedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QColor::NewQColor(const QString & name);
impl<'a> /*trait*/ QColor_NewQColor for (&'a  QString) {
  fn NewQColor(self) -> QColor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColorC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QColorC1ERK7QString(qthis, arg0)};
    let rsthis = QColor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromRgba<T: QColor_fromRgba>(&mut self, value: T) -> QColor {
    return value.fromRgba(self);
    // return 1;
  }
}

pub trait QColor_fromRgba {
  fn fromRgba(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromRgba(QRgb rgba);
impl<'a> /*trait*/ QColor_fromRgba for (u32) {
  fn fromRgba(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromRgbaEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN6QColor8fromRgbaEj(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHsv<T: QColor_setHsv>(&mut self, value: T)  {
     value.setHsv(self);
    // return 1;
  }
}

pub trait QColor_setHsv {
  fn setHsv(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setHsv(int h, int s, int v, int a);
impl<'a> /*trait*/ QColor_setHsv for (i32, i32, i32, i32) {
  fn setHsv(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setHsvEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN6QColor6setHsvEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn rgb<T: QColor_rgb>(&mut self, value: T) -> u32 {
    return value.rgb(self);
    // return 1;
  }
}

pub trait QColor_rgb {
  fn rgb(self, rsthis: &mut QColor) -> u32;
}

// proto:  unsigned int QColor::rgb();
impl<'a> /*trait*/ QColor_rgb for () {
  fn rgb(self, rsthis: &mut QColor) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3rgbEv()};
    let mut ret = unsafe {_ZNK6QColor3rgbEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHslF<T: QColor_setHslF>(&mut self, value: T)  {
     value.setHslF(self);
    // return 1;
  }
}

pub trait QColor_setHslF {
  fn setHslF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setHslF(qreal h, qreal s, qreal l, qreal a);
impl<'a> /*trait*/ QColor_setHslF for (f64, f64, f64, f64) {
  fn setHslF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setHslFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QColor7setHslFEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn saturation<T: QColor_saturation>(&mut self, value: T) -> i32 {
    return value.saturation(self);
    // return 1;
  }
}

pub trait QColor_saturation {
  fn saturation(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::saturation();
impl<'a> /*trait*/ QColor_saturation for () {
  fn saturation(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor10saturationEv()};
    let mut ret = unsafe {_ZNK6QColor10saturationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
  pub fn alphaF<T: QColor_alphaF>(&mut self, value: T) -> f64 {
    return value.alphaF(self);
    // return 1;
  }
}

pub trait QColor_alphaF {
  fn alphaF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::alphaF();
impl<'a> /*trait*/ QColor_alphaF for () {
  fn alphaF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6alphaFEv()};
    let mut ret = unsafe {_ZNK6QColor6alphaFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn value<T: QColor_value>(&mut self, value: T) -> i32 {
    return value.value(self);
    // return 1;
  }
}

pub trait QColor_value {
  fn value(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::value();
impl<'a> /*trait*/ QColor_value for () {
  fn value(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5valueEv()};
    let mut ret = unsafe {_ZNK6QColor5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHsvF<T: QColor_fromHsvF>(&mut self, value: T) -> QColor {
    return value.fromHsvF(self);
    // return 1;
  }
}

pub trait QColor_fromHsvF {
  fn fromHsvF(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromHsvF(qreal h, qreal s, qreal v, qreal a);
impl<'a> /*trait*/ QColor_fromHsvF for (f64, f64, f64, f64) {
  fn fromHsvF(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromHsvFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZN6QColor8fromHsvFEdddd(arg0, arg1, arg2, arg3)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn dark<T: QColor_dark>(&mut self, value: T) -> QColor {
    return value.dark(self);
    // return 1;
  }
}

pub trait QColor_dark {
  fn dark(self, rsthis: &mut QColor) -> QColor;
}

// proto:  QColor QColor::dark(int f);
impl<'a> /*trait*/ QColor_dark for (i32) {
  fn dark(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4darkEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor4darkEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setRedF<T: QColor_setRedF>(&mut self, value: T)  {
     value.setRedF(self);
    // return 1;
  }
}

pub trait QColor_setRedF {
  fn setRedF(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setRedF(qreal red);
impl<'a> /*trait*/ QColor_setRedF for (f64) {
  fn setRedF(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRedFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor7setRedFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromHsl<T: QColor_fromHsl>(&mut self, value: T) -> QColor {
    return value.fromHsl(self);
    // return 1;
  }
}

pub trait QColor_fromHsl {
  fn fromHsl(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromHsl(int h, int s, int l, int a);
impl<'a> /*trait*/ QColor_fromHsl for (i32, i32, i32, i32) {
  fn fromHsl(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromHslEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN6QColor7fromHslEiiii(arg0, arg1, arg2, arg3)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn setHsl<T: QColor_setHsl>(&mut self, value: T)  {
     value.setHsl(self);
    // return 1;
  }
}

pub trait QColor_setHsl {
  fn setHsl(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setHsl(int h, int s, int l, int a);
impl<'a> /*trait*/ QColor_setHsl for (i32, i32, i32, i32) {
  fn setHsl(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setHslEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN6QColor6setHslEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
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
  pub fn setGreen<T: QColor_setGreen>(&mut self, value: T)  {
     value.setGreen(self);
    // return 1;
  }
}

pub trait QColor_setGreen {
  fn setGreen(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::setGreen(int green);
impl<'a> /*trait*/ QColor_setGreen for (i32) {
  fn setGreen(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setGreenEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor8setGreenEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn getHsl<T: QColor_getHsl>(&mut self, value: T)  {
     value.getHsl(self);
    // return 1;
  }
}

pub trait QColor_getHsl {
  fn getHsl(self, rsthis: &mut QColor) ;
}

// proto:  void QColor::getHsl(int * h, int * s, int * l, int * a);
impl<'a> /*trait*/ QColor_getHsl for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getHsl(self, rsthis: &mut QColor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6getHslEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK6QColor6getHslEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn isValidColor<T: QColor_isValidColor>(&mut self, value: T) -> i8 {
    return value.isValidColor(self);
    // return 1;
  }
}

pub trait QColor_isValidColor {
  fn isValidColor(self, rsthis: &mut QColor) -> i8;
}

// proto: static bool QColor::isValidColor(const QString & name);
impl<'a> /*trait*/ QColor_isValidColor for (&'a  QString) {
  fn isValidColor(self, rsthis: &mut QColor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor12isValidColorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN6QColor12isValidColorERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hslSaturation<T: QColor_hslSaturation>(&mut self, value: T) -> i32 {
    return value.hslSaturation(self);
    // return 1;
  }
}

pub trait QColor_hslSaturation {
  fn hslSaturation(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::hslSaturation();
impl<'a> /*trait*/ QColor_hslSaturation for () {
  fn hslSaturation(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor13hslSaturationEv()};
    let mut ret = unsafe {_ZNK6QColor13hslSaturationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn fromRgbF<T: QColor_fromRgbF>(&mut self, value: T) -> QColor {
    return value.fromRgbF(self);
    // return 1;
  }
}

pub trait QColor_fromRgbF {
  fn fromRgbF(self, rsthis: &mut QColor) -> QColor;
}

// proto: static QColor QColor::fromRgbF(qreal r, qreal g, qreal b, qreal a);
impl<'a> /*trait*/ QColor_fromRgbF for (f64, f64, f64, f64) {
  fn fromRgbF(self, rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromRgbFEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZN6QColor8fromRgbFEdddd(arg0, arg1, arg2, arg3)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn blue<T: QColor_blue>(&mut self, value: T) -> i32 {
    return value.blue(self);
    // return 1;
  }
}

pub trait QColor_blue {
  fn blue(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::blue();
impl<'a> /*trait*/ QColor_blue for () {
  fn blue(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4blueEv()};
    let mut ret = unsafe {_ZNK6QColor4blueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn hsvHue<T: QColor_hsvHue>(&mut self, value: T) -> i32 {
    return value.hsvHue(self);
    // return 1;
  }
}

pub trait QColor_hsvHue {
  fn hsvHue(self, rsthis: &mut QColor) -> i32;
}

// proto:  int QColor::hsvHue();
impl<'a> /*trait*/ QColor_hsvHue for () {
  fn hsvHue(self, rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6hsvHueEv()};
    let mut ret = unsafe {_ZNK6QColor6hsvHueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn valueF<T: QColor_valueF>(&mut self, value: T) -> f64 {
    return value.valueF(self);
    // return 1;
  }
}

pub trait QColor_valueF {
  fn valueF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::valueF();
impl<'a> /*trait*/ QColor_valueF for () {
  fn valueF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6valueFEv()};
    let mut ret = unsafe {_ZNK6QColor6valueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QColor {
  pub fn cyanF<T: QColor_cyanF>(&mut self, value: T) -> f64 {
    return value.cyanF(self);
    // return 1;
  }
}

pub trait QColor_cyanF {
  fn cyanF(self, rsthis: &mut QColor) -> f64;
}

// proto:  double QColor::cyanF();
impl<'a> /*trait*/ QColor_cyanF for () {
  fn cyanF(self, rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5cyanFEv()};
    let mut ret = unsafe {_ZNK6QColor5cyanFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

