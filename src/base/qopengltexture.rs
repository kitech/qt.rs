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
  // proto:  void QOpenGLTexture::bind();
  fn _ZN14QOpenGLTexture4bindEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTexture::setFixedSamplePositions(bool fixed);
  fn _ZN14QOpenGLTexture23setFixedSamplePositionsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QOpenGLTexture::height();
  fn _ZNK14QOpenGLTexture6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QOpenGLTexture::isAutoMipMapGenerationEnabled();
  fn _ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLTexture::setCompressedData(int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions(qthis: *mut c_void, arg0: c_int, arg1: *const uint8_t, arg2: *mut c_void) ;
  // proto:  void QOpenGLTexture::setMaximumLevelOfDetail(float value);
  fn _ZN14QOpenGLTexture23setMaximumLevelOfDetailEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled);
  fn _ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QOpenGLTexture::depth();
  fn _ZNK14QOpenGLTexture5depthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel);
  fn _ZN14QOpenGLTexture15generateMipMapsEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiPKvPK27QOpenGLPixelTransferOptions(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const uint8_t, arg3: *mut c_void) ;
  // proto:  void QOpenGLTexture::setMipBaseLevel(int baseLevel);
  fn _ZN14QOpenGLTexture15setMipBaseLevelEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QPair<float, float> QOpenGLTexture::levelOfDetailRange();
  fn _ZNK14QOpenGLTexture18levelOfDetailRangeEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLTexture::create();
  fn _ZN14QOpenGLTexture6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiiPKvPK27QOpenGLPixelTransferOptions(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: *const uint8_t, arg4: *mut c_void) ;
  // proto:  void QOpenGLTexture::setLevelOfDetailRange(float min, float max);
  fn _ZN14QOpenGLTexture21setLevelOfDetailRangeEff(qthis: *mut c_void, arg0: c_float, arg1: c_float) ;
  // proto:  bool QOpenGLTexture::isStorageAllocated();
  fn _ZNK14QOpenGLTexture18isStorageAllocatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QOpenGLTexture::isTextureView();
  fn _ZNK14QOpenGLTexture13isTextureViewEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QOpenGLTexture::isFixedSamplePositions();
  fn _ZNK14QOpenGLTexture22isFixedSamplePositionsEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QOpenGLTexture::faces();
  fn _ZNK14QOpenGLTexture5facesEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTexture::setLayers(int layers);
  fn _ZN14QOpenGLTexture9setLayersEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QOpenGLTexture::setCompressedData(int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions(qthis: *mut c_void, arg0: c_int, arg1: *mut uint8_t, arg2: *mut c_void) ;
  // proto:  int QOpenGLTexture::width();
  fn _ZNK14QOpenGLTexture5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  int QOpenGLTexture::layers();
  fn _ZNK14QOpenGLTexture6layersEv(qthis: *mut c_void) -> c_int;
  // proto:  float QOpenGLTexture::minimumLevelOfDetail();
  fn _ZNK14QOpenGLTexture20minimumLevelOfDetailEv(qthis: *mut c_void) -> c_float;
  // proto:  void QOpenGLTexture::setBorderColor(int r, int g, int b, int a);
  fn _ZN14QOpenGLTexture14setBorderColorEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLTexture::setMinimumLevelOfDetail(float value);
  fn _ZN14QOpenGLTexture23setMinimumLevelOfDetailEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QOpenGLTexture::setMipLevels(int levels);
  fn _ZN14QOpenGLTexture12setMipLevelsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QPair<int, int> QOpenGLTexture::mipLevelRange();
  fn _ZNK14QOpenGLTexture13mipLevelRangeEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTexture::setMipMaxLevel(int maxLevel);
  fn _ZN14QOpenGLTexture14setMipMaxLevelEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  float QOpenGLTexture::levelofDetailBias();
  fn _ZNK14QOpenGLTexture17levelofDetailBiasEv(qthis: *mut c_void) -> c_float;
  // proto:  int QOpenGLTexture::maximumMipLevels();
  fn _ZNK14QOpenGLTexture16maximumMipLevelsEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QOpenGLTexture::isBound(uint unit);
  fn _ZN14QOpenGLTexture7isBoundEj(qthis: *mut c_void, arg0: c_uint) -> int8_t;
  // proto:  void QOpenGLTexture::setBorderColor(uint r, uint g, uint b, uint a);
  fn _ZN14QOpenGLTexture14setBorderColorEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) ;
  // proto:  void QOpenGLTexture::setMaximumAnisotropy(float anisotropy);
  fn _ZN14QOpenGLTexture20setMaximumAnisotropyEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QOpenGLTexture::setSamples(int samples);
  fn _ZN14QOpenGLTexture10setSamplesEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QOpenGLTexture::mipLevels();
  fn _ZNK14QOpenGLTexture9mipLevelsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTexture::setLevelofDetailBias(float bias);
  fn _ZN14QOpenGLTexture20setLevelofDetailBiasEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  QOpenGLTexture::GLuint QOpenGLTexture::textureId();
  fn _ZNK14QOpenGLTexture9textureIdEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel);
  fn _ZN14QOpenGLTexture16setMipLevelRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QOpenGLTexture::allocateStorage();
  fn _ZN14QOpenGLTexture15allocateStorageEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTexture::FreeQOpenGLTexture();
  fn _ZN14QOpenGLTextureD0Ev(qthis: *mut c_void) ;
  // proto:  int QOpenGLTexture::mipMaxLevel();
  fn _ZNK14QOpenGLTexture11mipMaxLevelEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTexture::setBorderColor(QColor color);
  fn _ZN14QOpenGLTexture14setBorderColorE6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLTexture::destroy();
  fn _ZN14QOpenGLTexture7destroyEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTexture::generateMipMaps();
  fn _ZN14QOpenGLTexture15generateMipMapsEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLTexture::release();
  fn _ZN14QOpenGLTexture7releaseEv(qthis: *mut c_void) ;
  // proto:  float QOpenGLTexture::maximumAnisotropy();
  fn _ZNK14QOpenGLTexture17maximumAnisotropyEv(qthis: *mut c_void) -> c_float;
  // proto:  float QOpenGLTexture::maximumLevelOfDetail();
  fn _ZNK14QOpenGLTexture20maximumLevelOfDetailEv(qthis: *mut c_void) -> c_float;
  // proto:  void QOpenGLTexture::setSize(int width, int height, int depth);
  fn _ZN14QOpenGLTexture7setSizeEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  bool QOpenGLTexture::isCreated();
  fn _ZNK14QOpenGLTexture9isCreatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QOpenGLTexture::isBound();
  fn _ZNK14QOpenGLTexture7isBoundEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLTexture::setBorderColor(float r, float g, float b, float a);
  fn _ZN14QOpenGLTexture14setBorderColorEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  int QOpenGLTexture::samples();
  fn _ZNK14QOpenGLTexture7samplesEv(qthis: *mut c_void) -> c_int;
  // proto:  int QOpenGLTexture::mipBaseLevel();
  fn _ZNK14QOpenGLTexture12mipBaseLevelEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLTexture::NewQOpenGLTexture(const QOpenGLTexture & );
  fn _ZN14QOpenGLTextureC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiiPvPK27QOpenGLPixelTransferOptions(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut uint8_t, arg4: *mut c_void) ;
  // proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
  fn _ZN14QOpenGLTexture17setCompressedDataEiiPvPK27QOpenGLPixelTransferOptions(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut uint8_t, arg3: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLTexture)=1
