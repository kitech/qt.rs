// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qopenglframebufferobjectformat::QOpenGLFramebufferObjectFormat;
use super::qimage::QImage;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QOpenGLFramebufferObject::isValid();
  fn _ZNK24QOpenGLFramebufferObject7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::takeTexture();
  fn _ZN24QOpenGLFramebufferObject11takeTextureEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, const QOpenGLFramebufferObjectFormat & format);
  fn _ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static bool QOpenGLFramebufferObject::bindDefault();
  fn _ZN24QOpenGLFramebufferObject11bindDefaultEv() -> int8_t;
  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
  fn _ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv() -> int8_t;
  // proto:  QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::texture();
  fn _ZNK24QOpenGLFramebufferObject7textureEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, GLenum target);
  fn _ZN24QOpenGLFramebufferObjectC1ERK5QSizej(qthis: *mut c_void, arg0: *mut c_void, arg1: c_uint) ;
  // proto:  bool QOpenGLFramebufferObject::release();
  fn _ZN24QOpenGLFramebufferObject7releaseEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
  fn _ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv() -> int8_t;
  // proto:  QImage QOpenGLFramebufferObject::toImage(bool flipped);
  fn _ZNK24QOpenGLFramebufferObject7toImageEb(qthis: *mut c_void, arg0: int8_t) -> *mut c_void;
  // proto:  QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::handle();
  fn _ZNK24QOpenGLFramebufferObject6handleEv(qthis: *mut c_void) ;
  // proto:  int QOpenGLFramebufferObject::height();
  fn _ZNK24QOpenGLFramebufferObject6heightEv(qthis: *mut c_void) -> c_int;
  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
  fn _ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj(arg0: *mut c_void, arg1: *mut c_void, arg2: c_uint, arg3: c_uint) ;
  // proto:  void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat & format);
  fn _ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, const QRect & targetRect, QOpenGLFramebufferObject * source, const QRect & sourceRect, GLbitfield buffers, GLenum filter);
  fn _ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: c_uint, arg5: c_uint) ;
  // proto:  QImage QOpenGLFramebufferObject::toImage();
  fn _ZNK24QOpenGLFramebufferObject7toImageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QOpenGLFramebufferObject::size();
  fn _ZNK24QOpenGLFramebufferObject4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QOpenGLFramebufferObject & );
  fn _ZN24QOpenGLFramebufferObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLFramebufferObject::FreeQOpenGLFramebufferObject();
  fn _ZN24QOpenGLFramebufferObjectD0Ev(qthis: *mut c_void) ;
  // proto:  QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
  fn _ZNK24QOpenGLFramebufferObject6formatEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLFramebufferObject::bind();
  fn _ZN24QOpenGLFramebufferObject4bindEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QOpenGLFramebufferObject::isBound();
  fn _ZNK24QOpenGLFramebufferObject7isBoundEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QOpenGLFramebufferObject::width();
  fn _ZNK24QOpenGLFramebufferObject5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, GLenum target);
  fn _ZN24QOpenGLFramebufferObjectC1Eiij(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uint) ;
}

