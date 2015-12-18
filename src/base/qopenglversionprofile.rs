// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsurfaceformat::QSurfaceFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QOpenGLVersionProfile::isLegacyVersion();
  fn _ZNK21QOpenGLVersionProfile15isLegacyVersionEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLVersionProfile::FreeQOpenGLVersionProfile();
  fn _ZN21QOpenGLVersionProfileD0Ev(qthis: *mut c_void) ;
  // proto:  bool QOpenGLVersionProfile::hasProfiles();
  fn _ZNK21QOpenGLVersionProfile11hasProfilesEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLVersionProfile::NewQOpenGLVersionProfile(const QSurfaceFormat & format);
  fn _ZN21QOpenGLVersionProfileC1ERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLVersionProfile::NewQOpenGLVersionProfile(const QOpenGLVersionProfile & other);
  fn _ZN21QOpenGLVersionProfileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLVersionProfile::NewQOpenGLVersionProfile();
  fn _ZN21QOpenGLVersionProfileC1Ev(qthis: *mut c_void) ;
  // proto:  QPair<int, int> QOpenGLVersionProfile::version();
  fn _ZNK21QOpenGLVersionProfile7versionEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
  fn _ZN21QOpenGLVersionProfile10setVersionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QOpenGLVersionProfile::isValid();
  fn _ZNK21QOpenGLVersionProfile7isValidEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QOpenGLVersionProfile)=8
pub struct QOpenGLVersionProfile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn isLegacyVersion<RetType, T: QOpenGLVersionProfile_isLegacyVersion<RetType>>(&mut self, value: T) -> RetType {
    return value.isLegacyVersion(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_isLegacyVersion<RetType> {
  fn isLegacyVersion(self, rsthis: &mut QOpenGLVersionProfile) -> RetType;
}

// proto:  bool QOpenGLVersionProfile::isLegacyVersion();
impl<'a> /*trait*/ QOpenGLVersionProfile_isLegacyVersion<i8> for () {
  fn isLegacyVersion(self, rsthis: &mut QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile15isLegacyVersionEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile15isLegacyVersionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn FreeQOpenGLVersionProfile<RetType, T: QOpenGLVersionProfile_FreeQOpenGLVersionProfile<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQOpenGLVersionProfile(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_FreeQOpenGLVersionProfile<RetType> {
  fn FreeQOpenGLVersionProfile(self, rsthis: &mut QOpenGLVersionProfile) -> RetType;
}

// proto:  void QOpenGLVersionProfile::FreeQOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_FreeQOpenGLVersionProfile<()> for () {
  fn FreeQOpenGLVersionProfile(self, rsthis: &mut QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileD0Ev()};
     unsafe {_ZN21QOpenGLVersionProfileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn hasProfiles<RetType, T: QOpenGLVersionProfile_hasProfiles<RetType>>(&mut self, value: T) -> RetType {
    return value.hasProfiles(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_hasProfiles<RetType> {
  fn hasProfiles(self, rsthis: &mut QOpenGLVersionProfile) -> RetType;
}

// proto:  bool QOpenGLVersionProfile::hasProfiles();
impl<'a> /*trait*/ QOpenGLVersionProfile_hasProfiles<i8> for () {
  fn hasProfiles(self, rsthis: &mut QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile11hasProfilesEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile11hasProfilesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn NewQOpenGLVersionProfile<T: QOpenGLVersionProfile_NewQOpenGLVersionProfile>(value: T) -> QOpenGLVersionProfile {
    let rsthis = value.NewQOpenGLVersionProfile();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_NewQOpenGLVersionProfile {
  fn NewQOpenGLVersionProfile(self) -> QOpenGLVersionProfile;
}

// proto: void QOpenGLVersionProfile::NewQOpenGLVersionProfile(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLVersionProfile_NewQOpenGLVersionProfile for (&'a  QSurfaceFormat) {
  fn NewQOpenGLVersionProfile(self) -> QOpenGLVersionProfile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC1ERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QOpenGLVersionProfileC1ERK14QSurfaceFormat(qthis, arg0)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOpenGLVersionProfile::NewQOpenGLVersionProfile(const QOpenGLVersionProfile & other);
impl<'a> /*trait*/ QOpenGLVersionProfile_NewQOpenGLVersionProfile for (&'a  QOpenGLVersionProfile) {
  fn NewQOpenGLVersionProfile(self) -> QOpenGLVersionProfile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QOpenGLVersionProfileC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOpenGLVersionProfile::NewQOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_NewQOpenGLVersionProfile for () {
  fn NewQOpenGLVersionProfile(self) -> QOpenGLVersionProfile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC1Ev()};
    unsafe {_ZN21QOpenGLVersionProfileC1Ev(qthis)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn version<RetType, T: QOpenGLVersionProfile_version<RetType>>(&mut self, value: T) -> RetType {
    return value.version(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_version<RetType> {
  fn version(self, rsthis: &mut QOpenGLVersionProfile) -> RetType;
}

// proto:  QPair<int, int> QOpenGLVersionProfile::version();
impl<'a> /*trait*/ QOpenGLVersionProfile_version<()> for () {
  fn version(self, rsthis: &mut QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7versionEv()};
     unsafe {_ZNK21QOpenGLVersionProfile7versionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn setVersion<RetType, T: QOpenGLVersionProfile_setVersion<RetType>>(&mut self, value: T) -> RetType {
    return value.setVersion(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_setVersion<RetType> {
  fn setVersion(self, rsthis: &mut QOpenGLVersionProfile) -> RetType;
}

// proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
impl<'a> /*trait*/ QOpenGLVersionProfile_setVersion<()> for (i32, i32) {
  fn setVersion(self, rsthis: &mut QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfile10setVersionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN21QOpenGLVersionProfile10setVersionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn isValid<RetType, T: QOpenGLVersionProfile_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_isValid<RetType> {
  fn isValid(self, rsthis: &mut QOpenGLVersionProfile) -> RetType;
}

// proto:  bool QOpenGLVersionProfile::isValid();
impl<'a> /*trait*/ QOpenGLVersionProfile_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7isValidEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

