// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtGui/qopenglshaderprogram.h
// dst-file: /src/gui/qopenglshaderprogram.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qopenglcontext::QOpenGLContext; // 773
use super::qvector3d::QVector3D; // 773
use super::super::core::qpoint::QPoint; // 771
use super::qtransform::QTransform; // 773
use super::qvector2d::QVector2D; // 773
use super::qcolor::QColor; // 773
use super::super::core::qsize::QSize; // 771
// use super::qopenglshaderprogram::QOpenGLShader; // 773
use super::qmatrix4x4::QMatrix4x4; // 773
use super::qvector4d::QVector4D; // 773
use super::super::core::qpoint::QPointF; // 771
use super::super::core::qsize::QSizeF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLShader_Class_Size() -> c_int;
  // proto:  void QOpenGLShader::QOpenGLShader(const QOpenGLShader & );
  fn dector_ZN13QOpenGLShaderC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QOpenGLShaderC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QOpenGLShader::isCompiled();
  fn _ZNK13QOpenGLShader10isCompiledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QOpenGLShader::metaObject();
  fn _ZNK13QOpenGLShader10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QOpenGLShader::log();
  fn _ZNK13QOpenGLShader3logEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
  fn _ZN13QOpenGLShader17compileSourceFileERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QByteArray QOpenGLShader::sourceCode();
  fn _ZNK13QOpenGLShader10sourceCodeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLShader::~QOpenGLShader();
  fn _ZN13QOpenGLShaderD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLShader::compileSourceCode(const QByteArray & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  GLuint QOpenGLShader::shaderId();
  fn _ZNK13QOpenGLShader8shaderIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  bool QOpenGLShader::compileSourceCode(const char * source);
  fn _ZN13QOpenGLShader17compileSourceCodeEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_char;
  fn QOpenGLShaderProgram_Class_Size() -> c_int;
  // proto:  bool QOpenGLShaderProgram::isLinked();
  fn _ZNK20QOpenGLShaderProgram8isLinkedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPoint & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK6QPoint(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcffff(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector3D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector3Di(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QTransform & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK10QTransform(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(int location, GLenum type, int offset, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram18setAttributeBufferEijiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_uint, arg2: c_int, arg3: c_int, arg4: c_int);
  // proto: static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms(QOpenGLContext * context);
  fn _ZN20QOpenGLShaderProgram23hasOpenGLShaderProgramsEP14QOpenGLContext(arg0: *mut c_void) -> c_char;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [2][2] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcPA2_Kf(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut *mut c_float);
  // proto:  void QOpenGLShaderProgram::setPatchVertexCount(int count);
  fn _ZN20QOpenGLShaderProgram19setPatchVertexCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const GLfloat * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPKfii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_float, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector3D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector3Di(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [3][3] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcPA3_Kf(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut *mut c_float);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector2D(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QByteArray & name, int location);
  fn _ZN20QOpenGLShaderProgram21bindAttributeLocationERK10QByteArrayi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLfloat * values, int count, int tupleSize);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKfii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_float, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiff(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float, arg2: c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector2D(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QColor & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK6QColor(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEifff(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  bool QOpenGLShaderProgram::bind();
  fn _ZN20QOpenGLShaderProgram4bindEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEif(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float);
  // proto:  void QOpenGLShaderProgram::enableAttributeArray(int location);
  fn _ZN20QOpenGLShaderProgram20enableAttributeArrayEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLuint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEij(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_uint);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSize & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK5QSize(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QOpenGLShaderProgram::addShader(QOpenGLShader * shader);
  fn _ZN20QOpenGLShaderProgram9addShaderEP13QOpenGLShader(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QOpenGLShaderProgram::attributeLocation(const QString & name);
  fn _ZNK20QOpenGLShaderProgram17attributeLocationERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, GLenum type, const void * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEijPKvii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_uint, arg2: *mut c_void, arg3: c_int, arg4: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPoint & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QMatrix4x4 * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK10QMatrix4x4i(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector2D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector2Di(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiff(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float, arg2: c_float);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QMatrix4x4 * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK10QMatrix4x4i(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QColor & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcffff(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float);
  // proto:  GLuint QOpenGLShaderProgram::programId();
  fn _ZNK20QOpenGLShaderProgram9programIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLuint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKji(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_uint, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::disableAttributeArray(int location);
  fn _ZN20QOpenGLShaderProgram21disableAttributeArrayEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_int, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLuint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKji(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_uint, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const GLfloat * values, int columns, int rows);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcPKfii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_float, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector4D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector4Di(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector2D(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEifff(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiffff(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float);
  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const char * name, int location);
  fn _ZN20QOpenGLShaderProgram21bindAttributeLocationEPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int);
  // proto:  int QOpenGLShaderProgram::attributeLocation(const char * name);
  fn _ZNK20QOpenGLShaderProgram17attributeLocationEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  int QOpenGLShaderProgram::uniformLocation(const QString & name);
  fn _ZNK20QOpenGLShaderProgram15uniformLocationERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPointF & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector4D(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector4D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector4Di(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  int QOpenGLShaderProgram::uniformLocation(const char * name);
  fn _ZNK20QOpenGLShaderProgram15uniformLocationEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  void QOpenGLShaderProgram::~QOpenGLShaderProgram();
  fn _ZN20QOpenGLShaderProgramD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVector<float> QOpenGLShaderProgram::defaultInnerTessellationLevels();
  fn _ZNK20QOpenGLShaderProgram30defaultInnerTessellationLevelsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLShaderProgram::link();
  fn _ZN20QOpenGLShaderProgram4linkEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector4D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector4Di(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  QList<QOpenGLShader *> QOpenGLShaderProgram::shaders();
  fn _ZNK20QOpenGLShaderProgram7shadersEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcf(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QVector<float> QOpenGLShaderProgram::defaultOuterTessellationLevels();
  fn _ZNK20QOpenGLShaderProgram30defaultOuterTessellationLevelsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QColor & color);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK6QColor(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector2D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector2Di(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [4][4] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcPA4_Kf(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut *mut c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcff(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float, arg2: c_float);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_int, arg2: c_int);
  // proto:  QString QOpenGLShaderProgram::log();
  fn _ZNK20QOpenGLShaderProgram3logEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSize & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  int QOpenGLShaderProgram::patchVertexCount();
  fn _ZNK20QOpenGLShaderProgram16patchVertexCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [2][2] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiPA2_Kf(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut *mut c_float);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QMatrix4x4 & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector4D(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QOpenGLShaderProgram::create();
  fn _ZN20QOpenGLShaderProgram6createEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSizeF & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::removeShader(QOpenGLShader * shader);
  fn _ZN20QOpenGLShaderProgram12removeShaderEP13QOpenGLShader(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector4D(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcff(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float, arg2: c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(const char * name, GLenum type, int offset, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram18setAttributeBufferEPKcjiii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_uint, arg2: c_int, arg3: c_int, arg4: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const GLfloat * values, int columns, int rows);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiPKfii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_float, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLShaderProgram::disableAttributeArray(const char * name);
  fn _ZN20QOpenGLShaderProgram21disableAttributeArrayEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPointF & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK7QPointF(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [4][4] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiPA4_Kf(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut *mut c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEif(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float);
  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QString & name, int location);
  fn _ZN20QOpenGLShaderProgram21bindAttributeLocationERK7QStringi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector3D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector3Di(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QColor & color);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QTransform & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector2D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector2Di(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  int QOpenGLShaderProgram::attributeLocation(const QByteArray & name);
  fn _ZNK20QOpenGLShaderProgram17attributeLocationERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcfff(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector4D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector4Di(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcfff(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [3][3] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiPA3_Kf(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut *mut c_float);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, GLenum type, const void * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcjPKvii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_uint, arg2: *mut c_void, arg3: c_int, arg4: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLuint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcj(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_uint);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int);
  // proto:  void QOpenGLShaderProgram::QOpenGLShaderProgram(QObject * parent);
  fn dector_ZN20QOpenGLShaderProgramC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN20QOpenGLShaderProgramC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcf(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_float);
  // proto:  void QOpenGLShaderProgram::enableAttributeArray(const char * name);
  fn _ZN20QOpenGLShaderProgram20enableAttributeArrayEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QMatrix4x4 & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector4D(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const GLfloat * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPKfii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_float, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSizeF & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::QOpenGLShaderProgram(const QOpenGLShaderProgram & );
  fn dector_ZN20QOpenGLShaderProgramC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN20QOpenGLShaderProgramC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiffff(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float);
  // proto:  void QOpenGLShaderProgram::removeAllShaders();
  fn _ZN20QOpenGLShaderProgram16removeAllShadersEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QOpenGLShaderProgram::maxGeometryOutputVertices();
  fn _ZNK20QOpenGLShaderProgram25maxGeometryOutputVerticesEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector3D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector3Di(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::release();
  fn _ZN20QOpenGLShaderProgram7releaseEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QOpenGLShaderProgram::metaObject();
  fn _ZNK20QOpenGLShaderProgram10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QOpenGLShaderProgram::uniformLocation(const QByteArray & name);
  fn _ZNK20QOpenGLShaderProgram15uniformLocationERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector2D(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void);
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector2D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector2Di(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: c_int);
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLfloat * values, int count, int tupleSize);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKfii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_float, arg2: c_int, arg3: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLShader)=1
#[derive(Default)]
pub struct QOpenGLShader {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLShaderProgram)=1
#[derive(Default)]
pub struct QOpenGLShaderProgram {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLShader {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLShader {
    return QOpenGLShader{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLShader {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLShader {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLShader::QOpenGLShader(const QOpenGLShader & );
impl /*struct*/ QOpenGLShader {
  pub fn New<T: QOpenGLShader_New>(value: T) -> QOpenGLShader {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLShader_New {
  fn New(self) -> QOpenGLShader;
}

  // proto:  void QOpenGLShader::QOpenGLShader(const QOpenGLShader & );
impl<'a> /*trait*/ QOpenGLShader_New for (&'a QOpenGLShader) {
  fn New(self) -> QOpenGLShader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLShader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QOpenGLShaderC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QOpenGLShaderC1ERKS_(arg0)} as u64;
    let rsthis = QOpenGLShader{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::isCompiled();
impl /*struct*/ QOpenGLShader {
  pub fn isCompiled<RetType, T: QOpenGLShader_isCompiled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCompiled(self);
    // return 1;
  }
}

pub trait QOpenGLShader_isCompiled<RetType> {
  fn isCompiled(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  bool QOpenGLShader::isCompiled();
impl<'a> /*trait*/ QOpenGLShader_isCompiled<i8> for () {
  fn isCompiled(self , rsthis: & QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10isCompiledEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader10isCompiledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLShader::metaObject();
impl /*struct*/ QOpenGLShader {
  pub fn metaObject<RetType, T: QOpenGLShader_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLShader_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLShader::metaObject();
impl<'a> /*trait*/ QOpenGLShader_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10metaObjectEv()};
     unsafe {_ZNK13QOpenGLShader10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QOpenGLShader::log();
impl /*struct*/ QOpenGLShader {
  pub fn log<RetType, T: QOpenGLShader_log<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.log(self);
    // return 1;
  }
}

pub trait QOpenGLShader_log<RetType> {
  fn log(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  QString QOpenGLShader::log();
impl<'a> /*trait*/ QOpenGLShader_log<QString> for () {
  fn log(self , rsthis: & QOpenGLShader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader3logEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader3logEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
impl /*struct*/ QOpenGLShader {
  pub fn compileSourceCode<RetType, T: QOpenGLShader_compileSourceCode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compileSourceCode(self);
    // return 1;
  }
}

pub trait QOpenGLShader_compileSourceCode<RetType> {
  fn compileSourceCode(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a QString) {
  fn compileSourceCode(self , rsthis: & QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
impl /*struct*/ QOpenGLShader {
  pub fn compileSourceFile<RetType, T: QOpenGLShader_compileSourceFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compileSourceFile(self);
    // return 1;
  }
}

pub trait QOpenGLShader_compileSourceFile<RetType> {
  fn compileSourceFile(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
impl<'a> /*trait*/ QOpenGLShader_compileSourceFile<i8> for (&'a QString) {
  fn compileSourceFile(self , rsthis: & QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceFileERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QOpenGLShader::sourceCode();
impl /*struct*/ QOpenGLShader {
  pub fn sourceCode<RetType, T: QOpenGLShader_sourceCode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sourceCode(self);
    // return 1;
  }
}

pub trait QOpenGLShader_sourceCode<RetType> {
  fn sourceCode(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  QByteArray QOpenGLShader::sourceCode();
impl<'a> /*trait*/ QOpenGLShader_sourceCode<QByteArray> for () {
  fn sourceCode(self , rsthis: & QOpenGLShader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10sourceCodeEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader10sourceCodeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLShader::~QOpenGLShader();
impl /*struct*/ QOpenGLShader {
  pub fn Free<RetType, T: QOpenGLShader_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLShader_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  void QOpenGLShader::~QOpenGLShader();
impl<'a> /*trait*/ QOpenGLShader_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderD0Ev()};
     unsafe {_ZN13QOpenGLShaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceCode(const QByteArray & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a QByteArray) {
  fn compileSourceCode(self , rsthis: & QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLShader::shaderId();
impl /*struct*/ QOpenGLShader {
  pub fn shaderId<RetType, T: QOpenGLShader_shaderId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shaderId(self);
    // return 1;
  }
}

pub trait QOpenGLShader_shaderId<RetType> {
  fn shaderId(self , rsthis: & QOpenGLShader) -> RetType;
}

  // proto:  GLuint QOpenGLShader::shaderId();
impl<'a> /*trait*/ QOpenGLShader_shaderId<u32> for () {
  fn shaderId(self , rsthis: & QOpenGLShader) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader8shaderIdEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader8shaderIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceCode(const char * source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a  String) {
  fn compileSourceCode(self , rsthis: & QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLShaderProgram {
    return QOpenGLShaderProgram{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLShaderProgram {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLShaderProgram {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QOpenGLShaderProgram::isLinked();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn isLinked<RetType, T: QOpenGLShaderProgram_isLinked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLinked(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_isLinked<RetType> {
  fn isLinked(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  bool QOpenGLShaderProgram::isLinked();
impl<'a> /*trait*/ QOpenGLShaderProgram_isLinked<i8> for () {
  fn isLinked(self , rsthis: & QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram8isLinkedEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram8isLinkedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector3D & value);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn setUniformValue<RetType, T: QOpenGLShaderProgram_setUniformValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUniformValue(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setUniformValue<RetType> {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QVector3D) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector3D()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPoint & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QPoint) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QPoint()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, f32, f32, f32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcffff()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector3D & value);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn setAttributeValue<RetType, T: QOpenGLShaderProgram_setAttributeValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAttributeValue(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setAttributeValue<RetType> {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, &'a QVector3D) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector3D()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector3D * values, int count);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn setUniformValueArray<RetType, T: QOpenGLShaderProgram_setUniformValueArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUniformValueArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setUniformValueArray<RetType> {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector3D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a QVector3D, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector3Di()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector3Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QTransform & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QTransform) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QTransform()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(int location, GLenum type, int offset, int tupleSize, int stride);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn setAttributeBuffer<RetType, T: QOpenGLShaderProgram_setAttributeBuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAttributeBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setAttributeBuffer<RetType> {
  fn setAttributeBuffer(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(int location, GLenum type, int offset, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeBuffer<()> for (i32, u32, i32, i32, i32) {
  fn setAttributeBuffer(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram18setAttributeBufferEijiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram18setAttributeBufferEijiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto: static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms(QOpenGLContext * context);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn hasOpenGLShaderPrograms_s<RetType, T: QOpenGLShaderProgram_hasOpenGLShaderPrograms_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasOpenGLShaderPrograms_s();
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_hasOpenGLShaderPrograms_s<RetType> {
  fn hasOpenGLShaderPrograms_s(self ) -> RetType;
}

  // proto: static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLShaderProgram_hasOpenGLShaderPrograms_s<i8> for (&'a QOpenGLContext) {
  fn hasOpenGLShaderPrograms_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram23hasOpenGLShaderProgramsEP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram23hasOpenGLShaderProgramsEP14QOpenGLContext(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [2][2] value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a  Vec<&'a  f32>) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcPA2_Kf()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut *mut c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcPA2_Kf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setPatchVertexCount(int count);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn setPatchVertexCount<RetType, T: QOpenGLShaderProgram_setPatchVertexCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPatchVertexCount(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setPatchVertexCount<RetType> {
  fn setPatchVertexCount(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::setPatchVertexCount(int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setPatchVertexCount<()> for (i32) {
  fn setPatchVertexCount(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram19setPatchVertexCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram19setPatchVertexCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const GLfloat * values, int tupleSize, int stride);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn setAttributeArray<RetType, T: QOpenGLShaderProgram_setAttributeArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAttributeArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setAttributeArray<RetType> {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const GLfloat * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (&'a  String, &'a  Vec<f32>, i32, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPKfii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector3D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a QVector3D, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector3Di()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector3Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector2D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QVector2D) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector2D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QByteArray & name, int location);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn bindAttributeLocation<RetType, T: QOpenGLShaderProgram_bindAttributeLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bindAttributeLocation(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_bindAttributeLocation<RetType> {
  fn bindAttributeLocation(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QByteArray & name, int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_bindAttributeLocation<()> for (&'a QByteArray, i32) {
  fn bindAttributeLocation(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21bindAttributeLocationERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21bindAttributeLocationERK10QByteArrayi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLfloat * values, int count, int tupleSize);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a  Vec<f32>, i32, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKfii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, f32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector2D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, &'a QVector2D) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector2D()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QColor & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, &'a QColor) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, f32, f32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEifff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEifff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShaderProgram::bind();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn bind<RetType, T: QOpenGLShaderProgram_bind<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_bind<RetType> {
  fn bind(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  bool QOpenGLShaderProgram::bind();
impl<'a> /*trait*/ QOpenGLShaderProgram_bind<i8> for () {
  fn bind(self , rsthis: & QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram4bindEv()};
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram4bindEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEif(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::enableAttributeArray(int location);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn enableAttributeArray<RetType, T: QOpenGLShaderProgram_enableAttributeArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.enableAttributeArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_enableAttributeArray<RetType> {
  fn enableAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::enableAttributeArray(int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_enableAttributeArray<()> for (i32) {
  fn enableAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20enableAttributeArrayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20enableAttributeArrayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, i32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLuint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, u32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEij(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSize & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QSize) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK5QSize()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK5QSize(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShaderProgram::addShader(QOpenGLShader * shader);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn addShader<RetType, T: QOpenGLShaderProgram_addShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addShader(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_addShader<RetType> {
  fn addShader(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  bool QOpenGLShaderProgram::addShader(QOpenGLShader * shader);
impl<'a> /*trait*/ QOpenGLShaderProgram_addShader<i8> for (&'a QOpenGLShader) {
  fn addShader(self , rsthis: & QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram9addShaderEP13QOpenGLShader()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram9addShaderEP13QOpenGLShader(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::attributeLocation(const QString & name);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn attributeLocation<RetType, T: QOpenGLShaderProgram_attributeLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.attributeLocation(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_attributeLocation<RetType> {
  fn attributeLocation(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  int QOpenGLShaderProgram::attributeLocation(const QString & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_attributeLocation<i32> for (&'a QString) {
  fn attributeLocation(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram17attributeLocationERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram17attributeLocationERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, GLenum type, const void * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (i32, u32, *mut c_void, i32, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEijPKvii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEijPKvii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPoint & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QPoint) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QPoint()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QMatrix4x4 * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a QMatrix4x4, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK10QMatrix4x4i()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK10QMatrix4x4i(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector2D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (i32, &'a QVector2D, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector2Di()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector2Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, f32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QMatrix4x4 * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a QMatrix4x4, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK10QMatrix4x4i()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK10QMatrix4x4i(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QColor & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, &'a QColor) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK6QColor()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, f32, f32, f32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcffff()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLShaderProgram::programId();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn programId<RetType, T: QOpenGLShaderProgram_programId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.programId(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_programId<RetType> {
  fn programId(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  GLuint QOpenGLShaderProgram::programId();
impl<'a> /*trait*/ QOpenGLShaderProgram_programId<u32> for () {
  fn programId(self , rsthis: & QOpenGLShaderProgram) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram9programIdEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram9programIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLuint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a  Vec<u32>, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKji()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKji(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::disableAttributeArray(int location);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn disableAttributeArray<RetType, T: QOpenGLShaderProgram_disableAttributeArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.disableAttributeArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_disableAttributeArray<RetType> {
  fn disableAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::disableAttributeArray(int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_disableAttributeArray<()> for (i32) {
  fn disableAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21disableAttributeArrayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21disableAttributeArrayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a  Vec<i32>, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLuint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a  Vec<u32>, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKji()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKji(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const GLfloat * values, int columns, int rows);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, &'a  Vec<f32>, i32, i32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcPKfii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector4D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (&'a  String, &'a QVector4D, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector4Di()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector4Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector2D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, &'a QVector2D) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector2D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, f32, f32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEifff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEifff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, f32, f32, f32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiffff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const char * name, int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_bindAttributeLocation<()> for (&'a  String, i32) {
  fn bindAttributeLocation(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21bindAttributeLocationEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21bindAttributeLocationEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::attributeLocation(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_attributeLocation<i32> for (&'a  String) {
  fn attributeLocation(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram17attributeLocationEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram17attributeLocationEPKc(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::uniformLocation(const QString & name);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn uniformLocation<RetType, T: QOpenGLShaderProgram_uniformLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uniformLocation(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_uniformLocation<RetType> {
  fn uniformLocation(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  int QOpenGLShaderProgram::uniformLocation(const QString & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_uniformLocation<i32> for (&'a QString) {
  fn uniformLocation(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram15uniformLocationERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram15uniformLocationERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPointF & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QPointF) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK7QPointF()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, &'a QVector4D) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector4D()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector4D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a QVector4D, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector4Di()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector4Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::uniformLocation(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_uniformLocation<i32> for (&'a  String) {
  fn uniformLocation(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram15uniformLocationEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram15uniformLocationEPKc(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::~QOpenGLShaderProgram();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn Free<RetType, T: QOpenGLShaderProgram_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::~QOpenGLShaderProgram();
impl<'a> /*trait*/ QOpenGLShaderProgram_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgramD0Ev()};
     unsafe {_ZN20QOpenGLShaderProgramD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVector<float> QOpenGLShaderProgram::defaultInnerTessellationLevels();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn defaultInnerTessellationLevels<RetType, T: QOpenGLShaderProgram_defaultInnerTessellationLevels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultInnerTessellationLevels(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_defaultInnerTessellationLevels<RetType> {
  fn defaultInnerTessellationLevels(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  QVector<float> QOpenGLShaderProgram::defaultInnerTessellationLevels();
impl<'a> /*trait*/ QOpenGLShaderProgram_defaultInnerTessellationLevels<()> for () {
  fn defaultInnerTessellationLevels(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram30defaultInnerTessellationLevelsEv()};
     unsafe {_ZNK20QOpenGLShaderProgram30defaultInnerTessellationLevelsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShaderProgram::link();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn link<RetType, T: QOpenGLShaderProgram_link<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.link(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_link<RetType> {
  fn link(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  bool QOpenGLShaderProgram::link();
impl<'a> /*trait*/ QOpenGLShaderProgram_link<i8> for () {
  fn link(self , rsthis: & QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram4linkEv()};
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram4linkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector4D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a QVector4D, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector4Di()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector4Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QList<QOpenGLShader *> QOpenGLShaderProgram::shaders();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn shaders<RetType, T: QOpenGLShaderProgram_shaders<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shaders(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_shaders<RetType> {
  fn shaders(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  QList<QOpenGLShader *> QOpenGLShaderProgram::shaders();
impl<'a> /*trait*/ QOpenGLShaderProgram_shaders<()> for () {
  fn shaders(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram7shadersEv()};
     unsafe {_ZNK20QOpenGLShaderProgram7shadersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcf()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, &'a QVector3D) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector3D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVector<float> QOpenGLShaderProgram::defaultOuterTessellationLevels();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn defaultOuterTessellationLevels<RetType, T: QOpenGLShaderProgram_defaultOuterTessellationLevels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultOuterTessellationLevels(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_defaultOuterTessellationLevels<RetType> {
  fn defaultOuterTessellationLevels(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  QVector<float> QOpenGLShaderProgram::defaultOuterTessellationLevels();
impl<'a> /*trait*/ QOpenGLShaderProgram_defaultOuterTessellationLevels<()> for () {
  fn defaultOuterTessellationLevels(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram30defaultOuterTessellationLevelsEv()};
     unsafe {_ZNK20QOpenGLShaderProgram30defaultOuterTessellationLevelsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QColor & color);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QColor) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector2D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (&'a  String, &'a QVector2D, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector2Di()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector2Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, f32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcff()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a  Vec<i32>, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QString QOpenGLShaderProgram::log();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn log<RetType, T: QOpenGLShaderProgram_log<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.log(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_log<RetType> {
  fn log(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  QString QOpenGLShaderProgram::log();
impl<'a> /*trait*/ QOpenGLShaderProgram_log<QString> for () {
  fn log(self , rsthis: & QOpenGLShaderProgram) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram3logEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram3logEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSize & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QSize) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK5QSize()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK5QSize(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::patchVertexCount();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn patchVertexCount<RetType, T: QOpenGLShaderProgram_patchVertexCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.patchVertexCount(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_patchVertexCount<RetType> {
  fn patchVertexCount(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  int QOpenGLShaderProgram::patchVertexCount();
impl<'a> /*trait*/ QOpenGLShaderProgram_patchVertexCount<i32> for () {
  fn patchVertexCount(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram16patchVertexCountEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram16patchVertexCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [2][2] value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a  Vec<&'a  f32>) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiPA2_Kf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut *mut c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiPA2_Kf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QMatrix4x4 & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QMatrix4x4) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QMatrix4x4()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QMatrix4x4(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, &'a QVector4D) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShaderProgram::create();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn create<RetType, T: QOpenGLShaderProgram_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_create<RetType> {
  fn create(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  bool QOpenGLShaderProgram::create();
impl<'a> /*trait*/ QOpenGLShaderProgram_create<i8> for () {
  fn create(self , rsthis: & QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram6createEv()};
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSizeF & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QSizeF) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QSizeF()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QSizeF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::removeShader(QOpenGLShader * shader);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn removeShader<RetType, T: QOpenGLShaderProgram_removeShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeShader(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_removeShader<RetType> {
  fn removeShader(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::removeShader(QOpenGLShader * shader);
impl<'a> /*trait*/ QOpenGLShaderProgram_removeShader<()> for (&'a QOpenGLShader) {
  fn removeShader(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram12removeShaderEP13QOpenGLShader()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram12removeShaderEP13QOpenGLShader(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QVector4D) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector4D()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, f32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcff()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(const char * name, GLenum type, int offset, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeBuffer<()> for (&'a  String, u32, i32, i32, i32) {
  fn setAttributeBuffer(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram18setAttributeBufferEPKcjiii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram18setAttributeBufferEPKcjiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const GLfloat * values, int columns, int rows);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, &'a  Vec<f32>, i32, i32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiPKfii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::disableAttributeArray(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_disableAttributeArray<()> for (&'a  String) {
  fn disableAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21disableAttributeArrayEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN20QOpenGLShaderProgram21disableAttributeArrayEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPointF & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QPointF) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK7QPointF()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (i32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEif(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QString & name, int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_bindAttributeLocation<()> for (&'a QString, i32) {
  fn bindAttributeLocation(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21bindAttributeLocationERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21bindAttributeLocationERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector3D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (i32, &'a QVector3D, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector3Di()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector3Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QColor & color);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QColor) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QColor()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QTransform & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QTransform) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QTransform()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QVector3D) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector3D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector2D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a QVector2D, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector2Di()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector2Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::attributeLocation(const QByteArray & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_attributeLocation<i32> for (&'a QByteArray) {
  fn attributeLocation(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram17attributeLocationERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram17attributeLocationERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, f32, f32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcfff()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcfff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector4D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (i32, &'a QVector4D, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector4Di()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector4Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue<()> for (&'a  String, f32, f32, f32) {
  fn setAttributeValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcfff()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcfff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, GLenum type, const void * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (&'a  String, u32, *mut c_void, i32, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcjPKvii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcjPKvii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLuint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, u32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, i32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::QOpenGLShaderProgram(QObject * parent);
impl /*struct*/ QOpenGLShaderProgram {
  pub fn New<T: QOpenGLShaderProgram_New>(value: T) -> QOpenGLShaderProgram {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_New {
  fn New(self) -> QOpenGLShaderProgram;
}

  // proto:  void QOpenGLShaderProgram::QOpenGLShaderProgram(QObject * parent);
impl<'a> /*trait*/ QOpenGLShaderProgram_New for (&'a QObject) {
  fn New(self) -> QOpenGLShaderProgram {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgramC1EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLShaderProgram_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN20QOpenGLShaderProgramC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN20QOpenGLShaderProgramC1EP7QObject(arg0)} as u64;
    let rsthis = QOpenGLShaderProgram{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcf()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::enableAttributeArray(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_enableAttributeArray<()> for (&'a  String) {
  fn enableAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20enableAttributeArrayEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN20QOpenGLShaderProgram20enableAttributeArrayEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QMatrix4x4 & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QMatrix4x4) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QMatrix4x4()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QMatrix4x4(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, &'a QVector4D) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const GLfloat * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (i32, &'a  Vec<f32>, i32, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEiPKfii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEiPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSizeF & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QSizeF) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QSizeF()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QSizeF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::QOpenGLShaderProgram(const QOpenGLShaderProgram & );
impl<'a> /*trait*/ QOpenGLShaderProgram_New for (&'a QOpenGLShaderProgram) {
  fn New(self) -> QOpenGLShaderProgram {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgramC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLShaderProgram_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN20QOpenGLShaderProgramC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN20QOpenGLShaderProgramC1ERKS_(arg0)} as u64;
    let rsthis = QOpenGLShaderProgram{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (i32, f32, f32, f32, f32) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiffff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::removeAllShaders();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn removeAllShaders<RetType, T: QOpenGLShaderProgram_removeAllShaders<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAllShaders(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_removeAllShaders<RetType> {
  fn removeAllShaders(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::removeAllShaders();
impl<'a> /*trait*/ QOpenGLShaderProgram_removeAllShaders<()> for () {
  fn removeAllShaders(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram16removeAllShadersEv()};
     unsafe {_ZN20QOpenGLShaderProgram16removeAllShadersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::maxGeometryOutputVertices();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn maxGeometryOutputVertices<RetType, T: QOpenGLShaderProgram_maxGeometryOutputVertices<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxGeometryOutputVertices(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_maxGeometryOutputVertices<RetType> {
  fn maxGeometryOutputVertices(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  int QOpenGLShaderProgram::maxGeometryOutputVertices();
impl<'a> /*trait*/ QOpenGLShaderProgram_maxGeometryOutputVertices<i32> for () {
  fn maxGeometryOutputVertices(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram25maxGeometryOutputVerticesEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram25maxGeometryOutputVerticesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector3D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (&'a  String, &'a QVector3D, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector3Di()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector3Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::release();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn release<RetType, T: QOpenGLShaderProgram_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_release<RetType> {
  fn release(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  void QOpenGLShaderProgram::release();
impl<'a> /*trait*/ QOpenGLShaderProgram_release<()> for () {
  fn release(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram7releaseEv()};
     unsafe {_ZN20QOpenGLShaderProgram7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLShaderProgram::metaObject();
impl /*struct*/ QOpenGLShaderProgram {
  pub fn metaObject<RetType, T: QOpenGLShaderProgram_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLShaderProgram) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLShaderProgram::metaObject();
impl<'a> /*trait*/ QOpenGLShaderProgram_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram10metaObjectEv()};
     unsafe {_ZNK20QOpenGLShaderProgram10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QOpenGLShaderProgram::uniformLocation(const QByteArray & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_uniformLocation<i32> for (&'a QByteArray) {
  fn uniformLocation(self , rsthis: & QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram15uniformLocationERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram15uniformLocationERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector2D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue<()> for (&'a  String, &'a QVector2D) {
  fn setUniformValue(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector2D()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector2D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray<()> for (&'a  String, &'a QVector2D, i32) {
  fn setAttributeArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector2Di()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector2Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLfloat * values, int count, int tupleSize);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray<()> for (i32, &'a  Vec<f32>, i32, i32) {
  fn setUniformValueArray(self , rsthis: & QOpenGLShaderProgram) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKfii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// <= body block end

