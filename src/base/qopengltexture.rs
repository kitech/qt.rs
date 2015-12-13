// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qopenglpixeltransferoptions::QOpenGLPixelTransferOptions;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QOpenGLTexture::bind();
  fn _ZN14QOpenGLTexture4bindEv() -> i32;
  // proto: void QOpenGLTexture::setFixedSamplePositions(bool fixed);
  fn _ZN14QOpenGLTexture23setFixedSamplePositionsEb(arg0: int8_t) -> i32;
  // proto: int QOpenGLTexture::height();
  fn _ZNK14QOpenGLTexture6heightEv() -> i32;
  // proto: bool QOpenGLTexture::isAutoMipMapGenerationEnabled();
  fn _ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv() -> i32;
  // proto: void QOpenGLTexture::setCompressedData(int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions(arg0: c_int, arg1: *const uint8_t, arg2: *const c_void) -> i32;
  // proto: void QOpenGLTexture::setMaximumLevelOfDetail(float value);
  fn _ZN14QOpenGLTexture23setMaximumLevelOfDetailEf(arg0: c_float) -> i32;
  // proto: void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled);
  fn _ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb(arg0: int8_t) -> i32;
  // proto: int QOpenGLTexture::depth();
  fn _ZNK14QOpenGLTexture5depthEv() -> i32;
  // proto: void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel);
  fn _ZN14QOpenGLTexture15generateMipMapsEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiPKvPK27QOpenGLPixelTransferOptions(arg0: c_int, arg1: c_int, arg2: *const uint8_t, arg3: *const c_void) -> i32;
  // proto: void QOpenGLTexture::setMipBaseLevel(int baseLevel);
  fn _ZN14QOpenGLTexture15setMipBaseLevelEi(arg0: c_int) -> i32;
  // proto: QPair<float, float> QOpenGLTexture::levelOfDetailRange();
  fn _ZNK14QOpenGLTexture18levelOfDetailRangeEv() -> i32;
  // proto: bool QOpenGLTexture::create();
  fn _ZN14QOpenGLTexture6createEv() -> i32;
  // proto: void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiiPKvPK27QOpenGLPixelTransferOptions(arg0: c_int, arg1: c_int, arg2: c_int, arg3: *const uint8_t, arg4: *const c_void) -> i32;
  // proto: void QOpenGLTexture::setLevelOfDetailRange(float min, float max);
  fn _ZN14QOpenGLTexture21setLevelOfDetailRangeEff(arg0: c_float, arg1: c_float) -> i32;
  // proto: void QOpenGLTexture::borderColor(unsigned int * border);
  fn _ZNK14QOpenGLTexture11borderColorEPj(arg0: *mut c_uint) -> i32;
  // proto: bool QOpenGLTexture::isStorageAllocated();
  fn _ZNK14QOpenGLTexture18isStorageAllocatedEv() -> i32;
  // proto: void QOpenGLTexture::borderColor(int * border);
  fn _ZNK14QOpenGLTexture11borderColorEPi(arg0: *mut c_int) -> i32;
  // proto: bool QOpenGLTexture::isTextureView();
  fn _ZNK14QOpenGLTexture13isTextureViewEv() -> i32;
  // proto: bool QOpenGLTexture::isFixedSamplePositions();
  fn _ZNK14QOpenGLTexture22isFixedSamplePositionsEv() -> i32;
  // proto: int QOpenGLTexture::faces();
  fn _ZNK14QOpenGLTexture5facesEv() -> i32;
  // proto: void QOpenGLTexture::setLayers(int layers);
  fn _ZN14QOpenGLTexture9setLayersEi(arg0: c_int) -> i32;
  // proto: void QOpenGLTexture::setCompressedData(int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions(arg0: c_int, arg1: *mut uint8_t, arg2: *const c_void) -> i32;
  // proto: int QOpenGLTexture::width();
  fn _ZNK14QOpenGLTexture5widthEv() -> i32;
  // proto: int QOpenGLTexture::layers();
  fn _ZNK14QOpenGLTexture6layersEv() -> i32;
  // proto: void QOpenGLTexture::borderColor(float * border);
  fn _ZNK14QOpenGLTexture11borderColorEPf(arg0: *mut c_float) -> i32;
  // proto: float QOpenGLTexture::minimumLevelOfDetail();
  fn _ZNK14QOpenGLTexture20minimumLevelOfDetailEv() -> i32;
  // proto: void QOpenGLTexture::setBorderColor(int r, int g, int b, int a);
  fn _ZN14QOpenGLTexture14setBorderColorEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QOpenGLTexture::setMinimumLevelOfDetail(float value);
  fn _ZN14QOpenGLTexture23setMinimumLevelOfDetailEf(arg0: c_float) -> i32;
  // proto: void QOpenGLTexture::setMipLevels(int levels);
  fn _ZN14QOpenGLTexture12setMipLevelsEi(arg0: c_int) -> i32;
  // proto: QPair<int, int> QOpenGLTexture::mipLevelRange();
  fn _ZNK14QOpenGLTexture13mipLevelRangeEv() -> i32;
  // proto: void QOpenGLTexture::setMipMaxLevel(int maxLevel);
  fn _ZN14QOpenGLTexture14setMipMaxLevelEi(arg0: c_int) -> i32;
  // proto: float QOpenGLTexture::levelofDetailBias();
  fn _ZNK14QOpenGLTexture17levelofDetailBiasEv() -> i32;
  // proto: int QOpenGLTexture::maximumMipLevels();
  fn _ZNK14QOpenGLTexture16maximumMipLevelsEv() -> i32;
  // proto: bool QOpenGLTexture::isBound(uint unit);
  fn _ZN14QOpenGLTexture7isBoundEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLTexture::setBorderColor(uint r, uint g, uint b, uint a);
  fn _ZN14QOpenGLTexture14setBorderColorEjjjj(arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) -> i32;
  // proto: void QOpenGLTexture::setMaximumAnisotropy(float anisotropy);
  fn _ZN14QOpenGLTexture20setMaximumAnisotropyEf(arg0: c_float) -> i32;
  // proto: void QOpenGLTexture::setSamples(int samples);
  fn _ZN14QOpenGLTexture10setSamplesEi(arg0: c_int) -> i32;
  // proto: int QOpenGLTexture::mipLevels();
  fn _ZNK14QOpenGLTexture9mipLevelsEv() -> i32;
  // proto: void QOpenGLTexture::setLevelofDetailBias(float bias);
  fn _ZN14QOpenGLTexture20setLevelofDetailBiasEf(arg0: c_float) -> i32;
  // proto: QOpenGLTexture::GLuint QOpenGLTexture::textureId();
  fn _ZNK14QOpenGLTexture9textureIdEv() -> i32;
  // proto: void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel);
  fn _ZN14QOpenGLTexture16setMipLevelRangeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QOpenGLTexture::allocateStorage();
  fn _ZN14QOpenGLTexture15allocateStorageEv() -> i32;
  // proto: void QOpenGLTexture::FreeQOpenGLTexture();
  fn _ZN14QOpenGLTextureD0Ev() -> i32;
  // proto: int QOpenGLTexture::mipMaxLevel();
  fn _ZNK14QOpenGLTexture11mipMaxLevelEv() -> i32;
  // proto: void QOpenGLTexture::setBorderColor(QColor color);
  fn _ZN14QOpenGLTexture14setBorderColorE6QColor(arg0: *mut c_void) -> i32;
  // proto: void QOpenGLTexture::destroy();
  fn _ZN14QOpenGLTexture7destroyEv() -> i32;
  // proto: void QOpenGLTexture::generateMipMaps();
  fn _ZN14QOpenGLTexture15generateMipMapsEv() -> i32;
  // proto: void QOpenGLTexture::release();
  fn _ZN14QOpenGLTexture7releaseEv() -> i32;
  // proto: float QOpenGLTexture::maximumAnisotropy();
  fn _ZNK14QOpenGLTexture17maximumAnisotropyEv() -> i32;
  // proto: float QOpenGLTexture::maximumLevelOfDetail();
  fn _ZNK14QOpenGLTexture20maximumLevelOfDetailEv() -> i32;
  // proto: void QOpenGLTexture::setSize(int width, int height, int depth);
  fn _ZN14QOpenGLTexture7setSizeEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: bool QOpenGLTexture::isCreated();
  fn _ZNK14QOpenGLTexture9isCreatedEv() -> i32;
  // proto: bool QOpenGLTexture::isBound();
  fn _ZNK14QOpenGLTexture7isBoundEv() -> i32;
  // proto: void QOpenGLTexture::setBorderColor(float r, float g, float b, float a);
  fn _ZN14QOpenGLTexture14setBorderColorEffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: int QOpenGLTexture::samples();
  fn _ZNK14QOpenGLTexture7samplesEv() -> i32;
  // proto: int QOpenGLTexture::mipBaseLevel();
  fn _ZNK14QOpenGLTexture12mipBaseLevelEv() -> i32;
  // proto: QColor QOpenGLTexture::borderColor();
  fn _ZNK14QOpenGLTexture11borderColorEv() -> i32;
  // proto: void QOpenGLTexture::NewQOpenGLTexture(const QOpenGLTexture & );
  fn _ZN14QOpenGLTextureC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiiPvPK27QOpenGLPixelTransferOptions(arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut uint8_t, arg4: *const c_void) -> i32;
  // proto: void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiPvPK27QOpenGLPixelTransferOptions(arg0: c_int, arg1: c_int, arg2: *mut uint8_t, arg3: *const c_void) -> i32;
}

