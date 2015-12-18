// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLShader::NewQOpenGLShader(const QOpenGLShader & );
  fn _ZN13QOpenGLShaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QOpenGLShader::isCompiled();
  fn _ZNK13QOpenGLShader10isCompiledEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QOpenGLShader::metaObject();
  fn _ZNK13QOpenGLShader10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QString QOpenGLShader::log();
  fn _ZNK13QOpenGLShader3logEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
  fn _ZN13QOpenGLShader17compileSourceFileERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QByteArray QOpenGLShader::sourceCode();
  fn _ZNK13QOpenGLShader10sourceCodeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLShader::FreeQOpenGLShader();
  fn _ZN13QOpenGLShaderD0Ev(qthis: *mut c_void) ;
  // proto:  bool QOpenGLShader::compileSourceCode(const QByteArray & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QOpenGLShader::GLuint QOpenGLShader::shaderId();
  fn _ZNK13QOpenGLShader8shaderIdEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLShader::compileSourceCode(const char * source);
  fn _ZN13QOpenGLShader17compileSourceCodeEPKc(qthis: *mut c_void, arg0: *const c_char) -> int8_t;
}

// body block begin
// class sizeof(QOpenGLShader)=1
pub struct QOpenGLShader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLShader {
  pub fn NewQOpenGLShader<T: QOpenGLShader_NewQOpenGLShader>(value: T) -> QOpenGLShader {
    let rsthis = value.NewQOpenGLShader();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLShader_NewQOpenGLShader {
  fn NewQOpenGLShader(self) -> QOpenGLShader;
}

// proto: void QOpenGLShader::NewQOpenGLShader(const QOpenGLShader & );
impl<'a> /*trait*/ QOpenGLShader_NewQOpenGLShader for (&'a  QOpenGLShader) {
  fn NewQOpenGLShader(self) -> QOpenGLShader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QOpenGLShaderC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLShader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn isCompiled<RetType, T: QOpenGLShader_isCompiled<RetType>>(&mut self, value: T) -> RetType {
    return value.isCompiled(self);
    // return 1;
  }
}

pub trait QOpenGLShader_isCompiled<RetType> {
  fn isCompiled(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  bool QOpenGLShader::isCompiled();
impl<'a> /*trait*/ QOpenGLShader_isCompiled<i8> for () {
  fn isCompiled(self, rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10isCompiledEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader10isCompiledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn metaObject<RetType, T: QOpenGLShader_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLShader_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  const QMetaObject * QOpenGLShader::metaObject();
impl<'a> /*trait*/ QOpenGLShader_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10metaObjectEv()};
     unsafe {_ZNK13QOpenGLShader10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn log<RetType, T: QOpenGLShader_log<RetType>>(&mut self, value: T) -> RetType {
    return value.log(self);
    // return 1;
  }
}

pub trait QOpenGLShader_log<RetType> {
  fn log(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  QString QOpenGLShader::log();
impl<'a> /*trait*/ QOpenGLShader_log<QString> for () {
  fn log(self, rsthis: &mut QOpenGLShader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader3logEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader3logEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn compileSourceCode<RetType, T: QOpenGLShader_compileSourceCode<RetType>>(&mut self, value: T) -> RetType {
    return value.compileSourceCode(self);
    // return 1;
  }
}

pub trait QOpenGLShader_compileSourceCode<RetType> {
  fn compileSourceCode(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a  QString) {
  fn compileSourceCode(self, rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn compileSourceFile<RetType, T: QOpenGLShader_compileSourceFile<RetType>>(&mut self, value: T) -> RetType {
    return value.compileSourceFile(self);
    // return 1;
  }
}

pub trait QOpenGLShader_compileSourceFile<RetType> {
  fn compileSourceFile(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
impl<'a> /*trait*/ QOpenGLShader_compileSourceFile<i8> for (&'a  QString) {
  fn compileSourceFile(self, rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceFileERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn sourceCode<RetType, T: QOpenGLShader_sourceCode<RetType>>(&mut self, value: T) -> RetType {
    return value.sourceCode(self);
    // return 1;
  }
}

pub trait QOpenGLShader_sourceCode<RetType> {
  fn sourceCode(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  QByteArray QOpenGLShader::sourceCode();
impl<'a> /*trait*/ QOpenGLShader_sourceCode<QByteArray> for () {
  fn sourceCode(self, rsthis: &mut QOpenGLShader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10sourceCodeEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader10sourceCodeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn FreeQOpenGLShader<RetType, T: QOpenGLShader_FreeQOpenGLShader<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQOpenGLShader(self);
    // return 1;
  }
}

pub trait QOpenGLShader_FreeQOpenGLShader<RetType> {
  fn FreeQOpenGLShader(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  void QOpenGLShader::FreeQOpenGLShader();
impl<'a> /*trait*/ QOpenGLShader_FreeQOpenGLShader<()> for () {
  fn FreeQOpenGLShader(self, rsthis: &mut QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderD0Ev()};
     unsafe {_ZN13QOpenGLShaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QOpenGLShader::compileSourceCode(const QByteArray & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a  QByteArray) {
  fn compileSourceCode(self, rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn shaderId<RetType, T: QOpenGLShader_shaderId<RetType>>(&mut self, value: T) -> RetType {
    return value.shaderId(self);
    // return 1;
  }
}

pub trait QOpenGLShader_shaderId<RetType> {
  fn shaderId(self, rsthis: &mut QOpenGLShader) -> RetType;
}

// proto:  QOpenGLShader::GLuint QOpenGLShader::shaderId();
impl<'a> /*trait*/ QOpenGLShader_shaderId<()> for () {
  fn shaderId(self, rsthis: &mut QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader8shaderIdEv()};
     unsafe {_ZNK13QOpenGLShader8shaderIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QOpenGLShader::compileSourceCode(const char * source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a  String) {
  fn compileSourceCode(self, rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

