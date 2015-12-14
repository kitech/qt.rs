// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qopenglcontext::QOpenGLContext;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
  fn _ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *const c_char) ;
  // proto:  void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
  fn _ZN16QOpenGLFunctions17glGenFramebuffersEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint) ;
  // proto:  void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform3ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_int) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *const c_float) ;
  // proto:  unsigned char QOpenGLFunctions::glIsBuffer(GLuint buffer);
  fn _ZN16QOpenGLFunctions10glIsBufferEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glLineWidth(GLfloat width);
  fn _ZN16QOpenGLFunctions11glLineWidthEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
  fn _ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
  fn _ZN16QOpenGLFunctions13glDepthRangefEff(qthis: *mut c_void, arg0: c_float, arg1: c_float) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *const c_float) ;
  // proto:  void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
  fn _ZN16QOpenGLFunctions16glTexParameterivEjjPKi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *const c_int) ;
  // proto:  void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
  fn _ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glDeleteProgram(GLuint program);
  fn _ZN16QOpenGLFunctions15glDeleteProgramEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
  fn _ZN16QOpenGLFunctions23glBlendEquationSeparateEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
  fn _ZN16QOpenGLFunctions21glStencilMaskSeparateEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
  fn _ZN16QOpenGLFunctions12glDrawArraysEjii(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int) ;
  // proto:  void QOpenGLFunctions::glFinish();
  fn _ZN16QOpenGLFunctions8glFinishEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
  fn _ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut uint8_t) ;
  // proto:  void QOpenGLFunctions::glActiveTexture(GLenum texture);
  fn _ZN16QOpenGLFunctions15glActiveTextureEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glFrontFace(GLenum mode);
  fn _ZN16QOpenGLFunctions11glFrontFaceEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_float) ;
  // proto:  void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
  fn _ZN16QOpenGLFunctions13glPixelStoreiEji(qthis: *mut c_void, arg0: c_uint, arg1: c_int) ;
  // proto:  void QOpenGLFunctions::glCullFace(GLenum mode);
  fn _ZN16QOpenGLFunctions10glCullFaceEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions13glGetShaderivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
  fn _ZN16QOpenGLFunctions11glUniform4iEiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) ;
  // proto:  void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
  fn _ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: *mut uint8_t) ;
  // proto:  void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
  fn _ZN16QOpenGLFunctions15glTexParameteriEjji(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int) ;
  // proto:  void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
  fn _ZN16QOpenGLFunctions12glClearColorEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QOpenGLFunctions::glClearDepthf(GLclampf depth);
  fn _ZN16QOpenGLFunctions13glClearDepthfEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
  fn _ZN16QOpenGLFunctions11glUniform2iEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  void QOpenGLFunctions::glGenerateMipmap(GLenum target);
  fn _ZN16QOpenGLFunctions16glGenerateMipmapEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
  fn _ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_int, arg8: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
  fn _ZN16QOpenGLFunctions11glUniform3iEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
  fn _ZN16QOpenGLFunctions13glGenTexturesEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint) ;
  // proto:  void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
  fn _ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  void QOpenGLFunctions::FreeQOpenGLFunctions();
  fn _ZN16QOpenGLFunctionsD0Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform4fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_float) ;
  // proto:  void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions14glGetProgramivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *const c_float) ;
  // proto:  void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
  fn _ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_char) ;
  // proto:  unsigned char QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions16glIsRenderbufferEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions19glCopyTexSubImage2DEjiiiiiii(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int) ;
  // proto:  void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
  fn _ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *const c_char, arg3: *const c_int) ;
  // proto:  void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_float) ;
  // proto:  void QOpenGLFunctions::glDepthFunc(GLenum func);
  fn _ZN16QOpenGLFunctions11glDepthFuncEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
  fn _ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
  fn _ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *const c_uint) ;
  // proto:  void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
  fn _ZN16QOpenGLFunctions6glHintEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  QOpenGLFunctions::GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
  fn _ZN16QOpenGLFunctions20glGetUniformLocationEjPKc(qthis: *mut c_void, arg0: c_uint, arg1: *const c_char) ;
  // proto:  unsigned char QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
  fn _ZN16QOpenGLFunctions15glIsFramebufferEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform1fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_float) ;
  // proto:  const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
  fn _ZN16QOpenGLFunctions11glGetStringEj(qthis: *mut c_void, arg0: c_uint) -> *const c_uchar;
  // proto:  void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *const c_float) ;
  // proto:  void QOpenGLFunctions::NewQOpenGLFunctions(QOpenGLContext * context);
  fn _ZN16QOpenGLFunctionsC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *const c_float) ;
  // proto:  void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
  fn _ZN16QOpenGLFunctions12glBindBufferEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
  fn _ZN16QOpenGLFunctions11glUniform2fEiff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float) ;
  // proto:  void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform3fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_float) ;
  // proto:  void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform2fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_float) ;
  // proto:  void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform1ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_int) ;
  // proto:  void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
  fn _ZN16QOpenGLFunctions12glBlendColorEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
  fn _ZN16QOpenGLFunctions14glDrawElementsEjijPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
  fn _ZN16QOpenGLFunctions17glBindFramebufferEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  unsigned char QOpenGLFunctions::glIsProgram(GLuint program);
  fn _ZN16QOpenGLFunctions11glIsProgramEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glBlendEquation(GLenum mode);
  fn _ZN16QOpenGLFunctions15glBlendEquationEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
  fn _ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi(qthis: *mut c_void, arg0: c_int, arg1: *const c_uint, arg2: c_uint, arg3: *const uint8_t, arg4: c_int) ;
  // proto:  void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
  fn _ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char) ;
  // proto:  void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
  fn _ZN16QOpenGLFunctions15glDeleteBuffersEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *const c_uint) ;
  // proto:  void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions9glScissorEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
  fn _ZN16QOpenGLFunctions18glGenRenderbuffersEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN16QOpenGLFunctions16glVertexAttrib3fEjfff(qthis: *mut c_void, arg0: c_uint, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateProgram();
  fn _ZN16QOpenGLFunctions15glCreateProgramEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform4ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_int) ;
  // proto:  void QOpenGLFunctions::glEnable(GLenum cap);
  fn _ZN16QOpenGLFunctions8glEnableEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
  fn _ZN16QOpenGLFunctions13glBindTextureEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
  fn _ZN16QOpenGLFunctions15glTexParameterfEjjf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_float) ;
  // proto:  void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions10glViewportEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
  fn _ZN16QOpenGLFunctions16glSampleCoverageEfh(qthis: *mut c_void, arg0: c_float, arg1: c_uchar) ;
  // proto:  void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
  fn _ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint, arg4: c_int) ;
  // proto:  void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
  fn _ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_uchar, arg4: c_int, arg5: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
  fn _ZN16QOpenGLFunctions15glPolygonOffsetEff(qthis: *mut c_void, arg0: c_float, arg1: c_float) ;
  // proto:  QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateShader(GLenum type);
  fn _ZN16QOpenGLFunctions14glCreateShaderEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
  fn _ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char) ;
  // proto:  unsigned char QOpenGLFunctions::glIsTexture(GLuint texture);
  fn _ZN16QOpenGLFunctions11glIsTextureEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
  fn _ZN16QOpenGLFunctions16glDeleteTexturesEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *const c_uint) ;
  // proto:  void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions13glGetIntegervEjPi(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
  fn _ZN16QOpenGLFunctions13glGetBooleanvEjPh(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_uchar) ;
  // proto:  void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions11glGetFloatvEjPf(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_float) ;
  // proto:  void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
  fn _ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *const c_uint) ;
  // proto:  QOpenGLFunctions::GLenum QOpenGLFunctions::glGetError();
  fn _ZN16QOpenGLFunctions10glGetErrorEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
  fn _ZN16QOpenGLFunctions14glDetachShaderEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
  fn _ZN16QOpenGLFunctions16glVertexAttrib2fEjff(qthis: *mut c_void, arg0: c_uint, arg1: c_float, arg2: c_float) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
  fn _ZN16QOpenGLFunctions16glVertexAttrib1fEjf(qthis: *mut c_void, arg0: c_uint, arg1: c_float) ;
  // proto:  void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
  fn _ZN16QOpenGLFunctions12glGenBuffersEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint) ;
  // proto:  void QOpenGLFunctions::glClearStencil(GLint s);
  fn _ZN16QOpenGLFunctions14glClearStencilEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QOpenGLFunctions::glStencilMask(GLuint mask);
  fn _ZN16QOpenGLFunctions13glStencilMaskEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
  fn _ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char) ;
  // proto:  void QOpenGLFunctions::glReleaseShaderCompiler();
  fn _ZN16QOpenGLFunctions23glReleaseShaderCompilerEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glDepthMask(GLboolean flag);
  fn _ZN16QOpenGLFunctions11glDepthMaskEh(qthis: *mut c_void, arg0: c_uchar) ;
  // proto:  void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
  fn _ZN16QOpenGLFunctions11glUniform1fEif(qthis: *mut c_void, arg0: c_int, arg1: c_float) ;
  // proto:  void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
  fn _ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_uint) ;
  // proto:  void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
  fn _ZN16QOpenGLFunctions11glStencilOpEjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint) ;
  // proto:  void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
  fn _ZN16QOpenGLFunctions13glStencilFuncEjij(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint) ;
  // proto:  void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
  fn _ZN16QOpenGLFunctions14glAttachShaderEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glDeleteShader(GLuint shader);
  fn _ZN16QOpenGLFunctions14glDeleteShaderEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glCompileShader(GLuint shader);
  fn _ZN16QOpenGLFunctions15glCompileShaderEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
  fn _ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) ;
  // proto:  void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
  fn _ZN16QOpenGLFunctions11glColorMaskEhhhh(qthis: *mut c_void, arg0: c_uchar, arg1: c_uchar, arg2: c_uchar, arg3: c_uchar) ;
  // proto:  unsigned char QOpenGLFunctions::glIsEnabled(GLenum cap);
  fn _ZN16QOpenGLFunctions11glIsEnabledEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions18glBindRenderbufferEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *const c_float) ;
  // proto:  void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
  fn _ZN16QOpenGLFunctions11glBlendFuncEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint) ;
  // proto:  void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN16QOpenGLFunctions11glUniform3fEifff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN16QOpenGLFunctions16glVertexAttrib4fEjffff(qthis: *mut c_void, arg0: c_uint, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) ;
  // proto:  QOpenGLFunctions::GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
  fn _ZN16QOpenGLFunctions19glGetAttribLocationEjPKc(qthis: *mut c_void, arg0: c_uint, arg1: *const c_char) ;
  // proto:  void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform2ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_int) ;
  // proto:  void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
  fn _ZN16QOpenGLFunctions14glGetUniformivEjiPi(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
  fn _ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *const uint8_t) ;
  // proto:  void QOpenGLFunctions::glUseProgram(GLuint program);
  fn _ZN16QOpenGLFunctions12glUseProgramEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glDisable(GLenum cap);
  fn _ZN16QOpenGLFunctions9glDisableEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN16QOpenGLFunctions11glUniform4fEiffff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) ;
  // proto:  void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
  fn _ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: c_uint) ;
  // proto:  void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
  fn _ZN16QOpenGLFunctions16glCopyTexImage2DEjijiiiii(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int) ;
  // proto:  void QOpenGLFunctions::glLinkProgram(GLuint program);
  fn _ZN16QOpenGLFunctions13glLinkProgramEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
  fn _ZN16QOpenGLFunctions12glBufferDataEjiPKvj(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *const uint8_t, arg3: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
  fn _ZN16QOpenGLFunctions14glGetUniformfvEjiPf(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_float) ;
  // proto:  void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions21glRenderbufferStorageEjjii(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: c_int) ;
  // proto:  unsigned char QOpenGLFunctions::glIsShader(GLuint shader);
  fn _ZN16QOpenGLFunctions10glIsShaderEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::initializeOpenGLFunctions();
  fn _ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
  fn _ZN16QOpenGLFunctions11glUniform1iEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
  fn _ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) ;
  // proto:  void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
  fn _ZN16QOpenGLFunctions16glTexParameterfvEjjPKf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *const c_float) ;
  // proto:  void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *const c_float) ;
  // proto:  void QOpenGLFunctions::glValidateProgram(GLuint program);
  fn _ZN16QOpenGLFunctions17glValidateProgramEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::NewQOpenGLFunctions();
  fn _ZN16QOpenGLFunctionsC1Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLFunctions::glFlush();
  fn _ZN16QOpenGLFunctions7glFlushEv(qthis: *mut c_void) ;
  // proto:  QOpenGLFunctions::GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
  fn _ZN16QOpenGLFunctions24glCheckFramebufferStatusEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
  fn _ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions19glGetTexParameterivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int) ;
  // proto:  void QOpenGLFunctions::glClear(GLbitfield mask);
  fn _ZN16QOpenGLFunctions7glClearEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
  fn _ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_char) ;
  // proto:  void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
  fn _ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj(qthis: *mut c_void, arg0: c_uint) ;
}