// body block begin
// class sizeof(QOpenGLTexture)=1
pub struct QOpenGLTexture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLTexture {
  pub fn bind<T: QOpenGLTexture_bind>(&mut self, value: T) -> i32 {
    value.bind(self);
    return 1;
  }
}

pub trait QOpenGLTexture_bind {
  fn bind(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::bind();
impl<'a> /*trait*/ QOpenGLTexture_bind for () {
  fn bind(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture4bindEv()};
    unsafe {_ZN14QOpenGLTexture4bindEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setFixedSamplePositions<T: QOpenGLTexture_setFixedSamplePositions>(&mut self, value: T) -> i32 {
    value.setFixedSamplePositions(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setFixedSamplePositions {
  fn setFixedSamplePositions(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setFixedSamplePositions(bool fixed);
impl<'a> /*trait*/ QOpenGLTexture_setFixedSamplePositions for (i8) {
  fn setFixedSamplePositions(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setFixedSamplePositionsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QOpenGLTexture23setFixedSamplePositionsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn height<T: QOpenGLTexture_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QOpenGLTexture_height {
  fn height(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::height();
impl<'a> /*trait*/ QOpenGLTexture_height for () {
  fn height(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture6heightEv()};
    unsafe {_ZNK14QOpenGLTexture6heightEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isAutoMipMapGenerationEnabled<T: QOpenGLTexture_isAutoMipMapGenerationEnabled>(&mut self, value: T) -> i32 {
    value.isAutoMipMapGenerationEnabled(self);
    return 1;
  }
}

pub trait QOpenGLTexture_isAutoMipMapGenerationEnabled {
  fn isAutoMipMapGenerationEnabled(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::isAutoMipMapGenerationEnabled();
impl<'a> /*trait*/ QOpenGLTexture_isAutoMipMapGenerationEnabled for () {
  fn isAutoMipMapGenerationEnabled(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv()};
    unsafe {_ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setCompressedData<T: QOpenGLTexture_setCompressedData>(&mut self, value: T) -> i32 {
    value.setCompressedData(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setCompressedData {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setCompressedData(int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const uint8_t;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMaximumLevelOfDetail<T: QOpenGLTexture_setMaximumLevelOfDetail>(&mut self, value: T) -> i32 {
    value.setMaximumLevelOfDetail(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMaximumLevelOfDetail {
  fn setMaximumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMaximumLevelOfDetail(float value);
impl<'a> /*trait*/ QOpenGLTexture_setMaximumLevelOfDetail for (f32) {
  fn setMaximumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setMaximumLevelOfDetailEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN14QOpenGLTexture23setMaximumLevelOfDetailEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setAutoMipMapGenerationEnabled<T: QOpenGLTexture_setAutoMipMapGenerationEnabled>(&mut self, value: T) -> i32 {
    value.setAutoMipMapGenerationEnabled(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setAutoMipMapGenerationEnabled {
  fn setAutoMipMapGenerationEnabled(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled);
impl<'a> /*trait*/ QOpenGLTexture_setAutoMipMapGenerationEnabled for (i8) {
  fn setAutoMipMapGenerationEnabled(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn depth<T: QOpenGLTexture_depth>(&mut self, value: T) -> i32 {
    value.depth(self);
    return 1;
  }
}

pub trait QOpenGLTexture_depth {
  fn depth(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::depth();
impl<'a> /*trait*/ QOpenGLTexture_depth for () {
  fn depth(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5depthEv()};
    unsafe {_ZNK14QOpenGLTexture5depthEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn generateMipMaps<T: QOpenGLTexture_generateMipMaps>(&mut self, value: T) -> i32 {
    value.generateMipMaps(self);
    return 1;
  }
}

pub trait QOpenGLTexture_generateMipMaps {
  fn generateMipMaps(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel);
impl<'a> /*trait*/ QOpenGLTexture_generateMipMaps for (i32, i8) {
  fn generateMipMaps(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15generateMipMapsEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN14QOpenGLTexture15generateMipMapsEib(arg0, arg1)};
    return 1;
  }
}

// proto: void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const uint8_t;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiPKvPK27QOpenGLPixelTransferOptions(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipBaseLevel<T: QOpenGLTexture_setMipBaseLevel>(&mut self, value: T) -> i32 {
    value.setMipBaseLevel(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMipBaseLevel {
  fn setMipBaseLevel(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMipBaseLevel(int baseLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipBaseLevel for (i32) {
  fn setMipBaseLevel(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15setMipBaseLevelEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QOpenGLTexture15setMipBaseLevelEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn levelOfDetailRange<T: QOpenGLTexture_levelOfDetailRange>(&mut self, value: T) -> i32 {
    value.levelOfDetailRange(self);
    return 1;
  }
}

pub trait QOpenGLTexture_levelOfDetailRange {
  fn levelOfDetailRange(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: QPair<float, float> QOpenGLTexture::levelOfDetailRange();
impl<'a> /*trait*/ QOpenGLTexture_levelOfDetailRange for () {
  fn levelOfDetailRange(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture18levelOfDetailRangeEv()};
    unsafe {_ZNK14QOpenGLTexture18levelOfDetailRangeEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn create<T: QOpenGLTexture_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QOpenGLTexture_create {
  fn create(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::create();
impl<'a> /*trait*/ QOpenGLTexture_create for () {
  fn create(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture6createEv()};
    unsafe {_ZN14QOpenGLTexture6createEv()};
    return 1;
  }
}

// proto: void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *const uint8_t;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiiPKvPK27QOpenGLPixelTransferOptions(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setLevelOfDetailRange<T: QOpenGLTexture_setLevelOfDetailRange>(&mut self, value: T) -> i32 {
    value.setLevelOfDetailRange(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setLevelOfDetailRange {
  fn setLevelOfDetailRange(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setLevelOfDetailRange(float min, float max);
impl<'a> /*trait*/ QOpenGLTexture_setLevelOfDetailRange for (f32, f32) {
  fn setLevelOfDetailRange(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture21setLevelOfDetailRangeEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    unsafe {_ZN14QOpenGLTexture21setLevelOfDetailRangeEff(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn borderColor<T: QOpenGLTexture_borderColor>(&mut self, value: T) -> i32 {
    value.borderColor(self);
    return 1;
  }
}

pub trait QOpenGLTexture_borderColor {
  fn borderColor(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::borderColor(unsigned int * border);
impl<'a> /*trait*/ QOpenGLTexture_borderColor for (&'a mut u32) {
  fn borderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEPj()};
    let arg0 = self  as *mut c_uint;
    unsafe {_ZNK14QOpenGLTexture11borderColorEPj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isStorageAllocated<T: QOpenGLTexture_isStorageAllocated>(&mut self, value: T) -> i32 {
    value.isStorageAllocated(self);
    return 1;
  }
}

pub trait QOpenGLTexture_isStorageAllocated {
  fn isStorageAllocated(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::isStorageAllocated();
impl<'a> /*trait*/ QOpenGLTexture_isStorageAllocated for () {
  fn isStorageAllocated(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture18isStorageAllocatedEv()};
    unsafe {_ZNK14QOpenGLTexture18isStorageAllocatedEv()};
    return 1;
  }
}

// proto: void QOpenGLTexture::borderColor(int * border);
impl<'a> /*trait*/ QOpenGLTexture_borderColor for (&'a mut i32) {
  fn borderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEPi()};
    let arg0 = self  as *mut c_int;
    unsafe {_ZNK14QOpenGLTexture11borderColorEPi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isTextureView<T: QOpenGLTexture_isTextureView>(&mut self, value: T) -> i32 {
    value.isTextureView(self);
    return 1;
  }
}

pub trait QOpenGLTexture_isTextureView {
  fn isTextureView(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::isTextureView();
impl<'a> /*trait*/ QOpenGLTexture_isTextureView for () {
  fn isTextureView(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture13isTextureViewEv()};
    unsafe {_ZNK14QOpenGLTexture13isTextureViewEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isFixedSamplePositions<T: QOpenGLTexture_isFixedSamplePositions>(&mut self, value: T) -> i32 {
    value.isFixedSamplePositions(self);
    return 1;
  }
}

pub trait QOpenGLTexture_isFixedSamplePositions {
  fn isFixedSamplePositions(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::isFixedSamplePositions();
impl<'a> /*trait*/ QOpenGLTexture_isFixedSamplePositions for () {
  fn isFixedSamplePositions(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture22isFixedSamplePositionsEv()};
    unsafe {_ZNK14QOpenGLTexture22isFixedSamplePositionsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn faces<T: QOpenGLTexture_faces>(&mut self, value: T) -> i32 {
    value.faces(self);
    return 1;
  }
}

pub trait QOpenGLTexture_faces {
  fn faces(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::faces();
impl<'a> /*trait*/ QOpenGLTexture_faces for () {
  fn faces(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5facesEv()};
    unsafe {_ZNK14QOpenGLTexture5facesEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setLayers<T: QOpenGLTexture_setLayers>(&mut self, value: T) -> i32 {
    value.setLayers(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setLayers {
  fn setLayers(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setLayers(int layers);
impl<'a> /*trait*/ QOpenGLTexture_setLayers for (i32) {
  fn setLayers(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture9setLayersEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QOpenGLTexture9setLayersEi(arg0)};
    return 1;
  }
}

// proto: void QOpenGLTexture::setCompressedData(int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn width<T: QOpenGLTexture_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QOpenGLTexture_width {
  fn width(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::width();
impl<'a> /*trait*/ QOpenGLTexture_width for () {
  fn width(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5widthEv()};
    unsafe {_ZNK14QOpenGLTexture5widthEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn layers<T: QOpenGLTexture_layers>(&mut self, value: T) -> i32 {
    value.layers(self);
    return 1;
  }
}

pub trait QOpenGLTexture_layers {
  fn layers(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::layers();
impl<'a> /*trait*/ QOpenGLTexture_layers for () {
  fn layers(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture6layersEv()};
    unsafe {_ZNK14QOpenGLTexture6layersEv()};
    return 1;
  }
}

// proto: void QOpenGLTexture::borderColor(float * border);
impl<'a> /*trait*/ QOpenGLTexture_borderColor for (&'a mut f32) {
  fn borderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEPf()};
    let arg0 = self  as *mut c_float;
    unsafe {_ZNK14QOpenGLTexture11borderColorEPf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn minimumLevelOfDetail<T: QOpenGLTexture_minimumLevelOfDetail>(&mut self, value: T) -> i32 {
    value.minimumLevelOfDetail(self);
    return 1;
  }
}

pub trait QOpenGLTexture_minimumLevelOfDetail {
  fn minimumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: float QOpenGLTexture::minimumLevelOfDetail();
impl<'a> /*trait*/ QOpenGLTexture_minimumLevelOfDetail for () {
  fn minimumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture20minimumLevelOfDetailEv()};
    unsafe {_ZNK14QOpenGLTexture20minimumLevelOfDetailEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setBorderColor<T: QOpenGLTexture_setBorderColor>(&mut self, value: T) -> i32 {
    value.setBorderColor(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setBorderColor {
  fn setBorderColor(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setBorderColor(int r, int g, int b, int a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (i32, i32, i32, i32) {
  fn setBorderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN14QOpenGLTexture14setBorderColorEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMinimumLevelOfDetail<T: QOpenGLTexture_setMinimumLevelOfDetail>(&mut self, value: T) -> i32 {
    value.setMinimumLevelOfDetail(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMinimumLevelOfDetail {
  fn setMinimumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMinimumLevelOfDetail(float value);
impl<'a> /*trait*/ QOpenGLTexture_setMinimumLevelOfDetail for (f32) {
  fn setMinimumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setMinimumLevelOfDetailEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN14QOpenGLTexture23setMinimumLevelOfDetailEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipLevels<T: QOpenGLTexture_setMipLevels>(&mut self, value: T) -> i32 {
    value.setMipLevels(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMipLevels {
  fn setMipLevels(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMipLevels(int levels);
impl<'a> /*trait*/ QOpenGLTexture_setMipLevels for (i32) {
  fn setMipLevels(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture12setMipLevelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QOpenGLTexture12setMipLevelsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipLevelRange<T: QOpenGLTexture_mipLevelRange>(&mut self, value: T) -> i32 {
    value.mipLevelRange(self);
    return 1;
  }
}

pub trait QOpenGLTexture_mipLevelRange {
  fn mipLevelRange(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: QPair<int, int> QOpenGLTexture::mipLevelRange();
impl<'a> /*trait*/ QOpenGLTexture_mipLevelRange for () {
  fn mipLevelRange(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture13mipLevelRangeEv()};
    unsafe {_ZNK14QOpenGLTexture13mipLevelRangeEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipMaxLevel<T: QOpenGLTexture_setMipMaxLevel>(&mut self, value: T) -> i32 {
    value.setMipMaxLevel(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMipMaxLevel {
  fn setMipMaxLevel(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMipMaxLevel(int maxLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipMaxLevel for (i32) {
  fn setMipMaxLevel(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setMipMaxLevelEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QOpenGLTexture14setMipMaxLevelEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn levelofDetailBias<T: QOpenGLTexture_levelofDetailBias>(&mut self, value: T) -> i32 {
    value.levelofDetailBias(self);
    return 1;
  }
}

pub trait QOpenGLTexture_levelofDetailBias {
  fn levelofDetailBias(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: float QOpenGLTexture::levelofDetailBias();
impl<'a> /*trait*/ QOpenGLTexture_levelofDetailBias for () {
  fn levelofDetailBias(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture17levelofDetailBiasEv()};
    unsafe {_ZNK14QOpenGLTexture17levelofDetailBiasEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn maximumMipLevels<T: QOpenGLTexture_maximumMipLevels>(&mut self, value: T) -> i32 {
    value.maximumMipLevels(self);
    return 1;
  }
}

pub trait QOpenGLTexture_maximumMipLevels {
  fn maximumMipLevels(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::maximumMipLevels();
impl<'a> /*trait*/ QOpenGLTexture_maximumMipLevels for () {
  fn maximumMipLevels(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture16maximumMipLevelsEv()};
    unsafe {_ZNK14QOpenGLTexture16maximumMipLevelsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isBound<T: QOpenGLTexture_isBound>(&mut self, value: T) -> i32 {
    value.isBound(self);
    return 1;
  }
}

pub trait QOpenGLTexture_isBound {
  fn isBound(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::isBound(uint unit);
impl<'a> /*trait*/ QOpenGLTexture_isBound for (u32) {
  fn isBound(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7isBoundEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN14QOpenGLTexture7isBoundEj(arg0)};
    return 1;
  }
}

// proto: void QOpenGLTexture::setBorderColor(uint r, uint g, uint b, uint a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (u32, u32, u32, u32) {
  fn setBorderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN14QOpenGLTexture14setBorderColorEjjjj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMaximumAnisotropy<T: QOpenGLTexture_setMaximumAnisotropy>(&mut self, value: T) -> i32 {
    value.setMaximumAnisotropy(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMaximumAnisotropy {
  fn setMaximumAnisotropy(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMaximumAnisotropy(float anisotropy);
impl<'a> /*trait*/ QOpenGLTexture_setMaximumAnisotropy for (f32) {
  fn setMaximumAnisotropy(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture20setMaximumAnisotropyEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN14QOpenGLTexture20setMaximumAnisotropyEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setSamples<T: QOpenGLTexture_setSamples>(&mut self, value: T) -> i32 {
    value.setSamples(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setSamples {
  fn setSamples(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setSamples(int samples);
impl<'a> /*trait*/ QOpenGLTexture_setSamples for (i32) {
  fn setSamples(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture10setSamplesEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QOpenGLTexture10setSamplesEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipLevels<T: QOpenGLTexture_mipLevels>(&mut self, value: T) -> i32 {
    value.mipLevels(self);
    return 1;
  }
}

pub trait QOpenGLTexture_mipLevels {
  fn mipLevels(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::mipLevels();
impl<'a> /*trait*/ QOpenGLTexture_mipLevels for () {
  fn mipLevels(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9mipLevelsEv()};
    unsafe {_ZNK14QOpenGLTexture9mipLevelsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setLevelofDetailBias<T: QOpenGLTexture_setLevelofDetailBias>(&mut self, value: T) -> i32 {
    value.setLevelofDetailBias(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setLevelofDetailBias {
  fn setLevelofDetailBias(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setLevelofDetailBias(float bias);
impl<'a> /*trait*/ QOpenGLTexture_setLevelofDetailBias for (f32) {
  fn setLevelofDetailBias(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture20setLevelofDetailBiasEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN14QOpenGLTexture20setLevelofDetailBiasEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn textureId<T: QOpenGLTexture_textureId>(&mut self, value: T) -> i32 {
    value.textureId(self);
    return 1;
  }
}

pub trait QOpenGLTexture_textureId {
  fn textureId(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: QOpenGLTexture::GLuint QOpenGLTexture::textureId();
impl<'a> /*trait*/ QOpenGLTexture_textureId for () {
  fn textureId(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9textureIdEv()};
    unsafe {_ZNK14QOpenGLTexture9textureIdEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipLevelRange<T: QOpenGLTexture_setMipLevelRange>(&mut self, value: T) -> i32 {
    value.setMipLevelRange(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setMipLevelRange {
  fn setMipLevelRange(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipLevelRange for (i32, i32) {
  fn setMipLevelRange(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture16setMipLevelRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN14QOpenGLTexture16setMipLevelRangeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn allocateStorage<T: QOpenGLTexture_allocateStorage>(&mut self, value: T) -> i32 {
    value.allocateStorage(self);
    return 1;
  }
}

pub trait QOpenGLTexture_allocateStorage {
  fn allocateStorage(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::allocateStorage();
impl<'a> /*trait*/ QOpenGLTexture_allocateStorage for () {
  fn allocateStorage(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15allocateStorageEv()};
    unsafe {_ZN14QOpenGLTexture15allocateStorageEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn FreeQOpenGLTexture<T: QOpenGLTexture_FreeQOpenGLTexture>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLTexture(self);
    return 1;
  }
}

pub trait QOpenGLTexture_FreeQOpenGLTexture {
  fn FreeQOpenGLTexture(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::FreeQOpenGLTexture();
impl<'a> /*trait*/ QOpenGLTexture_FreeQOpenGLTexture for () {
  fn FreeQOpenGLTexture(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTextureD0Ev()};
    unsafe {_ZN14QOpenGLTextureD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipMaxLevel<T: QOpenGLTexture_mipMaxLevel>(&mut self, value: T) -> i32 {
    value.mipMaxLevel(self);
    return 1;
  }
}

pub trait QOpenGLTexture_mipMaxLevel {
  fn mipMaxLevel(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::mipMaxLevel();
impl<'a> /*trait*/ QOpenGLTexture_mipMaxLevel for () {
  fn mipMaxLevel(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11mipMaxLevelEv()};
    unsafe {_ZNK14QOpenGLTexture11mipMaxLevelEv()};
    return 1;
  }
}

// proto: void QOpenGLTexture::setBorderColor(QColor color);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (QColor) {
  fn setBorderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorE6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLTexture14setBorderColorE6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn destroy<T: QOpenGLTexture_destroy>(&mut self, value: T) -> i32 {
    value.destroy(self);
    return 1;
  }
}

pub trait QOpenGLTexture_destroy {
  fn destroy(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::destroy();
impl<'a> /*trait*/ QOpenGLTexture_destroy for () {
  fn destroy(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7destroyEv()};
    unsafe {_ZN14QOpenGLTexture7destroyEv()};
    return 1;
  }
}

// proto: void QOpenGLTexture::generateMipMaps();
impl<'a> /*trait*/ QOpenGLTexture_generateMipMaps for () {
  fn generateMipMaps(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15generateMipMapsEv()};
    unsafe {_ZN14QOpenGLTexture15generateMipMapsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn release<T: QOpenGLTexture_release>(&mut self, value: T) -> i32 {
    value.release(self);
    return 1;
  }
}

pub trait QOpenGLTexture_release {
  fn release(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::release();
impl<'a> /*trait*/ QOpenGLTexture_release for () {
  fn release(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7releaseEv()};
    unsafe {_ZN14QOpenGLTexture7releaseEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn maximumAnisotropy<T: QOpenGLTexture_maximumAnisotropy>(&mut self, value: T) -> i32 {
    value.maximumAnisotropy(self);
    return 1;
  }
}

pub trait QOpenGLTexture_maximumAnisotropy {
  fn maximumAnisotropy(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: float QOpenGLTexture::maximumAnisotropy();
impl<'a> /*trait*/ QOpenGLTexture_maximumAnisotropy for () {
  fn maximumAnisotropy(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture17maximumAnisotropyEv()};
    unsafe {_ZNK14QOpenGLTexture17maximumAnisotropyEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn maximumLevelOfDetail<T: QOpenGLTexture_maximumLevelOfDetail>(&mut self, value: T) -> i32 {
    value.maximumLevelOfDetail(self);
    return 1;
  }
}

pub trait QOpenGLTexture_maximumLevelOfDetail {
  fn maximumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: float QOpenGLTexture::maximumLevelOfDetail();
impl<'a> /*trait*/ QOpenGLTexture_maximumLevelOfDetail for () {
  fn maximumLevelOfDetail(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture20maximumLevelOfDetailEv()};
    unsafe {_ZNK14QOpenGLTexture20maximumLevelOfDetailEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setSize<T: QOpenGLTexture_setSize>(&mut self, value: T) -> i32 {
    value.setSize(self);
    return 1;
  }
}

pub trait QOpenGLTexture_setSize {
  fn setSize(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: void QOpenGLTexture::setSize(int width, int height, int depth);
impl<'a> /*trait*/ QOpenGLTexture_setSize for (i32, i32, i32) {
  fn setSize(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7setSizeEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN14QOpenGLTexture7setSizeEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isCreated<T: QOpenGLTexture_isCreated>(&mut self, value: T) -> i32 {
    value.isCreated(self);
    return 1;
  }
}

pub trait QOpenGLTexture_isCreated {
  fn isCreated(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: bool QOpenGLTexture::isCreated();
impl<'a> /*trait*/ QOpenGLTexture_isCreated for () {
  fn isCreated(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9isCreatedEv()};
    unsafe {_ZNK14QOpenGLTexture9isCreatedEv()};
    return 1;
  }
}

// proto: bool QOpenGLTexture::isBound();
impl<'a> /*trait*/ QOpenGLTexture_isBound for () {
  fn isBound(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture7isBoundEv()};
    unsafe {_ZNK14QOpenGLTexture7isBoundEv()};
    return 1;
  }
}

// proto: void QOpenGLTexture::setBorderColor(float r, float g, float b, float a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (f32, f32, f32, f32) {
  fn setBorderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN14QOpenGLTexture14setBorderColorEffff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn samples<T: QOpenGLTexture_samples>(&mut self, value: T) -> i32 {
    value.samples(self);
    return 1;
  }
}

pub trait QOpenGLTexture_samples {
  fn samples(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::samples();
impl<'a> /*trait*/ QOpenGLTexture_samples for () {
  fn samples(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture7samplesEv()};
    unsafe {_ZNK14QOpenGLTexture7samplesEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipBaseLevel<T: QOpenGLTexture_mipBaseLevel>(&mut self, value: T) -> i32 {
    value.mipBaseLevel(self);
    return 1;
  }
}

pub trait QOpenGLTexture_mipBaseLevel {
  fn mipBaseLevel(self, this: &mut QOpenGLTexture) -> i32;
}

// proto: int QOpenGLTexture::mipBaseLevel();
impl<'a> /*trait*/ QOpenGLTexture_mipBaseLevel for () {
  fn mipBaseLevel(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture12mipBaseLevelEv()};
    unsafe {_ZNK14QOpenGLTexture12mipBaseLevelEv()};
    return 1;
  }
}

// proto: QColor QOpenGLTexture::borderColor();
impl<'a> /*trait*/ QOpenGLTexture_borderColor for () {
  fn borderColor(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEv()};
    unsafe {_ZNK14QOpenGLTexture11borderColorEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn NewQOpenGLTexture<T: QOpenGLTexture_NewQOpenGLTexture>(value: T) -> QOpenGLTexture {
    let rsthis = value.NewQOpenGLTexture();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLTexture_NewQOpenGLTexture {
  fn NewQOpenGLTexture(self) -> QOpenGLTexture;
}

// proto: void QOpenGLTexture::NewQOpenGLTexture(const QOpenGLTexture & );
impl<'a> /*trait*/ QOpenGLTexture_NewQOpenGLTexture for (&'a  QOpenGLTexture) {
  fn NewQOpenGLTexture(self) -> QOpenGLTexture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTextureC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTextureC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLTexture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut uint8_t;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiiPvPK27QOpenGLPixelTransferOptions(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

// proto: void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, this: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut uint8_t;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiPvPK27QOpenGLPixelTransferOptions(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

