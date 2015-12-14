// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpixmap::QPixmap;
use super::qimage::QImage;
use super::qcolor::QColor;
use super::qgradient::QGradient;
use super::qtransform::QTransform;
use super::qmatrix::QMatrix;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QBrush::NewQBrush();
  fn _ZN6QBrushC1Ev(qthis: *mut c_void) ;
  // proto:  void QBrush::NewQBrush(const QPixmap & pixmap);
  fn _ZN6QBrushC1ERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
  fn _ZN6QBrush10setTextureERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBrush::setTextureImage(const QImage & image);
  fn _ZN6QBrush15setTextureImageERK6QImage(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBrush::NewQBrush(const QColor & color, const QPixmap & pixmap);
  fn _ZN6QBrushC1ERK6QColorRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QPixmap QBrush::texture();
  fn _ZNK6QBrush7textureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::NewQBrush(const QGradient & gradient);
  fn _ZN6QBrushC1ERK9QGradient(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTransform QBrush::transform();
  fn _ZNK6QBrush9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::setTransform(const QTransform & );
  fn _ZN6QBrush12setTransformERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QBrush::isOpaque();
  fn _ZNK6QBrush8isOpaqueEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QGradient * QBrush::gradient();
  fn _ZNK6QBrush8gradientEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::FreeQBrush();
  fn _ZN6QBrushD0Ev(qthis: *mut c_void) ;
  // proto:  void QBrush::setMatrix(const QMatrix & mat);
  fn _ZN6QBrush9setMatrixERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBrush::setColor(const QColor & color);
  fn _ZN6QBrush8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBrush::NewQBrush(const QBrush & brush);
  fn _ZN6QBrushC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMatrix & QBrush::matrix();
  fn _ZNK6QBrush6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QImage QBrush::textureImage();
  fn _ZNK6QBrush12textureImageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QBrush::isDetached();
  fn _ZNK6QBrush10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBrush::swap(QBrush & other);
  fn _ZN6QBrush4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QColor & QBrush::color();
  fn _ZNK6QBrush5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::NewQBrush(const QImage & image);
  fn _ZN6QBrushC1ERK6QImage(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QBrush)=1
pub struct QBrush {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBrush {
  pub fn NewQBrush<T: QBrush_NewQBrush>(value: T) -> QBrush {
    let rsthis = value.NewQBrush();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_NewQBrush {
  fn NewQBrush(self) -> QBrush;
}

// proto: void QBrush::NewQBrush();
impl<'a> /*trait*/ QBrush_NewQBrush for () {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1Ev()};
    unsafe {_ZN6QBrushC1Ev(qthis)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QBrush::NewQBrush(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_NewQBrush for (&'a  QPixmap) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK7QPixmap(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn setTexture<T: QBrush_setTexture>(&mut self, value: T)  {
     value.setTexture(self);
    // return 1;
  }
}

pub trait QBrush_setTexture {
  fn setTexture(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::setTexture(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_setTexture for (&'a  QPixmap) {
  fn setTexture(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush10setTextureERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush10setTextureERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn setTextureImage<T: QBrush_setTextureImage>(&mut self, value: T)  {
     value.setTextureImage(self);
    // return 1;
  }
}

pub trait QBrush_setTextureImage {
  fn setTextureImage(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::setTextureImage(const QImage & image);
impl<'a> /*trait*/ QBrush_setTextureImage for (&'a  QImage) {
  fn setTextureImage(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush15setTextureImageERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush15setTextureImageERK6QImage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QBrush::NewQBrush(const QColor & color, const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_NewQBrush for (&'a  QColor, &'a  QPixmap) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK6QColorRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK6QColorRK7QPixmap(qthis, arg0, arg1)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn texture<T: QBrush_texture>(&mut self, value: T) -> QPixmap {
    return value.texture(self);
    // return 1;
  }
}

pub trait QBrush_texture {
  fn texture(self, rsthis: &mut QBrush) -> QPixmap;
}

// proto:  QPixmap QBrush::texture();
impl<'a> /*trait*/ QBrush_texture for () {
  fn texture(self, rsthis: &mut QBrush) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush7textureEv()};
    let mut ret = unsafe {_ZNK6QBrush7textureEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QBrush::NewQBrush(const QGradient & gradient);
impl<'a> /*trait*/ QBrush_NewQBrush for (&'a  QGradient) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK9QGradient()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK9QGradient(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn transform<T: QBrush_transform>(&mut self, value: T) -> QTransform {
    return value.transform(self);
    // return 1;
  }
}

pub trait QBrush_transform {
  fn transform(self, rsthis: &mut QBrush) -> QTransform;
}

// proto:  QTransform QBrush::transform();
impl<'a> /*trait*/ QBrush_transform for () {
  fn transform(self, rsthis: &mut QBrush) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush9transformEv()};
    let mut ret = unsafe {_ZNK6QBrush9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn setTransform<T: QBrush_setTransform>(&mut self, value: T)  {
     value.setTransform(self);
    // return 1;
  }
}

pub trait QBrush_setTransform {
  fn setTransform(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::setTransform(const QTransform & );
impl<'a> /*trait*/ QBrush_setTransform for (&'a  QTransform) {
  fn setTransform(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush12setTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush12setTransformERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn isOpaque<T: QBrush_isOpaque>(&mut self, value: T) -> i8 {
    return value.isOpaque(self);
    // return 1;
  }
}

pub trait QBrush_isOpaque {
  fn isOpaque(self, rsthis: &mut QBrush) -> i8;
}

// proto:  bool QBrush::isOpaque();
impl<'a> /*trait*/ QBrush_isOpaque for () {
  fn isOpaque(self, rsthis: &mut QBrush) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush8isOpaqueEv()};
    let mut ret = unsafe {_ZNK6QBrush8isOpaqueEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn gradient<T: QBrush_gradient>(&mut self, value: T) -> QGradient {
    return value.gradient(self);
    // return 1;
  }
}

pub trait QBrush_gradient {
  fn gradient(self, rsthis: &mut QBrush) -> QGradient;
}

// proto:  const QGradient * QBrush::gradient();
impl<'a> /*trait*/ QBrush_gradient for () {
  fn gradient(self, rsthis: &mut QBrush) -> QGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush8gradientEv()};
    let mut ret = unsafe {_ZNK6QBrush8gradientEv(rsthis.qclsinst)};
    let mut ret1 = QGradient{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn FreeQBrush<T: QBrush_FreeQBrush>(&mut self, value: T)  {
     value.FreeQBrush(self);
    // return 1;
  }
}

pub trait QBrush_FreeQBrush {
  fn FreeQBrush(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::FreeQBrush();
impl<'a> /*trait*/ QBrush_FreeQBrush for () {
  fn FreeQBrush(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushD0Ev()};
     unsafe {_ZN6QBrushD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn setMatrix<T: QBrush_setMatrix>(&mut self, value: T)  {
     value.setMatrix(self);
    // return 1;
  }
}

pub trait QBrush_setMatrix {
  fn setMatrix(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::setMatrix(const QMatrix & mat);
impl<'a> /*trait*/ QBrush_setMatrix for (&'a  QMatrix) {
  fn setMatrix(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush9setMatrixERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush9setMatrixERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn setColor<T: QBrush_setColor>(&mut self, value: T)  {
     value.setColor(self);
    // return 1;
  }
}

pub trait QBrush_setColor {
  fn setColor(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::setColor(const QColor & color);
impl<'a> /*trait*/ QBrush_setColor for (&'a  QColor) {
  fn setColor(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QBrush::NewQBrush(const QBrush & brush);
impl<'a> /*trait*/ QBrush_NewQBrush for (&'a  QBrush) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERKS_(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn matrix<T: QBrush_matrix>(&mut self, value: T) -> QMatrix {
    return value.matrix(self);
    // return 1;
  }
}

pub trait QBrush_matrix {
  fn matrix(self, rsthis: &mut QBrush) -> QMatrix;
}

// proto:  const QMatrix & QBrush::matrix();
impl<'a> /*trait*/ QBrush_matrix for () {
  fn matrix(self, rsthis: &mut QBrush) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush6matrixEv()};
    let mut ret = unsafe {_ZNK6QBrush6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn textureImage<T: QBrush_textureImage>(&mut self, value: T) -> QImage {
    return value.textureImage(self);
    // return 1;
  }
}

pub trait QBrush_textureImage {
  fn textureImage(self, rsthis: &mut QBrush) -> QImage;
}

// proto:  QImage QBrush::textureImage();
impl<'a> /*trait*/ QBrush_textureImage for () {
  fn textureImage(self, rsthis: &mut QBrush) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush12textureImageEv()};
    let mut ret = unsafe {_ZNK6QBrush12textureImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn isDetached<T: QBrush_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QBrush_isDetached {
  fn isDetached(self, rsthis: &mut QBrush) -> i8;
}

// proto:  bool QBrush::isDetached();
impl<'a> /*trait*/ QBrush_isDetached for () {
  fn isDetached(self, rsthis: &mut QBrush) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush10isDetachedEv()};
    let mut ret = unsafe {_ZNK6QBrush10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn swap<T: QBrush_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QBrush_swap {
  fn swap(self, rsthis: &mut QBrush) ;
}

// proto:  void QBrush::swap(QBrush & other);
impl<'a> /*trait*/ QBrush_swap for (&'a mut QBrush) {
  fn swap(self, rsthis: &mut QBrush)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn color<T: QBrush_color>(&mut self, value: T) -> QColor {
    return value.color(self);
    // return 1;
  }
}

pub trait QBrush_color {
  fn color(self, rsthis: &mut QBrush) -> QColor;
}

// proto:  const QColor & QBrush::color();
impl<'a> /*trait*/ QBrush_color for () {
  fn color(self, rsthis: &mut QBrush) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush5colorEv()};
    let mut ret = unsafe {_ZNK6QBrush5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QBrush::NewQBrush(const QImage & image);
impl<'a> /*trait*/ QBrush_NewQBrush for (&'a  QImage) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK6QImage(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

