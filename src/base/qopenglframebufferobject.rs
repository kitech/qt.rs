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
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QOpenGLFramebufferObject::isValid();
  fn _ZNK24QOpenGLFramebufferObject7isValidEv() -> i32;
  // proto: QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::takeTexture();
  fn _ZN24QOpenGLFramebufferObject11takeTextureEv() -> i32;
  // proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, const QOpenGLFramebufferObjectFormat & format);
  fn _ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QOpenGLFramebufferObject::bindDefault();
  fn _ZN24QOpenGLFramebufferObject11bindDefaultEv() -> i32;
  // proto: bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
  fn _ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv() -> i32;
  // proto: QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::texture();
  fn _ZNK24QOpenGLFramebufferObject7textureEv() -> i32;
  // proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, GLenum target);
  fn _ZN24QOpenGLFramebufferObjectC1ERK5QSizej(qthis: *mut c_void, arg0: *const c_void, arg1: c_uint) -> i32;
  // proto: bool QOpenGLFramebufferObject::release();
  fn _ZN24QOpenGLFramebufferObject7releaseEv() -> i32;
  // proto: bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
  fn _ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv() -> i32;
  // proto: QImage QOpenGLFramebufferObject::toImage(bool flipped);
  fn _ZNK24QOpenGLFramebufferObject7toImageEb(arg0: int8_t) -> i32;
  // proto: QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::handle();
  fn _ZNK24QOpenGLFramebufferObject6handleEv() -> i32;
  // proto: int QOpenGLFramebufferObject::height();
  fn _ZNK24QOpenGLFramebufferObject6heightEv() -> i32;
  // proto: void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
  fn _ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj(arg0: *mut c_void, arg1: *mut c_void, arg2: c_uint, arg3: c_uint) -> i32;
  // proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat & format);
  fn _ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, const QRect & targetRect, QOpenGLFramebufferObject * source, const QRect & sourceRect, GLbitfield buffers, GLenum filter);
  fn _ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void, arg3: *const c_void, arg4: c_uint, arg5: c_uint) -> i32;
  // proto: QImage QOpenGLFramebufferObject::toImage();
  fn _ZNK24QOpenGLFramebufferObject7toImageEv() -> i32;
  // proto: QSize QOpenGLFramebufferObject::size();
  fn _ZNK24QOpenGLFramebufferObject4sizeEv() -> i32;
  // proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QOpenGLFramebufferObject & );
  fn _ZN24QOpenGLFramebufferObjectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QOpenGLFramebufferObject::FreeQOpenGLFramebufferObject();
  fn _ZN24QOpenGLFramebufferObjectD0Ev() -> i32;
  // proto: QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
  fn _ZNK24QOpenGLFramebufferObject6formatEv() -> i32;
  // proto: bool QOpenGLFramebufferObject::bind();
  fn _ZN24QOpenGLFramebufferObject4bindEv() -> i32;
  // proto: bool QOpenGLFramebufferObject::isBound();
  fn _ZNK24QOpenGLFramebufferObject7isBoundEv() -> i32;
  // proto: int QOpenGLFramebufferObject::width();
  fn _ZNK24QOpenGLFramebufferObject5widthEv() -> i32;
  // proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, GLenum target);
  fn _ZN24QOpenGLFramebufferObjectC1Eiij(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uint) -> i32;
}

