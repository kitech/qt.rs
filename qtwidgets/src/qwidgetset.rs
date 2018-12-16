
// mod ::widgets::QWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qwidget.h
// #include <qwidget.h>
// #include <QtWidgets>

//  header block end

//  ext block begin

/*
#include <stdlib.h>
// extern C begin: 0
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QWidgetSet {
    pub qclsinst: usize /* *mut c_void*/,
}
// void swap(QSet<T> &)
impl QWidgetSet {
pub fn Swap_0(&self) -> (/*void*/) {
    // QWidgetSet_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QWidgetSet_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_not_equal_0(&self) -> bool {
    // QWidgetSet_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QWidgetSet {
pub fn Size_0(&self) -> i32 {
    // QWidgetSet_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool isEmpty()
impl QWidgetSet {
pub fn IsEmpty_0(&self) -> bool {
    // QWidgetSet_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int capacity()
impl QWidgetSet {
pub fn Capacity_0(&self) -> i32 {
    // QWidgetSet_capacity_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_capacity_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void reserve(int)
impl QWidgetSet {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QWidgetSet_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void squeeze()
impl QWidgetSet {
pub fn Squeeze_0(&self) -> (/*void*/) {
    // QWidgetSet_squeeze_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_squeeze_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach()
impl QWidgetSet {
pub fn Detach_0(&self) -> (/*void*/) {
    // QWidgetSet_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QWidgetSet {
pub fn IsDetached_0(&self) -> bool {
    // QWidgetSet_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QWidgetSet {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QWidgetSet_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void clear()
impl QWidgetSet {
pub fn Clear_0(&self) -> (/*void*/) {
    // QWidgetSet_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool remove(const T &)
impl QWidgetSet {
pub fn Remove_0(&self) -> bool {
    // QWidgetSet_remove_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_remove_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains(const T &)
impl QWidgetSet {
pub fn Contains_0(&self) -> bool {
    // QWidgetSet_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains(const QSet<T> &)
impl QWidgetSet {
pub fn Contains_1(&self) -> bool {
    // QWidgetSet_contains_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_contains_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QSet::iterator begin()
impl QWidgetSet {
pub fn Begin_0(&self) -> usize {
    // QWidgetSet_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator begin()
impl QWidgetSet {
pub fn Begin_1(&self) -> usize {
    // QWidgetSet_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator cbegin()
impl QWidgetSet {
pub fn Cbegin_0(&self) -> usize {
    // QWidgetSet_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator constBegin()
impl QWidgetSet {
pub fn ConstBegin_0(&self) -> usize {
    // QWidgetSet_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::iterator end()
impl QWidgetSet {
pub fn End_0(&self) -> usize {
    // QWidgetSet_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator end()
impl QWidgetSet {
pub fn End_1(&self) -> usize {
    // QWidgetSet_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator cend()
impl QWidgetSet {
pub fn Cend_0(&self) -> usize {
    // QWidgetSet_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator constEnd()
impl QWidgetSet {
pub fn ConstEnd_0(&self) -> usize {
    // QWidgetSet_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::reverse_iterator rbegin()
impl QWidgetSet {
pub fn Rbegin_0(&self) -> usize {
    // QWidgetSet_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::reverse_iterator rend()
impl QWidgetSet {
pub fn Rend_0(&self) -> usize {
    // QWidgetSet_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_reverse_iterator rbegin()
impl QWidgetSet {
pub fn Rbegin_1(&self) -> usize {
    // QWidgetSet_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_reverse_iterator rend()
impl QWidgetSet {
pub fn Rend_1(&self) -> usize {
    // QWidgetSet_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_reverse_iterator crbegin()
impl QWidgetSet {
pub fn Crbegin_0(&self) -> usize {
    // QWidgetSet_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_reverse_iterator crend()
impl QWidgetSet {
pub fn Crend_0(&self) -> usize {
    // QWidgetSet_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::iterator erase(QSet::iterator)
impl QWidgetSet {
pub fn Erase_0(&self) -> usize {
    // QWidgetSet_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::iterator erase(QSet::const_iterator)
impl QWidgetSet {
pub fn Erase_1(&self) -> usize {
    // QWidgetSet_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QWidgetSet {
pub fn Count_0(&self) -> i32 {
    // QWidgetSet_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QSet::iterator insert(const T &)
impl QWidgetSet {
pub fn Insert_0(&self) -> usize {
    // QWidgetSet_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::iterator find(const T &)
impl QWidgetSet {
pub fn Find_0(&self) -> usize {
    // QWidgetSet_find_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_find_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator find(const T &)
impl QWidgetSet {
pub fn Find_1(&self) -> usize {
    // QWidgetSet_find_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_find_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator constFind(const T &)
impl QWidgetSet {
pub fn ConstFind_0(&self) -> usize {
    // QWidgetSet_constFind_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_constFind_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & unite(const QSet<T> &)
impl QWidgetSet {
pub fn Unite_0(&self) -> usize {
    // QWidgetSet_unite_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_unite_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & intersect(const QSet<T> &)
impl QWidgetSet {
pub fn Intersect_0(&self) -> usize {
    // QWidgetSet_intersect_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_intersect_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool intersects(const QSet<T> &)
impl QWidgetSet {
pub fn Intersects_0(&self) -> bool {
    // QWidgetSet_intersects_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_intersects_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QSet<T> & subtract(const QSet<T> &)
impl QWidgetSet {
pub fn Subtract_0(&self) -> usize {
    // QWidgetSet_subtract_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_subtract_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool empty()
impl QWidgetSet {
pub fn Empty_0(&self) -> bool {
    // QWidgetSet_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QSet<T> & operator<<(const T &)
impl QWidgetSet {
pub fn Operator_left_shift_0(&self) -> usize {
    // QWidgetSet_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator|=(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_or_equal_0(&self) -> usize {
    // QWidgetSet_operator_or_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_or_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator|=(const T &)
impl QWidgetSet {
pub fn Operator_or_equal_1(&self) -> usize {
    // QWidgetSet_operator_or_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_or_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator&=(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_and_equal_0(&self) -> usize {
    // QWidgetSet_operator_and_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_and_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator&=(const T &)
impl QWidgetSet {
pub fn Operator_and_equal_1(&self) -> usize {
    // QWidgetSet_operator_and_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_and_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator+=(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_add_equal_0(&self) -> usize {
    // QWidgetSet_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator+=(const T &)
impl QWidgetSet {
pub fn Operator_add_equal_1(&self) -> usize {
    // QWidgetSet_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator-=(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_minus_equal_0(&self) -> usize {
    // QWidgetSet_operator_minus_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_minus_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> & operator-=(const T &)
impl QWidgetSet {
pub fn Operator_minus_equal_1(&self) -> usize {
    // QWidgetSet_operator_minus_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_minus_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> operator|(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_or_0(&self) -> usize {
    // QWidgetSet_operator_or_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_or_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> operator&(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_and_0(&self) -> usize {
    // QWidgetSet_operator_and_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_and_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> operator+(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_add_0(&self) -> usize {
    // QWidgetSet_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> operator-(const QSet<T> &)
impl QWidgetSet {
pub fn Operator_minus_0(&self) -> usize {
    // QWidgetSet_operator_minus_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_operator_minus_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> toList()
impl QWidgetSet {
pub fn ToList_0(&self) -> usize {
    // QWidgetSet_toList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_toList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> values()
impl QWidgetSet {
pub fn Values_0(&self) -> usize {
    // QWidgetSet_values_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_values_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> fromList(const QList<T> &)
impl QWidgetSet {
pub fn FromList_0(&self) -> usize {
    // QWidgetSet_fromList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_fromList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet::const_iterator m2c(QSet::iterator)
impl QWidgetSet {
pub fn M2c_0(&self) -> usize {
    // QWidgetSet_m2c_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_m2c_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool isValidIterator(const QSet::iterator &)
impl QWidgetSet {
pub fn IsValidIterator_0(&self) -> bool {
    // QWidgetSet_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isValidIterator(const QSet::const_iterator &)
impl QWidgetSet {
pub fn IsValidIterator_1(&self) -> bool {
    // QWidgetSet_isValidIterator_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetSet_isValidIterator_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
//  body block end