// body block begin
// class sizeof(QOpenGLFunctions)=8
pub struct QOpenGLFunctions {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindAttribLocation<T: QOpenGLFunctions_glBindAttribLocation>(&mut self, value: T)  {
     value.glBindAttribLocation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindAttribLocation {
  fn glBindAttribLocation(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glBindAttribLocation for (u32, u32, &'a  String) {
  fn glBindAttribLocation(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *const c_char;
     unsafe {_ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenFramebuffers<T: QOpenGLFunctions_glGenFramebuffers>(&mut self, value: T)  {
     value.glGenFramebuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenFramebuffers {
  fn glGenFramebuffers(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenFramebuffers for (i32, &'a mut u32) {
  fn glGenFramebuffers(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGenFramebuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions17glGenFramebuffersEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3iv<T: QOpenGLFunctions_glUniform3iv>(&mut self, value: T)  {
     value.glUniform3iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3iv {
  fn glUniform3iv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3iv for (i32, i32, &'a  i32) {
  fn glUniform3iv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform3ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform3ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib4fv<T: QOpenGLFunctions_glVertexAttrib4fv>(&mut self, value: T)  {
     value.glVertexAttrib4fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib4fv {
  fn glVertexAttrib4fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib4fv for (u32, &'a  f32) {
  fn glVertexAttrib4fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsBuffer<T: QOpenGLFunctions_glIsBuffer>(&mut self, value: T) -> u8 {
    return value.glIsBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsBuffer {
  fn glIsBuffer(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsBuffer(GLuint buffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsBuffer for (u32) {
  fn glIsBuffer(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glIsBufferEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions10glIsBufferEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glLineWidth<T: QOpenGLFunctions_glLineWidth>(&mut self, value: T)  {
     value.glLineWidth(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glLineWidth {
  fn glLineWidth(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glLineWidth(GLfloat width);
impl<'a> /*trait*/ QOpenGLFunctions_glLineWidth for (f32) {
  fn glLineWidth(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glLineWidthEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glLineWidthEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCompressedTexImage2D<T: QOpenGLFunctions_glCompressedTexImage2D>(&mut self, value: T)  {
     value.glCompressedTexImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCompressedTexImage2D {
  fn glCompressedTexImage2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glCompressedTexImage2D for (u32, i32, u32, i32, i32, i32, i32, &'a  u8) {
  fn glCompressedTexImage2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthRangef<T: QOpenGLFunctions_glDepthRangef>(&mut self, value: T)  {
     value.glDepthRangef(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDepthRangef {
  fn glDepthRangef(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthRangef for (f32, f32) {
  fn glDepthRangef(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glDepthRangefEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions13glDepthRangefEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib1fv<T: QOpenGLFunctions_glVertexAttrib1fv>(&mut self, value: T)  {
     value.glVertexAttrib1fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib1fv {
  fn glVertexAttrib1fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib1fv for (u32, &'a  f32) {
  fn glVertexAttrib1fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameteriv<T: QOpenGLFunctions_glTexParameteriv>(&mut self, value: T)  {
     value.glTexParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameteriv {
  fn glTexParameteriv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameteriv for (u32, u32, &'a  i32) {
  fn glTexParameteriv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glTexParameterivEjjPKi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *const c_int;
     unsafe {_ZN16QOpenGLFunctions16glTexParameterivEjjPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexSubImage2D<T: QOpenGLFunctions_glTexSubImage2D>(&mut self, value: T)  {
     value.glTexSubImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexSubImage2D {
  fn glTexSubImage2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glTexSubImage2D for (u32, i32, i32, i32, i32, i32, u32, u32, &'a  u8) {
  fn glTexSubImage2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_uint;
    let arg7 = self.7  as c_uint;
    let arg8 = self.8  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteProgram<T: QOpenGLFunctions_glDeleteProgram>(&mut self, value: T)  {
     value.glDeleteProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteProgram {
  fn glDeleteProgram(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDeleteProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteProgram for (u32) {
  fn glDeleteProgram(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glDeleteProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glDeleteProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendEquationSeparate<T: QOpenGLFunctions_glBlendEquationSeparate>(&mut self, value: T)  {
     value.glBlendEquationSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendEquationSeparate {
  fn glBlendEquationSeparate(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendEquationSeparate for (u32, u32) {
  fn glBlendEquationSeparate(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions23glBlendEquationSeparateEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions23glBlendEquationSeparateEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilMaskSeparate<T: QOpenGLFunctions_glStencilMaskSeparate>(&mut self, value: T)  {
     value.glStencilMaskSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilMaskSeparate {
  fn glStencilMaskSeparate(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilMaskSeparate for (u32, u32) {
  fn glStencilMaskSeparate(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glStencilMaskSeparateEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions21glStencilMaskSeparateEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDrawArrays<T: QOpenGLFunctions_glDrawArrays>(&mut self, value: T)  {
     value.glDrawArrays(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDrawArrays {
  fn glDrawArrays(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
impl<'a> /*trait*/ QOpenGLFunctions_glDrawArrays for (u32, i32, i32) {
  fn glDrawArrays(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glDrawArraysEjii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QOpenGLFunctions12glDrawArraysEjii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFinish<T: QOpenGLFunctions_glFinish>(&mut self, value: T)  {
     value.glFinish(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFinish {
  fn glFinish(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glFinish();
impl<'a> /*trait*/ QOpenGLFunctions_glFinish for () {
  fn glFinish(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions8glFinishEv()};
     unsafe {_ZN16QOpenGLFunctions8glFinishEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribPointerv<T: QOpenGLFunctions_glGetVertexAttribPointerv>(&mut self, value: T)  {
     value.glGetVertexAttribPointerv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribPointerv {
  fn glGetVertexAttribPointerv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribPointerv for (u32, u32, &'a mut u8) {
  fn glGetVertexAttribPointerv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut uint8_t;
     unsafe {_ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glActiveTexture<T: QOpenGLFunctions_glActiveTexture>(&mut self, value: T)  {
     value.glActiveTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glActiveTexture {
  fn glActiveTexture(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glActiveTexture(GLenum texture);
impl<'a> /*trait*/ QOpenGLFunctions_glActiveTexture for (u32) {
  fn glActiveTexture(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glActiveTextureEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glActiveTextureEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFrontFace<T: QOpenGLFunctions_glFrontFace>(&mut self, value: T)  {
     value.glFrontFace(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFrontFace {
  fn glFrontFace(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glFrontFace(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glFrontFace for (u32) {
  fn glFrontFace(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glFrontFaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glFrontFaceEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetTexParameterfv<T: QOpenGLFunctions_glGetTexParameterfv>(&mut self, value: T)  {
     value.glGetTexParameterfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetTexParameterfv {
  fn glGetTexParameterfv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetTexParameterfv for (u32, u32, &'a mut f32) {
  fn glGetTexParameterfv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glPixelStorei<T: QOpenGLFunctions_glPixelStorei>(&mut self, value: T)  {
     value.glPixelStorei(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glPixelStorei {
  fn glPixelStorei(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
impl<'a> /*trait*/ QOpenGLFunctions_glPixelStorei for (u32, i32) {
  fn glPixelStorei(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glPixelStoreiEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QOpenGLFunctions13glPixelStoreiEji(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCullFace<T: QOpenGLFunctions_glCullFace>(&mut self, value: T)  {
     value.glCullFace(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCullFace {
  fn glCullFace(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glCullFace(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glCullFace for (u32) {
  fn glCullFace(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glCullFaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions10glCullFaceEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderiv<T: QOpenGLFunctions_glGetShaderiv>(&mut self, value: T)  {
     value.glGetShaderiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderiv {
  fn glGetShaderiv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderiv for (u32, u32, &'a mut i32) {
  fn glGetShaderiv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetShaderivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions13glGetShaderivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4i<T: QOpenGLFunctions_glUniform4i>(&mut self, value: T)  {
     value.glUniform4i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4i {
  fn glUniform4i(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4i for (i32, i32, i32, i32, i32) {
  fn glUniform4i(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform4iEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
     unsafe {_ZN16QOpenGLFunctions11glUniform4iEiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glReadPixels<T: QOpenGLFunctions_glReadPixels>(&mut self, value: T)  {
     value.glReadPixels(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glReadPixels {
  fn glReadPixels(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glReadPixels for (i32, i32, i32, i32, u32, u32, &'a mut u8) {
  fn glReadPixels(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_uint;
    let arg5 = self.5  as c_uint;
    let arg6 = self.6  as *mut uint8_t;
     unsafe {_ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameteri<T: QOpenGLFunctions_glTexParameteri>(&mut self, value: T)  {
     value.glTexParameteri(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameteri {
  fn glTexParameteri(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameteri for (u32, u32, i32) {
  fn glTexParameteri(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexParameteriEjji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QOpenGLFunctions15glTexParameteriEjji(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribiv<T: QOpenGLFunctions_glGetVertexAttribiv>(&mut self, value: T)  {
     value.glGetVertexAttribiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribiv {
  fn glGetVertexAttribiv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribiv for (u32, u32, &'a mut i32) {
  fn glGetVertexAttribiv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClearColor<T: QOpenGLFunctions_glClearColor>(&mut self, value: T)  {
     value.glClearColor(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClearColor {
  fn glClearColor(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glClearColor for (f32, f32, f32, f32) {
  fn glClearColor(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glClearColorEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN16QOpenGLFunctions12glClearColorEffff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClearDepthf<T: QOpenGLFunctions_glClearDepthf>(&mut self, value: T)  {
     value.glClearDepthf(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClearDepthf {
  fn glClearDepthf(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glClearDepthf(GLclampf depth);
impl<'a> /*trait*/ QOpenGLFunctions_glClearDepthf for (f32) {
  fn glClearDepthf(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glClearDepthfEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN16QOpenGLFunctions13glClearDepthfEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2i<T: QOpenGLFunctions_glUniform2i>(&mut self, value: T)  {
     value.glUniform2i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2i {
  fn glUniform2i(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2i for (i32, i32, i32) {
  fn glUniform2i(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform2iEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QOpenGLFunctions11glUniform2iEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenerateMipmap<T: QOpenGLFunctions_glGenerateMipmap>(&mut self, value: T)  {
     value.glGenerateMipmap(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenerateMipmap {
  fn glGenerateMipmap(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGenerateMipmap(GLenum target);
impl<'a> /*trait*/ QOpenGLFunctions_glGenerateMipmap for (u32) {
  fn glGenerateMipmap(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glGenerateMipmapEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions16glGenerateMipmapEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCompressedTexSubImage2D<T: QOpenGLFunctions_glCompressedTexSubImage2D>(&mut self, value: T)  {
     value.glCompressedTexSubImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCompressedTexSubImage2D {
  fn glCompressedTexSubImage2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glCompressedTexSubImage2D for (u32, i32, i32, i32, i32, i32, u32, i32, &'a  u8) {
  fn glCompressedTexSubImage2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_uint;
    let arg7 = self.7  as c_int;
    let arg8 = self.8  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3i<T: QOpenGLFunctions_glUniform3i>(&mut self, value: T)  {
     value.glUniform3i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3i {
  fn glUniform3i(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3i for (i32, i32, i32, i32) {
  fn glUniform3i(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform3iEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN16QOpenGLFunctions11glUniform3iEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenTextures<T: QOpenGLFunctions_glGenTextures>(&mut self, value: T)  {
     value.glGenTextures(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenTextures {
  fn glGenTextures(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
impl<'a> /*trait*/ QOpenGLFunctions_glGenTextures for (i32, &'a mut u32) {
  fn glGenTextures(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGenTexturesEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions13glGenTexturesEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderPrecisionFormat<T: QOpenGLFunctions_glGetShaderPrecisionFormat>(&mut self, value: T)  {
     value.glGetShaderPrecisionFormat(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderPrecisionFormat {
  fn glGetShaderPrecisionFormat(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderPrecisionFormat for (u32, u32, &'a mut i32, &'a mut i32) {
  fn glGetShaderPrecisionFormat(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn FreeQOpenGLFunctions<T: QOpenGLFunctions_FreeQOpenGLFunctions>(&mut self, value: T)  {
     value.FreeQOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_FreeQOpenGLFunctions {
  fn FreeQOpenGLFunctions(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::FreeQOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_FreeQOpenGLFunctions for () {
  fn FreeQOpenGLFunctions(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsD0Ev()};
     unsafe {_ZN16QOpenGLFunctionsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4fv<T: QOpenGLFunctions_glUniform4fv>(&mut self, value: T)  {
     value.glUniform4fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4fv {
  fn glUniform4fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4fv for (i32, i32, &'a  f32) {
  fn glUniform4fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform4fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform4fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetProgramiv<T: QOpenGLFunctions_glGetProgramiv>(&mut self, value: T)  {
     value.glGetProgramiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetProgramiv {
  fn glGetProgramiv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetProgramiv for (u32, u32, &'a mut i32) {
  fn glGetProgramiv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetProgramivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions14glGetProgramivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib2fv<T: QOpenGLFunctions_glVertexAttrib2fv>(&mut self, value: T)  {
     value.glVertexAttrib2fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib2fv {
  fn glVertexAttrib2fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib2fv for (u32, &'a  f32) {
  fn glVertexAttrib2fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetActiveAttrib<T: QOpenGLFunctions_glGetActiveAttrib>(&mut self, value: T)  {
     value.glGetActiveAttrib(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetActiveAttrib {
  fn glGetActiveAttrib(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetActiveAttrib for (u32, u32, i32, &'a mut i32, &'a mut i32, &'a mut u32, &'a mut String) {
  fn glGetActiveAttrib(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
    let arg5 = self.5  as *mut c_uint;
    let arg6 = self.6.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsRenderbuffer<T: QOpenGLFunctions_glIsRenderbuffer>(&mut self, value: T) -> u8 {
    return value.glIsRenderbuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsRenderbuffer {
  fn glIsRenderbuffer(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsRenderbuffer for (u32) {
  fn glIsRenderbuffer(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glIsRenderbufferEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions16glIsRenderbufferEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCopyTexSubImage2D<T: QOpenGLFunctions_glCopyTexSubImage2D>(&mut self, value: T)  {
     value.glCopyTexSubImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCopyTexSubImage2D {
  fn glCopyTexSubImage2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glCopyTexSubImage2D for (u32, i32, i32, i32, i32, i32, i32, i32) {
  fn glCopyTexSubImage2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glCopyTexSubImage2DEjiiiiiii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
     unsafe {_ZN16QOpenGLFunctions19glCopyTexSubImage2DEjiiiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glShaderSource<T: QOpenGLFunctions_glShaderSource>(&mut self, value: T)  {
     value.glShaderSource(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glShaderSource {
  fn glShaderSource(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
impl<'a> /*trait*/ QOpenGLFunctions_glShaderSource for (u32, i32, &'a  String, &'a  i32) {
  fn glShaderSource(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as *const c_int;
     unsafe {_ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribfv<T: QOpenGLFunctions_glGetVertexAttribfv>(&mut self, value: T)  {
     value.glGetVertexAttribfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribfv {
  fn glGetVertexAttribfv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribfv for (u32, u32, &'a mut f32) {
  fn glGetVertexAttribfv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthFunc<T: QOpenGLFunctions_glDepthFunc>(&mut self, value: T)  {
     value.glDepthFunc(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDepthFunc {
  fn glDepthFunc(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDepthFunc(GLenum func);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthFunc for (u32) {
  fn glDepthFunc(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glDepthFuncEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glDepthFuncEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexImage2D<T: QOpenGLFunctions_glTexImage2D>(&mut self, value: T)  {
     value.glTexImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexImage2D {
  fn glTexImage2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glTexImage2D for (u32, i32, i32, i32, i32, i32, u32, u32, &'a  u8) {
  fn glTexImage2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_uint;
    let arg7 = self.7  as c_uint;
    let arg8 = self.8  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteFramebuffers<T: QOpenGLFunctions_glDeleteFramebuffers>(&mut self, value: T)  {
     value.glDeleteFramebuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteFramebuffers {
  fn glDeleteFramebuffers(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteFramebuffers for (i32, &'a  u32) {
  fn glDeleteFramebuffers(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
     unsafe {_ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glHint<T: QOpenGLFunctions_glHint>(&mut self, value: T)  {
     value.glHint(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glHint {
  fn glHint(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glHint for (u32, u32) {
  fn glHint(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions6glHintEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions6glHintEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformLocation<T: QOpenGLFunctions_glGetUniformLocation>(&mut self, value: T)  {
     value.glGetUniformLocation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformLocation {
  fn glGetUniformLocation(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  QOpenGLFunctions::GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformLocation for (u32, &'a  String) {
  fn glGetUniformLocation(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glGetUniformLocationEjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN16QOpenGLFunctions20glGetUniformLocationEjPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsFramebuffer<T: QOpenGLFunctions_glIsFramebuffer>(&mut self, value: T) -> u8 {
    return value.glIsFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsFramebuffer {
  fn glIsFramebuffer(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsFramebuffer for (u32) {
  fn glIsFramebuffer(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glIsFramebufferEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions15glIsFramebufferEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1fv<T: QOpenGLFunctions_glUniform1fv>(&mut self, value: T)  {
     value.glUniform1fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1fv {
  fn glUniform1fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1fv for (i32, i32, &'a  f32) {
  fn glUniform1fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform1fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform1fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetString<T: QOpenGLFunctions_glGetString>(&mut self, value: T) -> String {
    return value.glGetString(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetString {
  fn glGetString(self, rsthis: &mut QOpenGLFunctions) -> String;
}

// proto:  const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetString for (u32) {
  fn glGetString(self, rsthis: &mut QOpenGLFunctions) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glGetStringEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glGetStringEj(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix2fv<T: QOpenGLFunctions_glUniformMatrix2fv>(&mut self, value: T)  {
     value.glUniformMatrix2fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix2fv {
  fn glUniformMatrix2fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix2fv for (i32, i32, u8, &'a  f32) {
  fn glUniformMatrix2fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn NewQOpenGLFunctions<T: QOpenGLFunctions_NewQOpenGLFunctions>(value: T) -> QOpenGLFunctions {
    let rsthis = value.NewQOpenGLFunctions();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_NewQOpenGLFunctions {
  fn NewQOpenGLFunctions(self) -> QOpenGLFunctions;
}

// proto: void QOpenGLFunctions::NewQOpenGLFunctions(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_NewQOpenGLFunctions for (&'a mut QOpenGLContext) {
  fn NewQOpenGLFunctions(self) -> QOpenGLFunctions {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QOpenGLFunctionsC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix3fv<T: QOpenGLFunctions_glUniformMatrix3fv>(&mut self, value: T)  {
     value.glUniformMatrix3fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix3fv {
  fn glUniformMatrix3fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix3fv for (i32, i32, u8, &'a  f32) {
  fn glUniformMatrix3fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindBuffer<T: QOpenGLFunctions_glBindBuffer>(&mut self, value: T)  {
     value.glBindBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindBuffer {
  fn glBindBuffer(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindBuffer for (u32, u32) {
  fn glBindBuffer(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBindBufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions12glBindBufferEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2f<T: QOpenGLFunctions_glUniform2f>(&mut self, value: T)  {
     value.glUniform2f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2f {
  fn glUniform2f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2f for (i32, f32, f32) {
  fn glUniform2f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform2fEiff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glUniform2fEiff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3fv<T: QOpenGLFunctions_glUniform3fv>(&mut self, value: T)  {
     value.glUniform3fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3fv {
  fn glUniform3fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3fv for (i32, i32, &'a  f32) {
  fn glUniform3fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform3fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform3fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2fv<T: QOpenGLFunctions_glUniform2fv>(&mut self, value: T)  {
     value.glUniform2fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2fv {
  fn glUniform2fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2fv for (i32, i32, &'a  f32) {
  fn glUniform2fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform2fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform2fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetRenderbufferParameteriv<T: QOpenGLFunctions_glGetRenderbufferParameteriv>(&mut self, value: T)  {
     value.glGetRenderbufferParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetRenderbufferParameteriv {
  fn glGetRenderbufferParameteriv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetRenderbufferParameteriv for (u32, u32, &'a mut i32) {
  fn glGetRenderbufferParameteriv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetBufferParameteriv<T: QOpenGLFunctions_glGetBufferParameteriv>(&mut self, value: T)  {
     value.glGetBufferParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetBufferParameteriv {
  fn glGetBufferParameteriv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetBufferParameteriv for (u32, u32, &'a mut i32) {
  fn glGetBufferParameteriv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1iv<T: QOpenGLFunctions_glUniform1iv>(&mut self, value: T)  {
     value.glUniform1iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1iv {
  fn glUniform1iv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1iv for (i32, i32, &'a  i32) {
  fn glUniform1iv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform1ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform1ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendColor<T: QOpenGLFunctions_glBlendColor>(&mut self, value: T)  {
     value.glBlendColor(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendColor {
  fn glBlendColor(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendColor for (f32, f32, f32, f32) {
  fn glBlendColor(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBlendColorEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN16QOpenGLFunctions12glBlendColorEffff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDrawElements<T: QOpenGLFunctions_glDrawElements>(&mut self, value: T)  {
     value.glDrawElements(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDrawElements {
  fn glDrawElements(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
impl<'a> /*trait*/ QOpenGLFunctions_glDrawElements for (u32, i32, u32, &'a  u8) {
  fn glDrawElements(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDrawElementsEjijPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions14glDrawElementsEjijPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindFramebuffer<T: QOpenGLFunctions_glBindFramebuffer>(&mut self, value: T)  {
     value.glBindFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindFramebuffer {
  fn glBindFramebuffer(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindFramebuffer for (u32, u32) {
  fn glBindFramebuffer(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glBindFramebufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions17glBindFramebufferEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsProgram<T: QOpenGLFunctions_glIsProgram>(&mut self, value: T) -> u8 {
    return value.glIsProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsProgram {
  fn glIsProgram(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glIsProgram for (u32) {
  fn glIsProgram(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsProgramEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glIsProgramEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendEquation<T: QOpenGLFunctions_glBlendEquation>(&mut self, value: T)  {
     value.glBlendEquation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendEquation {
  fn glBlendEquation(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBlendEquation(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendEquation for (u32) {
  fn glBlendEquation(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glBlendEquationEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glBlendEquationEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glShaderBinary<T: QOpenGLFunctions_glShaderBinary>(&mut self, value: T)  {
     value.glShaderBinary(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glShaderBinary {
  fn glShaderBinary(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
impl<'a> /*trait*/ QOpenGLFunctions_glShaderBinary for (i32, &'a  u32, u32, &'a  u8, i32) {
  fn glShaderBinary(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *const uint8_t;
    let arg4 = self.4  as c_int;
     unsafe {_ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetProgramInfoLog<T: QOpenGLFunctions_glGetProgramInfoLog>(&mut self, value: T)  {
     value.glGetProgramInfoLog(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetProgramInfoLog {
  fn glGetProgramInfoLog(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
impl<'a> /*trait*/ QOpenGLFunctions_glGetProgramInfoLog for (u32, i32, &'a mut i32, &'a mut String) {
  fn glGetProgramInfoLog(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteBuffers<T: QOpenGLFunctions_glDeleteBuffers>(&mut self, value: T)  {
     value.glDeleteBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteBuffers {
  fn glDeleteBuffers(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteBuffers for (i32, &'a  u32) {
  fn glDeleteBuffers(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glDeleteBuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
     unsafe {_ZN16QOpenGLFunctions15glDeleteBuffersEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glScissor<T: QOpenGLFunctions_glScissor>(&mut self, value: T)  {
     value.glScissor(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glScissor {
  fn glScissor(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glScissor for (i32, i32, i32, i32) {
  fn glScissor(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions9glScissorEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN16QOpenGLFunctions9glScissorEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenRenderbuffers<T: QOpenGLFunctions_glGenRenderbuffers>(&mut self, value: T)  {
     value.glGenRenderbuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenRenderbuffers {
  fn glGenRenderbuffers(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenRenderbuffers for (i32, &'a mut u32) {
  fn glGenRenderbuffers(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGenRenderbuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions18glGenRenderbuffersEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib3f<T: QOpenGLFunctions_glVertexAttrib3f>(&mut self, value: T)  {
     value.glVertexAttrib3f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib3f {
  fn glVertexAttrib3f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib3f for (u32, f32, f32, f32) {
  fn glVertexAttrib3f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib3fEjfff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN16QOpenGLFunctions16glVertexAttrib3fEjfff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCreateProgram<T: QOpenGLFunctions_glCreateProgram>(&mut self, value: T)  {
     value.glCreateProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCreateProgram {
  fn glCreateProgram(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateProgram();
impl<'a> /*trait*/ QOpenGLFunctions_glCreateProgram for () {
  fn glCreateProgram(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glCreateProgramEv()};
     unsafe {_ZN16QOpenGLFunctions15glCreateProgramEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4iv<T: QOpenGLFunctions_glUniform4iv>(&mut self, value: T)  {
     value.glUniform4iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4iv {
  fn glUniform4iv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4iv for (i32, i32, &'a  i32) {
  fn glUniform4iv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform4ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform4ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glEnable<T: QOpenGLFunctions_glEnable>(&mut self, value: T)  {
     value.glEnable(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glEnable {
  fn glEnable(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glEnable(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glEnable for (u32) {
  fn glEnable(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions8glEnableEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions8glEnableEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindTexture<T: QOpenGLFunctions_glBindTexture>(&mut self, value: T)  {
     value.glBindTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindTexture {
  fn glBindTexture(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
impl<'a> /*trait*/ QOpenGLFunctions_glBindTexture for (u32, u32) {
  fn glBindTexture(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glBindTextureEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glBindTextureEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameterf<T: QOpenGLFunctions_glTexParameterf>(&mut self, value: T)  {
     value.glTexParameterf(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameterf {
  fn glTexParameterf(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameterf for (u32, u32, f32) {
  fn glTexParameterf(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexParameterfEjjf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_float;
     unsafe {_ZN16QOpenGLFunctions15glTexParameterfEjjf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glViewport<T: QOpenGLFunctions_glViewport>(&mut self, value: T)  {
     value.glViewport(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glViewport {
  fn glViewport(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glViewport for (i32, i32, i32, i32) {
  fn glViewport(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glViewportEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN16QOpenGLFunctions10glViewportEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glSampleCoverage<T: QOpenGLFunctions_glSampleCoverage>(&mut self, value: T)  {
     value.glSampleCoverage(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glSampleCoverage {
  fn glSampleCoverage(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
impl<'a> /*trait*/ QOpenGLFunctions_glSampleCoverage for (f32, u8) {
  fn glSampleCoverage(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glSampleCoverageEfh()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_uchar;
     unsafe {_ZN16QOpenGLFunctions16glSampleCoverageEfh(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFramebufferTexture2D<T: QOpenGLFunctions_glFramebufferTexture2D>(&mut self, value: T)  {
     value.glFramebufferTexture2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFramebufferTexture2D {
  fn glFramebufferTexture2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
impl<'a> /*trait*/ QOpenGLFunctions_glFramebufferTexture2D for (u32, u32, u32, u32, i32) {
  fn glFramebufferTexture2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    let arg4 = self.4  as c_int;
     unsafe {_ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttribPointer<T: QOpenGLFunctions_glVertexAttribPointer>(&mut self, value: T)  {
     value.glVertexAttribPointer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttribPointer {
  fn glVertexAttribPointer(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttribPointer for (u32, i32, u32, u8, i32, &'a  u8) {
  fn glVertexAttribPointer(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uchar;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glPolygonOffset<T: QOpenGLFunctions_glPolygonOffset>(&mut self, value: T)  {
     value.glPolygonOffset(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glPolygonOffset {
  fn glPolygonOffset(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
impl<'a> /*trait*/ QOpenGLFunctions_glPolygonOffset for (f32, f32) {
  fn glPolygonOffset(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glPolygonOffsetEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions15glPolygonOffsetEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCreateShader<T: QOpenGLFunctions_glCreateShader>(&mut self, value: T)  {
     value.glCreateShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCreateShader {
  fn glCreateShader(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateShader(GLenum type);
impl<'a> /*trait*/ QOpenGLFunctions_glCreateShader for (u32) {
  fn glCreateShader(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glCreateShaderEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glCreateShaderEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderSource<T: QOpenGLFunctions_glGetShaderSource>(&mut self, value: T)  {
     value.glGetShaderSource(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderSource {
  fn glGetShaderSource(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderSource for (u32, i32, &'a mut i32, &'a mut String) {
  fn glGetShaderSource(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsTexture<T: QOpenGLFunctions_glIsTexture>(&mut self, value: T) -> u8 {
    return value.glIsTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsTexture {
  fn glIsTexture(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsTexture(GLuint texture);
impl<'a> /*trait*/ QOpenGLFunctions_glIsTexture for (u32) {
  fn glIsTexture(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsTextureEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glIsTextureEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteTextures<T: QOpenGLFunctions_glDeleteTextures>(&mut self, value: T)  {
     value.glDeleteTextures(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteTextures {
  fn glDeleteTextures(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteTextures for (i32, &'a  u32) {
  fn glDeleteTextures(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glDeleteTexturesEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
     unsafe {_ZN16QOpenGLFunctions16glDeleteTexturesEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetIntegerv<T: QOpenGLFunctions_glGetIntegerv>(&mut self, value: T)  {
     value.glGetIntegerv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetIntegerv {
  fn glGetIntegerv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetIntegerv for (u32, &'a mut i32) {
  fn glGetIntegerv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetIntegervEjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions13glGetIntegervEjPi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetBooleanv<T: QOpenGLFunctions_glGetBooleanv>(&mut self, value: T)  {
     value.glGetBooleanv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetBooleanv {
  fn glGetBooleanv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetBooleanv for (u32, &'a mut String) {
  fn glGetBooleanv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetBooleanvEjPh()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_uchar;
     unsafe {_ZN16QOpenGLFunctions13glGetBooleanvEjPh(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetFloatv<T: QOpenGLFunctions_glGetFloatv>(&mut self, value: T)  {
     value.glGetFloatv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetFloatv {
  fn glGetFloatv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetFloatv for (u32, &'a mut f32) {
  fn glGetFloatv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glGetFloatvEjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions11glGetFloatvEjPf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteRenderbuffers<T: QOpenGLFunctions_glDeleteRenderbuffers>(&mut self, value: T)  {
     value.glDeleteRenderbuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteRenderbuffers {
  fn glDeleteRenderbuffers(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteRenderbuffers for (i32, &'a  u32) {
  fn glDeleteRenderbuffers(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
     unsafe {_ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetError<T: QOpenGLFunctions_glGetError>(&mut self, value: T)  {
     value.glGetError(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetError {
  fn glGetError(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  QOpenGLFunctions::GLenum QOpenGLFunctions::glGetError();
impl<'a> /*trait*/ QOpenGLFunctions_glGetError for () {
  fn glGetError(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glGetErrorEv()};
     unsafe {_ZN16QOpenGLFunctions10glGetErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDetachShader<T: QOpenGLFunctions_glDetachShader>(&mut self, value: T)  {
     value.glDetachShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDetachShader {
  fn glDetachShader(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glDetachShader for (u32, u32) {
  fn glDetachShader(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDetachShaderEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glDetachShaderEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib2f<T: QOpenGLFunctions_glVertexAttrib2f>(&mut self, value: T)  {
     value.glVertexAttrib2f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib2f {
  fn glVertexAttrib2f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib2f for (u32, f32, f32) {
  fn glVertexAttrib2f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib2fEjff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN16QOpenGLFunctions16glVertexAttrib2fEjff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib1f<T: QOpenGLFunctions_glVertexAttrib1f>(&mut self, value: T)  {
     value.glVertexAttrib1f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib1f {
  fn glVertexAttrib1f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib1f for (u32, f32) {
  fn glVertexAttrib1f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib1fEjf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions16glVertexAttrib1fEjf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenBuffers<T: QOpenGLFunctions_glGenBuffers>(&mut self, value: T)  {
     value.glGenBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenBuffers {
  fn glGenBuffers(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenBuffers for (i32, &'a mut u32) {
  fn glGenBuffers(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glGenBuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions12glGenBuffersEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClearStencil<T: QOpenGLFunctions_glClearStencil>(&mut self, value: T)  {
     value.glClearStencil(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClearStencil {
  fn glClearStencil(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glClearStencil(GLint s);
impl<'a> /*trait*/ QOpenGLFunctions_glClearStencil for (i32) {
  fn glClearStencil(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glClearStencilEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QOpenGLFunctions14glClearStencilEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilMask<T: QOpenGLFunctions_glStencilMask>(&mut self, value: T)  {
     value.glStencilMask(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilMask {
  fn glStencilMask(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glStencilMask(GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilMask for (u32) {
  fn glStencilMask(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glStencilMaskEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glStencilMaskEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderInfoLog<T: QOpenGLFunctions_glGetShaderInfoLog>(&mut self, value: T)  {
     value.glGetShaderInfoLog(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderInfoLog {
  fn glGetShaderInfoLog(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderInfoLog for (u32, i32, &'a mut i32, &'a mut String) {
  fn glGetShaderInfoLog(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glReleaseShaderCompiler<T: QOpenGLFunctions_glReleaseShaderCompiler>(&mut self, value: T)  {
     value.glReleaseShaderCompiler(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glReleaseShaderCompiler {
  fn glReleaseShaderCompiler(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glReleaseShaderCompiler();
impl<'a> /*trait*/ QOpenGLFunctions_glReleaseShaderCompiler for () {
  fn glReleaseShaderCompiler(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions23glReleaseShaderCompilerEv()};
     unsafe {_ZN16QOpenGLFunctions23glReleaseShaderCompilerEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthMask<T: QOpenGLFunctions_glDepthMask>(&mut self, value: T)  {
     value.glDepthMask(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDepthMask {
  fn glDepthMask(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDepthMask(GLboolean flag);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthMask for (u8) {
  fn glDepthMask(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glDepthMaskEh()};
    let arg0 = self  as c_uchar;
     unsafe {_ZN16QOpenGLFunctions11glDepthMaskEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetFramebufferAttachmentParameteriv<T: QOpenGLFunctions_glGetFramebufferAttachmentParameteriv>(&mut self, value: T)  {
     value.glGetFramebufferAttachmentParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetFramebufferAttachmentParameteriv {
  fn glGetFramebufferAttachmentParameteriv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetFramebufferAttachmentParameteriv for (u32, u32, u32, &'a mut i32) {
  fn glGetFramebufferAttachmentParameteriv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1f<T: QOpenGLFunctions_glUniform1f>(&mut self, value: T)  {
     value.glUniform1f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1f {
  fn glUniform1f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1f for (i32, f32) {
  fn glUniform1f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform1fEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glUniform1fEif(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetAttachedShaders<T: QOpenGLFunctions_glGetAttachedShaders>(&mut self, value: T)  {
     value.glGetAttachedShaders(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetAttachedShaders {
  fn glGetAttachedShaders(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
impl<'a> /*trait*/ QOpenGLFunctions_glGetAttachedShaders for (u32, i32, &'a mut i32, &'a mut u32) {
  fn glGetAttachedShaders(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilOp<T: QOpenGLFunctions_glStencilOp>(&mut self, value: T)  {
     value.glStencilOp(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilOp {
  fn glStencilOp(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilOp for (u32, u32, u32) {
  fn glStencilOp(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glStencilOpEjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glStencilOpEjjj(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilFunc<T: QOpenGLFunctions_glStencilFunc>(&mut self, value: T)  {
     value.glStencilFunc(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilFunc {
  fn glStencilFunc(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilFunc for (u32, i32, u32) {
  fn glStencilFunc(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glStencilFuncEjij()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glStencilFuncEjij(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glAttachShader<T: QOpenGLFunctions_glAttachShader>(&mut self, value: T)  {
     value.glAttachShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glAttachShader {
  fn glAttachShader(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glAttachShader for (u32, u32) {
  fn glAttachShader(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glAttachShaderEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glAttachShaderEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteShader<T: QOpenGLFunctions_glDeleteShader>(&mut self, value: T)  {
     value.glDeleteShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteShader {
  fn glDeleteShader(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDeleteShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteShader for (u32) {
  fn glDeleteShader(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDeleteShaderEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glDeleteShaderEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCompileShader<T: QOpenGLFunctions_glCompileShader>(&mut self, value: T)  {
     value.glCompileShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCompileShader {
  fn glCompileShader(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glCompileShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glCompileShader for (u32) {
  fn glCompileShader(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glCompileShaderEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glCompileShaderEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glEnableVertexAttribArray<T: QOpenGLFunctions_glEnableVertexAttribArray>(&mut self, value: T)  {
     value.glEnableVertexAttribArray(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glEnableVertexAttribArray {
  fn glEnableVertexAttribArray(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
impl<'a> /*trait*/ QOpenGLFunctions_glEnableVertexAttribArray for (u32) {
  fn glEnableVertexAttribArray(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFramebufferRenderbuffer<T: QOpenGLFunctions_glFramebufferRenderbuffer>(&mut self, value: T)  {
     value.glFramebufferRenderbuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFramebufferRenderbuffer {
  fn glFramebufferRenderbuffer(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glFramebufferRenderbuffer for (u32, u32, u32, u32) {
  fn glFramebufferRenderbuffer(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glColorMask<T: QOpenGLFunctions_glColorMask>(&mut self, value: T)  {
     value.glColorMask(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glColorMask {
  fn glColorMask(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glColorMask for (u8, u8, u8, u8) {
  fn glColorMask(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glColorMaskEhhhh()};
    let arg0 = self.0  as c_uchar;
    let arg1 = self.1  as c_uchar;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as c_uchar;
     unsafe {_ZN16QOpenGLFunctions11glColorMaskEhhhh(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsEnabled<T: QOpenGLFunctions_glIsEnabled>(&mut self, value: T) -> u8 {
    return value.glIsEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsEnabled {
  fn glIsEnabled(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsEnabled(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glIsEnabled for (u32) {
  fn glIsEnabled(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsEnabledEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glIsEnabledEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindRenderbuffer<T: QOpenGLFunctions_glBindRenderbuffer>(&mut self, value: T)  {
     value.glBindRenderbuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindRenderbuffer {
  fn glBindRenderbuffer(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindRenderbuffer for (u32, u32) {
  fn glBindRenderbuffer(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glBindRenderbufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions18glBindRenderbufferEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib3fv<T: QOpenGLFunctions_glVertexAttrib3fv>(&mut self, value: T)  {
     value.glVertexAttrib3fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib3fv {
  fn glVertexAttrib3fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib3fv for (u32, &'a  f32) {
  fn glVertexAttrib3fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendFunc<T: QOpenGLFunctions_glBlendFunc>(&mut self, value: T)  {
     value.glBlendFunc(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendFunc {
  fn glBlendFunc(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendFunc for (u32, u32) {
  fn glBlendFunc(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glBlendFuncEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glBlendFuncEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3f<T: QOpenGLFunctions_glUniform3f>(&mut self, value: T)  {
     value.glUniform3f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3f {
  fn glUniform3f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3f for (i32, f32, f32, f32) {
  fn glUniform3f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform3fEifff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glUniform3fEifff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib4f<T: QOpenGLFunctions_glVertexAttrib4f>(&mut self, value: T)  {
     value.glVertexAttrib4f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib4f {
  fn glVertexAttrib4f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib4f for (u32, f32, f32, f32, f32) {
  fn glVertexAttrib4f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib4fEjffff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN16QOpenGLFunctions16glVertexAttrib4fEjffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetAttribLocation<T: QOpenGLFunctions_glGetAttribLocation>(&mut self, value: T)  {
     value.glGetAttribLocation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetAttribLocation {
  fn glGetAttribLocation(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  QOpenGLFunctions::GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetAttribLocation for (u32, &'a  String) {
  fn glGetAttribLocation(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetAttribLocationEjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN16QOpenGLFunctions19glGetAttribLocationEjPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2iv<T: QOpenGLFunctions_glUniform2iv>(&mut self, value: T)  {
     value.glUniform2iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2iv {
  fn glUniform2iv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2iv for (i32, i32, &'a  i32) {
  fn glUniform2iv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform2ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform2ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformiv<T: QOpenGLFunctions_glGetUniformiv>(&mut self, value: T)  {
     value.glGetUniformiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformiv {
  fn glGetUniformiv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformiv for (u32, i32, &'a mut i32) {
  fn glGetUniformiv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetUniformivEjiPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions14glGetUniformivEjiPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBufferSubData<T: QOpenGLFunctions_glBufferSubData>(&mut self, value: T)  {
     value.glBufferSubData(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBufferSubData {
  fn glBufferSubData(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glBufferSubData for (u32, i32, i32, &'a  u8) {
  fn glBufferSubData(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *const uint8_t;
     unsafe {_ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUseProgram<T: QOpenGLFunctions_glUseProgram>(&mut self, value: T)  {
     value.glUseProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUseProgram {
  fn glUseProgram(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUseProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glUseProgram for (u32) {
  fn glUseProgram(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUseProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions12glUseProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDisable<T: QOpenGLFunctions_glDisable>(&mut self, value: T)  {
     value.glDisable(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDisable {
  fn glDisable(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDisable(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glDisable for (u32) {
  fn glDisable(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions9glDisableEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions9glDisableEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4f<T: QOpenGLFunctions_glUniform4f>(&mut self, value: T)  {
     value.glUniform4f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4f {
  fn glUniform4f(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4f for (i32, f32, f32, f32, f32) {
  fn glUniform4f(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform4fEiffff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glUniform4fEiffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilFuncSeparate<T: QOpenGLFunctions_glStencilFuncSeparate>(&mut self, value: T)  {
     value.glStencilFuncSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilFuncSeparate {
  fn glStencilFuncSeparate(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilFuncSeparate for (u32, u32, i32, u32) {
  fn glStencilFuncSeparate(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCopyTexImage2D<T: QOpenGLFunctions_glCopyTexImage2D>(&mut self, value: T)  {
     value.glCopyTexImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCopyTexImage2D {
  fn glCopyTexImage2D(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
impl<'a> /*trait*/ QOpenGLFunctions_glCopyTexImage2D for (u32, i32, u32, i32, i32, i32, i32, i32) {
  fn glCopyTexImage2D(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glCopyTexImage2DEjijiiiii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
     unsafe {_ZN16QOpenGLFunctions16glCopyTexImage2DEjijiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glLinkProgram<T: QOpenGLFunctions_glLinkProgram>(&mut self, value: T)  {
     value.glLinkProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glLinkProgram {
  fn glLinkProgram(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glLinkProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glLinkProgram for (u32) {
  fn glLinkProgram(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glLinkProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glLinkProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBufferData<T: QOpenGLFunctions_glBufferData>(&mut self, value: T)  {
     value.glBufferData(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBufferData {
  fn glBufferData(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
impl<'a> /*trait*/ QOpenGLFunctions_glBufferData for (u32, i32, &'a  u8, u32) {
  fn glBufferData(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBufferDataEjiPKvj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const uint8_t;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN16QOpenGLFunctions12glBufferDataEjiPKvj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformfv<T: QOpenGLFunctions_glGetUniformfv>(&mut self, value: T)  {
     value.glGetUniformfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformfv {
  fn glGetUniformfv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformfv for (u32, i32, &'a mut f32) {
  fn glGetUniformfv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetUniformfvEjiPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions14glGetUniformfvEjiPf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glRenderbufferStorage<T: QOpenGLFunctions_glRenderbufferStorage>(&mut self, value: T)  {
     value.glRenderbufferStorage(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glRenderbufferStorage {
  fn glRenderbufferStorage(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glRenderbufferStorage for (u32, u32, i32, i32) {
  fn glRenderbufferStorage(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glRenderbufferStorageEjjii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN16QOpenGLFunctions21glRenderbufferStorageEjjii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsShader<T: QOpenGLFunctions_glIsShader>(&mut self, value: T) -> u8 {
    return value.glIsShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsShader {
  fn glIsShader(self, rsthis: &mut QOpenGLFunctions) -> u8;
}

// proto:  unsigned char QOpenGLFunctions::glIsShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glIsShader for (u32) {
  fn glIsShader(self, rsthis: &mut QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glIsShaderEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions10glIsShaderEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn initializeOpenGLFunctions<T: QOpenGLFunctions_initializeOpenGLFunctions>(&mut self, value: T)  {
     value.initializeOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_initializeOpenGLFunctions {
  fn initializeOpenGLFunctions(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::initializeOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_initializeOpenGLFunctions for () {
  fn initializeOpenGLFunctions(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv()};
     unsafe {_ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1i<T: QOpenGLFunctions_glUniform1i>(&mut self, value: T)  {
     value.glUniform1i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1i {
  fn glUniform1i(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1i for (i32, i32) {
  fn glUniform1i(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform1iEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QOpenGLFunctions11glUniform1iEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendFuncSeparate<T: QOpenGLFunctions_glBlendFuncSeparate>(&mut self, value: T)  {
     value.glBlendFuncSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendFuncSeparate {
  fn glBlendFuncSeparate(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendFuncSeparate for (u32, u32, u32, u32) {
  fn glBlendFuncSeparate(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameterfv<T: QOpenGLFunctions_glTexParameterfv>(&mut self, value: T)  {
     value.glTexParameterfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameterfv {
  fn glTexParameterfv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameterfv for (u32, u32, &'a  f32) {
  fn glTexParameterfv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glTexParameterfvEjjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions16glTexParameterfvEjjPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix4fv<T: QOpenGLFunctions_glUniformMatrix4fv>(&mut self, value: T)  {
     value.glUniformMatrix4fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix4fv {
  fn glUniformMatrix4fv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix4fv for (i32, i32, u8, &'a  f32) {
  fn glUniformMatrix4fv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as *const c_float;
     unsafe {_ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glValidateProgram<T: QOpenGLFunctions_glValidateProgram>(&mut self, value: T)  {
     value.glValidateProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glValidateProgram {
  fn glValidateProgram(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glValidateProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glValidateProgram for (u32) {
  fn glValidateProgram(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glValidateProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions17glValidateProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QOpenGLFunctions::NewQOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_NewQOpenGLFunctions for () {
  fn NewQOpenGLFunctions(self) -> QOpenGLFunctions {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsC1Ev()};
    unsafe {_ZN16QOpenGLFunctionsC1Ev(qthis)};
    let rsthis = QOpenGLFunctions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFlush<T: QOpenGLFunctions_glFlush>(&mut self, value: T)  {
     value.glFlush(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFlush {
  fn glFlush(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glFlush();
impl<'a> /*trait*/ QOpenGLFunctions_glFlush for () {
  fn glFlush(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions7glFlushEv()};
     unsafe {_ZN16QOpenGLFunctions7glFlushEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCheckFramebufferStatus<T: QOpenGLFunctions_glCheckFramebufferStatus>(&mut self, value: T)  {
     value.glCheckFramebufferStatus(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCheckFramebufferStatus {
  fn glCheckFramebufferStatus(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  QOpenGLFunctions::GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
impl<'a> /*trait*/ QOpenGLFunctions_glCheckFramebufferStatus for (u32) {
  fn glCheckFramebufferStatus(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions24glCheckFramebufferStatusEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions24glCheckFramebufferStatusEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilOpSeparate<T: QOpenGLFunctions_glStencilOpSeparate>(&mut self, value: T)  {
     value.glStencilOpSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilOpSeparate {
  fn glStencilOpSeparate(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilOpSeparate for (u32, u32, u32, u32) {
  fn glStencilOpSeparate(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetTexParameteriv<T: QOpenGLFunctions_glGetTexParameteriv>(&mut self, value: T)  {
     value.glGetTexParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetTexParameteriv {
  fn glGetTexParameteriv(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetTexParameteriv for (u32, u32, &'a mut i32) {
  fn glGetTexParameteriv(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetTexParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions19glGetTexParameterivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClear<T: QOpenGLFunctions_glClear>(&mut self, value: T)  {
     value.glClear(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClear {
  fn glClear(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glClear(GLbitfield mask);
impl<'a> /*trait*/ QOpenGLFunctions_glClear for (u32) {
  fn glClear(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions7glClearEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions7glClearEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetActiveUniform<T: QOpenGLFunctions_glGetActiveUniform>(&mut self, value: T)  {
     value.glGetActiveUniform(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetActiveUniform {
  fn glGetActiveUniform(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetActiveUniform for (u32, u32, i32, &'a mut i32, &'a mut i32, &'a mut u32, &'a mut String) {
  fn glGetActiveUniform(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
    let arg5 = self.5  as *mut c_uint;
    let arg6 = self.6.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDisableVertexAttribArray<T: QOpenGLFunctions_glDisableVertexAttribArray>(&mut self, value: T)  {
     value.glDisableVertexAttribArray(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDisableVertexAttribArray {
  fn glDisableVertexAttribArray(self, rsthis: &mut QOpenGLFunctions) ;
}

// proto:  void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
impl<'a> /*trait*/ QOpenGLFunctions_glDisableVertexAttribArray for (u32) {
  fn glDisableVertexAttribArray(self, rsthis: &mut QOpenGLFunctions)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