// body block begin
// class sizeof(QOpenGLFramebufferObject)=1
pub struct QOpenGLFramebufferObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn isValid<T: QOpenGLFramebufferObject_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_isValid {
  fn isValid(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::isValid();
impl<'a> /*trait*/ QOpenGLFramebufferObject_isValid for () {
  fn isValid(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7isValidEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn takeTexture<T: QOpenGLFramebufferObject_takeTexture>(&mut self, value: T) -> i32 {
    value.takeTexture(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_takeTexture {
  fn takeTexture(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::takeTexture();
impl<'a> /*trait*/ QOpenGLFramebufferObject_takeTexture for () {
  fn takeTexture(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject11takeTextureEv()};
    unsafe {_ZN24QOpenGLFramebufferObject11takeTextureEv()};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(qthis, arg0, arg1)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn bindDefault<T: QOpenGLFramebufferObject_bindDefault>(&mut self, value: T) -> i32 {
    value.bindDefault(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_bindDefault {
  fn bindDefault(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::bindDefault();
impl<'a> /*trait*/ QOpenGLFramebufferObject_bindDefault for () {
  fn bindDefault(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject11bindDefaultEv()};
    unsafe {_ZN24QOpenGLFramebufferObject11bindDefaultEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn hasOpenGLFramebufferBlit<T: QOpenGLFramebufferObject_hasOpenGLFramebufferBlit>(&mut self, value: T) -> i32 {
    value.hasOpenGLFramebufferBlit(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_hasOpenGLFramebufferBlit {
  fn hasOpenGLFramebufferBlit(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
impl<'a> /*trait*/ QOpenGLFramebufferObject_hasOpenGLFramebufferBlit for () {
  fn hasOpenGLFramebufferBlit(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv()};
    unsafe {_ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn texture<T: QOpenGLFramebufferObject_texture>(&mut self, value: T) -> i32 {
    value.texture(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_texture {
  fn texture(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::texture();
impl<'a> /*trait*/ QOpenGLFramebufferObject_texture for () {
  fn texture(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7textureEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject7textureEv()};
    return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QSize & size, GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (&'a  QSize, u32) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERK5QSizej()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN24QOpenGLFramebufferObjectC1ERK5QSizej(qthis, arg0, arg1)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn release<T: QOpenGLFramebufferObject_release>(&mut self, value: T) -> i32 {
    value.release(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_release {
  fn release(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::release();
impl<'a> /*trait*/ QOpenGLFramebufferObject_release for () {
  fn release(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject7releaseEv()};
    unsafe {_ZN24QOpenGLFramebufferObject7releaseEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn hasOpenGLFramebufferObjects<T: QOpenGLFramebufferObject_hasOpenGLFramebufferObjects>(&mut self, value: T) -> i32 {
    value.hasOpenGLFramebufferObjects(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_hasOpenGLFramebufferObjects {
  fn hasOpenGLFramebufferObjects(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
impl<'a> /*trait*/ QOpenGLFramebufferObject_hasOpenGLFramebufferObjects for () {
  fn hasOpenGLFramebufferObjects(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv()};
    unsafe {_ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn toImage<T: QOpenGLFramebufferObject_toImage>(&mut self, value: T) -> i32 {
    value.toImage(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_toImage {
  fn toImage(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: QImage QOpenGLFramebufferObject::toImage(bool flipped);
impl<'a> /*trait*/ QOpenGLFramebufferObject_toImage for (i8) {
  fn toImage(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7toImageEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZNK24QOpenGLFramebufferObject7toImageEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn handle<T: QOpenGLFramebufferObject_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_handle {
  fn handle(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: QOpenGLFramebufferObject::GLuint QOpenGLFramebufferObject::handle();
impl<'a> /*trait*/ QOpenGLFramebufferObject_handle for () {
  fn handle(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6handleEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject6handleEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn height<T: QOpenGLFramebufferObject_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_height {
  fn height(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: int QOpenGLFramebufferObject::height();
impl<'a> /*trait*/ QOpenGLFramebufferObject_height for () {
  fn height(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6heightEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject6heightEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn blitFramebuffer<T: QOpenGLFramebufferObject_blitFramebuffer>(&mut self, value: T) -> i32 {
    value.blitFramebuffer(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_blitFramebuffer {
  fn blitFramebuffer(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
impl<'a> /*trait*/ QOpenGLFramebufferObject_blitFramebuffer for (&'a mut QOpenGLFramebufferObject, &'a mut QOpenGLFramebufferObject, u32, u32) {
  fn blitFramebuffer(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat & format);
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (i32, i32, &'a  QOpenGLFramebufferObjectFormat) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(qthis, arg0, arg1, arg2)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, const QRect & targetRect, QOpenGLFramebufferObject * source, const QRect & sourceRect, GLbitfield buffers, GLenum filter);
impl<'a> /*trait*/ QOpenGLFramebufferObject_blitFramebuffer for (&'a mut QOpenGLFramebufferObject, &'a  QRect, &'a mut QOpenGLFramebufferObject, &'a  QRect, u32, u32) {
  fn blitFramebuffer(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4  as c_uint;
    let arg5 = self.5  as c_uint;
    unsafe {_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

// proto: QImage QOpenGLFramebufferObject::toImage();
impl<'a> /*trait*/ QOpenGLFramebufferObject_toImage for () {
  fn toImage(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7toImageEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject7toImageEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn size<T: QOpenGLFramebufferObject_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_size {
  fn size(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: QSize QOpenGLFramebufferObject::size();
impl<'a> /*trait*/ QOpenGLFramebufferObject_size for () {
  fn size(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject4sizeEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject4sizeEv()};
    return 1;
  }
}

// proto: void QOpenGLFramebufferObject::NewQOpenGLFramebufferObject(const QOpenGLFramebufferObject & );
impl<'a> /*trait*/ QOpenGLFramebufferObject_NewQOpenGLFramebufferObject for (&'a  QOpenGLFramebufferObject) {
  fn NewQOpenGLFramebufferObject(self) -> QOpenGLFramebufferObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QOpenGLFramebufferObjectC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn FreeQOpenGLFramebufferObject<T: QOpenGLFramebufferObject_FreeQOpenGLFramebufferObject>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLFramebufferObject(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_FreeQOpenGLFramebufferObject {
  fn FreeQOpenGLFramebufferObject(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: void QOpenGLFramebufferObject::FreeQOpenGLFramebufferObject();
impl<'a> /*trait*/ QOpenGLFramebufferObject_FreeQOpenGLFramebufferObject for () {
  fn FreeQOpenGLFramebufferObject(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectD0Ev()};
    unsafe {_ZN24QOpenGLFramebufferObjectD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn format<T: QOpenGLFramebufferObject_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_format {
  fn format(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
impl<'a> /*trait*/ QOpenGLFramebufferObject_format for () {
  fn format(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6formatEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject6formatEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn bind<T: QOpenGLFramebufferObject_bind>(&mut self, value: T) -> i32 {
    value.bind(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_bind {
  fn bind(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::bind();
impl<'a> /*trait*/ QOpenGLFramebufferObject_bind for () {
  fn bind(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject4bindEv()};
    unsafe {_ZN24QOpenGLFramebufferObject4bindEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn isBound<T: QOpenGLFramebufferObject_isBound>(&mut self, value: T) -> i32 {
    value.isBound(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_isBound {
  fn isBound(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: bool QOpenGLFramebufferObject::isBound();
impl<'a> /*trait*/ QOpenGLFramebufferObject_isBound for () {
  fn isBound(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7isBoundEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject7isBoundEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn width<T: QOpenGLFramebufferObject_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QOpenGLFramebufferObject_width {
  fn width(self, this: &mut QOpenGLFramebufferObject) -> i32;
}

// proto: int QOpenGLFramebufferObject::width();
impl<'a> /*trait*/ QOpenGLFramebufferObject_width for () {
  fn width(self, this: &mut QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject5widthEv()};
    unsafe {_ZNK24QOpenGLFramebufferObject5widthEv()};
    return 1;
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

