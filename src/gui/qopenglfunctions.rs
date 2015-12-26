// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qopenglfunctions.h
// dst-file: /src/gui/qopenglfunctions.rs
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
use std::ops::Deref;
use super::qopenglcontext::QOpenGLContext; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLFunctionsPrivate_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctionsPrivate::QOpenGLFunctionsPrivate(QOpenGLContext * ctx);
  fn dector_ZN23QOpenGLFunctionsPrivateC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN23QOpenGLFunctionsPrivateC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  fn QOpenGLFunctions_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
  fn _ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_char);
  // proto:  void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
  fn _ZN16QOpenGLFunctions17glGenFramebuffersEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform3ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_float);
  // proto:  GLboolean QOpenGLFunctions::glIsBuffer(GLuint buffer);
  fn _ZN16QOpenGLFunctions10glIsBufferEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glLineWidth(GLfloat width);
  fn _ZN16QOpenGLFunctions11glLineWidthEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
  fn _ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: *mut c_void);
  // proto:  void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
  fn _ZN16QOpenGLFunctions13glDepthRangefEff(qthis: *mut c_void, arg0: c_float, arg1: c_float);
  // proto:  void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_float);
  // proto:  void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
  fn _ZN16QOpenGLFunctions16glTexParameterivEjjPKi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
  fn _ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: *mut c_void);
  // proto:  void QOpenGLFunctions::glDeleteProgram(GLuint program);
  fn _ZN16QOpenGLFunctions15glDeleteProgramEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
  fn _ZN16QOpenGLFunctions23glBlendEquationSeparateEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
  fn _ZN16QOpenGLFunctions21glStencilMaskSeparateEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
  fn _ZN16QOpenGLFunctions12glDrawArraysEjii(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int);
  // proto:  void QOpenGLFunctions::glFinish();
  fn _ZN16QOpenGLFunctions8glFinishEv(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
  fn _ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_void);
  // proto:  void QOpenGLFunctions::glActiveTexture(GLenum texture);
  fn _ZN16QOpenGLFunctions15glActiveTextureEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glFrontFace(GLenum mode);
  fn _ZN16QOpenGLFunctions11glFrontFaceEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
  fn _ZN16QOpenGLFunctions13glPixelStoreiEji(qthis: *mut c_void, arg0: c_uint, arg1: c_int);
  // proto:  void QOpenGLFunctions::glCullFace(GLenum mode);
  fn _ZN16QOpenGLFunctions10glCullFaceEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions13glGetShaderivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
  fn _ZN16QOpenGLFunctions11glUniform4iEiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
  // proto:  void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
  fn _ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: *mut c_void);
  // proto:  void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
  fn _ZN16QOpenGLFunctions15glTexParameteriEjji(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int);
  // proto:  void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
  fn _ZN16QOpenGLFunctions12glClearColorEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QOpenGLFunctions::glClearDepthf(GLclampf depth);
  fn _ZN16QOpenGLFunctions13glClearDepthfEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
  fn _ZN16QOpenGLFunctions11glUniform2iEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QOpenGLFunctions::glGenerateMipmap(GLenum target);
  fn _ZN16QOpenGLFunctions16glGenerateMipmapEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
  fn _ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_int, arg8: *mut c_void);
  // proto:  void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
  fn _ZN16QOpenGLFunctions11glUniform3iEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
  fn _ZN16QOpenGLFunctions13glGenTexturesEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
  fn _ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  void QOpenGLFunctions::~QOpenGLFunctions();
  fn _ZN16QOpenGLFunctionsD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform4fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions14glGetProgramivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_float);
  // proto:  void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
  fn _ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_char);
  // proto:  GLboolean QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions16glIsRenderbufferEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions19glCopyTexSubImage2DEjiiiiiii(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int);
  // proto:  void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
  fn _ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_char, arg3: *mut c_int);
  // proto:  void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glDepthFunc(GLenum func);
  fn _ZN16QOpenGLFunctions11glDepthFuncEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
  fn _ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: *mut c_void);
  // proto:  void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
  fn _ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
  fn _ZN16QOpenGLFunctions6glHintEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
  fn _ZN16QOpenGLFunctions20glGetUniformLocationEjPKc(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_char);
  // proto:  GLboolean QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
  fn _ZN16QOpenGLFunctions15glIsFramebufferEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform1fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_float);
  // proto:  const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
  fn _ZN16QOpenGLFunctions11glGetStringEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_uchar;
  // proto:  void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *mut c_float);
  // proto:  void QOpenGLFunctions::QOpenGLFunctions(QOpenGLContext * context);
  fn dector_ZN16QOpenGLFunctionsC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QOpenGLFunctionsC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *mut c_float);
  // proto:  void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
  fn _ZN16QOpenGLFunctions12glBindBufferEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
  fn _ZN16QOpenGLFunctions11glUniform2fEiff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float);
  // proto:  void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform3fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform2fvEiiPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform1ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
  fn _ZN16QOpenGLFunctions12glBlendColorEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
  fn _ZN16QOpenGLFunctions14glDrawElementsEjijPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: *mut c_void);
  // proto:  void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
  fn _ZN16QOpenGLFunctions17glBindFramebufferEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  GLboolean QOpenGLFunctions::glIsProgram(GLuint program);
  fn _ZN16QOpenGLFunctions11glIsProgramEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glBlendEquation(GLenum mode);
  fn _ZN16QOpenGLFunctions15glBlendEquationEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
  fn _ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint, arg2: c_uint, arg3: *mut c_void, arg4: c_int);
  // proto:  void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
  fn _ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char);
  // proto:  void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
  fn _ZN16QOpenGLFunctions15glDeleteBuffersEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions9glScissorEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
  fn _ZN16QOpenGLFunctions18glGenRenderbuffersEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN16QOpenGLFunctions16glVertexAttrib3fEjfff(qthis: *mut c_void, arg0: c_uint, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  GLuint QOpenGLFunctions::glCreateProgram();
  fn _ZN16QOpenGLFunctions15glCreateProgramEv(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform4ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glEnable(GLenum cap);
  fn _ZN16QOpenGLFunctions8glEnableEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
  fn _ZN16QOpenGLFunctions13glBindTextureEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
  fn _ZN16QOpenGLFunctions15glTexParameterfEjjf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_float);
  // proto:  void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions10glViewportEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
  fn _ZN16QOpenGLFunctions16glSampleCoverageEfh(qthis: *mut c_void, arg0: c_float, arg1: c_uchar);
  // proto:  void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
  fn _ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint, arg4: c_int);
  // proto:  void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
  fn _ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_uchar, arg4: c_int, arg5: *mut c_void);
  // proto:  void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
  fn _ZN16QOpenGLFunctions15glPolygonOffsetEff(qthis: *mut c_void, arg0: c_float, arg1: c_float);
  // proto:  GLuint QOpenGLFunctions::glCreateShader(GLenum type);
  fn _ZN16QOpenGLFunctions14glCreateShaderEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
  fn _ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char);
  // proto:  GLboolean QOpenGLFunctions::glIsTexture(GLuint texture);
  fn _ZN16QOpenGLFunctions11glIsTextureEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
  fn _ZN16QOpenGLFunctions16glDeleteTexturesEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions13glGetIntegervEjPi(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_int);
  // proto:  void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
  fn _ZN16QOpenGLFunctions13glGetBooleanvEjPh(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_uchar);
  // proto:  void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions11glGetFloatvEjPf(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_float);
  // proto:  void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
  fn _ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  GLenum QOpenGLFunctions::glGetError();
  fn _ZN16QOpenGLFunctions10glGetErrorEv(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
  fn _ZN16QOpenGLFunctions14glDetachShaderEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
  fn _ZN16QOpenGLFunctions16glVertexAttrib2fEjff(qthis: *mut c_void, arg0: c_uint, arg1: c_float, arg2: c_float);
  // proto:  void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
  fn _ZN16QOpenGLFunctions16glVertexAttrib1fEjf(qthis: *mut c_void, arg0: c_uint, arg1: c_float);
  // proto:  void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
  fn _ZN16QOpenGLFunctions12glGenBuffersEiPj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_uint);
  // proto:  void QOpenGLFunctions::glClearStencil(GLint s);
  fn _ZN16QOpenGLFunctions14glClearStencilEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QOpenGLFunctions::glStencilMask(GLuint mask);
  fn _ZN16QOpenGLFunctions13glStencilMaskEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
  fn _ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char);
  // proto:  void QOpenGLFunctions::glReleaseShaderCompiler();
  fn _ZN16QOpenGLFunctions23glReleaseShaderCompilerEv(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glDepthMask(GLboolean flag);
  fn _ZN16QOpenGLFunctions11glDepthMaskEh(qthis: *mut c_void, arg0: c_uchar);
  // proto:  void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: *mut c_int);
  // proto:  void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
  fn _ZN16QOpenGLFunctions11glUniform1fEif(qthis: *mut c_void, arg0: c_int, arg1: c_float);
  // proto:  void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
  fn _ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_uint);
  // proto:  void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
  fn _ZN16QOpenGLFunctions11glStencilOpEjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint);
  // proto:  void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
  fn _ZN16QOpenGLFunctions13glStencilFuncEjij(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint);
  // proto:  void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
  fn _ZN16QOpenGLFunctions14glAttachShaderEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glDeleteShader(GLuint shader);
  fn _ZN16QOpenGLFunctions14glDeleteShaderEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glCompileShader(GLuint shader);
  fn _ZN16QOpenGLFunctions15glCompileShaderEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
  fn _ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint);
  // proto:  void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
  fn _ZN16QOpenGLFunctions11glColorMaskEhhhh(qthis: *mut c_void, arg0: c_uchar, arg1: c_uchar, arg2: c_uchar, arg3: c_uchar);
  // proto:  GLboolean QOpenGLFunctions::glIsEnabled(GLenum cap);
  fn _ZN16QOpenGLFunctions11glIsEnabledEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions18glBindRenderbufferEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_float);
  // proto:  void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
  fn _ZN16QOpenGLFunctions11glBlendFuncEjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint);
  // proto:  void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN16QOpenGLFunctions11glUniform3fEifff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN16QOpenGLFunctions16glVertexAttrib4fEjffff(qthis: *mut c_void, arg0: c_uint, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float);
  // proto:  GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
  fn _ZN16QOpenGLFunctions19glGetAttribLocationEjPKc(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_char);
  // proto:  void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform2ivEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
  fn _ZN16QOpenGLFunctions14glGetUniformivEjiPi(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
  fn _ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *mut c_void);
  // proto:  void QOpenGLFunctions::glUseProgram(GLuint program);
  fn _ZN16QOpenGLFunctions12glUseProgramEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glDisable(GLenum cap);
  fn _ZN16QOpenGLFunctions9glDisableEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN16QOpenGLFunctions11glUniform4fEiffff(qthis: *mut c_void, arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float);
  // proto:  void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
  fn _ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: c_uint);
  // proto:  void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
  fn _ZN16QOpenGLFunctions16glCopyTexImage2DEjijiiiii(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int);
  // proto:  void QOpenGLFunctions::glLinkProgram(GLuint program);
  fn _ZN16QOpenGLFunctions13glLinkProgramEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
  fn _ZN16QOpenGLFunctions12glBufferDataEjiPKvj(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_void, arg3: c_uint);
  // proto:  void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
  fn _ZN16QOpenGLFunctions14glGetUniformfvEjiPf(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions21glRenderbufferStorageEjjii(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: c_int);
  // proto:  GLboolean QOpenGLFunctions::glIsShader(GLuint shader);
  fn _ZN16QOpenGLFunctions10glIsShaderEj(qthis: *mut c_void, arg0: c_uint) -> c_uchar;
  // proto:  void QOpenGLFunctions::initializeOpenGLFunctions();
  fn _ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
  fn _ZN16QOpenGLFunctions11glUniform1iEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
  fn _ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint);
  // proto:  void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
  fn _ZN16QOpenGLFunctions16glTexParameterfvEjjPKf(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_float);
  // proto:  void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *mut c_float);
  // proto:  void QOpenGLFunctions::glValidateProgram(GLuint program);
  fn _ZN16QOpenGLFunctions17glValidateProgramEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::QOpenGLFunctions();
  fn dector_ZN16QOpenGLFunctionsC1Ev() -> *mut c_void;
  fn _ZN16QOpenGLFunctionsC1Ev(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions::glFlush();
  fn _ZN16QOpenGLFunctions7glFlushEv(qthis: *mut c_void);
  // proto:  GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
  fn _ZN16QOpenGLFunctions24glCheckFramebufferStatusEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
  fn _ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint);
  // proto:  void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions19glGetTexParameterivEjjPi(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: *mut c_int);
  // proto:  void QOpenGLFunctions::glClear(GLbitfield mask);
  fn _ZN16QOpenGLFunctions7glClearEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
  fn _ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc(qthis: *mut c_void, arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_char);
  // proto:  void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
  fn _ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj(qthis: *mut c_void, arg0: c_uint);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLFunctionsPrivate)=1152
