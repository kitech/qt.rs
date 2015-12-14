// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvector3d::QVector3D;
use super::qpoint::QPoint;
use super::qtransform::QTransform;
use super::qopenglcontext::QOpenGLContext;
use super::qvector2d::QVector2D;
use super::qbytearray::QByteArray;
use super::qcolor::QColor;
use super::qsize::QSize;
use super::qopenglshader::QOpenGLShader;
use super::qstring::QString;
use super::qmatrix4x4::QMatrix4x4;
use super::qvector4d::QVector4D;
use super::qpointf::QPointF;
use super::qsizef::QSizeF;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QOpenGLShaderProgram::isLinked();
  fn _ZNK20QOpenGLShaderProgram8isLinkedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector3D(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPoint & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK6QPoint(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcffff(qthis: *mut c_void, arg0: *const c_char, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector3D(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector3D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector3Di(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QTransform & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK10QTransform(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(int location, GLenum type, int offset, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram18setAttributeBufferEijiii(qthis: *mut c_void, arg0: c_int, arg1: c_uint, arg2: c_int, arg3: c_int, arg4: c_int) ;
  // proto: static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms(QOpenGLContext * context);
  fn _ZN20QOpenGLShaderProgram23hasOpenGLShaderProgramsEP14QOpenGLContext(arg0: *mut c_void) -> int8_t;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [2][2] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcPA2_Kf(qthis: *mut c_void, arg0: *const c_char, arg1: *const *const c_float) ;
  // proto:  void QOpenGLShaderProgram::setPatchVertexCount(int count);
  fn _ZN20QOpenGLShaderProgram19setPatchVertexCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const GLfloat * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPKfii(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_float, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector3D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector3Di(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [3][3] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcPA3_Kf(qthis: *mut c_void, arg0: *const c_char, arg1: *const *const c_float) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector2D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QByteArray & name, int location);
  fn _ZN20QOpenGLShaderProgram21bindAttributeLocationERK10QByteArrayi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLfloat * values, int count, int tupleSize);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKfii(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_float, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector2D(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QColor & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEifff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  bool QOpenGLShaderProgram::bind();
  fn _ZN20QOpenGLShaderProgram4bindEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEif(qthis: *mut c_void, arg0: c_int, arg1: c_float) ;
  // proto:  void QOpenGLShaderProgram::enableAttributeArray(int location);
  fn _ZN20QOpenGLShaderProgram20enableAttributeArrayEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLuint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEij(qthis: *mut c_void, arg0: c_int, arg1: c_uint) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSize & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK5QSize(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  bool QOpenGLShaderProgram::addShader(QOpenGLShader * shader);
  fn _ZN20QOpenGLShaderProgram9addShaderEP13QOpenGLShader(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QOpenGLShaderProgram::attributeLocation(const QString & name);
  fn _ZNK20QOpenGLShaderProgram17attributeLocationERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, GLenum type, const void * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEijPKvii(qthis: *mut c_void, arg0: c_int, arg1: c_uint, arg2: *const uint8_t, arg3: c_int, arg4: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPoint & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QPoint(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QMatrix4x4 * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK10QMatrix4x4i(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector2D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector2Di(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QMatrix4x4 * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK10QMatrix4x4i(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QColor & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK6QColor(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcffff(qthis: *mut c_void, arg0: *const c_char, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) ;
  // proto:  QOpenGLShaderProgram::GLuint QOpenGLShaderProgram::programId();
  fn _ZNK20QOpenGLShaderProgram9programIdEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLuint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKji(qthis: *mut c_void, arg0: c_int, arg1: *const c_uint, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::disableAttributeArray(int location);
  fn _ZN20QOpenGLShaderProgram21disableAttributeArrayEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKii(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_int, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLuint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKji(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_uint, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const GLfloat * values, int columns, int rows);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcPKfii(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_float, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector4D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector4Di(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector2D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEifff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiffff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) ;
  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const char * name, int location);
  fn _ZN20QOpenGLShaderProgram21bindAttributeLocationEPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) ;
  // proto:  int QOpenGLShaderProgram::attributeLocation(const char * name);
  fn _ZNK20QOpenGLShaderProgram17attributeLocationEPKc(qthis: *mut c_void, arg0: *const c_char) -> c_int;
  // proto:  int QOpenGLShaderProgram::uniformLocation(const QString & name);
  fn _ZNK20QOpenGLShaderProgram15uniformLocationERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPointF & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK7QPointF(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector4D(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector4D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector4Di(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  int QOpenGLShaderProgram::uniformLocation(const char * name);
  fn _ZNK20QOpenGLShaderProgram15uniformLocationEPKc(qthis: *mut c_void, arg0: *const c_char) -> c_int;
  // proto:  void QOpenGLShaderProgram::FreeQOpenGLShaderProgram();
  fn _ZN20QOpenGLShaderProgramD0Ev(qthis: *mut c_void) ;
  // proto:  QVector<float> QOpenGLShaderProgram::defaultInnerTessellationLevels();
  fn _ZNK20QOpenGLShaderProgram30defaultInnerTessellationLevelsEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLShaderProgram::link();
  fn _ZN20QOpenGLShaderProgram4linkEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector4D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector4Di(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  QList<QOpenGLShader *> QOpenGLShaderProgram::shaders();
  fn _ZNK20QOpenGLShaderProgram7shadersEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcf(qthis: *mut c_void, arg0: *const c_char, arg1: c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector3D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QVector<float> QOpenGLShaderProgram::defaultOuterTessellationLevels();
  fn _ZNK20QOpenGLShaderProgram30defaultOuterTessellationLevelsEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QColor & color);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector2D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector2Di(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [4][4] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcPA4_Kf(qthis: *mut c_void, arg0: *const c_char, arg1: *mut *mut c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcff(qthis: *mut c_void, arg0: *const c_char, arg1: c_float, arg2: c_float) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLint * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKii(qthis: *mut c_void, arg0: c_int, arg1: *const c_int, arg2: c_int) ;
  // proto:  QString QOpenGLShaderProgram::log();
  fn _ZNK20QOpenGLShaderProgram3logEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSize & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK5QSize(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  int QOpenGLShaderProgram::patchVertexCount();
  fn _ZNK20QOpenGLShaderProgram16patchVertexCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [2][2] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiPA2_Kf(qthis: *mut c_void, arg0: c_int, arg1: *const *const c_float) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QMatrix4x4 & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QMatrix4x4(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector4D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  bool QOpenGLShaderProgram::create();
  fn _ZN20QOpenGLShaderProgram6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSizeF & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK6QSizeF(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::removeShader(QOpenGLShader * shader);
  fn _ZN20QOpenGLShaderProgram12removeShaderEP13QOpenGLShader(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector4D(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcff(qthis: *mut c_void, arg0: *const c_char, arg1: c_float, arg2: c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeBuffer(const char * name, GLenum type, int offset, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram18setAttributeBufferEPKcjiii(qthis: *mut c_void, arg0: *const c_char, arg1: c_uint, arg2: c_int, arg3: c_int, arg4: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const GLfloat * values, int columns, int rows);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEiPKfii(qthis: *mut c_void, arg0: c_int, arg1: *const c_float, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLShaderProgram::disableAttributeArray(const char * name);
  fn _ZN20QOpenGLShaderProgram21disableAttributeArrayEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPointF & point);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK7QPointF(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [4][4] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiPA4_Kf(qthis: *mut c_void, arg0: c_int, arg1: *mut *mut c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat value);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEif(qthis: *mut c_void, arg0: c_int, arg1: c_float) ;
  // proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QString & name, int location);
  fn _ZN20QOpenGLShaderProgram21bindAttributeLocationERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector3D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector3Di(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QColor & color);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QColor(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QTransform & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QTransform(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector3D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector3D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector2D * values, int count);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPK9QVector2Di(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  int QOpenGLShaderProgram::attributeLocation(const QByteArray & name);
  fn _ZNK20QOpenGLShaderProgram17attributeLocationERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcfff(qthis: *mut c_void, arg0: *const c_char, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector4D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPK9QVector4Di(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN20QOpenGLShaderProgram17setAttributeValueEPKcfff(qthis: *mut c_void, arg0: *const c_char, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [3][3] value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiPA3_Kf(qthis: *mut c_void, arg0: c_int, arg1: *mut *mut c_float) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, GLenum type, const void * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcjPKvii(qthis: *mut c_void, arg0: *const c_char, arg1: c_uint, arg2: *const uint8_t, arg3: c_int, arg4: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLuint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcj(qthis: *mut c_void, arg0: *const c_char, arg1: c_uint) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLint value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) ;
  // proto:  void QOpenGLShaderProgram::NewQOpenGLShaderProgram(QObject * parent);
  fn _ZN20QOpenGLShaderProgramC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcf(qthis: *mut c_void, arg0: *const c_char, arg1: c_float) ;
  // proto:  void QOpenGLShaderProgram::enableAttributeArray(const char * name);
  fn _ZN20QOpenGLShaderProgram20enableAttributeArrayEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QMatrix4x4 & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK10QMatrix4x4(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector4D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector4D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const GLfloat * values, int tupleSize, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEiPKfii(qthis: *mut c_void, arg0: c_int, arg1: *const c_float, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSizeF & size);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QSizeF(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::NewQOpenGLShaderProgram(const QOpenGLShaderProgram & );
  fn _ZN20QOpenGLShaderProgramC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEiffff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) ;
  // proto:  void QOpenGLShaderProgram::removeAllShaders();
  fn _ZN20QOpenGLShaderProgram16removeAllShadersEv(qthis: *mut c_void) ;
  // proto:  int QOpenGLShaderProgram::maxGeometryOutputVertices();
  fn _ZNK20QOpenGLShaderProgram25maxGeometryOutputVerticesEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector3D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector3Di(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::release();
  fn _ZN20QOpenGLShaderProgram7releaseEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QOpenGLShaderProgram::metaObject();
  fn _ZNK20QOpenGLShaderProgram10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QOpenGLShaderProgram::uniformLocation(const QByteArray & name);
  fn _ZNK20QOpenGLShaderProgram15uniformLocationERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector2D & value);
  fn _ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector2D(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) ;
  // proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector2D * values, int stride);
  fn _ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector2Di(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLfloat * values, int count, int tupleSize);
  fn _ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKfii(qthis: *mut c_void, arg0: c_int, arg1: *const c_float, arg2: c_int, arg3: c_int) ;
}

// body block begin
// class sizeof(QOpenGLShaderProgram)=1
pub struct QOpenGLShaderProgram {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn isLinked<T: QOpenGLShaderProgram_isLinked>(&mut self, value: T) -> i8 {
    return value.isLinked(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_isLinked {
  fn isLinked(self, rsthis: &mut QOpenGLShaderProgram) -> i8;
}

// proto:  bool QOpenGLShaderProgram::isLinked();
impl<'a> /*trait*/ QOpenGLShaderProgram_isLinked for () {
  fn isLinked(self, rsthis: &mut QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram8isLinkedEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram8isLinkedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn setUniformValue<T: QOpenGLShaderProgram_setUniformValue>(&mut self, value: T)  {
     value.setUniformValue(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setUniformValue {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QVector3D) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector3D()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPoint & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QPoint) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QPoint()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, f32, f32, f32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcffff()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn setAttributeValue<T: QOpenGLShaderProgram_setAttributeValue>(&mut self, value: T)  {
     value.setAttributeValue(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setAttributeValue {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, &'a  QVector3D) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector3D()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn setUniformValueArray<T: QOpenGLShaderProgram_setUniformValueArray>(&mut self, value: T)  {
     value.setUniformValueArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setUniformValueArray {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector3D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  QVector3D, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector3Di()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector3Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QTransform & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QTransform) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QTransform()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn setAttributeBuffer<T: QOpenGLShaderProgram_setAttributeBuffer>(&mut self, value: T)  {
     value.setAttributeBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setAttributeBuffer {
  fn setAttributeBuffer(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::setAttributeBuffer(int location, GLenum type, int offset, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeBuffer for (i32, u32, i32, i32, i32) {
  fn setAttributeBuffer(self, rsthis: &mut QOpenGLShaderProgram)  {
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

impl /*struct*/ QOpenGLShaderProgram {
  pub fn hasOpenGLShaderPrograms<T: QOpenGLShaderProgram_hasOpenGLShaderPrograms>(&mut self, value: T) -> i8 {
    return value.hasOpenGLShaderPrograms(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_hasOpenGLShaderPrograms {
  fn hasOpenGLShaderPrograms(self, rsthis: &mut QOpenGLShaderProgram) -> i8;
}

// proto: static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLShaderProgram_hasOpenGLShaderPrograms for (&'a mut QOpenGLContext) {
  fn hasOpenGLShaderPrograms(self, rsthis: &mut QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram23hasOpenGLShaderProgramsEP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram23hasOpenGLShaderProgramsEP14QOpenGLContext(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const GLfloat [2][2] value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  Vec<&'a  f32>) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcPA2_Kf()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const *const c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcPA2_Kf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn setPatchVertexCount<T: QOpenGLShaderProgram_setPatchVertexCount>(&mut self, value: T)  {
     value.setPatchVertexCount(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setPatchVertexCount {
  fn setPatchVertexCount(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::setPatchVertexCount(int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setPatchVertexCount for (i32) {
  fn setPatchVertexCount(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram19setPatchVertexCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram19setPatchVertexCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn setAttributeArray<T: QOpenGLShaderProgram_setAttributeArray>(&mut self, value: T)  {
     value.setAttributeArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_setAttributeArray {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const GLfloat * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (&'a  String, &'a  f32, i32, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPKfii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector3D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  QVector3D, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QVector2D) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector2D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn bindAttributeLocation<T: QOpenGLShaderProgram_bindAttributeLocation>(&mut self, value: T)  {
     value.bindAttributeLocation(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_bindAttributeLocation {
  fn bindAttributeLocation(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QByteArray & name, int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_bindAttributeLocation for (&'a  QByteArray, i32) {
  fn bindAttributeLocation(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21bindAttributeLocationERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21bindAttributeLocationERK10QByteArrayi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLfloat * values, int count, int tupleSize);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  f32, i32, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKfii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, f32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, &'a  QVector2D) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector2D()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QColor & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, &'a  QColor) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, f32, f32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
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

impl /*struct*/ QOpenGLShaderProgram {
  pub fn bind<T: QOpenGLShaderProgram_bind>(&mut self, value: T) -> i8 {
    return value.bind(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_bind {
  fn bind(self, rsthis: &mut QOpenGLShaderProgram) -> i8;
}

// proto:  bool QOpenGLShaderProgram::bind();
impl<'a> /*trait*/ QOpenGLShaderProgram_bind for () {
  fn bind(self, rsthis: &mut QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram4bindEv()};
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram4bindEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEif(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn enableAttributeArray<T: QOpenGLShaderProgram_enableAttributeArray>(&mut self, value: T)  {
     value.enableAttributeArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_enableAttributeArray {
  fn enableAttributeArray(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::enableAttributeArray(int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_enableAttributeArray for (i32) {
  fn enableAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20enableAttributeArrayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20enableAttributeArrayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, i32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLuint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, u32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEij(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSize & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QSize) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK5QSize()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK5QSize(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn addShader<T: QOpenGLShaderProgram_addShader>(&mut self, value: T) -> i8 {
    return value.addShader(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_addShader {
  fn addShader(self, rsthis: &mut QOpenGLShaderProgram) -> i8;
}

// proto:  bool QOpenGLShaderProgram::addShader(QOpenGLShader * shader);
impl<'a> /*trait*/ QOpenGLShaderProgram_addShader for (&'a mut QOpenGLShader) {
  fn addShader(self, rsthis: &mut QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram9addShaderEP13QOpenGLShader()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram9addShaderEP13QOpenGLShader(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn attributeLocation<T: QOpenGLShaderProgram_attributeLocation>(&mut self, value: T) -> i32 {
    return value.attributeLocation(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_attributeLocation {
  fn attributeLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32;
}

// proto:  int QOpenGLShaderProgram::attributeLocation(const QString & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_attributeLocation for (&'a  QString) {
  fn attributeLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram17attributeLocationERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram17attributeLocationERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(int location, GLenum type, const void * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (i32, u32, &'a  u8, i32, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEijPKvii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *const uint8_t;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEijPKvii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPoint & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QPoint) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QPoint()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QMatrix4x4 * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  QMatrix4x4, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (i32, &'a  QVector2D, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, f32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  QMatrix4x4, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK10QMatrix4x4i()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK10QMatrix4x4i(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QColor & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, &'a  QColor) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK6QColor()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, f32, f32, f32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcffff()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn programId<T: QOpenGLShaderProgram_programId>(&mut self, value: T)  {
     value.programId(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_programId {
  fn programId(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  QOpenGLShaderProgram::GLuint QOpenGLShaderProgram::programId();
impl<'a> /*trait*/ QOpenGLShaderProgram_programId for () {
  fn programId(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram9programIdEv()};
     unsafe {_ZNK20QOpenGLShaderProgram9programIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLuint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  u32, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKji()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKji(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn disableAttributeArray<T: QOpenGLShaderProgram_disableAttributeArray>(&mut self, value: T)  {
     value.disableAttributeArray(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_disableAttributeArray {
  fn disableAttributeArray(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::disableAttributeArray(int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_disableAttributeArray for (i32) {
  fn disableAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21disableAttributeArrayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21disableAttributeArrayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  i32, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const GLuint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  u32, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKji()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const c_uint;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPKji(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const GLfloat * values, int columns, int rows);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, &'a  f32, i32, i32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcPKfii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector4D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (&'a  String, &'a  QVector4D, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector4Di()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector4Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector2D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, &'a  QVector2D) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector2D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, f32, f32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, f32, f32, f32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_bindAttributeLocation for (&'a  String, i32) {
  fn bindAttributeLocation(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21bindAttributeLocationEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21bindAttributeLocationEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QOpenGLShaderProgram::attributeLocation(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_attributeLocation for (&'a  String) {
  fn attributeLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram17attributeLocationEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram17attributeLocationEPKc(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn uniformLocation<T: QOpenGLShaderProgram_uniformLocation>(&mut self, value: T) -> i32 {
    return value.uniformLocation(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_uniformLocation {
  fn uniformLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32;
}

// proto:  int QOpenGLShaderProgram::uniformLocation(const QString & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_uniformLocation for (&'a  QString) {
  fn uniformLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram15uniformLocationERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram15uniformLocationERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QPointF & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QPointF) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK7QPointF()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, &'a  QVector4D) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector4D()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector4D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  QVector4D, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_uniformLocation for (&'a  String) {
  fn uniformLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram15uniformLocationEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram15uniformLocationEPKc(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn FreeQOpenGLShaderProgram<T: QOpenGLShaderProgram_FreeQOpenGLShaderProgram>(&mut self, value: T)  {
     value.FreeQOpenGLShaderProgram(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_FreeQOpenGLShaderProgram {
  fn FreeQOpenGLShaderProgram(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::FreeQOpenGLShaderProgram();
impl<'a> /*trait*/ QOpenGLShaderProgram_FreeQOpenGLShaderProgram for () {
  fn FreeQOpenGLShaderProgram(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgramD0Ev()};
     unsafe {_ZN20QOpenGLShaderProgramD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn defaultInnerTessellationLevels<T: QOpenGLShaderProgram_defaultInnerTessellationLevels>(&mut self, value: T)  {
     value.defaultInnerTessellationLevels(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_defaultInnerTessellationLevels {
  fn defaultInnerTessellationLevels(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  QVector<float> QOpenGLShaderProgram::defaultInnerTessellationLevels();
impl<'a> /*trait*/ QOpenGLShaderProgram_defaultInnerTessellationLevels for () {
  fn defaultInnerTessellationLevels(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram30defaultInnerTessellationLevelsEv()};
     unsafe {_ZNK20QOpenGLShaderProgram30defaultInnerTessellationLevelsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn link<T: QOpenGLShaderProgram_link>(&mut self, value: T) -> i8 {
    return value.link(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_link {
  fn link(self, rsthis: &mut QOpenGLShaderProgram) -> i8;
}

// proto:  bool QOpenGLShaderProgram::link();
impl<'a> /*trait*/ QOpenGLShaderProgram_link for () {
  fn link(self, rsthis: &mut QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram4linkEv()};
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram4linkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector4D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  QVector4D, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector4Di()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector4Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn shaders<T: QOpenGLShaderProgram_shaders>(&mut self, value: T)  {
     value.shaders(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_shaders {
  fn shaders(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  QList<QOpenGLShader *> QOpenGLShaderProgram::shaders();
impl<'a> /*trait*/ QOpenGLShaderProgram_shaders for () {
  fn shaders(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram7shadersEv()};
     unsafe {_ZNK20QOpenGLShaderProgram7shadersEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcf()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, &'a  QVector3D) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector3D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn defaultOuterTessellationLevels<T: QOpenGLShaderProgram_defaultOuterTessellationLevels>(&mut self, value: T)  {
     value.defaultOuterTessellationLevels(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_defaultOuterTessellationLevels {
  fn defaultOuterTessellationLevels(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  QVector<float> QOpenGLShaderProgram::defaultOuterTessellationLevels();
impl<'a> /*trait*/ QOpenGLShaderProgram_defaultOuterTessellationLevels for () {
  fn defaultOuterTessellationLevels(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram30defaultOuterTessellationLevelsEv()};
     unsafe {_ZNK20QOpenGLShaderProgram30defaultOuterTessellationLevelsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QColor & color);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QColor) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(const char * name, const QVector2D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (&'a  String, &'a  QVector2D, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector2Di()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEPKcPK9QVector2Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(const char * name, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, f32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcff()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLint * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  i32, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn log<T: QOpenGLShaderProgram_log>(&mut self, value: T) -> QString {
    return value.log(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_log {
  fn log(self, rsthis: &mut QOpenGLShaderProgram) -> QString;
}

// proto:  QString QOpenGLShaderProgram::log();
impl<'a> /*trait*/ QOpenGLShaderProgram_log for () {
  fn log(self, rsthis: &mut QOpenGLShaderProgram) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram3logEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram3logEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSize & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QSize) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK5QSize()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK5QSize(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn patchVertexCount<T: QOpenGLShaderProgram_patchVertexCount>(&mut self, value: T) -> i32 {
    return value.patchVertexCount(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_patchVertexCount {
  fn patchVertexCount(self, rsthis: &mut QOpenGLShaderProgram) -> i32;
}

// proto:  int QOpenGLShaderProgram::patchVertexCount();
impl<'a> /*trait*/ QOpenGLShaderProgram_patchVertexCount for () {
  fn patchVertexCount(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram16patchVertexCountEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram16patchVertexCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const GLfloat [2][2] value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  Vec<&'a  f32>) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiPA2_Kf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *const *const c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiPA2_Kf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QMatrix4x4 & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QMatrix4x4) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QMatrix4x4()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QMatrix4x4(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, &'a  QVector4D) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn create<T: QOpenGLShaderProgram_create>(&mut self, value: T) -> i8 {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_create {
  fn create(self, rsthis: &mut QOpenGLShaderProgram) -> i8;
}

// proto:  bool QOpenGLShaderProgram::create();
impl<'a> /*trait*/ QOpenGLShaderProgram_create for () {
  fn create(self, rsthis: &mut QOpenGLShaderProgram) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram6createEv()};
    let mut ret = unsafe {_ZN20QOpenGLShaderProgram6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QSizeF & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QSizeF) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QSizeF()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK6QSizeF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn removeShader<T: QOpenGLShaderProgram_removeShader>(&mut self, value: T)  {
     value.removeShader(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_removeShader {
  fn removeShader(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::removeShader(QOpenGLShader * shader);
impl<'a> /*trait*/ QOpenGLShaderProgram_removeShader for (&'a mut QOpenGLShader) {
  fn removeShader(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram12removeShaderEP13QOpenGLShader()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram12removeShaderEP13QOpenGLShader(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QVector4D) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector4D()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, f32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcff()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeBuffer(const char * name, GLenum type, int offset, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeBuffer for (&'a  String, u32, i32, i32, i32) {
  fn setAttributeBuffer(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram18setAttributeBufferEPKcjiii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram18setAttributeBufferEPKcjiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, const GLfloat * values, int columns, int rows);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, &'a  f32, i32, i32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEiPKfii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEiPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::disableAttributeArray(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_disableAttributeArray for (&'a  String) {
  fn disableAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21disableAttributeArrayEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN20QOpenGLShaderProgram21disableAttributeArrayEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QPointF & point);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QPointF) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK7QPointF()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeValue(int location, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (i32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEif(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::bindAttributeLocation(const QString & name, int location);
impl<'a> /*trait*/ QOpenGLShaderProgram_bindAttributeLocation for (&'a  QString, i32) {
  fn bindAttributeLocation(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram21bindAttributeLocationERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram21bindAttributeLocationERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector3D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (i32, &'a  QVector3D, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QColor) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QColor()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QTransform & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QTransform) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QTransform()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector3D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QVector3D) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector3D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector2D * values, int count);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  QVector2D, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_attributeLocation for (&'a  QByteArray) {
  fn attributeLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram17attributeLocationERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram17attributeLocationERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, f32, f32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcfff()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcfff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const QVector4D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (i32, &'a  QVector4D, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
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
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeValue for (&'a  String, f32, f32, f32) {
  fn setAttributeValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeValueEPKcfff()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeValueEPKcfff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, GLenum type, const void * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (&'a  String, u32, &'a  u8, i32, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcjPKvii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *const uint8_t;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcjPKvii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLuint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, u32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcj()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLint value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, i32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn NewQOpenGLShaderProgram<T: QOpenGLShaderProgram_NewQOpenGLShaderProgram>(value: T) -> QOpenGLShaderProgram {
    let rsthis = value.NewQOpenGLShaderProgram();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_NewQOpenGLShaderProgram {
  fn NewQOpenGLShaderProgram(self) -> QOpenGLShaderProgram;
}

// proto: void QOpenGLShaderProgram::NewQOpenGLShaderProgram(QObject * parent);
impl<'a> /*trait*/ QOpenGLShaderProgram_NewQOpenGLShaderProgram for (&'a mut QObject) {
  fn NewQOpenGLShaderProgram(self) -> QOpenGLShaderProgram {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgramC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QOpenGLShaderProgramC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLShaderProgram{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, GLfloat value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcf()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_float;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::enableAttributeArray(const char * name);
impl<'a> /*trait*/ QOpenGLShaderProgram_enableAttributeArray for (&'a  String) {
  fn enableAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20enableAttributeArrayEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN20QOpenGLShaderProgram20enableAttributeArrayEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QMatrix4x4 & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QMatrix4x4) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QMatrix4x4()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK10QMatrix4x4(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, const QVector4D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, &'a  QVector4D) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEiRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(int location, const GLfloat * values, int tupleSize, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (i32, &'a  f32, i32, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEiPKfii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEiPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QSizeF & size);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QSizeF) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QSizeF()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK6QSizeF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QOpenGLShaderProgram::NewQOpenGLShaderProgram(const QOpenGLShaderProgram & );
impl<'a> /*trait*/ QOpenGLShaderProgram_NewQOpenGLShaderProgram for (&'a  QOpenGLShaderProgram) {
  fn NewQOpenGLShaderProgram(self) -> QOpenGLShaderProgram {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgramC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QOpenGLShaderProgramC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLShaderProgram{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(int location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (i32, f32, f32, f32, f32) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
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

impl /*struct*/ QOpenGLShaderProgram {
  pub fn removeAllShaders<T: QOpenGLShaderProgram_removeAllShaders>(&mut self, value: T)  {
     value.removeAllShaders(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_removeAllShaders {
  fn removeAllShaders(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::removeAllShaders();
impl<'a> /*trait*/ QOpenGLShaderProgram_removeAllShaders for () {
  fn removeAllShaders(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram16removeAllShadersEv()};
     unsafe {_ZN20QOpenGLShaderProgram16removeAllShadersEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn maxGeometryOutputVertices<T: QOpenGLShaderProgram_maxGeometryOutputVertices>(&mut self, value: T) -> i32 {
    return value.maxGeometryOutputVertices(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_maxGeometryOutputVertices {
  fn maxGeometryOutputVertices(self, rsthis: &mut QOpenGLShaderProgram) -> i32;
}

// proto:  int QOpenGLShaderProgram::maxGeometryOutputVertices();
impl<'a> /*trait*/ QOpenGLShaderProgram_maxGeometryOutputVertices for () {
  fn maxGeometryOutputVertices(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram25maxGeometryOutputVerticesEv()};
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram25maxGeometryOutputVerticesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector3D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (&'a  String, &'a  QVector3D, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector3Di()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector3Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn release<T: QOpenGLShaderProgram_release>(&mut self, value: T)  {
     value.release(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_release {
  fn release(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  void QOpenGLShaderProgram::release();
impl<'a> /*trait*/ QOpenGLShaderProgram_release for () {
  fn release(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram7releaseEv()};
     unsafe {_ZN20QOpenGLShaderProgram7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLShaderProgram {
  pub fn metaObject<T: QOpenGLShaderProgram_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLShaderProgram_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLShaderProgram) ;
}

// proto:  const QMetaObject * QOpenGLShaderProgram::metaObject();
impl<'a> /*trait*/ QOpenGLShaderProgram_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram10metaObjectEv()};
     unsafe {_ZNK20QOpenGLShaderProgram10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QOpenGLShaderProgram::uniformLocation(const QByteArray & name);
impl<'a> /*trait*/ QOpenGLShaderProgram_uniformLocation for (&'a  QByteArray) {
  fn uniformLocation(self, rsthis: &mut QOpenGLShaderProgram) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QOpenGLShaderProgram15uniformLocationERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QOpenGLShaderProgram15uniformLocationERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValue(const char * name, const QVector2D & value);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValue for (&'a  String, &'a  QVector2D) {
  fn setUniformValue(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector2D()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QOpenGLShaderProgram15setUniformValueEPKcRK9QVector2D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setAttributeArray(const char * name, const QVector2D * values, int stride);
impl<'a> /*trait*/ QOpenGLShaderProgram_setAttributeArray for (&'a  String, &'a  QVector2D, i32) {
  fn setAttributeArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector2Di()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram17setAttributeArrayEPKcPK9QVector2Di(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QOpenGLShaderProgram::setUniformValueArray(int location, const GLfloat * values, int count, int tupleSize);
impl<'a> /*trait*/ QOpenGLShaderProgram_setUniformValueArray for (i32, &'a  f32, i32, i32) {
  fn setUniformValueArray(self, rsthis: &mut QOpenGLShaderProgram)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKfii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_float;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN20QOpenGLShaderProgram20setUniformValueArrayEiPKfii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

