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
  fn _ZN6QColor10colorNamesEv() ;
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

// proto:  void QColor::getHsvF(qreal * h, qreal * s, qreal * v, qreal * a);
impl /*struct*/ QColor {
  pub fn getHsvF<RetType, T: QColor_getHsvF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getHsvF(self);
    // return 1;
  }
}

pub trait QColor_getHsvF<RetType> {
  fn getHsvF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getHsvF(qreal * h, qreal * s, qreal * v, qreal * a);
impl<'a> /*trait*/ QColor_getHsvF<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getHsvF(self , rsthis: &mut QColor) -> () {
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

// proto:  int QColor::alpha();
impl /*struct*/ QColor {
  pub fn alpha<RetType, T: QColor_alpha<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.alpha(self);
    // return 1;
  }
}

pub trait QColor_alpha<RetType> {
  fn alpha(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::alpha();
impl<'a> /*trait*/ QColor_alpha<i32> for () {
  fn alpha(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5alphaEv()};
    let mut ret = unsafe {_ZNK6QColor5alphaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  double QColor::hslSaturationF();
impl /*struct*/ QColor {
  pub fn hslSaturationF<RetType, T: QColor_hslSaturationF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hslSaturationF(self);
    // return 1;
  }
}

pub trait QColor_hslSaturationF<RetType> {
  fn hslSaturationF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::hslSaturationF();
impl<'a> /*trait*/ QColor_hslSaturationF<f64> for () {
  fn hslSaturationF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor14hslSaturationFEv()};
    let mut ret = unsafe {_ZNK6QColor14hslSaturationFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QColor::setAlphaF(qreal alpha);
impl /*struct*/ QColor {
  pub fn setAlphaF<RetType, T: QColor_setAlphaF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAlphaF(self);
    // return 1;
  }
}

pub trait QColor_setAlphaF<RetType> {
  fn setAlphaF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setAlphaF(qreal alpha);
impl<'a> /*trait*/ QColor_setAlphaF<()> for (f64) {
  fn setAlphaF(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9setAlphaFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor9setAlphaFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QColor::getRgb(int * r, int * g, int * b, int * a);
impl /*struct*/ QColor {
  pub fn getRgb<RetType, T: QColor_getRgb<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getRgb(self);
    // return 1;
  }
}

pub trait QColor_getRgb<RetType> {
  fn getRgb(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getRgb(int * r, int * g, int * b, int * a);
impl<'a> /*trait*/ QColor_getRgb<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getRgb(self , rsthis: &mut QColor) -> () {
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

// proto:  int QColor::hslHue();
impl /*struct*/ QColor {
  pub fn hslHue<RetType, T: QColor_hslHue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hslHue(self);
    // return 1;
  }
}

pub trait QColor_hslHue<RetType> {
  fn hslHue(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::hslHue();
impl<'a> /*trait*/ QColor_hslHue<i32> for () {
  fn hslHue(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6hslHueEv()};
    let mut ret = unsafe {_ZNK6QColor6hslHueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QColor::lightness();
impl /*struct*/ QColor {
  pub fn lightness<RetType, T: QColor_lightness<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lightness(self);
    // return 1;
  }
}

pub trait QColor_lightness<RetType> {
  fn lightness(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::lightness();
impl<'a> /*trait*/ QColor_lightness<i32> for () {
  fn lightness(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor9lightnessEv()};
    let mut ret = unsafe {_ZNK6QColor9lightnessEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QColor::setAlpha(int alpha);
impl /*struct*/ QColor {
  pub fn setAlpha<RetType, T: QColor_setAlpha<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAlpha(self);
    // return 1;
  }
}

pub trait QColor_setAlpha<RetType> {
  fn setAlpha(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setAlpha(int alpha);
impl<'a> /*trait*/ QColor_setAlpha<()> for (i32) {
  fn setAlpha(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setAlphaEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor8setAlphaEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QColor QColor::fromHslF(qreal h, qreal s, qreal l, qreal a);
impl /*struct*/ QColor {
  pub fn fromHslF_s<RetType, T: QColor_fromHslF_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHslF_s();
    // return 1;
  }
}

pub trait QColor_fromHslF_s<RetType> {
  fn fromHslF_s(self ) -> RetType;
}

// proto: static QColor QColor::fromHslF(qreal h, qreal s, qreal l, qreal a);
impl<'a> /*trait*/ QColor_fromHslF_s<QColor> for (f64, f64, f64, f64) {
  fn fromHslF_s(self ) -> QColor {
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

// proto:  void QColor::getCmyk(int * c, int * m, int * y, int * k, int * a);
impl /*struct*/ QColor {
  pub fn getCmyk<RetType, T: QColor_getCmyk<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getCmyk(self);
    // return 1;
  }
}

pub trait QColor_getCmyk<RetType> {
  fn getCmyk(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getCmyk(int * c, int * m, int * y, int * k, int * a);
impl<'a> /*trait*/ QColor_getCmyk<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getCmyk(self , rsthis: &mut QColor) -> () {
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

// proto:  int QColor::green();
impl /*struct*/ QColor {
  pub fn green<RetType, T: QColor_green<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.green(self);
    // return 1;
  }
}

pub trait QColor_green<RetType> {
  fn green(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::green();
impl<'a> /*trait*/ QColor_green<i32> for () {
  fn green(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5greenEv()};
    let mut ret = unsafe {_ZNK6QColor5greenEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QColor::hsvSaturation();
impl /*struct*/ QColor {
  pub fn hsvSaturation<RetType, T: QColor_hsvSaturation<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hsvSaturation(self);
    // return 1;
  }
}

pub trait QColor_hsvSaturation<RetType> {
  fn hsvSaturation(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::hsvSaturation();
impl<'a> /*trait*/ QColor_hsvSaturation<i32> for () {
  fn hsvSaturation(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor13hsvSaturationEv()};
    let mut ret = unsafe {_ZNK6QColor13hsvSaturationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QColor QColor::toHsl();
impl /*struct*/ QColor {
  pub fn toHsl<RetType, T: QColor_toHsl<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toHsl(self);
    // return 1;
  }
}

pub trait QColor_toHsl<RetType> {
  fn toHsl(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::toHsl();
impl<'a> /*trait*/ QColor_toHsl<QColor> for () {
  fn toHsl(self , rsthis: &mut QColor) -> QColor {
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

// proto:  void QColor::setBlue(int blue);
impl /*struct*/ QColor {
  pub fn setBlue<RetType, T: QColor_setBlue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBlue(self);
    // return 1;
  }
}

pub trait QColor_setBlue<RetType> {
  fn setBlue(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setBlue(int blue);
impl<'a> /*trait*/ QColor_setBlue<()> for (i32) {
  fn setBlue(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setBlueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor7setBlueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QColor::cyan();
impl /*struct*/ QColor {
  pub fn cyan<RetType, T: QColor_cyan<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cyan(self);
    // return 1;
  }
}

pub trait QColor_cyan<RetType> {
  fn cyan(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::cyan();
impl<'a> /*trait*/ QColor_cyan<i32> for () {
  fn cyan(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4cyanEv()};
    let mut ret = unsafe {_ZNK6QColor4cyanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QColor::setCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl /*struct*/ QColor {
  pub fn setCmykF<RetType, T: QColor_setCmykF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCmykF(self);
    // return 1;
  }
}

pub trait QColor_setCmykF<RetType> {
  fn setCmykF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl<'a> /*trait*/ QColor_setCmykF<()> for (f64, f64, f64, f64, f64) {
  fn setCmykF(self , rsthis: &mut QColor) -> () {
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

// proto: static QColor QColor::fromCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl /*struct*/ QColor {
  pub fn fromCmykF_s<RetType, T: QColor_fromCmykF_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromCmykF_s();
    // return 1;
  }
}

pub trait QColor_fromCmykF_s<RetType> {
  fn fromCmykF_s(self ) -> RetType;
}

// proto: static QColor QColor::fromCmykF(qreal c, qreal m, qreal y, qreal k, qreal a);
impl<'a> /*trait*/ QColor_fromCmykF_s<QColor> for (f64, f64, f64, f64, f64) {
  fn fromCmykF_s(self ) -> QColor {
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

// proto:  QColor QColor::light(int f);
impl /*struct*/ QColor {
  pub fn light<RetType, T: QColor_light<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.light(self);
    // return 1;
  }
}

pub trait QColor_light<RetType> {
  fn light(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::light(int f);
impl<'a> /*trait*/ QColor_light<QColor> for (i32) {
  fn light(self , rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5lightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor5lightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QColor::getHslF(qreal * h, qreal * s, qreal * l, qreal * a);
impl /*struct*/ QColor {
  pub fn getHslF<RetType, T: QColor_getHslF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getHslF(self);
    // return 1;
  }
}

pub trait QColor_getHslF<RetType> {
  fn getHslF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getHslF(qreal * h, qreal * s, qreal * l, qreal * a);
impl<'a> /*trait*/ QColor_getHslF<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getHslF(self , rsthis: &mut QColor) -> () {
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

// proto: static QColor QColor::fromRgb(QRgb rgb);
impl /*struct*/ QColor {
  pub fn fromRgb_s<RetType, T: QColor_fromRgb_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgb_s();
    // return 1;
  }
}

pub trait QColor_fromRgb_s<RetType> {
  fn fromRgb_s(self ) -> RetType;
}

// proto: static QColor QColor::fromRgb(QRgb rgb);
impl<'a> /*trait*/ QColor_fromRgb_s<QColor> for (u32) {
  fn fromRgb_s(self ) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7fromRgbEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN6QColor7fromRgbEj(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QColor::yellow();
impl /*struct*/ QColor {
  pub fn yellow<RetType, T: QColor_yellow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.yellow(self);
    // return 1;
  }
}

pub trait QColor_yellow<RetType> {
  fn yellow(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::yellow();
impl<'a> /*trait*/ QColor_yellow<i32> for () {
  fn yellow(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6yellowEv()};
    let mut ret = unsafe {_ZNK6QColor6yellowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QColor::getRgbF(qreal * r, qreal * g, qreal * b, qreal * a);
impl /*struct*/ QColor {
  pub fn getRgbF<RetType, T: QColor_getRgbF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getRgbF(self);
    // return 1;
  }
}

pub trait QColor_getRgbF<RetType> {
  fn getRgbF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getRgbF(qreal * r, qreal * g, qreal * b, qreal * a);
impl<'a> /*trait*/ QColor_getRgbF<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getRgbF(self , rsthis: &mut QColor) -> () {
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

// proto:  void QColor::setRgb(int r, int g, int b, int a);
impl /*struct*/ QColor {
  pub fn setRgb<RetType, T: QColor_setRgb<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRgb(self);
    // return 1;
  }
}

pub trait QColor_setRgb<RetType> {
  fn setRgb(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setRgb(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_setRgb<()> for (i32, i32, i32, i32) {
  fn setRgb(self , rsthis: &mut QColor) -> () {
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

// proto:  QString QColor::name();
impl /*struct*/ QColor {
  pub fn name<RetType, T: QColor_name<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QColor_name<RetType> {
  fn name(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QString QColor::name();
impl<'a> /*trait*/ QColor_name<QString> for () {
  fn name(self , rsthis: &mut QColor) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4nameEv()};
    let mut ret = unsafe {_ZNK6QColor4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QColor::redF();
impl /*struct*/ QColor {
  pub fn redF<RetType, T: QColor_redF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.redF(self);
    // return 1;
  }
}

pub trait QColor_redF<RetType> {
  fn redF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::redF();
impl<'a> /*trait*/ QColor_redF<f64> for () {
  fn redF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4redFEv()};
    let mut ret = unsafe {_ZNK6QColor4redFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QColor::blackF();
impl /*struct*/ QColor {
  pub fn blackF<RetType, T: QColor_blackF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.blackF(self);
    // return 1;
  }
}

pub trait QColor_blackF<RetType> {
  fn blackF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::blackF();
impl<'a> /*trait*/ QColor_blackF<f64> for () {
  fn blackF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6blackFEv()};
    let mut ret = unsafe {_ZNK6QColor6blackFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QColor::setHsvF(qreal h, qreal s, qreal v, qreal a);
impl /*struct*/ QColor {
  pub fn setHsvF<RetType, T: QColor_setHsvF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHsvF(self);
    // return 1;
  }
}

pub trait QColor_setHsvF<RetType> {
  fn setHsvF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setHsvF(qreal h, qreal s, qreal v, qreal a);
impl<'a> /*trait*/ QColor_setHsvF<()> for (f64, f64, f64, f64) {
  fn setHsvF(self , rsthis: &mut QColor) -> () {
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
impl<'a> /*trait*/ QColor_setRgb<()> for (u32) {
  fn setRgb(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor6setRgbEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QColor6setRgbEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QColor QColor::fromRgb(int r, int g, int b, int a);
impl<'a> /*trait*/ QColor_fromRgb_s<QColor> for (i32, i32, i32, i32) {
  fn fromRgb_s(self ) -> QColor {
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

// proto:  double QColor::hsvHueF();
impl /*struct*/ QColor {
  pub fn hsvHueF<RetType, T: QColor_hsvHueF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hsvHueF(self);
    // return 1;
  }
}

pub trait QColor_hsvHueF<RetType> {
  fn hsvHueF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::hsvHueF();
impl<'a> /*trait*/ QColor_hsvHueF<f64> for () {
  fn hsvHueF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7hsvHueFEv()};
    let mut ret = unsafe {_ZNK6QColor7hsvHueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QColor::hsvSaturationF();
impl /*struct*/ QColor {
  pub fn hsvSaturationF<RetType, T: QColor_hsvSaturationF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hsvSaturationF(self);
    // return 1;
  }
}

pub trait QColor_hsvSaturationF<RetType> {
  fn hsvSaturationF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::hsvSaturationF();
impl<'a> /*trait*/ QColor_hsvSaturationF<f64> for () {
  fn hsvSaturationF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor14hsvSaturationFEv()};
    let mut ret = unsafe {_ZNK6QColor14hsvSaturationFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QColor::yellowF();
impl /*struct*/ QColor {
  pub fn yellowF<RetType, T: QColor_yellowF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.yellowF(self);
    // return 1;
  }
}

pub trait QColor_yellowF<RetType> {
  fn yellowF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::yellowF();
impl<'a> /*trait*/ QColor_yellowF<f64> for () {
  fn yellowF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7yellowFEv()};
    let mut ret = unsafe {_ZNK6QColor7yellowFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  int QColor::black();
impl /*struct*/ QColor {
  pub fn black<RetType, T: QColor_black<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.black(self);
    // return 1;
  }
}

pub trait QColor_black<RetType> {
  fn black(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::black();
impl<'a> /*trait*/ QColor_black<i32> for () {
  fn black(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5blackEv()};
    let mut ret = unsafe {_ZNK6QColor5blackEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QColor::setGreenF(qreal green);
impl /*struct*/ QColor {
  pub fn setGreenF<RetType, T: QColor_setGreenF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setGreenF(self);
    // return 1;
  }
}

pub trait QColor_setGreenF<RetType> {
  fn setGreenF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setGreenF(qreal green);
impl<'a> /*trait*/ QColor_setGreenF<()> for (f64) {
  fn setGreenF(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor9setGreenFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor9setGreenFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  unsigned int QColor::rgba();
impl /*struct*/ QColor {
  pub fn rgba<RetType, T: QColor_rgba<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rgba(self);
    // return 1;
  }
}

pub trait QColor_rgba<RetType> {
  fn rgba(self , rsthis: &mut QColor) -> RetType;
}

// proto:  unsigned int QColor::rgba();
impl<'a> /*trait*/ QColor_rgba<u32> for () {
  fn rgba(self , rsthis: &mut QColor) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4rgbaEv()};
    let mut ret = unsafe {_ZNK6QColor4rgbaEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

// proto:  QColor QColor::toCmyk();
impl /*struct*/ QColor {
  pub fn toCmyk<RetType, T: QColor_toCmyk<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toCmyk(self);
    // return 1;
  }
}

pub trait QColor_toCmyk<RetType> {
  fn toCmyk(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::toCmyk();
impl<'a> /*trait*/ QColor_toCmyk<QColor> for () {
  fn toCmyk(self , rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6toCmykEv()};
    let mut ret = unsafe {_ZNK6QColor6toCmykEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QColor::greenF();
impl /*struct*/ QColor {
  pub fn greenF<RetType, T: QColor_greenF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.greenF(self);
    // return 1;
  }
}

pub trait QColor_greenF<RetType> {
  fn greenF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::greenF();
impl<'a> /*trait*/ QColor_greenF<f64> for () {
  fn greenF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6greenFEv()};
    let mut ret = unsafe {_ZNK6QColor6greenFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  int QColor::red();
impl /*struct*/ QColor {
  pub fn red<RetType, T: QColor_red<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.red(self);
    // return 1;
  }
}

pub trait QColor_red<RetType> {
  fn red(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::red();
impl<'a> /*trait*/ QColor_red<i32> for () {
  fn red(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3redEv()};
    let mut ret = unsafe {_ZNK6QColor3redEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QColor::setRgbF(qreal r, qreal g, qreal b, qreal a);
impl /*struct*/ QColor {
  pub fn setRgbF<RetType, T: QColor_setRgbF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRgbF(self);
    // return 1;
  }
}

pub trait QColor_setRgbF<RetType> {
  fn setRgbF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setRgbF(qreal r, qreal g, qreal b, qreal a);
impl<'a> /*trait*/ QColor_setRgbF<()> for (f64, f64, f64, f64) {
  fn setRgbF(self , rsthis: &mut QColor) -> () {
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

// proto:  double QColor::lightnessF();
impl /*struct*/ QColor {
  pub fn lightnessF<RetType, T: QColor_lightnessF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lightnessF(self);
    // return 1;
  }
}

pub trait QColor_lightnessF<RetType> {
  fn lightnessF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::lightnessF();
impl<'a> /*trait*/ QColor_lightnessF<f64> for () {
  fn lightnessF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor10lightnessFEv()};
    let mut ret = unsafe {_ZNK6QColor10lightnessFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QColor QColor::toHsv();
impl /*struct*/ QColor {
  pub fn toHsv<RetType, T: QColor_toHsv<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toHsv(self);
    // return 1;
  }
}

pub trait QColor_toHsv<RetType> {
  fn toHsv(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::toHsv();
impl<'a> /*trait*/ QColor_toHsv<QColor> for () {
  fn toHsv(self , rsthis: &mut QColor) -> QColor {
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

// proto: static QColor QColor::fromHsv(int h, int s, int v, int a);
impl /*struct*/ QColor {
  pub fn fromHsv_s<RetType, T: QColor_fromHsv_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHsv_s();
    // return 1;
  }
}

pub trait QColor_fromHsv_s<RetType> {
  fn fromHsv_s(self ) -> RetType;
}

// proto: static QColor QColor::fromHsv(int h, int s, int v, int a);
impl<'a> /*trait*/ QColor_fromHsv_s<QColor> for (i32, i32, i32, i32) {
  fn fromHsv_s(self ) -> QColor {
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

// proto:  double QColor::hueF();
impl /*struct*/ QColor {
  pub fn hueF<RetType, T: QColor_hueF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hueF(self);
    // return 1;
  }
}

pub trait QColor_hueF<RetType> {
  fn hueF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::hueF();
impl<'a> /*trait*/ QColor_hueF<f64> for () {
  fn hueF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4hueFEv()};
    let mut ret = unsafe {_ZNK6QColor4hueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QColor::setBlueF(qreal blue);
impl /*struct*/ QColor {
  pub fn setBlueF<RetType, T: QColor_setBlueF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBlueF(self);
    // return 1;
  }
}

pub trait QColor_setBlueF<RetType> {
  fn setBlueF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setBlueF(qreal blue);
impl<'a> /*trait*/ QColor_setBlueF<()> for (f64) {
  fn setBlueF(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setBlueFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor8setBlueFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QColor::saturationF();
impl /*struct*/ QColor {
  pub fn saturationF<RetType, T: QColor_saturationF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.saturationF(self);
    // return 1;
  }
}

pub trait QColor_saturationF<RetType> {
  fn saturationF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::saturationF();
impl<'a> /*trait*/ QColor_saturationF<f64> for () {
  fn saturationF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor11saturationFEv()};
    let mut ret = unsafe {_ZNK6QColor11saturationFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  bool QColor::isValid();
impl /*struct*/ QColor {
  pub fn isValid<RetType, T: QColor_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QColor_isValid<RetType> {
  fn isValid(self , rsthis: &mut QColor) -> RetType;
}

// proto:  bool QColor::isValid();
impl<'a> /*trait*/ QColor_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QColor) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7isValidEv()};
    let mut ret = unsafe {_ZNK6QColor7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QColor QColor::darker(int f);
impl /*struct*/ QColor {
  pub fn darker<RetType, T: QColor_darker<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.darker(self);
    // return 1;
  }
}

pub trait QColor_darker<RetType> {
  fn darker(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::darker(int f);
impl<'a> /*trait*/ QColor_darker<QColor> for (i32) {
  fn darker(self , rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6darkerEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor6darkerEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QColor::blueF();
impl /*struct*/ QColor {
  pub fn blueF<RetType, T: QColor_blueF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.blueF(self);
    // return 1;
  }
}

pub trait QColor_blueF<RetType> {
  fn blueF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::blueF();
impl<'a> /*trait*/ QColor_blueF<f64> for () {
  fn blueF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5blueFEv()};
    let mut ret = unsafe {_ZNK6QColor5blueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  int QColor::hue();
impl /*struct*/ QColor {
  pub fn hue<RetType, T: QColor_hue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hue(self);
    // return 1;
  }
}

pub trait QColor_hue<RetType> {
  fn hue(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::hue();
impl<'a> /*trait*/ QColor_hue<i32> for () {
  fn hue(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3hueEv()};
    let mut ret = unsafe {_ZNK6QColor3hueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QColor::setRgba(QRgb rgba);
impl /*struct*/ QColor {
  pub fn setRgba<RetType, T: QColor_setRgba<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRgba(self);
    // return 1;
  }
}

pub trait QColor_setRgba<RetType> {
  fn setRgba(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setRgba(QRgb rgba);
impl<'a> /*trait*/ QColor_setRgba<()> for (u32) {
  fn setRgba(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRgbaEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QColor7setRgbaEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QColor::setNamedColor(const QString & name);
impl /*struct*/ QColor {
  pub fn setNamedColor<RetType, T: QColor_setNamedColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setNamedColor(self);
    // return 1;
  }
}

pub trait QColor_setNamedColor<RetType> {
  fn setNamedColor(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setNamedColor(const QString & name);
impl<'a> /*trait*/ QColor_setNamedColor<()> for (&'a  QString) {
  fn setNamedColor(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor13setNamedColorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QColor13setNamedColorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QColor::magenta();
impl /*struct*/ QColor {
  pub fn magenta<RetType, T: QColor_magenta<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.magenta(self);
    // return 1;
  }
}

pub trait QColor_magenta<RetType> {
  fn magenta(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::magenta();
impl<'a> /*trait*/ QColor_magenta<i32> for () {
  fn magenta(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7magentaEv()};
    let mut ret = unsafe {_ZNK6QColor7magentaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QColor QColor::lighter(int f);
impl /*struct*/ QColor {
  pub fn lighter<RetType, T: QColor_lighter<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lighter(self);
    // return 1;
  }
}

pub trait QColor_lighter<RetType> {
  fn lighter(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::lighter(int f);
impl<'a> /*trait*/ QColor_lighter<QColor> for (i32) {
  fn lighter(self , rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7lighterEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor7lighterEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QColor QColor::toRgb();
impl /*struct*/ QColor {
  pub fn toRgb<RetType, T: QColor_toRgb<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toRgb(self);
    // return 1;
  }
}

pub trait QColor_toRgb<RetType> {
  fn toRgb(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::toRgb();
impl<'a> /*trait*/ QColor_toRgb<QColor> for () {
  fn toRgb(self , rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5toRgbEv()};
    let mut ret = unsafe {_ZNK6QColor5toRgbEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QColor::magentaF();
impl /*struct*/ QColor {
  pub fn magentaF<RetType, T: QColor_magentaF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.magentaF(self);
    // return 1;
  }
}

pub trait QColor_magentaF<RetType> {
  fn magentaF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::magentaF();
impl<'a> /*trait*/ QColor_magentaF<f64> for () {
  fn magentaF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor8magentaFEv()};
    let mut ret = unsafe {_ZNK6QColor8magentaFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QColor::hslHueF();
impl /*struct*/ QColor {
  pub fn hslHueF<RetType, T: QColor_hslHueF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hslHueF(self);
    // return 1;
  }
}

pub trait QColor_hslHueF<RetType> {
  fn hslHueF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::hslHueF();
impl<'a> /*trait*/ QColor_hslHueF<f64> for () {
  fn hslHueF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor7hslHueFEv()};
    let mut ret = unsafe {_ZNK6QColor7hslHueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: static QColor QColor::fromCmyk(int c, int m, int y, int k, int a);
impl /*struct*/ QColor {
  pub fn fromCmyk_s<RetType, T: QColor_fromCmyk_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromCmyk_s();
    // return 1;
  }
}

pub trait QColor_fromCmyk_s<RetType> {
  fn fromCmyk_s(self ) -> RetType;
}

// proto: static QColor QColor::fromCmyk(int c, int m, int y, int k, int a);
impl<'a> /*trait*/ QColor_fromCmyk_s<QColor> for (i32, i32, i32, i32, i32) {
  fn fromCmyk_s(self ) -> QColor {
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

// proto:  void QColor::setCmyk(int c, int m, int y, int k, int a);
impl /*struct*/ QColor {
  pub fn setCmyk<RetType, T: QColor_setCmyk<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCmyk(self);
    // return 1;
  }
}

pub trait QColor_setCmyk<RetType> {
  fn setCmyk(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setCmyk(int c, int m, int y, int k, int a);
impl<'a> /*trait*/ QColor_setCmyk<()> for (i32, i32, i32, i32, i32) {
  fn setCmyk(self , rsthis: &mut QColor) -> () {
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

// proto: static QStringList QColor::colorNames();
impl /*struct*/ QColor {
  pub fn colorNames_s<RetType, T: QColor_colorNames_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.colorNames_s();
    // return 1;
  }
}

pub trait QColor_colorNames_s<RetType> {
  fn colorNames_s(self ) -> RetType;
}

// proto: static QStringList QColor::colorNames();
impl<'a> /*trait*/ QColor_colorNames_s<()> for () {
  fn colorNames_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor10colorNamesEv()};
     unsafe {_ZN6QColor10colorNamesEv()};
    // return 1;
  }
}

// proto:  void QColor::getHsv(int * h, int * s, int * v, int * a);
impl /*struct*/ QColor {
  pub fn getHsv<RetType, T: QColor_getHsv<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getHsv(self);
    // return 1;
  }
}

pub trait QColor_getHsv<RetType> {
  fn getHsv(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getHsv(int * h, int * s, int * v, int * a);
impl<'a> /*trait*/ QColor_getHsv<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getHsv(self , rsthis: &mut QColor) -> () {
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

// proto:  void QColor::getCmykF(qreal * c, qreal * m, qreal * y, qreal * k, qreal * a);
impl /*struct*/ QColor {
  pub fn getCmykF<RetType, T: QColor_getCmykF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getCmykF(self);
    // return 1;
  }
}

pub trait QColor_getCmykF<RetType> {
  fn getCmykF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getCmykF(qreal * c, qreal * m, qreal * y, qreal * k, qreal * a);
impl<'a> /*trait*/ QColor_getCmykF<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getCmykF(self , rsthis: &mut QColor) -> () {
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

// proto:  void QColor::setRed(int red);
impl /*struct*/ QColor {
  pub fn setRed<RetType, T: QColor_setRed<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRed(self);
    // return 1;
  }
}

pub trait QColor_setRed<RetType> {
  fn setRed(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setRed(int red);
impl<'a> /*trait*/ QColor_setRed<()> for (i32) {
  fn setRed(self , rsthis: &mut QColor) -> () {
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

// proto: static QColor QColor::fromRgba(QRgb rgba);
impl /*struct*/ QColor {
  pub fn fromRgba_s<RetType, T: QColor_fromRgba_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba_s();
    // return 1;
  }
}

pub trait QColor_fromRgba_s<RetType> {
  fn fromRgba_s(self ) -> RetType;
}

// proto: static QColor QColor::fromRgba(QRgb rgba);
impl<'a> /*trait*/ QColor_fromRgba_s<QColor> for (u32) {
  fn fromRgba_s(self ) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8fromRgbaEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN6QColor8fromRgbaEj(arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QColor::setHsv(int h, int s, int v, int a);
impl /*struct*/ QColor {
  pub fn setHsv<RetType, T: QColor_setHsv<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHsv(self);
    // return 1;
  }
}

pub trait QColor_setHsv<RetType> {
  fn setHsv(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setHsv(int h, int s, int v, int a);
impl<'a> /*trait*/ QColor_setHsv<()> for (i32, i32, i32, i32) {
  fn setHsv(self , rsthis: &mut QColor) -> () {
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

// proto:  unsigned int QColor::rgb();
impl /*struct*/ QColor {
  pub fn rgb<RetType, T: QColor_rgb<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rgb(self);
    // return 1;
  }
}

pub trait QColor_rgb<RetType> {
  fn rgb(self , rsthis: &mut QColor) -> RetType;
}

// proto:  unsigned int QColor::rgb();
impl<'a> /*trait*/ QColor_rgb<u32> for () {
  fn rgb(self , rsthis: &mut QColor) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor3rgbEv()};
    let mut ret = unsafe {_ZNK6QColor3rgbEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

// proto:  void QColor::setHslF(qreal h, qreal s, qreal l, qreal a);
impl /*struct*/ QColor {
  pub fn setHslF<RetType, T: QColor_setHslF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHslF(self);
    // return 1;
  }
}

pub trait QColor_setHslF<RetType> {
  fn setHslF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setHslF(qreal h, qreal s, qreal l, qreal a);
impl<'a> /*trait*/ QColor_setHslF<()> for (f64, f64, f64, f64) {
  fn setHslF(self , rsthis: &mut QColor) -> () {
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

// proto:  int QColor::saturation();
impl /*struct*/ QColor {
  pub fn saturation<RetType, T: QColor_saturation<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.saturation(self);
    // return 1;
  }
}

pub trait QColor_saturation<RetType> {
  fn saturation(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::saturation();
impl<'a> /*trait*/ QColor_saturation<i32> for () {
  fn saturation(self , rsthis: &mut QColor) -> i32 {
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

// proto:  double QColor::alphaF();
impl /*struct*/ QColor {
  pub fn alphaF<RetType, T: QColor_alphaF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.alphaF(self);
    // return 1;
  }
}

pub trait QColor_alphaF<RetType> {
  fn alphaF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::alphaF();
impl<'a> /*trait*/ QColor_alphaF<f64> for () {
  fn alphaF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6alphaFEv()};
    let mut ret = unsafe {_ZNK6QColor6alphaFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  int QColor::value();
impl /*struct*/ QColor {
  pub fn value<RetType, T: QColor_value<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QColor_value<RetType> {
  fn value(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::value();
impl<'a> /*trait*/ QColor_value<i32> for () {
  fn value(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5valueEv()};
    let mut ret = unsafe {_ZNK6QColor5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static QColor QColor::fromHsvF(qreal h, qreal s, qreal v, qreal a);
impl /*struct*/ QColor {
  pub fn fromHsvF_s<RetType, T: QColor_fromHsvF_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHsvF_s();
    // return 1;
  }
}

pub trait QColor_fromHsvF_s<RetType> {
  fn fromHsvF_s(self ) -> RetType;
}

// proto: static QColor QColor::fromHsvF(qreal h, qreal s, qreal v, qreal a);
impl<'a> /*trait*/ QColor_fromHsvF_s<QColor> for (f64, f64, f64, f64) {
  fn fromHsvF_s(self ) -> QColor {
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

// proto:  QColor QColor::dark(int f);
impl /*struct*/ QColor {
  pub fn dark<RetType, T: QColor_dark<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.dark(self);
    // return 1;
  }
}

pub trait QColor_dark<RetType> {
  fn dark(self , rsthis: &mut QColor) -> RetType;
}

// proto:  QColor QColor::dark(int f);
impl<'a> /*trait*/ QColor_dark<QColor> for (i32) {
  fn dark(self , rsthis: &mut QColor) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4darkEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QColor4darkEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QColor::setRedF(qreal red);
impl /*struct*/ QColor {
  pub fn setRedF<RetType, T: QColor_setRedF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRedF(self);
    // return 1;
  }
}

pub trait QColor_setRedF<RetType> {
  fn setRedF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setRedF(qreal red);
impl<'a> /*trait*/ QColor_setRedF<()> for (f64) {
  fn setRedF(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor7setRedFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QColor7setRedFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QColor QColor::fromHsl(int h, int s, int l, int a);
impl /*struct*/ QColor {
  pub fn fromHsl_s<RetType, T: QColor_fromHsl_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHsl_s();
    // return 1;
  }
}

pub trait QColor_fromHsl_s<RetType> {
  fn fromHsl_s(self ) -> RetType;
}

// proto: static QColor QColor::fromHsl(int h, int s, int l, int a);
impl<'a> /*trait*/ QColor_fromHsl_s<QColor> for (i32, i32, i32, i32) {
  fn fromHsl_s(self ) -> QColor {
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

// proto:  void QColor::setHsl(int h, int s, int l, int a);
impl /*struct*/ QColor {
  pub fn setHsl<RetType, T: QColor_setHsl<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHsl(self);
    // return 1;
  }
}

pub trait QColor_setHsl<RetType> {
  fn setHsl(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setHsl(int h, int s, int l, int a);
impl<'a> /*trait*/ QColor_setHsl<()> for (i32, i32, i32, i32) {
  fn setHsl(self , rsthis: &mut QColor) -> () {
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

// proto:  void QColor::setGreen(int green);
impl /*struct*/ QColor {
  pub fn setGreen<RetType, T: QColor_setGreen<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setGreen(self);
    // return 1;
  }
}

pub trait QColor_setGreen<RetType> {
  fn setGreen(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::setGreen(int green);
impl<'a> /*trait*/ QColor_setGreen<()> for (i32) {
  fn setGreen(self , rsthis: &mut QColor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor8setGreenEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QColor8setGreenEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QColor::getHsl(int * h, int * s, int * l, int * a);
impl /*struct*/ QColor {
  pub fn getHsl<RetType, T: QColor_getHsl<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getHsl(self);
    // return 1;
  }
}

pub trait QColor_getHsl<RetType> {
  fn getHsl(self , rsthis: &mut QColor) -> RetType;
}

// proto:  void QColor::getHsl(int * h, int * s, int * l, int * a);
impl<'a> /*trait*/ QColor_getHsl<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getHsl(self , rsthis: &mut QColor) -> () {
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

// proto: static bool QColor::isValidColor(const QString & name);
impl /*struct*/ QColor {
  pub fn isValidColor_s<RetType, T: QColor_isValidColor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValidColor_s();
    // return 1;
  }
}

pub trait QColor_isValidColor_s<RetType> {
  fn isValidColor_s(self ) -> RetType;
}

// proto: static bool QColor::isValidColor(const QString & name);
impl<'a> /*trait*/ QColor_isValidColor_s<i8> for (&'a  QString) {
  fn isValidColor_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QColor12isValidColorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN6QColor12isValidColorERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QColor::hslSaturation();
impl /*struct*/ QColor {
  pub fn hslSaturation<RetType, T: QColor_hslSaturation<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hslSaturation(self);
    // return 1;
  }
}

pub trait QColor_hslSaturation<RetType> {
  fn hslSaturation(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::hslSaturation();
impl<'a> /*trait*/ QColor_hslSaturation<i32> for () {
  fn hslSaturation(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor13hslSaturationEv()};
    let mut ret = unsafe {_ZNK6QColor13hslSaturationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static QColor QColor::fromRgbF(qreal r, qreal g, qreal b, qreal a);
impl /*struct*/ QColor {
  pub fn fromRgbF_s<RetType, T: QColor_fromRgbF_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgbF_s();
    // return 1;
  }
}

pub trait QColor_fromRgbF_s<RetType> {
  fn fromRgbF_s(self ) -> RetType;
}

// proto: static QColor QColor::fromRgbF(qreal r, qreal g, qreal b, qreal a);
impl<'a> /*trait*/ QColor_fromRgbF_s<QColor> for (f64, f64, f64, f64) {
  fn fromRgbF_s(self ) -> QColor {
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

// proto:  int QColor::blue();
impl /*struct*/ QColor {
  pub fn blue<RetType, T: QColor_blue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.blue(self);
    // return 1;
  }
}

pub trait QColor_blue<RetType> {
  fn blue(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::blue();
impl<'a> /*trait*/ QColor_blue<i32> for () {
  fn blue(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor4blueEv()};
    let mut ret = unsafe {_ZNK6QColor4blueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QColor::hsvHue();
impl /*struct*/ QColor {
  pub fn hsvHue<RetType, T: QColor_hsvHue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hsvHue(self);
    // return 1;
  }
}

pub trait QColor_hsvHue<RetType> {
  fn hsvHue(self , rsthis: &mut QColor) -> RetType;
}

// proto:  int QColor::hsvHue();
impl<'a> /*trait*/ QColor_hsvHue<i32> for () {
  fn hsvHue(self , rsthis: &mut QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6hsvHueEv()};
    let mut ret = unsafe {_ZNK6QColor6hsvHueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  double QColor::valueF();
impl /*struct*/ QColor {
  pub fn valueF<RetType, T: QColor_valueF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.valueF(self);
    // return 1;
  }
}

pub trait QColor_valueF<RetType> {
  fn valueF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::valueF();
impl<'a> /*trait*/ QColor_valueF<f64> for () {
  fn valueF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor6valueFEv()};
    let mut ret = unsafe {_ZNK6QColor6valueFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QColor::cyanF();
impl /*struct*/ QColor {
  pub fn cyanF<RetType, T: QColor_cyanF<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cyanF(self);
    // return 1;
  }
}

pub trait QColor_cyanF<RetType> {
  fn cyanF(self , rsthis: &mut QColor) -> RetType;
}

// proto:  double QColor::cyanF();
impl<'a> /*trait*/ QColor_cyanF<f64> for () {
  fn cyanF(self , rsthis: &mut QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QColor5cyanFEv()};
    let mut ret = unsafe {_ZNK6QColor5cyanFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

