// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QTextFragment::charFormatIndex();
  fn _ZNK13QTextFragment15charFormatIndexEv() -> i32;
  // proto: int QTextFragment::position();
  fn _ZNK13QTextFragment8positionEv() -> i32;
  // proto: void QTextFragment::NewQTextFragment(const QTextFragment & o);
  fn _ZN13QTextFragmentC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QTextFragment::contains(int position);
  fn _ZNK13QTextFragment8containsEi(arg0: c_int) -> i32;
  // proto: void QTextFragment::NewQTextFragment();
  fn _ZN13QTextFragmentC1Ev(qthis: *mut c_void) -> i32;
  // proto: QString QTextFragment::text();
  fn _ZNK13QTextFragment4textEv() -> i32;
  // proto: QList<QGlyphRun> QTextFragment::glyphRuns(int from, int length);
  fn _ZNK13QTextFragment9glyphRunsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QTextFragment::isValid();
  fn _ZNK13QTextFragment7isValidEv() -> i32;
  // proto: QTextCharFormat QTextFragment::charFormat();
  fn _ZNK13QTextFragment10charFormatEv() -> i32;
  // proto: int QTextFragment::length();
  fn _ZNK13QTextFragment6lengthEv() -> i32;
}

// body block begin
// class sizeof(QTextFragment)=16
pub struct QTextFragment {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFragment {
  pub fn charFormatIndex<T: QTextFragment_charFormatIndex>(&mut self, value: T) -> i32 {
    value.charFormatIndex(self);
    return 1;
  }
}

pub trait QTextFragment_charFormatIndex {
  fn charFormatIndex(self, this: &mut QTextFragment) -> i32;
}

// proto: int QTextFragment::charFormatIndex();
impl<'a> /*trait*/ QTextFragment_charFormatIndex for () {
  fn charFormatIndex(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment15charFormatIndexEv()};
    unsafe {_ZNK13QTextFragment15charFormatIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn position<T: QTextFragment_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QTextFragment_position {
  fn position(self, this: &mut QTextFragment) -> i32;
}

// proto: int QTextFragment::position();
impl<'a> /*trait*/ QTextFragment_position for () {
  fn position(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment8positionEv()};
    unsafe {_ZNK13QTextFragment8positionEv()};
    return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn NewQTextFragment<T: QTextFragment_NewQTextFragment>(value: T) -> QTextFragment {
    let rsthis = value.NewQTextFragment();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFragment_NewQTextFragment {
  fn NewQTextFragment(self) -> QTextFragment;
}

// proto: void QTextFragment::NewQTextFragment(const QTextFragment & o);
impl<'a> /*trait*/ QTextFragment_NewQTextFragment for (&'a  QTextFragment) {
  fn NewQTextFragment(self) -> QTextFragment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextFragmentC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTextFragmentC1ERKS_(qthis, arg0)};
    let rsthis = QTextFragment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn contains<T: QTextFragment_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QTextFragment_contains {
  fn contains(self, this: &mut QTextFragment) -> i32;
}

// proto: bool QTextFragment::contains(int position);
impl<'a> /*trait*/ QTextFragment_contains for (i32) {
  fn contains(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment8containsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QTextFragment8containsEi(arg0)};
    return 1;
  }
}

// proto: void QTextFragment::NewQTextFragment();
impl<'a> /*trait*/ QTextFragment_NewQTextFragment for () {
  fn NewQTextFragment(self) -> QTextFragment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextFragmentC1Ev()};
    unsafe {_ZN13QTextFragmentC1Ev(qthis)};
    let rsthis = QTextFragment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn text<T: QTextFragment_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QTextFragment_text {
  fn text(self, this: &mut QTextFragment) -> i32;
}

// proto: QString QTextFragment::text();
impl<'a> /*trait*/ QTextFragment_text for () {
  fn text(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment4textEv()};
    unsafe {_ZNK13QTextFragment4textEv()};
    return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn glyphRuns<T: QTextFragment_glyphRuns>(&mut self, value: T) -> i32 {
    value.glyphRuns(self);
    return 1;
  }
}

pub trait QTextFragment_glyphRuns {
  fn glyphRuns(self, this: &mut QTextFragment) -> i32;
}

// proto: QList<QGlyphRun> QTextFragment::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextFragment_glyphRuns for (i32, i32) {
  fn glyphRuns(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK13QTextFragment9glyphRunsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn isValid<T: QTextFragment_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextFragment_isValid {
  fn isValid(self, this: &mut QTextFragment) -> i32;
}

// proto: bool QTextFragment::isValid();
impl<'a> /*trait*/ QTextFragment_isValid for () {
  fn isValid(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment7isValidEv()};
    unsafe {_ZNK13QTextFragment7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn charFormat<T: QTextFragment_charFormat>(&mut self, value: T) -> i32 {
    value.charFormat(self);
    return 1;
  }
}

pub trait QTextFragment_charFormat {
  fn charFormat(self, this: &mut QTextFragment) -> i32;
}

// proto: QTextCharFormat QTextFragment::charFormat();
impl<'a> /*trait*/ QTextFragment_charFormat for () {
  fn charFormat(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment10charFormatEv()};
    unsafe {_ZNK13QTextFragment10charFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn length<T: QTextFragment_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QTextFragment_length {
  fn length(self, this: &mut QTextFragment) -> i32;
}

// proto: int QTextFragment::length();
impl<'a> /*trait*/ QTextFragment_length for () {
  fn length(self, this: &mut QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment6lengthEv()};
    unsafe {_ZNK13QTextFragment6lengthEv()};
    return 1;
  }
}

