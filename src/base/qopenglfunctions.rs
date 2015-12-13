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
  // proto: void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
  fn _ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc(arg0: c_uint, arg1: c_uint, arg2: *const c_char) -> i32;
  // proto: void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
  fn _ZN16QOpenGLFunctions17glGenFramebuffersEiPj(arg0: c_int, arg1: *mut c_uint) -> i32;
  // proto: void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform3ivEiiPKi(arg0: c_int, arg1: c_int, arg2: *const c_int) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf(arg0: c_uint, arg1: *const c_float) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsBuffer(GLuint buffer);
  fn _ZN16QOpenGLFunctions10glIsBufferEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glLineWidth(GLfloat width);
  fn _ZN16QOpenGLFunctions11glLineWidthEf(arg0: c_float) -> i32;
  // proto: void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
  fn _ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv(arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
  fn _ZN16QOpenGLFunctions13glDepthRangefEff(arg0: c_float, arg1: c_float) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf(arg0: c_uint, arg1: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
  fn _ZN16QOpenGLFunctions16glTexParameterivEjjPKi(arg0: c_uint, arg1: c_uint, arg2: *const c_int) -> i32;
  // proto: void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
  fn _ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv(arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glDeleteProgram(GLuint program);
  fn _ZN16QOpenGLFunctions15glDeleteProgramEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
  fn _ZN16QOpenGLFunctions23glBlendEquationSeparateEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
  fn _ZN16QOpenGLFunctions21glStencilMaskSeparateEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
  fn _ZN16QOpenGLFunctions12glDrawArraysEjii(arg0: c_uint, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QOpenGLFunctions::glFinish();
  fn _ZN16QOpenGLFunctions8glFinishEv() -> i32;
  // proto: void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
  fn _ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv(arg0: c_uint, arg1: c_uint, arg2: *mut uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glActiveTexture(GLenum texture);
  fn _ZN16QOpenGLFunctions15glActiveTextureEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glFrontFace(GLenum mode);
  fn _ZN16QOpenGLFunctions11glFrontFaceEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf(arg0: c_uint, arg1: c_uint, arg2: *mut c_float) -> i32;
  // proto: void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
  fn _ZN16QOpenGLFunctions13glPixelStoreiEji(arg0: c_uint, arg1: c_int) -> i32;
  // proto: void QOpenGLFunctions::glCullFace(GLenum mode);
  fn _ZN16QOpenGLFunctions10glCullFaceEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions13glGetShaderivEjjPi(arg0: c_uint, arg1: c_uint, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
  fn _ZN16QOpenGLFunctions11glUniform4iEiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> i32;
  // proto: void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
  fn _ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: *mut uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
  fn _ZN16QOpenGLFunctions15glTexParameteriEjji(arg0: c_uint, arg1: c_uint, arg2: c_int) -> i32;
  // proto: void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi(arg0: c_uint, arg1: c_uint, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
  fn _ZN16QOpenGLFunctions12glClearColorEffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: void QOpenGLFunctions::glClearDepthf(GLclampf depth);
  fn _ZN16QOpenGLFunctions13glClearDepthfEf(arg0: c_float) -> i32;
  // proto: void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
  fn _ZN16QOpenGLFunctions11glUniform2iEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QOpenGLFunctions::glGenerateMipmap(GLenum target);
  fn _ZN16QOpenGLFunctions16glGenerateMipmapEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
  fn _ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv(arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_int, arg8: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
  fn _ZN16QOpenGLFunctions11glUniform3iEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
  fn _ZN16QOpenGLFunctions13glGenTexturesEiPj(arg0: c_int, arg1: *mut c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
  fn _ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_(arg0: c_uint, arg1: c_uint, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::FreeQOpenGLFunctions();
  fn _ZN16QOpenGLFunctionsD0Ev() -> i32;
  // proto: void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform4fvEiiPKf(arg0: c_int, arg1: c_int, arg2: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions14glGetProgramivEjjPi(arg0: c_uint, arg1: c_uint, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf(arg0: c_uint, arg1: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
  fn _ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc(arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_char) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions16glIsRenderbufferEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions19glCopyTexSubImage2DEjiiiiiii(arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int) -> i32;
  // proto: void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
  fn _ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi(arg0: c_uint, arg1: c_int, arg2: *const c_char, arg3: *const c_int) -> i32;
  // proto: void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf(arg0: c_uint, arg1: c_uint, arg2: *mut c_float) -> i32;
  // proto: void QOpenGLFunctions::glDepthFunc(GLenum func);
  fn _ZN16QOpenGLFunctions11glDepthFuncEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
  fn _ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv(arg0: c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
  fn _ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj(arg0: c_int, arg1: *const c_uint) -> i32;
  // proto: void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
  fn _ZN16QOpenGLFunctions6glHintEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: QOpenGLFunctions::GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
  fn _ZN16QOpenGLFunctions20glGetUniformLocationEjPKc(arg0: c_uint, arg1: *const c_char) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
  fn _ZN16QOpenGLFunctions15glIsFramebufferEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform1fvEiiPKf(arg0: c_int, arg1: c_int, arg2: *const c_float) -> i32;
  // proto: const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
  fn _ZN16QOpenGLFunctions11glGetStringEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf(arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::NewQOpenGLFunctions(QOpenGLContext * context);
  fn _ZN16QOpenGLFunctionsC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf(arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
  fn _ZN16QOpenGLFunctions12glBindBufferEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
  fn _ZN16QOpenGLFunctions11glUniform2fEiff(arg0: c_int, arg1: c_float, arg2: c_float) -> i32;
  // proto: void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform3fvEiiPKf(arg0: c_int, arg1: c_int, arg2: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
  fn _ZN16QOpenGLFunctions12glUniform2fvEiiPKf(arg0: c_int, arg1: c_int, arg2: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi(arg0: c_uint, arg1: c_uint, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi(arg0: c_uint, arg1: c_uint, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform1ivEiiPKi(arg0: c_int, arg1: c_int, arg2: *const c_int) -> i32;
  // proto: void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
  fn _ZN16QOpenGLFunctions12glBlendColorEffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
  fn _ZN16QOpenGLFunctions14glDrawElementsEjijPKv(arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
  fn _ZN16QOpenGLFunctions17glBindFramebufferEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsProgram(GLuint program);
  fn _ZN16QOpenGLFunctions11glIsProgramEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glBlendEquation(GLenum mode);
  fn _ZN16QOpenGLFunctions15glBlendEquationEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
  fn _ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi(arg0: c_int, arg1: *const c_uint, arg2: c_uint, arg3: *const uint8_t, arg4: c_int) -> i32;
  // proto: void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
  fn _ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc(arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char) -> i32;
  // proto: void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
  fn _ZN16QOpenGLFunctions15glDeleteBuffersEiPKj(arg0: c_int, arg1: *const c_uint) -> i32;
  // proto: void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions9glScissorEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
  fn _ZN16QOpenGLFunctions18glGenRenderbuffersEiPj(arg0: c_int, arg1: *mut c_uint) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN16QOpenGLFunctions16glVertexAttrib3fEjfff(arg0: c_uint, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateProgram();
  fn _ZN16QOpenGLFunctions15glCreateProgramEv() -> i32;
  // proto: void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform4ivEiiPKi(arg0: c_int, arg1: c_int, arg2: *const c_int) -> i32;
  // proto: void QOpenGLFunctions::glEnable(GLenum cap);
  fn _ZN16QOpenGLFunctions8glEnableEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
  fn _ZN16QOpenGLFunctions13glBindTextureEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
  fn _ZN16QOpenGLFunctions15glTexParameterfEjjf(arg0: c_uint, arg1: c_uint, arg2: c_float) -> i32;
  // proto: void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions10glViewportEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
  fn _ZN16QOpenGLFunctions16glSampleCoverageEfh(arg0: c_float, arg1: c_uchar) -> i32;
  // proto: void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
  fn _ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji(arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint, arg4: c_int) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
  fn _ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv(arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_uchar, arg4: c_int, arg5: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
  fn _ZN16QOpenGLFunctions15glPolygonOffsetEff(arg0: c_float, arg1: c_float) -> i32;
  // proto: QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateShader(GLenum type);
  fn _ZN16QOpenGLFunctions14glCreateShaderEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
  fn _ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc(arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsTexture(GLuint texture);
  fn _ZN16QOpenGLFunctions11glIsTextureEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
  fn _ZN16QOpenGLFunctions16glDeleteTexturesEiPKj(arg0: c_int, arg1: *const c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions13glGetIntegervEjPi(arg0: c_uint, arg1: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
  fn _ZN16QOpenGLFunctions13glGetBooleanvEjPh(arg0: c_uint, arg1: *mut c_uchar) -> i32;
  // proto: void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
  fn _ZN16QOpenGLFunctions11glGetFloatvEjPf(arg0: c_uint, arg1: *mut c_float) -> i32;
  // proto: void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
  fn _ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj(arg0: c_int, arg1: *const c_uint) -> i32;
  // proto: QOpenGLFunctions::GLenum QOpenGLFunctions::glGetError();
  fn _ZN16QOpenGLFunctions10glGetErrorEv() -> i32;
  // proto: void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
  fn _ZN16QOpenGLFunctions14glDetachShaderEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
  fn _ZN16QOpenGLFunctions16glVertexAttrib2fEjff(arg0: c_uint, arg1: c_float, arg2: c_float) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
  fn _ZN16QOpenGLFunctions16glVertexAttrib1fEjf(arg0: c_uint, arg1: c_float) -> i32;
  // proto: void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
  fn _ZN16QOpenGLFunctions12glGenBuffersEiPj(arg0: c_int, arg1: *mut c_uint) -> i32;
  // proto: void QOpenGLFunctions::glClearStencil(GLint s);
  fn _ZN16QOpenGLFunctions14glClearStencilEi(arg0: c_int) -> i32;
  // proto: void QOpenGLFunctions::glStencilMask(GLuint mask);
  fn _ZN16QOpenGLFunctions13glStencilMaskEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
  fn _ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc(arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_char) -> i32;
  // proto: void QOpenGLFunctions::glReleaseShaderCompiler();
  fn _ZN16QOpenGLFunctions23glReleaseShaderCompilerEv() -> i32;
  // proto: void QOpenGLFunctions::glDepthMask(GLboolean flag);
  fn _ZN16QOpenGLFunctions11glDepthMaskEh(arg0: c_uchar) -> i32;
  // proto: void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi(arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
  fn _ZN16QOpenGLFunctions11glUniform1fEif(arg0: c_int, arg1: c_float) -> i32;
  // proto: void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
  fn _ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj(arg0: c_uint, arg1: c_int, arg2: *mut c_int, arg3: *mut c_uint) -> i32;
  // proto: void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
  fn _ZN16QOpenGLFunctions11glStencilOpEjjj(arg0: c_uint, arg1: c_uint, arg2: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
  fn _ZN16QOpenGLFunctions13glStencilFuncEjij(arg0: c_uint, arg1: c_int, arg2: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
  fn _ZN16QOpenGLFunctions14glAttachShaderEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glDeleteShader(GLuint shader);
  fn _ZN16QOpenGLFunctions14glDeleteShaderEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glCompileShader(GLuint shader);
  fn _ZN16QOpenGLFunctions15glCompileShaderEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
  fn _ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj(arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
  fn _ZN16QOpenGLFunctions11glColorMaskEhhhh(arg0: c_uchar, arg1: c_uchar, arg2: c_uchar, arg3: c_uchar) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsEnabled(GLenum cap);
  fn _ZN16QOpenGLFunctions11glIsEnabledEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
  fn _ZN16QOpenGLFunctions18glBindRenderbufferEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
  fn _ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf(arg0: c_uint, arg1: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
  fn _ZN16QOpenGLFunctions11glBlendFuncEjj(arg0: c_uint, arg1: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
  fn _ZN16QOpenGLFunctions11glUniform3fEifff(arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN16QOpenGLFunctions16glVertexAttrib4fEjffff(arg0: c_uint, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) -> i32;
  // proto: QOpenGLFunctions::GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
  fn _ZN16QOpenGLFunctions19glGetAttribLocationEjPKc(arg0: c_uint, arg1: *const c_char) -> i32;
  // proto: void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
  fn _ZN16QOpenGLFunctions12glUniform2ivEiiPKi(arg0: c_int, arg1: c_int, arg2: *const c_int) -> i32;
  // proto: void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
  fn _ZN16QOpenGLFunctions14glGetUniformivEjiPi(arg0: c_uint, arg1: c_int, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
  fn _ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv(arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *const uint8_t) -> i32;
  // proto: void QOpenGLFunctions::glUseProgram(GLuint program);
  fn _ZN16QOpenGLFunctions12glUseProgramEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glDisable(GLenum cap);
  fn _ZN16QOpenGLFunctions9glDisableEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  fn _ZN16QOpenGLFunctions11glUniform4fEiffff(arg0: c_int, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float) -> i32;
  // proto: void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
  fn _ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij(arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
  fn _ZN16QOpenGLFunctions16glCopyTexImage2DEjijiiiii(arg0: c_uint, arg1: c_int, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int) -> i32;
  // proto: void QOpenGLFunctions::glLinkProgram(GLuint program);
  fn _ZN16QOpenGLFunctions13glLinkProgramEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
  fn _ZN16QOpenGLFunctions12glBufferDataEjiPKvj(arg0: c_uint, arg1: c_int, arg2: *const uint8_t, arg3: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
  fn _ZN16QOpenGLFunctions14glGetUniformfvEjiPf(arg0: c_uint, arg1: c_int, arg2: *mut c_float) -> i32;
  // proto: void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
  fn _ZN16QOpenGLFunctions21glRenderbufferStorageEjjii(arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: c_int) -> i32;
  // proto: unsigned char QOpenGLFunctions::glIsShader(GLuint shader);
  fn _ZN16QOpenGLFunctions10glIsShaderEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::initializeOpenGLFunctions();
  fn _ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv() -> i32;
  // proto: void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
  fn _ZN16QOpenGLFunctions11glUniform1iEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
  fn _ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj(arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
  fn _ZN16QOpenGLFunctions16glTexParameterfvEjjPKf(arg0: c_uint, arg1: c_uint, arg2: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
  fn _ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf(arg0: c_int, arg1: c_int, arg2: c_uchar, arg3: *const c_float) -> i32;
  // proto: void QOpenGLFunctions::glValidateProgram(GLuint program);
  fn _ZN16QOpenGLFunctions17glValidateProgramEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::NewQOpenGLFunctions();
  fn _ZN16QOpenGLFunctionsC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QOpenGLFunctions::glFlush();
  fn _ZN16QOpenGLFunctions7glFlushEv() -> i32;
  // proto: QOpenGLFunctions::GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
  fn _ZN16QOpenGLFunctions24glCheckFramebufferStatusEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
  fn _ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj(arg0: c_uint, arg1: c_uint, arg2: c_uint, arg3: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
  fn _ZN16QOpenGLFunctions19glGetTexParameterivEjjPi(arg0: c_uint, arg1: c_uint, arg2: *mut c_int) -> i32;
  // proto: void QOpenGLFunctions::glClear(GLbitfield mask);
  fn _ZN16QOpenGLFunctions7glClearEj(arg0: c_uint) -> i32;
  // proto: void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
  fn _ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc(arg0: c_uint, arg1: c_uint, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_char) -> i32;
  // proto: void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
  fn _ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj(arg0: c_uint) -> i32;
}

// body block begin
// class sizeof(QOpenGLFunctions)=8
pub struct QOpenGLFunctions {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindAttribLocation<T: QOpenGLFunctions_glBindAttribLocation>(&mut self, value: T) -> i32 {
    value.glBindAttribLocation(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBindAttribLocation {
  fn glBindAttribLocation(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glBindAttribLocation for (u32, u32, &'a  String) {
  fn glBindAttribLocation(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN16QOpenGLFunctions20glBindAttribLocationEjjPKc(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenFramebuffers<T: QOpenGLFunctions_glGenFramebuffers>(&mut self, value: T) -> i32 {
    value.glGenFramebuffers(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGenFramebuffers {
  fn glGenFramebuffers(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGenFramebuffers(GLsizei n, GLuint * framebuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenFramebuffers for (i32, &'a mut u32) {
  fn glGenFramebuffers(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGenFramebuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
    unsafe {_ZN16QOpenGLFunctions17glGenFramebuffersEiPj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3iv<T: QOpenGLFunctions_glUniform3iv>(&mut self, value: T) -> i32 {
    value.glUniform3iv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3iv {
  fn glUniform3iv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform3iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3iv for (i32, i32, &'a  i32) {
  fn glUniform3iv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform3ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
    unsafe {_ZN16QOpenGLFunctions12glUniform3ivEiiPKi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib4fv<T: QOpenGLFunctions_glVertexAttrib4fv>(&mut self, value: T) -> i32 {
    value.glVertexAttrib4fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib4fv {
  fn glVertexAttrib4fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib4fv for (u32, &'a  f32) {
  fn glVertexAttrib4fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions17glVertexAttrib4fvEjPKf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsBuffer<T: QOpenGLFunctions_glIsBuffer>(&mut self, value: T) -> i32 {
    value.glIsBuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsBuffer {
  fn glIsBuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsBuffer(GLuint buffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsBuffer for (u32) {
  fn glIsBuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glIsBufferEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions10glIsBufferEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glLineWidth<T: QOpenGLFunctions_glLineWidth>(&mut self, value: T) -> i32 {
    value.glLineWidth(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glLineWidth {
  fn glLineWidth(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glLineWidth(GLfloat width);
impl<'a> /*trait*/ QOpenGLFunctions_glLineWidth for (f32) {
  fn glLineWidth(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glLineWidthEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN16QOpenGLFunctions11glLineWidthEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCompressedTexImage2D<T: QOpenGLFunctions_glCompressedTexImage2D>(&mut self, value: T) -> i32 {
    value.glCompressedTexImage2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCompressedTexImage2D {
  fn glCompressedTexImage2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glCompressedTexImage2D for (u32, i32, u32, i32, i32, i32, i32, &'a  u8) {
  fn glCompressedTexImage2D(self, this: &mut QOpenGLFunctions) -> i32 {
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
    unsafe {_ZN16QOpenGLFunctions22glCompressedTexImage2DEjijiiiiPKv(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthRangef<T: QOpenGLFunctions_glDepthRangef>(&mut self, value: T) -> i32 {
    value.glDepthRangef(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDepthRangef {
  fn glDepthRangef(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDepthRangef(GLclampf zNear, GLclampf zFar);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthRangef for (f32, f32) {
  fn glDepthRangef(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glDepthRangefEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    unsafe {_ZN16QOpenGLFunctions13glDepthRangefEff(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib1fv<T: QOpenGLFunctions_glVertexAttrib1fv>(&mut self, value: T) -> i32 {
    value.glVertexAttrib1fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib1fv {
  fn glVertexAttrib1fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib1fv for (u32, &'a  f32) {
  fn glVertexAttrib1fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions17glVertexAttrib1fvEjPKf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameteriv<T: QOpenGLFunctions_glTexParameteriv>(&mut self, value: T) -> i32 {
    value.glTexParameteriv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameteriv {
  fn glTexParameteriv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameteriv for (u32, u32, &'a  i32) {
  fn glTexParameteriv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glTexParameterivEjjPKi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *const c_int;
    unsafe {_ZN16QOpenGLFunctions16glTexParameterivEjjPKi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexSubImage2D<T: QOpenGLFunctions_glTexSubImage2D>(&mut self, value: T) -> i32 {
    value.glTexSubImage2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glTexSubImage2D {
  fn glTexSubImage2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glTexSubImage2D for (u32, i32, i32, i32, i32, i32, u32, u32, &'a  u8) {
  fn glTexSubImage2D(self, this: &mut QOpenGLFunctions) -> i32 {
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
    unsafe {_ZN16QOpenGLFunctions15glTexSubImage2DEjiiiiijjPKv(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteProgram<T: QOpenGLFunctions_glDeleteProgram>(&mut self, value: T) -> i32 {
    value.glDeleteProgram(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteProgram {
  fn glDeleteProgram(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDeleteProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteProgram for (u32) {
  fn glDeleteProgram(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glDeleteProgramEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions15glDeleteProgramEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendEquationSeparate<T: QOpenGLFunctions_glBlendEquationSeparate>(&mut self, value: T) -> i32 {
    value.glBlendEquationSeparate(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBlendEquationSeparate {
  fn glBlendEquationSeparate(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendEquationSeparate for (u32, u32) {
  fn glBlendEquationSeparate(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions23glBlendEquationSeparateEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions23glBlendEquationSeparateEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilMaskSeparate<T: QOpenGLFunctions_glStencilMaskSeparate>(&mut self, value: T) -> i32 {
    value.glStencilMaskSeparate(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glStencilMaskSeparate {
  fn glStencilMaskSeparate(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glStencilMaskSeparate(GLenum face, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilMaskSeparate for (u32, u32) {
  fn glStencilMaskSeparate(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glStencilMaskSeparateEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions21glStencilMaskSeparateEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDrawArrays<T: QOpenGLFunctions_glDrawArrays>(&mut self, value: T) -> i32 {
    value.glDrawArrays(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDrawArrays {
  fn glDrawArrays(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDrawArrays(GLenum mode, GLint first, GLsizei count);
impl<'a> /*trait*/ QOpenGLFunctions_glDrawArrays for (u32, i32, i32) {
  fn glDrawArrays(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glDrawArraysEjii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QOpenGLFunctions12glDrawArraysEjii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFinish<T: QOpenGLFunctions_glFinish>(&mut self, value: T) -> i32 {
    value.glFinish(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glFinish {
  fn glFinish(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glFinish();
impl<'a> /*trait*/ QOpenGLFunctions_glFinish for () {
  fn glFinish(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions8glFinishEv()};
    unsafe {_ZN16QOpenGLFunctions8glFinishEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribPointerv<T: QOpenGLFunctions_glGetVertexAttribPointerv>(&mut self, value: T) -> i32 {
    value.glGetVertexAttribPointerv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribPointerv {
  fn glGetVertexAttribPointerv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, GLenum pname, void ** pointer);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribPointerv for (u32, u32, &'a mut u8) {
  fn glGetVertexAttribPointerv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut uint8_t;
    unsafe {_ZN16QOpenGLFunctions25glGetVertexAttribPointervEjjPPv(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glActiveTexture<T: QOpenGLFunctions_glActiveTexture>(&mut self, value: T) -> i32 {
    value.glActiveTexture(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glActiveTexture {
  fn glActiveTexture(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glActiveTexture(GLenum texture);
impl<'a> /*trait*/ QOpenGLFunctions_glActiveTexture for (u32) {
  fn glActiveTexture(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glActiveTextureEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions15glActiveTextureEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFrontFace<T: QOpenGLFunctions_glFrontFace>(&mut self, value: T) -> i32 {
    value.glFrontFace(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glFrontFace {
  fn glFrontFace(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glFrontFace(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glFrontFace for (u32) {
  fn glFrontFace(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glFrontFaceEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glFrontFaceEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetTexParameterfv<T: QOpenGLFunctions_glGetTexParameterfv>(&mut self, value: T) -> i32 {
    value.glGetTexParameterfv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetTexParameterfv {
  fn glGetTexParameterfv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetTexParameterfv for (u32, u32, &'a mut f32) {
  fn glGetTexParameterfv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_float;
    unsafe {_ZN16QOpenGLFunctions19glGetTexParameterfvEjjPf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glPixelStorei<T: QOpenGLFunctions_glPixelStorei>(&mut self, value: T) -> i32 {
    value.glPixelStorei(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glPixelStorei {
  fn glPixelStorei(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glPixelStorei(GLenum pname, GLint param);
impl<'a> /*trait*/ QOpenGLFunctions_glPixelStorei for (u32, i32) {
  fn glPixelStorei(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glPixelStoreiEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QOpenGLFunctions13glPixelStoreiEji(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCullFace<T: QOpenGLFunctions_glCullFace>(&mut self, value: T) -> i32 {
    value.glCullFace(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCullFace {
  fn glCullFace(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glCullFace(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glCullFace for (u32) {
  fn glCullFace(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glCullFaceEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions10glCullFaceEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderiv<T: QOpenGLFunctions_glGetShaderiv>(&mut self, value: T) -> i32 {
    value.glGetShaderiv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderiv {
  fn glGetShaderiv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetShaderiv(GLuint shader, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderiv for (u32, u32, &'a mut i32) {
  fn glGetShaderiv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetShaderivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions13glGetShaderivEjjPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4i<T: QOpenGLFunctions_glUniform4i>(&mut self, value: T) -> i32 {
    value.glUniform4i(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4i {
  fn glUniform4i(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4i for (i32, i32, i32, i32, i32) {
  fn glUniform4i(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform4iEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    unsafe {_ZN16QOpenGLFunctions11glUniform4iEiiiii(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glReadPixels<T: QOpenGLFunctions_glReadPixels>(&mut self, value: T) -> i32 {
    value.glReadPixels(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glReadPixels {
  fn glReadPixels(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glReadPixels for (i32, i32, i32, i32, u32, u32, &'a mut u8) {
  fn glReadPixels(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_uint;
    let arg5 = self.5  as c_uint;
    let arg6 = self.6  as *mut uint8_t;
    unsafe {_ZN16QOpenGLFunctions12glReadPixelsEiiiijjPv(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameteri<T: QOpenGLFunctions_glTexParameteri>(&mut self, value: T) -> i32 {
    value.glTexParameteri(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameteri {
  fn glTexParameteri(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glTexParameteri(GLenum target, GLenum pname, GLint param);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameteri for (u32, u32, i32) {
  fn glTexParameteri(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexParameteriEjji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QOpenGLFunctions15glTexParameteriEjji(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribiv<T: QOpenGLFunctions_glGetVertexAttribiv>(&mut self, value: T) -> i32 {
    value.glGetVertexAttribiv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribiv {
  fn glGetVertexAttribiv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribiv for (u32, u32, &'a mut i32) {
  fn glGetVertexAttribiv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions19glGetVertexAttribivEjjPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClearColor<T: QOpenGLFunctions_glClearColor>(&mut self, value: T) -> i32 {
    value.glClearColor(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glClearColor {
  fn glClearColor(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glClearColor for (f32, f32, f32, f32) {
  fn glClearColor(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glClearColorEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN16QOpenGLFunctions12glClearColorEffff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClearDepthf<T: QOpenGLFunctions_glClearDepthf>(&mut self, value: T) -> i32 {
    value.glClearDepthf(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glClearDepthf {
  fn glClearDepthf(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glClearDepthf(GLclampf depth);
impl<'a> /*trait*/ QOpenGLFunctions_glClearDepthf for (f32) {
  fn glClearDepthf(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glClearDepthfEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN16QOpenGLFunctions13glClearDepthfEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2i<T: QOpenGLFunctions_glUniform2i>(&mut self, value: T) -> i32 {
    value.glUniform2i(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2i {
  fn glUniform2i(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2i for (i32, i32, i32) {
  fn glUniform2i(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform2iEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QOpenGLFunctions11glUniform2iEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenerateMipmap<T: QOpenGLFunctions_glGenerateMipmap>(&mut self, value: T) -> i32 {
    value.glGenerateMipmap(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGenerateMipmap {
  fn glGenerateMipmap(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGenerateMipmap(GLenum target);
impl<'a> /*trait*/ QOpenGLFunctions_glGenerateMipmap for (u32) {
  fn glGenerateMipmap(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glGenerateMipmapEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions16glGenerateMipmapEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCompressedTexSubImage2D<T: QOpenGLFunctions_glCompressedTexSubImage2D>(&mut self, value: T) -> i32 {
    value.glCompressedTexSubImage2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCompressedTexSubImage2D {
  fn glCompressedTexSubImage2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glCompressedTexSubImage2D for (u32, i32, i32, i32, i32, i32, u32, i32, &'a  u8) {
  fn glCompressedTexSubImage2D(self, this: &mut QOpenGLFunctions) -> i32 {
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
    unsafe {_ZN16QOpenGLFunctions25glCompressedTexSubImage2DEjiiiiijiPKv(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3i<T: QOpenGLFunctions_glUniform3i>(&mut self, value: T) -> i32 {
    value.glUniform3i(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3i {
  fn glUniform3i(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3i for (i32, i32, i32, i32) {
  fn glUniform3i(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform3iEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN16QOpenGLFunctions11glUniform3iEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenTextures<T: QOpenGLFunctions_glGenTextures>(&mut self, value: T) -> i32 {
    value.glGenTextures(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGenTextures {
  fn glGenTextures(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGenTextures(GLsizei n, GLuint * textures);
impl<'a> /*trait*/ QOpenGLFunctions_glGenTextures for (i32, &'a mut u32) {
  fn glGenTextures(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGenTexturesEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
    unsafe {_ZN16QOpenGLFunctions13glGenTexturesEiPj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderPrecisionFormat<T: QOpenGLFunctions_glGetShaderPrecisionFormat>(&mut self, value: T) -> i32 {
    value.glGetShaderPrecisionFormat(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderPrecisionFormat {
  fn glGetShaderPrecisionFormat(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint * range, GLint * precision);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderPrecisionFormat for (u32, u32, &'a mut i32, &'a mut i32) {
  fn glGetShaderPrecisionFormat(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions26glGetShaderPrecisionFormatEjjPiS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn FreeQOpenGLFunctions<T: QOpenGLFunctions_FreeQOpenGLFunctions>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLFunctions(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_FreeQOpenGLFunctions {
  fn FreeQOpenGLFunctions(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::FreeQOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_FreeQOpenGLFunctions for () {
  fn FreeQOpenGLFunctions(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctionsD0Ev()};
    unsafe {_ZN16QOpenGLFunctionsD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4fv<T: QOpenGLFunctions_glUniform4fv>(&mut self, value: T) -> i32 {
    value.glUniform4fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4fv {
  fn glUniform4fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform4fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4fv for (i32, i32, &'a  f32) {
  fn glUniform4fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform4fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions12glUniform4fvEiiPKf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetProgramiv<T: QOpenGLFunctions_glGetProgramiv>(&mut self, value: T) -> i32 {
    value.glGetProgramiv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetProgramiv {
  fn glGetProgramiv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetProgramiv(GLuint program, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetProgramiv for (u32, u32, &'a mut i32) {
  fn glGetProgramiv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetProgramivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions14glGetProgramivEjjPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib2fv<T: QOpenGLFunctions_glVertexAttrib2fv>(&mut self, value: T) -> i32 {
    value.glVertexAttrib2fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib2fv {
  fn glVertexAttrib2fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib2fv for (u32, &'a  f32) {
  fn glVertexAttrib2fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions17glVertexAttrib2fvEjPKf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetActiveAttrib<T: QOpenGLFunctions_glGetActiveAttrib>(&mut self, value: T) -> i32 {
    value.glGetActiveAttrib(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetActiveAttrib {
  fn glGetActiveAttrib(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetActiveAttrib for (u32, u32, i32, &'a mut i32, &'a mut i32, &'a mut u32, &'a mut String) {
  fn glGetActiveAttrib(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
    let arg5 = self.5  as *mut c_uint;
    let arg6 = self.6.as_ptr()  as *mut c_char;
    unsafe {_ZN16QOpenGLFunctions17glGetActiveAttribEjjiPiS0_PjPc(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsRenderbuffer<T: QOpenGLFunctions_glIsRenderbuffer>(&mut self, value: T) -> i32 {
    value.glIsRenderbuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsRenderbuffer {
  fn glIsRenderbuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsRenderbuffer for (u32) {
  fn glIsRenderbuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glIsRenderbufferEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions16glIsRenderbufferEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCopyTexSubImage2D<T: QOpenGLFunctions_glCopyTexSubImage2D>(&mut self, value: T) -> i32 {
    value.glCopyTexSubImage2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCopyTexSubImage2D {
  fn glCopyTexSubImage2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glCopyTexSubImage2D for (u32, i32, i32, i32, i32, i32, i32, i32) {
  fn glCopyTexSubImage2D(self, this: &mut QOpenGLFunctions) -> i32 {
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
    unsafe {_ZN16QOpenGLFunctions19glCopyTexSubImage2DEjiiiiiii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glShaderSource<T: QOpenGLFunctions_glShaderSource>(&mut self, value: T) -> i32 {
    value.glShaderSource(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glShaderSource {
  fn glShaderSource(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glShaderSource(GLuint shader, GLsizei count, const char ** string, const GLint * length);
impl<'a> /*trait*/ QOpenGLFunctions_glShaderSource for (u32, i32, &'a  String, &'a  i32) {
  fn glShaderSource(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as *const c_int;
    unsafe {_ZN16QOpenGLFunctions14glShaderSourceEjiPPKcPKi(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetVertexAttribfv<T: QOpenGLFunctions_glGetVertexAttribfv>(&mut self, value: T) -> i32 {
    value.glGetVertexAttribfv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetVertexAttribfv {
  fn glGetVertexAttribfv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetVertexAttribfv for (u32, u32, &'a mut f32) {
  fn glGetVertexAttribfv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_float;
    unsafe {_ZN16QOpenGLFunctions19glGetVertexAttribfvEjjPf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthFunc<T: QOpenGLFunctions_glDepthFunc>(&mut self, value: T) -> i32 {
    value.glDepthFunc(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDepthFunc {
  fn glDepthFunc(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDepthFunc(GLenum func);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthFunc for (u32) {
  fn glDepthFunc(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glDepthFuncEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glDepthFuncEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexImage2D<T: QOpenGLFunctions_glTexImage2D>(&mut self, value: T) -> i32 {
    value.glTexImage2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glTexImage2D {
  fn glTexImage2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid * pixels);
impl<'a> /*trait*/ QOpenGLFunctions_glTexImage2D for (u32, i32, i32, i32, i32, i32, u32, u32, &'a  u8) {
  fn glTexImage2D(self, this: &mut QOpenGLFunctions) -> i32 {
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
    unsafe {_ZN16QOpenGLFunctions12glTexImage2DEjiiiiijjPKv(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteFramebuffers<T: QOpenGLFunctions_glDeleteFramebuffers>(&mut self, value: T) -> i32 {
    value.glDeleteFramebuffers(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteFramebuffers {
  fn glDeleteFramebuffers(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDeleteFramebuffers(GLsizei n, const GLuint * framebuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteFramebuffers for (i32, &'a  u32) {
  fn glDeleteFramebuffers(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    unsafe {_ZN16QOpenGLFunctions20glDeleteFramebuffersEiPKj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glHint<T: QOpenGLFunctions_glHint>(&mut self, value: T) -> i32 {
    value.glHint(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glHint {
  fn glHint(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glHint(GLenum target, GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glHint for (u32, u32) {
  fn glHint(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions6glHintEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions6glHintEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformLocation<T: QOpenGLFunctions_glGetUniformLocation>(&mut self, value: T) -> i32 {
    value.glGetUniformLocation(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformLocation {
  fn glGetUniformLocation(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: QOpenGLFunctions::GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformLocation for (u32, &'a  String) {
  fn glGetUniformLocation(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glGetUniformLocationEjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN16QOpenGLFunctions20glGetUniformLocationEjPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsFramebuffer<T: QOpenGLFunctions_glIsFramebuffer>(&mut self, value: T) -> i32 {
    value.glIsFramebuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsFramebuffer {
  fn glIsFramebuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glIsFramebuffer for (u32) {
  fn glIsFramebuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glIsFramebufferEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions15glIsFramebufferEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1fv<T: QOpenGLFunctions_glUniform1fv>(&mut self, value: T) -> i32 {
    value.glUniform1fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1fv {
  fn glUniform1fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform1fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1fv for (i32, i32, &'a  f32) {
  fn glUniform1fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform1fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions12glUniform1fvEiiPKf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetString<T: QOpenGLFunctions_glGetString>(&mut self, value: T) -> i32 {
    value.glGetString(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetString {
  fn glGetString(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: const GLubyte * QOpenGLFunctions::glGetString(GLenum name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetString for (u32) {
  fn glGetString(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glGetStringEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glGetStringEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix2fv<T: QOpenGLFunctions_glUniformMatrix2fv>(&mut self, value: T) -> i32 {
    value.glUniformMatrix2fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix2fv {
  fn glUniformMatrix2fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix2fv for (i32, i32, u8, &'a  f32) {
  fn glUniformMatrix2fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions18glUniformMatrix2fvEiihPKf(arg0, arg1, arg2, arg3)};
    return 1;
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
  pub fn glUniformMatrix3fv<T: QOpenGLFunctions_glUniformMatrix3fv>(&mut self, value: T) -> i32 {
    value.glUniformMatrix3fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix3fv {
  fn glUniformMatrix3fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix3fv for (i32, i32, u8, &'a  f32) {
  fn glUniformMatrix3fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions18glUniformMatrix3fvEiihPKf(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindBuffer<T: QOpenGLFunctions_glBindBuffer>(&mut self, value: T) -> i32 {
    value.glBindBuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBindBuffer {
  fn glBindBuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBindBuffer(GLenum target, GLuint buffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindBuffer for (u32, u32) {
  fn glBindBuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBindBufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions12glBindBufferEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2f<T: QOpenGLFunctions_glUniform2f>(&mut self, value: T) -> i32 {
    value.glUniform2f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2f {
  fn glUniform2f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform2f(GLint location, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2f for (i32, f32, f32) {
  fn glUniform2f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform2fEiff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN16QOpenGLFunctions11glUniform2fEiff(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3fv<T: QOpenGLFunctions_glUniform3fv>(&mut self, value: T) -> i32 {
    value.glUniform3fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3fv {
  fn glUniform3fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform3fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3fv for (i32, i32, &'a  f32) {
  fn glUniform3fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform3fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions12glUniform3fvEiiPKf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2fv<T: QOpenGLFunctions_glUniform2fv>(&mut self, value: T) -> i32 {
    value.glUniform2fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2fv {
  fn glUniform2fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform2fv(GLint location, GLsizei count, const GLfloat * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2fv for (i32, i32, &'a  f32) {
  fn glUniform2fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform2fvEiiPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions12glUniform2fvEiiPKf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetRenderbufferParameteriv<T: QOpenGLFunctions_glGetRenderbufferParameteriv>(&mut self, value: T) -> i32 {
    value.glGetRenderbufferParameteriv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetRenderbufferParameteriv {
  fn glGetRenderbufferParameteriv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetRenderbufferParameteriv for (u32, u32, &'a mut i32) {
  fn glGetRenderbufferParameteriv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions28glGetRenderbufferParameterivEjjPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetBufferParameteriv<T: QOpenGLFunctions_glGetBufferParameteriv>(&mut self, value: T) -> i32 {
    value.glGetBufferParameteriv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetBufferParameteriv {
  fn glGetBufferParameteriv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetBufferParameteriv for (u32, u32, &'a mut i32) {
  fn glGetBufferParameteriv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions22glGetBufferParameterivEjjPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1iv<T: QOpenGLFunctions_glUniform1iv>(&mut self, value: T) -> i32 {
    value.glUniform1iv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1iv {
  fn glUniform1iv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform1iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1iv for (i32, i32, &'a  i32) {
  fn glUniform1iv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform1ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
    unsafe {_ZN16QOpenGLFunctions12glUniform1ivEiiPKi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendColor<T: QOpenGLFunctions_glBlendColor>(&mut self, value: T) -> i32 {
    value.glBlendColor(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBlendColor {
  fn glBlendColor(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBlendColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendColor for (f32, f32, f32, f32) {
  fn glBlendColor(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBlendColorEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN16QOpenGLFunctions12glBlendColorEffff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDrawElements<T: QOpenGLFunctions_glDrawElements>(&mut self, value: T) -> i32 {
    value.glDrawElements(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDrawElements {
  fn glDrawElements(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDrawElements(GLenum mode, GLsizei count, GLenum type, const GLvoid * indices);
impl<'a> /*trait*/ QOpenGLFunctions_glDrawElements for (u32, i32, u32, &'a  u8) {
  fn glDrawElements(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDrawElementsEjijPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *const uint8_t;
    unsafe {_ZN16QOpenGLFunctions14glDrawElementsEjijPKv(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindFramebuffer<T: QOpenGLFunctions_glBindFramebuffer>(&mut self, value: T) -> i32 {
    value.glBindFramebuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBindFramebuffer {
  fn glBindFramebuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBindFramebuffer(GLenum target, GLuint framebuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindFramebuffer for (u32, u32) {
  fn glBindFramebuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glBindFramebufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions17glBindFramebufferEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsProgram<T: QOpenGLFunctions_glIsProgram>(&mut self, value: T) -> i32 {
    value.glIsProgram(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsProgram {
  fn glIsProgram(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glIsProgram for (u32) {
  fn glIsProgram(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsProgramEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glIsProgramEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendEquation<T: QOpenGLFunctions_glBlendEquation>(&mut self, value: T) -> i32 {
    value.glBlendEquation(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBlendEquation {
  fn glBlendEquation(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBlendEquation(GLenum mode);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendEquation for (u32) {
  fn glBlendEquation(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glBlendEquationEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions15glBlendEquationEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glShaderBinary<T: QOpenGLFunctions_glShaderBinary>(&mut self, value: T) -> i32 {
    value.glShaderBinary(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glShaderBinary {
  fn glShaderBinary(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint * shaders, GLenum binaryformat, const void * binary, GLint length);
impl<'a> /*trait*/ QOpenGLFunctions_glShaderBinary for (i32, &'a  u32, u32, &'a  u8, i32) {
  fn glShaderBinary(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *const uint8_t;
    let arg4 = self.4  as c_int;
    unsafe {_ZN16QOpenGLFunctions14glShaderBinaryEiPKjjPKvi(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetProgramInfoLog<T: QOpenGLFunctions_glGetProgramInfoLog>(&mut self, value: T) -> i32 {
    value.glGetProgramInfoLog(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetProgramInfoLog {
  fn glGetProgramInfoLog(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, GLsizei bufsize, GLsizei * length, char * infolog);
impl<'a> /*trait*/ QOpenGLFunctions_glGetProgramInfoLog for (u32, i32, &'a mut i32, &'a mut String) {
  fn glGetProgramInfoLog(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    unsafe {_ZN16QOpenGLFunctions19glGetProgramInfoLogEjiPiPc(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteBuffers<T: QOpenGLFunctions_glDeleteBuffers>(&mut self, value: T) -> i32 {
    value.glDeleteBuffers(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteBuffers {
  fn glDeleteBuffers(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDeleteBuffers(GLsizei n, const GLuint * buffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteBuffers for (i32, &'a  u32) {
  fn glDeleteBuffers(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glDeleteBuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    unsafe {_ZN16QOpenGLFunctions15glDeleteBuffersEiPKj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glScissor<T: QOpenGLFunctions_glScissor>(&mut self, value: T) -> i32 {
    value.glScissor(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glScissor {
  fn glScissor(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glScissor for (i32, i32, i32, i32) {
  fn glScissor(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions9glScissorEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN16QOpenGLFunctions9glScissorEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenRenderbuffers<T: QOpenGLFunctions_glGenRenderbuffers>(&mut self, value: T) -> i32 {
    value.glGenRenderbuffers(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGenRenderbuffers {
  fn glGenRenderbuffers(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGenRenderbuffers(GLsizei n, GLuint * renderbuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenRenderbuffers for (i32, &'a mut u32) {
  fn glGenRenderbuffers(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGenRenderbuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
    unsafe {_ZN16QOpenGLFunctions18glGenRenderbuffersEiPj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib3f<T: QOpenGLFunctions_glVertexAttrib3f>(&mut self, value: T) -> i32 {
    value.glVertexAttrib3f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib3f {
  fn glVertexAttrib3f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib3f for (u32, f32, f32, f32) {
  fn glVertexAttrib3f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib3fEjfff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN16QOpenGLFunctions16glVertexAttrib3fEjfff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCreateProgram<T: QOpenGLFunctions_glCreateProgram>(&mut self, value: T) -> i32 {
    value.glCreateProgram(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCreateProgram {
  fn glCreateProgram(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateProgram();
impl<'a> /*trait*/ QOpenGLFunctions_glCreateProgram for () {
  fn glCreateProgram(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glCreateProgramEv()};
    unsafe {_ZN16QOpenGLFunctions15glCreateProgramEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4iv<T: QOpenGLFunctions_glUniform4iv>(&mut self, value: T) -> i32 {
    value.glUniform4iv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4iv {
  fn glUniform4iv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform4iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4iv for (i32, i32, &'a  i32) {
  fn glUniform4iv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform4ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
    unsafe {_ZN16QOpenGLFunctions12glUniform4ivEiiPKi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glEnable<T: QOpenGLFunctions_glEnable>(&mut self, value: T) -> i32 {
    value.glEnable(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glEnable {
  fn glEnable(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glEnable(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glEnable for (u32) {
  fn glEnable(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions8glEnableEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions8glEnableEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindTexture<T: QOpenGLFunctions_glBindTexture>(&mut self, value: T) -> i32 {
    value.glBindTexture(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBindTexture {
  fn glBindTexture(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBindTexture(GLenum target, GLuint texture);
impl<'a> /*trait*/ QOpenGLFunctions_glBindTexture for (u32, u32) {
  fn glBindTexture(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glBindTextureEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions13glBindTextureEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameterf<T: QOpenGLFunctions_glTexParameterf>(&mut self, value: T) -> i32 {
    value.glTexParameterf(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameterf {
  fn glTexParameterf(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glTexParameterf(GLenum target, GLenum pname, GLfloat param);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameterf for (u32, u32, f32) {
  fn glTexParameterf(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glTexParameterfEjjf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_float;
    unsafe {_ZN16QOpenGLFunctions15glTexParameterfEjjf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glViewport<T: QOpenGLFunctions_glViewport>(&mut self, value: T) -> i32 {
    value.glViewport(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glViewport {
  fn glViewport(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glViewport for (i32, i32, i32, i32) {
  fn glViewport(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glViewportEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN16QOpenGLFunctions10glViewportEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glSampleCoverage<T: QOpenGLFunctions_glSampleCoverage>(&mut self, value: T) -> i32 {
    value.glSampleCoverage(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glSampleCoverage {
  fn glSampleCoverage(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glSampleCoverage(GLclampf value, GLboolean invert);
impl<'a> /*trait*/ QOpenGLFunctions_glSampleCoverage for (f32, u8) {
  fn glSampleCoverage(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glSampleCoverageEfh()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_uchar;
    unsafe {_ZN16QOpenGLFunctions16glSampleCoverageEfh(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFramebufferTexture2D<T: QOpenGLFunctions_glFramebufferTexture2D>(&mut self, value: T) -> i32 {
    value.glFramebufferTexture2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glFramebufferTexture2D {
  fn glFramebufferTexture2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
impl<'a> /*trait*/ QOpenGLFunctions_glFramebufferTexture2D for (u32, u32, u32, u32, i32) {
  fn glFramebufferTexture2D(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    let arg4 = self.4  as c_int;
    unsafe {_ZN16QOpenGLFunctions22glFramebufferTexture2DEjjjji(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttribPointer<T: QOpenGLFunctions_glVertexAttribPointer>(&mut self, value: T) -> i32 {
    value.glVertexAttribPointer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttribPointer {
  fn glVertexAttribPointer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * ptr);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttribPointer for (u32, i32, u32, u8, i32, &'a  u8) {
  fn glVertexAttribPointer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uchar;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as *const uint8_t;
    unsafe {_ZN16QOpenGLFunctions21glVertexAttribPointerEjijhiPKv(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glPolygonOffset<T: QOpenGLFunctions_glPolygonOffset>(&mut self, value: T) -> i32 {
    value.glPolygonOffset(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glPolygonOffset {
  fn glPolygonOffset(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glPolygonOffset(GLfloat factor, GLfloat units);
impl<'a> /*trait*/ QOpenGLFunctions_glPolygonOffset for (f32, f32) {
  fn glPolygonOffset(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glPolygonOffsetEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    unsafe {_ZN16QOpenGLFunctions15glPolygonOffsetEff(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCreateShader<T: QOpenGLFunctions_glCreateShader>(&mut self, value: T) -> i32 {
    value.glCreateShader(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCreateShader {
  fn glCreateShader(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: QOpenGLFunctions::GLuint QOpenGLFunctions::glCreateShader(GLenum type);
impl<'a> /*trait*/ QOpenGLFunctions_glCreateShader for (u32) {
  fn glCreateShader(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glCreateShaderEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions14glCreateShaderEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderSource<T: QOpenGLFunctions_glGetShaderSource>(&mut self, value: T) -> i32 {
    value.glGetShaderSource(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderSource {
  fn glGetShaderSource(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetShaderSource(GLuint shader, GLsizei bufsize, GLsizei * length, char * source);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderSource for (u32, i32, &'a mut i32, &'a mut String) {
  fn glGetShaderSource(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    unsafe {_ZN16QOpenGLFunctions17glGetShaderSourceEjiPiPc(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsTexture<T: QOpenGLFunctions_glIsTexture>(&mut self, value: T) -> i32 {
    value.glIsTexture(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsTexture {
  fn glIsTexture(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsTexture(GLuint texture);
impl<'a> /*trait*/ QOpenGLFunctions_glIsTexture for (u32) {
  fn glIsTexture(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsTextureEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glIsTextureEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteTextures<T: QOpenGLFunctions_glDeleteTextures>(&mut self, value: T) -> i32 {
    value.glDeleteTextures(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteTextures {
  fn glDeleteTextures(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDeleteTextures(GLsizei n, const GLuint * textures);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteTextures for (i32, &'a  u32) {
  fn glDeleteTextures(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glDeleteTexturesEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    unsafe {_ZN16QOpenGLFunctions16glDeleteTexturesEiPKj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetIntegerv<T: QOpenGLFunctions_glGetIntegerv>(&mut self, value: T) -> i32 {
    value.glGetIntegerv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetIntegerv {
  fn glGetIntegerv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetIntegerv(GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetIntegerv for (u32, &'a mut i32) {
  fn glGetIntegerv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetIntegervEjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions13glGetIntegervEjPi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetBooleanv<T: QOpenGLFunctions_glGetBooleanv>(&mut self, value: T) -> i32 {
    value.glGetBooleanv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetBooleanv {
  fn glGetBooleanv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetBooleanv(GLenum pname, GLboolean * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetBooleanv for (u32, &'a mut String) {
  fn glGetBooleanv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glGetBooleanvEjPh()};
    let arg0 = self.0  as c_uint;
    let arg1 = 0  as *mut c_uchar;
    unsafe {_ZN16QOpenGLFunctions13glGetBooleanvEjPh(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetFloatv<T: QOpenGLFunctions_glGetFloatv>(&mut self, value: T) -> i32 {
    value.glGetFloatv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetFloatv {
  fn glGetFloatv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetFloatv(GLenum pname, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetFloatv for (u32, &'a mut f32) {
  fn glGetFloatv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glGetFloatvEjPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *mut c_float;
    unsafe {_ZN16QOpenGLFunctions11glGetFloatvEjPf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteRenderbuffers<T: QOpenGLFunctions_glDeleteRenderbuffers>(&mut self, value: T) -> i32 {
    value.glDeleteRenderbuffers(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteRenderbuffers {
  fn glDeleteRenderbuffers(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDeleteRenderbuffers(GLsizei n, const GLuint * renderbuffers);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteRenderbuffers for (i32, &'a  u32) {
  fn glDeleteRenderbuffers(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_uint;
    unsafe {_ZN16QOpenGLFunctions21glDeleteRenderbuffersEiPKj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetError<T: QOpenGLFunctions_glGetError>(&mut self, value: T) -> i32 {
    value.glGetError(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetError {
  fn glGetError(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: QOpenGLFunctions::GLenum QOpenGLFunctions::glGetError();
impl<'a> /*trait*/ QOpenGLFunctions_glGetError for () {
  fn glGetError(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glGetErrorEv()};
    unsafe {_ZN16QOpenGLFunctions10glGetErrorEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDetachShader<T: QOpenGLFunctions_glDetachShader>(&mut self, value: T) -> i32 {
    value.glDetachShader(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDetachShader {
  fn glDetachShader(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glDetachShader for (u32, u32) {
  fn glDetachShader(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDetachShaderEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions14glDetachShaderEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib2f<T: QOpenGLFunctions_glVertexAttrib2f>(&mut self, value: T) -> i32 {
    value.glVertexAttrib2f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib2f {
  fn glVertexAttrib2f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, GLfloat x, GLfloat y);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib2f for (u32, f32, f32) {
  fn glVertexAttrib2f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib2fEjff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN16QOpenGLFunctions16glVertexAttrib2fEjff(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib1f<T: QOpenGLFunctions_glVertexAttrib1f>(&mut self, value: T) -> i32 {
    value.glVertexAttrib1f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib1f {
  fn glVertexAttrib1f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, GLfloat x);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib1f for (u32, f32) {
  fn glVertexAttrib1f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib1fEjf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    unsafe {_ZN16QOpenGLFunctions16glVertexAttrib1fEjf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGenBuffers<T: QOpenGLFunctions_glGenBuffers>(&mut self, value: T) -> i32 {
    value.glGenBuffers(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGenBuffers {
  fn glGenBuffers(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGenBuffers(GLsizei n, GLuint * buffers);
impl<'a> /*trait*/ QOpenGLFunctions_glGenBuffers for (i32, &'a mut u32) {
  fn glGenBuffers(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glGenBuffersEiPj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_uint;
    unsafe {_ZN16QOpenGLFunctions12glGenBuffersEiPj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClearStencil<T: QOpenGLFunctions_glClearStencil>(&mut self, value: T) -> i32 {
    value.glClearStencil(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glClearStencil {
  fn glClearStencil(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glClearStencil(GLint s);
impl<'a> /*trait*/ QOpenGLFunctions_glClearStencil for (i32) {
  fn glClearStencil(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glClearStencilEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QOpenGLFunctions14glClearStencilEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilMask<T: QOpenGLFunctions_glStencilMask>(&mut self, value: T) -> i32 {
    value.glStencilMask(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glStencilMask {
  fn glStencilMask(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glStencilMask(GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilMask for (u32) {
  fn glStencilMask(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glStencilMaskEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions13glStencilMaskEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetShaderInfoLog<T: QOpenGLFunctions_glGetShaderInfoLog>(&mut self, value: T) -> i32 {
    value.glGetShaderInfoLog(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetShaderInfoLog {
  fn glGetShaderInfoLog(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, GLsizei bufsize, GLsizei * length, char * infolog);
impl<'a> /*trait*/ QOpenGLFunctions_glGetShaderInfoLog for (u32, i32, &'a mut i32, &'a mut String) {
  fn glGetShaderInfoLog(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    unsafe {_ZN16QOpenGLFunctions18glGetShaderInfoLogEjiPiPc(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glReleaseShaderCompiler<T: QOpenGLFunctions_glReleaseShaderCompiler>(&mut self, value: T) -> i32 {
    value.glReleaseShaderCompiler(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glReleaseShaderCompiler {
  fn glReleaseShaderCompiler(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glReleaseShaderCompiler();
impl<'a> /*trait*/ QOpenGLFunctions_glReleaseShaderCompiler for () {
  fn glReleaseShaderCompiler(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions23glReleaseShaderCompilerEv()};
    unsafe {_ZN16QOpenGLFunctions23glReleaseShaderCompilerEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDepthMask<T: QOpenGLFunctions_glDepthMask>(&mut self, value: T) -> i32 {
    value.glDepthMask(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDepthMask {
  fn glDepthMask(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDepthMask(GLboolean flag);
impl<'a> /*trait*/ QOpenGLFunctions_glDepthMask for (u8) {
  fn glDepthMask(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glDepthMaskEh()};
    let arg0 = self  as c_uchar;
    unsafe {_ZN16QOpenGLFunctions11glDepthMaskEh(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetFramebufferAttachmentParameteriv<T: QOpenGLFunctions_glGetFramebufferAttachmentParameteriv>(&mut self, value: T) -> i32 {
    value.glGetFramebufferAttachmentParameteriv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetFramebufferAttachmentParameteriv {
  fn glGetFramebufferAttachmentParameteriv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetFramebufferAttachmentParameteriv for (u32, u32, u32, &'a mut i32) {
  fn glGetFramebufferAttachmentParameteriv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions37glGetFramebufferAttachmentParameterivEjjjPi(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1f<T: QOpenGLFunctions_glUniform1f>(&mut self, value: T) -> i32 {
    value.glUniform1f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1f {
  fn glUniform1f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform1f(GLint location, GLfloat x);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1f for (i32, f32) {
  fn glUniform1f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform1fEif()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    unsafe {_ZN16QOpenGLFunctions11glUniform1fEif(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetAttachedShaders<T: QOpenGLFunctions_glGetAttachedShaders>(&mut self, value: T) -> i32 {
    value.glGetAttachedShaders(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetAttachedShaders {
  fn glGetAttachedShaders(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetAttachedShaders(GLuint program, GLsizei maxcount, GLsizei * count, GLuint * shaders);
impl<'a> /*trait*/ QOpenGLFunctions_glGetAttachedShaders for (u32, i32, &'a mut i32, &'a mut u32) {
  fn glGetAttachedShaders(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_uint;
    unsafe {_ZN16QOpenGLFunctions20glGetAttachedShadersEjiPiPj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilOp<T: QOpenGLFunctions_glStencilOp>(&mut self, value: T) -> i32 {
    value.glStencilOp(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glStencilOp {
  fn glStencilOp(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilOp for (u32, u32, u32) {
  fn glStencilOp(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glStencilOpEjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glStencilOpEjjj(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilFunc<T: QOpenGLFunctions_glStencilFunc>(&mut self, value: T) -> i32 {
    value.glStencilFunc(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glStencilFunc {
  fn glStencilFunc(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glStencilFunc(GLenum func, GLint ref, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilFunc for (u32, i32, u32) {
  fn glStencilFunc(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glStencilFuncEjij()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    unsafe {_ZN16QOpenGLFunctions13glStencilFuncEjij(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glAttachShader<T: QOpenGLFunctions_glAttachShader>(&mut self, value: T) -> i32 {
    value.glAttachShader(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glAttachShader {
  fn glAttachShader(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glAttachShader for (u32, u32) {
  fn glAttachShader(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glAttachShaderEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions14glAttachShaderEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDeleteShader<T: QOpenGLFunctions_glDeleteShader>(&mut self, value: T) -> i32 {
    value.glDeleteShader(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDeleteShader {
  fn glDeleteShader(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDeleteShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glDeleteShader for (u32) {
  fn glDeleteShader(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glDeleteShaderEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions14glDeleteShaderEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCompileShader<T: QOpenGLFunctions_glCompileShader>(&mut self, value: T) -> i32 {
    value.glCompileShader(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCompileShader {
  fn glCompileShader(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glCompileShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glCompileShader for (u32) {
  fn glCompileShader(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glCompileShaderEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions15glCompileShaderEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glEnableVertexAttribArray<T: QOpenGLFunctions_glEnableVertexAttribArray>(&mut self, value: T) -> i32 {
    value.glEnableVertexAttribArray(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glEnableVertexAttribArray {
  fn glEnableVertexAttribArray(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index);
impl<'a> /*trait*/ QOpenGLFunctions_glEnableVertexAttribArray for (u32) {
  fn glEnableVertexAttribArray(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions25glEnableVertexAttribArrayEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glFramebufferRenderbuffer<T: QOpenGLFunctions_glFramebufferRenderbuffer>(&mut self, value: T) -> i32 {
    value.glFramebufferRenderbuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glFramebufferRenderbuffer {
  fn glFramebufferRenderbuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glFramebufferRenderbuffer for (u32, u32, u32, u32) {
  fn glFramebufferRenderbuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN16QOpenGLFunctions25glFramebufferRenderbufferEjjjj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glColorMask<T: QOpenGLFunctions_glColorMask>(&mut self, value: T) -> i32 {
    value.glColorMask(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glColorMask {
  fn glColorMask(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
impl<'a> /*trait*/ QOpenGLFunctions_glColorMask for (u8, u8, u8, u8) {
  fn glColorMask(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glColorMaskEhhhh()};
    let arg0 = self.0  as c_uchar;
    let arg1 = self.1  as c_uchar;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as c_uchar;
    unsafe {_ZN16QOpenGLFunctions11glColorMaskEhhhh(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsEnabled<T: QOpenGLFunctions_glIsEnabled>(&mut self, value: T) -> i32 {
    value.glIsEnabled(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsEnabled {
  fn glIsEnabled(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsEnabled(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glIsEnabled for (u32) {
  fn glIsEnabled(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glIsEnabledEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glIsEnabledEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBindRenderbuffer<T: QOpenGLFunctions_glBindRenderbuffer>(&mut self, value: T) -> i32 {
    value.glBindRenderbuffer(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBindRenderbuffer {
  fn glBindRenderbuffer(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBindRenderbuffer(GLenum target, GLuint renderbuffer);
impl<'a> /*trait*/ QOpenGLFunctions_glBindRenderbuffer for (u32, u32) {
  fn glBindRenderbuffer(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glBindRenderbufferEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions18glBindRenderbufferEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib3fv<T: QOpenGLFunctions_glVertexAttrib3fv>(&mut self, value: T) -> i32 {
    value.glVertexAttrib3fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib3fv {
  fn glVertexAttrib3fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const GLfloat * values);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib3fv for (u32, &'a  f32) {
  fn glVertexAttrib3fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions17glVertexAttrib3fvEjPKf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendFunc<T: QOpenGLFunctions_glBlendFunc>(&mut self, value: T) -> i32 {
    value.glBlendFunc(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBlendFunc {
  fn glBlendFunc(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBlendFunc(GLenum sfactor, GLenum dfactor);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendFunc for (u32, u32) {
  fn glBlendFunc(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glBlendFuncEjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN16QOpenGLFunctions11glBlendFuncEjj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform3f<T: QOpenGLFunctions_glUniform3f>(&mut self, value: T) -> i32 {
    value.glUniform3f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform3f {
  fn glUniform3f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform3f(GLint location, GLfloat x, GLfloat y, GLfloat z);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform3f for (i32, f32, f32, f32) {
  fn glUniform3f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform3fEifff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN16QOpenGLFunctions11glUniform3fEifff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glVertexAttrib4f<T: QOpenGLFunctions_glVertexAttrib4f>(&mut self, value: T) -> i32 {
    value.glVertexAttrib4f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glVertexAttrib4f {
  fn glVertexAttrib4f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLFunctions_glVertexAttrib4f for (u32, f32, f32, f32, f32) {
  fn glVertexAttrib4f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glVertexAttrib4fEjffff()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    unsafe {_ZN16QOpenGLFunctions16glVertexAttrib4fEjffff(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetAttribLocation<T: QOpenGLFunctions_glGetAttribLocation>(&mut self, value: T) -> i32 {
    value.glGetAttribLocation(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetAttribLocation {
  fn glGetAttribLocation(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: QOpenGLFunctions::GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetAttribLocation for (u32, &'a  String) {
  fn glGetAttribLocation(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetAttribLocationEjPKc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN16QOpenGLFunctions19glGetAttribLocationEjPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform2iv<T: QOpenGLFunctions_glUniform2iv>(&mut self, value: T) -> i32 {
    value.glUniform2iv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform2iv {
  fn glUniform2iv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform2iv(GLint location, GLsizei count, const GLint * v);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform2iv for (i32, i32, &'a  i32) {
  fn glUniform2iv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUniform2ivEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
    unsafe {_ZN16QOpenGLFunctions12glUniform2ivEiiPKi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformiv<T: QOpenGLFunctions_glGetUniformiv>(&mut self, value: T) -> i32 {
    value.glGetUniformiv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformiv {
  fn glGetUniformiv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformiv for (u32, i32, &'a mut i32) {
  fn glGetUniformiv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetUniformivEjiPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions14glGetUniformivEjiPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBufferSubData<T: QOpenGLFunctions_glBufferSubData>(&mut self, value: T) -> i32 {
    value.glBufferSubData(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBufferSubData {
  fn glBufferSubData(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBufferSubData(GLenum target, qopengl_GLintptr offset, qopengl_GLsizeiptr size, const void * data);
impl<'a> /*trait*/ QOpenGLFunctions_glBufferSubData for (u32, i32, i32, &'a  u8) {
  fn glBufferSubData(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *const uint8_t;
    unsafe {_ZN16QOpenGLFunctions15glBufferSubDataEjiiPKv(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUseProgram<T: QOpenGLFunctions_glUseProgram>(&mut self, value: T) -> i32 {
    value.glUseProgram(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUseProgram {
  fn glUseProgram(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUseProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glUseProgram for (u32) {
  fn glUseProgram(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glUseProgramEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions12glUseProgramEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDisable<T: QOpenGLFunctions_glDisable>(&mut self, value: T) -> i32 {
    value.glDisable(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDisable {
  fn glDisable(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDisable(GLenum cap);
impl<'a> /*trait*/ QOpenGLFunctions_glDisable for (u32) {
  fn glDisable(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions9glDisableEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions9glDisableEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform4f<T: QOpenGLFunctions_glUniform4f>(&mut self, value: T) -> i32 {
    value.glUniform4f(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform4f {
  fn glUniform4f(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform4f(GLint location, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform4f for (i32, f32, f32, f32, f32) {
  fn glUniform4f(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform4fEiffff()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    unsafe {_ZN16QOpenGLFunctions11glUniform4fEiffff(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilFuncSeparate<T: QOpenGLFunctions_glStencilFuncSeparate>(&mut self, value: T) -> i32 {
    value.glStencilFuncSeparate(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glStencilFuncSeparate {
  fn glStencilFuncSeparate(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilFuncSeparate for (u32, u32, i32, u32) {
  fn glStencilFuncSeparate(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN16QOpenGLFunctions21glStencilFuncSeparateEjjij(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCopyTexImage2D<T: QOpenGLFunctions_glCopyTexImage2D>(&mut self, value: T) -> i32 {
    value.glCopyTexImage2D(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCopyTexImage2D {
  fn glCopyTexImage2D(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
impl<'a> /*trait*/ QOpenGLFunctions_glCopyTexImage2D for (u32, i32, u32, i32, i32, i32, i32, i32) {
  fn glCopyTexImage2D(self, this: &mut QOpenGLFunctions) -> i32 {
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
    unsafe {_ZN16QOpenGLFunctions16glCopyTexImage2DEjijiiiii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glLinkProgram<T: QOpenGLFunctions_glLinkProgram>(&mut self, value: T) -> i32 {
    value.glLinkProgram(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glLinkProgram {
  fn glLinkProgram(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glLinkProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glLinkProgram for (u32) {
  fn glLinkProgram(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions13glLinkProgramEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions13glLinkProgramEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBufferData<T: QOpenGLFunctions_glBufferData>(&mut self, value: T) -> i32 {
    value.glBufferData(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBufferData {
  fn glBufferData(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBufferData(GLenum target, qopengl_GLsizeiptr size, const void * data, GLenum usage);
impl<'a> /*trait*/ QOpenGLFunctions_glBufferData for (u32, i32, &'a  u8, u32) {
  fn glBufferData(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions12glBufferDataEjiPKvj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const uint8_t;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN16QOpenGLFunctions12glBufferDataEjiPKvj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetUniformfv<T: QOpenGLFunctions_glGetUniformfv>(&mut self, value: T) -> i32 {
    value.glGetUniformfv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetUniformfv {
  fn glGetUniformfv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetUniformfv for (u32, i32, &'a mut f32) {
  fn glGetUniformfv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions14glGetUniformfvEjiPf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_float;
    unsafe {_ZN16QOpenGLFunctions14glGetUniformfvEjiPf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glRenderbufferStorage<T: QOpenGLFunctions_glRenderbufferStorage>(&mut self, value: T) -> i32 {
    value.glRenderbufferStorage(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glRenderbufferStorage {
  fn glRenderbufferStorage(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
impl<'a> /*trait*/ QOpenGLFunctions_glRenderbufferStorage for (u32, u32, i32, i32) {
  fn glRenderbufferStorage(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions21glRenderbufferStorageEjjii()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN16QOpenGLFunctions21glRenderbufferStorageEjjii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glIsShader<T: QOpenGLFunctions_glIsShader>(&mut self, value: T) -> i32 {
    value.glIsShader(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glIsShader {
  fn glIsShader(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: unsigned char QOpenGLFunctions::glIsShader(GLuint shader);
impl<'a> /*trait*/ QOpenGLFunctions_glIsShader for (u32) {
  fn glIsShader(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions10glIsShaderEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions10glIsShaderEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn initializeOpenGLFunctions<T: QOpenGLFunctions_initializeOpenGLFunctions>(&mut self, value: T) -> i32 {
    value.initializeOpenGLFunctions(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_initializeOpenGLFunctions {
  fn initializeOpenGLFunctions(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::initializeOpenGLFunctions();
impl<'a> /*trait*/ QOpenGLFunctions_initializeOpenGLFunctions for () {
  fn initializeOpenGLFunctions(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv()};
    unsafe {_ZN16QOpenGLFunctions25initializeOpenGLFunctionsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniform1i<T: QOpenGLFunctions_glUniform1i>(&mut self, value: T) -> i32 {
    value.glUniform1i(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniform1i {
  fn glUniform1i(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniform1i(GLint location, GLint x);
impl<'a> /*trait*/ QOpenGLFunctions_glUniform1i for (i32, i32) {
  fn glUniform1i(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions11glUniform1iEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QOpenGLFunctions11glUniform1iEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glBlendFuncSeparate<T: QOpenGLFunctions_glBlendFuncSeparate>(&mut self, value: T) -> i32 {
    value.glBlendFuncSeparate(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glBlendFuncSeparate {
  fn glBlendFuncSeparate(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glBlendFuncSeparate(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha);
impl<'a> /*trait*/ QOpenGLFunctions_glBlendFuncSeparate for (u32, u32, u32, u32) {
  fn glBlendFuncSeparate(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN16QOpenGLFunctions19glBlendFuncSeparateEjjjj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glTexParameterfv<T: QOpenGLFunctions_glTexParameterfv>(&mut self, value: T) -> i32 {
    value.glTexParameterfv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glTexParameterfv {
  fn glTexParameterfv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
impl<'a> /*trait*/ QOpenGLFunctions_glTexParameterfv for (u32, u32, &'a  f32) {
  fn glTexParameterfv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions16glTexParameterfvEjjPKf()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions16glTexParameterfvEjjPKf(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glUniformMatrix4fv<T: QOpenGLFunctions_glUniformMatrix4fv>(&mut self, value: T) -> i32 {
    value.glUniformMatrix4fv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glUniformMatrix4fv {
  fn glUniformMatrix4fv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
impl<'a> /*trait*/ QOpenGLFunctions_glUniformMatrix4fv for (i32, i32, u8, &'a  f32) {
  fn glUniformMatrix4fv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uchar;
    let arg3 = self.3  as *const c_float;
    unsafe {_ZN16QOpenGLFunctions18glUniformMatrix4fvEiihPKf(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glValidateProgram<T: QOpenGLFunctions_glValidateProgram>(&mut self, value: T) -> i32 {
    value.glValidateProgram(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glValidateProgram {
  fn glValidateProgram(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glValidateProgram(GLuint program);
impl<'a> /*trait*/ QOpenGLFunctions_glValidateProgram for (u32) {
  fn glValidateProgram(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions17glValidateProgramEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions17glValidateProgramEj(arg0)};
    return 1;
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
  pub fn glFlush<T: QOpenGLFunctions_glFlush>(&mut self, value: T) -> i32 {
    value.glFlush(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glFlush {
  fn glFlush(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glFlush();
impl<'a> /*trait*/ QOpenGLFunctions_glFlush for () {
  fn glFlush(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions7glFlushEv()};
    unsafe {_ZN16QOpenGLFunctions7glFlushEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glCheckFramebufferStatus<T: QOpenGLFunctions_glCheckFramebufferStatus>(&mut self, value: T) -> i32 {
    value.glCheckFramebufferStatus(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glCheckFramebufferStatus {
  fn glCheckFramebufferStatus(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: QOpenGLFunctions::GLenum QOpenGLFunctions::glCheckFramebufferStatus(GLenum target);
impl<'a> /*trait*/ QOpenGLFunctions_glCheckFramebufferStatus for (u32) {
  fn glCheckFramebufferStatus(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions24glCheckFramebufferStatusEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions24glCheckFramebufferStatusEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glStencilOpSeparate<T: QOpenGLFunctions_glStencilOpSeparate>(&mut self, value: T) -> i32 {
    value.glStencilOpSeparate(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glStencilOpSeparate {
  fn glStencilOpSeparate(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glStencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);
impl<'a> /*trait*/ QOpenGLFunctions_glStencilOpSeparate for (u32, u32, u32, u32) {
  fn glStencilOpSeparate(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
    unsafe {_ZN16QOpenGLFunctions19glStencilOpSeparateEjjjj(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetTexParameteriv<T: QOpenGLFunctions_glGetTexParameteriv>(&mut self, value: T) -> i32 {
    value.glGetTexParameteriv(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetTexParameteriv {
  fn glGetTexParameteriv(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
impl<'a> /*trait*/ QOpenGLFunctions_glGetTexParameteriv for (u32, u32, &'a mut i32) {
  fn glGetTexParameteriv(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions19glGetTexParameterivEjjPi()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZN16QOpenGLFunctions19glGetTexParameterivEjjPi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glClear<T: QOpenGLFunctions_glClear>(&mut self, value: T) -> i32 {
    value.glClear(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glClear {
  fn glClear(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glClear(GLbitfield mask);
impl<'a> /*trait*/ QOpenGLFunctions_glClear for (u32) {
  fn glClear(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions7glClearEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions7glClearEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glGetActiveUniform<T: QOpenGLFunctions_glGetActiveUniform>(&mut self, value: T) -> i32 {
    value.glGetActiveUniform(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glGetActiveUniform {
  fn glGetActiveUniform(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, GLsizei bufsize, GLsizei * length, GLint * size, GLenum * type, char * name);
impl<'a> /*trait*/ QOpenGLFunctions_glGetActiveUniform for (u32, u32, i32, &'a mut i32, &'a mut i32, &'a mut u32, &'a mut String) {
  fn glGetActiveUniform(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_uint;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
    let arg5 = self.5  as *mut c_uint;
    let arg6 = self.6.as_ptr()  as *mut c_char;
    unsafe {_ZN16QOpenGLFunctions18glGetActiveUniformEjjiPiS0_PjPc(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions {
  pub fn glDisableVertexAttribArray<T: QOpenGLFunctions_glDisableVertexAttribArray>(&mut self, value: T) -> i32 {
    value.glDisableVertexAttribArray(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_glDisableVertexAttribArray {
  fn glDisableVertexAttribArray(self, this: &mut QOpenGLFunctions) -> i32;
}

// proto: void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index);
impl<'a> /*trait*/ QOpenGLFunctions_glDisableVertexAttribArray for (u32) {
  fn glDisableVertexAttribArray(self, this: &mut QOpenGLFunctions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN16QOpenGLFunctions26glDisableVertexAttribArrayEj(arg0)};
    return 1;
  }
}