pub struct QOpenGLTexture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLTexture {
  pub fn bind<T: QOpenGLTexture_bind>(&mut self, value: T)  {
     value.bind(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_bind {
  fn bind(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::bind();
impl<'a> /*trait*/ QOpenGLTexture_bind for () {
  fn bind(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture4bindEv()};
     unsafe {_ZN14QOpenGLTexture4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setFixedSamplePositions<T: QOpenGLTexture_setFixedSamplePositions>(&mut self, value: T)  {
     value.setFixedSamplePositions(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setFixedSamplePositions {
  fn setFixedSamplePositions(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setFixedSamplePositions(bool fixed);
impl<'a> /*trait*/ QOpenGLTexture_setFixedSamplePositions for (i8) {
  fn setFixedSamplePositions(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setFixedSamplePositionsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QOpenGLTexture23setFixedSamplePositionsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn height<T: QOpenGLTexture_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_height {
  fn height(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::height();
impl<'a> /*trait*/ QOpenGLTexture_height for () {
  fn height(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture6heightEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isAutoMipMapGenerationEnabled<T: QOpenGLTexture_isAutoMipMapGenerationEnabled>(&mut self, value: T) -> i8 {
    return value.isAutoMipMapGenerationEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isAutoMipMapGenerationEnabled {
  fn isAutoMipMapGenerationEnabled(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::isAutoMipMapGenerationEnabled();
impl<'a> /*trait*/ QOpenGLTexture_isAutoMipMapGenerationEnabled for () {
  fn isAutoMipMapGenerationEnabled(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setCompressedData<T: QOpenGLTexture_setCompressedData>(&mut self, value: T)  {
     value.setCompressedData(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setCompressedData {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setCompressedData(int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const uint8_t;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMaximumLevelOfDetail<T: QOpenGLTexture_setMaximumLevelOfDetail>(&mut self, value: T)  {
     value.setMaximumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMaximumLevelOfDetail {
  fn setMaximumLevelOfDetail(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMaximumLevelOfDetail(float value);
impl<'a> /*trait*/ QOpenGLTexture_setMaximumLevelOfDetail for (f32) {
  fn setMaximumLevelOfDetail(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setMaximumLevelOfDetailEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture23setMaximumLevelOfDetailEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setAutoMipMapGenerationEnabled<T: QOpenGLTexture_setAutoMipMapGenerationEnabled>(&mut self, value: T)  {
     value.setAutoMipMapGenerationEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setAutoMipMapGenerationEnabled {
  fn setAutoMipMapGenerationEnabled(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled);
impl<'a> /*trait*/ QOpenGLTexture_setAutoMipMapGenerationEnabled for (i8) {
  fn setAutoMipMapGenerationEnabled(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn depth<T: QOpenGLTexture_depth>(&mut self, value: T) -> i32 {
    return value.depth(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_depth {
  fn depth(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::depth();
impl<'a> /*trait*/ QOpenGLTexture_depth for () {
  fn depth(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5depthEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn generateMipMaps<T: QOpenGLTexture_generateMipMaps>(&mut self, value: T)  {
     value.generateMipMaps(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_generateMipMaps {
  fn generateMipMaps(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel);
impl<'a> /*trait*/ QOpenGLTexture_generateMipMaps for (i32, i8) {
  fn generateMipMaps(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15generateMipMapsEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN14QOpenGLTexture15generateMipMapsEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const uint8_t;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiPKvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipBaseLevel<T: QOpenGLTexture_setMipBaseLevel>(&mut self, value: T)  {
     value.setMipBaseLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipBaseLevel {
  fn setMipBaseLevel(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMipBaseLevel(int baseLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipBaseLevel for (i32) {
  fn setMipBaseLevel(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15setMipBaseLevelEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture15setMipBaseLevelEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn levelOfDetailRange<T: QOpenGLTexture_levelOfDetailRange>(&mut self, value: T)  {
     value.levelOfDetailRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_levelOfDetailRange {
  fn levelOfDetailRange(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  QPair<float, float> QOpenGLTexture::levelOfDetailRange();
impl<'a> /*trait*/ QOpenGLTexture_levelOfDetailRange for () {
  fn levelOfDetailRange(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture18levelOfDetailRangeEv()};
     unsafe {_ZNK14QOpenGLTexture18levelOfDetailRangeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn create<T: QOpenGLTexture_create>(&mut self, value: T) -> i8 {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_create {
  fn create(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::create();
impl<'a> /*trait*/ QOpenGLTexture_create for () {
  fn create(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture6createEv()};
    let mut ret = unsafe {_ZN14QOpenGLTexture6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *const uint8_t;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiiPKvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setLevelOfDetailRange<T: QOpenGLTexture_setLevelOfDetailRange>(&mut self, value: T)  {
     value.setLevelOfDetailRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setLevelOfDetailRange {
  fn setLevelOfDetailRange(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setLevelOfDetailRange(float min, float max);
impl<'a> /*trait*/ QOpenGLTexture_setLevelOfDetailRange for (f32, f32) {
  fn setLevelOfDetailRange(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture21setLevelOfDetailRangeEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN14QOpenGLTexture21setLevelOfDetailRangeEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isStorageAllocated<T: QOpenGLTexture_isStorageAllocated>(&mut self, value: T) -> i8 {
    return value.isStorageAllocated(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isStorageAllocated {
  fn isStorageAllocated(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::isStorageAllocated();
impl<'a> /*trait*/ QOpenGLTexture_isStorageAllocated for () {
  fn isStorageAllocated(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture18isStorageAllocatedEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture18isStorageAllocatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isTextureView<T: QOpenGLTexture_isTextureView>(&mut self, value: T) -> i8 {
    return value.isTextureView(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isTextureView {
  fn isTextureView(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::isTextureView();
impl<'a> /*trait*/ QOpenGLTexture_isTextureView for () {
  fn isTextureView(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture13isTextureViewEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture13isTextureViewEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isFixedSamplePositions<T: QOpenGLTexture_isFixedSamplePositions>(&mut self, value: T) -> i8 {
    return value.isFixedSamplePositions(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isFixedSamplePositions {
  fn isFixedSamplePositions(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::isFixedSamplePositions();
impl<'a> /*trait*/ QOpenGLTexture_isFixedSamplePositions for () {
  fn isFixedSamplePositions(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture22isFixedSamplePositionsEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture22isFixedSamplePositionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn faces<T: QOpenGLTexture_faces>(&mut self, value: T) -> i32 {
    return value.faces(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_faces {
  fn faces(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::faces();
impl<'a> /*trait*/ QOpenGLTexture_faces for () {
  fn faces(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5facesEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture5facesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setLayers<T: QOpenGLTexture_setLayers>(&mut self, value: T)  {
     value.setLayers(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setLayers {
  fn setLayers(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setLayers(int layers);
impl<'a> /*trait*/ QOpenGLTexture_setLayers for (i32) {
  fn setLayers(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture9setLayersEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture9setLayersEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn width<T: QOpenGLTexture_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_width {
  fn width(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::width();
impl<'a> /*trait*/ QOpenGLTexture_width for () {
  fn width(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5widthEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn layers<T: QOpenGLTexture_layers>(&mut self, value: T) -> i32 {
    return value.layers(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_layers {
  fn layers(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::layers();
impl<'a> /*trait*/ QOpenGLTexture_layers for () {
  fn layers(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture6layersEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture6layersEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn minimumLevelOfDetail<T: QOpenGLTexture_minimumLevelOfDetail>(&mut self, value: T) -> f32 {
    return value.minimumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_minimumLevelOfDetail {
  fn minimumLevelOfDetail(self, rsthis: &mut QOpenGLTexture) -> f32;
}

// proto:  float QOpenGLTexture::minimumLevelOfDetail();
impl<'a> /*trait*/ QOpenGLTexture_minimumLevelOfDetail for () {
  fn minimumLevelOfDetail(self, rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture20minimumLevelOfDetailEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture20minimumLevelOfDetailEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setBorderColor<T: QOpenGLTexture_setBorderColor>(&mut self, value: T)  {
     value.setBorderColor(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setBorderColor {
  fn setBorderColor(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setBorderColor(int r, int g, int b, int a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (i32, i32, i32, i32) {
  fn setBorderColor(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN14QOpenGLTexture14setBorderColorEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMinimumLevelOfDetail<T: QOpenGLTexture_setMinimumLevelOfDetail>(&mut self, value: T)  {
     value.setMinimumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMinimumLevelOfDetail {
  fn setMinimumLevelOfDetail(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMinimumLevelOfDetail(float value);
impl<'a> /*trait*/ QOpenGLTexture_setMinimumLevelOfDetail for (f32) {
  fn setMinimumLevelOfDetail(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setMinimumLevelOfDetailEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture23setMinimumLevelOfDetailEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipLevels<T: QOpenGLTexture_setMipLevels>(&mut self, value: T)  {
     value.setMipLevels(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipLevels {
  fn setMipLevels(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMipLevels(int levels);
impl<'a> /*trait*/ QOpenGLTexture_setMipLevels for (i32) {
  fn setMipLevels(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture12setMipLevelsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture12setMipLevelsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipLevelRange<T: QOpenGLTexture_mipLevelRange>(&mut self, value: T)  {
     value.mipLevelRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipLevelRange {
  fn mipLevelRange(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  QPair<int, int> QOpenGLTexture::mipLevelRange();
impl<'a> /*trait*/ QOpenGLTexture_mipLevelRange for () {
  fn mipLevelRange(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture13mipLevelRangeEv()};
     unsafe {_ZNK14QOpenGLTexture13mipLevelRangeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipMaxLevel<T: QOpenGLTexture_setMipMaxLevel>(&mut self, value: T)  {
     value.setMipMaxLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipMaxLevel {
  fn setMipMaxLevel(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMipMaxLevel(int maxLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipMaxLevel for (i32) {
  fn setMipMaxLevel(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setMipMaxLevelEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture14setMipMaxLevelEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn levelofDetailBias<T: QOpenGLTexture_levelofDetailBias>(&mut self, value: T) -> f32 {
    return value.levelofDetailBias(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_levelofDetailBias {
  fn levelofDetailBias(self, rsthis: &mut QOpenGLTexture) -> f32;
}

// proto:  float QOpenGLTexture::levelofDetailBias();
impl<'a> /*trait*/ QOpenGLTexture_levelofDetailBias for () {
  fn levelofDetailBias(self, rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture17levelofDetailBiasEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture17levelofDetailBiasEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn maximumMipLevels<T: QOpenGLTexture_maximumMipLevels>(&mut self, value: T) -> i32 {
    return value.maximumMipLevels(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_maximumMipLevels {
  fn maximumMipLevels(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::maximumMipLevels();
impl<'a> /*trait*/ QOpenGLTexture_maximumMipLevels for () {
  fn maximumMipLevels(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture16maximumMipLevelsEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture16maximumMipLevelsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isBound<T: QOpenGLTexture_isBound>(&mut self, value: T) -> i8 {
    return value.isBound(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isBound {
  fn isBound(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::isBound(uint unit);
impl<'a> /*trait*/ QOpenGLTexture_isBound for (u32) {
  fn isBound(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7isBoundEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN14QOpenGLTexture7isBoundEj(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(uint r, uint g, uint b, uint a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (u32, u32, u32, u32) {
  fn setBorderColor(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN14QOpenGLTexture14setBorderColorEjjjj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMaximumAnisotropy<T: QOpenGLTexture_setMaximumAnisotropy>(&mut self, value: T)  {
     value.setMaximumAnisotropy(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMaximumAnisotropy {
  fn setMaximumAnisotropy(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMaximumAnisotropy(float anisotropy);
impl<'a> /*trait*/ QOpenGLTexture_setMaximumAnisotropy for (f32) {
  fn setMaximumAnisotropy(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture20setMaximumAnisotropyEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture20setMaximumAnisotropyEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setSamples<T: QOpenGLTexture_setSamples>(&mut self, value: T)  {
     value.setSamples(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setSamples {
  fn setSamples(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setSamples(int samples);
impl<'a> /*trait*/ QOpenGLTexture_setSamples for (i32) {
  fn setSamples(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture10setSamplesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture10setSamplesEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipLevels<T: QOpenGLTexture_mipLevels>(&mut self, value: T) -> i32 {
    return value.mipLevels(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipLevels {
  fn mipLevels(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::mipLevels();
impl<'a> /*trait*/ QOpenGLTexture_mipLevels for () {
  fn mipLevels(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9mipLevelsEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture9mipLevelsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setLevelofDetailBias<T: QOpenGLTexture_setLevelofDetailBias>(&mut self, value: T)  {
     value.setLevelofDetailBias(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setLevelofDetailBias {
  fn setLevelofDetailBias(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setLevelofDetailBias(float bias);
impl<'a> /*trait*/ QOpenGLTexture_setLevelofDetailBias for (f32) {
  fn setLevelofDetailBias(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture20setLevelofDetailBiasEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture20setLevelofDetailBiasEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn textureId<T: QOpenGLTexture_textureId>(&mut self, value: T)  {
     value.textureId(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_textureId {
  fn textureId(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  QOpenGLTexture::GLuint QOpenGLTexture::textureId();
impl<'a> /*trait*/ QOpenGLTexture_textureId for () {
  fn textureId(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9textureIdEv()};
     unsafe {_ZNK14QOpenGLTexture9textureIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setMipLevelRange<T: QOpenGLTexture_setMipLevelRange>(&mut self, value: T)  {
     value.setMipLevelRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipLevelRange {
  fn setMipLevelRange(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipLevelRange for (i32, i32) {
  fn setMipLevelRange(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture16setMipLevelRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN14QOpenGLTexture16setMipLevelRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn allocateStorage<T: QOpenGLTexture_allocateStorage>(&mut self, value: T)  {
     value.allocateStorage(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_allocateStorage {
  fn allocateStorage(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::allocateStorage();
impl<'a> /*trait*/ QOpenGLTexture_allocateStorage for () {
  fn allocateStorage(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15allocateStorageEv()};
     unsafe {_ZN14QOpenGLTexture15allocateStorageEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn FreeQOpenGLTexture<T: QOpenGLTexture_FreeQOpenGLTexture>(&mut self, value: T)  {
     value.FreeQOpenGLTexture(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_FreeQOpenGLTexture {
  fn FreeQOpenGLTexture(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::FreeQOpenGLTexture();
impl<'a> /*trait*/ QOpenGLTexture_FreeQOpenGLTexture for () {
  fn FreeQOpenGLTexture(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTextureD0Ev()};
     unsafe {_ZN14QOpenGLTextureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipMaxLevel<T: QOpenGLTexture_mipMaxLevel>(&mut self, value: T) -> i32 {
    return value.mipMaxLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipMaxLevel {
  fn mipMaxLevel(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::mipMaxLevel();
impl<'a> /*trait*/ QOpenGLTexture_mipMaxLevel for () {
  fn mipMaxLevel(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11mipMaxLevelEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture11mipMaxLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(QColor color);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (QColor) {
  fn setBorderColor(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorE6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture14setBorderColorE6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn destroy<T: QOpenGLTexture_destroy>(&mut self, value: T)  {
     value.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_destroy {
  fn destroy(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::destroy();
impl<'a> /*trait*/ QOpenGLTexture_destroy for () {
  fn destroy(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7destroyEv()};
     unsafe {_ZN14QOpenGLTexture7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::generateMipMaps();
impl<'a> /*trait*/ QOpenGLTexture_generateMipMaps for () {
  fn generateMipMaps(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15generateMipMapsEv()};
     unsafe {_ZN14QOpenGLTexture15generateMipMapsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn release<T: QOpenGLTexture_release>(&mut self, value: T)  {
     value.release(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_release {
  fn release(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::release();
impl<'a> /*trait*/ QOpenGLTexture_release for () {
  fn release(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7releaseEv()};
     unsafe {_ZN14QOpenGLTexture7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn maximumAnisotropy<T: QOpenGLTexture_maximumAnisotropy>(&mut self, value: T) -> f32 {
    return value.maximumAnisotropy(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_maximumAnisotropy {
  fn maximumAnisotropy(self, rsthis: &mut QOpenGLTexture) -> f32;
}

// proto:  float QOpenGLTexture::maximumAnisotropy();
impl<'a> /*trait*/ QOpenGLTexture_maximumAnisotropy for () {
  fn maximumAnisotropy(self, rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture17maximumAnisotropyEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture17maximumAnisotropyEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn maximumLevelOfDetail<T: QOpenGLTexture_maximumLevelOfDetail>(&mut self, value: T) -> f32 {
    return value.maximumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_maximumLevelOfDetail {
  fn maximumLevelOfDetail(self, rsthis: &mut QOpenGLTexture) -> f32;
}

// proto:  float QOpenGLTexture::maximumLevelOfDetail();
impl<'a> /*trait*/ QOpenGLTexture_maximumLevelOfDetail for () {
  fn maximumLevelOfDetail(self, rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture20maximumLevelOfDetailEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture20maximumLevelOfDetailEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn setSize<T: QOpenGLTexture_setSize>(&mut self, value: T)  {
     value.setSize(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setSize {
  fn setSize(self, rsthis: &mut QOpenGLTexture) ;
}

// proto:  void QOpenGLTexture::setSize(int width, int height, int depth);
impl<'a> /*trait*/ QOpenGLTexture_setSize for (i32, i32, i32) {
  fn setSize(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7setSizeEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN14QOpenGLTexture7setSizeEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn isCreated<T: QOpenGLTexture_isCreated>(&mut self, value: T) -> i8 {
    return value.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isCreated {
  fn isCreated(self, rsthis: &mut QOpenGLTexture) -> i8;
}

// proto:  bool QOpenGLTexture::isCreated();
impl<'a> /*trait*/ QOpenGLTexture_isCreated for () {
  fn isCreated(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9isCreatedEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isBound();
impl<'a> /*trait*/ QOpenGLTexture_isBound for () {
  fn isBound(self, rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture7isBoundEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture7isBoundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(float r, float g, float b, float a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor for (f32, f32, f32, f32) {
  fn setBorderColor(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN14QOpenGLTexture14setBorderColorEffff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn samples<T: QOpenGLTexture_samples>(&mut self, value: T) -> i32 {
    return value.samples(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_samples {
  fn samples(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::samples();
impl<'a> /*trait*/ QOpenGLTexture_samples for () {
  fn samples(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture7samplesEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture7samplesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLTexture {
  pub fn mipBaseLevel<T: QOpenGLTexture_mipBaseLevel>(&mut self, value: T) -> i32 {
    return value.mipBaseLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipBaseLevel {
  fn mipBaseLevel(self, rsthis: &mut QOpenGLTexture) -> i32;
}

// proto:  int QOpenGLTexture::mipBaseLevel();
impl<'a> /*trait*/ QOpenGLTexture_mipBaseLevel for () {
  fn mipBaseLevel(self, rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture12mipBaseLevelEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture12mipBaseLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLTextureC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLTexture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut uint8_t;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiiPvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData for (i32, i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self, rsthis: &mut QOpenGLTexture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut uint8_t;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiiPvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