// body block begin
// class sizeof(QOpenGLFramebufferObject)=1
pub struct QOpenGLFramebufferObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn isValid<RetType, T: QOpenGLFramebufferObject_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_isValid<RetType> {
  fn isValid(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  bool QOpenGLFramebufferObject::isValid();
impl<'a> /*trait*/ QOpenGLFramebufferObject_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7isValidEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn takeTexture<RetType, T: QOpenGLFramebufferObject_takeTexture<RetType>>(&mut self, value: T) -> RetType {
    return value.takeTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_takeTexture<RetType> {
  fn takeTexture(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::takeTexture();
impl<'a> /*trait*/ QOpenGLFramebufferObject_takeTexture<()> for () {
  fn takeTexture(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject11takeTextureEv()};
     unsafe {_ZN24QOpenGLFramebufferObject11takeTextureEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn NewQOpenGLFramebufferObject<T: QOpenGLFramebufferObject_NewQOpenGLFramebufferObject>(value: T) -> QOpenGLFramebufferObject {
    let rsthis = value.NewQOpenGLFramebufferObject();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_NewQOpenGLFramebufferObject {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject;
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, const QOpenGLFramebufferObjectFormat & format);
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (&'a  QSize, &'a  QOpenGLFramebufferObjectFormat) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(qthis, arg0, arg1)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn bindDefault<RetType, T: QOpenGLFramebufferObject_bindDefault<RetType>>(&mut self, value: T) -> RetType {
    return value.bindDefault(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_bindDefault<RetType> {
  fn bindDefault(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto: static bool QOpenGLFramebufferObject::bindDefault();
impl<'a> /*trait*/ QOpenGLFramebufferObject_bindDefault<i8> for () {
  fn bindDefault(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject11bindDefaultEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject11bindDefaultEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn hasOpenGLFramebufferBlit<RetType, T: QOpenGLFramebufferObject_hasOpenGLFramebufferBlit<RetType>>(&mut self, value: T) -> RetType {
    return value.hasOpenGLFramebufferBlit(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_hasOpenGLFramebufferBlit<RetType> {
  fn hasOpenGLFramebufferBlit(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
impl<'a> /*trait*/ QOpenGLFramebufferObject_hasOpenGLFramebufferBlit<i8> for () {
  fn hasOpenGLFramebufferBlit(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn texture<RetType, T: QOpenGLFramebufferObject_texture<RetType>>(&mut self, value: T) -> RetType {
    return value.texture(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_texture<RetType> {
  fn texture(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::texture();
impl<'a> /*trait*/ QOpenGLFramebufferObject_texture<()> for () {
  fn texture(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7textureEv()};
     unsafe {_ZNK24QOpenGLFramebufferObject7textureEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (&'a  QSize, u32) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERK5QSizej()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN24QOpenGLFramebufferObjectC1ERK5QSizej(qthis, arg0, arg1)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn release<RetType, T: QOpenGLFramebufferObject_release<RetType>>(&mut self, value: T) -> RetType {
    return value.release(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_release<RetType> {
  fn release(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  bool QOpenGLFramebufferObject::release();
impl<'a> /*trait*/ QOpenGLFramebufferObject_release<i8> for () {
  fn release(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject7releaseEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject7releaseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn hasOpenGLFramebufferObjects<RetType, T: QOpenGLFramebufferObject_hasOpenGLFramebufferObjects<RetType>>(&mut self, value: T) -> RetType {
    return value.hasOpenGLFramebufferObjects(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_hasOpenGLFramebufferObjects<RetType> {
  fn hasOpenGLFramebufferObjects(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
impl<'a> /*trait*/ QOpenGLFramebufferObject_hasOpenGLFramebufferObjects<i8> for () {
  fn hasOpenGLFramebufferObjects(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn toImage<RetType, T: QOpenGLFramebufferObject_toImage<RetType>>(&mut self, value: T) -> RetType {
    return value.toImage(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_toImage<RetType> {
  fn toImage(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  QImage QOpenGLFramebufferObject::toImage(bool flipped);
impl<'a> /*trait*/ QOpenGLFramebufferObject_toImage<QImage> for (i8) {
  fn toImage(self, rsthis: &mut QOpenGLFramebufferObject) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7toImageEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7toImageEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn handle<RetType, T: QOpenGLFramebufferObject_handle<RetType>>(&mut self, value: T) -> RetType {
    return value.handle(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_handle<RetType> {
  fn handle(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::handle();
impl<'a> /*trait*/ QOpenGLFramebufferObject_handle<()> for () {
  fn handle(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6handleEv()};
     unsafe {_ZNK24QOpenGLFramebufferObject6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn height<RetType, T: QOpenGLFramebufferObject_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_height<RetType> {
  fn height(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  int QOpenGLFramebufferObject::height();
impl<'a> /*trait*/ QOpenGLFramebufferObject_height<i32> for () {
  fn height(self, rsthis: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6heightEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn blitFramebuffer<RetType, T: QOpenGLFramebufferObject_blitFramebuffer<RetType>>(&mut self, value: T) -> RetType {
    return value.blitFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_blitFramebuffer<RetType> {
  fn blitFramebuffer(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
impl<'a> /*trait*/ QOpenGLFramebufferObject_blitFramebuffer<()> for (&'a mut QOpenGLFramebufferObject, &'a mut QOpenGLFramebufferObject, u32, u32) {
  fn blitFramebuffer(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj(arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat & format);
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (i32, i32, &'a  QOpenGLFramebufferObjectFormat) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(qthis, arg0, arg1, arg2)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, const QRect & targetRect, QOpenGLFramebufferObject * source, const QRect & sourceRect, GLbitfield buffers, GLenum filter);
impl<'a> /*trait*/ QOpenGLFramebufferObject_blitFramebuffer<()> for (&'a mut QOpenGLFramebufferObject, &'a  QRect, &'a mut QOpenGLFramebufferObject, &'a  QRect, u32, u32) {
  fn blitFramebuffer(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4  as c_uint;
    let arg5 = self.5  as c_uint;
     unsafe {_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj(arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

// proto:  QImage QOpenGLFramebufferObject::toImage();
impl<'a> /*trait*/ QOpenGLFramebufferObject_toImage<QImage> for () {
  fn toImage(self, rsthis: &mut QOpenGLFramebufferObject) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7toImageEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7toImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn size<RetType, T: QOpenGLFramebufferObject_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_size<RetType> {
  fn size(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  QSize QOpenGLFramebufferObject::size();
impl<'a> /*trait*/ QOpenGLFramebufferObject_size<QSize> for () {
  fn size(self, rsthis: &mut QOpenGLFramebufferObject) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject4sizeEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QOpenGLFramebufferObject & );
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (&'a  QOpenGLFramebufferObject) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLFramebufferObjectC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn FreeQOpenGLFramebufferObject<RetType, T: QOpenGLFramebufferObject_FreeQOpenGLFramebufferObject<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQOpenGLFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_FreeQOpenGLFramebufferObject<RetType> {
  fn FreeQOpenGLFramebufferObject(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  void QOpenGLFramebufferObject::FreeQOpenGLFramebufferObject();
impl<'a> /*trait*/ QOpenGLFramebufferObject_FreeQOpenGLFramebufferObject<()> for () {
  fn FreeQOpenGLFramebufferObject(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectD0Ev()};
     unsafe {_ZN24QOpenGLFramebufferObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn format<RetType, T: QOpenGLFramebufferObject_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_format<RetType> {
  fn format(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
impl<'a> /*trait*/ QOpenGLFramebufferObject_format<()> for () {
  fn format(self, rsthis: &mut QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6formatEv()};
     unsafe {_ZNK24QOpenGLFramebufferObject6formatEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn bind<RetType, T: QOpenGLFramebufferObject_bind<RetType>>(&mut self, value: T) -> RetType {
    return value.bind(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_bind<RetType> {
  fn bind(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  bool QOpenGLFramebufferObject::bind();
impl<'a> /*trait*/ QOpenGLFramebufferObject_bind<i8> for () {
  fn bind(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject4bindEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject4bindEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn isBound<RetType, T: QOpenGLFramebufferObject_isBound<RetType>>(&mut self, value: T) -> RetType {
    return value.isBound(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_isBound<RetType> {
  fn isBound(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  bool QOpenGLFramebufferObject::isBound();
impl<'a> /*trait*/ QOpenGLFramebufferObject_isBound<i8> for () {
  fn isBound(self, rsthis: &mut QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7isBoundEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7isBoundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn width<RetType, T: QOpenGLFramebufferObject_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_width<RetType> {
  fn width(self, rsthis: &mut QOpenGLFramebufferObject) -> RetType;
}

// proto:  int QOpenGLFramebufferObject::width();
impl<'a> /*trait*/ QOpenGLFramebufferObject_width<i32> for () {
  fn width(self, rsthis: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject5widthEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (i32, i32, u32) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1Eiij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    unsafe {_ZN24QOpenGLFramebufferObjectC1Eiij(qthis, arg0, arg1, arg2)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