pub struct QOpenGLFunctionsPrivate {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions)=8
pub struct QOpenGLFunctions {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctionsPrivate {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctionsPrivate {
    return QOpenGLFunctionsPrivate{qclsinst: qthis};
  }
}
  // proto:  void QOpenGLFunctionsPrivate::QOpenGLFunctionsPrivate(QOpenGLContext * ctx);
impl /*struct*/ QOpenGLFunctionsPrivate {
  pub fn New<T: QOpenGLFunctionsPrivate_New>(value: T) -> QOpenGLFunctionsPrivate {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctionsPrivate_New {
  fn New(self) -> QOpenGLFunctionsPrivate;
}

  // proto:  void QOpenGLFunctionsPrivate::QOpenGLFunctionsPrivate(QOpenGLContext * ctx);
impl<'a> /*trait*/ QOpenGLFunctionsPrivate_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctionsPrivate {
    // let qthis: *mut c_void = unsafe{calloc(1, 1152)};
    // unsafe{_ZN23QOpenGLFunctionsPrivateC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctionsPrivate_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN23QOpenGLFunctionsPrivateC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN23QOpenGLFunctionsPrivateC1EP14QOpenGLContext(arg0)};
    let rsthis = QOpenGLFunctionsPrivate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions {
    return QOpenGLFunctions{qclsinst: qthis};
  }
}
  // proto:  void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBindAttribLocation<RetType, T: QOpenGLFunctions_glBindAttribLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBindAttribLocation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindAttribLocation<RetType> {
  fn glBindAttribLocation(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glBindAttribLocation<()> for (u32, u32, &'a  String) {
  fn glBindAttribLocation(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGenFramebuffers<RetType, T: QOpenGLFunctions_glGenFramebuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGenFramebuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenFramebuffers<RetType> {
  fn glGenFramebuffers(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenFramebuffers<()> for (i32, &'a mut Vec<u32>) {
  fn glGenFramebuffers(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGenFramebuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions17glGenFramebuffersEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3iv<RetType, T: QOpenGLFunctions_glUniform3iv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform3iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3iv<RetType> {
  fn glUniform3iv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3iv<()> for (i32, i32, &'a  Vec<i32>) {
  fn glUniform3iv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform3ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform3ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib4fv<RetType, T: QOpenGLFunctions_glVertexAttrib4fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib4fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib4fv<RetType> {
  fn glVertexAttrib4fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib4fv<()> for (u32, &'a  Vec<f32>) {
  fn glVertexAttrib4fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLboolean QOpenGLFunctions::glIsBuffer(GLuint buffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsBuffer<RetType, T: QOpenGLFunctions_glIsBuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsBuffer<RetType> {
  fn glIsBuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsBuffer(GLuint buffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsBuffer<u8> for (u32) {
  fn glIsBuffer(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glIsBufferEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions10glIsBufferEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glLineWidth(GLfloat width);
impl /*struct*/ QOpenGLFunctions {
  pub fn glLineWidth<RetType, T: QOpenGLFunctions_glLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glLineWidth(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glLineWidth<RetType> {
  fn glLineWidth(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glLineWidth(GLfloat width);
impl<'a> /*trait*/ QOpenGLFunctions_glLineWidth<()> for (f32) {
  fn glLineWidth(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glLineWidthEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glLineWidthEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCompressedTexImage2D<RetType, T: QOpenGLFunctions_glCompressedTexImage2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCompressedTexImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCompressedTexImage2D<RetType> {
  fn glCompressedTexImage2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glCompressedTexImage2D<()> for (u32, i32, u32, i32, i32, i32, i32, *mut c_void) {
  fn glCompressedTexImage2D(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthRangef<RetType, T: QOpenGLFunctions_glDepthRangef<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDepthRangef(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDepthRangef<RetType> {
  fn glDepthRangef(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthRangef<()> for (f32, f32) {
  fn glDepthRangef(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glDepthRangefEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions13glDepthRangefEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib1fv<RetType, T: QOpenGLFunctions_glVertexAttrib1fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib1fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib1fv<RetType> {
  fn glVertexAttrib1fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib1fv<()> for (u32, &'a  Vec<f32>) {
  fn glVertexAttrib1fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameteriv<RetType, T: QOpenGLFunctions_glTexParameteriv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glTexParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameteriv<RetType> {
  fn glTexParameteriv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameteriv<()> for (u32, u32, &'a  Vec<i32>) {
  fn glTexParameteriv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glTexParameterivEjjPKi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions16glTexParameterivEjjPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
impl /*struct*/ QOpenGLFunctions {
  pub fn glTexSubImage2D<RetType, T: QOpenGLFunctions_glTexSubImage2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glTexSubImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexSubImage2D<RetType> {
  fn glTexSubImage2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glTexSubImage2D<()> for (u32, i32, i32, i32, i32, i32, u32, u32, *mut c_void) {
  fn glTexSubImage2D(self , rsthis: & QOpenGLFunctions) -> () {
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
    let arg8 = self.8  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDeleteProgram(GLuint program);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteProgram<RetType, T: QOpenGLFunctions_glDeleteProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDeleteProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteProgram<RetType> {
  fn glDeleteProgram(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDeleteProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteProgram<()> for (u32) {
  fn glDeleteProgram(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glDeleteProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glDeleteProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendEquationSeparate<RetType, T: QOpenGLFunctions_glBlendEquationSeparate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBlendEquationSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendEquationSeparate<RetType> {
  fn glBlendEquationSeparate(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendEquationSeparate<()> for (u32, u32) {
  fn glBlendEquationSeparate(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions23glBlendEquationSeparateEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions23glBlendEquationSeparateEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilMaskSeparate<RetType, T: QOpenGLFunctions_glStencilMaskSeparate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glStencilMaskSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilMaskSeparate<RetType> {
  fn glStencilMaskSeparate(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilMaskSeparate<()> for (u32, u32) {
  fn glStencilMaskSeparate(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glStencilMaskSeparateEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions21glStencilMaskSeparateEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDrawArrays<RetType, T: QOpenGLFunctions_glDrawArrays<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDrawArrays(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDrawArrays<RetType> {
  fn glDrawArrays(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
impl<'a> /*trait*/ QOpenGLFunctions_glDrawArrays<()> for (u32, i32, i32) {
  fn glDrawArrays(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glDrawArraysEjii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QOpenGLFunctions12glDrawArraysEjii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glFinish();
impl /*struct*/ QOpenGLFunctions {
  pub fn glFinish<RetType, T: QOpenGLFunctions_glFinish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glFinish(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFinish<RetType> {
  fn glFinish(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glFinish();
impl<'a> /*trait*/ QOpenGLFunctions_glFinish<()> for () {
  fn glFinish(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions8glFinishEv()};
     unsafe {_ZN16QOpenGLFunctions8glFinishEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribPointerv<RetType, T: QOpenGLFunctions_glGetVertexAttribPointerv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetVertexAttribPointerv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribPointerv<RetType> {
  fn glGetVertexAttribPointerv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribPointerv<()> for (u32, u32, *mut c_void) {
  fn glGetVertexAttribPointerv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glActiveTexture(GLenum texture);
impl /*struct*/ QOpenGLFunctions {
  pub fn glActiveTexture<RetType, T: QOpenGLFunctions_glActiveTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glActiveTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glActiveTexture<RetType> {
  fn glActiveTexture(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glActiveTexture(GLenum texture);
impl<'a> /*trait*/ QOpenGLFunctions_glActiveTexture<()> for (u32) {
  fn glActiveTexture(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glActiveTextureEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glActiveTextureEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glFrontFace(GLenum mode);
impl /*struct*/ QOpenGLFunctions {
  pub fn glFrontFace<RetType, T: QOpenGLFunctions_glFrontFace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glFrontFace(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFrontFace<RetType> {
  fn glFrontFace(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glFrontFace(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glFrontFace<()> for (u32) {
  fn glFrontFace(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glFrontFaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glFrontFaceEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetTexParameterfv<RetType, T: QOpenGLFunctions_glGetTexParameterfv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetTexParameterfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetTexParameterfv<RetType> {
  fn glGetTexParameterfv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetTexParameterfv<()> for (u32, u32, &'a mut Vec<f32>) {
  fn glGetTexParameterfv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
impl /*struct*/ QOpenGLFunctions {
  pub fn glPixelStorei<RetType, T: QOpenGLFunctions_glPixelStorei<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glPixelStorei(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glPixelStorei<RetType> {
  fn glPixelStorei(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
impl<'a> /*trait*/ QOpenGLFunctions_glPixelStorei<()> for (u32, i32) {
  fn glPixelStorei(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glPixelStoreiEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QOpenGLFunctions13glPixelStoreiEji(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glCullFace(GLenum mode);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCullFace<RetType, T: QOpenGLFunctions_glCullFace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCullFace(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCullFace<RetType> {
  fn glCullFace(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glCullFace(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glCullFace<()> for (u32) {
  fn glCullFace(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glCullFaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions10glCullFaceEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderiv<RetType, T: QOpenGLFunctions_glGetShaderiv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetShaderiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderiv<RetType> {
  fn glGetShaderiv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderiv<()> for (u32, u32, &'a mut Vec<i32>) {
  fn glGetShaderiv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetShaderivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions13glGetShaderivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4i<RetType, T: QOpenGLFunctions_glUniform4i<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform4i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4i<RetType> {
  fn glUniform4i(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4i<()> for (i32, i32, i32, i32, i32) {
  fn glUniform4i(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
impl /*struct*/ QOpenGLFunctions {
  pub fn glReadPixels<RetType, T: QOpenGLFunctions_glReadPixels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glReadPixels(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glReadPixels<RetType> {
  fn glReadPixels(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glReadPixels<()> for (i32, i32, i32, i32, u32, u32, *mut c_void) {
  fn glReadPixels(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_uint;
    let arg5 = self.5  as c_uint;
    let arg6 = self.6  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameteri<RetType, T: QOpenGLFunctions_glTexParameteri<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glTexParameteri(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameteri<RetType> {
  fn glTexParameteri(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameteri<()> for (u32, u32, i32) {
  fn glTexParameteri(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexParameteriEjji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QOpenGLFunctions15glTexParameteriEjji(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribiv<RetType, T: QOpenGLFunctions_glGetVertexAttribiv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetVertexAttribiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribiv<RetType> {
  fn glGetVertexAttribiv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribiv<()> for (u32, u32, &'a mut Vec<i32>) {
  fn glGetVertexAttribiv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl /*struct*/ QOpenGLFunctions {
  pub fn glClearColor<RetType, T: QOpenGLFunctions_glClearColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glClearColor(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClearColor<RetType> {
  fn glClearColor(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glClearColor<()> for (f32, f32, f32, f32) {
  fn glClearColor(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glClearDepthf(GLclampf depth);
impl /*struct*/ QOpenGLFunctions {
  pub fn glClearDepthf<RetType, T: QOpenGLFunctions_glClearDepthf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glClearDepthf(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClearDepthf<RetType> {
  fn glClearDepthf(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glClearDepthf(GLclampf depth);
impl<'a> /*trait*/ QOpenGLFunctions_glClearDepthf<()> for (f32) {
  fn glClearDepthf(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glClearDepthfEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN16QOpenGLFunctions13glClearDepthfEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2i<RetType, T: QOpenGLFunctions_glUniform2i<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform2i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2i<RetType> {
  fn glUniform2i(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2i<()> for (i32, i32, i32) {
  fn glUniform2i(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform2iEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QOpenGLFunctions11glUniform2iEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGenerateMipmap(GLenum target);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGenerateMipmap<RetType, T: QOpenGLFunctions_glGenerateMipmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGenerateMipmap(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenerateMipmap<RetType> {
  fn glGenerateMipmap(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGenerateMipmap(GLenum target);
impl<'a> /*trait*/ QOpenGLFunctions_glGenerateMipmap<()> for (u32) {
  fn glGenerateMipmap(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glGenerateMipmapEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions16glGenerateMipmapEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCompressedTexSubImage2D<RetType, T: QOpenGLFunctions_glCompressedTexSubImage2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCompressedTexSubImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCompressedTexSubImage2D<RetType> {
  fn glCompressedTexSubImage2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glCompressedTexSubImage2D<()> for (u32, i32, i32, i32, i32, i32, u32, i32, *mut c_void) {
  fn glCompressedTexSubImage2D(self , rsthis: & QOpenGLFunctions) -> () {
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
    let arg8 = self.8  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3i<RetType, T: QOpenGLFunctions_glUniform3i<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform3i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3i<RetType> {
  fn glUniform3i(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3i<()> for (i32, i32, i32, i32) {
  fn glUniform3i(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGenTextures<RetType, T: QOpenGLFunctions_glGenTextures<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGenTextures(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenTextures<RetType> {
  fn glGenTextures(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
impl<'a> /*trait*/ QOpenGLFunctions_glGenTextures<()> for (i32, &'a mut Vec<u32>) {
  fn glGenTextures(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGenTexturesEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions13glGenTexturesEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderPrecisionFormat<RetType, T: QOpenGLFunctions_glGetShaderPrecisionFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetShaderPrecisionFormat(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderPrecisionFormat<RetType> {
  fn glGetShaderPrecisionFormat(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderPrecisionFormat<()> for (u32, u32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn glGetShaderPrecisionFormat(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::~QOpenGLFunctions();
impl /*struct*/ QOpenGLFunctions {
  pub fn Free<RetType, T: QOpenGLFunctions_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::~QOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsD0Ev()};
     unsafe {_ZN16QOpenGLFunctionsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4fv<RetType, T: QOpenGLFunctions_glUniform4fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform4fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4fv<RetType> {
  fn glUniform4fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4fv<()> for (i32, i32, &'a  Vec<f32>) {
  fn glUniform4fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform4fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform4fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetProgramiv<RetType, T: QOpenGLFunctions_glGetProgramiv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetProgramiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetProgramiv<RetType> {
  fn glGetProgramiv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetProgramiv<()> for (u32, u32, &'a mut Vec<i32>) {
  fn glGetProgramiv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetProgramivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions14glGetProgramivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib2fv<RetType, T: QOpenGLFunctions_glVertexAttrib2fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib2fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib2fv<RetType> {
  fn glVertexAttrib2fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib2fv<()> for (u32, &'a  Vec<f32>) {
  fn glVertexAttrib2fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetActiveAttrib<RetType, T: QOpenGLFunctions_glGetActiveAttrib<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetActiveAttrib(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetActiveAttrib<RetType> {
  fn glGetActiveAttrib(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetActiveAttrib<()> for (u32, u32, i32, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<u32>, &'a mut String) {
  fn glGetActiveAttrib(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let arg4 = self.4.as_ptr()  as *mut c_int;
    let arg5 = self.5.as_ptr()  as *mut c_uint;
    let arg6 = self.6.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  GLboolean QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsRenderbuffer<RetType, T: QOpenGLFunctions_glIsRenderbuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsRenderbuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsRenderbuffer<RetType> {
  fn glIsRenderbuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsRenderbuffer<u8> for (u32) {
  fn glIsRenderbuffer(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glIsRenderbufferEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions16glIsRenderbufferEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCopyTexSubImage2D<RetType, T: QOpenGLFunctions_glCopyTexSubImage2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCopyTexSubImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCopyTexSubImage2D<RetType> {
  fn glCopyTexSubImage2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glCopyTexSubImage2D<()> for (u32, i32, i32, i32, i32, i32, i32, i32) {
  fn glCopyTexSubImage2D(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
impl /*struct*/ QOpenGLFunctions {
  pub fn glShaderSource<RetType, T: QOpenGLFunctions_glShaderSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glShaderSource(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glShaderSource<RetType> {
  fn glShaderSource(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
impl<'a> /*trait*/ QOpenGLFunctions_glShaderSource<()> for (u32, i32, &'a  String, &'a  Vec<i32>) {
  fn glShaderSource(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribfv<RetType, T: QOpenGLFunctions_glGetVertexAttribfv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetVertexAttribfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribfv<RetType> {
  fn glGetVertexAttribfv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribfv<()> for (u32, u32, &'a mut Vec<f32>) {
  fn glGetVertexAttribfv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDepthFunc(GLenum func);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthFunc<RetType, T: QOpenGLFunctions_glDepthFunc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDepthFunc(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDepthFunc<RetType> {
  fn glDepthFunc(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDepthFunc(GLenum func);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthFunc<()> for (u32) {
  fn glDepthFunc(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glDepthFuncEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glDepthFuncEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
impl /*struct*/ QOpenGLFunctions {
  pub fn glTexImage2D<RetType, T: QOpenGLFunctions_glTexImage2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glTexImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexImage2D<RetType> {
  fn glTexImage2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glTexImage2D<()> for (u32, i32, i32, i32, i32, i32, u32, u32, *mut c_void) {
  fn glTexImage2D(self , rsthis: & QOpenGLFunctions) -> () {
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
    let arg8 = self.8  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteFramebuffers<RetType, T: QOpenGLFunctions_glDeleteFramebuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDeleteFramebuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteFramebuffers<RetType> {
  fn glDeleteFramebuffers(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteFramebuffers<()> for (i32, &'a  Vec<u32>) {
  fn glDeleteFramebuffers(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
impl /*struct*/ QOpenGLFunctions {
  pub fn glHint<RetType, T: QOpenGLFunctions_glHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glHint(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glHint<RetType> {
  fn glHint(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glHint<()> for (u32, u32) {
  fn glHint(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions6glHintEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions6glHintEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformLocation<RetType, T: QOpenGLFunctions_glGetUniformLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetUniformLocation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformLocation<RetType> {
  fn glGetUniformLocation(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformLocation<()> for (u32, &'a  String) {
  fn glGetUniformLocation(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glGetUniformLocationEjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions20glGetUniformLocationEjPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLboolean QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsFramebuffer<RetType, T: QOpenGLFunctions_glIsFramebuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsFramebuffer<RetType> {
  fn glIsFramebuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsFramebuffer<u8> for (u32) {
  fn glIsFramebuffer(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glIsFramebufferEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions15glIsFramebufferEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1fv<RetType, T: QOpenGLFunctions_glUniform1fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform1fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1fv<RetType> {
  fn glUniform1fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1fv<()> for (i32, i32, &'a  Vec<f32>) {
  fn glUniform1fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform1fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform1fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetString<RetType, T: QOpenGLFunctions_glGetString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetString(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetString<RetType> {
  fn glGetString(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetString<String> for (u32) {
  fn glGetString(self , rsthis: & QOpenGLFunctions) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glGetStringEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glGetStringEj(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix2fv<RetType, T: QOpenGLFunctions_glUniformMatrix2fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniformMatrix2fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix2fv<RetType> {
  fn glUniformMatrix2fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix2fv<()> for (i32, i32, u8, &'a  Vec<f32>) {
  fn glUniformMatrix2fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::QOpenGLFunctions(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions {
  pub fn New<T: QOpenGLFunctions_New>(value: T) -> QOpenGLFunctions {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_New {
  fn New(self) -> QOpenGLFunctions;
}

  // proto:  void QOpenGLFunctions::QOpenGLFunctions(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QOpenGLFunctionsC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QOpenGLFunctionsC1EP14QOpenGLContext(arg0)};
    let rsthis = QOpenGLFunctions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix3fv<RetType, T: QOpenGLFunctions_glUniformMatrix3fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniformMatrix3fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix3fv<RetType> {
  fn glUniformMatrix3fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix3fv<()> for (i32, i32, u8, &'a  Vec<f32>) {
  fn glUniformMatrix3fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBindBuffer<RetType, T: QOpenGLFunctions_glBindBuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBindBuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindBuffer<RetType> {
  fn glBindBuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindBuffer<()> for (u32, u32) {
  fn glBindBuffer(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBindBufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions12glBindBufferEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2f<RetType, T: QOpenGLFunctions_glUniform2f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform2f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2f<RetType> {
  fn glUniform2f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2f<()> for (i32, f32, f32) {
  fn glUniform2f(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform2fEiff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glUniform2fEiff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3fv<RetType, T: QOpenGLFunctions_glUniform3fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform3fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3fv<RetType> {
  fn glUniform3fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3fv<()> for (i32, i32, &'a  Vec<f32>) {
  fn glUniform3fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform3fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform3fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2fv<RetType, T: QOpenGLFunctions_glUniform2fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform2fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2fv<RetType> {
  fn glUniform2fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2fv<()> for (i32, i32, &'a  Vec<f32>) {
  fn glUniform2fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform2fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions12glUniform2fvEiiPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetRenderbufferParameteriv<RetType, T: QOpenGLFunctions_glGetRenderbufferParameteriv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetRenderbufferParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetRenderbufferParameteriv<RetType> {
  fn glGetRenderbufferParameteriv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetRenderbufferParameteriv<()> for (u32, u32, &'a mut Vec<i32>) {
  fn glGetRenderbufferParameteriv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetBufferParameteriv<RetType, T: QOpenGLFunctions_glGetBufferParameteriv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetBufferParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetBufferParameteriv<RetType> {
  fn glGetBufferParameteriv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetBufferParameteriv<()> for (u32, u32, &'a mut Vec<i32>) {
  fn glGetBufferParameteriv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1iv<RetType, T: QOpenGLFunctions_glUniform1iv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform1iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1iv<RetType> {
  fn glUniform1iv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1iv<()> for (i32, i32, &'a  Vec<i32>) {
  fn glUniform1iv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform1ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform1ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendColor<RetType, T: QOpenGLFunctions_glBlendColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBlendColor(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendColor<RetType> {
  fn glBlendColor(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendColor<()> for (f32, f32, f32, f32) {
  fn glBlendColor(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDrawElements<RetType, T: QOpenGLFunctions_glDrawElements<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDrawElements(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDrawElements<RetType> {
  fn glDrawElements(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
impl<'a> /*trait*/ QOpenGLFunctions_glDrawElements<()> for (u32, i32, u32, *mut c_void) {
  fn glDrawElements(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDrawElementsEjijPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions14glDrawElementsEjijPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBindFramebuffer<RetType, T: QOpenGLFunctions_glBindFramebuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBindFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindFramebuffer<RetType> {
  fn glBindFramebuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindFramebuffer<()> for (u32, u32) {
  fn glBindFramebuffer(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glBindFramebufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions17glBindFramebufferEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLboolean QOpenGLFunctions::glIsProgram(GLuint program);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsProgram<RetType, T: QOpenGLFunctions_glIsProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsProgram<RetType> {
  fn glIsProgram(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glIsProgram<u8> for (u32) {
  fn glIsProgram(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsProgramEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glIsProgramEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBlendEquation(GLenum mode);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendEquation<RetType, T: QOpenGLFunctions_glBlendEquation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBlendEquation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendEquation<RetType> {
  fn glBlendEquation(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBlendEquation(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendEquation<()> for (u32) {
  fn glBlendEquation(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glBlendEquationEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glBlendEquationEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
impl /*struct*/ QOpenGLFunctions {
  pub fn glShaderBinary<RetType, T: QOpenGLFunctions_glShaderBinary<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glShaderBinary(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glShaderBinary<RetType> {
  fn glShaderBinary(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
impl<'a> /*trait*/ QOpenGLFunctions_glShaderBinary<()> for (i32, &'a  Vec<u32>, u32, *mut c_void, i32) {
  fn glShaderBinary(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *mut c_void;
    let arg4 = self.4  as c_int;
     unsafe {_ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetProgramInfoLog<RetType, T: QOpenGLFunctions_glGetProgramInfoLog<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetProgramInfoLog(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetProgramInfoLog<RetType> {
  fn glGetProgramInfoLog(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
impl<'a> /*trait*/ QOpenGLFunctions_glGetProgramInfoLog<()> for (u32, i32, &'a mut Vec<i32>, &'a mut String) {
  fn glGetProgramInfoLog(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteBuffers<RetType, T: QOpenGLFunctions_glDeleteBuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDeleteBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteBuffers<RetType> {
  fn glDeleteBuffers(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteBuffers<()> for (i32, &'a  Vec<u32>) {
  fn glDeleteBuffers(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glDeleteBuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions15glDeleteBuffersEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
impl /*struct*/ QOpenGLFunctions {
  pub fn glScissor<RetType, T: QOpenGLFunctions_glScissor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glScissor(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glScissor<RetType> {
  fn glScissor(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glScissor<()> for (i32, i32, i32, i32) {
  fn glScissor(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGenRenderbuffers<RetType, T: QOpenGLFunctions_glGenRenderbuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGenRenderbuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenRenderbuffers<RetType> {
  fn glGenRenderbuffers(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenRenderbuffers<()> for (i32, &'a mut Vec<u32>) {
  fn glGenRenderbuffers(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGenRenderbuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions18glGenRenderbuffersEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib3f<RetType, T: QOpenGLFunctions_glVertexAttrib3f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib3f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib3f<RetType> {
  fn glVertexAttrib3f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib3f<()> for (u32, f32, f32, f32) {
  fn glVertexAttrib3f(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  GLuint QOpenGLFunctions::glCreateProgram();
impl /*struct*/ QOpenGLFunctions {
  pub fn glCreateProgram<RetType, T: QOpenGLFunctions_glCreateProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCreateProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCreateProgram<RetType> {
  fn glCreateProgram(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLuint QOpenGLFunctions::glCreateProgram();
impl<'a> /*trait*/ QOpenGLFunctions_glCreateProgram<()> for () {
  fn glCreateProgram(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glCreateProgramEv()};
     unsafe {_ZN16QOpenGLFunctions15glCreateProgramEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4iv<RetType, T: QOpenGLFunctions_glUniform4iv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform4iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4iv<RetType> {
  fn glUniform4iv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4iv<()> for (i32, i32, &'a  Vec<i32>) {
  fn glUniform4iv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform4ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform4ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glEnable(GLenum cap);
impl /*struct*/ QOpenGLFunctions {
  pub fn glEnable<RetType, T: QOpenGLFunctions_glEnable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glEnable(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glEnable<RetType> {
  fn glEnable(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glEnable(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glEnable<()> for (u32) {
  fn glEnable(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions8glEnableEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions8glEnableEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBindTexture<RetType, T: QOpenGLFunctions_glBindTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBindTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindTexture<RetType> {
  fn glBindTexture(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
impl<'a> /*trait*/ QOpenGLFunctions_glBindTexture<()> for (u32, u32) {
  fn glBindTexture(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glBindTextureEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glBindTextureEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameterf<RetType, T: QOpenGLFunctions_glTexParameterf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glTexParameterf(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameterf<RetType> {
  fn glTexParameterf(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameterf<()> for (u32, u32, f32) {
  fn glTexParameterf(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexParameterfEjjf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_float;
     unsafe {_ZN16QOpenGLFunctions15glTexParameterfEjjf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
impl /*struct*/ QOpenGLFunctions {
  pub fn glViewport<RetType, T: QOpenGLFunctions_glViewport<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glViewport(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glViewport<RetType> {
  fn glViewport(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glViewport<()> for (i32, i32, i32, i32) {
  fn glViewport(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
impl /*struct*/ QOpenGLFunctions {
  pub fn glSampleCoverage<RetType, T: QOpenGLFunctions_glSampleCoverage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glSampleCoverage(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glSampleCoverage<RetType> {
  fn glSampleCoverage(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
impl<'a> /*trait*/ QOpenGLFunctions_glSampleCoverage<()> for (f32, u8) {
  fn glSampleCoverage(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glSampleCoverageEfh()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_uchar;
     unsafe {_ZN16QOpenGLFunctions16glSampleCoverageEfh(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
impl /*struct*/ QOpenGLFunctions {
  pub fn glFramebufferTexture2D<RetType, T: QOpenGLFunctions_glFramebufferTexture2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glFramebufferTexture2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFramebufferTexture2D<RetType> {
  fn glFramebufferTexture2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
impl<'a> /*trait*/ QOpenGLFunctions_glFramebufferTexture2D<()> for (u32, u32, u32, u32, i32) {
  fn glFramebufferTexture2D(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttribPointer<RetType, T: QOpenGLFunctions_glVertexAttribPointer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttribPointer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttribPointer<RetType> {
  fn glVertexAttribPointer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttribPointer<()> for (u32, i32, u32, u8, i32, *mut c_void) {
  fn glVertexAttribPointer(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uchar;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
impl /*struct*/ QOpenGLFunctions {
  pub fn glPolygonOffset<RetType, T: QOpenGLFunctions_glPolygonOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glPolygonOffset(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glPolygonOffset<RetType> {
  fn glPolygonOffset(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
impl<'a> /*trait*/ QOpenGLFunctions_glPolygonOffset<()> for (f32, f32) {
  fn glPolygonOffset(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glPolygonOffsetEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions15glPolygonOffsetEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLFunctions::glCreateShader(GLenum type);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCreateShader<RetType, T: QOpenGLFunctions_glCreateShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCreateShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCreateShader<RetType> {
  fn glCreateShader(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLuint QOpenGLFunctions::glCreateShader(GLenum type);
impl<'a> /*trait*/ QOpenGLFunctions_glCreateShader<()> for (u32) {
  fn glCreateShader(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glCreateShaderEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glCreateShaderEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderSource<RetType, T: QOpenGLFunctions_glGetShaderSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetShaderSource(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderSource<RetType> {
  fn glGetShaderSource(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderSource<()> for (u32, i32, &'a mut Vec<i32>, &'a mut String) {
  fn glGetShaderSource(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  GLboolean QOpenGLFunctions::glIsTexture(GLuint texture);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsTexture<RetType, T: QOpenGLFunctions_glIsTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsTexture<RetType> {
  fn glIsTexture(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsTexture(GLuint texture);
impl<'a> /*trait*/ QOpenGLFunctions_glIsTexture<u8> for (u32) {
  fn glIsTexture(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsTextureEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glIsTextureEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteTextures<RetType, T: QOpenGLFunctions_glDeleteTextures<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDeleteTextures(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteTextures<RetType> {
  fn glDeleteTextures(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteTextures<()> for (i32, &'a  Vec<u32>) {
  fn glDeleteTextures(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glDeleteTexturesEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions16glDeleteTexturesEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetIntegerv<RetType, T: QOpenGLFunctions_glGetIntegerv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetIntegerv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetIntegerv<RetType> {
  fn glGetIntegerv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetIntegerv<()> for (u32, &'a mut Vec<i32>) {
  fn glGetIntegerv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetIntegervEjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions13glGetIntegervEjPi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetBooleanv<RetType, T: QOpenGLFunctions_glGetBooleanv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetBooleanv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetBooleanv<RetType> {
  fn glGetBooleanv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetBooleanv<()> for (u32, &'a mut String) {
  fn glGetBooleanv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetBooleanvEjPh()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_uchar;
     unsafe {_ZN16QOpenGLFunctions13glGetBooleanvEjPh(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetFloatv<RetType, T: QOpenGLFunctions_glGetFloatv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetFloatv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetFloatv<RetType> {
  fn glGetFloatv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetFloatv<()> for (u32, &'a mut Vec<f32>) {
  fn glGetFloatv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glGetFloatvEjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions11glGetFloatvEjPf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteRenderbuffers<RetType, T: QOpenGLFunctions_glDeleteRenderbuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDeleteRenderbuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteRenderbuffers<RetType> {
  fn glDeleteRenderbuffers(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteRenderbuffers<()> for (i32, &'a  Vec<u32>) {
  fn glDeleteRenderbuffers(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  GLenum QOpenGLFunctions::glGetError();
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetError<RetType, T: QOpenGLFunctions_glGetError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetError(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetError<RetType> {
  fn glGetError(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLenum QOpenGLFunctions::glGetError();
impl<'a> /*trait*/ QOpenGLFunctions_glGetError<()> for () {
  fn glGetError(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glGetErrorEv()};
     unsafe {_ZN16QOpenGLFunctions10glGetErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDetachShader<RetType, T: QOpenGLFunctions_glDetachShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDetachShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDetachShader<RetType> {
  fn glDetachShader(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glDetachShader<()> for (u32, u32) {
  fn glDetachShader(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDetachShaderEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glDetachShaderEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib2f<RetType, T: QOpenGLFunctions_glVertexAttrib2f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib2f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib2f<RetType> {
  fn glVertexAttrib2f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib2f<()> for (u32, f32, f32) {
  fn glVertexAttrib2f(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib2fEjff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN16QOpenGLFunctions16glVertexAttrib2fEjff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib1f<RetType, T: QOpenGLFunctions_glVertexAttrib1f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib1f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib1f<RetType> {
  fn glVertexAttrib1f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib1f<()> for (u32, f32) {
  fn glVertexAttrib1f(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib1fEjf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions16glVertexAttrib1fEjf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGenBuffers<RetType, T: QOpenGLFunctions_glGenBuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGenBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGenBuffers<RetType> {
  fn glGenBuffers(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenBuffers<()> for (i32, &'a mut Vec<u32>) {
  fn glGenBuffers(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glGenBuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions12glGenBuffersEiPj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glClearStencil(GLint s);
impl /*struct*/ QOpenGLFunctions {
  pub fn glClearStencil<RetType, T: QOpenGLFunctions_glClearStencil<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glClearStencil(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClearStencil<RetType> {
  fn glClearStencil(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glClearStencil(GLint s);
impl<'a> /*trait*/ QOpenGLFunctions_glClearStencil<()> for (i32) {
  fn glClearStencil(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glClearStencilEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QOpenGLFunctions14glClearStencilEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glStencilMask(GLuint mask);
impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilMask<RetType, T: QOpenGLFunctions_glStencilMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glStencilMask(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilMask<RetType> {
  fn glStencilMask(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glStencilMask(GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilMask<()> for (u32) {
  fn glStencilMask(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glStencilMaskEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glStencilMaskEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderInfoLog<RetType, T: QOpenGLFunctions_glGetShaderInfoLog<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetShaderInfoLog(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderInfoLog<RetType> {
  fn glGetShaderInfoLog(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderInfoLog<()> for (u32, i32, &'a mut Vec<i32>, &'a mut String) {
  fn glGetShaderInfoLog(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glReleaseShaderCompiler();
impl /*struct*/ QOpenGLFunctions {
  pub fn glReleaseShaderCompiler<RetType, T: QOpenGLFunctions_glReleaseShaderCompiler<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glReleaseShaderCompiler(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glReleaseShaderCompiler<RetType> {
  fn glReleaseShaderCompiler(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glReleaseShaderCompiler();
impl<'a> /*trait*/ QOpenGLFunctions_glReleaseShaderCompiler<()> for () {
  fn glReleaseShaderCompiler(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions23glReleaseShaderCompilerEv()};
     unsafe {_ZN16QOpenGLFunctions23glReleaseShaderCompilerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDepthMask(GLboolean flag);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthMask<RetType, T: QOpenGLFunctions_glDepthMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDepthMask(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDepthMask<RetType> {
  fn glDepthMask(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDepthMask(GLboolean flag);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthMask<()> for (u8) {
  fn glDepthMask(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glDepthMaskEh()};
    let arg0 = self  as c_uchar;
     unsafe {_ZN16QOpenGLFunctions11glDepthMaskEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetFramebufferAttachmentParameteriv<RetType, T: QOpenGLFunctions_glGetFramebufferAttachmentParameteriv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetFramebufferAttachmentParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetFramebufferAttachmentParameteriv<RetType> {
  fn glGetFramebufferAttachmentParameteriv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetFramebufferAttachmentParameteriv<()> for (u32, u32, u32, &'a mut Vec<i32>) {
  fn glGetFramebufferAttachmentParameteriv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1f<RetType, T: QOpenGLFunctions_glUniform1f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform1f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1f<RetType> {
  fn glUniform1f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1f<()> for (i32, f32) {
  fn glUniform1f(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform1fEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
     unsafe {_ZN16QOpenGLFunctions11glUniform1fEif(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetAttachedShaders<RetType, T: QOpenGLFunctions_glGetAttachedShaders<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetAttachedShaders(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetAttachedShaders<RetType> {
  fn glGetAttachedShaders(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
impl<'a> /*trait*/ QOpenGLFunctions_glGetAttachedShaders<()> for (u32, i32, &'a mut Vec<i32>, &'a mut Vec<u32>) {
  fn glGetAttachedShaders(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_uint;
     unsafe {_ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilOp<RetType, T: QOpenGLFunctions_glStencilOp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glStencilOp(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilOp<RetType> {
  fn glStencilOp(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilOp<()> for (u32, u32, u32) {
  fn glStencilOp(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glStencilOpEjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glStencilOpEjjj(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilFunc<RetType, T: QOpenGLFunctions_glStencilFunc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glStencilFunc(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilFunc<RetType> {
  fn glStencilFunc(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilFunc<()> for (u32, i32, u32) {
  fn glStencilFunc(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glStencilFuncEjij()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glStencilFuncEjij(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
impl /*struct*/ QOpenGLFunctions {
  pub fn glAttachShader<RetType, T: QOpenGLFunctions_glAttachShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glAttachShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glAttachShader<RetType> {
  fn glAttachShader(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glAttachShader<()> for (u32, u32) {
  fn glAttachShader(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glAttachShaderEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glAttachShaderEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDeleteShader(GLuint shader);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteShader<RetType, T: QOpenGLFunctions_glDeleteShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDeleteShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteShader<RetType> {
  fn glDeleteShader(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDeleteShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteShader<()> for (u32) {
  fn glDeleteShader(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDeleteShaderEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions14glDeleteShaderEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glCompileShader(GLuint shader);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCompileShader<RetType, T: QOpenGLFunctions_glCompileShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCompileShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCompileShader<RetType> {
  fn glCompileShader(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glCompileShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glCompileShader<()> for (u32) {
  fn glCompileShader(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glCompileShaderEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions15glCompileShaderEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
impl /*struct*/ QOpenGLFunctions {
  pub fn glEnableVertexAttribArray<RetType, T: QOpenGLFunctions_glEnableVertexAttribArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glEnableVertexAttribArray(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glEnableVertexAttribArray<RetType> {
  fn glEnableVertexAttribArray(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
impl<'a> /*trait*/ QOpenGLFunctions_glEnableVertexAttribArray<()> for (u32) {
  fn glEnableVertexAttribArray(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glFramebufferRenderbuffer<RetType, T: QOpenGLFunctions_glFramebufferRenderbuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glFramebufferRenderbuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFramebufferRenderbuffer<RetType> {
  fn glFramebufferRenderbuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glFramebufferRenderbuffer<()> for (u32, u32, u32, u32) {
  fn glFramebufferRenderbuffer(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
impl /*struct*/ QOpenGLFunctions {
  pub fn glColorMask<RetType, T: QOpenGLFunctions_glColorMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glColorMask(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glColorMask<RetType> {
  fn glColorMask(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glColorMask<()> for (u8, u8, u8, u8) {
  fn glColorMask(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  GLboolean QOpenGLFunctions::glIsEnabled(GLenum cap);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsEnabled<RetType, T: QOpenGLFunctions_glIsEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsEnabled<RetType> {
  fn glIsEnabled(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsEnabled(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glIsEnabled<u8> for (u32) {
  fn glIsEnabled(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsEnabledEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions11glIsEnabledEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBindRenderbuffer<RetType, T: QOpenGLFunctions_glBindRenderbuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBindRenderbuffer(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBindRenderbuffer<RetType> {
  fn glBindRenderbuffer(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindRenderbuffer<()> for (u32, u32) {
  fn glBindRenderbuffer(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glBindRenderbufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions18glBindRenderbufferEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib3fv<RetType, T: QOpenGLFunctions_glVertexAttrib3fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib3fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib3fv<RetType> {
  fn glVertexAttrib3fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib3fv<()> for (u32, &'a  Vec<f32>) {
  fn glVertexAttrib3fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendFunc<RetType, T: QOpenGLFunctions_glBlendFunc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBlendFunc(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendFunc<RetType> {
  fn glBlendFunc(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendFunc<()> for (u32, u32) {
  fn glBlendFunc(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glBlendFuncEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN16QOpenGLFunctions11glBlendFuncEjj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3f<RetType, T: QOpenGLFunctions_glUniform3f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform3f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3f<RetType> {
  fn glUniform3f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3f<()> for (i32, f32, f32, f32) {
  fn glUniform3f(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib4f<RetType, T: QOpenGLFunctions_glVertexAttrib4f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glVertexAttrib4f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib4f<RetType> {
  fn glVertexAttrib4f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib4f<()> for (u32, f32, f32, f32, f32) {
  fn glVertexAttrib4f(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetAttribLocation<RetType, T: QOpenGLFunctions_glGetAttribLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetAttribLocation(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetAttribLocation<RetType> {
  fn glGetAttribLocation(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetAttribLocation<()> for (u32, &'a  String) {
  fn glGetAttribLocation(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetAttribLocationEjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions19glGetAttribLocationEjPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2iv<RetType, T: QOpenGLFunctions_glUniform2iv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform2iv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2iv<RetType> {
  fn glUniform2iv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2iv<()> for (i32, i32, &'a  Vec<i32>) {
  fn glUniform2iv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform2ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions12glUniform2ivEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformiv<RetType, T: QOpenGLFunctions_glGetUniformiv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetUniformiv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformiv<RetType> {
  fn glGetUniformiv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformiv<()> for (u32, i32, &'a mut Vec<i32>) {
  fn glGetUniformiv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetUniformivEjiPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions14glGetUniformivEjiPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBufferSubData<RetType, T: QOpenGLFunctions_glBufferSubData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBufferSubData(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBufferSubData<RetType> {
  fn glBufferSubData(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glBufferSubData<()> for (u32, i32, i32, *mut c_void) {
  fn glBufferSubData(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_void;
     unsafe {_ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUseProgram(GLuint program);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUseProgram<RetType, T: QOpenGLFunctions_glUseProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUseProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUseProgram<RetType> {
  fn glUseProgram(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUseProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glUseProgram<()> for (u32) {
  fn glUseProgram(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUseProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions12glUseProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDisable(GLenum cap);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDisable<RetType, T: QOpenGLFunctions_glDisable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDisable(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDisable<RetType> {
  fn glDisable(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDisable(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glDisable<()> for (u32) {
  fn glDisable(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions9glDisableEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions9glDisableEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4f<RetType, T: QOpenGLFunctions_glUniform4f<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform4f(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4f<RetType> {
  fn glUniform4f(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4f<()> for (i32, f32, f32, f32, f32) {
  fn glUniform4f(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilFuncSeparate<RetType, T: QOpenGLFunctions_glStencilFuncSeparate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glStencilFuncSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilFuncSeparate<RetType> {
  fn glStencilFuncSeparate(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilFuncSeparate<()> for (u32, u32, i32, u32) {
  fn glStencilFuncSeparate(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCopyTexImage2D<RetType, T: QOpenGLFunctions_glCopyTexImage2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCopyTexImage2D(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCopyTexImage2D<RetType> {
  fn glCopyTexImage2D(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
impl<'a> /*trait*/ QOpenGLFunctions_glCopyTexImage2D<()> for (u32, i32, u32, i32, i32, i32, i32, i32) {
  fn glCopyTexImage2D(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glLinkProgram(GLuint program);
impl /*struct*/ QOpenGLFunctions {
  pub fn glLinkProgram<RetType, T: QOpenGLFunctions_glLinkProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glLinkProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glLinkProgram<RetType> {
  fn glLinkProgram(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glLinkProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glLinkProgram<()> for (u32) {
  fn glLinkProgram(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glLinkProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions13glLinkProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBufferData<RetType, T: QOpenGLFunctions_glBufferData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBufferData(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBufferData<RetType> {
  fn glBufferData(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
impl<'a> /*trait*/ QOpenGLFunctions_glBufferData<()> for (u32, i32, *mut c_void, u32) {
  fn glBufferData(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBufferDataEjiPKvj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_void;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN16QOpenGLFunctions12glBufferDataEjiPKvj(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformfv<RetType, T: QOpenGLFunctions_glGetUniformfv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetUniformfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformfv<RetType> {
  fn glGetUniformfv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformfv<()> for (u32, i32, &'a mut Vec<f32>) {
  fn glGetUniformfv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetUniformfvEjiPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions14glGetUniformfvEjiPf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
impl /*struct*/ QOpenGLFunctions {
  pub fn glRenderbufferStorage<RetType, T: QOpenGLFunctions_glRenderbufferStorage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glRenderbufferStorage(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glRenderbufferStorage<RetType> {
  fn glRenderbufferStorage(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glRenderbufferStorage<()> for (u32, u32, i32, i32) {
  fn glRenderbufferStorage(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  GLboolean QOpenGLFunctions::glIsShader(GLuint shader);
impl /*struct*/ QOpenGLFunctions {
  pub fn glIsShader<RetType, T: QOpenGLFunctions_glIsShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glIsShader(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glIsShader<RetType> {
  fn glIsShader(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLboolean QOpenGLFunctions::glIsShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glIsShader<u8> for (u32) {
  fn glIsShader(self , rsthis: & QOpenGLFunctions) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glIsShaderEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN16QOpenGLFunctions10glIsShaderEj(rsthis.qclsinst, arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::initializeOpenGLFunctions();
impl /*struct*/ QOpenGLFunctions {
  pub fn initializeOpenGLFunctions<RetType, T: QOpenGLFunctions_initializeOpenGLFunctions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initializeOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_initializeOpenGLFunctions<RetType> {
  fn initializeOpenGLFunctions(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::initializeOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_initializeOpenGLFunctions<()> for () {
  fn initializeOpenGLFunctions(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv()};
     unsafe {_ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1i<RetType, T: QOpenGLFunctions_glUniform1i<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniform1i(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1i<RetType> {
  fn glUniform1i(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1i<()> for (i32, i32) {
  fn glUniform1i(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform1iEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QOpenGLFunctions11glUniform1iEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendFuncSeparate<RetType, T: QOpenGLFunctions_glBlendFuncSeparate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glBlendFuncSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glBlendFuncSeparate<RetType> {
  fn glBlendFuncSeparate(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendFuncSeparate<()> for (u32, u32, u32, u32) {
  fn glBlendFuncSeparate(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameterfv<RetType, T: QOpenGLFunctions_glTexParameterfv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glTexParameterfv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameterfv<RetType> {
  fn glTexParameterfv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameterfv<()> for (u32, u32, &'a  Vec<f32>) {
  fn glTexParameterfv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glTexParameterfvEjjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions16glTexParameterfvEjjPKf(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix4fv<RetType, T: QOpenGLFunctions_glUniformMatrix4fv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glUniformMatrix4fv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix4fv<RetType> {
  fn glUniformMatrix4fv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix4fv<()> for (i32, i32, u8, &'a  Vec<f32>) {
  fn glUniformMatrix4fv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3.as_ptr()  as *mut c_float;
     unsafe {_ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glValidateProgram(GLuint program);
impl /*struct*/ QOpenGLFunctions {
  pub fn glValidateProgram<RetType, T: QOpenGLFunctions_glValidateProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glValidateProgram(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glValidateProgram<RetType> {
  fn glValidateProgram(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glValidateProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glValidateProgram<()> for (u32) {
  fn glValidateProgram(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glValidateProgramEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions17glValidateProgramEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::QOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_New for () {
  fn New(self) -> QOpenGLFunctions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsC1Ev()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN16QOpenGLFunctionsC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN16QOpenGLFunctionsC1Ev()};
    let rsthis = QOpenGLFunctions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glFlush();
impl /*struct*/ QOpenGLFunctions {
  pub fn glFlush<RetType, T: QOpenGLFunctions_glFlush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glFlush(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glFlush<RetType> {
  fn glFlush(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glFlush();
impl<'a> /*trait*/ QOpenGLFunctions_glFlush<()> for () {
  fn glFlush(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions7glFlushEv()};
     unsafe {_ZN16QOpenGLFunctions7glFlushEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
impl /*struct*/ QOpenGLFunctions {
  pub fn glCheckFramebufferStatus<RetType, T: QOpenGLFunctions_glCheckFramebufferStatus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glCheckFramebufferStatus(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glCheckFramebufferStatus<RetType> {
  fn glCheckFramebufferStatus(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
impl<'a> /*trait*/ QOpenGLFunctions_glCheckFramebufferStatus<()> for (u32) {
  fn glCheckFramebufferStatus(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions24glCheckFramebufferStatusEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions24glCheckFramebufferStatusEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilOpSeparate<RetType, T: QOpenGLFunctions_glStencilOpSeparate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glStencilOpSeparate(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glStencilOpSeparate<RetType> {
  fn glStencilOpSeparate(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilOpSeparate<()> for (u32, u32, u32, u32) {
  fn glStencilOpSeparate(self , rsthis: & QOpenGLFunctions) -> () {
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

  // proto:  void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetTexParameteriv<RetType, T: QOpenGLFunctions_glGetTexParameteriv<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetTexParameteriv(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetTexParameteriv<RetType> {
  fn glGetTexParameteriv(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetTexParameteriv<()> for (u32, u32, &'a mut Vec<i32>) {
  fn glGetTexParameteriv(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetTexParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN16QOpenGLFunctions19glGetTexParameterivEjjPi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glClear(GLbitfield mask);
impl /*struct*/ QOpenGLFunctions {
  pub fn glClear<RetType, T: QOpenGLFunctions_glClear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glClear(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glClear<RetType> {
  fn glClear(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glClear(GLbitfield mask);
impl<'a> /*trait*/ QOpenGLFunctions_glClear<()> for (u32) {
  fn glClear(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions7glClearEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions7glClearEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl /*struct*/ QOpenGLFunctions {
  pub fn glGetActiveUniform<RetType, T: QOpenGLFunctions_glGetActiveUniform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glGetActiveUniform(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glGetActiveUniform<RetType> {
  fn glGetActiveUniform(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetActiveUniform<()> for (u32, u32, i32, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<u32>, &'a mut String) {
  fn glGetActiveUniform(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let arg4 = self.4.as_ptr()  as *mut c_int;
    let arg5 = self.5.as_ptr()  as *mut c_uint;
    let arg6 = self.6.as_ptr()  as *mut c_char;
     unsafe {_ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
impl /*struct*/ QOpenGLFunctions {
  pub fn glDisableVertexAttribArray<RetType, T: QOpenGLFunctions_glDisableVertexAttribArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glDisableVertexAttribArray(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_glDisableVertexAttribArray<RetType> {
  fn glDisableVertexAttribArray(self , rsthis: & QOpenGLFunctions) -> RetType;
}

  // proto:  void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
impl<'a> /*trait*/ QOpenGLFunctions_glDisableVertexAttribArray<()> for (u32) {
  fn glDisableVertexAttribArray(self , rsthis: & QOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

