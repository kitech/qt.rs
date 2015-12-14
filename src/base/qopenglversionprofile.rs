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
  pub fn isLegacyVersion<T: QOpenGLVersionProfile_isLegacyVersion>(&mut self, value: T) -> i8 {
    return value.isLegacyVersion(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_isLegacyVersion {
  fn isLegacyVersion(self, rsthis: &mut QOpenGLVersionProfile) -> i8;
}

// proto:  bool QOpenGLVersionProfile::isLegacyVersion();
impl<'a> /*trait*/ QOpenGLVersionProfile_isLegacyVersion for () {
  fn isLegacyVersion(self, rsthis: &mut QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile15isLegacyVersionEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile15isLegacyVersionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn FreeQOpenGLVersionProfile<T: QOpenGLVersionProfile_FreeQOpenGLVersionProfile>(&mut self, value: T)  {
     value.FreeQOpenGLVersionProfile(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_FreeQOpenGLVersionProfile {
  fn FreeQOpenGLVersionProfile(self, rsthis: &mut QOpenGLVersionProfile) ;
}

// proto:  void QOpenGLVersionProfile::FreeQOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_FreeQOpenGLVersionProfile for () {
  fn FreeQOpenGLVersionProfile(self, rsthis: &mut QOpenGLVersionProfile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileD0Ev()};
     unsafe {_ZN21QOpenGLVersionProfileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn hasProfiles<T: QOpenGLVersionProfile_hasProfiles>(&mut self, value: T) -> i8 {
    return value.hasProfiles(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_hasProfiles {
  fn hasProfiles(self, rsthis: &mut QOpenGLVersionProfile) -> i8;
}

// proto:  bool QOpenGLVersionProfile::hasProfiles();
impl<'a> /*trait*/ QOpenGLVersionProfile_hasProfiles for () {
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
  pub fn version<T: QOpenGLVersionProfile_version>(&mut self, value: T)  {
     value.version(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_version {
  fn version(self, rsthis: &mut QOpenGLVersionProfile) ;
}

// proto:  QPair<int, int> QOpenGLVersionProfile::version();
impl<'a> /*trait*/ QOpenGLVersionProfile_version for () {
  fn version(self, rsthis: &mut QOpenGLVersionProfile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7versionEv()};
     unsafe {_ZNK21QOpenGLVersionProfile7versionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn setVersion<T: QOpenGLVersionProfile_setVersion>(&mut self, value: T)  {
     value.setVersion(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_setVersion {
  fn setVersion(self, rsthis: &mut QOpenGLVersionProfile) ;
}

// proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
impl<'a> /*trait*/ QOpenGLVersionProfile_setVersion for (i32, i32) {
  fn setVersion(self, rsthis: &mut QOpenGLVersionProfile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfile10setVersionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN21QOpenGLVersionProfile10setVersionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn isValid<T: QOpenGLVersionProfile_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_isValid {
  fn isValid(self, rsthis: &mut QOpenGLVersionProfile) -> i8;
}

// proto:  bool QOpenGLVersionProfile::isValid();
impl<'a> /*trait*/ QOpenGLVersionProfile_isValid for () {
  fn isValid(self, rsthis: &mut QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7isValidEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

