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
  // proto:  void QOpenGLTexture::borderColor(unsigned int * border);
  fn _ZNK14QOpenGLTexture11borderColorEPj(qthis: *mut c_void, arg0: *mut c_uint) ;
  // proto:  bool QOpenGLTexture::isStorageAllocated();
  fn _ZNK14QOpenGLTexture18isStorageAllocatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLTexture::borderColor(int * border);
  fn _ZNK14QOpenGLTexture11borderColorEPi(qthis: *mut c_void, arg0: *mut c_int) ;
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
  // proto:  void QOpenGLTexture::borderColor(float * border);
  fn _ZNK14QOpenGLTexture11borderColorEPf(qthis: *mut c_void, arg0: *mut c_float) ;
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
  // proto:  QColor QOpenGLTexture::borderColor();
  fn _ZNK14QOpenGLTexture11borderColorEv(qthis: *mut c_void) -> *mut c_void;
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

// proto:  void QOpenGLTexture::bind();
impl /*struct*/ QOpenGLTexture {
  pub fn bind<RetType, T: QOpenGLTexture_bind<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_bind<RetType> {
  fn bind(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::bind();
impl<'a> /*trait*/ QOpenGLTexture_bind<()> for () {
  fn bind(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture4bindEv()};
     unsafe {_ZN14QOpenGLTexture4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setFixedSamplePositions(bool fixed);
impl /*struct*/ QOpenGLTexture {
  pub fn setFixedSamplePositions<RetType, T: QOpenGLTexture_setFixedSamplePositions<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFixedSamplePositions(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setFixedSamplePositions<RetType> {
  fn setFixedSamplePositions(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setFixedSamplePositions(bool fixed);
impl<'a> /*trait*/ QOpenGLTexture_setFixedSamplePositions<()> for (i8) {
  fn setFixedSamplePositions(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setFixedSamplePositionsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QOpenGLTexture23setFixedSamplePositionsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QOpenGLTexture::height();
impl /*struct*/ QOpenGLTexture {
  pub fn height<RetType, T: QOpenGLTexture_height<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_height<RetType> {
  fn height(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::height();
impl<'a> /*trait*/ QOpenGLTexture_height<i32> for () {
  fn height(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture6heightEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isAutoMipMapGenerationEnabled();
impl /*struct*/ QOpenGLTexture {
  pub fn isAutoMipMapGenerationEnabled<RetType, T: QOpenGLTexture_isAutoMipMapGenerationEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isAutoMipMapGenerationEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isAutoMipMapGenerationEnabled<RetType> {
  fn isAutoMipMapGenerationEnabled(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::isAutoMipMapGenerationEnabled();
impl<'a> /*trait*/ QOpenGLTexture_isAutoMipMapGenerationEnabled<i8> for () {
  fn isAutoMipMapGenerationEnabled(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture29isAutoMipMapGenerationEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl /*struct*/ QOpenGLTexture {
  pub fn setCompressedData<RetType, T: QOpenGLTexture_setCompressedData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCompressedData(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setCompressedData<RetType> {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setCompressedData(int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData<()> for (i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const uint8_t;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiPKvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setMaximumLevelOfDetail(float value);
impl /*struct*/ QOpenGLTexture {
  pub fn setMaximumLevelOfDetail<RetType, T: QOpenGLTexture_setMaximumLevelOfDetail<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMaximumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMaximumLevelOfDetail<RetType> {
  fn setMaximumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMaximumLevelOfDetail(float value);
impl<'a> /*trait*/ QOpenGLTexture_setMaximumLevelOfDetail<()> for (f32) {
  fn setMaximumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setMaximumLevelOfDetailEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture23setMaximumLevelOfDetailEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled);
impl /*struct*/ QOpenGLTexture {
  pub fn setAutoMipMapGenerationEnabled<RetType, T: QOpenGLTexture_setAutoMipMapGenerationEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAutoMipMapGenerationEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setAutoMipMapGenerationEnabled<RetType> {
  fn setAutoMipMapGenerationEnabled(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled);
impl<'a> /*trait*/ QOpenGLTexture_setAutoMipMapGenerationEnabled<()> for (i8) {
  fn setAutoMipMapGenerationEnabled(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QOpenGLTexture30setAutoMipMapGenerationEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QOpenGLTexture::depth();
impl /*struct*/ QOpenGLTexture {
  pub fn depth<RetType, T: QOpenGLTexture_depth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_depth<RetType> {
  fn depth(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::depth();
impl<'a> /*trait*/ QOpenGLTexture_depth<i32> for () {
  fn depth(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5depthEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel);
impl /*struct*/ QOpenGLTexture {
  pub fn generateMipMaps<RetType, T: QOpenGLTexture_generateMipMaps<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.generateMipMaps(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_generateMipMaps<RetType> {
  fn generateMipMaps(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel);
impl<'a> /*trait*/ QOpenGLTexture_generateMipMaps<()> for (i32, i8) {
  fn generateMipMaps(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15generateMipMapsEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN14QOpenGLTexture15generateMipMapsEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData<()> for (i32, i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> () {
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

// proto:  void QOpenGLTexture::setMipBaseLevel(int baseLevel);
impl /*struct*/ QOpenGLTexture {
  pub fn setMipBaseLevel<RetType, T: QOpenGLTexture_setMipBaseLevel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMipBaseLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipBaseLevel<RetType> {
  fn setMipBaseLevel(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMipBaseLevel(int baseLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipBaseLevel<()> for (i32) {
  fn setMipBaseLevel(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15setMipBaseLevelEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture15setMipBaseLevelEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPair<float, float> QOpenGLTexture::levelOfDetailRange();
impl /*struct*/ QOpenGLTexture {
  pub fn levelOfDetailRange<RetType, T: QOpenGLTexture_levelOfDetailRange<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.levelOfDetailRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_levelOfDetailRange<RetType> {
  fn levelOfDetailRange(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  QPair<float, float> QOpenGLTexture::levelOfDetailRange();
impl<'a> /*trait*/ QOpenGLTexture_levelOfDetailRange<()> for () {
  fn levelOfDetailRange(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture18levelOfDetailRangeEv()};
     unsafe {_ZNK14QOpenGLTexture18levelOfDetailRangeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::create();
impl /*struct*/ QOpenGLTexture {
  pub fn create<RetType, T: QOpenGLTexture_create<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_create<RetType> {
  fn create(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::create();
impl<'a> /*trait*/ QOpenGLTexture_create<i8> for () {
  fn create(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture6createEv()};
    let mut ret = unsafe {_ZN14QOpenGLTexture6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData<()> for (i32, i32, i32, &'a  u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> () {
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

// proto:  void QOpenGLTexture::setLevelOfDetailRange(float min, float max);
impl /*struct*/ QOpenGLTexture {
  pub fn setLevelOfDetailRange<RetType, T: QOpenGLTexture_setLevelOfDetailRange<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLevelOfDetailRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setLevelOfDetailRange<RetType> {
  fn setLevelOfDetailRange(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setLevelOfDetailRange(float min, float max);
impl<'a> /*trait*/ QOpenGLTexture_setLevelOfDetailRange<()> for (f32, f32) {
  fn setLevelOfDetailRange(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture21setLevelOfDetailRangeEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN14QOpenGLTexture21setLevelOfDetailRangeEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::borderColor(unsigned int * border);
impl /*struct*/ QOpenGLTexture {
  pub fn borderColor<RetType, T: QOpenGLTexture_borderColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.borderColor(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_borderColor<RetType> {
  fn borderColor(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::borderColor(unsigned int * border);
impl<'a> /*trait*/ QOpenGLTexture_borderColor<()> for (&'a mut u32) {
  fn borderColor(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEPj()};
    let arg0 = self  as *mut c_uint;
     unsafe {_ZNK14QOpenGLTexture11borderColorEPj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isStorageAllocated();
impl /*struct*/ QOpenGLTexture {
  pub fn isStorageAllocated<RetType, T: QOpenGLTexture_isStorageAllocated<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isStorageAllocated(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isStorageAllocated<RetType> {
  fn isStorageAllocated(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::isStorageAllocated();
impl<'a> /*trait*/ QOpenGLTexture_isStorageAllocated<i8> for () {
  fn isStorageAllocated(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture18isStorageAllocatedEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture18isStorageAllocatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::borderColor(int * border);
impl<'a> /*trait*/ QOpenGLTexture_borderColor<()> for (&'a mut i32) {
  fn borderColor(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEPi()};
    let arg0 = self  as *mut c_int;
     unsafe {_ZNK14QOpenGLTexture11borderColorEPi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isTextureView();
impl /*struct*/ QOpenGLTexture {
  pub fn isTextureView<RetType, T: QOpenGLTexture_isTextureView<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isTextureView(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isTextureView<RetType> {
  fn isTextureView(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::isTextureView();
impl<'a> /*trait*/ QOpenGLTexture_isTextureView<i8> for () {
  fn isTextureView(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture13isTextureViewEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture13isTextureViewEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isFixedSamplePositions();
impl /*struct*/ QOpenGLTexture {
  pub fn isFixedSamplePositions<RetType, T: QOpenGLTexture_isFixedSamplePositions<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isFixedSamplePositions(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isFixedSamplePositions<RetType> {
  fn isFixedSamplePositions(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::isFixedSamplePositions();
impl<'a> /*trait*/ QOpenGLTexture_isFixedSamplePositions<i8> for () {
  fn isFixedSamplePositions(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture22isFixedSamplePositionsEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture22isFixedSamplePositionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QOpenGLTexture::faces();
impl /*struct*/ QOpenGLTexture {
  pub fn faces<RetType, T: QOpenGLTexture_faces<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.faces(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_faces<RetType> {
  fn faces(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::faces();
impl<'a> /*trait*/ QOpenGLTexture_faces<i32> for () {
  fn faces(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5facesEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture5facesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setLayers(int layers);
impl /*struct*/ QOpenGLTexture {
  pub fn setLayers<RetType, T: QOpenGLTexture_setLayers<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLayers(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setLayers<RetType> {
  fn setLayers(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setLayers(int layers);
impl<'a> /*trait*/ QOpenGLTexture_setLayers<()> for (i32) {
  fn setLayers(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture9setLayersEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture9setLayersEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setCompressedData(int dataSize, void * data, const QOpenGLPixelTransferOptions *const options);
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData<()> for (i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture17setCompressedDataEiPvPK27QOpenGLPixelTransferOptions(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  int QOpenGLTexture::width();
impl /*struct*/ QOpenGLTexture {
  pub fn width<RetType, T: QOpenGLTexture_width<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_width<RetType> {
  fn width(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::width();
impl<'a> /*trait*/ QOpenGLTexture_width<i32> for () {
  fn width(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture5widthEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QOpenGLTexture::layers();
impl /*struct*/ QOpenGLTexture {
  pub fn layers<RetType, T: QOpenGLTexture_layers<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.layers(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_layers<RetType> {
  fn layers(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::layers();
impl<'a> /*trait*/ QOpenGLTexture_layers<i32> for () {
  fn layers(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture6layersEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture6layersEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::borderColor(float * border);
impl<'a> /*trait*/ QOpenGLTexture_borderColor<()> for (&'a mut f32) {
  fn borderColor(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEPf()};
    let arg0 = self  as *mut c_float;
     unsafe {_ZNK14QOpenGLTexture11borderColorEPf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  float QOpenGLTexture::minimumLevelOfDetail();
impl /*struct*/ QOpenGLTexture {
  pub fn minimumLevelOfDetail<RetType, T: QOpenGLTexture_minimumLevelOfDetail<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_minimumLevelOfDetail<RetType> {
  fn minimumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  float QOpenGLTexture::minimumLevelOfDetail();
impl<'a> /*trait*/ QOpenGLTexture_minimumLevelOfDetail<f32> for () {
  fn minimumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture20minimumLevelOfDetailEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture20minimumLevelOfDetailEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(int r, int g, int b, int a);
impl /*struct*/ QOpenGLTexture {
  pub fn setBorderColor<RetType, T: QOpenGLTexture_setBorderColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBorderColor(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setBorderColor<RetType> {
  fn setBorderColor(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setBorderColor(int r, int g, int b, int a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor<()> for (i32, i32, i32, i32) {
  fn setBorderColor(self , rsthis: &mut QOpenGLTexture) -> () {
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

// proto:  void QOpenGLTexture::setMinimumLevelOfDetail(float value);
impl /*struct*/ QOpenGLTexture {
  pub fn setMinimumLevelOfDetail<RetType, T: QOpenGLTexture_setMinimumLevelOfDetail<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMinimumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMinimumLevelOfDetail<RetType> {
  fn setMinimumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMinimumLevelOfDetail(float value);
impl<'a> /*trait*/ QOpenGLTexture_setMinimumLevelOfDetail<()> for (f32) {
  fn setMinimumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture23setMinimumLevelOfDetailEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture23setMinimumLevelOfDetailEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setMipLevels(int levels);
impl /*struct*/ QOpenGLTexture {
  pub fn setMipLevels<RetType, T: QOpenGLTexture_setMipLevels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMipLevels(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipLevels<RetType> {
  fn setMipLevels(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMipLevels(int levels);
impl<'a> /*trait*/ QOpenGLTexture_setMipLevels<()> for (i32) {
  fn setMipLevels(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture12setMipLevelsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture12setMipLevelsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPair<int, int> QOpenGLTexture::mipLevelRange();
impl /*struct*/ QOpenGLTexture {
  pub fn mipLevelRange<RetType, T: QOpenGLTexture_mipLevelRange<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mipLevelRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipLevelRange<RetType> {
  fn mipLevelRange(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  QPair<int, int> QOpenGLTexture::mipLevelRange();
impl<'a> /*trait*/ QOpenGLTexture_mipLevelRange<()> for () {
  fn mipLevelRange(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture13mipLevelRangeEv()};
     unsafe {_ZNK14QOpenGLTexture13mipLevelRangeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setMipMaxLevel(int maxLevel);
impl /*struct*/ QOpenGLTexture {
  pub fn setMipMaxLevel<RetType, T: QOpenGLTexture_setMipMaxLevel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMipMaxLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipMaxLevel<RetType> {
  fn setMipMaxLevel(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMipMaxLevel(int maxLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipMaxLevel<()> for (i32) {
  fn setMipMaxLevel(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setMipMaxLevelEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture14setMipMaxLevelEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  float QOpenGLTexture::levelofDetailBias();
impl /*struct*/ QOpenGLTexture {
  pub fn levelofDetailBias<RetType, T: QOpenGLTexture_levelofDetailBias<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.levelofDetailBias(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_levelofDetailBias<RetType> {
  fn levelofDetailBias(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  float QOpenGLTexture::levelofDetailBias();
impl<'a> /*trait*/ QOpenGLTexture_levelofDetailBias<f32> for () {
  fn levelofDetailBias(self , rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture17levelofDetailBiasEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture17levelofDetailBiasEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto:  int QOpenGLTexture::maximumMipLevels();
impl /*struct*/ QOpenGLTexture {
  pub fn maximumMipLevels<RetType, T: QOpenGLTexture_maximumMipLevels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumMipLevels(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_maximumMipLevels<RetType> {
  fn maximumMipLevels(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::maximumMipLevels();
impl<'a> /*trait*/ QOpenGLTexture_maximumMipLevels<i32> for () {
  fn maximumMipLevels(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture16maximumMipLevelsEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture16maximumMipLevelsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isBound(uint unit);
impl /*struct*/ QOpenGLTexture {
  pub fn isBound<RetType, T: QOpenGLTexture_isBound<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isBound(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isBound<RetType> {
  fn isBound(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::isBound(uint unit);
impl<'a> /*trait*/ QOpenGLTexture_isBound<i8> for (u32) {
  fn isBound(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7isBoundEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN14QOpenGLTexture7isBoundEj(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(uint r, uint g, uint b, uint a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor<()> for (u32, u32, u32, u32) {
  fn setBorderColor(self , rsthis: &mut QOpenGLTexture) -> () {
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

// proto:  void QOpenGLTexture::setMaximumAnisotropy(float anisotropy);
impl /*struct*/ QOpenGLTexture {
  pub fn setMaximumAnisotropy<RetType, T: QOpenGLTexture_setMaximumAnisotropy<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMaximumAnisotropy(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMaximumAnisotropy<RetType> {
  fn setMaximumAnisotropy(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMaximumAnisotropy(float anisotropy);
impl<'a> /*trait*/ QOpenGLTexture_setMaximumAnisotropy<()> for (f32) {
  fn setMaximumAnisotropy(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture20setMaximumAnisotropyEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture20setMaximumAnisotropyEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setSamples(int samples);
impl /*struct*/ QOpenGLTexture {
  pub fn setSamples<RetType, T: QOpenGLTexture_setSamples<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSamples(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setSamples<RetType> {
  fn setSamples(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setSamples(int samples);
impl<'a> /*trait*/ QOpenGLTexture_setSamples<()> for (i32) {
  fn setSamples(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture10setSamplesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QOpenGLTexture10setSamplesEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QOpenGLTexture::mipLevels();
impl /*struct*/ QOpenGLTexture {
  pub fn mipLevels<RetType, T: QOpenGLTexture_mipLevels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mipLevels(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipLevels<RetType> {
  fn mipLevels(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::mipLevels();
impl<'a> /*trait*/ QOpenGLTexture_mipLevels<i32> for () {
  fn mipLevels(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9mipLevelsEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture9mipLevelsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setLevelofDetailBias(float bias);
impl /*struct*/ QOpenGLTexture {
  pub fn setLevelofDetailBias<RetType, T: QOpenGLTexture_setLevelofDetailBias<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLevelofDetailBias(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setLevelofDetailBias<RetType> {
  fn setLevelofDetailBias(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setLevelofDetailBias(float bias);
impl<'a> /*trait*/ QOpenGLTexture_setLevelofDetailBias<()> for (f32) {
  fn setLevelofDetailBias(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture20setLevelofDetailBiasEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN14QOpenGLTexture20setLevelofDetailBiasEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QOpenGLTexture::GLuint QOpenGLTexture::textureId();
impl /*struct*/ QOpenGLTexture {
  pub fn textureId<RetType, T: QOpenGLTexture_textureId<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.textureId(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_textureId<RetType> {
  fn textureId(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  QOpenGLTexture::GLuint QOpenGLTexture::textureId();
impl<'a> /*trait*/ QOpenGLTexture_textureId<()> for () {
  fn textureId(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9textureIdEv()};
     unsafe {_ZNK14QOpenGLTexture9textureIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel);
impl /*struct*/ QOpenGLTexture {
  pub fn setMipLevelRange<RetType, T: QOpenGLTexture_setMipLevelRange<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMipLevelRange(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setMipLevelRange<RetType> {
  fn setMipLevelRange(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel);
impl<'a> /*trait*/ QOpenGLTexture_setMipLevelRange<()> for (i32, i32) {
  fn setMipLevelRange(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture16setMipLevelRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN14QOpenGLTexture16setMipLevelRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::allocateStorage();
impl /*struct*/ QOpenGLTexture {
  pub fn allocateStorage<RetType, T: QOpenGLTexture_allocateStorage<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.allocateStorage(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_allocateStorage<RetType> {
  fn allocateStorage(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::allocateStorage();
impl<'a> /*trait*/ QOpenGLTexture_allocateStorage<()> for () {
  fn allocateStorage(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15allocateStorageEv()};
     unsafe {_ZN14QOpenGLTexture15allocateStorageEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::FreeQOpenGLTexture();
impl /*struct*/ QOpenGLTexture {
  pub fn FreeQOpenGLTexture<RetType, T: QOpenGLTexture_FreeQOpenGLTexture<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLTexture(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_FreeQOpenGLTexture<RetType> {
  fn FreeQOpenGLTexture(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::FreeQOpenGLTexture();
impl<'a> /*trait*/ QOpenGLTexture_FreeQOpenGLTexture<()> for () {
  fn FreeQOpenGLTexture(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTextureD0Ev()};
     unsafe {_ZN14QOpenGLTextureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QOpenGLTexture::mipMaxLevel();
impl /*struct*/ QOpenGLTexture {
  pub fn mipMaxLevel<RetType, T: QOpenGLTexture_mipMaxLevel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mipMaxLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipMaxLevel<RetType> {
  fn mipMaxLevel(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::mipMaxLevel();
impl<'a> /*trait*/ QOpenGLTexture_mipMaxLevel<i32> for () {
  fn mipMaxLevel(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11mipMaxLevelEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture11mipMaxLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(QColor color);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor<()> for (QColor) {
  fn setBorderColor(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture14setBorderColorE6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLTexture14setBorderColorE6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::destroy();
impl /*struct*/ QOpenGLTexture {
  pub fn destroy<RetType, T: QOpenGLTexture_destroy<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_destroy<RetType> {
  fn destroy(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::destroy();
impl<'a> /*trait*/ QOpenGLTexture_destroy<()> for () {
  fn destroy(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7destroyEv()};
     unsafe {_ZN14QOpenGLTexture7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::generateMipMaps();
impl<'a> /*trait*/ QOpenGLTexture_generateMipMaps<()> for () {
  fn generateMipMaps(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture15generateMipMapsEv()};
     unsafe {_ZN14QOpenGLTexture15generateMipMapsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLTexture::release();
impl /*struct*/ QOpenGLTexture {
  pub fn release<RetType, T: QOpenGLTexture_release<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_release<RetType> {
  fn release(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::release();
impl<'a> /*trait*/ QOpenGLTexture_release<()> for () {
  fn release(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7releaseEv()};
     unsafe {_ZN14QOpenGLTexture7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  float QOpenGLTexture::maximumAnisotropy();
impl /*struct*/ QOpenGLTexture {
  pub fn maximumAnisotropy<RetType, T: QOpenGLTexture_maximumAnisotropy<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumAnisotropy(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_maximumAnisotropy<RetType> {
  fn maximumAnisotropy(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  float QOpenGLTexture::maximumAnisotropy();
impl<'a> /*trait*/ QOpenGLTexture_maximumAnisotropy<f32> for () {
  fn maximumAnisotropy(self , rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture17maximumAnisotropyEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture17maximumAnisotropyEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto:  float QOpenGLTexture::maximumLevelOfDetail();
impl /*struct*/ QOpenGLTexture {
  pub fn maximumLevelOfDetail<RetType, T: QOpenGLTexture_maximumLevelOfDetail<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumLevelOfDetail(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_maximumLevelOfDetail<RetType> {
  fn maximumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  float QOpenGLTexture::maximumLevelOfDetail();
impl<'a> /*trait*/ QOpenGLTexture_maximumLevelOfDetail<f32> for () {
  fn maximumLevelOfDetail(self , rsthis: &mut QOpenGLTexture) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture20maximumLevelOfDetailEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture20maximumLevelOfDetailEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setSize(int width, int height, int depth);
impl /*struct*/ QOpenGLTexture {
  pub fn setSize<RetType, T: QOpenGLTexture_setSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSize(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_setSize<RetType> {
  fn setSize(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  void QOpenGLTexture::setSize(int width, int height, int depth);
impl<'a> /*trait*/ QOpenGLTexture_setSize<()> for (i32, i32, i32) {
  fn setSize(self , rsthis: &mut QOpenGLTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLTexture7setSizeEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN14QOpenGLTexture7setSizeEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isCreated();
impl /*struct*/ QOpenGLTexture {
  pub fn isCreated<RetType, T: QOpenGLTexture_isCreated<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_isCreated<RetType> {
  fn isCreated(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  bool QOpenGLTexture::isCreated();
impl<'a> /*trait*/ QOpenGLTexture_isCreated<i8> for () {
  fn isCreated(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture9isCreatedEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QOpenGLTexture::isBound();
impl<'a> /*trait*/ QOpenGLTexture_isBound<i8> for () {
  fn isBound(self , rsthis: &mut QOpenGLTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture7isBoundEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture7isBoundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLTexture::setBorderColor(float r, float g, float b, float a);
impl<'a> /*trait*/ QOpenGLTexture_setBorderColor<()> for (f32, f32, f32, f32) {
  fn setBorderColor(self , rsthis: &mut QOpenGLTexture) -> () {
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

// proto:  int QOpenGLTexture::samples();
impl /*struct*/ QOpenGLTexture {
  pub fn samples<RetType, T: QOpenGLTexture_samples<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.samples(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_samples<RetType> {
  fn samples(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::samples();
impl<'a> /*trait*/ QOpenGLTexture_samples<i32> for () {
  fn samples(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture7samplesEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture7samplesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QOpenGLTexture::mipBaseLevel();
impl /*struct*/ QOpenGLTexture {
  pub fn mipBaseLevel<RetType, T: QOpenGLTexture_mipBaseLevel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mipBaseLevel(self);
    // return 1;
  }
}

pub trait QOpenGLTexture_mipBaseLevel<RetType> {
  fn mipBaseLevel(self , rsthis: &mut QOpenGLTexture) -> RetType;
}

// proto:  int QOpenGLTexture::mipBaseLevel();
impl<'a> /*trait*/ QOpenGLTexture_mipBaseLevel<i32> for () {
  fn mipBaseLevel(self , rsthis: &mut QOpenGLTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture12mipBaseLevelEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture12mipBaseLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QColor QOpenGLTexture::borderColor();
impl<'a> /*trait*/ QOpenGLTexture_borderColor<QColor> for () {
  fn borderColor(self , rsthis: &mut QOpenGLTexture) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLTexture11borderColorEv()};
    let mut ret = unsafe {_ZNK14QOpenGLTexture11borderColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
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
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData<()> for (i32, i32, i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> () {
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
impl<'a> /*trait*/ QOpenGLTexture_setCompressedData<()> for (i32, i32, &'a mut u8, &'a  QOpenGLPixelTransferOptions) {
  fn setCompressedData(self , rsthis: &mut QOpenGLTexture) -> () {
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

